// DEPYLER-0931: TDD tests for With statement context field extraction
// Location: crates/depyler-core/tests/depyler_0931_with_context_test.rs
//
// Bug: extract_fields_recursive doesn't traverse HirStmt::With.context
// Symptom: E0425 "cannot find value" when args.field used in `with open(args.file):`
// Root cause: stmt_gen.rs:6173 only traverses body, not context

use depyler_core::DepylerPipeline;

/// Test that args.field in `with open(args.file)` is detected and bound
#[test]
fn test_depyler_0931_with_open_args_field() {
    let python = r#"
import argparse

def cmd_read(args):
    with open(args.file) as f:
        content = f.read()
        print(content)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    read = subparsers.add_parser("read")
    read.add_argument("file")
    args = parser.parse_args()
    if args.command == "read":
        cmd_read(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // The match pattern should bind `file` since it's used in the body
    // Check that we have `ref file` binding
    assert!(
        result.contains("ref file"),
        "Expected match pattern to bind 'file' field. \
         The 'with open(args.file)' expression should trigger field extraction.\n\
         Generated code:\n{}",
        &result[result.find("match").unwrap_or(0)..].chars().take(500).collect::<String>()
    );
}

/// Test multiple args.field in with statement context managers
#[test]
fn test_depyler_0931_with_multiple_args_fields() {
    let python = r#"
import argparse
import json

def cmd_validate(args):
    with open(args.config) as f:
        config = json.load(f)
    with open(args.schema) as f:
        schema = json.load(f)
    print("valid")

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    validate = subparsers.add_parser("validate")
    validate.add_argument("config")
    validate.add_argument("--schema", required=True)
    args = parser.parse_args()
    if args.command == "validate":
        cmd_validate(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // Both config and schema should be bound
    assert!(
        result.contains("ref config"),
        "Expected 'config' field to be bound from with statement"
    );
    assert!(
        result.contains("ref schema"),
        "Expected 'schema' field to be bound from with statement"
    );
}

/// Test that with statement in direct match body extracts fields
#[test]
fn test_depyler_0931_with_in_match_body() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    read = subparsers.add_parser("read")
    read.add_argument("filename")
    args = parser.parse_args()
    if args.command == "read":
        with open(args.filename) as f:
            print(f.read())

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // filename should be bound in the match pattern
    assert!(
        result.contains("ref filename"),
        "Expected 'filename' field to be bound in match pattern"
    );
}

/// Test nested with statements
#[test]
fn test_depyler_0931_nested_with_statements() {
    let python = r#"
import argparse

def cmd_copy(args):
    with open(args.source) as src:
        with open(args.dest, 'w') as dst:
            dst.write(src.read())

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    copy = subparsers.add_parser("copy")
    copy.add_argument("source")
    copy.add_argument("dest")
    args = parser.parse_args()
    if args.command == "copy":
        cmd_copy(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // Both source and dest should be bound
    assert!(
        result.contains("ref source"),
        "Expected 'source' field to be bound from nested with"
    );
    assert!(
        result.contains("ref dest"),
        "Expected 'dest' field to be bound from nested with"
    );
}
