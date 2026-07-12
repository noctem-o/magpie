use magpie_claims::{
    policy::ClaimDomain, policy::EvidenceKind, resolve_deadbolt_occurrence_context,
    DeadboltAnchorIdentity, DeadboltAnchorIndex, DeadboltOccurrenceContextResolution, StandingView,
    TypedClaimNode, TypedEvidenceNode, DEADBOLT_OCCURRENCE_SCHEMA,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, VerifyingKey};
use serde_json::{json, Value};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/anchor-log.jsonl"
);
const VERIFYING_KEY_HEX: &str = "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";
const BUNDLE_KIND: &str = "kernel-decision-witness";
const WITNESS_ROOT: &str = "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b";
const WITNESS_ALGORITHM: &str = "sha256";
const FOREIGN_PROFILE: &str = "kernel-decision-witness-v1";
const RUN_ID: &str = "portable-run-0001";
const EVENT_HASH: &str = "572054edbb5b662f3dcaccc38a323e032c7c6423041c86eadbcfe94414fe5a3b";
const TEST_SEED: [u8; 32] = [41u8; 32];

fn fixture_store() -> MemStore {
    let data = std::fs::read(FIXTURE_PATH).expect("portable anchor fixture must be committed");
    MemStore::from_records(
        data.split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect(),
    )
}

fn fixture_reader() -> LogReader<MemStore> {
    LogReader::open(fixture_store(), fixture_verifying_key())
}

fn fixture_verifying_key() -> VerifyingKey {
    VerifyingKey::from_bytes(&decode_hex_32(VERIFYING_KEY_HEX))
        .expect("fixture verifying key must be valid")
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let mut output = [0u8; 32];
    for (index, slot) in output.iter_mut().enumerate() {
        let high = hex_nibble(value.as_bytes()[index * 2]);
        let low = hex_nibble(value.as_bytes()[index * 2 + 1]);
        *slot = (high << 4) | low;
    }
    output
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("fixture hex must be lowercase"),
    }
}

fn fixture_identity() -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: BUNDLE_KIND.into(),
        witness_root: WITNESS_ROOT.into(),
        witness_algorithm: WITNESS_ALGORITHM.into(),
        canonicalization_profile: FOREIGN_PROFILE.into(),
        run_id: RUN_ID.into(),
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

fn anchor_json(identity: &DeadboltAnchorIdentity) -> Value {
    json!({
        "schema": DEADBOLT_OCCURRENCE_SCHEMA,
        "bundle_kind": identity.bundle_kind,
        "witness_root": identity.witness_root,
        "witness_algorithm": identity.witness_algorithm,
        "canonicalization_profile": identity.canonicalization_profile,
        "run_id": identity.run_id,
    })
}

fn claim_metadata(identity: &DeadboltAnchorIdentity) -> String {
    json!({
        "claim_domain": "Occurrence/Inclusion",
        "deadbolt_occurrence": anchor_json(identity),
    })
    .to_string()
}

fn evidence_metadata(identity: &DeadboltAnchorIdentity) -> String {
    json!({
        "deadbolt_anchor_ref": anchor_json(identity),
    })
    .to_string()
}

fn fixture_index() -> DeadboltAnchorIndex {
    let mut index = DeadboltAnchorIndex::new();
    assert_eq!(fixture_reader().replay(&mut index).unwrap(), 2);
    index
}

fn resolve(
    claim_metadata_json: &str,
    evidence_metadata_json: &str,
    anchors: &DeadboltAnchorIndex,
) -> DeadboltOccurrenceContextResolution {
    resolve_deadbolt_occurrence_context(
        EvidenceKind::DeadboltAnchor,
        ClaimDomain::OccurrenceInclusion,
        claim_metadata_json,
        evidence_metadata_json,
        anchors,
    )
}

