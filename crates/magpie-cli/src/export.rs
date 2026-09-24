//! The `magpie-desk-export-v0` format read by the Magpie Desk viewer.
//!
//! The export carries recorded facts plus standing under the one policy the
//! caller named. It covers typed claims, typed evidence, links, and notes;
//! legacy tag-1 to tag-3 records have no domain or scope and are left out, with
//! a count on stderr. No currentness resolver exists yet, so every claim's
//! currentness is `unknown` rather than inferred.

use std::collections::BTreeMap;
use std::io::Write;

use magpie_claims::policy::ClaimDomain;
use magpie_log::{Status, VerifiedReplaySummary, VerifyingKey};
use serde::Serialize;

use crate::index::{claim_domain, LedgerIndex, Recorded};
use crate::policy::{PolicyChoice, Resolver};
use crate::trace::trace_lines;
use crate::CliError;

const FORMAT: &str = "magpie-desk-export-v0";

#[derive(Serialize)]
pub(crate) struct DeskExport {
    format: &'static str,
    source: DeskSource,
    policy: DeskPolicy,
    claims: Vec<DeskClaim>,
    evidence: Vec<DeskEvidence>,
    edges: Vec<DeskEdge>,
    notes: Vec<DeskNote>,
}

#[derive(Serialize)]
struct DeskSource {
    log_tip: Option<String>,
    verifying_key: Option<String>,
    event_count: Option<u64>,
    producer: String,
}

#[derive(Serialize)]
struct DeskPolicy {
    id: &'static str,
    note: &'static str,
}

#[derive(Serialize)]
struct DeskRecorded {
    seq: u64,
    agent: String,
    source: String,
}

impl From<&Recorded> for DeskRecorded {
    fn from(recorded: &Recorded) -> Self {
        Self {
            seq: recorded.seq,
            agent: recorded.agent.clone(),
            source: recorded.source.clone(),
        }
    }
}

#[derive(Serialize)]
struct DeskStanding {
    status: Option<Status>,
    trace: Vec<String>,
}

#[derive(Serialize)]
struct DeskCurrentness {
    state: &'static str,
    reason: &'static str,
}

#[derive(Serialize)]
struct DeskClaim {
    id: String,
    statement: String,
    domain: &'static str,
    scope: String,
    actor_class: String,
    recorded: DeskRecorded,
    standing: DeskStanding,
    currentness: DeskCurrentness,
    labels: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct DeskEvidence {
    id: String,
    kind: String,
    summary: String,
    actor_class: String,
    content_hash: String,
    recorded: DeskRecorded,
}

#[derive(Serialize)]
struct DeskEdge {
    id: String,
    kind: String,
    source_id: String,
    target_id: String,
    rationale: String,
    recorded: DeskRecorded,
}

#[derive(Serialize)]
struct DeskNote {
    text: String,
    recorded: DeskRecorded,
}

pub(crate) fn desk_v0(
    index: &LedgerIndex,
    summary: VerifiedReplaySummary,
    verifying_key: VerifyingKey,
    policy: PolicyChoice,
    resolver: &Resolver<'_>,
    err: &mut dyn Write,
) -> Result<DeskExport, CliError> {
    let mut claims = Vec::new();
    let mut skipped = Vec::new();
    for (id, claim) in &index.claims {
        let Some(typed) = &claim.typed else {
            skipped.push(id.as_str());
            continue;
        };
        let domain = claim_domain(&typed.metadata_json)
            .and_then(|domain| ClaimDomain::try_from(domain.as_str()).ok());
        let Some(domain) = domain else {
            skipped.push(id.as_str());
            continue;
        };
        let resolution = resolver.resolve(policy, id)?;
        claims.push(DeskClaim {
            id: id.clone(),
            statement: claim.statement.clone(),
            domain: domain.as_str(),
            scope: typed.scope.clone(),
            actor_class: typed.actor_class.clone(),
            recorded: (&claim.recorded).into(),
            standing: DeskStanding {
                status: resolution.governed,
                trace: trace_lines(&resolution.explanation),
            },
            currentness: DeskCurrentness {
                state: "unknown",
                reason: "no currentness resolver is implemented yet",
            },
            labels: BTreeMap::new(),
        });
    }
    if !skipped.is_empty() {
        writeln!(
            err,
            "note: left out {} claim(s) without a typed, well-formed claim_domain: {}",
            skipped.len(),
            skipped.join(", ")
        )?;
    }

    let evidence = index
        .evidence
        .iter()
        .map(|(id, row)| DeskEvidence {
            id: id.clone(),
            kind: row.kind.clone(),
            summary: row.summary.clone(),
            actor_class: row.actor_class.clone(),
            content_hash: row.content_hash.clone(),
            recorded: (&row.recorded).into(),
        })
        .collect();
    let edges = index
        .edges
        .iter()
        .map(|(id, row)| DeskEdge {
            id: id.clone(),
            kind: row.kind.clone(),
            source_id: row.source_id.clone(),
            target_id: row.target_id.clone(),
            rationale: row.rationale.clone(),
            recorded: (&row.recorded).into(),
        })
        .collect();
    let notes = index
        .notes
        .iter()
        .map(|note| DeskNote {
            text: note.text.clone(),
            recorded: (&note.recorded).into(),
        })
        .collect();

    Ok(DeskExport {
        format: FORMAT,
        source: DeskSource {
            log_tip: Some(summary.tip().to_hex()),
            verifying_key: Some(hex::encode(verifying_key.as_bytes())),
            event_count: Some(summary.event_count()),
            producer: format!("magpie-cli {}", env!("CARGO_PKG_VERSION")),
        },
        policy: DeskPolicy {
            id: policy.id(),
            note: "standing computed under this policy only",
        },
        claims,
        evidence,
        edges,
        notes,
    })
}
