use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn it_fails_if_no_args_provided() {
    let mut cmd = Command::cargo_bin("echo-rs").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn it_runs_with_one_argument() -> TestResult {
    run(&["hello"], "hello\n")
}

#[test]
fn it_omits_newline() -> TestResult {
    run(&["-n", "hello"], "hello")
}

#[test]
fn it_runs_with_multiple_arguments() -> TestResult {
    run(&["hello", "world"], "hello world\n")
}

#[test]
fn it_omits_newline_with_multiple_arguments() -> TestResult {
    run(&["-n", "hello", "world"], "hello world")
}

fn run(args: &[&str], expected_output: &'static str) -> TestResult {
    let mut cmd = Command::cargo_bin("echo-rs")?;
    cmd.args(args).assert().success().stdout(expected_output);
    Ok(())
}