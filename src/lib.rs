mod error;
mod panic;
mod traits;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::{error::Error, traits::Diagnostic};

/// Default severity for diagnostics is `Severity::Error`.
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Severity {
    Warning,
    Error,
    Info,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Error
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_serialize_severity() {
    use serde_json::json;

    assert_eq!(json!(Severity::Info), json!("Info"));
    assert_eq!(json!(Severity::Warning), json!("Warning"));
    assert_eq!(json!(Severity::Error), json!("Error"));
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_severity() {
    use serde_json::json;

    let severity: Severity = serde_json::from_value(json!("Info")).unwrap();
    assert_eq!(severity, Severity::Info);

    let severity: Severity = serde_json::from_value(json!("Warning")).unwrap();
    assert_eq!(severity, Severity::Warning);

    let severity: Severity = serde_json::from_value(json!("Error")).unwrap();
    assert_eq!(severity, Severity::Error);
}

/// A span with an optional label.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LabeledSourceSpan {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    label: Option<String>,
    span: SourceSpan,
}

/// Represents the start and length of the span.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}

/// Represents the offset from the beginning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceOffset(usize);
