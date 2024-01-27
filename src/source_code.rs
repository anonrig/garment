pub trait SourceCode: Send + Sync {}

impl SourceCode for String {}
