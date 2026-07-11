//! Closed ADR-0002 policy vocabulary and support/refutation ceilings.
//!
//! This module encodes only the ceiling policy surfaces. A support ceiling is
//! the maximum positive contribution policy permits. A refutation ceiling is
//! the maximum negative contribution policy permits. Neither is automatic
//! achieved-standing aggregation.

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

/// Privileged context required before a support-policy cell could be admitted.
///
/// This classification does not admit, trust, aggregate, or promote a
/// candidate. In particular, [`SupportContextRequirement::NoPrivilegedContext`]
/// means only that this surface requires no HumanRoot or verifier-specific
/// privileged context; all other policy and achieved-standing rules remain
/// separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupportContextRequirement {
    NoSupportContribution,
    NoPrivilegedContext,
    HumanAdmission,
    DeterministicVerifierContext,
    DeadboltVerifierContext,
}

/// Classifies the privileged context required by one closed support-policy cell.
///
/// The result is candidate policy only. It neither proves that the context
/// exists nor accepts an evidence-kind or actor-class label as authority.
pub fn support_context_requirement(
    kind: EvidenceKind,
    domain: ClaimDomain,
) -> SupportContextRequirement {
    match (kind, domain) {
        (EvidenceKind::DeterministicVerification, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::DeterministicVerifierContext
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::DeterministicVerifierContext
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::DeterministicVerifierContext
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::HumanRatification, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::HumanRatification, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::HumanRatification, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::HumanAdmission
        }
        (EvidenceKind::HumanRatification, ClaimDomain::Interpretation) => {
            SupportContextRequirement::HumanAdmission
        }
        (EvidenceKind::HumanRatification, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::HumanRatification, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::HumanAdmission
        }

        (EvidenceKind::DeadboltAnchor, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::DeadboltVerifierContext
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::DeadboltVerifierContext
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::DeadboltVerifierContext
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::Interpretation) => {
            SupportContextRequirement::DeadboltVerifierContext
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::DeadboltAnchor, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::ExecutionEvidence, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExecutionEvidence, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::ExternalSource, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExternalSource, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExternalSource, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExternalSource, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::ExternalSource, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::ExternalSource, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ExternalSource, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::ModelSelfReport, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::ModelSelfReport, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }

        (EvidenceKind::LensReadout, ClaimDomain::OccurrenceInclusion) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::LensReadout, ClaimDomain::ExactMachineCheckable) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::LensReadout, ClaimDomain::OperationalObservation) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::LensReadout, ClaimDomain::Interpretation) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::LensReadout, ClaimDomain::ExternalReport) => {
            SupportContextRequirement::NoSupportContribution
        }
        (EvidenceKind::LensReadout, ClaimDomain::ModelIntrospection) => {
            SupportContextRequirement::NoPrivilegedContext
        }
        (EvidenceKind::LensReadout, ClaimDomain::HumanJudgment) => {
            SupportContextRequirement::NoSupportContribution
        }
    }
}

