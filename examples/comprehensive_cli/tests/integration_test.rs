use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_comprehensive_cli_basic_args() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")  // positional
        .arg("extra1.txt")  // extras (nargs="+")
        .arg("extra2.txt")
        .arg("--api-key").arg("test-key-123")  // required
        .assert()
        .success();
}

#[test]
fn test_missing_required_api_key() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("api-key"));
}

#[test]
fn test_action_store_true() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--debug")  // action="store_true"
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""debug": true"#));
}

#[test]
fn test_action_store_false() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--no-color")  // action="store_false", dest="color"
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""color": false"#));
}

#[test]
fn test_action_count() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("-V").arg("-V").arg("-V")  // action="count"
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""verbosity": 3"#));
}

#[test]
fn test_action_append() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("-I").arg("/usr/include")  // action="append"
        .arg("-I").arg("/opt/include")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""/usr/include""#))
        .stdout(predicate::str::contains(r#""/opt/include""#));
}

#[test]
fn test_nargs_specific_number() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--coords").arg("3.5").arg("7.2")  // nargs=2
        .assert()
        .success();
}

#[test]
fn test_type_int() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--count").arg("42")  // type=int
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""count": 42"#));
}

#[test]
fn test_type_float() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--rate").arg("1.5")  // type=float
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""rate": 1.5"#));
}

#[test]
fn test_default_values() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""timeout": 30"#))  // default=30
        .stdout(predicate::str::contains(r#""threshold": 0.95"#))  // default=0.95
        .stdout(predicate::str::contains(r#""name": "unnamed""#));  // default="unnamed"
}

#[test]
fn test_choices_validation() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    // Valid choice
    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--format").arg("json")  // choices=["json", "yaml", "xml"]
        .assert()
        .success();

    // Invalid choice
    let mut cmd2 = Command::cargo_bin("comprehensive_cli").unwrap();
    cmd2.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--format").arg("csv")
        .assert()
        .failure();
}

#[test]
fn test_const_with_nargs_optional() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    // With flag but no value → uses const="auto"
    cmd.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .arg("--mode")  // nargs="?", const="auto", default="manual"
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""mode": "auto""#));

    // Without flag → uses default="manual"
    let mut cmd2 = Command::cargo_bin("comprehensive_cli").unwrap();
    cmd2.arg("input.txt")
        .arg("extra1.txt")
        .arg("--api-key").arg("key")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""mode": "manual""#));
}

#[test]
fn test_help_generation() {
    let mut cmd = Command::cargo_bin("comprehensive_cli").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Comprehensive CLI application"))
        .stdout(predicate::str::contains("Input file to process"))
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("Example: ./cli input.txt"));
}
