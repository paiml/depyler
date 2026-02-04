//! Coverage tests for rust_gen/type_tokens.rs
//!
//! DEPYLER-99MODE-001: Targets type_tokens.rs (711 lines)
//! Covers: HIR Type to Rust TokenStream conversion, primitive types,
//! collection types, generic types, nested types.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_token_int() {
    let code = "def f(x: int) -> int:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_float() {
    let code = "def f(x: float) -> float:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_str() {
    let code = "def f(x: str) -> str:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_bool() {
    let code = "def f(x: bool) -> bool:\n    return x\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_list() {
    let code = "def f() -> list:\n    return [1, 2, 3]\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_dict() {
    let code = "def f() -> dict:\n    return {\"a\": 1}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_tuple() {
    let code = "def f() -> tuple:\n    return (1, 2)\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_set() {
    let code = "def f() -> set:\n    return {1, 2, 3}\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_optional() {
    let code = r#"
from typing import Optional

def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_token_list_generic() {
    let code = r#"
from typing import List

def f(items: List[int]) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_token_dict_generic() {
    let code = r#"
from typing import Dict

def f(d: Dict[str, int]) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_token_none_return() {
    let code = "def f():\n    pass\n";
    assert!(transpile_ok(code));
}

#[test]
fn test_token_mixed_params() {
    let code = r#"
def f(x: int, y: float, s: str, flag: bool) -> str:
    return s
"#;
    assert!(transpile_ok(code));
}
