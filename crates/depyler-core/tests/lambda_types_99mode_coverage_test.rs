//! Coverage tests for lambda_types.rs
//!
//! DEPYLER-99MODE-001: Targets lambda_types.rs (1,167 lines)
//! Covers: AWS Lambda type mapping, event type routing,
//! response type mapping, type conversion rules.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Event handler type mapping
// ============================================================================

#[test]
fn test_lambda_type_basic_handler() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_api_gateway() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    method = event.get("httpMethod", "GET")
    path = event.get("path", "/")
    return {"statusCode": 200, "body": method}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_s3_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        bucket = record.get("s3", {})
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_sqs_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        body = record.get("body", "")
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_sns_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        sns = record.get("Sns", {})
        message = sns.get("Message", "")
    return {"processed": True}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_dynamodb_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        name = record.get("eventName", "")
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Response type patterns
// ============================================================================

#[test]
fn test_lambda_type_response_with_headers() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {
        "statusCode": 200,
        "headers": {"Content-Type": "application/json"},
        "body": "OK"
    }
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_error_response() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {"statusCode": 500, "body": "Internal Error"}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion patterns
// ============================================================================

#[test]
fn test_lambda_type_string_param() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    name = str(event.get("name", ""))
    return {"statusCode": 200, "body": name}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_int_param() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    count = int(event.get("count", 0))
    return {"statusCode": 200, "count": count}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_type_list_records() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    records = event.get("Records", [])
    count = len(records)
    return {"count": count}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Error handling type patterns
// ============================================================================

#[test]
fn test_lambda_type_try_except() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    try:
        value = event["key"]
        return {"statusCode": 200, "value": value}
    except:
        return {"statusCode": 400, "error": "missing key"}
"#;
    assert!(transpile_ok(code));
}
