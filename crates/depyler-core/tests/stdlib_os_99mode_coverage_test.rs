//! Coverage tests for rust_gen/stdlib_method_gen/os.rs
//!
//! DEPYLER-99MODE-001: Targets os.rs (~571 lines)
//! Covers: os.getcwd, os.path.join, os.path.exists,
//! os.environ, os.listdir, os.makedirs.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_os_getcwd() {
    let code = r#"
import os

def f() -> str:
    return os.getcwd()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_join() {
    let code = r#"
import os

def f(base: str, name: str) -> str:
    return os.path.join(base, name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_exists() {
    let code = r#"
import os

def f(path: str) -> bool:
    return os.path.exists(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_environ() {
    let code = r#"
import os

def f(key: str) -> str:
    return os.environ.get(key, "")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_listdir() {
    let code = r#"
import os

def f(path: str) -> list:
    return os.listdir(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_basename() {
    let code = r#"
import os

def f(path: str) -> str:
    return os.path.basename(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_dirname() {
    let code = r#"
import os

def f(path: str) -> str:
    return os.path.dirname(path)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_combined() {
    let code = r#"
import os

def f() -> dict:
    cwd = os.getcwd()
    return {"cwd": cwd}
"#;
    assert!(transpile_ok(code));
}
