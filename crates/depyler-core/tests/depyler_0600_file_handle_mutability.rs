//! TDD Tests for File Handle Mutability (DEPYLER-0600 Bug #2)
//!
//! Bug: `write_to(f, ...)` should be `write_to(&mut f, ...)`
//! when function signature expects `&mut File`
//!
//! Root cause: File types weren't triggering `should_borrow`, so
//! the `function_param_muts` check at line 3003 was never reached.
//!
//! Fix: Check `function_param_muts` first to determine if &mut is needed.

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

/// Test: Passing File handle to function expecting &mut File
#[test]
fn test_file_handle_to_mut_param() {
    let python = r#"
def write_to(f, msg: str):
    f.write(msg)

def main():
    f = open("out.txt", "w")
    write_to(f, "hello\n")
    f.close()
"#;

    let rust = transpile(python).expect("Transpilation should succeed");

    // The call should use &mut f, not f
    assert!(
        rust.contains("&mut f") || rust.contains("&mut f,"),
        "Should pass file handle as &mut. Generated:\n{}", rust
    );

    assert_compiles(&rust, "file_handle_to_mut_param");
}

/// Test: Multiple file handles to mutating functions
#[test]
#[ignore] // Complex pattern - separate issue with for loop over file
fn test_multiple_file_handles() {
    let python = r#"
def copy_line(src, dst, line: str):
    dst.write(line)

def process_files():
    src = open("in.txt", "r")
    dst = open("out.txt", "w")
    for line in src:
        copy_line(src, dst, line)
"#;

    let rust = transpile(python).expect("Transpilation should succeed");
    assert_compiles(&rust, "multiple_file_handles");
}
