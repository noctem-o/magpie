//! Crate-internal raw-input frontend for the frozen portable-verifier language.
//!
//! This module deliberately remains separate from the compatibility-only
//! `LogReader` path. It is the staged input boundary for the later complete
//! portable-history conformer; it is not itself a complete verifier.

mod framing;
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
mod tests;
