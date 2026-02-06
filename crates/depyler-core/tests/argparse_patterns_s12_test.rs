//! Session 12 Batch 96: Argparse and CLI patterns
//!
//! Targets argparse_transform.rs cold paths (77.64% line coverage).
//! Tests Python argparse patterns to exercise CLI codegen.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) = AstBridge::new()
        .with_source(python_code.to_string())
        .python_to_hir(ast)
        .expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

#[test]
fn test_s12_b96_simple_argparse() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser(description="A tool")
    parser.add_argument("input", help="Input file")
    parser.add_argument("--output", default="out.txt", help="Output file")
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main") || result.contains("argparse") || result.contains("clap"), "Got: {}", result);
}

#[test]
fn test_s12_b96_argparse_bool_flag() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--quiet", action="store_false")
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b96_argparse_int_arg() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--count", type=int, default=10)
    parser.add_argument("--threshold", type=float, default=0.5)
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b96_argparse_required() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True, help="Config file path")
    parser.add_argument("--mode", choices=["train", "test"], default="train")
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b96_argparse_nargs() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+", help="Input files")
    parser.add_argument("--tags", nargs="*", default=[])
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b96_argparse_subparsers() {
    let code = r##"
import argparse

def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command")
    build_parser = subparsers.add_parser("build")
    build_parser.add_argument("--release", action="store_true")
    test_parser = subparsers.add_parser("test")
    test_parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn main"), "Got: {}", result);
}

#[test]
fn test_s12_b96_sys_argv() {
    let code = r#"
import sys

def get_args() -> list:
    return sys.argv[1:]

def get_program_name() -> str:
    return sys.argv[0]
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_args"), "Got: {}", result);
}

#[test]
fn test_s12_b96_manual_arg_parse() {
    let code = r##"
import sys

def parse_args() -> dict:
    args = {}
    i = 1
    while i < len(sys.argv):
        arg = sys.argv[i]
        if arg.startswith("--"):
            key = arg[2:]
            if i + 1 < len(sys.argv) and not sys.argv[i + 1].startswith("--"):
                args[key] = sys.argv[i + 1]
                i += 2
            else:
                args[key] = "true"
                i += 1
        else:
            i += 1
    return args
"##;
    let result = transpile(code);
    assert!(result.contains("fn parse_args"), "Got: {}", result);
}
