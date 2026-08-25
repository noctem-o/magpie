//! Non-normative fuzz/metamorphic assurance for the merged portable verifier.
//!
//! Everything here drives the SAME crate-internal production path used by the
//! frozen 432-case conformer tests
//! ([`super::conformer::verify_complete_history`]). It adds no parser, no
//! oracle, no production API surface, and no semantic decision: every expected
//! relationship is derived from the frozen portable input-language contract,
//! FORMAT, and Accepted ADR-0010 — never from production diagnostics, and the
//! manifest expectation is never allowed to choose an execution path.
//!
//! Metamorphic discipline: execute the original input first, apply one
//! contract-justified transformation, execute the transformed input through
//! the same verifier, then compare the relationship the frozen contract
//! requires for that transformation.

use std::path::{Path, PathBuf};

use serde_json::Value;

use super::conformer::{
    verify_complete_history, verify_complete_history_with_trace, CompleteHistoryError,
    CompleteHistoryOutcome, PortableRejectionClass,
};
use crate::signature_profile::V_SIG_PROFILE_ID;

const GOLDEN_KEY: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";
/// A different admissible prime-subgroup key (frozen K15 material).
const OTHER_ADMISSIBLE_KEY: &str =
    "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d2ac";

// P2-1: explicit campaign bounds. These constants are the single source of
// truth for every input-size claim in the PR description, and the campaign
// asserts the observed maxima against them mechanically.
const CAMPAIGN_SEED: u64 = 0xA55E_171A;
/// Default `cargo test` smoke size (200 inputs per strategy).
const SMOKE_LOGICAL_INPUTS: usize = 600;
/// Full extended campaign: 12,000 logical inputs / 24,000 conformer executions.
const EXTENDED_LOGICAL_INPUTS: usize = 12_000;
const STRATEGY_COUNT: usize = 3;
/// Strategy 0 (uniform random): length is `0..UNIFORM_MAX_LEN` bytes.
const UNIFORM_MAX_LEN: usize = 600;
/// Strategy 1 (mutated frozen ACCEPT histories): starts from whole frozen
/// seeds (largest is ~17 KB) and applies `1..=MAX_MUTATIONS_PER_INPUT`
/// byte flip/delete/insert operations, so its bound is
/// `largest_frozen_accept_seed + MAX_MUTATIONS_PER_INPUT` — asserted, not
/// hard-coded here.
const MAX_MUTATIONS_PER_INPUT: usize = 8;
/// Strategy 2 (grammar-ish fragments): `0..FRAGMENT_MAX_PIECES` pieces.
const FRAGMENT_MAX_PIECES: usize = 12;
/// Longest piece in the fragment table (the large integer token).
const FRAGMENT_MAX_PIECE_LEN: usize = 20;

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_manifest() -> Value {
    let bytes =
        std::fs::read(repository_root().join("fixtures/verifier-language-v1/manifest.json"))
            .expect("frozen manifest must exist");
    serde_json::from_slice(&bytes).expect("frozen manifest must parse")
}

/// Every frozen ACCEPT case as a metamorphic seed: (case_id, key, exact bytes).
fn accept_seeds() -> Vec<(String, String, Vec<u8>)> {
    let root = repository_root();
    let manifest = load_manifest();
    let mut seeds = Vec::new();
    for case in manifest["cases"].as_array().expect("manifest cases") {
        if case["expected"]["verdict"] == "ACCEPT" {
            let id = case["id"].as_str().expect("case id").to_string();
            let key = case["external_verifying_key_hex"]
                .as_str()
                .expect("case key")
                .to_string();
            let path = root.join(case["input_path"].as_str().expect("input path"));
            let input = std::fs::read(path).unwrap_or_else(|e| panic!("{id}: read failed: {e}"));
            seeds.push((id, key, input));
        }
    }
    assert!(seeds.len() >= 10, "expected a meaningful ACCEPT seed pool");
    seeds
}

fn seed_by_id(seeds: &[(String, String, Vec<u8>)], id: &str) -> (String, String, Vec<u8>) {
    seeds
        .iter()
        .find(|(case_id, _, _)| case_id == id)
        .unwrap_or_else(|| panic!("seed case {id} missing"))
        .clone()
}

/// Deterministic xorshift64* — fixed seeds make every campaign reproducible.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: usize) -> usize {
        if bound == 0 {
            0
        } else {
            (self.next_u64() % bound as u64) as usize
        }
    }
}

fn run(profile: &str, key: &str, input: &[u8]) -> CompleteHistoryOutcome {
    verify_complete_history(profile, key, input).expect("campaign input must stay governed")
}

fn reject_class_of(profile: &str, key: &str, input: &[u8]) -> PortableRejectionClass {
    match run(profile, key, input) {
        CompleteHistoryOutcome::Reject(rejection) => rejection.class(),
        other => panic!("expected a governed rejection, got {other:?}"),
    }
}

fn accept_of(profile: &str, key: &str, input: &[u8]) -> (u64, crate::ContentHash) {
    match run(profile, key, input) {
        CompleteHistoryOutcome::Accept(accepted) => (accepted.event_count(), accepted.tip()),
        other => panic!("expected ACCEPT, got {other:?}"),
    }
}

