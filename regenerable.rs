use ed25519_dalek::SigningKey;
use magpie_claims::ClaimsView;
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut t = 0u64;
    LogWriter::open_with_clock(store, SigningKey::from_bytes(&SEED), Box::new(move || {
        t += 1;
        t
    }))
    .unwrap()
}

/// The thesis, as a test: a projection is fully regenerable from the log. Build
/// it, drop ALL derived state, rebuild from the log alone — and get back the
/// exact same bytes.
#[test]
fn projection_is_byte_identical_after_dropping_all_derived_state() {
    let store = MemStore::new();

    // 1. The gate writes the manuscript's claims into the log (the only writer).
    {
        let mut w = writer(store.clone());
        w.append(
            Provenance::new("george", "manuscript"),
            Payload::ClaimAsserted {
                claim_id: "thm2".into(),
                statement: "Four exponentially small spectral scales share one semiclassical exponent.".into(),
                status: Status::Conjectured,
            },
        )
        .unwrap();
        w.append(
            Provenance::new("claude", "verification"),
            Payload::EvidenceRecorded {
                claim_id: "thm2".into(),
                summary: "Independent numeric check to ~1e-15 relative error.".into(),
            },
        )
        .unwrap();
        w.append(
            Provenance::new("george", "manuscript"),
            Payload::ClaimStatusChanged {
                claim_id: "thm2".into(),
                from: Status::Conjectured,
                to: Status::Settled,
                reason: "Theorem 2 proven and independently reproduced (N=64..192).".into(),
            },
        )
        .unwrap();
        w.append(
            Provenance::new("george", "manuscript"),
            Payload::ClaimAsserted {
                claim_id: "f5".into(),
                statement: "Uniform Riccati sandwich (open problem F5).".into(),
                status: Status::Open,
            },
        )
        .unwrap();
    } // the writer — the capability — goes out of scope here

    let vk = SigningKey::from_bytes(&SEED).verifying_key();
    let reader = LogReader::open(store.clone(), vk);

    // 2. Build projection A.
    let mut view_a = ClaimsView::new();
    let n = reader.replay(&mut view_a).unwrap();
    assert_eq!(n, 4);
    let bytes_a = view_a.canonical_bytes();

    // The fold did the right thing: evidence moved thm2 to Settled; F5 stays Open.
    assert_eq!(view_a.get("thm2").unwrap().status, Status::Settled);
    assert_eq!(view_a.get("thm2").unwrap().evidence.len(), 1);
    assert_eq!(view_a.get("thm2").unwrap().transitions, 1);
    assert_eq!(view_a.get("f5").unwrap().status, Status::Open);

    // 3. DROP all derived state. Only the log remains.
    drop(view_a);

    // 4. Rebuild from scratch.
    let mut view_b = ClaimsView::new();
    reader.replay(&mut view_b).unwrap();
    let bytes_b = view_b.canonical_bytes();

    // 5. The thesis holds: byte-for-byte identical.
    assert_eq!(bytes_a, bytes_b, "the projection must be regenerable from the log alone");
}
