//! A 30-second governed-standing tour.
//! Run with: `cargo run --example tour -p magpie-claims`
//!
//! The fixed key and clock make this demonstration deterministic. Never reuse
//! this key outside tests and demonstrations.

use magpie_claims::{
    replay_standing_context, DeadboltAnchorIdentity, DeadboltAnchorOccurrence,
    DeadboltOccurrenceContextTrace, StandingPolicyRule, StandingReplaySnapshot,
    StandingTraceReason, DEADBOLT_OCCURRENCE_SCHEMA, MAGPIE_CLAIMS_POLICY_ID,
    MAGPIE_CLAIMS_POLICY_V1_ID,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use serde_json::{json, Value};

const SEED: [u8; 32] = [71u8; 32];
const EXPECTED_EVENT_COUNT: u64 = 11;
const ANCHOR_SEQUENCE: u64 = 10;
const PROVENANCE_AGENT: &str = "governed-standing-tour";
const PROVENANCE_SOURCE: &str = "signed-history-v1";

const POSITIVE_CLAIM_ID: &str = "claim-occurrence-matched";
const POSITIVE_EVIDENCE_ID: &str = "evidence-occurrence-matched";
const POSITIVE_EDGE_ID: &str = "edge-occurrence-matched";
const POSITIVE_SCOPE: &str = "scope:occurrence-matched";

const MISMATCH_CLAIM_ID: &str = "claim-occurrence-mismatch";
const MISMATCH_EVIDENCE_ID: &str = "evidence-occurrence-mismatch";
const MISMATCH_EDGE_ID: &str = "edge-occurrence-mismatch";
const MISMATCH_SCOPE: &str = "scope:occurrence-mismatch";

const INTERPRETATION_CLAIM_ID: &str = "claim-anchor-interpretation";
const INTERPRETATION_EVIDENCE_ID: &str = "evidence-anchor-interpretation";
const INTERPRETATION_EDGE_ID: &str = "edge-anchor-interpretation";
const INTERPRETATION_SCOPE: &str = "scope:anchor-interpretation";

#[derive(Debug, PartialEq, Eq)]
struct ResolutionBytes {
    positive_v0: Vec<u8>,
    positive_v1: Vec<u8>,
    mismatch_v0: Vec<u8>,
    mismatch_v1: Vec<u8>,
    interpretation_v0: Vec<u8>,
    interpretation_v1: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
struct TourFacts {
    event_count: u64,
    typed_claim_count: usize,
    typed_evidence_count: usize,
    edge_count: usize,
    anchor_identity_count: usize,
    positive_v0: Option<Status>,
    positive_v1: Option<Status>,
    positive_scalar: Option<Status>,
    positive_rule: StandingPolicyRule,
    positive_contribution: Option<Status>,
    positive_occurrence: DeadboltAnchorOccurrence,
    mismatch_v0: Option<Status>,
    mismatch_v1: Option<Status>,
    mismatch_rule: StandingPolicyRule,
    mismatch_context: DeadboltOccurrenceContextTrace,
    mismatch_contribution: Option<Status>,
    mismatch_blockers: Vec<StandingTraceReason>,
    interpretation_v0: Option<Status>,
    interpretation_v1: Option<Status>,
    interpretation_domain: String,
    interpretation_ceiling: Option<Status>,
    interpretation_has_application: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct TourCapture {
    snapshot_bytes: Vec<u8>,
    resolutions: ResolutionBytes,
    facts: TourFacts,
}

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn anchor_identity() -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "kernel-decision-witness-v1".into(),
        run_id: "governed-standing-tour-run".into(),
    }
}

fn identity_json(identity: &DeadboltAnchorIdentity) -> Value {
    json!({
        "schema": DEADBOLT_OCCURRENCE_SCHEMA,
        "bundle_kind": identity.bundle_kind,
        "witness_root": identity.witness_root,
        "witness_algorithm": identity.witness_algorithm,
        "canonicalization_profile": identity.canonicalization_profile,
        "run_id": identity.run_id,
    })
}

fn claim_metadata(domain: &str, identity: &DeadboltAnchorIdentity) -> String {
    json!({
        "claim_domain": domain,
        "deadbolt_occurrence": identity_json(identity),
    })
    .to_string()
}

fn evidence_metadata(identity: &DeadboltAnchorIdentity) -> String {
    json!({"deadbolt_anchor_ref": identity_json(identity)}).to_string()
}

fn claim_payload(claim_id: &str, statement: &str, scope: &str, metadata_json: String) -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: claim_id.into(),
        statement: statement.into(),
        scope_ref: scope.into(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json,
    }
}

