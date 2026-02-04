//! Coverage tests for rust_gen/argparse_transform.rs
//!
//! DEPYLER-99MODE-001: Targets argparse_transform.rs (3,206 lines)
//! Covers: ArgumentParser creation, positional/optional args, actions,
//! nargs, defaults, choices, subcommands, argument groups.

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
// Basic ArgumentParser creation
// ============================================================================

#[test]
fn test_argparse_basic_parser() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A test CLI")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_parser_with_epilog() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(
        description="Test CLI",
        epilog="See docs for details"
    )
    args = parser.parse_args()
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
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("filename", help="Input file")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional_with_type() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("count", type=int, help="Number of items")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional_nargs_plus() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("files", nargs="+", help="Input files")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_positional_nargs_star() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("extras", nargs="*", help="Extra args")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Optional flags
// ============================================================================

#[test]
fn test_argparse_store_true() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_store_false() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--no-color", action="store_false", dest="color")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_optional_with_default() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("-o", "--output", default="out.txt", help="Output file")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_count_action() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("-d", "--debug", action="count", help="Debug level")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_append_action() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("-D", action="append", dest="defines", help="Define vars")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_choices() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--format", choices=["json", "csv", "xml"], help="Format")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_required_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--config", required=True, help="Config file")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_nargs_optional() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--config", nargs="?", help="Optional config")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Subcommands
// ============================================================================

#[test]
fn test_argparse_subcommands() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Git-like CLI")
    subparsers = parser.add_subparsers(dest="command", required=True)

    clone_parser = subparsers.add_parser("clone", help="Clone repo")
    clone_parser.add_argument("url", help="Repository URL")

    push_parser = subparsers.add_parser("push", help="Push changes")
    push_parser.add_argument("--force", action="store_true")

    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_subcommand_with_depth() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    subparsers = parser.add_subparsers(dest="command")

    build_parser = subparsers.add_parser("build", help="Build project")
    build_parser.add_argument("--release", action="store_true")
    build_parser.add_argument("--target", type=str, default="debug")

    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple arguments combined
// ============================================================================

#[test]
fn test_argparse_multiple_args() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Multi-arg CLI")
    parser.add_argument("input", help="Input file")
    parser.add_argument("-o", "--output", help="Output file")
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-n", "--count", type=int, default=1)
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_with_field_access() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("name", help="Name")
    parser.add_argument("-g", "--greeting", default="Hello")
    args = parser.parse_args()
    message = args.greeting + " " + args.name
    print(message)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_int_type() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_long_only_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_argparse_short_only_flag() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="CLI")
    parser.add_argument("-q", action="store_true")
    args = parser.parse_args()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex integration
// ============================================================================

#[test]
fn test_argparse_comprehensive_cli() {
    let code = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(
        description="Comprehensive CLI tool",
        epilog="Run with --help for more"
    )
    parser.add_argument("input", help="Input file path")
    parser.add_argument("-o", "--output", default="output.txt")
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-n", "--count", type=int, default=10)
    parser.add_argument("--format", choices=["json", "csv"])
    parser.add_argument("-d", "--debug", action="count")

    args = parser.parse_args()
    if args.verbose:
        print("Verbose mode enabled")
"#;
    assert!(transpile_ok(code));
}
