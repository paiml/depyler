//! Coverage tests for lambda_inference/pattern_extraction.rs
//!
//! DEPYLER-99MODE-001: Targets pattern_extraction.rs (865 lines)
//! Covers: Lambda event type inference, access pattern extraction,
//! subscript chain analysis, attribute chain detection.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[test]
fn test_pattern_event_records() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    records = event.get("Records", [])
    return {"count": len(records)}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_body() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    body = event.get("body", "")
    return {"body": body}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_http_method() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    method = event.get("httpMethod", "GET")
    return {"method": method}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_path() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    path = event.get("path", "/")
    return {"path": path}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_in_if() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    if "body" in event:
        return {"statusCode": 200}
    return {"statusCode": 400}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_in_loop() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        body = record.get("body", "")
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_nested_access() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    headers = event.get("headers", {})
    content_type = headers.get("Content-Type", "")
    return {"type": content_type}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_event_passed_to_function() {
    let code = r#"
def process(data: dict) -> dict:
    return {"processed": True}

def handler(event: dict, context: dict) -> dict:
    return process(event)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pattern_complex_handler() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    method = event.get("httpMethod", "GET")
    path = event.get("path", "/")
    body = event.get("body", "")
    if method == "POST" and body:
        return {"statusCode": 201, "body": body}
    return {"statusCode": 200, "path": path}
"#;
    assert!(transpile_ok(code));
}