fn traced(key: &str, input: &[u8]) -> (CompleteHistoryOutcome, Vec<crate::ContentHash>) {
    verify_complete_history_with_trace(V_SIG_PROFILE_ID, key, input).expect("no operational error")
}

// ---------------------------------------------------------------------------
// Property family A — totality, containment, determinism.
// ---------------------------------------------------------------------------

/// Shared deterministic totality/containment/determinism campaign used by BOTH
/// the default smoke test and the explicit extended test (P2-6): one engine,
/// one property body, no semantic fork between modes.
fn run_totality_campaign(logical_inputs: usize) {
    let seeds = accept_seeds();
    let mutated_upper_bound = seeds
        .iter()
        .map(|(_, _, bytes)| bytes.len())
        .max()
        .expect("non-empty ACCEPT seed pool")
        + MAX_MUTATIONS_PER_INPUT;
    let mut rng = Rng::new(CAMPAIGN_SEED);
    let mut max_len = [0_usize; STRATEGY_COUNT];
    let mut accepts = 0_usize;
    let mut empty_accepts = 0_usize;
    let mut non_empty_accepts = 0_usize;
    let mut executed = 0_usize;

    // Strategy-indexed generation is the clearest shape here; the lint fires
    // only because of the bookkeeping array, so it is explicitly allowed.
    #[allow(clippy::needless_range_loop)]
    for strategy in 0..STRATEGY_COUNT {
        for _ in 0..logical_inputs / STRATEGY_COUNT {
            let input: Vec<u8> = match strategy {
                // Uniform random bytes, `0..UNIFORM_MAX_LEN`.
                0 => {
                    let len = rng.below(UNIFORM_MAX_LEN);
                    (0..len).map(|_| (rng.next_u64() & 0xFF) as u8).collect()
                }
                // Whole frozen-ACCEPT histories (up to ~17 KB) with
                // `1..=MAX_MUTATIONS_PER_INPUT` flip/cut/insert mutations.
                1 => {
                    let (_, _, base) = &seeds[rng.below(seeds.len())];
                    let mut bytes = base.clone();
                    for _ in 0..(1 + rng.below(MAX_MUTATIONS_PER_INPUT)) {
                        if bytes.is_empty() {
                            break;
                        }
                        match rng.below(3) {
                            0 => {
                                let at = rng.below(bytes.len());
                                bytes[at] = (rng.next_u64() & 0xFF) as u8;
                            }
                            1 => {
                                let at = rng.below(bytes.len());
                                bytes.remove(at);
                            }
                            _ => {
                                let at = rng.below(bytes.len() + 1);
                                bytes.insert(at, (rng.next_u64() & 0xFF) as u8);
                            }
                        }
                    }
                    bytes
                }
                // Grammar-ish fragment assembly around known boundaries.
                _ => {
                    let pieces: [&[u8]; 11] = [
                        b"{\"core\":{\"seq\":",
                        b"18446744073709551616",
                        b"\"hash\":\"",
                        b"zz",
                        b"\"signature\":\"",
                        br#""\uD800""#,
                        b"}",
                        b"\xef\xbb\xbf",
                        b"\r",
                        b"\n",
                        b" ",
                    ];
                    debug_assert_eq!(
                        FRAGMENT_MAX_PIECE_LEN,
                        pieces.iter().map(|p| p.len()).max().unwrap_or(0),
                        "fragment piece bound drifted from the table"
                    );
                    let mut bytes = Vec::new();
                    for _ in 0..rng.below(FRAGMENT_MAX_PIECES) {
                        bytes.extend_from_slice(pieces[rng.below(pieces.len())]);
                    }
                    bytes
                }
            };
            max_len[strategy] = max_len[strategy].max(input.len());

            let first = verify_complete_history(V_SIG_PROFILE_ID, GOLDEN_KEY, &input);
            // Containment: every outcome is typed — one of the ten governed
            // classes, an ACCEPT, or one of the documented operational
            // families. No panic, no abort, no third shape.
            match &first {
                Ok(CompleteHistoryOutcome::Accept(_)) => {
                    accepts += 1;
                    if input.is_empty() {
                        empty_accepts += 1;
                    } else {
                        non_empty_accepts += 1;
                    }
                }
                Ok(CompleteHistoryOutcome::Reject(rejection)) => {
                    assert!(matches!(
                        rejection.class(),
                        PortableRejectionClass::ExternalKey
                            | PortableRejectionClass::Framing
                            | PortableRejectionClass::JsonSyntax
                            | PortableRejectionClass::Schema
                            | PortableRejectionClass::Sequence
                            | PortableRejectionClass::PreviousLink
                            | PortableRejectionClass::ContentHash
                            | PortableRejectionClass::Signature
                            | PortableRejectionClass::PayloadValidation
                            | PortableRejectionClass::Genesis
                    ));
                }
                Err(error) => {
                    assert!(matches!(
                        error,
                        CompleteHistoryError::Frontend(_)
                            | CompleteHistoryError::EventCountExhausted
                    ));
                }
            }
            // Determinism: identical bytes must produce the identical verdict.
            let second = verify_complete_history(V_SIG_PROFILE_ID, GOLDEN_KEY, &input);
            assert_eq!(first, second, "nondeterministic verdict at {executed}");
            executed += 1;
        }
    }

    // Mechanically asserted per-strategy bounds (P2-1): prose about input
    // sizes cannot drift from the generator because these hold on every run.
    assert!(
        max_len[0] <= UNIFORM_MAX_LEN,
        "uniform strategy produced {} > {} bytes",
        max_len[0],
        UNIFORM_MAX_LEN
    );
    assert!(
        max_len[1] <= mutated_upper_bound,
        "mutation strategy produced {} > {} bytes",
        max_len[1],
        mutated_upper_bound
    );
    assert!(
        max_len[2] <= FRAGMENT_MAX_PIECES * FRAGMENT_MAX_PIECE_LEN,
        "fragment strategy produced {} > {} bytes",
        max_len[2],
        FRAGMENT_MAX_PIECES * FRAGMENT_MAX_PIECE_LEN
    );

    // Honest accounting (P2-1): most random-input ACCEPTs are the empty
    // history; semantic depth comes from families B-J, not this count.
    eprintln!(
        "assurance A[{logical_inputs} logical / {} executions]: accepts {accepts} \
         (empty-history {empty_accepts}, non-empty {non_empty_accepts}); \
         observed max input lengths uniform={} mutated={max_mutated} fragments={}; \
         zero panics, every verdict typed and deterministic",
        2 * executed,
        max_len[0],
        max_len[2],
        max_mutated = max_len[1],
    );
}

