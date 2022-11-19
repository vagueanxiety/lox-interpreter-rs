use common::run_test;
use test_case::test_case;

mod common;

#[test_case("invalid_return_in_initializer", false, true; "Invalid return in initializers")]
#[test_case("initializer_edge_cases", true, false; "Edge cases of initializers")]
#[test_case("invalid_this", false, true; "Invalid use of this")]
#[test_case("instance_methods", true, false; "Calling methods of an instance")]
#[test_case("instance_fields", true, true; "Getting and setting fields of an instance")]
fn ch12_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
