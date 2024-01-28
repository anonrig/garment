use std::fmt::Display;

use garment::{
    Diagnostic, GraphicalReportHandler, LabeledSourceSpan, NamedSource, Report, Severity,
    SourceSpan,
};

fn format_report<R: AsRef<dyn Diagnostic>>(report: R) -> String {
    let mut out = String::new();
    GraphicalReportHandler::new().render_report(&mut out, report.as_ref()).unwrap();
    out
}

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
    assert_eq!(&out, "severity:Error");
}

#[test]
#[ignore]
fn external_source() {
    use thiserror::Error;
    #[derive(Error, Debug)]
    #[error("oops!")]
    struct MyBad {
        highlight: SourceSpan,
    }

    impl Diagnostic for MyBad {
        fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
            Some(Box::new("try doing it better next time?"))
        }

        fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSourceSpan> + '_>> {
            Some(Box::new(std::iter::once(LabeledSourceSpan::new_with_span(
                Some("this bit here".into()),
                self.highlight.clone(),
            ))))
        }
    }

    let src = "source\n  text\n    here".to_string();
    let err = Report::new(MyBad { highlight: SourceSpan::from((9, 4)) })
        .with_source_code(NamedSource::new("bad_file.rs", src));
    let out = format_report(err);
    println!("Error: {out}");
    let expected = r"oops::my::bad

  × oops!
   ╭─[bad_file.rs:2:3]
 1 │ source
 2 │   text
   ·   ──┬─
   ·     ╰── this bit here
 3 │     here
   ╰────
  help: try doing it better next time?
"
    .trim_start()
    .to_string();
    assert_eq!(expected, out);
}
