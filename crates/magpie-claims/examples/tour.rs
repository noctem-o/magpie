//! A 30-second tour of the spine. Run with: `cargo run --example tour -p magpie-claims`
//!
//! In real use the signing key is generated once with a CSPRNG and held by the
//! gate, e.g.:
//!
//! ```ignore
//! let signing_key = SigningKey::generate(&mut OsRng);
//! ```
//!
//! Here we use a fixed seed so the tour is deterministic.
//!
//! (Reconstructed 2026-07-02 to the documented flow — write → verify → replay →
//! drop → replay → byte-identical — after the original was lost in transit.)

use ed25519_dalek::SigningKey;
use magpie_claims::ClaimsView;
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

fn main() {
    let store = MemStore::new();

    // ── 1. WRITE ─ the gate (sole holder of the LogWriter) appends events ──
    // Opening the empty store writes the genesis event (seq 0) automatically:
    // it pins the canonicalization profile and the verifying key.
    println!("── 1. write ──");
    {
        let mut t = 0u64;
        let mut gate = LogWriter::open_with_clock(
            store.clone(),
            SigningKey::from_bytes(&SEED),
            Box::new(move || {
                t += 1;
                t
            }),
        )
        .unwrap();

        let events = [
            (
                Provenance::new("george", "manuscript"),
                Payload::ClaimAsserted {
                    claim_id: "thm2".into(),
                    statement:
                        "Four exponentially small spectral scales share one semiclassical exponent."
                            .into(),
                    status: Status::Conjectured,
                },
            ),
            (
                Provenance::new("claude", "verification"),
                Payload::EvidenceRecorded {
                    claim_id: "thm2".into(),
                    summary: "Independent numeric check to ~1e-15 relative error.".into(),
                },
            ),
            (
                Provenance::new("george", "manuscript"),
                Payload::ClaimStatusChanged {
                    claim_id: "thm2".into(),
                    from: Status::Conjectured,
                    to: Status::Settled,
                    reason: "Theorem 2 proven and independently reproduced (N=64..192).".into(),
                },
            ),
            (
                Provenance::new("george", "manuscript"),
                Payload::ClaimAsserted {
                    claim_id: "f5".into(),
                    statement: "Uniform Riccati sandwich (open problem F5).".into(),
                    status: Status::Open,
                },
            ),
        ];

        println!("  seq 0 written on open: genesis (profile + verifying key)");
        for (prov, payload) in events {
            let ev = gate.append(prov, payload).unwrap();
            println!("  appended seq {}  hash {:?}", ev.core.seq, ev.hash);
        }
    } // the write capability goes out of scope — nothing below can append

    // ── 2. VERIFY ─ a reader validates the whole chain ─────────────────────
    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    let n = reader.verify_chain().unwrap();
    println!("── 2. verify ──\n  chain valid: {n} events, every hash and signature checked");

    // ── 3. REPLAY ─ fold the log into a derived projection ────────────────
    let mut view = ClaimsView::new();
    reader.replay(&mut view).unwrap();
    println!("── 3. replay ──");
    for (id, claim) in &view.claims {
        println!(
            "  {id}: {:?}  (evidence: {}, transitions: {})",
            claim.status,
            claim.evidence.len(),
            claim.transitions
        );
    }
    let bytes_before = view.canonical_bytes();

    // ── 4. DROP ─ throw away every derived byte ───────────────────────────
    drop(view);
    println!("── 4. drop ──\n  all derived state destroyed; only the log remains");

    // ── 5. REPLAY AGAIN ─ regenerate from the log alone ───────────────────
    let mut regrown = ClaimsView::new();
    reader.replay(&mut regrown).unwrap();
    let bytes_after = regrown.canonical_bytes();

    assert_eq!(bytes_before, bytes_after);
    println!(
        "── 5. regenerate ──\n  byte-for-byte identical ({} bytes). \
         Conserve the log. Derive the rest.",
        bytes_after.len()
    );
}
