// DEPYLER-0363: ArgParse transpilation tests
// Status: RED phase - These tests MUST fail initially
//
// Tests verify that Python argparse code transpiles to compiling Rust clap code.

use depyler_core::DepylerPipeline;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

fn transpile(python_code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    match pipeline.transpile(python_code) {
        Ok(rust_code) => rust_code,
        Err(e) => panic!("Transpilation failed: {:?}", e),
    }
}

fn compile_rust_as_bin(rust_code: &str) -> Result<(), String> {
    let mut temp_file = NamedTempFile::new().map_err(|e| e.to_string())?;
    temp_file.write_all(rust_code.as_bytes()).map_err(|e| e.to_string())?;

    let output = Command::new("rustc")
        .arg("--crate-type")
        .arg("bin")
        .arg("--edition")
        .arg("2021")
        .arg(temp_file.path())
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Compilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_argparse_basic_import() {
    let python = r#"
import argparse

def main():
    return 0
"#;

    let rust = transpile(python);

    eprintln!("Generated Rust code:\n{}\n", rust);

    // Must NOT contain Python artifacts
    assert!(!rust.contains("argparse"), "Should not contain 'argparse' in Rust output");
    assert!(!rust.contains("TODO"), "Should not contain TODO comments about argparse");

    // If argparse is used, should contain clap
    // (This test just imports, so may not need clap yet)
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_argparse_argument_parser() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser(description="Test program")
    args = parser.parse_args()
    return 0
"#;

    let rust = transpile(python);

    // Should contain clap usage
    assert!(rust.contains("clap"), "Should use clap crate");
    assert!(rust.contains("Parser") || rust.contains("#[derive"), "Should use clap Parser derive");

    // Must NOT contain Python artifacts
    assert!(!rust.contains("argparse"), "Should not contain 'argparse'");
    assert!(!rust.contains("ArgumentParser"), "Should not contain 'ArgumentParser'");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_argparse_positional_argument() {
    let python = r#"
import argparse
from pathlib import Path

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+", type=Path, help="Files to process")
    args = parser.parse_args()
    return 0
"#;

    let rust = transpile(python);

    // Should contain clap struct with files field
    assert!(rust.contains("files"), "Should have files field");
    assert!(rust.contains("Vec<") || rust.contains("vec!"), "Should use Vec for nargs='+'");
    assert!(rust.contains("PathBuf") || rust.contains("Path"), "Should use Path types");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_argparse_optional_flags() {
    let python = r#"
import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output")
    parser.add_argument("-c", "--count", type=int, default=1, help="Count value")
    args = parser.parse_args()
    return 0
"#;

    let rust = transpile(python);

    // Should contain clap arguments
    assert!(rust.contains("verbose"), "Should have verbose field");
    assert!(rust.contains("count"), "Should have count field");
    assert!(rust.contains("bool") || rust.contains("true"), "store_true should map to bool");

    // Should have clap attributes
    assert!(rust.contains("#[arg") || rust.contains("#[clap"), "Should use clap attributes");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_path_read_text_method() {
    let python = r#"
from pathlib import Path

def read_file(filepath: Path) -> str:
    content = filepath.read_text()
    return content
"#;

    let rust = transpile(python);

    // Should use fs::read_to_string
    assert!(rust.contains("fs::read_to_string") || rust.contains("read_to_string"),
            "Should use fs::read_to_string instead of read_text");

    // Should import Path correctly
    assert!(rust.contains("use std::path::Path") || rust.contains("std::path::Path"),
            "Should import Path");

    // Must NOT have Python method
    assert!(!rust.contains("read_text"), "Should not contain Python read_text method");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_string_splitlines_method() {
    let python = r#"
def count_lines(text: str) -> int:
    lines = text.splitlines()
    return len(lines)
"#;

    let rust = transpile(python);

    // Should use .lines()
    assert!(rust.contains(".lines()"), "Should use .lines() method");

    // Must NOT have Python method
    assert!(!rust.contains("splitlines"), "Should not contain Python splitlines method");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_string_split_whitespace() {
    let python = r#"
def count_words(text: str) -> int:
    words = text.split()
    return len(words)
"#;

    let rust = transpile(python);

    // Should use .split_whitespace()
    assert!(rust.contains("split_whitespace"), "Should use split_whitespace()");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_try_except_with_ioerror() {
    let python = r#"
from pathlib import Path

def safe_read(filepath: Path) -> str:
    try:
        content = filepath.read_text()
        return content
    except IOError as e:
        print(f"Error: {e}")
        return ""
"#;

    let rust = transpile(python);

    // Should use match on Result
    assert!(rust.contains("match"), "Should use match for error handling");
    assert!(rust.contains("Ok(") && rust.contains("Err("),
            "Should have Ok and Err branches");

    // Should NOT have orphaned statements
    let brace_count = rust.chars().filter(|&c| c == '{').count() as i32 -
                      rust.chars().filter(|&c| c == '}').count() as i32;
    assert_eq!(brace_count, 0, "Braces should be balanced");

    // Must compile
    compile_rust_as_bin(&rust).expect("Generated Rust code should compile");
}

#[test]
#[ignore] // RED phase - expected to fail
fn test_depyler_0363_wordcount_full_integration() {
    let python = r#"
#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path
from typing import NamedTuple

class Stats(NamedTuple):
    lines: int
    words: int
    chars: int
    filename: str

def count_file(filepath: Path) -> Stats:
    try:
        content = filepath.read_text()
        lines = len(content.splitlines())
        words = len(content.split())
        chars = len(content)
        return Stats(lines, words, chars, str(filepath))
    except IOError as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return Stats(0, 0, 0, str(filepath))

def main() -> int:
    parser = argparse.ArgumentParser(description="Count words")
    parser.add_argument("files", nargs="+", type=Path)
    parser.add_argument("-l", "--lines", action="store_true")
    args = parser.parse_args()

    for filepath in args.files:
        stats = count_file(filepath)
        if args.lines:
            print(f"{stats.lines} {stats.filename}")

    return 0

if __name__ == "__main__":
    sys.exit(main())
"#;

    let rust = transpile(python);

    // High-level checks
    assert!(!rust.contains("TODO"), "Should not have TODO comments");
    assert!(!rust.contains("argparse"), "Should not contain Python argparse");
    assert!(rust.contains("clap"), "Should use clap");

    // Must compile
    compile_rust_as_bin(&rust).expect("Full wordcount should compile");
}

// Property test: any valid argparse code should transpile to compiling Rust
#[test]
#[ignore] // RED phase
fn test_depyler_0363_property_argparse_always_compiles() {
    // Simple smoke test - full property test would use proptest crate
    let test_cases = vec![
        r#"
import argparse
parser = argparse.ArgumentParser()
        "#,
        r#"
import argparse
parser = argparse.ArgumentParser(description="test")
parser.add_argument("input")
        "#,
        r#"
import argparse
parser = argparse.ArgumentParser()
parser.add_argument("-v", "--verbose", action="store_true")
        "#,
    ];

    for (i, python) in test_cases.iter().enumerate() {
        let rust = transpile(python);
        compile_rust_as_bin(&rust)
            .unwrap_or_else(|e| panic!("Test case {} failed to compile: {}", i, e));
    }
}