/// Bounded default smoke campaign covering ALL three generation strategies;
/// cheap enough to run in ordinary `cargo test` (P2-6).
#[test]
fn assurance_a_smoke_totality_containment_and_determinism() {
    run_totality_campaign(SMOKE_LOGICAL_INPUTS);
}

/// Full extended deterministic campaign: 12,000 logical inputs executed twice
/// each (24,000 production-conformer runs) across all three strategies.
///
/// Run explicitly with:
/// `cargo test -p magpie-log --locked --lib assurance_a_extended -- --ignored`
#[test]
#[ignore = "extended 12k-input campaign (~4 minutes locally); run explicitly when finalizing assurance evidence"]
fn assurance_a_extended_totality_containment_and_determinism() {
    run_totality_campaign(EXTENDED_LOGICAL_INPUTS);
}

// ---------------------------------------------------------------------------
// Property family B — formatting-preserving transformations keep ACCEPT.
// ---------------------------------------------------------------------------

/// Insert one space after every structural ':', ',', '{', '}' byte outside
/// JSON strings (escape-aware), which is insignificant JSON whitespace under
/// the frozen lexical law.
fn spaces_after_structurals(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() + 32);
    let mut in_string = false;
    let mut escaped = false;
    for &byte in input {
        out.push(byte);
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b':' | b',' | b'{' | b'}' => out.push(b' '),
            _ => {}
        }
    }
    out
}

