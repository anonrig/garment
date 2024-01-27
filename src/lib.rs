mod debug_report_handler;
mod diagnostic;
mod graphical_report_handler;
mod labeled_source_span;
mod panic;
mod report;
mod report_handler;
mod source_offset;
mod source_span;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::{
    debug_report_handler::DebugReportHandler, diagnostic::Diagnostic,
    graphical_report_handler::GraphicalReportHandler, labeled_source_span::LabeledSourceSpan,
    report::Report, report_handler::ReportHandler, source_offset::SourceOffset,
    source_span::SourceSpan,
};

pub type ByteOffset = usize;

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
