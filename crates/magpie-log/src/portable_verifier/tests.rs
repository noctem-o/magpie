use crate::signature_profile::V_SIG_PROFILE_ID;
use crate::Payload;

use super::framing::FrameStep;
use super::frontend::{FrontendSession, FrontendStep};
use super::preflight::{FrontendStartError, PreparedFrontend};
use super::FrontendRejectionClass;

const GOLDEN_KEY: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";
const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const ZERO_SIGNATURE: &str = concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0000000000000000000000000000000000000000000000000000000000000000",
);

fn cursor_for(input: &[u8]) -> super::framing::FrameCursor<'_> {
    PreparedFrontend::new(V_SIG_PROFILE_ID, GOLDEN_KEY, input)
        .expect("golden key preflight must pass")
        .into_parts()
        .1
}

fn expect_framing(input: &[u8], expected_line: usize, expected_record_index: Option<usize>) {
    match cursor_for(input).next() {
        FrameStep::Rejected(rejection) => {
            assert_eq!(rejection.class(), FrontendRejectionClass::Framing);
            assert_eq!(rejection.line(), Some(expected_line));
            assert_eq!(rejection.record_index(), expected_record_index);
        }
        FrameStep::End => panic!("framing failure unexpectedly reached end"),
        FrameStep::Candidate(_) => panic!("framing failure unexpectedly produced a candidate"),
    }
}

fn note_record(text: &str) -> String {
    format!(
        r#"{{"core":{{"seq":0,"timestamp_nanos":0,"prev_hash":"{ZERO_HASH}","provenance":{{"agent":"agent","source":"source"}},"payload":{{"kind":"Note","text":"{text}"}}}},"hash":"{ZERO_HASH}","signature":"{ZERO_SIGNATURE}"}}"#
    )
}

fn frontend(input: &[u8]) -> FrontendSession<'_> {
    FrontendSession::new(V_SIG_PROFILE_ID, GOLDEN_KEY, input)
        .expect("golden-key frontend preflight must pass")
}

fn expect_frontend_rejection(
    input: &[u8],
    expected_class: FrontendRejectionClass,
    expected_line: usize,
    expected_record_index: usize,
) {
    match frontend(input).next().expect("frontend must execute") {
        FrontendStep::Rejected(rejection) => {
            assert_eq!(rejection.class(), expected_class);
            assert_eq!(rejection.line(), Some(expected_line));
            assert_eq!(rejection.record_index(), Some(expected_record_index));
        }
        FrontendStep::End => panic!("rejection unexpectedly reached end"),
        FrontendStep::Pending(_) => panic!("rejection unexpectedly yielded a pending record"),
    }
}

#[test]
fn profile_and_complete_external_key_gate_precede_history() {
    let malformed_history = b"\xff\r\n{";

    assert!(matches!(
        PreparedFrontend::new("latest", GOLDEN_KEY, malformed_history),
        Err(FrontendStartError::UnsupportedProfile(_))
    ));

    for invalid_key in [
        "",
        "EA4A6C63E29C520ABEF5507B132EC5F9954776AEBEBE7B92421EEA691446D22C",
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22g",
        "0200000000000000000000000000000000000000000000000000000000000000",
        "0100000000000000000000000000000000000000000000000000000000000000",
    ] {
        match PreparedFrontend::new(V_SIG_PROFILE_ID, invalid_key, malformed_history) {
            Err(FrontendStartError::Rejected(rejection)) => {
                assert_eq!(rejection.class(), FrontendRejectionClass::ExternalKey);
                assert_eq!(rejection.line(), None);
                assert_eq!(rejection.record_index(), None);
            }
            Err(FrontendStartError::UnsupportedProfile(_)) => {
                panic!("exact profile was unexpectedly unsupported")
            }
            Ok(_) => panic!("invalid external key unexpectedly passed"),
        }
    }
}

#[test]
fn invalid_key_beats_malformed_history() {
    match FrontendSession::new(V_SIG_PROFILE_ID, "not-a-key", b"{malformed") {
        Err(FrontendStartError::Rejected(rejection)) => {
            assert_eq!(rejection.class(), FrontendRejectionClass::ExternalKey);
            assert_eq!(rejection.line(), None);
            assert_eq!(rejection.record_index(), None);
        }
        _ => panic!("invalid external key must win before malformed history"),
    }
}

#[test]
fn invalid_key_beats_empty_history() {
    match FrontendSession::new(V_SIG_PROFILE_ID, "not-a-key", b"") {
        Err(FrontendStartError::Rejected(rejection)) => {
            assert_eq!(rejection.class(), FrontendRejectionClass::ExternalKey);
            assert_eq!(rejection.line(), None);
            assert_eq!(rejection.record_index(), None);
        }
        _ => panic!("invalid external key must win before empty-snapshot handling"),
    }
}

#[test]
fn unsupported_profile_is_operational_not_a_portable_rejection() {
    assert!(matches!(
        FrontendSession::new("latest", GOLDEN_KEY, b"{malformed"),
        Err(FrontendStartError::UnsupportedProfile(_))
    ));
}

