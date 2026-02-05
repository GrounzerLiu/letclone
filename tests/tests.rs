#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    // Passing tests
    t.pass("tests/ui/pass_*.rs");
    // Failing tests
    t.compile_fail("tests/ui/fail_*.rs");
}
