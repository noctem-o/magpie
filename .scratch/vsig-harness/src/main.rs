#[path = "../../../crates/magpie-log/src/vsig.rs"]
mod vsig;

use vsig::{verify_signature_with_profile, SignatureVerificationProfile, VsigRejection};

fn hex_to_bytes(h: &str) -> Result<Vec<u8>, String> {
    let h = h.trim();
    if h.len() % 2 != 0 {
        return Err(format!("odd hex len {}", h.len()));
    }
    let mut out = Vec::with_capacity(h.len() / 2);
    for i in (0..h.len()).step_by(2) {
        let b = u8::from_str_radix(&h[i..i + 2], 16)
            .map_err(|e| format!("bad hex at {}", i))?;
        out.push(b);
    }
    Ok(out)
}

fn reject_class(r: &VsigRejection) -> &'static str {
    match r {
        VsigRejection::ExternalKey => "ExternalKey",
        VsigRejection::Signature => "Signature",
    }
}

/// Hash field must be exactly 32 bytes (enclosing profile's contract);
/// V_sig consumes `[u8; 32]` by value.
fn hash32(rec: &serde_json::Value, ctx: &str) -> [u8; 32] {
    let b = hex_to_bytes(rec["hash"].as_str().unwrap_or_else(|| panic!("{ctx}: no hash")))
        .unwrap_or_else(|e| panic!("{ctx}: bad hash hex: {e}"));
    if b.len() != 32 {
        panic!("{ctx}: hash len {} != 32", b.len());
    }
    b.try_into().unwrap()
}

struct NamedVector {
    case_id: &'static str,
    label: &'static str,
    expected_verdict: &'static str,
    expected_class: Option<&'static str>,
}

