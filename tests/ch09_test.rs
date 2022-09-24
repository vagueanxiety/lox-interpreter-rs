use common::run_test;
use test_case::test_case;

mod common;

#[test_case("logical_op", true, false; "Logical operators")]
#[test_case("if_else", true, false; "Simple if-else branching")]
fn ch09_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
