//! A-022 characterization: `actor_class` is recorded attribution, not a
//! standing input.
//!
//! Each case holds the substantive history constant and varies only
//! `actor_class` on the claim, the evidence, and the edge. The tests pin what
//! policies v0-v4 do today. They do not decide what a future admission
//! boundary should allow any actor class to write.

use magpie_claims::{
    replay_origin_admission_context_v0, OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use serde_json::Value;
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [122; 32];
const SCOPE: &str = "scope:actor-differential";
const CLAIM_ID: &str = "claim-a";
const OTHER_CLAIM_ID: &str = "claim-b";
const ACTORS: [&str; 4] = [
    "AgentProposer",
    "AutomatedVerifier",
    "HumanRoot",
    "SourceImporter",
];

/// SHA-256 of the bytes `abc`.
const ABC_DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const ZERO_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const ABC_HEX: &str = "616263";
const ABD_HEX: &str = "616264";

/// One actor class for each typed event position.
#[derive(Clone, Copy, Debug)]
struct Actors {
    claim: &'static str,
    evidence: &'static str,
    edge: &'static str,
}

/// Assignments in which every pair of positions takes every pair of classes
/// (a 4x4 Latin square), plus the four uniform assignments. Each assignment
/// costs one signed, verified replay, so the full cross product is not run.
fn actor_assignments() -> Vec<Actors> {
    let mut assignments = Vec::new();
    for claim in 0..ACTORS.len() {
        for evidence in 0..ACTORS.len() {
            assignments.push(Actors {
                claim: ACTORS[claim],
                evidence: ACTORS[evidence],
                edge: ACTORS[(claim + evidence) % ACTORS.len()],
            });
        }
    }
    // The square already holds the first uniform assignment.
    for actor in &ACTORS[1..] {
        assignments.push(Actors {
            claim: actor,
            evidence: actor,
            edge: actor,
        });
    }
    assignments
}

/// The same payloads with only `actor_class` replaced.
fn with_actors(payloads: &[Payload], actors: Actors) -> Vec<Payload> {
    payloads
        .iter()
        .cloned()
        .map(|mut payload| {
            match &mut payload {
                Payload::ClaimAssertedV2 { actor_class, .. } => {
                    *actor_class = actors.claim.to_owned();
                }
                Payload::EvidenceRegistered { actor_class, .. } => {
                    *actor_class = actors.evidence.to_owned();
                }
                Payload::JustificationEdgeRecorded { actor_class, .. } => {
                    *actor_class = actors.edge.to_owned();
                }
                Payload::Genesis { .. }
                | Payload::ClaimAsserted { .. }
                | Payload::EvidenceRecorded { .. }
                | Payload::ClaimStatusChanged { .. }
                | Payload::Note { .. }
                | Payload::SegmentAnchored { .. } => {}
            }
            payload
        })
        .collect()
}

fn replay(payloads: Vec<Payload>) -> OriginAdmissionReplayContextV0 {
    let key = SigningKey::from_bytes(&SEED);
    let store = MemStore::new();
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            key.clone(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for payload in payloads {
            writer
                .append(Provenance::new("test", "actor-differential"), payload)
                .unwrap();
        }
    }
    let reader = LogReader::open(store, key.verifying_key());
    replay_origin_admission_context_v0(&reader).unwrap()
}

/// Everything policies v0-v4 report for one claim.
///
/// The v3 and v4 audits name the verified prefix they were produced from. That
/// identity is a hash over the events, so it necessarily differs when a signed
/// `actor_class` differs; it is removed so the rest of the audit can be
/// compared exactly.
#[derive(Debug, PartialEq)]
struct PolicyOutcomes {
    standings: [Option<Status>; 5],
    legacy_raw: Option<Status>,
    v0: Value,
    v1: Value,
    v2: Value,
    v3: Value,
    v4: Value,
}

fn without_prefix_identity(mut value: Value) -> Value {
    fn strip(value: &mut Value) {
        match value {
            Value::Object(map) => {
                map.remove("verified_prefix_identity");
                map.values_mut().for_each(strip);
            }
            Value::Array(items) => items.iter_mut().for_each(strip),
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
    strip(&mut value);
    value
}

fn json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}

fn outcomes(payloads: Vec<Payload>, claim_ids: &[&str]) -> Vec<PolicyOutcomes> {
    let context = replay(payloads);
    let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
    let snapshot = context.snapshot();
    claim_ids
        .iter()
        .map(|claim_id| {
            let v0 = snapshot.standing().resolved_standing_with_trace(claim_id);
            let v1 = snapshot.resolved_standing_with_trace_v1(claim_id);
            let v2 = snapshot.resolved_standing_with_trace_v2(claim_id);
            let v3 = context.resolved_standing_with_trace_v3(claim_id, &closure);
            let v4 = context.resolved_standing_with_trace_v4(claim_id, &closure);
            PolicyOutcomes {
                standings: [
                    v0.governed_standing,
                    v1.governed_standing,
                    v2.governed_standing,
                    v3.governed_standing(),
                    v4.governed_standing(),
                ],
                legacy_raw: v0.legacy_raw_standing,
                v0: json(&v0.canonical_bytes()),
                v1: json(&v1.canonical_bytes()),
                v2: json(&v2.canonical_bytes()),
                v3: without_prefix_identity(json(&v3.canonical_bytes())),
                v4: without_prefix_identity(json(&v4.canonical_bytes())),
            }
        })
        .collect()
}

/// Resolve each claim under every actor assignment and require the complete
/// outcome of every policy to be identical. Returns that one outcome per claim.
fn actor_invariant_outcomes(payloads: &[Payload], claim_ids: &[&str]) -> Vec<PolicyOutcomes> {
    let assignments = actor_assignments();
    let baseline = outcomes(with_actors(payloads, assignments[0]), claim_ids);
    for actors in assignments {
        assert_eq!(
            outcomes(with_actors(payloads, actors), claim_ids),
            baseline,
            "resolved differently under {actors:?}"
        );
    }
    baseline
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn machine_statement() -> String {
    format!("sha256_bytes_equals_v0:{ABC_DIGEST}")
}

/// A claim that some bytes hash to `ABC_DIGEST`; the bytes arrive as a witness.
fn machine_claim(claim_id: &str) -> Payload {
    let statement = machine_statement();
    Payload::ClaimAssertedV2 {
        claim_id: claim_id.to_owned(),
        content_hash: sha256_hex(statement.as_bytes()),
        statement,
        scope_ref: SCOPE.to_owned(),
        actor_class: String::new(),
        metadata_json: format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{ABC_DIGEST}"}}}}"#
        ),
    }
}

fn typed_claim(claim_id: &str, domain: &str) -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: claim_id.to_owned(),
        statement: "A governed claim.".to_owned(),
        scope_ref: SCOPE.to_owned(),
        actor_class: String::new(),
        content_hash: String::new(),
        metadata_json: format!(r#"{{"claim_domain":"{domain}"}}"#),
    }
}

fn witness(subject_claim_id: &str, witness_hex: &str) -> String {
    format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{subject_claim_id}","scope_ref":"{SCOPE}","witness_hex":"{witness_hex}"}}}}"#
    )
}

