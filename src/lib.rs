mod traits;

/// Default severity for diagnostics is `Severity::Error`.
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq, Ord)]
pub enum Severity {
    Warning,
    Error,
    Info,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Error
    }
}

/// A span with an optional label.
/// TODO(@anonrig): Why don't we merge this with SourceSpan and make the label
/// optional?
pub struct LabeledSourceSpan {
    label: Option<String>,
    span: SourceSpan,
}

/// Represents the start and length of the span.
pub struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}

/// Represents the offset from the beginning.
pub struct SourceOffset(usize);
