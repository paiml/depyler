//! TDD Tests for Multiple Context Managers (DEPYLER-0600)
//!
//! Bug: `with open(...) as f1, open(...) as f2:` fails with
//!      "Multiple context managers not yet supported"
//!
//! Fix: Recursively nest context managers:
//!   `with A as a, B as b: body` becomes:
//!   `with A as a: with B as b: body`
//!
//! Test Coverage:
//!   1. Two file context managers (read + write)
//!   2. Three context managers
//!   3. Mixed context manager types
//!   4. Context managers without targets
//!   5. Nested body statements

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python: &str) -> Result<String, String> {
    let ast = parse(python, Mode::Module, "<test>").map_err(|e| e.to_string())?;
    let (hir, _) = AstBridge::new().python_to_hir(ast).map_err(|e| e.to_string())?;
    let type_mapper = TypeMapper::default();
    let (rust_code, _deps) = generate_rust_file(&hir, &type_mapper).map_err(|e| e.to_string())?;
    Ok(rust_code)
}

fn assert_compiles(rust_code: &str, test_name: &str) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join(format!("{}.rs", test_name));
    std::fs::write(&file_path, rust_code).expect("Failed to write file");

    let output = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021", "-o"])
        .arg(temp_dir.path().join("libtest.rlib"))
        .arg(&file_path)
        .output()
        .expect("Failed to run rustc");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "Rust compilation failed for {}:\n{}\n\nGenerated code:\n{}",
            test_name, stderr, rust_code
        );
    }
}

/// Test 1: Two file context managers (the verificar failure case)
#[test]
fn test_two_file_context_managers() {
    let python = r#"
def copy_file():
    with open("in.txt", "r") as fin, open("out.txt", "w") as fout:
        fout.write(fin.read())
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // Should generate nested with blocks
    assert!(rust.contains("File::open"), "Should use File::open for read");
    assert!(rust.contains("File::create"), "Should use File::create for write");

    // Should compile
    assert_compiles(&rust, "two_file_context_managers");
}

/// Test 2: Three context managers
#[test]
fn test_three_context_managers() {
    let python = r#"
def multi_file():
    with open("a.txt", "r") as a, open("b.txt", "r") as b, open("c.txt", "w") as c:
        content_a = a.read()
        content_b = b.read()
        c.write(content_a)
        c.write(content_b)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "three_context_managers");
}

/// Test 3: Context managers without targets (as clause)
#[test]
fn test_context_managers_without_targets() {
    let python = r#"
def no_targets():
    with open("a.txt", "r"), open("b.txt", "w"):
        pass
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "context_managers_without_targets");
}

/// Test 4: Mixed - one with target, one without
#[test]
fn test_mixed_targets() {
    let python = r#"
def mixed():
    with open("in.txt", "r") as f, open("out.txt", "w"):
        print(f.read())
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "mixed_targets");
}

/// Test 5: Body with multiple statements
#[test]
fn test_multiple_statements_in_body() {
    let python = r#"
def process_files():
    with open("in.txt", "r") as fin, open("out.txt", "w") as fout:
        content = fin.read()
        processed = content.upper()
        fout.write(processed)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "multiple_statements_in_body");
}
