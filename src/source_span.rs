use crate::source_offset::SourceOffset;

/// Represents the start and length of the span.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}

impl SourceSpan {
    pub const fn new(start: SourceOffset, length: SourceOffset) -> Self {
        Self { offset: start, length: length.offset() }
    }
}
