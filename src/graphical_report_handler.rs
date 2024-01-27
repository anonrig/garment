use std::fmt;

use crate::Diagnostic;

#[derive(Debug, Default, Clone)]
pub struct GraphicalReportHandler;

impl GraphicalReportHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn render_report(
        &self,
        f: &mut impl fmt::Write,
        diagnostic: &(dyn Diagnostic),
    ) -> fmt::Result {
        write!(f, "severity:{:?}", diagnostic.severity().unwrap_or_default())?;
        Ok(())
    }
}
