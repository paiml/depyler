use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("marco-polo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Marco Polo CLI"))
        .stdout(predicate::str::contains("canonical Depyler example"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("marco-polo").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("marco-polo"));
}

#[test]
fn test_invalid_difficulty() {
    let mut cmd = Command::cargo_bin("marco-polo").unwrap();
    cmd.arg("--difficulty")
        .arg("impossible")
        .assert()
        .failure()
        .stderr(predicate::str::contains("possible values"));
}