fn evidence_payload(
    evidence_id: &str,
    summary: &str,
    scope: &str,
    metadata_json: String,
) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: evidence_id.into(),
        evidence_kind: "DeadboltAnchor".into(),
        summary: summary.into(),
        scope_ref: scope.into(),
        actor_class: "DeadboltAnchorer".into(),
        content_hash: String::new(),
        metadata_json,
    }
}

fn edge_payload(edge_id: &str, source_id: &str, target_id: &str, scope: &str) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: edge_id.into(),
        edge_kind: "supports".into(),
        source_id: source_id.into(),
        target_id: target_id.into(),
        scope_ref: scope.into(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate support for the governed-standing tour.".into(),
        metadata_json: "{}".into(),
    }
}

fn anchor_payload(identity: &DeadboltAnchorIdentity) -> Payload {
    Payload::SegmentAnchored {
        bundle_kind: identity.bundle_kind.clone(),
        witness_root: identity.witness_root.clone(),
        witness_algorithm: identity.witness_algorithm.clone(),
        canonicalization_profile: identity.canonicalization_profile.clone(),
        run_id: identity.run_id.clone(),
    }
}

fn deterministic_history(identity: &DeadboltAnchorIdentity) -> Vec<Payload> {
    let mut mismatched_reference = identity.clone();
    mismatched_reference.run_id = "governed-standing-tour-other-run".into();

    vec![
        claim_payload(
            POSITIVE_CLAIM_ID,
            "The exact five-field Deadbolt anchor identity occurs in this signed history.",
            POSITIVE_SCOPE,
            claim_metadata("Occurrence/Inclusion", identity),
        ),
        evidence_payload(
            POSITIVE_EVIDENCE_ID,
            "Exact reference to the occurrence identity.",
            POSITIVE_SCOPE,
            evidence_metadata(identity),
        ),
        edge_payload(
            POSITIVE_EDGE_ID,
            POSITIVE_EVIDENCE_ID,
            POSITIVE_CLAIM_ID,
            POSITIVE_SCOPE,
        ),
        claim_payload(
            MISMATCH_CLAIM_ID,
            "A hostile candidate whose claim and evidence references disagree.",
            MISMATCH_SCOPE,
            claim_metadata("Occurrence/Inclusion", identity),
        ),
        evidence_payload(
            MISMATCH_EVIDENCE_ID,
            "Reference with exactly one changed run_id.",
            MISMATCH_SCOPE,
            evidence_metadata(&mismatched_reference),
        ),
        edge_payload(
            MISMATCH_EDGE_ID,
            MISMATCH_EVIDENCE_ID,
            MISMATCH_CLAIM_ID,
            MISMATCH_SCOPE,
        ),
        claim_payload(
            INTERPRETATION_CLAIM_ID,
            "The anchored action was wise and its interpretation is true.",
            INTERPRETATION_SCOPE,
            claim_metadata("Interpretation", identity),
        ),
        evidence_payload(
            INTERPRETATION_EVIDENCE_ID,
            "The same occurrence cannot settle its interpretation.",
            INTERPRETATION_SCOPE,
            evidence_metadata(identity),
        ),
        edge_payload(
            INTERPRETATION_EDGE_ID,
            INTERPRETATION_EVIDENCE_ID,
            INTERPRETATION_CLAIM_ID,
            INTERPRETATION_SCOPE,
        ),
        anchor_payload(identity),
    ]
}

