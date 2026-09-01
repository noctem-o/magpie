//! Crate-internal complete conformer for the frozen portable-verifier language.
//!
//! The exact-byte [`crate::FileStore`] adapter is the narrow production/file
//! surface for this language. It remains separate from compatibility-only
//! `LogReader`; the frontend owns exact input recognition through Schema and
//! the conformer owns the six remaining stages and complete outcome.

mod conformer;
mod file;
mod framing;
mod frontend;
mod json_syntax;
mod lexical;
mod preflight;
mod schema;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FrontendRejectionClass {
    ExternalKey,
    Framing,
    JsonSyntax,
    Schema,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FrontendRejection {
    class: FrontendRejectionClass,
    line: Option<usize>,
    record_index: Option<usize>,
}

impl FrontendRejection {
    fn external_key() -> Self {
        Self {
            class: FrontendRejectionClass::ExternalKey,
            line: None,
            record_index: None,
        }
    }

    fn framing(line: usize, record_index: Option<usize>) -> Self {
        Self {
            class: FrontendRejectionClass::Framing,
            line: Some(line),
            record_index,
        }
    }

    fn current_record(class: FrontendRejectionClass, line: usize, record_index: usize) -> Self {
        debug_assert!(matches!(
            class,
            FrontendRejectionClass::JsonSyntax | FrontendRejectionClass::Schema
        ));
        Self {
            class,
            line: Some(line),
            record_index: Some(record_index),
        }
    }

    pub(crate) fn class(&self) -> FrontendRejectionClass {
        self.class
    }

    pub(crate) fn line(&self) -> Option<usize> {
        self.line
    }

    pub(crate) fn record_index(&self) -> Option<usize> {
        self.record_index
    }
}

#[cfg(test)]
mod assurance;
#[cfg(test)]
mod corpus_tests;
#[cfg(test)]
mod tests;
