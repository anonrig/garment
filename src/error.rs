use std::{fmt, marker::PhantomData};

use crate::traits::Diagnostic;

pub struct Error {
    // A stub for getting the APIs working.
    // To be removed and replaced with the real thing.
    _marker: std::marker::PhantomData<*const ()>,
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be thread safe and `'static`, so that the `Error`
    /// will be as well.
    ///
    /// If the error type does not provide a backtrace, a backtrace will be
    /// created here to ensure that a backtrace exists.
    pub fn new<E>(_error: E) -> Self
    where
        E: Diagnostic + Send + Sync + 'static,
    {
        Self { _marker: PhantomData }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Display")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: fmt::Error")
    }
}
