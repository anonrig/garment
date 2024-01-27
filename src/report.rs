use std::fmt;

use crate::diagnostic::Diagnostic;

/// An error reporter.
///
/// API taken from [std::error::Report](https://doc.rust-lang.org/std/error/struct.Report.html).
pub struct Report<D = Box<dyn Diagnostic>> {
    diagnostic: D,
}

impl<D> Report<D>
where
    Report<D>: From<D>,
{
    pub fn new(error: D) -> Report<D> {
        Self::from(error)
    }
}

impl<D> From<D> for Report<D>
where
    D: Diagnostic,
{
    fn from(error: D) -> Self {
        Self { diagnostic: error }
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
    Report<D>: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Debug")
    }
}