fn evidence(evidence_id: &str, evidence_kind: &str, metadata_json: &str) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: evidence_id.to_owned(),
        evidence_kind: evidence_kind.to_owned(),
        summary: "Asserts that it was verified.".to_owned(),
        scope_ref: SCOPE.to_owned(),
        actor_class: String::new(),
        content_hash: String::new(),
        metadata_json: metadata_json.to_owned(),
    }
}

fn edge(edge_id: &str, edge_kind: &str, source_id: &str, target_id: &str) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: edge_id.to_owned(),
        edge_kind: edge_kind.to_owned(),
        source_id: source_id.to_owned(),
        target_id: target_id.to_owned(),
        scope_ref: SCOPE.to_owned(),
        actor_class: String::new(),
        rationale: "Candidate relationship only.".to_owned(),
        metadata_json: "{}".to_owned(),
    }
}

const CONJECTURED: Option<Status> = Some(Status::Conjectured);
const SUPPORTED: Option<Status> = Some(Status::Supported);

#[test]
fn assignments_cover_every_pair_of_positions_and_every_uniform_class() {
    let assignments = actor_assignments();
    assert_eq!(assignments.len(), 19);
    let pairs = |select: fn(&Actors) -> (&str, &str)| {
        assignments
            .iter()
            .map(select)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    };
    assert_eq!(pairs(|actors| (actors.claim, actors.evidence)), 16);
    assert_eq!(pairs(|actors| (actors.claim, actors.edge)), 16);
    assert_eq!(pairs(|actors| (actors.evidence, actors.edge)), 16);
    for actor in ACTORS {
        assert!(assignments.iter().any(|actors| actors.claim == actor
            && actors.evidence == actor
            && actors.edge == actor));
    }
}