fn replay_generated(payloads: Vec<Payload>) -> DeadboltAnchorIndex {
    let store = MemStore::new();
    let signing_key = SigningKey::from_bytes(&TEST_SEED);
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key.clone(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for payload in payloads {
            writer
                .append(Provenance::new("test", "deadbolt-occurrence"), payload)
                .unwrap();
        }
    }

    let reader = LogReader::open(store, signing_key.verifying_key());
    let mut index = DeadboltAnchorIndex::new();
    reader.replay(&mut index).unwrap();
    index
}

#[test]
fn deadbolt_occurrence_fixture_builds_exact_auditable_index() {
    let index = fixture_index();
    let identity = fixture_identity();

    assert_eq!(index.len(), 1);
    assert!(!index.is_empty());
    let occurrences = index.occurrences(&identity);
    assert_eq!(occurrences.len(), 1);
    let occurrence = &occurrences[0];
    assert_eq!(occurrence.identity, identity);
    assert_eq!(
        occurrence.identity.canonicalization_profile,
        FOREIGN_PROFILE
    );
    assert_ne!(
        occurrence.identity.canonicalization_profile,
        magpie_log::CANONICALIZATION_PROFILE
    );
    assert_eq!(occurrence.sequence, 1);
    assert_eq!(occurrence.event_hash, EVENT_HASH);
    assert_eq!(occurrence.provenance_agent, "deadbolt-fixture");
    assert_eq!(occurrence.provenance_source, "portable-anchor-v1");
}

#[test]
fn deadbolt_occurrence_index_replay_bytes_are_deterministic() {
    let first = fixture_index();
    let second = fixture_index();

    assert_eq!(first, second);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
}

#[test]
fn deadbolt_occurrence_index_ignores_unrelated_payloads() {
    let index = replay_generated(vec![Payload::Note {
        text: "not an anchor".into(),
    }]);

    assert!(index.is_empty());
    assert_eq!(index.len(), 0);
}

