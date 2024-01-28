use garment::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("TestError")]
#[diagnostic(severity(warning))]
struct TestError;

fn main() {
    let error = TestError;
    println!("{error:?}");
}
