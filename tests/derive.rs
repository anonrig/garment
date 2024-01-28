use garment::Diagnostic;
use thiserror::Error;

#[test]
fn test() {
    #[derive(Error, Debug, Diagnostic)]
    #[error("TestError")]
    struct TestError;
}