#[test]
fn deadbolt_occurrence_index_retains_repeated_identity_in_sequence_order() {
    let identity = fixture_identity();
    let index = replay_generated(vec![anchor_payload(&identity), anchor_payload(&identity)]);

    assert_eq!(index.len(), 1);
    assert_eq!(
        index
            .occurrences(&identity)
            .iter()
            .map(|occurrence| occurrence.sequence)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(index.first_occurrence(&identity).unwrap().sequence, 1);
}

#[test]
fn deadbolt_occurrence_exact_three_way_match_returns_fixture_occurrence() {
    let identity = fixture_identity();
    let result = resolve(
        &claim_metadata(&identity),
        &evidence_metadata(&identity),
        &fixture_index(),
    );

    let DeadboltOccurrenceContextResolution::Matched(occurrence) = result else {
        panic!("exact fixture identity must match");
    };
    assert_eq!(occurrence.identity, identity);
    assert_eq!(occurrence.sequence, 1);
    assert_eq!(occurrence.event_hash, EVENT_HASH);
}

#[derive(Clone, Copy, Debug)]
enum IdentityField {
    BundleKind,
    WitnessRoot,
    WitnessAlgorithm,
    CanonicalizationProfile,
    RunId,
}

fn change_one_field(
    identity: &DeadboltAnchorIdentity,
    field: IdentityField,
) -> DeadboltAnchorIdentity {
    let mut changed = identity.clone();
    match field {
        IdentityField::BundleKind => changed.bundle_kind = "other-kind".into(),
        IdentityField::WitnessRoot => changed.witness_root = "b".repeat(64),
        IdentityField::WitnessAlgorithm => changed.witness_algorithm = "other-algorithm".into(),
        IdentityField::CanonicalizationProfile => {
            changed.canonicalization_profile = "other-profile".into()
        }
        IdentityField::RunId => changed.run_id = "other-run".into(),
    }
    changed
}

#[test]
fn deadbolt_occurrence_each_identity_field_must_match_claim_and_evidence() {
    let anchored = fixture_identity();
    let anchors = fixture_index();

    for field in [
        IdentityField::BundleKind,
        IdentityField::WitnessRoot,
        IdentityField::WitnessAlgorithm,
        IdentityField::CanonicalizationProfile,
        IdentityField::RunId,
    ] {
        let changed = change_one_field(&anchored, field);
        assert_eq!(
            resolve(
                &claim_metadata(&anchored),
                &evidence_metadata(&changed),
                &anchors,
            ),
            DeadboltOccurrenceContextResolution::PredicateReferenceMismatch,
            "{field:?} disagreement"
        );
        assert_eq!(
            resolve(
                &claim_metadata(&changed),
                &evidence_metadata(&changed),
                &anchors,
            ),
            DeadboltOccurrenceContextResolution::AnchorNotFound,
            "{field:?} unanchored identity"
        );
    }
}

#[test]
fn deadbolt_occurrence_missing_and_malformed_outer_metadata_fail_closed() {
    let identity = fixture_identity();
    let anchors = fixture_index();

    assert_eq!(
        resolve("{}", &evidence_metadata(&identity), &anchors),
        DeadboltOccurrenceContextResolution::MissingClaimPredicate
    );
    assert_eq!(
        resolve(&claim_metadata(&identity), "{}", &anchors),
        DeadboltOccurrenceContextResolution::MissingEvidenceReference
    );
    assert_eq!(
        resolve("{", &evidence_metadata(&identity), &anchors),
        DeadboltOccurrenceContextResolution::MalformedClaimPredicate
    );
    assert_eq!(
        resolve(&claim_metadata(&identity), "{", &anchors),
        DeadboltOccurrenceContextResolution::MalformedEvidenceReference
    );
}

fn invalid_anchor_objects() -> Vec<(&'static str, String)> {
    let valid = anchor_json(&fixture_identity());
    let mut missing_field = valid.clone();
    missing_field.as_object_mut().unwrap().remove("run_id");
    let mut wrong_type = valid.clone();
    wrong_type["witness_algorithm"] = json!(7);
    let mut unknown_field = valid.clone();
    unknown_field["verified"] = json!(true);
    let mut unknown_schema = valid.clone();
    unknown_schema["schema"] = json!("segment_anchored_v2");
    let mut uppercase_root = valid.clone();
    uppercase_root["witness_root"] = json!(WITNESS_ROOT.to_uppercase());
    let mut short_root = valid.clone();
    short_root["witness_root"] = json!("abc");
    let mut empty_string = valid.clone();
    empty_string["bundle_kind"] = json!("");
    let mut null_field = valid.clone();
    null_field["run_id"] = Value::Null;
    let mut array_field = valid.clone();
    array_field["canonicalization_profile"] = json!([FOREIGN_PROFILE]);
    let mut alias = valid;
    let root = alias
        .as_object_mut()
        .unwrap()
        .remove("witness_root")
        .unwrap();
    alias["witnessRoot"] = root;

    vec![
        ("object is null", "null".into()),
        ("object is array", "[]".into()),
        ("missing field", missing_field.to_string()),
        ("wrong field type", wrong_type.to_string()),
        (
            "duplicate nested key",
            format!(
                r#"{{"schema":"{0}","schema":"{0}","bundle_kind":"{1}","witness_root":"{2}","witness_algorithm":"{3}","canonicalization_profile":"{4}","run_id":"{5}"}}"#,
                DEADBOLT_OCCURRENCE_SCHEMA,
                BUNDLE_KIND,
                WITNESS_ROOT,
                WITNESS_ALGORITHM,
                FOREIGN_PROFILE,
                RUN_ID
            ),
        ),
        ("unknown nested field", unknown_field.to_string()),
        ("unknown schema", unknown_schema.to_string()),
        ("uppercase witness root", uppercase_root.to_string()),
        ("short witness root", short_root.to_string()),
        ("empty required string", empty_string.to_string()),
        ("null field", null_field.to_string()),
        ("array field", array_field.to_string()),
        ("field alias", alias.to_string()),
    ]
}

#[test]
fn deadbolt_occurrence_authoritative_objects_are_strict_on_both_surfaces() {
    let identity = fixture_identity();
    let anchors = fixture_index();

    for (case, invalid_object) in invalid_anchor_objects() {
        let claim = format!(r#"{{"deadbolt_occurrence":{invalid_object}}}"#);
        assert_eq!(
            resolve(&claim, &evidence_metadata(&identity), &anchors),
            DeadboltOccurrenceContextResolution::MalformedClaimPredicate,
            "claim case: {case}"
        );

        let evidence = format!(r#"{{"deadbolt_anchor_ref":{invalid_object}}}"#);
        assert_eq!(
            resolve(&claim_metadata(&identity), &evidence, &anchors),
            DeadboltOccurrenceContextResolution::MalformedEvidenceReference,
            "evidence case: {case}"
        );
    }
}

#[test]
fn deadbolt_occurrence_duplicate_authoritative_outer_keys_fail_closed() {
    let identity = fixture_identity();
    let object = anchor_json(&identity).to_string();
    let anchors = fixture_index();
    let claim = format!(r#"{{"deadbolt_occurrence":{object},"deadbolt_occurrence":{object}}}"#);
    let evidence = format!(r#"{{"deadbolt_anchor_ref":{object},"deadbolt_anchor_ref":{object}}}"#);

    assert_eq!(
        resolve(&claim, &evidence_metadata(&identity), &anchors),
        DeadboltOccurrenceContextResolution::MalformedClaimPredicate
    );
    assert_eq!(
        resolve(&claim_metadata(&identity), &evidence, &anchors),
        DeadboltOccurrenceContextResolution::MalformedEvidenceReference
    );
}

#[test]
fn deadbolt_occurrence_labels_prose_and_verified_boole_cannot_replace_metadata() {
    let identity = fixture_identity();
    let anchors = fixture_index();

    assert_eq!(
        resolve(&claim_metadata(&identity), "{}", &anchors),
        DeadboltOccurrenceContextResolution::MissingEvidenceReference
    );
    assert_eq!(
        resolve(
            &claim_metadata(&identity),
            &format!(r#"{{"witness_root":"{WITNESS_ROOT}"}}"#),
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::MissingEvidenceReference
    );
    assert_eq!(
        resolve(
            &format!(r#"{{"statement":"root {WITNESS_ROOT}"}}"#),
            &evidence_metadata(&identity),
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::MissingClaimPredicate
    );
    assert_eq!(
        resolve(
            r#"{"verified":true,"actor_class":"DeadboltAnchorer"}"#,
            r#"{"verified":true,"summary":"verified"}"#,
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::MissingClaimPredicate
    );
}

#[test]
fn deadbolt_occurrence_unrelated_top_level_metadata_is_non_authoritative() {
    let identity = fixture_identity();
    let claim = json!({
        "claim_domain": "Occurrence/Inclusion",
        "verified": false,
        "deadbolt_occurrence": anchor_json(&identity),
    })
    .to_string();
    let evidence = json!({
        "verified": true,
        "admitted": true,
        "deadbolt_anchor_ref": anchor_json(&identity),
    })
    .to_string();

    assert!(matches!(
        resolve(&claim, &evidence, &fixture_index()),
        DeadboltOccurrenceContextResolution::Matched(_)
    ));
}

#[test]
fn deadbolt_occurrence_wrong_kind_or_domain_never_matches() {
    let identity = fixture_identity();
    let claim = claim_metadata(&identity);
    let evidence = evidence_metadata(&identity);
    let anchors = fixture_index();

    for kind in [
        EvidenceKind::ExternalSource,
        EvidenceKind::DeterministicVerification,
    ] {
        assert_eq!(
            resolve_deadbolt_occurrence_context(
                kind,
                ClaimDomain::OccurrenceInclusion,
                &claim,
                &evidence,
                &anchors,
            ),
            DeadboltOccurrenceContextResolution::WrongEvidenceKind
        );
    }
    assert_eq!(
        resolve_deadbolt_occurrence_context(
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::Interpretation,
            &claim,
            &evidence,
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::WrongClaimDomain
    );
}

#[test]
fn deadbolt_occurrence_actor_class_and_prose_are_not_resolver_inputs() {
    let identity = fixture_identity();
    let claim_metadata_json = claim_metadata(&identity);
    let evidence_metadata_json = evidence_metadata(&identity);
    let first_claim = TypedClaimNode {
        statement: format!("prose alone names {WITNESS_ROOT}"),
        scope_ref: "scope:test".into(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: claim_metadata_json.clone(),
    };
    let second_claim = TypedClaimNode {
        statement: "different prose".into(),
        scope_ref: "scope:test".into(),
        actor_class: "HumanRoot".into(),
        content_hash: String::new(),
        metadata_json: claim_metadata_json,
    };
    let first_evidence = TypedEvidenceNode {
        evidence_kind: "DeadboltAnchor".into(),
        summary: "claims verified authority".into(),
        scope_ref: "scope:test".into(),
        actor_class: "DeadboltAnchorer".into(),
        content_hash: WITNESS_ROOT.into(),
        metadata_json: evidence_metadata_json.clone(),
    };
    let second_evidence = TypedEvidenceNode {
        evidence_kind: "DeadboltAnchor".into(),
        summary: "different summary".into(),
        scope_ref: "scope:test".into(),
        actor_class: "SourceImporter".into(),
        content_hash: String::new(),
        metadata_json: evidence_metadata_json,
    };

    let anchors = fixture_index();
    let first = resolve(
        &first_claim.metadata_json,
        &first_evidence.metadata_json,
        &anchors,
    );
    let second = resolve(
        &second_claim.metadata_json,
        &second_evidence.metadata_json,
        &anchors,
    );

    assert_eq!(first, second);
    assert!(matches!(
        first,
        DeadboltOccurrenceContextResolution::Matched(_)
    ));
}

#[test]
fn deadbolt_occurrence_matching_unanchored_tuple_fails_with_other_anchor_present() {
    let changed = change_one_field(&fixture_identity(), IdentityField::RunId);

    assert_eq!(
        resolve(
            &claim_metadata(&changed),
            &evidence_metadata(&changed),
            &fixture_index(),
        ),
        DeadboltOccurrenceContextResolution::AnchorNotFound
    );
}

#[test]
fn deadbolt_occurrence_identity_values_are_not_normalized() {
    let anchored = fixture_identity();
    let anchors = fixture_index();

    let mut whitespace = anchored.clone();
    whitespace.run_id.push(' ');
    assert_eq!(
        resolve(
            &claim_metadata(&whitespace),
            &evidence_metadata(&whitespace),
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::AnchorNotFound
    );

    let mut case_changed = anchored;
    case_changed.canonicalization_profile = FOREIGN_PROFILE.to_uppercase();
    assert_eq!(
        resolve(
            &claim_metadata(&case_changed),
            &evidence_metadata(&case_changed),
            &anchors,
        ),
        DeadboltOccurrenceContextResolution::AnchorNotFound
    );
}

#[test]
fn deadbolt_occurrence_index_remains_separate_from_standing_view() {
    let reader = fixture_reader();
    let mut index = DeadboltAnchorIndex::new();
    let mut standing = StandingView::new();

    assert_eq!(reader.replay(&mut index).unwrap(), 2);
    assert_eq!(reader.replay(&mut standing).unwrap(), 2);
    assert_eq!(index.len(), 1);
    assert!(standing.is_structurally_empty());
    assert_eq!(standing.resolved_standing(RUN_ID), None);
}
