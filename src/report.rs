use std::fmt;

use crate::{Diagnostic, SourceCode};

/// An error reporter.
///
/// API taken from [std::error::Report](https://doc.rust-lang.org/std/error/struct.Report.html).
pub struct Report<D = Box<dyn Diagnostic>> {
    diagnostic: D,

    source_code: Option<Box<dyn SourceCode>>,
}

impl<D> Report<D>
where
    Self: From<D>,
{
    pub fn new(error: D) -> Self {
        Self::from(error)
    }

    /// Provide source code for this error
    #[must_use]
    pub fn with_source_code(mut self, source_code: impl SourceCode + 'static) -> Self {
        self.source_code.replace(Box::new(source_code));
        self
    }
}

impl<D> From<D> for Report<D>
where
    D: Diagnostic,
{
    fn from(error: D) -> Self {
        Self { diagnostic: error, source_code: None }
    }
}

impl<D> AsRef<dyn Diagnostic> for Report<D>
where
    D: Diagnostic + 'static,
{
    fn as_ref(&self) -> &(dyn Diagnostic + 'static) {
        &self.diagnostic
    }
}

impl<D> fmt::Display for Report<D>
where
    D: Diagnostic,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Display")
    }
}

impl<D> fmt::Debug for Report<D>
where
    Self: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Debug")
    }
}