#[test]
fn assurance_b_equivalent_spellings_preserve_accept_count_and_tip() {
    let seeds = accept_seeds();

    for (id, key, input) in &seeds {
        let baseline = accept_of(V_SIG_PROFILE_ID, key, input);

        // B1: added structural whitespace.
        let spaced = spaces_after_structurals(input);
        assert_eq!(
            accept_of(V_SIG_PROFILE_ID, key, &spaced),
            baseline,
            "{id}: whitespace variant changed the result"
        );

        // B2: mixed LF/CRLF terminators are explicitly equivalent framing.
        // Convert only bare LFs — a CR-aware pass leaves existing CRLF
        // terminators untouched (a naive replace would spell `\r\r\n`, which
        // the frozen law rightly rejects as a lone-CR framing violation).
        if input.contains(&b'\n') {
            let mut crlf = Vec::with_capacity(input.len() + 8);
            let mut previous = 0_u8;
            for &byte in input {
                if byte == b'\n' && previous != b'\r' {
                    crlf.push(b'\r');
                }
                crlf.push(byte);
                previous = byte;
            }
            assert_eq!(
                accept_of(V_SIG_PROFILE_ID, key, &crlf),
                baseline,
                "{id}: CRLF variant changed the result"
            );
        }

        // B3: member order is insignificant. Frozen ACCEPT material contains
        // no duplicate names, so reparsing into Values (alphabetically
        // reordered by serde_json's default map) preserves every typed value
        // while changing source layout.
        let reordered: Vec<u8> = std::str::from_utf8(input)
            .expect("ACCEPT seeds are UTF-8")
            .lines()
            .map(|line| {
                let value: Value = serde_json::from_str(line).expect("seed record parses");
                serde_json::to_string(&value).expect("reserializes")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes();
        assert_eq!(
            accept_of(V_SIG_PROFILE_ID, key, &reordered),
            baseline,
            "{id}: member-reordered variant changed the result"
        );
    }

    // B4: escape-decoded scalars compare equal — escaping the first letter of
    // the Genesis canonicalization profile spells the identical scalar
    // sequence through a legal source difference.
    let (d2_id, d2_key, d2_input) = seed_by_id(&seeds, "a21-d2-identity-r-equation-true");
    let baseline = accept_of(V_SIG_PROFILE_ID, &d2_key, &d2_input);
    let escaped = String::from_utf8(d2_input.clone()).expect("utf8").replacen(
        "\"magpie-core-v1\"",
        "\"\\u006dagpie-core-v1\"",
        1,
    );
    assert_ne!(escaped.as_bytes(), d2_input.as_slice());
    assert_eq!(
        accept_of(V_SIG_PROFILE_ID, &d2_key, escaped.as_bytes()),
        baseline,
        "{d2_id}: escape-equivalent profile spelling changed the result"
    );

    eprintln!("assurance B: whitespace/CRLF/reorder/escape variants preserved ACCEPT, count, tip");
}

// ---------------------------------------------------------------------------
// Property family C — governed lexical distinctions keep their stage owner.
// ---------------------------------------------------------------------------

fn synthetic_note_record(seq_spelling: &str, hash_hex: &str, sig_hex: &str) -> String {
    format!(
        r#"{{"core":{{"seq":{seq},"timestamp_nanos":0,"prev_hash":"{zero}","provenance":{{"agent":"a","source":"s"}},"payload":{{"kind":"Note","text":"t"}}}},"hash":"{hash}","signature":"{sig}"}}"#,
        seq = seq_spelling,
        zero = "0".repeat(64),
        hash = hash_hex,
        sig = sig_hex,
    )
}

#[test]
fn assurance_c_lexical_boundaries_remain_with_their_frozen_stage() {
    // External-key lexical/admissibility failures precede history inspection.
    for key in [
        "EA4A6C63E29C520ABEF5507B132EC5F9954776AEBEBE7B92421EEA691446D22C",
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22",
        "gg4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
        // y = p: representation-invalid despite perfect hex shape.
        "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
    ] {
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, key, b"{malformed"),
            PortableRejectionClass::ExternalKey,
            "key {key} must fail preflight before any history byte"
        );
    }

    // Raw integer spellings forbidden by RFC 8259 are JsonSyntax even where a
    // schema/u64 reading would also refuse them. (Negatives, -0, fractions,
    // and exponents are syntactically VALID JSON — frozen law assigns those
    // to Schema, so they are covered in the Schema block below.)
    for spelling in ["+1", "+0", "01", "00"] {
        let record = synthetic_note_record(spelling, &"0".repeat(64), &"0".repeat(128));
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, GOLDEN_KEY, record.as_bytes()),
            PortableRejectionClass::JsonSyntax,
            "spelling {spelling}"
        );
    }

    // Syntactically valid JSON numbers that violate the u64 lexical/range law
    // are Schema: negative, -0, fraction, exponent.
    for spelling in ["-1", "-0", "1.0", "1e0"] {
        let record = synthetic_note_record(spelling, &"0".repeat(64), &"0".repeat(128));
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, GOLDEN_KEY, record.as_bytes()),
            PortableRejectionClass::Schema,
            "valid-JSON-but-not-u64 spelling {spelling}"
        );
    }

    // u64 boundaries: MAX passes Schema and reaches Sequence; MAX+1 is Schema.
    let zeros = "0".repeat(64);
    let sig_zeros = "0".repeat(128);
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            GOLDEN_KEY,
            synthetic_note_record("18446744073709551615", &zeros, &sig_zeros).as_bytes()
        ),
        PortableRejectionClass::Sequence
    );
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            GOLDEN_KEY,
            synthetic_note_record("18446744073709551616", &zeros, &sig_zeros).as_bytes()
        ),
        PortableRejectionClass::Schema
    );

    // Hex transport law: uppercase/short stay Schema; a well-shaped but wrong
    // stored hash travels onward to ContentHash.
    let upper = format!("F{}", "0".repeat(63));
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            GOLDEN_KEY,
            synthetic_note_record("0", &upper, &sig_zeros).as_bytes()
        ),
        PortableRejectionClass::Schema
    );
    let short = "0".repeat(63);
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            GOLDEN_KEY,
            synthetic_note_record("0", &short, &sig_zeros).as_bytes()
        ),
        PortableRejectionClass::Schema
    );

    // UTF-8 and framing boundaries remain Magpie-owned.
    for (input, why) in [
        (&b"\xff{}\n"[..], "invalid UTF-8"),
        ("\u{feff}{}".as_bytes(), "candidate-offset-zero BOM"),
        (b"{}\r".as_slice(), "terminal lone CR"),
        // Bare space/tab-only candidate (with braces it would be ordinary
        // trailing whitespace, not a whitespace-only candidate).
        (b" \t\n".as_slice(), "whitespace-only candidate"),
    ] {
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, GOLDEN_KEY, input),
            PortableRejectionClass::Framing,
            "{why}"
        );
    }

    // Extra final terminator: needs a VALID first record, otherwise an
    // incomplete record fails at Schema before the second terminator matters.
    let terminator_seeds = accept_seeds();
    let (_, _, golden_input) = seed_by_id(&terminator_seeds, "p1-golden-v1");
    let first_line = String::from_utf8(golden_input)
        .expect("utf8")
        .split('\n')
        .next()
        .expect("first record")
        .to_string();
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            GOLDEN_KEY,
            format!("{first_line}\n\n").as_bytes()
        ),
        PortableRejectionClass::Framing
    );

    // Duplicate required key, literal and escape-equivalent spellings.
    let duplicate = format!(
        r#"{{"core":null,"c\u006fre":null,"hash":"{h}","signature":"{s}"}}"#,
        h = zeros,
        s = sig_zeros,
    );
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, GOLDEN_KEY, duplicate.as_bytes()),
        PortableRejectionClass::Schema
    );

    eprintln!("assurance C: every probed lexical boundary kept its frozen stage owner");
}

