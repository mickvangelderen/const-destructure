#[test]
fn compilation_failure_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("compile_fail_tests/*.rs");
}
