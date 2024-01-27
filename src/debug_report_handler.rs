use std::fmt;

use crate::Diagnostic;

#[derive(Debug, Default, Clone)]
pub struct DebugReportHandler;

impl DebugReportHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn render_report(
        &self,
        f: &mut fmt::Formatter<'_>,
        diagnostic: &(dyn Diagnostic),
    ) -> fmt::Result {
        let mut diag = f.debug_struct("Diagnostic");
        diag.field("message", &format!("{}", diagnostic));
        if let Some(code) = diagnostic.code() {
            diag.field("code", &code.to_string());
        }
        if let Some(severity) = diagnostic.severity() {
            diag.field("severity", &format!("{:?}", severity));
        }
        if let Some(url) = diagnostic.url() {
            diag.field("url", &url.to_string());
        }
        if let Some(help) = diagnostic.help() {
            diag.field("help", &help.to_string());
        }
        if let Some(labels) = diagnostic.labels() {
            let labels: Vec<_> = labels.collect();
            diag.field("labels", &format!("{:?}", labels));
        }
        if let Some(cause) = diagnostic.diagnostic_source() {
            diag.field("caused by", &format!("{:?}", cause));
        }
        diag.finish()?;
        writeln!(f)?;
        Ok(())
    }
}
