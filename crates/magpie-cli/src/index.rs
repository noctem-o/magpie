//! The ledger index: a projection that gives every command its view of the log.
//!
//! It is a pure fold over completely verified replay events, rebuilt from zero
//! on every invocation. Claim, evidence, and link ids share one namespace here
//! because links refer to them without saying which kind they name. When a log
//! written elsewhere repeats an id, the first occurrence wins and the id is
//! listed in `repeated_ids`; standing is still computed by the kernel's own
//! projections, not from this index.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use magpie_log::{
    LogError, LogReader, LogStore, Payload, Projection, SignedEvent, Status, VerifiedReplayEvent,
    VerifiedReplaySummary,
};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;

/// Where and by whom an event was recorded.
#[derive(Clone, Debug)]
pub(crate) struct Recorded {
    pub seq: u64,
    pub timestamp_nanos: u64,
    pub agent: String,
    pub source: String,
    pub hash: String,
}

impl Recorded {
    fn of(event: &SignedEvent) -> Self {
        Self {
            seq: event.core.seq,
            timestamp_nanos: event.core.timestamp_nanos,
            agent: event.core.provenance.agent.clone(),
            source: event.core.provenance.source.clone(),
            hash: event.hash.to_hex(),
        }
    }
}

pub(crate) struct EventRow {
    pub recorded: Recorded,
    pub kind: &'static str,
    pub subject: String,
}

/// Typed fields from an ADR-0002 `ClaimAssertedV2` assertion.
pub(crate) struct TypedClaim {
    pub scope: String,
    pub actor_class: String,
    pub metadata_json: String,
}

pub(crate) struct ClaimRow {
    pub statement: String,
    /// `Some` for typed claims; `None` for legacy tag-1 claims.
    pub typed: Option<TypedClaim>,
    /// The raw status a legacy claim asserted about itself. Quarantined: never standing.
    pub legacy_status: Option<Status>,
    pub legacy_notes: Vec<(String, Recorded)>,
    pub recorded: Recorded,
}

pub(crate) struct EvidenceRow {
    pub kind: String,
    pub summary: String,
    pub scope: String,
    pub actor_class: String,
    pub content_hash: String,
    pub metadata_json: String,
    pub recorded: Recorded,
}

pub(crate) struct EdgeRow {
    pub kind: String,
    pub source_id: String,
    pub target_id: String,
    pub scope: String,
    pub actor_class: String,
    pub rationale: String,
    pub recorded: Recorded,
}

pub(crate) struct NoteRow {
    pub text: String,
    pub recorded: Recorded,
}

#[derive(Default)]
pub(crate) struct LedgerIndex {
    pub events: Vec<EventRow>,
    pub claims: BTreeMap<String, ClaimRow>,
    pub evidence: BTreeMap<String, EvidenceRow>,
    pub edges: BTreeMap<String, EdgeRow>,
    pub notes: Vec<NoteRow>,
    pub repeated_ids: BTreeSet<String>,
}

impl LedgerIndex {
    /// Replay one completely verified snapshot of the log into a fresh index.
    pub(crate) fn replay<S: LogStore>(
        reader: &LogReader<S>,
    ) -> Result<(Self, VerifiedReplaySummary), LogError> {
        let mut index = Self::default();
        let summary = reader.replay_with_summary(&mut index)?;
        Ok((index, summary))
    }

    pub(crate) fn id_in_use(&self, id: &str) -> bool {
        self.claims.contains_key(id)
            || self.evidence.contains_key(id)
            || self.edges.contains_key(id)
    }

    /// The scope recorded on a typed claim, evidence node, or link.
    pub(crate) fn scope_of(&self, id: &str) -> Option<&str> {
        if let Some(claim) = self.claims.get(id) {
            return claim.typed.as_ref().map(|typed| typed.scope.as_str());
        }
        if let Some(evidence) = self.evidence.get(id) {
            return Some(&evidence.scope);
        }
        self.edges.get(id).map(|edge| edge.scope.as_str())
    }

    fn reserve_id(&mut self, id: &str) -> bool {
        if self.id_in_use(id) {
            self.repeated_ids.insert(id.to_owned());
            false
        } else {
            true
        }
    }
}

