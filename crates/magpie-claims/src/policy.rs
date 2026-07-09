//! Closed ADR-0002 policy vocabulary and support ceilings.
//!
//! This module encodes only the support-ceiling surface. A ceiling is the
//! maximum positive contribution policy permits; it is not automatic promotion.

use std::{error::Error, fmt, str::FromStr};

use magpie_log::Status;

/// ADR-0002 evidence kinds admitted by the standing policy vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceKind {
    DeterministicVerification,
    HumanRatification,
    DeadboltAnchor,
    ExecutionEvidence,
    BehavioralEvaluation,
    ExternalSource,
    ModelSelfReport,
    LensReadout,
}

impl EvidenceKind {
    pub const ALL: [Self; 8] = [
        Self::DeterministicVerification,
        Self::HumanRatification,
        Self::DeadboltAnchor,
        Self::ExecutionEvidence,
        Self::BehavioralEvaluation,
        Self::ExternalSource,
        Self::ModelSelfReport,
        Self::LensReadout,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicVerification => "DeterministicVerification",
            Self::HumanRatification => "HumanRatification",
            Self::DeadboltAnchor => "DeadboltAnchor",
            Self::ExecutionEvidence => "ExecutionEvidence",
            Self::BehavioralEvaluation => "BehavioralEvaluation",
            Self::ExternalSource => "ExternalSource",
            Self::ModelSelfReport => "ModelSelfReport",
            Self::LensReadout => "LensReadout",
        }
    }
}

impl TryFrom<&str> for EvidenceKind {
    type Error = PolicyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DeterministicVerification" => Ok(Self::DeterministicVerification),
            "HumanRatification" => Ok(Self::HumanRatification),
            "DeadboltAnchor" => Ok(Self::DeadboltAnchor),
            "ExecutionEvidence" => Ok(Self::ExecutionEvidence),
            "BehavioralEvaluation" => Ok(Self::BehavioralEvaluation),
            "ExternalSource" => Ok(Self::ExternalSource),
            "ModelSelfReport" => Ok(Self::ModelSelfReport),
            "LensReadout" => Ok(Self::LensReadout),
            _ => Err(PolicyParseError::unknown("evidence_kind", value)),
        }
    }
}

impl FromStr for EvidenceKind {
    type Err = PolicyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl fmt::Display for EvidenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// ADR-0002 claim domains admitted by the standing policy vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimDomain {
    OccurrenceInclusion,
    ExactMachineCheckable,
    OperationalObservation,
    Interpretation,
    ExternalReport,
    ModelIntrospection,
    HumanJudgment,
}

impl ClaimDomain {
    pub const ALL: [Self; 7] = [
        Self::OccurrenceInclusion,
        Self::ExactMachineCheckable,
        Self::OperationalObservation,
        Self::Interpretation,
        Self::ExternalReport,
        Self::ModelIntrospection,
        Self::HumanJudgment,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::OccurrenceInclusion => "Occurrence/Inclusion",
            Self::ExactMachineCheckable => "ExactMachineCheckable",
            Self::OperationalObservation => "OperationalObservation",
            Self::Interpretation => "Interpretation",
            Self::ExternalReport => "ExternalReport",
            Self::ModelIntrospection => "ModelIntrospection",
            Self::HumanJudgment => "HumanJudgment",
        }
    }
}

impl TryFrom<&str> for ClaimDomain {
    type Error = PolicyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Occurrence/Inclusion" => Ok(Self::OccurrenceInclusion),
            "ExactMachineCheckable" => Ok(Self::ExactMachineCheckable),
            "OperationalObservation" => Ok(Self::OperationalObservation),
            "Interpretation" => Ok(Self::Interpretation),
            "ExternalReport" => Ok(Self::ExternalReport),
            "ModelIntrospection" => Ok(Self::ModelIntrospection),
            "HumanJudgment" => Ok(Self::HumanJudgment),
            _ => Err(PolicyParseError::unknown("claim_domain", value)),
        }
    }
}

