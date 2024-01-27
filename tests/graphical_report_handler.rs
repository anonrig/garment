use garment::{Diagnostic, GraphicalReportHandler, Report, Severity};

#[test]
fn test() {
    use thiserror::Error;
    #[derive(Error, Debug)]
    #[error("TestError")]
    struct TestError;
    impl Diagnostic for TestError {
        fn severity(&self) -> Option<Severity> {
            Some(Severity::Error)
        }
    }
    let error = Report::new(TestError);

    let mut out = String::new();
    GraphicalReportHandler::new().render_report(&mut out, error.as_ref()).unwrap();
    assert_eq!(&out, "severity:Error")
}