impl Projection for LedgerIndex {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>) {
        let raw = event.event();
        let recorded = Recorded::of(raw);
        let (kind, subject) = match &raw.core.payload {
            Payload::Genesis { .. } => ("genesis", "store created".to_owned()),
            Payload::ClaimAsserted {
                claim_id,
                statement,
                status,
            } => {
                if self.reserve_id(claim_id) {
                    self.claims.insert(
                        claim_id.clone(),
                        ClaimRow {
                            statement: statement.clone(),
                            typed: None,
                            legacy_status: Some(*status),
                            legacy_notes: Vec::new(),
                            recorded: recorded.clone(),
                        },
                    );
                }
                ("claim_asserted", claim_id.clone())
            }
            Payload::EvidenceRecorded { claim_id, summary } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim
                        .legacy_notes
                        .push((format!("legacy evidence: {summary}"), recorded.clone()));
                }
                ("evidence_recorded", claim_id.clone())
            }
            Payload::ClaimStatusChanged {
                claim_id,
                from,
                to,
                reason,
            } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.legacy_notes.push((
                        format!("legacy raw status {from:?} -> {to:?}: {reason}"),
                        recorded.clone(),
                    ));
                }
                ("claim_status_changed", claim_id.clone())
            }
            Payload::Note { text } => {
                self.notes.push(NoteRow {
                    text: text.clone(),
                    recorded: recorded.clone(),
                });
                ("note", text.clone())
            }
            Payload::SegmentAnchored {
                bundle_kind,
                witness_root: _,
                witness_algorithm: _,
                canonicalization_profile: _,
                run_id,
            } => ("segment_anchored", format!("{bundle_kind} {run_id}")),
            Payload::ClaimAssertedV2 {
                claim_id,
                statement,
                scope_ref,
                actor_class,
                content_hash: _,
                metadata_json,
            } => {
                if self.reserve_id(claim_id) {
                    self.claims.insert(
                        claim_id.clone(),
                        ClaimRow {
                            statement: statement.clone(),
                            typed: Some(TypedClaim {
                                scope: scope_ref.clone(),
                                actor_class: actor_class.clone(),
                                metadata_json: metadata_json.clone(),
                            }),
                            legacy_status: None,
                            legacy_notes: Vec::new(),
                            recorded: recorded.clone(),
                        },
                    );
                }
                ("claim_asserted_v2", claim_id.clone())
            }
            Payload::EvidenceRegistered {
                evidence_id,
                evidence_kind,
                summary,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json,
            } => {
                if self.reserve_id(evidence_id) {
                    self.evidence.insert(
                        evidence_id.clone(),
                        EvidenceRow {
                            kind: evidence_kind.clone(),
                            summary: summary.clone(),
                            scope: scope_ref.clone(),
                            actor_class: actor_class.clone(),
                            content_hash: content_hash.clone(),
                            metadata_json: metadata_json.clone(),
                            recorded: recorded.clone(),
                        },
                    );
                }
                ("evidence_registered", evidence_id.clone())
            }
            Payload::JustificationEdgeRecorded {
                edge_id,
                edge_kind,
                source_id,
                target_id,
                scope_ref,
                actor_class,
                rationale,
                metadata_json: _,
            } => {
                if self.reserve_id(edge_id) {
                    self.edges.insert(
                        edge_id.clone(),
                        EdgeRow {
                            kind: edge_kind.clone(),
                            source_id: source_id.clone(),
                            target_id: target_id.clone(),
                            scope: scope_ref.clone(),
                            actor_class: actor_class.clone(),
                            rationale: rationale.clone(),
                            recorded: recorded.clone(),
                        },
                    );
                }
                ("justification_edge_recorded", edge_id.clone())
            }
        };
        self.events.push(EventRow {
            recorded,
            kind,
            subject,
        });
    }
}

/// The top-level keys of a metadata object, refusing duplicate keys the way the
/// standing projection does. `None` if the text is not such an object.
pub(crate) fn metadata_object(text: &str) -> Option<BTreeMap<String, Value>> {
    struct Strict(BTreeMap<String, Value>);

    impl<'de> Deserialize<'de> for Strict {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct StrictVisitor;

            impl<'de> Visitor<'de> for StrictVisitor {
                type Value = Strict;

                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str("a JSON object with unique keys")
                }

                fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Strict, A::Error> {
                    let mut entries = BTreeMap::new();
                    while let Some((key, value)) = map.next_entry::<String, Value>()? {
                        if entries.contains_key(&key) {
                            return Err(de::Error::custom(format!("duplicate key `{key}`")));
                        }
                        entries.insert(key, value);
                    }
                    Ok(Strict(entries))
                }
            }

            deserializer.deserialize_map(StrictVisitor)
        }
    }

    serde_json::from_str::<Strict>(text)
        .ok()
        .map(|strict| strict.0)
}

/// The `claim_domain` recorded in a typed claim's metadata, if well formed.
pub(crate) fn claim_domain(metadata_json: &str) -> Option<String> {
    match metadata_object(metadata_json)?.get("claim_domain") {
        Some(Value::String(domain)) => Some(domain.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_domain_refuses_duplicate_keys_and_wrong_types() {
        assert_eq!(
            claim_domain(r#"{"claim_domain":"Interpretation"}"#).as_deref(),
            Some("Interpretation")
        );
        assert_eq!(
            claim_domain(r#"{"claim_domain":"Interpretation","claim_domain":"HumanJudgment"}"#),
            None
        );
        assert_eq!(claim_domain(r#"{"claim_domain":7}"#), None);
        assert_eq!(claim_domain("not json"), None);
        assert_eq!(claim_domain("{}"), None);
    }
}
