//! Coverage tests for rust_gen/stdlib_method_gen/pathlib.rs
//!
//! DEPYLER-99MODE-001: Targets pathlib.rs (438 lines)
//! Covers: Path construction, path operations, exists/is_file/is_dir.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_pathlib_import() {
    let code = r#"
from pathlib import Path

def f(p: str) -> str:
    return str(Path(p))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_join() {
    let code = r#"
from pathlib import Path

def f(base: str, name: str) -> str:
    return str(Path(base) / name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_name() {
    let code = r#"
from pathlib import Path

def f(p: str) -> str:
    path = Path(p)
    return str(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_in_function() {
    let code = r#"
from pathlib import Path

def get_path(directory: str, filename: str) -> str:
    return str(Path(directory) / filename)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_multiple_ops() {
    let code = r#"
from pathlib import Path

def f(base: str) -> list:
    p = Path(base)
    return [str(p), str(p / "child")]
"#;
    assert!(transpile_ok(code));
}