/// Maximum negative refutation contribution for an evidence kind and claim domain.
///
/// `None` means this evidence kind has no admissible direct negative
/// refutation contribution for the domain. This is only a policy ceiling, not
/// achieved standing and not automatic refutation.
pub fn refutation_ceiling(kind: EvidenceKind, domain: ClaimDomain) -> Option<Status> {
    match (kind, domain) {
        (EvidenceKind::DeterministicVerification, ClaimDomain::OccurrenceInclusion) => {
            Some(Status::Refuted)
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExactMachineCheckable) => {
            Some(Status::Refuted)
        }
        (EvidenceKind::DeterministicVerification, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::Interpretation) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::DeterministicVerification, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::HumanRatification, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::Interpretation) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::DeadboltAnchor, ClaimDomain::OccurrenceInclusion) => Some(Status::Refuted),
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::Interpretation) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::DeadboltAnchor, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ExecutionEvidence, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::Interpretation) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::ExecutionEvidence, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::Interpretation) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::BehavioralEvaluation, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ExternalSource, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::Interpretation) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::ExternalSource, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::ModelSelfReport, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::Interpretation) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::ModelSelfReport, ClaimDomain::HumanJudgment) => None,

        (EvidenceKind::LensReadout, ClaimDomain::OccurrenceInclusion) => None,
        (EvidenceKind::LensReadout, ClaimDomain::ExactMachineCheckable) => None,
        (EvidenceKind::LensReadout, ClaimDomain::OperationalObservation) => None,
        (EvidenceKind::LensReadout, ClaimDomain::Interpretation) => None,
        (EvidenceKind::LensReadout, ClaimDomain::ExternalReport) => None,
        (EvidenceKind::LensReadout, ClaimDomain::ModelIntrospection) => None,
        (EvidenceKind::LensReadout, ClaimDomain::HumanJudgment) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_SUPPORT_MATRIX: [(EvidenceKind, ClaimDomain, Option<Status>); 56] = [
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

    const EXPECTED_REFUTATION_MATRIX: [(EvidenceKind, ClaimDomain, Option<Status>); 56] = [
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::OccurrenceInclusion,
            Some(Status::Refuted),
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExactMachineCheckable,
            Some(Status::Refuted),
        ),
        (
            EvidenceKind::DeterministicVerification,
            ClaimDomain::OperationalObservation,
            None,
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
            None,
        ),
        (
            EvidenceKind::HumanRatification,
            ClaimDomain::Interpretation,
            None,
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
            None,
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::OccurrenceInclusion,
            Some(Status::Refuted),
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::ExactMachineCheckable,
            None,
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::OperationalObservation,
            None,
        ),
        (
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::Interpretation,
            None,
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
            None,
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
            None,
        ),
        (
            EvidenceKind::BehavioralEvaluation,
            ClaimDomain::Interpretation,
            None,
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
            None,
        ),
        (
            EvidenceKind::ExternalSource,
            ClaimDomain::ExternalReport,
            None,
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
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::ExternalReport,
            None,
        ),
        (
            EvidenceKind::ModelSelfReport,
            ClaimDomain::ModelIntrospection,
            None,
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
        (EvidenceKind::LensReadout, ClaimDomain::Interpretation, None),
        (EvidenceKind::LensReadout, ClaimDomain::ExternalReport, None),
        (
            EvidenceKind::LensReadout,
            ClaimDomain::ModelIntrospection,
            None,
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
            EXPECTED_SUPPORT_MATRIX.len(),
            EvidenceKind::ALL.len() * ClaimDomain::ALL.len()
        );

        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                let matching_cells = EXPECTED_SUPPORT_MATRIX
                    .iter()
                    .filter(|(expected_kind, expected_domain, _)| {
                        *expected_kind == kind && *expected_domain == domain
                    })
                    .count();
                assert_eq!(matching_cells, 1, "{kind} x {domain} appears once");

                let (_, _, expected) = EXPECTED_SUPPORT_MATRIX
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

    #[test]
    fn support_context_requirement_covers_full_matrix() {
        let mut cell_count = 0usize;

        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                match support_context_requirement(kind, domain) {
                    SupportContextRequirement::NoSupportContribution
                    | SupportContextRequirement::NoPrivilegedContext
                    | SupportContextRequirement::HumanAdmission
                    | SupportContextRequirement::DeterministicVerifierContext
                    | SupportContextRequirement::DeadboltVerifierContext => {}
                }
                cell_count += 1;
            }
        }

        assert_eq!(cell_count, 56);
    }

    #[test]
    fn support_context_requirement_never_revives_missing_support_cell() {
        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                assert_eq!(
                    support_context_requirement(kind, domain)
                        == SupportContextRequirement::NoSupportContribution,
                    support_ceiling(kind, domain).is_none(),
                    "{kind} x {domain}"
                );
            }
        }
    }

    #[test]
    fn support_context_requirement_category_counts_are_frozen() {
        let mut counts = [0usize; 5];

        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                let index = match support_context_requirement(kind, domain) {
                    SupportContextRequirement::NoSupportContribution => 0,
                    SupportContextRequirement::HumanAdmission => 1,
                    SupportContextRequirement::DeterministicVerifierContext => 2,
                    SupportContextRequirement::DeadboltVerifierContext => 3,
                    SupportContextRequirement::NoPrivilegedContext => 4,
                };
                counts[index] += 1;
            }
        }

        assert_eq!(counts, [37, 3, 3, 4, 9]);
    }

    #[test]
    fn support_context_requirement_doctrine_sensitive_cells_are_explicit() {
        use SupportContextRequirement::{
            DeadboltVerifierContext, DeterministicVerifierContext, HumanAdmission,
            NoPrivilegedContext, NoSupportContribution,
        };

        for (kind, domain, expected) in [
            (
                EvidenceKind::HumanRatification,
                ClaimDomain::HumanJudgment,
                HumanAdmission,
            ),
            (
                EvidenceKind::HumanRatification,
                ClaimDomain::OccurrenceInclusion,
                NoSupportContribution,
            ),
            (
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
                DeterministicVerifierContext,
            ),
            (
                EvidenceKind::DeterministicVerification,
                ClaimDomain::Interpretation,
                NoSupportContribution,
            ),
            (
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion,
                DeadboltVerifierContext,
            ),
            (
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::Interpretation,
                DeadboltVerifierContext,
            ),
            (
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::ExternalReport,
                NoSupportContribution,
            ),
            (
                EvidenceKind::ExternalSource,
                ClaimDomain::ExternalReport,
                NoPrivilegedContext,
            ),
            (
                EvidenceKind::ModelSelfReport,
                ClaimDomain::ModelIntrospection,
                NoPrivilegedContext,
            ),
            (
                EvidenceKind::LensReadout,
                ClaimDomain::Interpretation,
                NoPrivilegedContext,
            ),
        ] {
            assert_eq!(support_context_requirement(kind, domain), expected);
        }
    }

    #[test]
    fn full_refutation_ceiling_matrix_matches_docs() {
        assert_eq!(
            EXPECTED_REFUTATION_MATRIX.len(),
            EvidenceKind::ALL.len() * ClaimDomain::ALL.len()
        );

        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                let matching_cells = EXPECTED_REFUTATION_MATRIX
                    .iter()
                    .filter(|(expected_kind, expected_domain, _)| {
                        *expected_kind == kind && *expected_domain == domain
                    })
                    .count();
                assert_eq!(matching_cells, 1, "{kind} x {domain} appears once");

                let (_, _, expected) = EXPECTED_REFUTATION_MATRIX
                    .iter()
                    .find(|(expected_kind, expected_domain, _)| {
                        *expected_kind == kind && *expected_domain == domain
                    })
                    .expect("all cells are covered");
                assert_eq!(
                    refutation_ceiling(kind, domain),
                    *expected,
                    "{kind} x {domain}"
                );
            }
        }
    }

    #[test]
    fn refutation_ceiling_matrix_has_56_cells() {
        assert_eq!(EXPECTED_REFUTATION_MATRIX.len(), 56);
        assert_eq!(EvidenceKind::ALL.len() * ClaimDomain::ALL.len(), 56);
    }

    #[test]
    fn refutation_ceiling_only_returns_refuted_or_none() {
        for kind in EvidenceKind::ALL {
            for domain in ClaimDomain::ALL {
                assert!(
                    matches!(
                        refutation_ceiling(kind, domain),
                        Some(Status::Refuted) | None
                    ),
                    "{kind} x {domain} returned a non-refutation status"
                );
            }
        }
    }

    #[test]
    fn deterministic_verification_can_refute_exact_domains() {
        assert_eq!(
            refutation_ceiling(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::OccurrenceInclusion
            ),
            Some(Status::Refuted)
        );
        assert_eq!(
            refutation_ceiling(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable
            ),
            Some(Status::Refuted)
        );

        for domain in [
            ClaimDomain::OperationalObservation,
            ClaimDomain::Interpretation,
            ClaimDomain::ExternalReport,
            ClaimDomain::ModelIntrospection,
            ClaimDomain::HumanJudgment,
        ] {
            assert_eq!(
                refutation_ceiling(EvidenceKind::DeterministicVerification, domain),
                None,
                "DeterministicVerification x {domain}"
            );
        }
    }

    #[test]
    fn deadbolt_anchor_can_refute_occurrence_inclusion_only() {
        assert_eq!(
            refutation_ceiling(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion
            ),
            Some(Status::Refuted)
        );

        for domain in [
            ClaimDomain::ExactMachineCheckable,
            ClaimDomain::OperationalObservation,
            ClaimDomain::Interpretation,
            ClaimDomain::ExternalReport,
            ClaimDomain::ModelIntrospection,
            ClaimDomain::HumanJudgment,
        ] {
            assert_eq!(
                refutation_ceiling(EvidenceKind::DeadboltAnchor, domain),
                None,
                "DeadboltAnchor x {domain}"
            );
        }
    }

    #[test]
    fn human_ratification_never_refutes() {
        for domain in ClaimDomain::ALL {
            assert_eq!(
                refutation_ceiling(EvidenceKind::HumanRatification, domain),
                None,
                "HumanRatification x {domain}"
            );
        }
    }

    #[test]
    fn external_source_never_refutes() {
        for domain in ClaimDomain::ALL {
            assert_eq!(
                refutation_ceiling(EvidenceKind::ExternalSource, domain),
                None,
                "ExternalSource x {domain}"
            );
        }
    }

    #[test]
    fn model_self_report_and_lens_readout_never_refute() {
        for kind in [EvidenceKind::ModelSelfReport, EvidenceKind::LensReadout] {
            for domain in ClaimDomain::ALL {
                assert_eq!(refutation_ceiling(kind, domain), None, "{kind} x {domain}");
            }
        }
    }

    #[test]
    fn execution_and_behavioral_evidence_do_not_directly_refute() {
        for kind in [
            EvidenceKind::ExecutionEvidence,
            EvidenceKind::BehavioralEvaluation,
        ] {
            for domain in ClaimDomain::ALL {
                assert_eq!(refutation_ceiling(kind, domain), None, "{kind} x {domain}");
            }
        }
    }

    #[test]
    fn support_and_refutation_ceilings_are_separate() {
        assert_eq!(
            support_ceiling(EvidenceKind::ExternalSource, ClaimDomain::ExternalReport),
            Some(Status::Supported)
        );
        assert_eq!(
            refutation_ceiling(EvidenceKind::ExternalSource, ClaimDomain::ExternalReport),
            None
        );

        assert_eq!(
            support_ceiling(EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment),
            Some(Status::Supported)
        );
        assert_eq!(
            refutation_ceiling(EvidenceKind::HumanRatification, ClaimDomain::HumanJudgment),
            None
        );

        assert_eq!(
            support_ceiling(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion
            ),
            Some(Status::Settled)
        );
        assert_eq!(
            refutation_ceiling(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion
            ),
            Some(Status::Refuted)
        );
    }
}
