// DEPYLER-0930: TDD tests for duplicate identifier in match patterns
// Location: crates/depyler-core/tests/depyler_0930_duplicate_match_test.rs
//
// Bug: stmt_gen.rs:407 pushes args without deduplication check
// Symptom: E0416 "identifier bound more than once" in match patterns
// Examples affected: pathlib, difflib, tempfile, zipfile, nested_subs

use depyler_core::DepylerPipeline;

/// Test that subcommand with single positional arg doesn't duplicate in match
#[test]
fn test_depyler_0930_single_arg_no_duplicate() {
    let python = r#"
import argparse

def cmd_info(args):
    print(args.path)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    info = subparsers.add_parser("info")
    info.add_argument("path")
    args = parser.parse_args()
    if args.command == "info":
        cmd_info(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // E0416 occurs when same identifier bound twice: `ref path, ref path`
    // The pattern should only have `ref path` once
    let binding_count = result.matches("ref path").count();
    assert!(
        binding_count <= 1,
        "Expected at most 1 'ref path' binding, found {}. \
         This indicates E0416 duplicate identifier bug.\n\
         Generated code snippet:\n{}",
        binding_count,
        &result[result.find("match").unwrap_or(0)..].chars().take(500).collect::<String>()
    );
}

/// Test that subcommand with multiple args doesn't duplicate any
#[test]
fn test_depyler_0930_multiple_args_no_duplicates() {
    let python = r#"
import argparse

def cmd_glob(args):
    print(args.directory)
    print(args.pattern)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    glob = subparsers.add_parser("glob")
    glob.add_argument("directory")
    glob.add_argument("pattern")
    args = parser.parse_args()
    if args.command == "glob":
        cmd_glob(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // Check for duplicate bindings
    let dir_count = result.matches("ref directory").count();
    let pat_count = result.matches("ref pattern").count();

    assert!(
        dir_count <= 1,
        "Expected at most 1 'ref directory' binding, found {}",
        dir_count
    );
    assert!(
        pat_count <= 1,
        "Expected at most 1 'ref pattern' binding, found {}",
        pat_count
    );
}

/// Test pathlib-style multiple subcommands don't have duplicate bindings
#[test]
fn test_depyler_0930_multiple_subcommands_no_duplicates() {
    let python = r#"
import argparse

def cmd_info(args):
    print(args.path)

def cmd_glob(args):
    print(args.directory)
    print(args.pattern)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")

    info = subparsers.add_parser("info")
    info.add_argument("path")

    glob = subparsers.add_parser("glob")
    glob.add_argument("directory")
    glob.add_argument("pattern")

    args = parser.parse_args()
    if args.command == "info":
        cmd_info(args)
    elif args.command == "glob":
        cmd_glob(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // No field should appear more than once in any single match pattern
    // Count total occurrences - each should be at most 1 per subcommand
    let path_count = result.matches("ref path").count();
    let dir_count = result.matches("ref directory").count();
    let pat_count = result.matches("ref pattern").count();

    // With 2 subcommands, path appears in 1, directory+pattern in 1
    // So max occurrences should be 1 each
    assert!(
        path_count <= 1,
        "Expected at most 1 'ref path', found {}",
        path_count
    );
    assert!(
        dir_count <= 1,
        "Expected at most 1 'ref directory', found {}",
        dir_count
    );
    assert!(
        pat_count <= 1,
        "Expected at most 1 'ref pattern', found {}",
        pat_count
    );
}

/// Test that generated code compiles without E0416
#[test]
fn test_depyler_0930_compiles_without_e0416() {
    let python = r#"
import argparse

def cmd_info(args):
    print(args.path)

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    info = subparsers.add_parser("info")
    info.add_argument("path")
    args = parser.parse_args()
    if args.command == "info":
        cmd_info(args)

if __name__ == "__main__":
    main()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python).expect("transpilation should succeed");

    // The generated code should NOT contain duplicate bindings like:
    // `Commands::Info { ref path, ref path, .. }`
    // It SHOULD contain single bindings:
    // `Commands::Info { ref path, .. }`

    // Check we don't have consecutive duplicate refs
    assert!(
        !result.contains("ref path, ref path"),
        "Found duplicate 'ref path, ref path' binding - E0416 bug present"
    );
    assert!(
        !result.contains("ref path,\n            ref path"),
        "Found multiline duplicate 'ref path' binding - E0416 bug present"
    );
}
