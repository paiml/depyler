//! DEPYLER-0493: Constructor Pattern Recognition Tests
//!
//! Tests that Python class instantiation is correctly transpiled to Rust
//! constructor patterns (::new(), ::open(), etc.) instead of calling structs
//! as functions.

use depyler_core::DepylerPipeline;

#[test]
fn test_tempfile_namedtempfile_constructor() {
    let python = r#"
import tempfile

def create_temp():
    temp_file = tempfile.NamedTemporaryFile(delete=False)
    return temp_file
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should generate ::new() pattern, not call struct as function
    assert!(
        rust.contains("tempfile::NamedTempFile::new()"),
        "Expected tempfile::NamedTempFile::new() pattern, got:\n{}",
        rust
    );

    // Should NOT call struct as function
    assert!(
        !rust.contains("tempfile::NamedTempFile()"),
        "Should not call struct as function, got:\n{}",
        rust
    );
}

#[test]
fn test_tempfile_tempdir_constructor() {
    let python = r#"
import tempfile

def create_tempdir():
    temp_dir = tempfile.TemporaryDirectory()
    return temp_dir
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should generate ::new() pattern for TempDir
    assert!(
        rust.contains("tempfile::TempDir::new()"),
        "Expected tempfile::TempDir::new() pattern, got:\n{}",
        rust
    );
}

#[test]
fn test_constructor_pattern_with_args() {
    let python = r#"
import tempfile

def create_with_args():
    # Note: Rust NamedTempFile::new() doesn't take args,
    # but test that args are passed through correctly
    temp = tempfile.NamedTemporaryFile(delete=False)
    return temp
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should use ::new() pattern (kwargs like delete=False are handled separately)
    assert!(
        rust.contains("::new()"),
        "Expected ::new() constructor pattern, got:\n{}",
        rust
    );
}

#[test]
fn test_multiple_constructor_patterns() {
    let python = r#"
import tempfile

def create_multiple():
    temp_file = tempfile.NamedTemporaryFile()
    temp_dir = tempfile.TemporaryDirectory()
    return (temp_file, temp_dir)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should generate ::new() for both types
    assert!(
        rust.contains("NamedTempFile::new()"),
        "Expected NamedTempFile::new(), got:\n{}",
        rust
    );

    assert!(
        rust.contains("TempDir::new()"),
        "Expected TempDir::new(), got:\n{}",
        rust
    );
}

#[test]
fn test_function_pattern_not_constructor() {
    // Test that functions (not structs) are called correctly
    let python = r#"
import tempfile

def use_tempfile_function():
    # tempfile.mkstemp() is a function, not a constructor
    fd = tempfile.mkstemp()
    return fd
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // mkstemp should be called as a function (mapped to tempfile::tempfile in Rust)
    assert!(
        rust.contains("tempfile::tempfile()"),
        "Expected function call pattern, got:\n{}",
        rust
    );

    // Should NOT have ::new() for functions
    assert!(
        !rust.contains("tempfile::new()"),
        "Function should not use ::new() pattern, got:\n{}",
        rust
    );
}

#[test]
fn test_io_bufreader_constructor() {
    let python = r#"
import io

def create_bufreader(file):
    reader = io.BufferedReader(file)
    return reader
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should generate ::new() pattern for BufReader
    assert!(
        rust.contains("std::io::BufReader::new"),
        "Expected std::io::BufReader::new() pattern, got:\n{}",
        rust
    );

    // Should NOT call struct as function
    assert!(
        !rust.contains("BufReader()"),
        "Should not call BufReader as function, got:\n{}",
        rust
    );
}

#[test]
fn test_io_bufwriter_constructor() {
    let python = r#"
import io

def create_bufwriter(file):
    writer = io.BufferedWriter(file)
    return writer
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should generate ::new() pattern for BufWriter
    assert!(
        rust.contains("std::io::BufWriter::new"),
        "Expected std::io::BufWriter::new() pattern, got:\n{}",
        rust
    );
}
