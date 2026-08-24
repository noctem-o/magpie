use super::FrontendRejection;

const UTF8_BOM: &[u8; 3] = b"\xef\xbb\xbf";

pub(super) struct FrameCursor<'input> {
    input: &'input [u8],
    offset: usize,
    line: usize,
    record_index: usize,
}

pub(super) enum FrameStep<'input> {
    End,
    Rejected(FrontendRejection),
    Candidate(FramedCandidate<'input>),
}

pub(super) struct FramedCandidate<'input> {
    text: &'input str,
    line: usize,
    record_index: usize,
    continuation: FrameCursor<'input>,
}

impl<'input> FrameCursor<'input> {
    pub(super) fn new(input: &'input [u8]) -> Self {
        Self {
            input,
            offset: 0,
            line: 1,
            record_index: 0,
        }
    }

    pub(super) fn next(self) -> FrameStep<'input> {
        if self.offset == self.input.len() {
            return FrameStep::End;
        }

        let remaining = &self.input[self.offset..];
        let lf_offset = remaining.iter().position(|byte| *byte == b'\n');
        let (candidate, next_offset, next_line) = match lf_offset {
            Some(relative_lf) => {
                let bytes_before_lf = &remaining[..relative_lf];
                let candidate = bytes_before_lf
                    .strip_suffix(b"\r")
                    .unwrap_or(bytes_before_lf);
                (candidate, self.offset + relative_lf + 1, self.line + 1)
            }
            None => (remaining, self.input.len(), self.line),
        };

        if candidate.is_empty() {
            return FrameStep::Rejected(FrontendRejection::framing(self.line, None));
        }

        let current_index = self.record_index;
        if candidate.iter().all(|byte| matches!(byte, b' ' | b'\t'))
            || candidate.contains(&b'\r')
            || candidate.starts_with(UTF8_BOM)
        {
            return FrameStep::Rejected(FrontendRejection::framing(self.line, Some(current_index)));
        }

        let text = match std::str::from_utf8(candidate) {
            Ok(text) => text,
            Err(_) => {
                return FrameStep::Rejected(FrontendRejection::framing(
                    self.line,
                    Some(current_index),
                ));
            }
        };

        FrameStep::Candidate(FramedCandidate {
            text,
            line: self.line,
            record_index: current_index,
            continuation: FrameCursor {
                input: self.input,
                offset: next_offset,
                line: next_line,
                record_index: current_index + 1,
            },
        })
    }
}

impl<'input> FramedCandidate<'input> {
    pub(super) fn text(&self) -> &'input str {
        self.text
    }

    pub(super) fn line(&self) -> usize {
        self.line
    }

    pub(super) fn record_index(&self) -> usize {
        self.record_index
    }

    pub(super) fn into_continuation(self) -> FrameCursor<'input> {
        self.continuation
    }
}