// ---------------------------------------------------------------------------
// Shared line-level mutation helpers for families D, F, G.
//
// Transport-flipping helpers deliberately keep the mutated field inside the
// lowercase-hex language (0 <-> 1) so Schema still admits the record and the
// defect surfaces at its own later stage.
// ---------------------------------------------------------------------------

fn flip_first_hex_char_after(line: &str, member: &str) -> Option<String> {
    let needle = format!("\"{member}\":\"");
    let start = line.find(&needle)? + needle.len();
    let mut out = line.to_string();
    let ch = out.as_bytes()[start] as char;
    let flipped = if ch == '0' { '1' } else { '0' };
    out.replace_range(start..start + 1, &flipped.to_string());
    Some(out)
}

fn shift_seq(line: &str) -> String {
    line.replacen("\"seq\":", "\"seq\":1", 1)
}

fn swap_signature_halves(line: &str) -> String {
    let needle = "\"signature\":\"";
    let start = line.find(needle).expect("signature member") + needle.len();
    let (left, right) = line[start..start + 128].split_at(64);
    let mut out = line.to_string();
    out.replace_range(start..start + 128, &format!("{right}{left}"));
    out
}

fn reserialize_line(line: &str) -> String {
    let value: Value = serde_json::from_str(line).expect("record parses");
    serde_json::to_string(&value).expect("reserializes")
}

// ---------------------------------------------------------------------------
// Property family D — first-failure stability under added later-stage defects.
// Uses the two-record deadbolt ACCEPT history: record 0 is SegmentAnchored,
// record 1 completes the chain, so every transport defect lands mid-chain.
// ---------------------------------------------------------------------------

#[test]
fn assurance_d_first_failure_is_stable_under_added_later_defects() {
    let seeds = accept_seeds();
    let (deadbolt_id, deadbolt_key, deadbolt_input) = seed_by_id(&seeds, "p2-deadbolt-anchor-v1");
    assert_eq!(
        accept_of(V_SIG_PROFILE_ID, &deadbolt_key, &deadbolt_input).0,
        2,
        "{deadbolt_id} baseline"
    );
    let text = String::from_utf8(deadbolt_input.clone()).expect("utf8");
    let (head, tail) = text.split_once('\n').expect("two records");

    let joined = |head: &str| format!("{head}\n{tail}").into_bytes();

    // Single defects land on their own stage.
    for (name, mutated, class) in [
        (
            "sequence",
            shift_seq(head),
            PortableRejectionClass::Sequence,
        ),
        (
            "previous-link",
            flip_first_hex_char_after(head, "prev_hash").expect("prev_hash member"),
            PortableRejectionClass::PreviousLink,
        ),
        (
            "content-hash",
            flip_first_hex_char_after(head, "hash").expect("hash member"),
            PortableRejectionClass::ContentHash,
        ),
        (
            "signature",
            flip_first_hex_char_after(head, "signature").expect("signature member"),
            PortableRejectionClass::Signature,
        ),
    ] {
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, &deadbolt_key, &joined(&mutated)),
            class,
            "{name} defect alone"
        );
    }

    // Ladder: adding a strictly later defect never outranks the earlier one.
    // Sequence rows shift the sequence and add one transport defect; later
    // rows stack two transport defects (both Schema-admissible lowercase hex).
    let ladder: &[(&str, &str, PortableRejectionClass)] = &[
        ("seq", "prev_hash", PortableRejectionClass::Sequence),
        ("seq", "hash", PortableRejectionClass::Sequence),
        ("seq", "signature", PortableRejectionClass::Sequence),
        ("prev_hash", "hash", PortableRejectionClass::PreviousLink),
        (
            "prev_hash",
            "signature",
            PortableRejectionClass::PreviousLink,
        ),
        ("hash", "signature", PortableRejectionClass::ContentHash),
    ];
    for (early_field, late_field, class) in ladder {
        let line = if *early_field == "seq" {
            flip_first_hex_char_after(&shift_seq(head), late_field).expect("late flip")
        } else {
            let early_flip = flip_first_hex_char_after(head, early_field).expect("early flip");
            flip_first_hex_char_after(&early_flip, late_field).expect("late flip")
        };
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, &deadbolt_key, &joined(&line)),
            *class,
            "early={early_field} late={late_field}"
        );
    }

    // A later physical record can never outrank a current-record terminal.
    let poisoned = format!("{}\n{{\n", shift_seq(head)).into_bytes();
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &deadbolt_key, &poisoned),
        PortableRejectionClass::Sequence
    );

    // P2-5: Signature before PayloadValidation. Source witness: frozen
    // `payload-empty-evidenceregistered-summary`, whose record zero has a
    // valid recomputed hash and valid signature and fails only at
    // PayloadValidation (empty required summary). Flipping the first
    // signature hex char must be the ONLY change: the helper operates on the
    // complete fixture text (including its terminal LF) and preserves it, so
    // the transformation below is proven to be exactly one differing byte,
    // located inside the transported signature value, with identical length
    // and trailing-LF shape. EventCore bytes and the stored hash are
    // untouched, and the mutated transport is still Schema-valid 128
    // lowercase hex — so any change of governed result can only come from
    // Signature outranking the already-present PayloadValidation defect.
    let (pv_key, pv_input) = frozen_case_bytes("payload-empty-evidenceregistered-summary");
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &pv_key, &pv_input),
        PortableRejectionClass::PayloadValidation,
        "source witness fails at PayloadValidation alone"
    );
    let source_bytes = pv_input.clone();
    let sig_broken = flip_first_hex_char_after(
        std::str::from_utf8(&pv_input).expect("witness is UTF-8"),
        "signature",
    )
    .expect("signature member present in witness record")
    .into_bytes();
    // Mechanically prove the transformation is signature-only:
    assert_eq!(
        sig_broken.len(),
        source_bytes.len(),
        "signature mutation must not change input length"
    );
    let diffs: Vec<usize> = sig_broken
        .iter()
        .zip(source_bytes.iter())
        .enumerate()
        .filter_map(|(at, (new, old))| (new != old).then_some(at))
        .collect();
    assert_eq!(diffs.len(), 1, "exactly one byte may differ: {diffs:?}");
    let needle = "\"signature\":\"";
    let sig_start = std::str::from_utf8(&source_bytes)
        .expect("utf8")
        .find(needle)
        .expect("signature member")
        + needle.len();
    let diff_at = diffs[0];
    assert!(
        diff_at >= sig_start && diff_at < sig_start + 128,
        "differing byte {diff_at} must lie inside the transported signature \
         value [{sig_start}, {})",
        sig_start + 128
    );
    assert_eq!(diff_at, sig_start, "the first intended signature nibble");
    for shape in [&source_bytes, &sig_broken] {
        assert!(
            shape.ends_with(b"\n")
                && !shape.ends_with(b"\n\n")
                && !shape.windows(2).any(|pair| pair == b"\n\n"),
            "trailing-LF shape must be exactly one terminal LF"
        );
    }
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &pv_key, &sig_broken),
        PortableRejectionClass::Signature,
        "one-byte Schema-valid signature mutation moves the failure to Signature"
    );

    eprintln!(
        "assurance D: adjacent precedence chain Sequence->PreviousLink->ContentHash->\
         Signature->PayloadValidation fully evidenced; later-record isolation holds"
    );
}

