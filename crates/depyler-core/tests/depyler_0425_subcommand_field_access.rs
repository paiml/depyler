// DEPYLER-0425: Subcommand Field Access Requires Pattern Matching
//
// This test verifies that when handler functions access subcommand-specific fields
// (like args.url in a clone handler), the transpiler generates proper pattern matching
// to extract those fields from the Commands enum variant.
#![allow(non_snake_case)] // Allow DEPYLER-XXXX test naming convention

use depyler_core::DepylerPipeline;

fn transpile_str(python: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).map_err(|e| e.into())
}

#[test]
fn test_DEPYLER_0425_basic_subcommand_field_extraction() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url")

    args = parser.parse_args()
    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    print(f"Clone: {args.url}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_str(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Must contain pattern matching to extract url from Commands::Clone
    // Pattern can be: if let Commands::Clone, or match Commands::Clone { ref url, .. }
    assert!(
        rust_code.contains("if let Commands::Clone") ||
        rust_code.contains("Commands::Clone { url }") ||
        rust_code.contains("Commands::Clone { ref url"),
        "Generated code must pattern match Commands::Clone variant to access url field.\nGenerated:\n{}",
        rust_code
    );

    // Must NOT try to access args.url directly (would fail compilation)
    assert!(
        !rust_code.contains("args.url"),
        "Generated code must NOT access args.url directly (field doesn't exist on Args struct).\nGenerated:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0425"]
fn test_DEPYLER_0425_multiple_subcommand_fields() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url")
    clone_parser.add_argument("--branch", default="main")

    args = parser.parse_args()
    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    print(f"Clone {args.url} (branch: {args.branch})")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_str(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Must extract both url and branch from variant
    assert!(
        rust_code.contains("Commands::Clone")
            && (rust_code.contains("{ url") || rust_code.contains("url,"))
            && rust_code.contains("branch"),
        "Generated code must extract both url and branch from Commands::Clone.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0425"]
fn test_DEPYLER_0425_global_and_subcommand_fields() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url")

    args = parser.parse_args()
    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    if args.verbose:
        print(f"Verbose: Clone {args.url}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_str(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Global field (verbose) should still use args.verbose
    assert!(
        rust_code.contains("args.verbose"),
        "Global fields like verbose should still use args.verbose.\nGenerated:\n{}",
        rust_code
    );

    // Subcommand field (url) should be extracted via pattern matching
    assert!(
        rust_code.contains("Commands::Clone") && rust_code.contains("url"),
        "Subcommand fields must be extracted via pattern matching.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT have args.url
    assert!(
        !rust_code.contains("args.url"),
        "Should not access args.url directly.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_DEPYLER_0425_multiple_subcommands() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone")
    clone_parser.add_argument("url")

    push_parser = subparsers.add_parser("push")
    push_parser.add_argument("remote")

    args = parser.parse_args()
    if args.command == "clone":
        handle_clone(args)
    elif args.command == "push":
        handle_push(args)

def handle_clone(args):
    print(f"Clone: {args.url}")

def handle_push(args):
    print(f"Push to: {args.remote}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_str(python);
    assert!(result.is_ok(), "Transpilation should succeed");

    let rust_code = result.unwrap();

    // Each handler must pattern match its specific variant
    assert!(
        rust_code.contains("Commands::Clone") && rust_code.contains("url"),
        "handle_clone must extract url from Commands::Clone.\nGenerated:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("Commands::Push") && rust_code.contains("remote"),
        "handle_push must extract remote from Commands::Push.\nGenerated:\n{}",
        rust_code
    );

    // Should NOT have direct field access
    assert!(
        !rust_code.contains("args.url") && !rust_code.contains("args.remote"),
        "Should not access subcommand fields directly on args.\nGenerated:\n{}",
        rust_code
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0425"]
fn test_DEPYLER_0425_git_clone_example() {
    // This is the actual git_clone.py example that's failing
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(
        description="Git-like CLI example with subcommands",
        prog="git_clone.py",
    )
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output")
    parser.add_argument("--version", action="version", version="1.0.0")

    subparsers = parser.add_subparsers(dest="command", help="Available commands", required=True)

    parser_clone = subparsers.add_parser("clone", help="Clone a repository")
    parser_clone.add_argument("url", help="Repository URL to clone")

    parser_push = subparsers.add_parser("push", help="Push to a remote repository")
    parser_push.add_argument("remote", help="Remote name to push to")

    parser_pull = subparsers.add_parser("pull", help="Pull from a remote repository")
    parser_pull.add_argument("remote", help="Remote name to pull from")

    args = parser.parse_args()

    if args.command == "clone":
        handle_clone(args)
    elif args.command == "push":
        handle_push(args)
    elif args.command == "pull":
        handle_pull(args)

def handle_clone(args):
    if args.verbose:
        print("Verbose mode: ON")
        print(f"Clone: {args.url}")
        print("This is a demo - no actual cloning performed")
    else:
        print(f"Clone: {args.url}")

def handle_push(args):
    if args.verbose:
        print("Verbose mode: ON")
        print(f"Push to: {args.remote}")
        print("This is a demo - no actual pushing performed")
    else:
        print(f"Push to: {args.remote}")

def handle_pull(args):
    if args.verbose:
        print("Verbose mode: ON")
        print(f"Pull from: {args.remote}")
        print("This is a demo - no actual pulling performed")
    else:
        print(f"Pull from: {args.remote}")

if __name__ == "__main__":
    main()
"#;

    let result = transpile_str(python);
    assert!(result.is_ok(), "git_clone.py transpilation should succeed");

    let rust_code = result.unwrap();

    // All three handlers must use pattern matching
    assert!(
        rust_code.contains("Commands::Clone") && rust_code.contains("{ url }"),
        "handle_clone must extract url via pattern matching.\nGenerated:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("Commands::Push") && rust_code.contains("{ remote }"),
        "handle_push must extract remote via pattern matching.\nGenerated:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("Commands::Pull") && rust_code.contains("{ remote }"),
        "handle_pull must extract remote via pattern matching.\nGenerated:\n{}",
        rust_code
    );

    // Must NOT have direct field access (causes compilation errors)
    assert!(
        !rust_code.contains("args.url"),
        "Must not access args.url directly (E0609 error).\nGenerated:\n{}",
        rust_code
    );

    assert!(
        !rust_code.contains("args.remote"),
        "Must not access args.remote directly (E0609 error).\nGenerated:\n{}",
        rust_code
    );

    // Global fields should still work
    assert!(
        rust_code.contains("args.verbose"),
        "Global field args.verbose should still be accessible.\nGenerated:\n{}",
        rust_code
    );
}