#[test]
fn exact_framing_assigns_frozen_coordinates() {
    assert!(matches!(cursor_for(b"").next(), FrameStep::End));
    expect_framing(b"\n", 1, None);
    expect_framing(b"\r\n", 1, None);
    expect_framing(b" \t\n", 1, Some(0));
    expect_framing(b"{}\r", 1, Some(0));
    expect_framing(b"\xef\xbb\xbf{}\n", 1, Some(0));
    expect_framing(b"\xff\n", 1, Some(0));

    let first = match cursor_for(b"{}\r\n{}\n").next() {
        FrameStep::Candidate(candidate) => candidate,
        FrameStep::End => panic!("first candidate missing"),
        FrameStep::Rejected(rejection) => panic!("first candidate rejected: {rejection:?}"),
    };
    assert_eq!(first.text(), "{}");
    assert_eq!(first.line(), 1);
    assert_eq!(first.record_index(), 0);
    let second = match first.into_continuation().next() {
        FrameStep::Candidate(candidate) => candidate,
        FrameStep::End => panic!("second candidate missing"),
        FrameStep::Rejected(rejection) => panic!("second candidate rejected: {rejection:?}"),
    };
    assert_eq!(second.text(), "{}");
    assert_eq!(second.line(), 2);
    assert_eq!(second.record_index(), 1);
    assert!(matches!(second.into_continuation().next(), FrameStep::End));
}

#[test]
fn lone_cr_is_framing() {
    expect_framing(b"{}\r", 1, Some(0));
}

#[test]
fn crlf_only_is_zero_length_record_framing() {
    expect_framing(b"\r\n", 1, None);
}

#[test]
fn bom_at_candidate_start_is_framing() {
    expect_framing(b"\xef\xbb\xbf{}", 1, Some(0));
}

#[test]
fn invalid_utf8_is_framing_without_replacement() {
    expect_framing(b"\xff", 1, Some(0));
}

#[test]
fn extra_final_terminator_is_a_zero_length_record() {
    let first = match cursor_for(b"{}\n\n").next() {
        FrameStep::Candidate(candidate) => candidate,
        _ => panic!("first candidate must frame"),
    };
    match first.into_continuation().next() {
        FrameStep::Rejected(rejection) => {
            assert_eq!(rejection.class(), FrontendRejectionClass::Framing);
            assert_eq!(rejection.line(), Some(2));
            assert_eq!(rejection.record_index(), None);
        }
        _ => panic!("extra final terminator must reject"),
    }
}

#[test]
fn complete_syntax_and_schema_are_separate_ordered_phases() {
    expect_frontend_rejection(
        b"{\"core\":{\"seq\":01}}",
        FrontendRejectionClass::JsonSyntax,
        1,
        0,
    );
    expect_frontend_rejection(
        b"{\"core\":null,\"c\\u006fre\":null,\"hash\":null,\"signature\":null}",
        FrontendRejectionClass::Schema,
        1,
        0,
    );

    let huge = note_record("ok").replacen(
        "\"seq\":0",
        "\"seq\":99999999999999999999999999999999999999999999999999999",
        1,
    );
    expect_frontend_rejection(huge.as_bytes(), FrontendRejectionClass::Schema, 1, 0);

    let uppercase_hash = format!("A{}", "0".repeat(63));
    let uppercase = note_record("ok").replacen(ZERO_HASH, &uppercase_hash, 1);
    expect_frontend_rejection(uppercase.as_bytes(), FrontendRejectionClass::Schema, 1, 0);
}

#[test]
fn huge_valid_json_integer_is_schema_not_jsonsyntax() {
    let huge = note_record("ok").replacen(
        "\"seq\":0",
        "\"seq\":99999999999999999999999999999999999999999999999999999",
        1,
    );
    expect_frontend_rejection(huge.as_bytes(), FrontendRejectionClass::Schema, 1, 0);
}

#[test]
fn leading_zero_is_jsonsyntax() {
    expect_frontend_rejection(
        b"{\"core\":{\"seq\":01}}",
        FrontendRejectionClass::JsonSyntax,
        1,
        0,
    );
}

#[test]
fn uppercase_hex_is_schema() {
    let uppercase_hash = format!("A{}", "0".repeat(63));
    let uppercase = note_record("ok").replacen(ZERO_HASH, &uppercase_hash, 1);
    expect_frontend_rejection(uppercase.as_bytes(), FrontendRejectionClass::Schema, 1, 0);
}

#[test]
fn bom_utf8_and_ufeff_obey_magpie_owned_stage_boundaries() {
    expect_framing(b"\xef\xbb\xbf{}", 1, Some(0));
    expect_framing(b"\xff", 1, Some(0));
    expect_frontend_rejection(b" \xef\xbb\xbf{}", FrontendRejectionClass::JsonSyntax, 1, 0);

    let literal = note_record("\u{feff}");
    let pending = match frontend(literal.as_bytes()).next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => panic!("U+FEFF inside a string must reach the pending record"),
    };
    assert!(matches!(
        pending.record().core().payload,
        Payload::Note { .. }
    ));

    let escaped = note_record(r#"\uFEFF"#);
    assert!(matches!(
        frontend(escaped.as_bytes()).next().unwrap(),
        FrontendStep::Pending(_)
    ));
}

