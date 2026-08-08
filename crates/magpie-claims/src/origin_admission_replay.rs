//! Standing-inert exact-prefix and origin-binding candidate replay substrate.
//!
//! Exact prefix identities cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::VerifiedLogPrefixIdentityV0;
//! let _identity = VerifiedLogPrefixIdentityV0 {
//!     event_count: 0,
//!     tip_sha256: String::new(),
//! };
//! ```
//!
//! Serialized prefix identities cannot be deserialized into replay authority:
//!
//! ```compile_fail
//! use magpie_claims::VerifiedLogPrefixIdentityV0;
//! let _: VerifiedLogPrefixIdentityV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Replay contexts cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, StandingReplaySnapshot,
//!     VerifiedLogPrefixIdentityV0,
//! };
//! fn fabricate(
//!     snapshot: StandingReplaySnapshot,
//!     verified_prefix_identity: VerifiedLogPrefixIdentityV0,
//! ) {
//!     let _context = OriginAdmissionReplayContextV0 {
//!         snapshot,
//!         verified_prefix_identity,
//!     };
//! }
//! ```
//!
//! Serialized replay contexts cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionReplayContextV0;
//! let _: OriginAdmissionReplayContextV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! No public constructor accepts a snapshot and caller-held prefix identity:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, StandingReplaySnapshot,
//!     VerifiedLogPrefixIdentityV0,
//! };
//! fn transpose(
//!     snapshot: StandingReplaySnapshot,
//!     identity: VerifiedLogPrefixIdentityV0,
//! ) {
//!     let _ = OriginAdmissionReplayContextV0::new(snapshot, identity);
//! }
//! ```
//!
//! No public candidate enumerator accepts a caller-created anchor index:
//!
//! ```compile_fail
//! use magpie_claims::{origin_binding_candidates_v0, DeadboltAnchorIndex};
//! let anchors = DeadboltAnchorIndex::new();
//! let _ = origin_binding_candidates_v0(&anchors);
//! ```

use magpie_log::{LogError, LogReader, LogStore, Status, VerifiedReplaySummary};
use serde::Serialize;

use crate::admitted_contribution_audit::{
    resolve_admitted_contribution_audit_v0, AdmittedContributionAuditV0,
};
use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
use crate::deadbolt_context::DeadboltAnchorOccurrence;
use crate::origin_admission_audit::{resolve_origin_admission_audit_v0, OriginAdmissionAuditV0};
use crate::origin_binding_verifier::{
    ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0, ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use crate::replay_snapshot::{replay_standing_context_with_summary, StandingReplaySnapshot};
use crate::resolution_content_closure::ResolutionContentClosureV0;
use crate::standing_v3::{
    compose_standing_resolution_v3, StandingResolutionV3, StandingV3CompositionInputV0,
};
use crate::support_contribution_audit::{
    resolve_support_contribution_audit_v0, SupportContributionAuditV0,
};

/// Exact identity of the completely verified log prefix used for one replay.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct VerifiedLogPrefixIdentityV0 {
    event_count: u64,
    tip_sha256: String,
}

impl VerifiedLogPrefixIdentityV0 {
    fn from_replay_summary(summary: VerifiedReplaySummary) -> Self {
        Self {
            event_count: summary.event_count(),
            tip_sha256: summary.tip().to_hex(),
        }
    }

    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    pub fn tip_sha256(&self) -> &str {
        &self.tip_sha256
    }
}

#[cfg(test)]
pub(crate) fn stage_verified_log_prefix_identity_v0_for_tests(
    event_count: u64,
    tip_sha256: &str,
) -> VerifiedLogPrefixIdentityV0 {
    assert!(
        tip_sha256.len() == 64
            && tip_sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "staged verified-prefix tip must be exactly 64 lowercase hexadecimal characters"
    );
    VerifiedLogPrefixIdentityV0 {
        event_count,
        tip_sha256: tip_sha256.to_owned(),
    }
}

