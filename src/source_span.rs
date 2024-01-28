use crate::{ByteOffset, SourceOffset};

/// Represents the start and length of the span.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}

impl SourceSpan {
    #[must_use]
    pub const fn new(start: SourceOffset, length: SourceOffset) -> Self {
        Self { offset: start, length: length.offset() }
    }
}

impl From<(ByteOffset, usize)> for SourceSpan {
    fn from((start, len): (ByteOffset, usize)) -> Self {
        Self { offset: SourceOffset::from(start), length: len }
    }
}
