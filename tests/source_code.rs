use std::sync::Arc;

use garment::SourceCode;

#[test]
fn compile_test() {
    let soure_code = "";
    soure_code.read_span();

    let soure_code = String::new();
    soure_code.read_span();

    let soure_code = Arc::new("");
    soure_code.read_span();

    let soure_code = Arc::new(String::new());
    soure_code.read_span();
}
