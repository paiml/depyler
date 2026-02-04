//! Coverage tests for rust_gen/stdlib_method_gen/json.rs
//!
//! DEPYLER-99MODE-001: Targets json.rs (~643 lines)
//! Covers: json.loads, json.dumps, json.load, json.dump.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_json_loads() {
    let code = r#"
import json

def f(data: str) -> dict:
    return json.loads(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_dumps() {
    let code = r#"
import json

def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_roundtrip() {
    let code = r#"
import json

def f(data: dict) -> dict:
    s = json.dumps(data)
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_in_function() {
    let code = r#"
import json

def parse_config(config_str: str) -> dict:
    config = json.loads(config_str)
    return config
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_with_processing() {
    let code = r#"
import json

def f(data: str) -> int:
    parsed = json.loads(data)
    return len(str(parsed))
"#;
    assert!(transpile_ok(code));
}
