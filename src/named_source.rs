use std::fmt::Debug;

use crate::SourceCode;

pub struct NamedSource {
    source: Box<dyn SourceCode + 'static>,
    name: String,
}

impl Debug for NamedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamedSource").field("name", &self.name).field("source", &"<redacted>");
        Ok(())
    }
}

impl NamedSource {
    pub fn new(name: impl AsRef<str>, source: impl SourceCode + Send + Sync + 'static) -> Self {
        Self { source: Box::new(source), name: name.as_ref().to_string() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inner(&self) -> &(dyn SourceCode + 'static) {
        &*self.source
    }
}

impl SourceCode for NamedSource {}
