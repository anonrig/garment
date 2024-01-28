use std::sync::Arc;

pub trait SourceCode: Send + Sync {
    fn read_span(&self);
}

impl SourceCode for str {
    fn read_span(&self) {}
}

impl<'s> SourceCode for &'s str {
    fn read_span(&self) {}
}

impl SourceCode for String {
    fn read_span(&self) {}
}

impl<T: ?Sized + SourceCode> SourceCode for Arc<T> {
    fn read_span(&self) {}
}