fn main() {
    let base = std::env::var("HARNESS_BASE").unwrap();
    let manifest_path = format!("{}/fixtures/verifier-language-v1/manifest.json", base);
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();

    let mut cases: std::collections::HashMap<
        String,
        (String, String, Option<String>, Option<String>, Option<u64>),
    > = std::collections::HashMap::new();
    for c in manifest["cases"].as_array().unwrap().iter() {
        let id = c["id"].as_str().unwrap().to_string();
        let input_path = c["input_path"].as_str().unwrap().to_string();
        let key = c["external_verifying_key_hex"].as_str().unwrap().to_string();
        let ev = c["expected"]["verdict"].as_str().map(|s| s.to_string());
        let ec = c["expected"]["class"].as_str().map(|s| s.to_string());
        let eri = c["expected"]["record_index"].as_u64();
        cases.insert(id, (input_path, key, ev, ec, eri));
    }

    let profile = SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1;
    let mut pass = 0u32;
    let mut fail = 0u32;
    let mut fails: Vec<String> = Vec::new();

    let named: Vec<NamedVector> = vec![
        NamedVector { case_id: "a21-s-equals-l", label: "S1", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-s-all-ff", label: "S2", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-r-identity-equation-false", label: "S3", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-s4-identity-key-s-zero", label: "S4", expected_verdict: "REJECT", expected_class: Some("ExternalKey") },
        NamedVector { case_id: "a21-s5-identity-key-s-l-minus-one", label: "S5", expected_verdict: "REJECT", expected_class: Some("ExternalKey") },
        NamedVector { case_id: "a21-d1-order-two-forged-chain", label: "D1", expected_verdict: "REJECT", expected_class: Some("ExternalKey") },
        NamedVector { case_id: "a21-d2-identity-r-equation-true", label: "D2", expected_verdict: "ACCEPT", expected_class: None },
        NamedVector { case_id: "a21-t2-mixed-torsion-key", label: "T2", expected_verdict: "REJECT", expected_class: Some("ExternalKey") },
        NamedVector { case_id: "a21-altered-message", label: "M1", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-r-nondecompress", label: "R1", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-r-y-equals-p", label: "R2", expected_verdict: "REJECT", expected_class: Some("Signature") },
        NamedVector { case_id: "a21-r-x-zero-sign-one", label: "R3", expected_verdict: "REJECT", expected_class: Some("Signature") },
    ];

    for nv in &named {
        let (input_path, key, _ev, _ec, _eri) = match cases.get(nv.case_id) {
            Some(x) => x.clone(),
            None => { fails.push(format!("{}: case {} not in manifest", nv.label, nv.case_id)); fail += 1; continue; }
        };
        let full = format!("{}/{}", base, input_path);
        let data = std::fs::read(&full).unwrap_or_else(|e| panic!("{}: read {}: {}", nv.label, full, e));
        let recs: Vec<serde_json::Value> = data
            .split(|&b| b == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_str(std::str::from_utf8(l).unwrap()).unwrap())
            .collect();
        if recs.is_empty() {
            fails.push(format!("{} [{}]: input file has no records", nv.label, nv.case_id));
            fail += 1;
            println!("{:4}  {:>3}  {:<24} want={:<8} got=<no records>", "FAIL", nv.label, nv.case_id, nv.expected_verdict);
            continue;
        }
        let rec = &recs[0];
        let sig_hex = rec["signature"].as_str().unwrap().to_string();
        let ch = hash32(rec, nv.case_id);
        let sig = hex_to_bytes(&sig_hex).unwrap();
        let keyb = hex_to_bytes(&key).unwrap();

        let res = verify_signature_with_profile(&profile, &keyb, &sig, ch);
        let got_accept = res.is_ok();
        let got_class = res.as_ref().err().map(|e| reject_class(e));
        let want_accept = nv.expected_verdict == "ACCEPT";
        let want_class = nv.expected_class;

        let ok = if want_accept {
            got_accept
        } else {
            !got_accept && got_class == want_class
        };
        let status = if ok { "PASS" } else { "FAIL" };
        if ok { pass += 1; } else {
            fail += 1;
            fails.push(format!("{} [{}]: want {} {} got accept={} class={:?}", nv.label, nv.case_id, nv.expected_verdict, want_class.unwrap_or("-"), got_accept, got_class));
        }
        println!("{:4}  {:>3}  {:<24} want={:<8} got accept={:<5} class={:?}  ({} recs)",
            status, nv.label, nv.case_id, nv.expected_verdict, got_accept, got_class, recs.len());
    }

    // ---- golden-v1 (9 records) and deadbolt-anchor-v1 (2 records) ----
    check_chain("golden-v1", &base, "crates/magpie-log/testdata/golden-v1.jsonl", "p1-golden-v1", &mut pass, &mut fail, &mut fails, &profile);
    check_chain("deadbolt-anchor-v1", &base, "fixtures/deadbolt-anchor-v1/anchor-log.jsonl", "p2-deadbolt-anchor-v1", &mut pass, &mut fail, &mut fails, &profile);

    println!("\n=== A21 + golden + deadbolt: {} PASS, {} FAIL ===", pass, fail);
    for f in &fails { println!("  FAIL: {}", f); }
    if fail > 0 { std::process::exit(1); }
}

fn check_chain(
    name: &str, base: &str, rel: &str, case_id: &str,
    pass: &mut u32, fail: &mut u32, fails: &mut Vec<String>,
    profile: &SignatureVerificationProfile,
) {
    let manifest_path = format!("{}/fixtures/verifier-language-v1/manifest.json", base);
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    let mut key = String::new();
    for c in manifest["cases"].as_array().unwrap().iter() {
        if c["id"].as_str().unwrap() == case_id {
            key = c["external_verifying_key_hex"].as_str().unwrap().to_string();
        }
    }
    let keyb = hex_to_bytes(&key).unwrap();
    let path = format!("{}/{}", base, rel);
    let data = std::fs::read_to_string(&path).unwrap();
    let recs: Vec<serde_json::Value> = data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();

    let mut all_ok = true;
    for (i, rec) in recs.iter().enumerate() {
        let ch = hash32(rec, &format!("{name} rec {i}"));
        let sig = hex_to_bytes(rec["signature"].as_str().unwrap()).unwrap();
        if let Err(e) = verify_signature_with_profile(profile, &keyb, &sig, ch) {
            all_ok = false;
            *fail += 1;
            fails.push(format!("{} rec {}: reject {}", name, i, reject_class(&e)));
            println!("  {} rec {}: REJECT {}", name, i, reject_class(&e));
        }
    }
    if all_ok {
        *pass += 1;
        println!("  {:<20} all {} records ACCEPT", name, recs.len());
    } else {
        println!("  {:<20} FAILURES (see above)", name);
    }
}
