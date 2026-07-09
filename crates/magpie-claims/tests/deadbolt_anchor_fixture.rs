use magpie_claims::{ClaimsView, StandingView};
use magpie_log::{LogReader, MemStore, VerifyingKey};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/anchor-log.jsonl"
);
const VERIFYING_KEY_HEX: &str = "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";

fn fixture_store() -> MemStore {
    let data = std::fs::read(FIXTURE_PATH).expect("portable anchor fixture must be committed");
    MemStore::from_records(
        data.split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect(),
    )
}

fn verifying_key() -> VerifyingKey {
    VerifyingKey::from_bytes(&decode_hex_32(VERIFYING_KEY_HEX))
        .expect("fixture verifying key must be valid")
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let mut out = [0u8; 32];
    for (i, slot) in out.iter_mut().enumerate() {
        let hi = hex_nibble(value.as_bytes()[i * 2]);
        let lo = hex_nibble(value.as_bytes()[i * 2 + 1]);
        *slot = (hi << 4) | lo;
    }
    out
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("fixture hex must be lowercase"),
    }
}

#[test]
fn portable_anchor_fixture_does_not_create_claim_or_standing_authority() {
    let reader = LogReader::open(fixture_store(), verifying_key());

    let mut claims = ClaimsView::new();
    assert_eq!(reader.replay(&mut claims).unwrap(), 2);
    assert!(claims.is_empty());

    let mut standing = StandingView::new();
    assert_eq!(reader.replay(&mut standing).unwrap(), 2);
    assert!(standing.is_empty());
    assert!(standing.is_structurally_empty());
    assert_eq!(standing.claim_len(), 0);
    assert_eq!(standing.typed_claim_len(), 0);
    assert_eq!(standing.typed_evidence_len(), 0);
    assert_eq!(standing.justification_edge_len(), 0);
    assert_eq!(standing.resolved_standing("portable-run-0001"), None);
}
