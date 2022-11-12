use common::run_test;
use test_case::test_case;

mod common;

#[test_case("leaky_scope", true, false; "Potentially leaky scope")]
fn ch11_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
