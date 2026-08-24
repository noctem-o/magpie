use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use magpie_log::signature_profile::{
    ProfiledSignatureVerifier, SignatureVerificationProfile, V_SIG_PROFILE_ID,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

const GOLDEN_KEY_HEX: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";

const ACCEPT_CASE_IDS: [&str; 17] = [
    "a21-d2-identity-r-equation-true",
    "n13-feff-escaped-string",
    "n13-feff-literal-string",
    "n30-final-crlf",
    "n30-final-lf",
    "n30-mixed-crlf-lf",
    "n30-no-final-terminator",
    "p1-golden-v1",
    "p2-deadbolt-anchor-v1",
    "p7-member-order-permutation",
    "p8-equivalent-string-escape",
    "p8-leading-trailing-space-tab",
    "p8-space-tab-json",
    "positive-complete-vocabulary",
    "u64-p1-timestamp-9007199254740993",
    "u64-p2-timestamp-9223372036854775808",
    "u64-p3-timestamp-18446744073709551615",
];

const EXTERNAL_KEY_CASE_IDS: [&str; 4] = [
    "a21-d1-order-two-forged-chain",
    "a21-s4-identity-key-s-zero",
    "a21-s5-identity-key-s-l-minus-one",
    "a21-t2-mixed-torsion-key",
];

const SIGNATURE_CASE_IDS: [(&str, usize); 16] = [
    ("a21-altered-message", 0),
    ("a21-altered-signature", 0),
    ("a21-r-identity-equation-false", 0),
    ("a21-r-mixed-torsion-cofactor", 0),
    ("a21-r-nondecompress", 0),
    ("a21-r-nonidentity-small-order", 0),
    ("a21-r-order-four-cofactor", 0),
    ("a21-r-x-zero-sign-one", 0),
    ("a21-r-y-equals-p", 0),
    ("a21-s-all-ff", 0),
    ("a21-s-equals-l", 0),
    ("a21-s-l-minus-one-equation-false", 0),
    ("a21-s-l-plus-one", 0),
    ("n28-signature-before-genesis", 0),
    ("n28-signature-before-payload", 0),
    ("n29-valid-prefix-bad-signature-later", 1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedRelationResult {
    Accept,
    ExternalKey,
    Signature { record_index: usize },
}

#[test]
fn profile_selection_is_exact_and_accessors_retain_primitive_coordinates() {
    let profile = SignatureVerificationProfile::from_identity(V_SIG_PROFILE_ID)
        .expect("the exact frozen identity must be supported");
    assert_eq!(
        profile,
        SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1
    );
    assert_eq!(profile.identity(), V_SIG_PROFILE_ID);

    for unsupported in [
        "",
        "latest",
        "MAGPIE-ED25519-CANONICAL-PRIME-SUBGROUP-V1",
        " magpie-ed25519-canonical-prime-subgroup-v1",
        "magpie-ed25519-canonical-prime-subgroup-v1 ",
        "magpie-ed25519-canonical-prime-subgroup-v1\n",
        "magpie-ed25519-canonical-prime-subgroup-v2",
        "unknown-profile",
    ] {
        assert!(
            SignatureVerificationProfile::from_identity(unsupported).is_err(),
            "unsupported identity unexpectedly selected: {unsupported:?}"
        );
    }

    let external_key = decode_hex::<32>(GOLDEN_KEY_HEX);
    let verifier = ProfiledSignatureVerifier::new(profile, external_key)
        .expect("the frozen golden key must be structurally admissible");
    assert_eq!(verifier.profile(), profile);
    assert_eq!(verifier.external_key_bytes(), &external_key);
}

/// V_sig relation only — not portable-verifier conformance.
///
/// This deliberately extracts already well-shaped key/hash/signature bytes
/// from the exact frozen artifacts. It does not implement framing, JSON-law,
/// duplicate-member, schema, lexical, integer, hex, or first-failure parser
/// semantics.
#[test]
fn frozen_37_case_adapter_is_relation_only() {
    let repository_root = repository_root();
    let manifest_path = repository_root.join("fixtures/verifier-language-v1/manifest.json");
    let manifest_bytes =
        std::fs::read(&manifest_path).expect("the frozen corpus manifest must exist");
    let manifest: Value =
        serde_json::from_slice(&manifest_bytes).expect("the frozen manifest must be JSON");

    assert_eq!(
        manifest["producing_context"]["signature_profile_identity"],
        V_SIG_PROFILE_ID
    );

    let exact_expected = exact_expected_relation_inventory();
    let cases = manifest["cases"]
        .as_array()
        .expect("the frozen manifest must contain cases");
    let selected: Vec<&Value> = cases
        .iter()
        .filter(|case| is_record_bearing_v_sig_relation_case(case))
        .collect();

    let observed_ids: BTreeSet<String> = selected
        .iter()
        .map(|case| required_str(case, "id").to_owned())
        .collect();
    let expected_ids: BTreeSet<String> = exact_expected.keys().map(|id| (*id).to_owned()).collect();
    assert_eq!(
        observed_ids, expected_ids,
        "the exact frozen 37-case V_sig relation inventory drifted"
    );
    assert_eq!(selected.len(), 37);
    assert_eq!(
        selected
            .iter()
            .filter(|case| case["expected"]["verdict"] == "ACCEPT")
            .count(),
        17
    );
    assert_eq!(
        selected
            .iter()
            .filter(|case| case["expected"]["class"] == "ExternalKey")
            .count(),
        4
    );
    assert_eq!(
        selected
            .iter()
            .filter(|case| case["expected"]["class"] == "Signature")
            .count(),
        16
    );

    let profile = SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1;
    for case in selected {
        let case_id = required_str(case, "id");
        let exact_result = exact_expected
            .get(case_id)
            .copied()
            .expect("selected case must be in the exact relation inventory");
        assert_manifest_result_matches(case_id, case, exact_result);

        let input_path = repository_root.join(required_str(case, "input_path"));
        let input_bytes =
            std::fs::read(&input_path).expect("selected frozen input bytes must exist");
        assert_eq!(
            hex::encode(Sha256::digest(&input_bytes)),
            required_str(case, "input_sha256"),
            "{case_id}: exact frozen input digest drifted"
        );

        let external_key = decode_hex::<32>(required_str(case, "external_verifying_key_hex"));
        let verifier_result = ProfiledSignatureVerifier::new(profile, external_key);
        if exact_result == ExpectedRelationResult::ExternalKey {
            assert!(
                verifier_result.is_err(),
                "{case_id}: expected external-key rejection"
            );
            continue;
        }

        let verifier = verifier_result
            .unwrap_or_else(|error| panic!("{case_id}: admissible key rejected: {error:?}"));
        let records = extract_well_shaped_records(&input_bytes, case_id);

        match exact_result {
            ExpectedRelationResult::Accept => {
                for (record_index, record) in records.iter().enumerate() {
                    let content_hash = record_hex::<32>(record, "hash", case_id, record_index);
                    let signature = record_hex::<64>(record, "signature", case_id, record_index);
                    verifier
                        .verify(&content_hash, &signature)
                        .unwrap_or_else(|error| {
                            panic!(
                                "{case_id}: record {record_index} unexpectedly rejected: {error:?}"
                            )
                        });
                }
                assert_eq!(
                    u64::try_from(records.len()).expect("test record count must fit u64"),
                    case["expected"]["event_count"]
                        .as_u64()
                        .expect("ACCEPT must carry event_count"),
                    "{case_id}: extracted record count drifted"
                );
            }
            ExpectedRelationResult::Signature {
                record_index: expected_index,
            } => {
                let first_rejection =
                    records
                        .iter()
                        .enumerate()
                        .find_map(|(record_index, record)| {
                            let content_hash =
                                record_hex::<32>(record, "hash", case_id, record_index);
                            let signature =
                                record_hex::<64>(record, "signature", case_id, record_index);
                            verifier
                                .verify(&content_hash, &signature)
                                .err()
                                .map(|_| record_index)
                        });
                assert_eq!(
                    first_rejection,
                    Some(expected_index),
                    "{case_id}: V_sig relation rejection coordinate drifted"
                );
            }
            ExpectedRelationResult::ExternalKey => unreachable!("handled before record extraction"),
        }
    }
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn exact_expected_relation_inventory() -> BTreeMap<&'static str, ExpectedRelationResult> {
    let mut expected = BTreeMap::new();
    for case_id in ACCEPT_CASE_IDS {
        assert_eq!(
            expected.insert(case_id, ExpectedRelationResult::Accept),
            None
        );
    }
    for case_id in EXTERNAL_KEY_CASE_IDS {
        assert_eq!(
            expected.insert(case_id, ExpectedRelationResult::ExternalKey),
            None
        );
    }
    for (case_id, record_index) in SIGNATURE_CASE_IDS {
        assert_eq!(
            expected.insert(case_id, ExpectedRelationResult::Signature { record_index },),
            None
        );
    }
    expected
}

fn is_record_bearing_v_sig_relation_case(case: &Value) -> bool {
    let key = required_str(case, "external_verifying_key_hex");
    let key_is_exact_lower_hex = key.len() == 64
        && key
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    let is_record_bearing = case["byte_properties"]["length"]
        .as_u64()
        .is_some_and(|length| length > 0);
    let is_relevant_result = case["expected"]["verdict"] == "ACCEPT"
        || case["expected"]["class"] == "ExternalKey"
        || case["expected"]["class"] == "Signature";

    key_is_exact_lower_hex && is_record_bearing && is_relevant_result
}

fn assert_manifest_result_matches(
    case_id: &str,
    case: &Value,
    exact_result: ExpectedRelationResult,
) {
    match exact_result {
        ExpectedRelationResult::Accept => {
            assert_eq!(case["expected"]["verdict"], "ACCEPT", "{case_id}");
            assert!(case["expected"]["class"].is_null(), "{case_id}");
        }
        ExpectedRelationResult::ExternalKey => {
            assert_eq!(case["expected"]["verdict"], "REJECT", "{case_id}");
            assert_eq!(case["expected"]["class"], "ExternalKey", "{case_id}");
            assert!(case["expected"]["record_index"].is_null(), "{case_id}");
        }
        ExpectedRelationResult::Signature { record_index } => {
            assert_eq!(case["expected"]["verdict"], "REJECT", "{case_id}");
            assert_eq!(case["expected"]["class"], "Signature", "{case_id}");
            assert_eq!(
                case["expected"]["record_index"].as_u64(),
                Some(u64::try_from(record_index).expect("test index must fit u64")),
                "{case_id}"
            );
        }
    }
}

fn extract_well_shaped_records(input_bytes: &[u8], case_id: &str) -> Vec<Value> {
    input_bytes
        .split(|byte| *byte == b'\n')
        .filter_map(|candidate| {
            let candidate = candidate.strip_suffix(b"\r").unwrap_or(candidate);
            if candidate.is_empty() {
                return None;
            }
            Some(
                serde_json::from_slice(candidate)
                    .unwrap_or_else(|error| panic!("{case_id}: extraction failed: {error}")),
            )
        })
        .collect()
}

fn record_hex<const LENGTH: usize>(
    record: &Value,
    member: &str,
    case_id: &str,
    record_index: usize,
) -> [u8; LENGTH] {
    let value = record[member].as_str().unwrap_or_else(|| {
        panic!("{case_id}: record {record_index} lacks string member {member:?}")
    });
    decode_hex(value)
}

fn required_str<'a>(value: &'a Value, member: &str) -> &'a str {
    value[member]
        .as_str()
        .unwrap_or_else(|| panic!("manifest member {member:?} must be a string"))
}

fn decode_hex<const LENGTH: usize>(value: &str) -> [u8; LENGTH] {
    assert_eq!(value.len(), LENGTH * 2);
    let mut decoded = [0_u8; LENGTH];
    hex::decode_to_slice(value, &mut decoded).expect("frozen value must be exact hexadecimal");
    decoded
}