impl FromStr for ClaimDomain {
    type Err = PolicyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl fmt::Display for ClaimDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when a string is not in a closed policy vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyParseError {
    vocabulary: &'static str,
    value: String,
}

impl PolicyParseError {
    fn unknown(vocabulary: &'static str, value: &str) -> Self {
        Self {
            vocabulary,
            value: value.to_owned(),
        }
    }

    pub fn vocabulary(&self) -> &'static str {
        self.vocabulary
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for PolicyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unknown {} value {:?}; expected a closed ADR-0002 policy value",
            self.vocabulary, self.value
        )
    }
}

impl Error for PolicyParseError {}

/// Maximum positive support contribution for an evidence kind and claim domain.
///
/// `None` means this evidence kind has no admissible positive support
/// contribution for the domain. This is only a policy ceiling, not achieved
/// standing and not automatic promotion.
pub fn support_ceiling(kind: EvidenceKind, domain: ClaimDomain) -> Option<Status> {
    match (kind, domain) {
        (EvidenceKind::DeterministicVerification, ClaimDomain::OccurrenceInclusion) => {
            Some(Status::Settled)
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExactMachineCheckable) => {
            Some(Status::Settled)
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::OperationalObservation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::Interpretation) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::HumanRatification, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::OperationalObservation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::HumanRatification, ClaimDomain::Interpretation) => Some(Status::Supported),
        (EvidenceKind::HumanRatification, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment) => Some(Status::Supported),

        (EvidenceKind::DeadboltAnchor, ClaimDomain::OccurrenceInclusion) => Some(Status::Settled),
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExactMachineCheckable) => {
            Some(Status::Supported)
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::OperationalObservation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::Interpretation) => Some(Status::Supported),
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ExecutionEvidence, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::OperationalObservation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::Interpretation) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OperationalObservation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::Interpretation) => {
            Some(Status::Supported)
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ExternalSource, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::Interpretation) => Some(Status::Supported),
        (EvidenceKind::ExternalSource, ClaimDomain::ExternalReport) => Some(Status::Supported),
        (EvidenceKind::ExternalSource, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ModelSelfReport, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::Interpretation) => Some(Status::Conjectured),
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::ModelIntrospection) => {
            Some(Status::Conjectured)
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::LensReadout, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::LensReadout, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::LensReadout, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::LensReadout, ClaimDomain::Interpretation) => Some(Status::Conjectured),
        (EvidenceKind::LensReadout, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::LensReadout, ClaimDomain::ModelIntrospection) => Some(Status::Conjectured),
        (EvidenceKind::LensReadout, ClaimDomain::HumanJudgment) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_MATRIX: [(EvidenceKind, ClaimDomain, Option<Status>); 56] = [
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::OccurrenceInclusion,
            Some(Status::Settled),
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExactMachineCheckable,
            Some(Status::Settled),
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::OperationalObservation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::Interpretation,
            None,
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::OperationalObservation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::Interpretation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::HumanJudgment,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::OccurrenceInclusion,
            Some(Status::Settled),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::ExactMachineCheckable,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::OperationalObservation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::Interpretation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::OperationalObservation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::Interpretation,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::ExecutionEvidence,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::OperationalObservation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::Interpretation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::OperationalObservation,
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::Interpretation,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::ExternalReport,
            Some(Status::Supported),
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::ModelIntrospection,
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::OperationalObservation,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::Interpretation,
            Some(Status::Conjectured),
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::ModelIntrospection,
            Some(Status::Conjectured),
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::HumanJudgment,
            None,
        ),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::OccurrenceInclusion,
            None,
        ),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::OperationalObservation,
            None,
        ),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::Interpretation,
            Some(Status::Conjectured),
        ),
        (EvidenceKind::LensReadout, ClaimDomain::ExternalReport, None),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::ModelIntrospection,
            Some(Status::Conjectured),
        ),
        (EvidenceKind::LensReadout, ClaimDomain::HumanJudgment, None),
    ];

    #[test]
    fn evidence_kinds_parse_from_exact_strings() {
        for kind in EvidenceKind::ALL {
            assert_eq!(EvidenceKind::try_from(kind.as_str()), Ok(kind));
            assert_eq!(kind.to_string(), kind.as_str());
        }
    }

    #[test]
    fn claim_domains_parse_from_exact_strings() {
        for domain in ClaimDomain::ALL {
            assert_eq!(ClaimDomain::try_from(domain.as_str()), Ok(domain));
            assert_eq!(domain.to_string(), domain.as_str());
        }
    }

    #[test]
    fn unknown_evidence_kinds_fail() {
        let error = EvidenceKind::try_from("HumanOverride").unwrap_err();

        assert_eq!(error.vocabulary(), "evidence_kind");
        assert_eq!(error.value(), "HumanOverride");
    }

    #[test]
    fn unknown_claim_domains_fail() {
        let error = ClaimDomain::try_from("OccurrenceInclusion").unwrap_err();

        assert_eq!(error.vocabulary(), "claim_domain");
        assert_eq!(error.value(), "OccurrenceInclusion");
    }

    #[test]
    fn full_support_ceiling_matrix_matches_docs() {
        assert_eq!(
            EXPECTED_MATRIX.len(),
            EvidenceKind::ALL.len() * ClaimDomain::ALL.len()
        );

        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                let matching_cells = EXPECTED_MATRIX
                    .iter()
                    .filter(|(expected_kind, expected_domain, _)| {
                        *expected_kind == kind && *expected_domain == domain
                    })
                    .count();
                assert_eq!(matching_cells, 1, "{kind} x {domain} appears once");

                let (_, _, expected) = EXPECTED_MATRIX
                    .iter()
                    .find(|(expected_kind, expected_domain, _)| {
                        *expected_kind == kind && *expected_domain == domain
                    })
                    .expect("all cells are covered");
                assert_eq!(
                    support_ceiling(kind, domain),
                    *expected,
                    "{kind} x {domain}"
                );
            }
        }
    }

    #[test]
    fn human_ratification_never_settles() {
        for domain in ClaimDomain::ALL {
            assert_ne!(
                support_ceiling(EvidenceKind::HumanRatification, domain),
                Some(Status::Settled),
                "HumanRatification x {domain}"
            );
        }
    }

    #[test]
    fn human_ratification_supported_domains_are_explicit() {
        assert_eq!(
            support_ceiling(EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment),
            Some(Status::Supported)
        );
        assert_eq!(
            support_ceiling(EvidenceKind::HumanRatification, ClaimDomain::Interpretation),
            Some(Status::Supported)
        );
        assert_eq!(
            support_ceiling(
                EvidenceKind::HumanRatification,
                ClaimDomain::OperationalObservation
            ),
            Some(Status::Supported)
        );
        assert_eq!(
            support_ceiling(
                EvidenceKind::HumanRatification,
                ClaimDomain::OccurrenceInclusion
            ),
            None
        );
        assert_eq!(
            support_ceiling(
                EvidenceKind::HumanRatification,
                ClaimDomain::ExactMachineCheckable
            ),
            None
        );
    }

    #[test]
    fn model_self_report_and_lens_readout_never_exceed_conjectured() {
        for kind in [EvidenceKind::ModelSelfReport, EvidenceKind::LensReadout] {
            for domain in ClaimDomain::ALL {
                assert!(
                    matches!(
                        support_ceiling(kind, domain),
                        None | Some(Status::Conjectured)
                    ),
                    "{kind} x {domain} exceeded Conjectured"
                );
            }
        }
    }

    #[test]
    fn deadbolt_anchor_settles_occurrence_but_not_interpretation() {
        assert_eq!(
            support_ceiling(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion
            ),
            Some(Status::Settled)
        );
        assert_eq!(
            support_ceiling(EvidenceKind::DeadboltAnchor, ClaimDomain::Interpretation),
            Some(Status::Supported)
        );
    }
}
