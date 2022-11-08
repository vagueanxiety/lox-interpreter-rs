use common::run_test;
use test_case::test_case;

mod common;

#[test_case("simple_fun", true, false; "Simple function")]
fn ch10_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
