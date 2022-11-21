use common::run_test;
use test_case::test_case;

mod common;

#[test_case("invalid_super_in_non_subclass", false, true; "Invalid super in non-subclass")]
#[test_case("invalid_super_in_nonclass", false, true; "Invalid super in non-class")]
#[test_case("superclass_semantics", true, false; "Superclass semantics")]
#[test_case("invoke_supermethod", true, false; "Invoker supermethods")]
#[test_case("simple_superclass", true, false; "Simple superclass")]
#[test_case("inherit_from_nonclass", false, true; "inherit from non-class")]
#[test_case("inherit_from_self", false, true; "Inherit from self")]
fn ch13_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
