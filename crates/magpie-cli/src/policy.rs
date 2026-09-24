//! Explicit standing-policy selection and resolution.
//!
//! The caller always names the policy. There is no default and no "latest",
//! matching the kernel's rule that a new policy never silently replaces the
//! caller's chosen semantics. Policies v3 and v4 also take a resolution content
//! closure; this tool supplies the empty closure, so rules that need supplied
//! artifact or bundle bytes report those inputs as missing rather than looking
//! for them anywhere.

use magpie_claims::{
    OriginAdmissionReplayContextV0, ResolutionContentClosureV0, MAGPIE_CLAIMS_POLICY_ID,
    MAGPIE_CLAIMS_POLICY_V1_ID, MAGPIE_CLAIMS_POLICY_V2_ID, MAGPIE_CLAIMS_POLICY_V3_ID,
    MAGPIE_CLAIMS_POLICY_V4_ID,
};
use magpie_log::Status;
use serde_json::Value;

use crate::CliError;

pub(crate) const POLICY_HINT: &str =
    "choose a standing policy with --policy v0, v1, v2, v3 or v4; Magpie never picks one for you";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PolicyChoice {
    V0,
    V1,
    V2,
    V3,
    V4,
}

impl PolicyChoice {
    const ALL: [PolicyChoice; 5] = [
        PolicyChoice::V0,
        PolicyChoice::V1,
        PolicyChoice::V2,
        PolicyChoice::V3,
        PolicyChoice::V4,
    ];

    /// Accept the short form (`v2`) or the full policy identity.
    pub(crate) fn parse(text: &str) -> Result<Self, CliError> {
        Self::ALL
            .into_iter()
            .find(|policy| text == policy.short() || text == policy.id())
            .ok_or_else(|| CliError::Usage(format!("unknown policy `{text}`; {POLICY_HINT}")))
    }

    fn short(self) -> &'static str {
        match self {
            PolicyChoice::V0 => "v0",
            PolicyChoice::V1 => "v1",
            PolicyChoice::V2 => "v2",
            PolicyChoice::V3 => "v3",
            PolicyChoice::V4 => "v4",
        }
    }

    pub(crate) fn id(self) -> &'static str {
        match self {
            PolicyChoice::V0 => MAGPIE_CLAIMS_POLICY_ID,
            PolicyChoice::V1 => MAGPIE_CLAIMS_POLICY_V1_ID,
            PolicyChoice::V2 => MAGPIE_CLAIMS_POLICY_V2_ID,
            PolicyChoice::V3 => MAGPIE_CLAIMS_POLICY_V3_ID,
            PolicyChoice::V4 => MAGPIE_CLAIMS_POLICY_V4_ID,
        }
    }
}

/// One claim's governed standing and the policy's own explanation of it.
pub(crate) struct Resolution {
    pub governed: Option<Status>,
    /// The resolver's complete audit structure, serialized unchanged.
    pub explanation: Value,
}

/// Resolves claims against one verified replay context.
pub(crate) struct Resolver<'a> {
    context: &'a OriginAdmissionReplayContextV0,
    closure: ResolutionContentClosureV0,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(context: &'a OriginAdmissionReplayContextV0) -> Result<Self, CliError> {
        let closure = ResolutionContentClosureV0::construct(&[], &[]).map_err(|error| {
            CliError::Io(format!(
                "could not build the empty resolution closure: {error:?}"
            ))
        })?;
        Ok(Self { context, closure })
    }

    pub(crate) fn resolve(
        &self,
        policy: PolicyChoice,
        claim_id: &str,
    ) -> Result<Resolution, CliError> {
        let snapshot = self.context.snapshot();
        let (governed, explanation) = match policy {
            PolicyChoice::V0 => {
                let resolution = snapshot.standing().resolved_standing_with_trace(claim_id);
                (
                    resolution.governed_standing,
                    serde_json::to_value(&resolution)?,
                )
            }
            PolicyChoice::V1 => {
                let resolution = snapshot.resolved_standing_with_trace_v1(claim_id);
                (
                    resolution.governed_standing,
                    serde_json::to_value(&resolution)?,
                )
            }
            PolicyChoice::V2 => {
                let resolution = snapshot.resolved_standing_with_trace_v2(claim_id);
                (
                    resolution.governed_standing,
                    serde_json::to_value(&resolution)?,
                )
            }
            PolicyChoice::V3 => {
                let resolution = self
                    .context
                    .resolved_standing_with_trace_v3(claim_id, &self.closure);
                (
                    resolution.governed_standing(),
                    serde_json::to_value(&resolution)?,
                )
            }
            PolicyChoice::V4 => {
                let resolution = self
                    .context
                    .resolved_standing_with_trace_v4(claim_id, &self.closure);
                (
                    resolution.governed_standing(),
                    serde_json::to_value(&resolution)?,
                )
            }
        };
        Ok(Resolution {
            governed,
            explanation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_and_full_policy_names_parse_and_nothing_else_does() {
        assert_eq!(PolicyChoice::parse("v2").unwrap(), PolicyChoice::V2);
        assert_eq!(
            PolicyChoice::parse("magpie-claims-standing-v4").unwrap(),
            PolicyChoice::V4
        );
        for text in ["latest", "v5", "V2", ""] {
            assert!(
                PolicyChoice::parse(text).is_err(),
                "{text:?} must not parse"
            );
        }
    }
}
