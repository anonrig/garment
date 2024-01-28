use crate::{LabeledSourceSpan, Severity};
use std::{error::Error, fmt::Display};

pub trait Diagnostic: Error {
    /// A unique identifier/code for the error.
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// The severity of the error.
    fn severity(&self) -> Option<Severity> {
        None
    }

    /// Additional help text for the error.
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// Labels to apply to this `Diagnostic`'s [`Diagnostic::code`]
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSourceSpan> + '_>> {
        None
    }

    /// Additional related `Diagnostic`s.
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        None
    }

    /// URL to visit for a more detailed description of the error.
    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        None
    }

    /// The cause of the error.
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}