/// Co-derived standing, anchor, and exact verified-prefix replay context.
///
/// This context is replay substrate only. It establishes neither binding-byte
/// availability nor origin admission, conflict resolution, support, or standing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginAdmissionReplayContextV0 {
    snapshot: StandingReplaySnapshot,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
}

impl OriginAdmissionReplayContextV0 {
    pub fn snapshot(&self) -> &StandingReplaySnapshot {
        &self.snapshot
    }

    pub fn verified_prefix_identity(&self) -> &VerifiedLogPrefixIdentityV0 {
        &self.verified_prefix_identity
    }

    pub fn resolve_origin_admission_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> OriginAdmissionAuditV0 {
        resolve_origin_admission_audit_v0(self, closure)
    }

    pub fn resolve_admitted_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> AdmittedContributionAuditV0 {
        resolve_admitted_contribution_audit_v0(self, closure)
    }

    pub fn resolve_support_contribution_audit_v0(
        &self,
        closure: &ResolutionContentClosureV0,
    ) -> SupportContributionAuditV0 {
        resolve_support_contribution_audit_v0(self, closure)
    }

    /// Resolve one claim under the explicitly selected standing policy v3.
    pub fn resolved_standing_v3(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> Option<Status> {
        self.resolved_standing_with_trace_v3(claim_id, closure)
            .governed_standing()
    }

    /// Resolve one claim with the complete deterministic policy-v3 trace.
    pub fn resolved_standing_with_trace_v3(
        &self,
        claim_id: &str,
        closure: &ResolutionContentClosureV0,
    ) -> StandingResolutionV3 {
        let inherited_v2 = self.snapshot().resolved_standing_with_trace_v2(claim_id);
        let support_contribution_audit = self.resolve_support_contribution_audit_v0(closure);
        let requested_claim_scope = self
            .snapshot()
            .standing()
            .typed_claim(claim_id)
            .map(|claim| claim.scope_ref.clone());
        let input = StandingV3CompositionInputV0::new(
            claim_id,
            requested_claim_scope,
            self.verified_prefix_identity().clone(),
            closure.identity().clone(),
            inherited_v2,
            support_contribution_audit,
        );
        compose_standing_resolution_v3(input)
    }

    pub(crate) fn origin_binding_candidates_v0(&self) -> Vec<OriginBindingCandidateV0> {
        self.snapshot
            .anchors()
            .entries()
            .filter(|(identity, _occurrences)| {
                matches!(
                    identity.bundle_kind.as_str(),
                    ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0
                        | ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0
                ) && identity.witness_algorithm == ORIGIN_BINDING_WITNESS_ALGORITHM_V0
                    && identity.canonicalization_profile
                        == ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0
            })
            .map(|(identity, occurrences)| OriginBindingCandidateV0 {
                selector: ArtifactProvenanceAnchorSelectorV0::new(
                    &identity.bundle_kind,
                    &identity.witness_root,
                    &identity.witness_algorithm,
                    &identity.canonicalization_profile,
                    &identity.run_id,
                ),
                occurrences: occurrences.to_vec(),
            })
            .collect()
    }
}

/// Construct standing, anchors, count, and tip from one retained verified replay.
pub fn replay_origin_admission_context_v0<S: LogStore>(
    reader: &LogReader<S>,
) -> Result<OriginAdmissionReplayContextV0, LogError> {
    let (snapshot, summary) = replay_standing_context_with_summary(reader)?;
    let verified_prefix_identity = VerifiedLogPrefixIdentityV0::from_replay_summary(summary);

    Ok(OriginAdmissionReplayContextV0 {
        snapshot,
        verified_prefix_identity,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OriginBindingCandidateV0 {
    selector: ArtifactProvenanceAnchorSelectorV0,
    occurrences: Vec<DeadboltAnchorOccurrence>,
}

impl OriginBindingCandidateV0 {
    pub(crate) fn selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.selector
    }

    pub(crate) fn occurrences(&self) -> &[DeadboltAnchorOccurrence] {
        &self.occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deadbolt_context::DeadboltAnchorIdentity;
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey};

    const SEED: [u8; 32] = [47; 32];

    #[test]
    fn staged_verified_prefix_identity_is_test_only_and_exact() {
        let identity = stage_verified_log_prefix_identity_v0_for_tests(7, &"1".repeat(64));

        assert_eq!(identity.event_count(), 7);
        assert_eq!(identity.tip_sha256(), "1".repeat(64));
    }

    #[test]
    #[should_panic(
        expected = "staged verified-prefix tip must be exactly 64 lowercase hexadecimal characters"
    )]
    fn staged_verified_prefix_identity_rejects_noncanonical_digest() {
        let _ = stage_verified_log_prefix_identity_v0_for_tests(7, &"A".repeat(64));
    }

    fn identity(
        bundle_kind: &str,
        root: char,
        witness_algorithm: &str,
        canonicalization_profile: &str,
        run_id: &str,
    ) -> DeadboltAnchorIdentity {
        DeadboltAnchorIdentity {
            bundle_kind: bundle_kind.to_owned(),
            witness_root: root.to_string().repeat(64),
            witness_algorithm: witness_algorithm.to_owned(),
            canonicalization_profile: canonicalization_profile.to_owned(),
            run_id: run_id.to_owned(),
        }
    }

    fn supported_acquisition(root: char, run_id: &str) -> DeadboltAnchorIdentity {
        identity(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            root,
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            run_id,
        )
    }

    fn supported_derivation(root: char, run_id: &str) -> DeadboltAnchorIdentity {
        identity(
            ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
            root,
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            run_id,
        )
    }

    fn replay_context(identities: &[DeadboltAnchorIdentity]) -> OriginAdmissionReplayContextV0 {
        let store = MemStore::new();
        {
            let mut timestamp = 0u64;
            let mut writer = LogWriter::<MemStore>::open_with_clock(
                store.clone(),
                SigningKey::from_bytes(&SEED),
                Box::new(move || {
                    timestamp += 1;
                    timestamp
                }),
            )
            .unwrap();
            for identity in identities {
                writer
                    .append(
                        Provenance::new("candidate-test", "same-replay-anchor"),
                        Payload::SegmentAnchored {
                            bundle_kind: identity.bundle_kind.clone(),
                            witness_root: identity.witness_root.clone(),
                            witness_algorithm: identity.witness_algorithm.clone(),
                            canonicalization_profile: identity.canonicalization_profile.clone(),
                            run_id: identity.run_id.clone(),
                        },
                    )
                    .unwrap();
            }
        }
        let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
        replay_origin_admission_context_v0(&reader).unwrap()
    }

    fn selectors(
        context: &OriginAdmissionReplayContextV0,
    ) -> Vec<ArtifactProvenanceAnchorSelectorV0> {
        context
            .origin_binding_candidates_v0()
            .into_iter()
            .map(|candidate| candidate.selector().clone())
            .collect()
    }

    #[test]
    fn exact_acquisition_and_derivation_families_are_included() {
        let acquisition = supported_acquisition('a', "run-acquisition");
        let derivation = supported_derivation('b', "run-derivation");
        let context = replay_context(&[derivation.clone(), acquisition.clone()]);

        let candidates = context.origin_binding_candidates_v0();

        assert_eq!(candidates.len(), 2);
        assert_eq!(
            candidates[0].selector().bundle_kind(),
            acquisition.bundle_kind
        );
        assert_eq!(
            candidates[1].selector().bundle_kind(),
            derivation.bundle_kind
        );
    }

    #[test]
    fn wrong_bundle_kind_is_excluded_but_remains_indexed() {
        let wrong = identity(
            "unrelated-bundle-v0",
            'a',
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-wrong-kind",
        );
        let context = replay_context(std::slice::from_ref(&wrong));

        assert_eq!(context.snapshot().anchors().occurrences(&wrong).len(), 1);
        assert!(context.origin_binding_candidates_v0().is_empty());
    }

    #[test]
    fn right_kind_with_wrong_witness_algorithm_is_excluded_but_remains_indexed() {
        let wrong = identity(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            'a',
            "sha512",
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-wrong-algorithm",
        );
        let context = replay_context(std::slice::from_ref(&wrong));

        assert_eq!(context.snapshot().anchors().occurrences(&wrong).len(), 1);
        assert!(context.origin_binding_candidates_v0().is_empty());
    }

    #[test]
    fn right_kind_with_wrong_profile_is_excluded_but_remains_indexed() {
        let wrong = identity(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            'a',
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            "other-profile-v0",
            "run-wrong-profile",
        );
        let context = replay_context(std::slice::from_ref(&wrong));

        assert_eq!(context.snapshot().anchors().occurrences(&wrong).len(), 1);
        assert!(context.origin_binding_candidates_v0().is_empty());
    }

    #[test]
    fn repeated_exact_identity_is_one_candidate_with_all_ordered_occurrences() {
        let supported = supported_acquisition('a', "run-repeat");
        let context = replay_context(&[supported.clone(), supported.clone(), supported.clone()]);

        let candidates = context.origin_binding_candidates_v0();

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].occurrences().len(), 3);
        assert_eq!(
            candidates[0]
                .occurrences()
                .iter()
                .map(|occurrence| occurrence.sequence)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn exact_supported_identity_fields_separate_candidates() {
        let identities = [
            supported_acquisition('a', "run-a"),
            supported_derivation('a', "run-a"),
            supported_acquisition('b', "run-a"),
            supported_acquisition('a', "run-b"),
        ];
        let context = replay_context(&identities);

        let candidates = context.origin_binding_candidates_v0();

        assert_eq!(candidates.len(), 4);
        for identity in identities {
            assert!(candidates.iter().any(|candidate| {
                candidate.selector().bundle_kind() == identity.bundle_kind
                    && candidate.selector().witness_root() == identity.witness_root
                    && candidate.selector().witness_algorithm() == identity.witness_algorithm
                    && candidate.selector().canonicalization_profile()
                        == identity.canonicalization_profile
                    && candidate.selector().run_id() == identity.run_id
            }));
        }
    }

    #[test]
    fn candidate_order_is_five_field_lexical_order_not_insertion_order() {
        let expected_identities = [
            supported_acquisition('a', "run-a"),
            supported_acquisition('a', "run-z"),
            supported_acquisition('b', "run-a"),
            supported_derivation('a', "run-z"),
        ];
        let insertion_order: Vec<_> = expected_identities.iter().rev().cloned().collect();
        let context = replay_context(&insertion_order);
        let expected: Vec<_> = expected_identities
            .iter()
            .map(|identity| {
                ArtifactProvenanceAnchorSelectorV0::new(
                    &identity.bundle_kind,
                    &identity.witness_root,
                    &identity.witness_algorithm,
                    &identity.canonicalization_profile,
                    &identity.run_id,
                )
            })
            .collect();

        assert_eq!(selectors(&context), expected);
    }

    #[test]
    fn unrelated_anchors_do_not_influence_supported_candidates() {
        let supported = supported_acquisition('a', "run-supported");
        let unrelated = identity(
            "unrelated-bundle-v0",
            'b',
            "sha512",
            "unrelated-profile-v0",
            "run-unrelated",
        );
        let supported_only = replay_context(std::slice::from_ref(&supported));
        let with_unrelated = replay_context(&[supported, unrelated]);

        assert_eq!(
            supported_only.origin_binding_candidates_v0(),
            with_unrelated.origin_binding_candidates_v0()
        );
    }
}
