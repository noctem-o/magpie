//! Regenerates `testdata/golden-v1.jsonl` and prints the expected values for
//! `tests/golden.rs`. Run only for a reviewed format-surface change: either
//! when cutting a new canonicalization profile, or when appending additive
//! payload tags whose old fixture records must remain byte-for-byte unchanged.
//! For an existing committed fixture, the golden file and hardcoded hashes are
//! the ground truth, and this program must reproduce them exactly.
//!
//! `cargo run --example regen_golden -p magpie-log`

use ed25519_dalek::SigningKey;
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

fn main() {
    let store = MemStore::new();
    {
        let mut t = 0u64;
        let mut w = LogWriter::<MemStore>::open_with_clock(
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
        // seq 6 — ADR-0002 typed claim assertion coverage.
        w.append(
            Provenance::new("george", "adr-0002"),
            Payload::ClaimAssertedV2 {
                claim_id: "claim-v2-thm2".into(),
                statement:
                    "Theorem 2 standing is governed by typed evidence and deterministic replay."
                        .into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "HumanRoot".into(),
                content_hash: "".into(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
        // seq 7 — ADR-0002 typed evidence coverage.
        w.append(
            Provenance::new("deadbolt", "adr-0002"),
            Payload::EvidenceRegistered {
                evidence_id: "ev-deadbolt-anchor-0001".into(),
                evidence_kind: "DeadboltAnchor".into(),
                summary:
                    "Deadbolt anchor can prove occurrence and inclusion, not interpretation truth."
                        .into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "DeadboltAnchorer".into(),
                content_hash: "851d2a8f265e21192c4b1f1ff3bee2a8dc6305a848160412c74b66b74a909141"
                    .into(),
                metadata_json: "{\"cites_segment\":\"run-0001\"}".into(),
            },
        )
        .unwrap();
        // seq 8 — ADR-0002 typed justification edge coverage.
        w.append(
            Provenance::new("george", "adr-0002"),
            Payload::JustificationEdgeRecorded {
                edge_id: "edge-0001".into(),
                edge_kind: "supports".into(),
                source_id: "ev-deadbolt-anchor-0001".into(),
                target_id: "claim-v2-thm2".into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "HumanRoot".into(),
                rationale:
                    "The evidence supports occurrence/inclusion only; interpretation remains governed by StandingView ceilings."
                        .into(),
                metadata_json: "{}".into(),
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
