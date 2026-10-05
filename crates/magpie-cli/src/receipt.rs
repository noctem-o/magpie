//! Exact-byte acquisition and transport only. All receipt semantics live in claims.

use std::io::Write;
use std::path::PathBuf;

use magpie_claims::{
    check_standing_receipt_v0, produce_standing_receipt_v0, StandingReceiptContextV0,
    StandingReceiptErrorV0, StandingReceiptHistoryIdentityV0, StandingReceiptRequestV0,
};

use crate::args::ReceiptInput;
use crate::CliError;

fn failure(error: StandingReceiptErrorV0) -> CliError {
    match error {
        StandingReceiptErrorV0::Operational(error) => CliError::Io(error.to_string()),
        other => CliError::Refused(other.to_string()),
    }
}

pub(crate) fn execute(
    input: ReceiptInput,
    detached_path: Option<PathBuf>,
    out: &mut dyn Write,
) -> Result<(), CliError> {
    // One retained byte image, with no Snapshot, store, scratch copy or normalization.
    let history_bytes = std::fs::read(&input.history)?;
    let history = StandingReceiptHistoryIdentityV0::of(&history_bytes)
        .map_err(|error| CliError::Io(error.to_string()))?;
    let request = StandingReceiptRequestV0 {
        context: StandingReceiptContextV0 {
            profile: input.profile,
            history,
            verifying_key: input.verifying_key,
            expectation: input.expectation,
            claim_id: input.claim_id,
        },
        policy_id: input.policy,
    };
    let receipt = match detached_path {
        Some(path) => {
            let detached_bytes = std::fs::read(path)?;
            check_standing_receipt_v0(&request, &history_bytes, &detached_bytes)
        }
        None => produce_standing_receipt_v0(&request, &history_bytes),
    }
    .map_err(failure)?;
    // Nothing is emitted before every gate succeeds. An output device can still
    // fail after a partial write; report exit 3 rather than an epistemic result.
    out.write_all(receipt.canonical_bytes())?;
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use magpie_claims::StandingReceiptOperationalErrorV0;

    #[test]
    fn operational_inability_and_refusal_keep_existing_exit_classes() {
        assert!(matches!(
            failure(StandingReceiptErrorV0::Operational(
                StandingReceiptOperationalErrorV0::HistoryByteCountOverflow
            )),
            CliError::Io(_)
        ));
        assert!(matches!(
            failure(StandingReceiptErrorV0::DetachedBytesMismatch),
            CliError::Refused(_)
        ));
    }
}
