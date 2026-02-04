//! Coverage tests for lambda_inference/mod.rs
//!
//! DEPYLER-99MODE-001: Targets lambda_inference/mod.rs (1,083 lines)
//! Covers: AWS Lambda event type inference, pattern matching,
//! S3/API Gateway/SNS/SQS/DynamoDB/EventBridge event detection.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// Basic Lambda handler
// ============================================================================

#[test]
fn test_lambda_basic_handler() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    return {"statusCode": 200, "body": "OK"}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// S3 event pattern
// ============================================================================

#[test]
fn test_lambda_s3_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    bucket = event["Records"][0]["s3"]["bucket"]["name"]
    key = event["Records"][0]["s3"]["object"]["key"]
    return {"bucket": bucket, "key": key}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// API Gateway event pattern
// ============================================================================

#[test]
fn test_lambda_api_gateway() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    method = event["httpMethod"]
    path = event["path"]
    return {"statusCode": 200, "body": path}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_api_gateway_v2() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    body = event["body"]
    return {"statusCode": 200, "body": body}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// SQS event pattern
// ============================================================================

#[test]
fn test_lambda_sqs_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event["Records"]:
        message_id = record["messageId"]
        body = record["body"]
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// SNS event pattern
// ============================================================================

#[test]
fn test_lambda_sns_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    message = event["Records"][0]["Sns"]["Message"]
    return {"message": message}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// DynamoDB event pattern
// ============================================================================

#[test]
fn test_lambda_dynamodb_event() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    for record in event["Records"]:
        event_name = record["eventName"]
    return {"processed": True}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// EventBridge pattern
// ============================================================================

#[test]
fn test_lambda_eventbridge() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    source = event["source"]
    detail = event["detail"]
    return {"source": source}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex handlers
// ============================================================================

#[test]
fn test_lambda_handler_with_processing() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    items = event.get("items", [])
    total = 0
    for item in items:
        total += int(item)
    return {"total": total, "count": len(items)}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_handler_with_error() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    try:
        data = event["data"]
        return {"statusCode": 200, "body": data}
    except:
        return {"statusCode": 500, "body": "Error"}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_handler_with_condition() {
    let code = r#"
def handler(event: dict, context: dict) -> dict:
    action = event.get("action", "default")
    if action == "create":
        return {"statusCode": 201}
    elif action == "delete":
        return {"statusCode": 204}
    return {"statusCode": 200}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Non-Lambda patterns (negative tests)
// ============================================================================

#[test]
fn test_lambda_non_handler() {
    let code = r#"
def process(data: list) -> int:
    return sum(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_class_handler() {
    let code = r#"
class LambdaHandler:
    def __init__(self):
        self.count = 0

    def handle(self, event: dict) -> dict:
        self.count += 1
        return {"count": self.count}
"#;
    assert!(transpile_ok(code));
}
