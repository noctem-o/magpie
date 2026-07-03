//! Regenerates `testdata/golden-v1.jsonl` and prints the expected values for
//! `tests/golden.rs`. Run **only** when cutting a NEW canonicalization
//! profile — for an existing profile, the committed golden file and the
//! hardcoded hashes are the ground truth, and this program must reproduce
//! them exactly.
//!
//! `cargo run --example regen_golden -p magpie-log`

use ed25519_dalek::SigningKey;
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

fn main() {
    let store = MemStore::new();
    {
        let mut t = 0u64;
        let mut w = LogWriter::open_with_clock(
            store.clone(),
            SigningKey::from_bytes(&SEED),
            Box::new(move || {
                t += 1;
                t
            }),
        )
        .unwrap();
        w.append(
            Provenance::new("george", "manuscript"),
            Payload::ClaimAsserted {
                claim_id: "thm2".into(),
                statement:
                    "Four exponentially small spectral scales share one semiclassical exponent."
                        .into(),
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
        // seq 5 — anchor coverage. The root is a deterministic fixture value
        // (sha256 of a fixed string), formatted exactly as deadbolt surfaces
        // roots: 64 lowercase hex chars.
        w.append(
            Provenance::new("deadbolt", "kernel-witness"),
            Payload::SegmentAnchored {
                bundle_kind: "kernel-decision-witness".into(),
                witness_root: {
                    use sha2::{Digest, Sha256};
                    hex::encode(Sha256::digest(b"fixture-segment-0"))
                },
                witness_algorithm: "sha256".into(),
                canonicalization_profile: "phase5-interim-jcs-like-v1".into(),
                run_id: "run-0001".into(),
            },
        )
        .unwrap();
    }

    // Write the fixture file.
    let records = store.records();
    let mut out = Vec::new();
    for r in &records {
        out.extend_from_slice(r);
        out.push(b'\n');
    }
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/golden-v1.jsonl");
    std::fs::write(path, &out).unwrap();
    println!("wrote {path} ({} records)", records.len());

    // Print the values tests/golden.rs pins.
    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    println!("\nEXPECTED_HASHES:");
    for ev in reader.events().unwrap() {
        println!("    \"{}\", // seq {}", ev.hash.to_hex(), ev.core.seq);
    }
    println!("\nEXPECTED_SIGS:");
    for ev in reader.events().unwrap() {
        println!("    \"{}\", // seq {}", ev.signature.to_hex(), ev.core.seq);
    }
    for ev in reader.events().unwrap() {
        if ev.core.seq == 0 || ev.core.seq == 3 {
            println!(
                "\nCANONICAL_BYTES seq {} ({} bytes):",
                ev.core.seq,
                ev.core.canonical_bytes().len()
            );
            println!("{}", hex::encode(ev.core.canonical_bytes()));
        }
    }
}
