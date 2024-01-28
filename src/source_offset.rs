use crate::ByteOffset;

/// Represents the offset from the beginning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceOffset(usize);

impl SourceOffset {
    #[must_use]
    pub const fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Actual byte offset.
    #[must_use]
    pub const fn offset(&self) -> ByteOffset {
        self.0
    }
}

impl From<ByteOffset> for SourceOffset {
    fn from(bytes: ByteOffset) -> Self {
        Self(bytes)
    }
}