#[test]
fn recomputed_witness_supports_from_v2_under_every_actor_class() {
    // Fully agent-attributed material is one of the assignments. Support comes
    // from the policy re-hashing the witness bytes, not from any label.
    let payloads = [
        machine_claim(CLAIM_ID),
        evidence(
            "evidence-a",
            "DeterministicVerification",
            &witness(CLAIM_ID, ABC_HEX),
        ),
        edge("edge-a", "supports", "evidence-a", CLAIM_ID),
    ];
    let outcome = actor_invariant_outcomes(&payloads, &[CLAIM_ID]);
    assert_eq!(
        outcome[0].standings,
        [CONJECTURED, CONJECTURED, SUPPORTED, SUPPORTED, SUPPORTED]
    );
}

#[test]
fn deterministic_label_without_a_matching_witness_supports_under_no_actor_class() {
    let cases = [
        ("claim-bare", "{}".to_owned()),
        ("claim-boolean", r#"{"verified":true}"#.to_owned()),
        ("claim-mismatch", witness("claim-mismatch", ABD_HEX)),
    ];
    let mut payloads = Vec::new();
    for (claim_id, metadata) in &cases {
        let evidence_id = format!("evidence-{claim_id}");
        payloads.push(machine_claim(claim_id));
        payloads.push(evidence(
            &evidence_id,
            "DeterministicVerification",
            metadata,
        ));
        payloads.push(edge(
            &format!("edge-{claim_id}"),
            "supports",
            &evidence_id,
            claim_id,
        ));
    }
    let claim_ids = cases.map(|(claim_id, _)| claim_id);
    for (outcome, claim_id) in actor_invariant_outcomes(&payloads, &claim_ids)
        .iter()
        .zip(claim_ids)
    {
        assert_eq!(outcome.standings, [CONJECTURED; 5], "{claim_id}");
    }
}

#[test]
fn witness_bound_to_one_claim_supports_no_other_claim_under_any_actor_class() {
    // Both claims state the same predicate. The valid witness for claim A is
    // linked to claim B directly and through a verbatim copy.
    let bound_to_a = witness(CLAIM_ID, ABC_HEX);
    let payloads = [
        machine_claim(CLAIM_ID),
        machine_claim(OTHER_CLAIM_ID),
        evidence("evidence-a", "DeterministicVerification", &bound_to_a),
        evidence("evidence-copy", "DeterministicVerification", &bound_to_a),
        edge("edge-a", "supports", "evidence-a", CLAIM_ID),
        edge("edge-reused", "supports", "evidence-a", OTHER_CLAIM_ID),
        edge("edge-copy", "supports", "evidence-copy", OTHER_CLAIM_ID),
    ];
    let outcome = actor_invariant_outcomes(&payloads, &[CLAIM_ID, OTHER_CLAIM_ID]);
    assert_eq!(
        outcome[0].standings,
        [CONJECTURED, CONJECTURED, SUPPORTED, SUPPORTED, SUPPORTED]
    );
    assert_eq!(outcome[1].standings, [CONJECTURED; 5]);
}

#[test]
fn human_ratification_contributes_under_no_policy_and_no_actor_class() {
    // Both domains give HumanRatification a `Supported` ceiling, gated on an
    // admission that no policy implements. A `HumanRoot` label does not supply
    // it, and neither does any other.
    let cases = [
        ("claim-interpretation", "Interpretation"),
        ("claim-operational", "OperationalObservation"),
    ];
    let mut payloads = Vec::new();
    for (claim_id, domain) in cases {
        let evidence_id = format!("evidence-{claim_id}");
        payloads.push(typed_claim(claim_id, domain));
        payloads.push(evidence(&evidence_id, "HumanRatification", "{}"));
        for edge_kind in ["supports", "ratifies"] {
            payloads.push(edge(
                &format!("edge-{edge_kind}-{claim_id}"),
                edge_kind,
                &evidence_id,
                claim_id,
            ));
        }
    }
    let claim_ids = cases.map(|(claim_id, _)| claim_id);
    for (outcome, claim_id) in actor_invariant_outcomes(&payloads, &claim_ids)
        .iter()
        .zip(claim_ids)
    {
        assert_eq!(outcome.standings, [CONJECTURED; 5], "{claim_id}");
    }
}

#[test]
fn legacy_status_change_moves_no_governed_standing_under_any_actor_class() {
    let cases = [
        ("claim-settled", Status::Settled),
        ("claim-supported", Status::Supported),
        ("claim-refuted", Status::Refuted),
    ];
    let mut payloads = Vec::new();
    for (claim_id, decreed) in cases {
        payloads.push(typed_claim(claim_id, "ExternalReport"));
        payloads.push(Payload::ClaimStatusChanged {
            claim_id: claim_id.to_owned(),
            from: Status::Conjectured,
            to: decreed,
            reason: "Legacy manual decree.".to_owned(),
        });
    }
    let claim_ids = cases.map(|(claim_id, _)| claim_id);
    for (outcome, (claim_id, decreed)) in actor_invariant_outcomes(&payloads, &claim_ids)
        .iter()
        .zip(cases)
    {
        // The decree stays visible as legacy raw status and nowhere else.
        assert_eq!(outcome.legacy_raw, Some(decreed), "{claim_id}");
        assert_eq!(outcome.standings, [CONJECTURED; 5], "{claim_id}");
    }
}

#[test]
fn inline_refutation_is_recomputed_by_v4_under_every_actor_class() {
    // The claim carries its own subject bytes (`abc`) and a digest they do not
    // hash to. Policy v4 recomputes the digest itself.
    let statement = format!(
        "magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:{ZERO_DIGEST}:{ABC_HEX}"
    );
    let payloads = [
        Payload::ClaimAssertedV2 {
            claim_id: CLAIM_ID.to_owned(),
            content_hash: sha256_hex(statement.as_bytes()),
            statement,
            scope_ref: SCOPE.to_owned(),
            actor_class: String::new(),
            metadata_json: format!(
                r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"magpie-machine-predicate-inline-bytes-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","expected_sha256":"{ZERO_DIGEST}","subject_hex":"{ABC_HEX}"}}}}"#
            ),
        },
        evidence(
            "evidence-a",
            "DeterministicVerification",
            &format!(
                r#"{{"inline_predicate_attestation":{{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}"}}}}"#
            ),
        ),
        edge("edge-a", "contradicts", "evidence-a", CLAIM_ID),
    ];
    let outcome = actor_invariant_outcomes(&payloads, &[CLAIM_ID]);
    assert_eq!(
        outcome[0].standings,
        [
            CONJECTURED,
            CONJECTURED,
            CONJECTURED,
            CONJECTURED,
            Some(Status::Refuted)
        ]
    );
}
