use garment::{Diagnostic, Report};

use thiserror::Error;
#[derive(Error, Debug)]
#[error("TestError")]
struct TestError;
impl Diagnostic for TestError {}

fn get_error() -> Report<TestError> {
    let test_error = TestError;
    Report::new(test_error)
}

#[test]
fn display() {
    let error = get_error();
    assert_eq!(format!("{error}"), "TODO: fmt::Display");
}

#[test]
fn debug() {
    let error = get_error();
    assert_eq!(format!("{error:?}"), "TODO: fmt::Debug");
}

#[test]
fn send_sync() {
    use std::{sync::Arc, thread};
    let error = Arc::new(get_error());
    for _ in 0..2 {
        let error = Arc::clone(&error);
        // This line will fail to compile with
        // `error[E0277]: `___` cannot be shared between threads safely`
        // if `Error` does not implement `Send` and `Sync`.
        _ = thread::spawn(move || format!("{error}")).join();
    }
}
