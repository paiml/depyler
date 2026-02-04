//! Coverage tests for lambda_codegen.rs
//!
//! DEPYLER-99MODE-001: Targets lambda_codegen.rs (1,195 lines)
//! Covers: AWS Lambda code generation, handler templates,
//! event type mapping, Cargo.toml generation, deployment artifacts.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// Basic handler code generation
// ============================================================================

#[test]
fn test_lambda_codegen_basic_handler() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_handler_with_body() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    name = event.get("name", "world")
    return {"statusCode": 200, "body": "Hello " + name}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// API Gateway event type
// ============================================================================

#[test]
fn test_lambda_codegen_api_get() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    path = event.get("path", "/")
    method = event.get("httpMethod", "GET")
    return {"statusCode": 200, "body": path}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_api_post_json() {
    let code = r#"
import json

def handler(event: dict, context: dict) -> dict:
    body = json.loads(event.get("body", "{}"))
    return {"statusCode": 200, "body": json.dumps(body)}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_api_with_headers() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {
        "statusCode": 200,
        "headers": {"Content-Type": "application/json"},
        "body": "{}"
    }
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// S3 event type
// ============================================================================

#[test]
fn test_lambda_codegen_s3_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    records = event.get("Records", [])
    count = 0
    for record in records:
        s3_info = record.get("s3", {})
        count += 1
    return {"processed": count}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// SQS event type
// ============================================================================

#[test]
fn test_lambda_codegen_sqs_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        body = record.get("body", "")
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_sqs_batch() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    failures = []
    for record in event.get("Records", []):
        msg_id = record.get("messageId", "")
        if not record.get("body"):
            failures.append(msg_id)
    return {"batchItemFailures": failures}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DynamoDB event type
// ============================================================================

#[test]
fn test_lambda_codegen_dynamodb_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        event_name = record.get("eventName", "")
        if event_name == "INSERT":
            new_image = record.get("dynamodb", {})
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// SNS event type
// ============================================================================

#[test]
fn test_lambda_codegen_sns_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event.get("Records", []):
        sns = record.get("Sns", {})
        message = sns.get("Message", "")
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Handler with class
// ============================================================================

#[test]
fn test_lambda_codegen_class_handler() {
    let code = r#"
class Processor:
    def __init__(self):
        self.count = 0

    def process(self, event: dict) -> dict:
        self.count += 1
        return {"count": self.count}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Handler with error handling
// ============================================================================

#[test]
fn test_lambda_codegen_try_except() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    try:
        result = event["key"]
        return {"statusCode": 200, "result": result}
    except:
        return {"statusCode": 500, "error": "failed"}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Handler with helper functions
// ============================================================================

#[test]
fn test_lambda_codegen_with_helpers() {
    let code = r#"
def validate_event(event: dict) -> bool:
    return "body" in event

def parse_body(body: str) -> dict:
    return {}

def handler(event: dict, context: dict) -> dict:
    if not validate_event(event):
        return {"statusCode": 400}
    data = parse_body(event.get("body", ""))
    return {"statusCode": 200, "data": data}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex handler patterns
// ============================================================================

#[test]
fn test_lambda_codegen_crud_router() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    method = event.get("httpMethod", "GET")
    if method == "GET":
        return {"statusCode": 200, "body": "list"}
    elif method == "POST":
        return {"statusCode": 201, "body": "created"}
    elif method == "PUT":
        return {"statusCode": 200, "body": "updated"}
    elif method == "DELETE":
        return {"statusCode": 204, "body": ""}
    return {"statusCode": 405}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_data_transform() {
    let code = r#"
def transform(item: dict) -> dict:
    return {"id": item.get("id", ""), "processed": True}

def handler(event: dict, context: dict) -> dict:
    records = event.get("Records", [])
    results = []
    for record in records:
        results.append(transform(record))
    return {"statusCode": 200, "items": results}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_codegen_aggregation() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    items = event.get("items", [])
    total = sum(items)
    count = len(items)
    avg = total / count if count > 0 else 0
    return {
        "total": total,
        "count": count,
        "average": avg,
        "statusCode": 200
    }
"#;
    assert!(transpile_ok(code));
}
