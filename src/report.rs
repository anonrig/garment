use std::{fmt, ops::Deref};

use crate::{Diagnostic, SourceCode};

/// An error reporter.
///
/// API taken from [std::error::Report](https://doc.rust-lang.org/std/error/struct.Report.html).
pub struct Report<D = Box<dyn Diagnostic + Send + Sync>> {
    diagnostic: D,

    source_code: Option<Box<dyn SourceCode>>,
}

impl Report {
    #[must_use]
    pub fn new_boxed(diagnostic: Box<dyn Diagnostic + Send + Sync + 'static>) -> Self {
        Self { diagnostic, source_code: None }
    }

    /// Provide source code for this error
    #[must_use]
    pub fn with_source_code(mut self, source_code: impl SourceCode + 'static) -> Self {
        self.source_code.replace(Box::new(source_code));
        self
    }
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

impl<D> Deref for Report<D>
where
    D: Diagnostic + 'static,
{
    type Target = dyn Diagnostic + 'static;

    fn deref(&self) -> &Self::Target {
        &self.diagnostic
    }
}

impl Deref for Report {
    type Target = dyn Diagnostic + 'static;

    fn deref(&self) -> &Self::Target {
        &*self.diagnostic
    }
}

impl AsRef<dyn Diagnostic> for Report {
    fn as_ref(&self) -> &(dyn Diagnostic + 'static) {
        &*self.diagnostic
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

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Display")
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