// ---------------------------------------------------------------------------
// Property family E — prefix isolation and the tip law, using the nine-record
// golden history and the cfg(test) ordered-hash observer.
// ---------------------------------------------------------------------------

#[test]
fn assurance_e_prefix_isolation_and_recomputed_tip_law() {
    let seeds = accept_seeds();
    let (_, golden_key, golden_input) = seed_by_id(&seeds, "p1-golden-v1");
    let text = String::from_utf8(golden_input.clone()).expect("utf8");
    let lines: Vec<&str> = text.trim_end_matches('\n').split('\n').collect();
    assert_eq!(lines.len(), 9, "golden history has nine records");

    let (full_outcome, full_hashes) = traced(&golden_key, &golden_input);
    let (count, tip) = match full_outcome {
        CompleteHistoryOutcome::Accept(accepted) => (accepted.event_count(), accepted.tip()),
        other => panic!("expected ACCEPT, got {other:?}"),
    };
    assert_eq!(count, 9);
    assert_eq!(tip, full_hashes[8], "final tip is the last committed hash");

    // E2/E3: defects confined to the final record leave every earlier
    // transition byte-identical and commit nothing for the failing record.
    for (name, broken_last, expected_class) in [
        (
            "malformed final record",
            "{\"core\":{\"seq\":8".to_string(),
            PortableRejectionClass::JsonSyntax,
        ),
        (
            "signature-broken final record",
            flip_first_hex_char_after(lines[8], "signature").expect("signature member"),
            PortableRejectionClass::Signature,
        ),
    ] {
        let mut mutated = lines.clone();
        mutated[8] = &broken_last;
        let input = format!("{}\n", mutated.join("\n")).into_bytes();
        let (outcome, hashes) = traced(&golden_key, &input);
        match outcome {
            CompleteHistoryOutcome::Reject(rejection) => {
                assert_eq!(rejection.class(), expected_class, "{name}");
                assert_eq!(rejection.record_index(), Some(8), "{name}");
            }
            other => panic!("{name}: expected REJECT, got {other:?}"),
        }
        assert_eq!(hashes.len(), 8, "{name}: failing record commits nothing");
        assert_eq!(
            hashes.as_slice(),
            &full_hashes[..8],
            "{name}: earlier transitions unchanged by suffix defect"
        );
    }

    // E4: literally every non-empty prefix (P2-2) accepts with exactly that
    // many records, the observed trace is exactly that long, and the tip is
    // the k-th committed hash — the previous record's recomputed hash, never a
    // transported value.
    for k in 1..=lines.len() {
        let prefix = format!("{}\n", lines[..k].join("\n")).into_bytes();
        let (outcome, hashes) = traced(&golden_key, &prefix);
        match outcome {
            CompleteHistoryOutcome::Accept(accepted) => {
                assert_eq!(accepted.event_count(), k as u64, "prefix {k}");
                assert_eq!(hashes.len(), k, "prefix {k} trace length");
                assert_eq!(accepted.tip(), hashes[k - 1], "prefix {k}");
                assert_eq!(hashes.as_slice(), &full_hashes[..k], "prefix {k}");
            }
            other => panic!("prefix of length {k}: expected ACCEPT, got {other:?}"),
        }
    }

    eprintln!("assurance E: prefix isolation, zero-commit-on-failure, and tip law hold");
}

