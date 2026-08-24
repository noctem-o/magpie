use crate::signature_profile::V_SIG_PROFILE_ID;

use super::framing::FrameStep;
use super::preflight::{FrontendStartError, PreparedFrontend};
use super::FrontendRejectionClass;

const GOLDEN_KEY: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";

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
