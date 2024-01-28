use crate::{source_offset::SourceOffset, source_span::SourceSpan, ByteOffset};

/// A span with an optional label.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LabeledSourceSpan {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    label: Option<String>,
    span: SourceSpan,
}

impl LabeledSourceSpan {
    #[must_use]
    pub const fn new(label: Option<String>, offset: ByteOffset, len: usize) -> Self {
        Self { label, span: SourceSpan::new(SourceOffset::new(offset), SourceOffset::new(len)) }
    }

    pub fn new_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self {
        Self { label, span: span.into() }
    }
}