// ---------------------------------------------------------------------------
// Property family F — canonical content-hash boundary.
// ---------------------------------------------------------------------------

#[test]
fn assurance_f_canonical_hash_boundary_is_typed_content_only() {
    let seeds = accept_seeds();
    let (_, golden_key, golden_input) = seed_by_id(&seeds, "p1-golden-v1");
    let text = String::from_utf8(golden_input.clone()).expect("utf8");
    let lines: Vec<&str> = text.trim_end_matches('\n').split('\n').collect();

    // F1: two legal source spellings of the SAME typed genesis record produce
    // the identical recomputed tip.
    let one_record = format!("{}\n", lines[0]).into_bytes();
    let (_, baseline_hashes) = traced(&golden_key, &one_record);
    let permuted = format!("{}\n", reserialize_line(lines[0])).into_bytes();
    let (_, permuted_hashes) = traced(&golden_key, &permuted);
    assert_eq!(
        baseline_hashes, permuted_hashes,
        "equivalent spellings must canonicalize identically"
    );

    // F2: a genuine typed-content change (timestamp_nanos) must not retain
    // ACCEPT — with unchanged transport it fails at ContentHash, proving the
    // hash preimage really covers typed core content.
    let retimed = lines[0].replacen("\"timestamp_nanos\":", "\"timestamp_nanos\":1", 1);
    let mutated = format!("{}\n", retimed).into_bytes();
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &golden_key, &mutated),
        PortableRejectionClass::ContentHash
    );

    // F3: swapping R/S halves keeps Schema-valid transport but must fail
    // Signature — the equation binds the exact R and S values.
    let swapped_sig = swap_signature_halves(lines[0]);
    let swapped = format!("{}\n", swapped_sig).into_bytes();
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &golden_key, &swapped),
        PortableRejectionClass::Signature
    );

    eprintln!("assurance F: canonical hashing follows typed content, not source spelling");
}

// ---------------------------------------------------------------------------
// Property family G — signature/key/profile binding composition.
// ---------------------------------------------------------------------------

#[test]
fn assurance_g_signature_key_and_profile_bindings_hold() {
    let seeds = accept_seeds();
    let (_, _, golden_input) = seed_by_id(&seeds, "p1-golden-v1");

    // Message/content binding: frozen altered-message and altered-signature
    // material stays Signature through the production path.
    let manifest = load_manifest();
    for case_id in ["a21-altered-message", "a21-altered-signature"] {
        let case = manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"].as_str() == Some(case_id))
            .unwrap_or_else(|| panic!("{case_id} present"));
        let input = std::fs::read(repository_root().join(case["input_path"].as_str().unwrap()))
            .expect("case bytes");
        let key = case["external_verifying_key_hex"].as_str().unwrap();
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, key, &input),
            PortableRejectionClass::Signature,
            "{case_id}"
        );
    }

    // Key binding: the same valid history under a DIFFERENT admissible
    // external key must fail at Signature (not trust, not Genesis).
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, OTHER_ADMISSIBLE_KEY, &golden_input),
        PortableRejectionClass::Signature
    );

    // Profile binding: selection is exact and fail-closed; anything else is an
    // operational configuration result, never a governed class.
    let outcome = verify_complete_history("latest", GOLDEN_KEY, &golden_input);
    assert!(
        matches!(outcome, Err(CompleteHistoryError::UnsupportedProfile(_))),
        "unknown profile must stay operational"
    );

    eprintln!("assurance G: signature, external-key, and profile bindings hold");
}

// ---------------------------------------------------------------------------
// Property families H and I — late-stage semantics on real frozen material.
//
// PayloadValidation and Genesis failures require correctly hashed and signed
// records whose payload/genesis fields violate L0 rules; constructing those
// from scratch would need a signer, so the metamorphic ladders compose frozen
// PV/Genesis cases with additional later-stage defects instead.
// ---------------------------------------------------------------------------

fn frozen_case_bytes(case_id: &str) -> (String, Vec<u8>) {
    let manifest = load_manifest();
    let case = manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("{case_id} present"));
    let input = std::fs::read(repository_root().join(case["input_path"].as_str().unwrap()))
        .expect("case bytes");
    (
        case["external_verifying_key_hex"]
            .as_str()
            .unwrap()
            .to_string(),
        input,
    )
}

