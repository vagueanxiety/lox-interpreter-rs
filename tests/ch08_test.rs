use common::run_test;
use test_case::test_case;

mod common;

#[test_case("nested_writes_to_outer_vars", true, false; "Nested writes to outer variables")]
#[test_case("restoring_scope", true, false; "Restoring scope once a block ends")]
#[test_case("new_var_reads_outer_shadow", false, true; "New variable reads from outer shadow")]
fn ch08_test(test_name: &str, check_output: bool, check_error: bool) {
    run_test(test_name, check_output, check_error);
}
