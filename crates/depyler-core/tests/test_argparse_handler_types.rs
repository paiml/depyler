//! DEPYLER-0424: Test argparse handler function parameter types
//!
//! Verifies that functions receiving parse_args() result are typed as `&Args`
//! instead of `&serde_json::Value`.

use depyler_core::DepylerPipeline;

fn transpile_str(python: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(python).map_err(|e| e.into())
}

#[test]
fn test_DEPYLER_0424_handler_function_simple() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Test CLI")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    handle_command(args)

def handle_command(args):
    if args.verbose:
        print("Verbose mode")
"#;

    let rust = transpile_str(python).unwrap();

    // Args struct should be at module level (not inside main)
    assert!(
        rust.contains("struct Args"),
        "Args struct not found in output"
    );

    // Handler should use &Args, not &serde_json::Value
    assert!(
        rust.contains("fn handle_command(args: &Args)"),
        "Handler function should have parameter type &Args, got:\n{}",
        rust
    );
    assert!(
        !rust.contains("serde_json::Value"),
        "Should not use serde_json::Value for args parameter"
    );

    // Call site should pass &args
    assert!(
        rust.contains("handle_command(&args)"),
        "Call site should pass &args by reference"
    );
}

#[test]
fn test_DEPYLER_0424_subcommand_handlers() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")

    subparsers = parser.add_subparsers(dest="command", required=True)

    parser_clone = subparsers.add_parser("clone")
    parser_clone.add_argument("url")

    args = parser.parse_args()

    if args.command == "clone":
        handle_clone(args)

def handle_clone(args):
    print(args.url)
"#;

    let rust = transpile_str(python).unwrap();

    // Handler should use &Args
    assert!(
        rust.contains("fn handle_clone(args: &Args)"),
        "Subcommand handler should have parameter type &Args, got:\n{}",
        rust
    );

    // Should not use serde_json::Value
    assert!(
        !rust.contains("fn handle_clone(args: &serde_json::Value)"),
        "Should not use serde_json::Value"
    );

    // Call site should pass &args
    assert!(
        rust.contains("handle_clone(&args)") || rust.contains("handle_clone(& args)"),
        "Call site should pass &args by reference"
    );
}

#[test]
fn test_DEPYLER_0424_multiple_handlers() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--debug", action="store_true")
    args = parser.parse_args()

    handler_one(args)
    handler_two(args)

def handler_one(args):
    if args.debug:
        print("Handler one")

def handler_two(args):
    if args.debug:
        print("Handler two")
"#;

    let rust = transpile_str(python).unwrap();

    // Both handlers should use &Args
    assert!(
        rust.contains("fn handler_one(args: &Args)"),
        "handler_one should have parameter type &Args"
    );
    assert!(
        rust.contains("fn handler_two(args: &Args)"),
        "handler_two should have parameter type &Args"
    );

    // No serde_json::Value usage
    assert!(
        !rust.contains("serde_json::Value"),
        "Should not use serde_json::Value"
    );
}

#[test]
fn test_DEPYLER_0424_different_var_name() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--flag")
    parsed_args = parser.parse_args()  # Different var name
    process(parsed_args)

def process(parsed_args):
    print(parsed_args.flag)
"#;

    let rust = transpile_str(python).unwrap();

    // Handler should use &Args even with different variable name
    assert!(
        rust.contains("fn process(parsed_args: &Args)"),
        "Handler should have parameter type &Args regardless of variable name, got:\n{}",
        rust
    );
}