#[test]
fn ufeff_after_space_is_jsonsyntax() {
    expect_frontend_rejection(b" \xef\xbb\xbf{}", FrontendRejectionClass::JsonSyntax, 1, 0);
}

#[test]
fn ufeff_inside_string_is_not_bom() {
    for record in [note_record("\u{feff}"), note_record(r#"\uFEFF"#)] {
        assert!(matches!(
            frontend(record.as_bytes()).next().unwrap(),
            FrontendStep::Pending(_)
        ));
    }
}

#[test]
fn later_payload_and_genesis_rules_do_not_move_into_schema() {
    let late_payload = format!(
        r#"{{"core":{{"seq":0,"timestamp_nanos":0,"prev_hash":"{ZERO_HASH}","provenance":{{"agent":"","source":""}},"payload":{{"kind":"ClaimAssertedV2","claim_id":"","statement":"","scope_ref":"","actor_class":"unknown","content_hash":"UPPERCASE","metadata_json":"not json"}}}},"hash":"{ZERO_HASH}","signature":"{ZERO_SIGNATURE}"}}"#
    );
    assert!(matches!(
        frontend(late_payload.as_bytes()).next().unwrap(),
        FrontendStep::Pending(_)
    ));

    let late_genesis = format!(
        r#"{{"core":{{"seq":0,"timestamp_nanos":0,"prev_hash":"{ZERO_HASH}","provenance":{{"agent":"a","source":"s"}},"payload":{{"kind":"Genesis","canonicalization_profile":"wrong","verifying_key":"NOT HEX"}}}},"hash":"{ZERO_HASH}","signature":"{ZERO_SIGNATURE}"}}"#
    );
    assert!(matches!(
        frontend(late_genesis.as_bytes()).next().unwrap(),
        FrontendStep::Pending(_)
    ));
}

#[test]
fn full_width_sequence_reaches_the_later_sequence_stage() {
    let upper_sequence = note_record("ok").replacen("\"seq\":0", "\"seq\":18446744073709551615", 1);
    let pending = match frontend(upper_sequence.as_bytes()).next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => panic!("u64::MAX sequence must pass frontend Schema"),
    };
    assert_eq!(pending.record().core().seq, u64::MAX);
}

#[test]
fn pending_record_owns_continuation_and_next_record_is_unavailable_while_pending() {
    let first_record = note_record("first");
    let second_record = note_record("second");
    let input = format!("{first_record}\n{second_record}");

    let pending = match frontend(input.as_bytes()).next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => panic!("first record must become pending"),
    };
    assert_eq!(pending.record().line(), 1);
    assert_eq!(pending.record().record_index(), 0);
    assert_eq!(pending.record().stored_hash(), &[0_u8; 32]);
    assert_eq!(pending.record().signature(), &[0_u8; 64]);
    assert_eq!(
        pending.signature_verifier().external_key_bytes(),
        &super::lexical::decode_lower_hex::<32>(GOLDEN_KEY).unwrap()
    );

    // No `next` operation exists on PendingRecord. The only consuming route
    // reports current-record semantic success and returns the sole session.
    let ready = pending
        .complete_semantics::<()>(Ok(()))
        .expect("test-only semantic-success report must return the cursor");
    let second = match ready.next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => panic!("second record must remain unavailable until release"),
    };
    assert_eq!(second.record().line(), 2);
    assert_eq!(second.record().record_index(), 1);
}

#[test]
fn malformed_later_record_cannot_preempt_current_semantics() {
    let first_record = note_record("destined for a semantic failure");
    let input = format!("{first_record}\n{{");

    let pending = match frontend(input.as_bytes()).next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => panic!("current record must return before later JSON is inspected"),
    };
    assert_eq!(pending.record().record_index(), 0);
    drop(pending);

    let pending = match frontend(input.as_bytes()).next().unwrap() {
        FrontendStep::Pending(pending) => pending,
        _ => unreachable!(),
    };
    let ready = pending.complete_semantics::<()>(Ok(())).unwrap();
    match ready.next().unwrap() {
        FrontendStep::Rejected(rejection) => {
            assert_eq!(rejection.class(), FrontendRejectionClass::JsonSyntax);
            assert_eq!(rejection.line(), Some(2));
            assert_eq!(rejection.record_index(), Some(1));
        }
        _ => panic!("later malformed record must reject only after release"),
    }
}

#[test]
fn serde_diagnostic_text_never_controls_frontend_classification() {
    for malformed in [b"{".as_slice(), b"[1,".as_slice(), b"{} garbage".as_slice()] {
        expect_frontend_rejection(malformed, FrontendRejectionClass::JsonSyntax, 1, 0);
    }
}
