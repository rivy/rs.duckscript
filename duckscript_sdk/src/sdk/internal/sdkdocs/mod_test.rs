use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("internal")], "out = internal::sdkdocs", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("internal")],
        "out = internal::sdkdocs ./target/temp.md",
        CommandValidation::Match("out".to_string(), "./target/temp.md".to_string()),
    );
}
