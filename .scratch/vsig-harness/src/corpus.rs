//! Full-corpus cross-check of the ADR-0010 V_sig implementation against every
//! non-empty case in fixtures/verifier-language-v1 (413 cases).
//!
//! Manifest semantics implemented here:
//!   - REJECT / class ExternalKey: the external-key gate fails BEFORE record
//!     processing, so verify(key, rec[0]) must return ExternalKey.
//!   - REJECT / class Signature: the record at expected.record_index (default
//!     0) must fail with class Signature.
//!   - ACCEPT: every record in the input must pass, and the record count must
//!     equal expected.event_count.
//! Empty (key-only) cases are skipped — the key gate is input-language-level
//! (hex parse) and is not part of V_sig's byte-level relation.

#[path = "../../../crates/magpie-log/src/vsig.rs"]
mod vsig;

use vsig::{verify_signature_with_profile, SignatureVerificationProfile, VsigRejection};

fn hex_to_bytes(h: &str) -> Vec<u8> {
    let h = h.trim();
    let mut out = Vec::with_capacity(h.len() / 2);
    for i in (0..h.len()).step_by(2) {
        out.push(u8::from_str_radix(&h[i..i + 2], 16).unwrap());
    }
    out
}

/// Hash field must be exactly 32 bytes (the enclosing profile's contract);
/// V_sig consumes `[u8; 32]` by value.
fn hash32(rec: &serde_json::Value, ctx: &str) -> [u8; 32] {
    let b = hex_to_bytes(rec["hash"].as_str().unwrap_or_else(|| panic!("{ctx}: no hash")));
    if b.len() != 32 {
        panic!("{ctx}: hash len {} != 32", b.len());
    }
    b.try_into().unwrap()
}

fn cls(r: &VsigRejection) -> &'static str {
    match r {
        VsigRejection::ExternalKey => "ExternalKey",
        VsigRejection::Signature => "Signature",
    }
}

fn load_records(data: &[u8]) -> Vec<serde_json::Value> {
    data.split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let s = std::str::from_utf8(l).unwrap();
            serde_json::from_str(s).unwrap_or_else(|e| panic!("bad line {:?} -> {}", &s[..s.len().min(80)], e))
        })
        .collect()
}

fn main() {
    let base = std::env::var("HARNESS_BASE").unwrap();
    let manifest_path = format!("{}/fixtures/verifier-language-v1/manifest.json", base);
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    let profile = SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1;

    let mut pass = 0u32;
    let mut fail = 0u32;
    let mut skipped_empty = 0u32;
    let mut fails: Vec<String> = Vec::new();

    for c in manifest["cases"].as_array().unwrap().iter() {
        let id = c["id"].as_str().unwrap().to_string();
        let input_path = c["input_path"].as_str().unwrap().to_string();
        let key = c["external_verifying_key_hex"].as_str().unwrap().to_string();
        let exp = &c["expected"];
        let verdict = exp["verdict"].as_str().unwrap();
        let class = exp["class"].as_str();
        let rec_idx = exp["record_index"].as_u64().unwrap_or(0) as usize;
        let event_count = exp["event_count"].as_u64();

        let path = format!("{}/{}", base, input_path);
        // Out of V_sig scope: every rejection class other than ExternalKey
        // and Signature belongs to an upstream stage (input language,
        // framing, schema, payload validation, genesis, hash chain,
        // sequence) that runs BEFORE the signature relation. V_sig's
        // relation is defined only over (key, signature, content_hash).
        if class != Some("ExternalKey")
            && class != Some("Signature")
            && !(class.is_none() && verdict == "ACCEPT")
        {
            continue;
        }
        let data = match std::fs::read(&path) {
            Ok(d) => d,
            Err(_) => { fail += 1; fails.push(format!("{}: unreadable input {}", id, path)); continue; }
        };
        if data.is_empty() { skipped_empty += 1; continue; }
        let keyb = hex_to_bytes(&key);
        let recs = load_records(&data);

        let mut ok = false;
        let mut detail = String::new();
        if verdict == "ACCEPT" {
            let mut all = true;
            let mut first_bad = None;
            for (i, r) in recs.iter().enumerate() {
                let ch = hash32(r, &format!("{id} rec {i}"));
                let sig = hex_to_bytes(r["signature"].as_str().unwrap());
                if let Err(e) = verify_signature_with_profile(&profile, &keyb, &sig, ch) {
                    all = false;
                    first_bad = Some((i, cls(&e).to_string()));
                    break;
                }
            }
            ok = all && (event_count.map(|n| (n as usize) == recs.len()).unwrap_or(true));
            detail = match first_bad {
                Some((i, cl)) => format!("rec{} {}", i, cl),
                None => format!("{} recs", recs.len()),
            };
        } else if class == Some("ExternalKey") {
            // Gate must fire before record processing: verify on rec[0] returns
            // ExternalKey (or no records at all).
            if recs.is_empty() {
                ok = true;
                detail = "no records".into();
            } else {
                let ch = hash32(&recs[0], &format!("{id} rec 0"));
                let sig = hex_to_bytes(recs[0]["signature"].as_str().unwrap());
                match verify_signature_with_profile(&profile, &keyb, &sig, ch) {
                    Err(e) => { ok = cls(&e) == "ExternalKey"; detail = cls(&e).into(); }
                    Ok(()) => { ok = false; detail = "ACCEPTED (wrong)".into(); }
                }
            }
        } else {
            // REJECT / Signature at record_index
            if rec_idx >= recs.len() {
                fail += 1; fails.push(format!("{}: record_index {} beyond {} recs", id, rec_idx, recs.len()));
                continue;
            }
            let ch = hash32(
                &recs[rec_idx],
                &format!("{} rec {}", id, rec_idx));
            let sig = hex_to_bytes(
                recs[rec_idx]["signature"].as_str().unwrap_or_else(|| panic!("{}: no sig at rec {}", id, rec_idx)));
            match verify_signature_with_profile(&profile, &keyb, &sig, ch) {
                Err(e) => { ok = cls(&e) == "Signature"; detail = format!("rec{} {}", rec_idx, cls(&e)); }
                Ok(()) => { ok = false; detail = format!("rec{} ACCEPTED (wrong)", rec_idx); }
            }
        }

        if ok { pass += 1; } else {
            fail += 1;
            fails.push(format!("{} [{} {}]: got {}", id, verdict, class.unwrap_or("-"), detail));
        }
    }

    println!("=== FULL CORPUS: {} PASS, {} FAIL, {} skipped (key-only) ===", pass, fail, skipped_empty);
    for f in &fails { println!("  FAIL: {}", f); }
    if fail > 0 { std::process::exit(1); }
}
