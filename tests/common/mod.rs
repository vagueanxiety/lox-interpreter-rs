use lox_interpreter_rs::Interpreter;
use pretty_assertions::assert_eq;
use std::fs;

pub fn run_test(test_name: &str, check_output: bool, expect_error: bool) {
    let file_name = format!("tests/{test_name}.lox");
    let input = fs::read_to_string(format!("{file_name}")).expect("Failed to read input file");

    let mut output = Vec::new();
    let mut error_output = Vec::new();

    let mut it = Interpreter::new();
    it.run(input.to_string(), &mut output, &mut error_output, false)
        .expect("Interpreter Error");
    let output = String::from_utf8(output).expect("Not UTF-8");
    let error_output = String::from_utf8(error_output).expect("Not UTF-8");

    if expect_error {
        let expected_error =
            fs::read_to_string(format!("{file_name}.e")).expect("Failed to read error file");
        assert_eq!(error_output, expected_error);
    } else {
        assert_eq!(error_output, "");
    }

    if check_output {
        let expected_output =
            fs::read_to_string(format!("{file_name}.o")).expect("Failed to read output file");
        assert_eq!(output, expected_output);
    }
}
