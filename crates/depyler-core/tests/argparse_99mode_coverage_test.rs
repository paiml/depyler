//! Coverage tests for argparse_transform.rs
//!
//! DEPYLER-99MODE-001: Targets argparse_transform.rs uncovered paths
//! Covers: subcommand patterns, field access analysis, enum generation,
//! struct generation, type inference edge cases, keyword escaping.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Basic argparse patterns
// ============================================================================

#[test]
fn test_argparse_basic_string_arg() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A tool")
    parser.add_argument("--name", type=str, default="World")
    args = parser.parse_args()
    print("Hello, " + args.name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_int_arg() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int, default=1)
    args = parser.parse_args()
    for i in range(args.count):
        print(i)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_float_arg() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rate", type=float, default=0.1)
    args = parser.parse_args()
    print(args.rate)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_bool_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    if args.verbose:
        print("Verbose mode")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_store_false() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--no-cache", action="store_false", dest="cache")
    args = parser.parse_args()
    if args.cache:
        print("Using cache")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Positional arguments
// ============================================================================

#[test]
fn test_argparse_positional_string() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("filename")
    args = parser.parse_args()
    print(args.filename)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional_with_type() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("port", type=int)
    args = parser.parse_args()
    print(args.port)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional_and_optional() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file")
    parser.add_argument("--output", type=str, default="out.txt")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    print(args.input_file)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple arguments
// ============================================================================

#[test]
fn test_argparse_many_arguments() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", type=str, default="localhost")
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--workers", type=int, default=4)
    parser.add_argument("--timeout", type=float, default=30.0)
    parser.add_argument("--debug", action="store_true")
    args = parser.parse_args()
    print(args.host)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_short_and_long() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", "--name", type=str, default="World")
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-c", "--count", type=int, default=1)
    args = parser.parse_args()
    print(args.name)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Subcommand patterns
// ============================================================================

#[test]
fn test_argparse_subcommand_basic() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")

    hello_parser = subparsers.add_parser("hello")
    hello_parser.add_argument("--name", type=str, default="World")

    args = parser.parse_args()
    if args.command == "hello":
        print("Hello, " + args.name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_two_subcommands() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")

    add_parser = subparsers.add_parser("add")
    add_parser.add_argument("--value", type=int, default=0)

    remove_parser = subparsers.add_parser("remove")
    remove_parser.add_argument("--key", type=str)

    args = parser.parse_args()
    if args.command == "add":
        print(args.value)
    elif args.command == "remove":
        print(args.key)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_subcommand_with_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    subparsers = parser.add_subparsers(dest="command")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("--target", type=str, default="all")

    args = parser.parse_args()
    if args.verbose:
        print("Verbose mode")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Help and description
// ============================================================================

#[test]
fn test_argparse_with_description() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A tool for processing data")
    parser.add_argument("--input", type=str, help="Input file path")
    parser.add_argument("--output", type=str, help="Output file path")
    args = parser.parse_args()
    print(args.input)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_arg_with_help() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, help="Your name", default="User")
    args = parser.parse_args()
    print(args.name)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Nargs patterns
// ============================================================================

#[test]
fn test_argparse_nargs_star() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="*")
    args = parser.parse_args()
    for f in args.files:
        print(f)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_nargs_plus() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()
    print(len(args.files))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_nargs_question() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", nargs="?", default="config.toml")
    args = parser.parse_args()
    print(args.config)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Choices
// ============================================================================

#[test]
fn test_argparse_with_choices() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["json", "csv", "xml"], default="json")
    args = parser.parse_args()
    print(args.format)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Required arguments
// ============================================================================

#[test]
fn test_argparse_required_optional() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True)
    args = parser.parse_args()
    print(args.config)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex field access patterns
// ============================================================================

#[test]
fn test_argparse_field_in_if() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--level", type=int, default=0)
    parser.add_argument("--name", type=str, default="test")
    args = parser.parse_args()
    if args.level > 0:
        print(args.name)
    else:
        print("default")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_field_in_loop() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int, default=3)
    parser.add_argument("--message", type=str, default="hello")
    args = parser.parse_args()
    for i in range(args.count):
        print(args.message)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_field_in_function_call() {
    let code = r#"
import argparse

def process(name: str, count: int):
    for i in range(count):
        print(name)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, default="world")
    parser.add_argument("--count", type=int, default=1)
    args = parser.parse_args()
    process(args.name, args.count)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Append action
// ============================================================================

#[test]
fn test_argparse_append_action() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--include", action="append")
    args = parser.parse_args()
    if args.include:
        for item in args.include:
            print(item)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dest parameter
// ============================================================================

#[test]
fn test_argparse_dest() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-file", dest="output_file", type=str)
    args = parser.parse_args()
    if args.output_file:
        print(args.output_file)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple subcommands with shared fields
// ============================================================================

#[test]
fn test_argparse_three_subcommands() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")

    init_parser = subparsers.add_parser("init")
    init_parser.add_argument("--template", type=str, default="default")

    build_parser = subparsers.add_parser("build")
    build_parser.add_argument("--release", action="store_true")

    test_parser = subparsers.add_parser("test")
    test_parser.add_argument("--pattern", type=str, default="*")

    args = parser.parse_args()
    if args.command == "init":
        print(args.template)
    elif args.command == "build":
        if args.release:
            print("release build")
    elif args.command == "test":
        print(args.pattern)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Real-world-like patterns
// ============================================================================

#[test]
fn test_argparse_wordcount_cli() {
    let code = r#"
import argparse

def count_words(text: str) -> int:
    return len(text.split())

def main():
    parser = argparse.ArgumentParser(description="Word counter")
    parser.add_argument("text", type=str, help="Text to count")
    parser.add_argument("--unique", action="store_true", help="Count unique words")
    args = parser.parse_args()
    if args.unique:
        words = set(args.text.split())
        print(len(words))
    else:
        print(count_words(args.text))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_greeting_cli() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("name", type=str)
    parser.add_argument("--greeting", type=str, default="Hello")
    parser.add_argument("--shout", action="store_true")
    args = parser.parse_args()
    message = args.greeting + ", " + args.name + "!"
    if args.shout:
        message = message.upper()
    print(message)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_calculator_cli() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("a", type=float)
    parser.add_argument("b", type=float)
    parser.add_argument("--op", type=str, default="add", choices=["add", "sub", "mul", "div"])
    args = parser.parse_args()
    if args.op == "add":
        print(args.a + args.b)
    elif args.op == "sub":
        print(args.a - args.b)
    elif args.op == "mul":
        print(args.a * args.b)
    elif args.op == "div":
        if args.b != 0:
            print(args.a / args.b)
"#;
    assert!(transpile_ok(code));
}
