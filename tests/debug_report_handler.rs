use std::fmt::{self, Display};

use garment::{DebugReportHandler, Diagnostic, LabeledSourceSpan, Report, Severity};

#[test]
fn test() {
    use thiserror::Error;
    #[derive(Error, Debug)]
    #[error("TestError")]
    struct SourceError;
    impl Diagnostic for SourceError {}

    #[derive(Error, Debug)]
    #[error("TestError")]
    struct TestError {
        source: SourceError,
    };

    impl Diagnostic for TestError {
        fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("code"))
        }

        fn severity(&self) -> Option<Severity> {
            Some(Severity::Error)
        }

        fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("help"))
        }

        fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSourceSpan> + '_>> {
            Some(Box::new(vec![LabeledSourceSpan::new(None, 0, 1)].into_iter()))
        }

        fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("url"))
        }

        fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
            Some(&self.source)
        }
    }

    struct Tester;

    impl fmt::Debug for Tester {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let error = Report::new(TestError { source: SourceError });
            DebugReportHandler::new().render_report(fmt, error.as_ref())
        }
    }

    let tester = Tester;

    let expected = r#"Diagnostic { message: "TestError", code: "code", severity: "Error", url: "url", help: "help", labels: "[LabeledSourceSpan { label: None, span: SourceSpan { offset: SourceOffset(0), length: 1 } }]", caused by: "SourceError" }
"#;
    assert_eq!(format!("{tester:?}"), expected);
}