#[test]
fn assurance_h_payload_rules_stay_at_payloadvalidation_before_genesis() {
    // Each witness is a frozen single-record case whose record zero:
    //   1. has a valid path through Framing/JsonSyntax/Schema (it reaches the
    //      semantic stages at all),
    //   2. carries a correctly hashed, correctly signed EventCore, and
    //   3. violates an L0 payload rule (unknown vocabulary / bad
    //      witness_root hex / empty required string),
    // while ALSO containing a later Genesis position/kind defect: record zero
    // is a non-Genesis payload, so Genesis would reject it afterwards.
    // The governed result must be PayloadValidation — proving PayloadValidation
    // outranks the later Genesis defect. There is no optional branch here: if
    // any witness failed to reach PayloadValidation, or if its class were not
    // PayloadValidation, these assertions fail loudly.
    for case_id in [
        "n20-actor-unknown",
        "n20-evidence-unknown",
        "n20-edge-unknown",
        "n3-witness-root-nonhex",
        "payload-empty-evidenceregistered-summary",
    ] {
        let (key, input) = frozen_case_bytes(case_id);
        let text = String::from_utf8(input.clone()).expect("witness is UTF-8");
        // Non-vacuity guard: every selected witness is a non-Genesis record,
        // i.e. it genuinely carries the later Genesis position/kind defect.
        assert!(
            !text.contains("\"kind\":\"Genesis\""),
            "{case_id}: witness must be a NON-Genesis payload for the \
             PayloadValidation-before-Genesis(position/kind) relation to hold"
        );
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, &key, &input),
            PortableRejectionClass::PayloadValidation,
            "{case_id} must reject at PayloadValidation despite its later \
             Genesis position/kind defect"
        );
    }

    // metadata_json positive-path evidence: an ACCEPT history carrying a
    // metadata_json member is accepted end to end. This shows ordinary
    // metadata_json content does not disturb payload validation; it does NOT
    // by itself discriminate opaque-text handling from recursive JSON parsing.
    // See the PR description for the exact scope of this claim.
    let (vocab_key, vocab_input) = frozen_case_bytes("positive-complete-vocabulary");
    let vocab_text = String::from_utf8(vocab_input.clone()).expect("utf8");
    assert!(
        vocab_text.contains("\"metadata_json\""),
        "witness must actually carry a metadata_json member"
    );
    let outcome = run(V_SIG_PROFILE_ID, &vocab_key, &vocab_input);
    assert!(
        matches!(outcome, CompleteHistoryOutcome::Accept(_)),
        "positive metadata_json material must remain ACCEPT"
    );

    eprintln!(
        "assurance H: PayloadValidation outranks later Genesis position/kind on all five witnesses"
    );
}

#[test]
fn assurance_i_genesis_suborder_and_binding() {
    // Frozen Genesis witnesses keep their class through the production path.
    for case_id in [
        "n25-non-genesis-at-zero",
        "n25-genesis-after-zero",
        "n26-wrong-genesis-profile",
        "n27-genesis-key-mismatch",
        "n3-genesis-key-nonhex",
        "n1-genesis-key-uppercase",
    ] {
        let (key, input) = frozen_case_bytes(case_id);
        assert_eq!(
            reject_class_of(V_SIG_PROFILE_ID, &key, &input),
            PortableRejectionClass::Genesis,
            "{case_id}"
        );
    }

    // Genesis-internal stacking on frozen material is impossible for payload
    // text: the profile and declared key are hashed canonical content, so any
    // text mutation changes the EventCore and ContentHash correctly fires
    // first (confirmed in family F). That precedence is itself the assertion:
    // a profile mutation on a Genesis case must surface as ContentHash, never
    // silently as Genesis or ACCEPT.
    let (profile_case_key, profile_case_input) = frozen_case_bytes("n26-wrong-genesis-profile");
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, &profile_case_key, &profile_case_input),
        PortableRejectionClass::Genesis,
        "wrong profile alone"
    );
    let retimed_profile_case = String::from_utf8(profile_case_input.clone())
        .expect("utf8")
        .replacen(
            "\"canonicalization_profile\":\"",
            "\"canonicalization_profile\":\"x",
            1,
        );
    assert_ne!(
        retimed_profile_case.as_bytes(),
        profile_case_input.as_slice()
    );
    assert_eq!(
        reject_class_of(
            V_SIG_PROFILE_ID,
            &profile_case_key,
            retimed_profile_case.as_bytes()
        ),
        PortableRejectionClass::ContentHash,
        "payload-text mutation is caught by ContentHash before Genesis"
    );

    // Genesis never selects the key: the K15-style different-admissible-key
    // probe above (family G) plus n27 prove binding is against the supplied
    // key, and equality never bootstraps trust — asserted structurally by the
    // conformer comparing decoded bytes only.

    eprintln!("assurance I: Genesis position/profile/key laws held for every probe");
}

// ---------------------------------------------------------------------------
// Property family J — empty history and preflight precedence.
// ---------------------------------------------------------------------------

#[test]
fn assurance_j_empty_history_and_preflight_precedence() {
    // Valid profile + admissible key + zero bytes => ACCEPT(0, ZERO).
    let outcome = run(V_SIG_PROFILE_ID, GOLDEN_KEY, b"");
    match outcome {
        CompleteHistoryOutcome::Accept(accepted) => {
            assert_eq!(accepted.event_count(), 0);
            assert_eq!(accepted.tip(), crate::ContentHash::ZERO);
        }
        other => panic!("empty history must ACCEPT, got {other:?}"),
    }

    // Preflight precedes examination of history bytes — even none.
    assert_eq!(
        reject_class_of(V_SIG_PROFILE_ID, "not-a-key", b""),
        PortableRejectionClass::ExternalKey
    );

    // Unsupported profile stays operational/configuration for empty input.
    let outcome = verify_complete_history("latest", GOLDEN_KEY, b"");
    assert!(matches!(
        outcome,
        Err(CompleteHistoryError::UnsupportedProfile(_))
    ));

    eprintln!("assurance J: empty-history ACCEPT and preflight precedence confirmed");
}
