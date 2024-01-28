use std::ops::Range;

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

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset.offset()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl From<(ByteOffset, usize)> for SourceSpan {
    fn from((start, len): (ByteOffset, usize)) -> Self {
        Self { offset: SourceOffset::from(start), length: len }
    }
}

impl From<Range<ByteOffset>> for SourceSpan {
    fn from(range: Range<ByteOffset>) -> Self {
        Self { offset: range.start.into(), length: range.len() }
    }
}