fn capture_and_assert(
    snapshot: &StandingReplaySnapshot,
    identity: &DeadboltAnchorIdentity,
    print_results: bool,
) -> TourCapture {
    assert_eq!(snapshot.event_count(), EXPECTED_EVENT_COUNT);
    assert_eq!(snapshot.standing().typed_claim_len(), 3);
    assert_eq!(snapshot.standing().typed_evidence_len(), 3);
    assert_eq!(snapshot.standing().justification_edge_len(), 3);
    assert_eq!(snapshot.anchors().len(), 1);

    let positive_v0 = snapshot
        .standing()
        .resolved_standing_with_trace(POSITIVE_CLAIM_ID);
    let positive_v1 = snapshot.resolved_standing_with_trace_v1(POSITIVE_CLAIM_ID);
    assert_eq!(positive_v0.policy_id, MAGPIE_CLAIMS_POLICY_ID);
    assert_eq!(positive_v1.policy_id, MAGPIE_CLAIMS_POLICY_V1_ID);
    assert_eq!(positive_v0.governed_standing, Some(Status::Conjectured));
    assert_eq!(positive_v1.governed_standing, Some(Status::Settled));
    assert_eq!(positive_v0.trace.len(), 1);
    assert_eq!(
        positive_v0.trace[0].candidate_ceiling,
        Some(Status::Settled)
    );
    assert_eq!(
        positive_v0.trace[0].reasons,
        vec![
            StandingTraceReason::AcceptedCandidate,
            StandingTraceReason::RequiresVerifierContext,
            StandingTraceReason::CeilingIsCandidateOnly,
        ]
    );
    assert_eq!(
        snapshot.standing().resolved_standing(POSITIVE_CLAIM_ID),
        positive_v0.governed_standing
    );
    assert_eq!(positive_v1.trace.len(), 1);
    let positive_application = positive_v1.trace[0]
        .application
        .as_ref()
        .expect("the exact occurrence candidate must enter the v1 lane");
    assert_eq!(
        positive_application.rule,
        StandingPolicyRule::DeadboltOccurrenceInclusionV1
    );
    assert_eq!(
        positive_application.achieved_standing,
        Some(Status::Settled)
    );
    let DeadboltOccurrenceContextTrace::Matched { occurrence } = &positive_application.context
    else {
        panic!("the positive lane must retain exact matched context")
    };
    assert_eq!(&occurrence.identity, identity);
    assert_eq!(occurrence.sequence, ANCHOR_SEQUENCE);
    assert_eq!(occurrence.event_hash.len(), 64);
    assert!(occurrence
        .event_hash
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    assert_eq!(occurrence.provenance_agent, PROVENANCE_AGENT);
    assert_eq!(occurrence.provenance_source, PROVENANCE_SOURCE);

    let mismatch_v0 = snapshot
        .standing()
        .resolved_standing_with_trace(MISMATCH_CLAIM_ID);
    let mismatch_v1 = snapshot.resolved_standing_with_trace_v1(MISMATCH_CLAIM_ID);
    assert_eq!(mismatch_v0.governed_standing, Some(Status::Conjectured));
    assert_eq!(mismatch_v1.governed_standing, Some(Status::Conjectured));
    assert_eq!(mismatch_v0.trace.len(), 1);
    assert_eq!(
        mismatch_v0.trace[0].candidate_ceiling,
        Some(Status::Settled)
    );
    assert_eq!(
        mismatch_v0.trace[0].reasons,
        vec![
            StandingTraceReason::AcceptedCandidate,
            StandingTraceReason::RequiresVerifierContext,
            StandingTraceReason::CeilingIsCandidateOnly,
        ]
    );
    assert_eq!(mismatch_v1.trace.len(), 1);
    let mismatch_application = mismatch_v1.trace[0]
        .application
        .as_ref()
        .expect("the structurally valid hostile candidate must attempt the lane");
    assert_eq!(
        mismatch_application.rule,
        StandingPolicyRule::DeadboltOccurrenceInclusionV1
    );
    assert_eq!(
        mismatch_application.context,
        DeadboltOccurrenceContextTrace::PredicateReferenceMismatch
    );
    assert_eq!(mismatch_application.achieved_standing, None);
    assert!(mismatch_v1
        .blockers
        .contains(&StandingTraceReason::RequiresVerifierContext));
    assert!(mismatch_v1
        .blockers
        .contains(&StandingTraceReason::CeilingIsCandidateOnly));

    let interpretation_v0 = snapshot
        .standing()
        .resolved_standing_with_trace(INTERPRETATION_CLAIM_ID);
    let interpretation_v1 = snapshot.resolved_standing_with_trace_v1(INTERPRETATION_CLAIM_ID);
    assert_eq!(
        interpretation_v0.governed_standing,
        Some(Status::Conjectured)
    );
    assert_eq!(
        interpretation_v1.governed_standing,
        Some(Status::Conjectured)
    );
    assert_ne!(interpretation_v1.governed_standing, Some(Status::Settled));
    assert_eq!(interpretation_v1.trace.len(), 1);
    let interpretation_candidate = &interpretation_v1.trace[0].candidate;
    assert_eq!(
        interpretation_candidate.claim_domain.as_deref(),
        Some("Interpretation")
    );
    assert_eq!(
        interpretation_candidate.candidate_ceiling,
        Some(Status::Supported)
    );
    assert!(interpretation_v1.trace[0].application.is_none());

    if print_results {
        println!("── 4. resolve positive occurrence claim ──");
        println!("  v0 policy: {}", positive_v0.policy_id);
        println!(
            "  v0 governed standing: {:?}",
            positive_v0.governed_standing.unwrap()
        );
        println!("  v0 candidate ceiling: Settled (not achieved standing)");
        println!("  v1 policy: {}", positive_v1.policy_id);
        println!(
            "  v1 governed standing: {:?}",
            positive_v1.governed_standing.unwrap()
        );
        println!("  lane/rule: {:?}", positive_application.rule);
        println!("  context: Matched");
        println!(
            "  achieved contribution: {:?}",
            positive_application.achieved_standing.unwrap()
        );
        println!(
            "  matched audit: seq {}, provenance {}/{}",
            occurrence.sequence, occurrence.provenance_agent, occurrence.provenance_source
        );

        println!("── 5. resolve hostile controls ──");
        println!("  mismatch v0 governed standing: Conjectured");
        println!("  mismatch v1 governed standing: Conjectured");
        println!("  mismatch lane attempted: {:?}", mismatch_application.rule);
        println!("  mismatch context: PredicateReferenceMismatch");
        println!("  mismatch achieved contribution: none");
        println!("  interpretation claim domain: Interpretation");
        println!("  interpretation v0 ceiling: Supported");
        println!("  interpretation v1 governed standing: Conjectured");
        println!("  interpretation lane application: none");
        println!("  occurrence proves no interpretation of the occurrence");
    }

    TourCapture {
        snapshot_bytes: snapshot.canonical_bytes(),
        resolutions: ResolutionBytes {
            positive_v0: positive_v0.canonical_bytes(),
            positive_v1: positive_v1.canonical_bytes(),
            mismatch_v0: mismatch_v0.canonical_bytes(),
            mismatch_v1: mismatch_v1.canonical_bytes(),
            interpretation_v0: interpretation_v0.canonical_bytes(),
            interpretation_v1: interpretation_v1.canonical_bytes(),
        },
        facts: TourFacts {
            event_count: snapshot.event_count(),
            typed_claim_count: snapshot.standing().typed_claim_len(),
            typed_evidence_count: snapshot.standing().typed_evidence_len(),
            edge_count: snapshot.standing().justification_edge_len(),
            anchor_identity_count: snapshot.anchors().len(),
            positive_v0: positive_v0.governed_standing,
            positive_v1: positive_v1.governed_standing,
            positive_scalar: snapshot.standing().resolved_standing(POSITIVE_CLAIM_ID),
            positive_rule: positive_application.rule,
            positive_contribution: positive_application.achieved_standing,
            positive_occurrence: occurrence.clone(),
            mismatch_v0: mismatch_v0.governed_standing,
            mismatch_v1: mismatch_v1.governed_standing,
            mismatch_rule: mismatch_application.rule,
            mismatch_context: mismatch_application.context.clone(),
            mismatch_contribution: mismatch_application.achieved_standing,
            mismatch_blockers: mismatch_v1.blockers.clone(),
            interpretation_v0: interpretation_v0.governed_standing,
            interpretation_v1: interpretation_v1.governed_standing,
            interpretation_domain: interpretation_candidate
                .claim_domain
                .clone()
                .expect("the interpretation domain must remain in trace"),
            interpretation_ceiling: interpretation_candidate.candidate_ceiling,
            interpretation_has_application: interpretation_v1.trace[0].application.is_some(),
        },
    }
}

fn main() {
    let identity = anchor_identity();
    let store = MemStore::new();

    println!("── 1. write signed history ──");
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .expect("the deterministic writer must open");

        for payload in deterministic_history(&identity) {
            writer
                .append(
                    Provenance::new(PROVENANCE_AGENT, PROVENANCE_SOURCE),
                    payload,
                )
                .expect("every governed-tour event must append");
        }
        assert_eq!(writer.len(), EXPECTED_EVENT_COUNT);
    }
    println!("  genesis");
    println!("  3 typed claims + 3 typed evidence + 3 support edges");
    println!("  1 exact Deadbolt occurrence anchor");

    let reader = LogReader::open(store, signing_key().verifying_key());
    println!("── 2. verify complete chain ──");
    let verified_count = reader
        .verify_chain()
        .expect("the complete signed history must verify");
    assert_eq!(verified_count, EXPECTED_EVENT_COUNT);
    println!("  {verified_count} events verified before replay");

    println!("── 3. replay one coherent snapshot ──");
    let snapshot = replay_standing_context(&reader)
        .expect("standing and anchor context must co-replay from one verified snapshot");
    assert_eq!(snapshot.event_count(), verified_count);
    println!("  StandingView + DeadboltAnchorIndex");
    println!("  same completely verified observational origin");

    let before = capture_and_assert(&snapshot, &identity, true);

    drop(snapshot);
    println!("── 6. drop all derived state ──");
    println!("  all derived standing state dropped");

    println!("── 7. replay and regenerate ──");
    let replayed = replay_standing_context(&reader)
        .expect("the signed history alone must regenerate governed standing");
    let after = capture_and_assert(&replayed, &identity, false);

    assert_eq!(
        before.facts, after.facts,
        "typed standing facts must replay"
    );
    assert_eq!(
        before.snapshot_bytes, after.snapshot_bytes,
        "the complete co-replayed snapshot must be byte-identical"
    );
    assert_eq!(
        before.resolutions, after.resolutions,
        "every governed resolution must be byte-identical"
    );

    println!("  replayed from signed history");
    println!("  snapshot bytes identical");
    println!("  all governed-resolution bytes identical");
    println!("Conserve the log. Derive the rest.");
}
