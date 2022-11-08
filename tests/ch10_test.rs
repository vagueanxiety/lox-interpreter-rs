use common::run_test;
use test_case::test_case;

mod common;

#[test_case("native_fun", true, false; "Native function")]
#[test_case("fibonacci", true, false; "Recursive fibonacci")]
#[test_case("early_return", true, false; "Early return statement")]
#[test_case("simple_closure", true, false; "Simple Closure")]
#[test_case("closure", true, false; "Function returned as a closure")]
#[test_case("simple_fun", true, false; "Simple function")]
fn ch10_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
