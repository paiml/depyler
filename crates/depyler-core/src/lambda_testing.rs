use anyhow::Result;
use depyler_annotations::{LambdaAnnotations, LambdaEventType};
// use lambda_runtime::{Context, LambdaEvent};
// use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
// use std::time::Duration;
// use tokio::time::timeout;

/// Local Lambda testing harness for development and CI/CD
#[derive(Debug, Clone)]
pub struct LambdaTestHarness {
    test_events: HashMap<LambdaEventType, Vec<TestEvent>>,
    test_context: TestContext,
    performance_benchmarks: PerformanceBenchmarks,
}

#[derive(Debug, Clone)]
pub struct TestEvent {
    pub name: String,
    pub event_data: Value,
    pub expected_response: Option<Value>,
    pub should_succeed: bool,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TestContext {
    pub function_name: String,
    pub function_version: String,
    pub memory_limit_mb: u32,
    pub timeout_ms: u64,
    pub aws_request_id: String,
    pub invoked_function_arn: String,
}

#[derive(Debug, Clone)]
pub struct PerformanceBenchmarks {
    pub max_cold_start_ms: u64,
    pub max_warm_start_ms: u64,
    pub max_memory_usage_mb: u32,
    pub min_throughput_rps: u32,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub duration_ms: u64,
    pub memory_usage_mb: Option<u32>,
    pub error_message: Option<String>,
    pub response: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub cold_start_ms: u64,
    pub warm_start_ms: u64,
    pub memory_usage_mb: u32,
    pub throughput_rps: f64,
    pub binary_size_kb: u32,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            function_name: "test-function".to_string(),
            function_version: "$LATEST".to_string(),
            memory_limit_mb: 128,
            timeout_ms: 15000,
            aws_request_id: "test-request-id".to_string(),
            invoked_function_arn: "arn:aws:lambda:us-east-1:123456789012:function:test-function"
                .to_string(),
        }
    }
}

impl Default for PerformanceBenchmarks {
    fn default() -> Self {
        Self {
            max_cold_start_ms: 100,
            max_warm_start_ms: 10,
            max_memory_usage_mb: 64,
            min_throughput_rps: 100,
        }
    }
}

impl Default for LambdaTestHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaTestHarness {
    pub fn new() -> Self {
        let mut test_events = HashMap::new();

        // Add default test events for common Lambda event types
        test_events.insert(
            LambdaEventType::ApiGatewayProxyRequest,
            vec![
                TestEvent {
                    name: "basic_get_request".to_string(),
                    event_data: json!({
                        "httpMethod": "GET",
                        "path": "/test",
                        "headers": {
                            "Content-Type": "application/json"
                        },
                        "queryStringParameters": {
                            "param1": "value1"
                        },
                        "body": null,
                        "pathParameters": {
                            "id": "123"
                        },
                        "requestContext": {
                            "requestId": "test-request-id",
                            "stage": "test"
                        }
                    }),
                    expected_response: Some(json!({
                        "statusCode": 200,
                        "headers": {
                            "Content-Type": "application/json"
                        },
                        "body": "{\"message\":\"success\"}"
                    })),
                    should_succeed: true,
                    description: "Basic GET request test".to_string(),
                },
                TestEvent {
                    name: "post_request_with_body".to_string(),
                    event_data: json!({
                        "httpMethod": "POST",
                        "path": "/test",
                        "headers": {
                            "Content-Type": "application/json"
                        },
                        "body": "{\"name\":\"test\",\"value\":42}",
                        "requestContext": {
                            "requestId": "test-request-id-2"
                        }
                    }),
                    expected_response: None,
                    should_succeed: true,
                    description: "POST request with JSON body".to_string(),
                },
            ],
        );

        test_events.insert(
            LambdaEventType::S3Event,
            vec![TestEvent {
                name: "s3_object_created".to_string(),
                event_data: json!({
                    "Records": [
                        {
                            "s3": {
                                "bucket": {
                                    "name": "test-bucket"
                                },
                                "object": {
                                    "key": "test-file.txt",
                                    "size": 1024
                                }
                            },
                            "eventName": "ObjectCreated:Put"
                        }
                    ]
                }),
                expected_response: None,
                should_succeed: true,
                description: "S3 object created event".to_string(),
            }],
        );

        test_events.insert(
            LambdaEventType::SqsEvent,
            vec![
                TestEvent {
                    name: "sqs_single_message".to_string(),
                    event_data: json!({
                        "Records": [
                            {
                                "messageId": "test-message-id",
                                "receiptHandle": "test-receipt-handle",
                                "body": "{\"message\":\"Hello World\"}",
                                "attributes": {
                                    "ApproximateReceiveCount": "1"
                                }
                            }
                        ]
                    }),
                    expected_response: Some(json!({
                        "batchItemFailures": []
                    })),
                    should_succeed: true,
                    description: "Single SQS message processing".to_string(),
                },
                TestEvent {
                    name: "sqs_batch_messages".to_string(),
                    event_data: json!({
                        "Records": [
                            {
                                "messageId": "msg1",
                                "body": "{\"id\":1}"
                            },
                            {
                                "messageId": "msg2",
                                "body": "{\"id\":2}"
                            },
                            {
                                "messageId": "msg3",
                                "body": "invalid json"
                            }
                        ]
                    }),
                    expected_response: Some(json!({
                        "batchItemFailures": [
                            {
                                "itemIdentifier": "msg3"
                            }
                        ]
                    })),
                    should_succeed: true,
                    description: "Batch SQS messages with one failure".to_string(),
                },
            ],
        );

        Self {
            test_events,
            test_context: TestContext::default(),
            performance_benchmarks: PerformanceBenchmarks::default(),
        }
    }

    pub fn with_context(mut self, context: TestContext) -> Self {
        self.test_context = context;
        self
    }

    pub fn with_benchmarks(mut self, benchmarks: PerformanceBenchmarks) -> Self {
        self.performance_benchmarks = benchmarks;
        self
    }

    /// Add a custom test event
    pub fn add_test_event(&mut self, event_type: LambdaEventType, test_event: TestEvent) {
        self.test_events
            .entry(event_type)
            .or_default()
            .push(test_event);
    }

    /// Generate test suite for a Lambda function
    pub fn generate_test_suite(&self, annotations: &LambdaAnnotations) -> Result<String> {
        let mut test_code = String::new();

        test_code.push_str(&self.generate_test_imports());
        test_code.push_str(&self.generate_test_helpers());

        if let Some(ref event_type) = annotations.event_type {
            if let Some(events) = self.test_events.get(event_type) {
                for event in events {
                    test_code.push_str(&self.generate_individual_test(event, event_type)?);
                }
            }
        }

        test_code.push_str(&self.generate_performance_tests());
        test_code.push_str(&self.generate_integration_tests());

        Ok(test_code)
    }

    fn generate_test_imports(&self) -> String {
        r#"#[cfg(test)]
mod tests {{
    use super::*;
    use lambda_runtime::{{Context, LambdaEvent}};
    use serde_json::{{json, Value}};
    use std::time::Instant;
    // use tokio::time::timeout;

"#
        .to_string()
    }

    fn generate_test_helpers(&self) -> String {
        format!(
            r#"    // Test helper functions
    fn create_test_context() -> Context {{
        Context {{
            request_id: "{}".to_string(),
            deadline: std::time::Instant::now() + std::time::Duration::from_millis({}),
            invoked_function_arn: "{}".to_string(),
            xray_trace_id: None,
            client_context: None,
            identity: None,
            env_config: lambda_runtime::Config::default(),
        }}
    }}

    async fn run_with_timeout<F, T>(
        future: F,
        timeout_ms: u64,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        F: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {{
        match timeout(std::time::Duration::from_millis(timeout_ms), future).await {{
            Ok(result) => result,
            Err(_) => Err(format!("Test timed out after {{}}ms", timeout_ms).into()),
        }}
    }}

"#,
            self.test_context.aws_request_id,
            self.test_context.timeout_ms,
            self.test_context.invoked_function_arn
        )
    }

    fn generate_individual_test(
        &self,
        test_event: &TestEvent,
        _event_type: &LambdaEventType,
    ) -> Result<String> {
        let test_function_name = format!("test_{}", test_event.name);
        let handler_name = "handler"; // This could be configurable

        let mut test_code = format!(
            r#"    #[tokio::test]
    async fn {}() {{
        // {}
        let event_data = {};
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);

        let start_time = Instant::now();
        let result = run_with_timeout(
            async {{ {}(event).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>) }},
            {}
        ).await;
        let duration = start_time.elapsed();

        println!("Test '{}' completed in {{:?}}", duration);

"#,
            test_function_name,
            test_event.description,
            serde_json::to_string_pretty(&test_event.event_data)?,
            handler_name,
            self.test_context.timeout_ms,
            test_event.name
        );

        if test_event.should_succeed {
            test_code.push_str("        assert!(result.is_ok(), \"Test should succeed but failed: {:?}\", result.err());\n");

            if let Some(ref expected) = test_event.expected_response {
                test_code.push_str(&format!(
                    r#"        
        let response = result.unwrap();
        let expected_response = {};
        assert_eq!(response, expected_response, "Response doesn't match expected");
"#,
                    serde_json::to_string_pretty(expected)?
                ));
            }
        } else {
            test_code.push_str(
                "        assert!(result.is_err(), \"Test should fail but succeeded\");\n",
            );
        }

        // Add performance assertions
        test_code.push_str(&format!(
            r#"        
        // Performance assertions
        assert!(duration.as_millis() < {}, "Test took too long: {{:?}}", duration);
"#,
            self.performance_benchmarks.max_warm_start_ms
        ));

        test_code.push_str("    }\n\n");
        Ok(test_code)
    }

    fn generate_performance_tests(&self) -> String {
        format!(
            r#"    #[tokio::test]
    async fn test_cold_start_performance() {{
        // Simulate cold start by running handler for the first time
        let event_data = json!({{ "test": "cold_start" }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);

        let start_time = Instant::now();
        let _result = handler(event).await;
        let cold_start_duration = start_time.elapsed();

        println!("Cold start duration: {{:?}}", cold_start_duration);
        assert!(
            cold_start_duration.as_millis() < {},
            "Cold start took too long: {{:?}}",
            cold_start_duration
        );
    }}

    #[tokio::test]
    async fn test_warm_start_performance() {{
        // First invocation (warm up)
        let event_data = json!({{ "test": "warm_up" }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);
        let _ = handler(event).await;

        // Second invocation (warm start)
        let event_data = json!({{ "test": "warm_start" }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);

        let start_time = Instant::now();
        let _result = handler(event).await;
        let warm_start_duration = start_time.elapsed();

        println!("Warm start duration: {{:?}}", warm_start_duration);
        assert!(
            warm_start_duration.as_millis() < {},
            "Warm start took too long: {{:?}}",
            warm_start_duration
        );
    }}

    #[tokio::test]
    async fn test_memory_usage() {{
        // This is a placeholder - actual memory testing would require 
        // system-specific tools or profiling crates
        println!("Memory usage test - implement with system profiling tools");
        
        // You could use crates like `memory-stats` or system calls here
        // For now, we'll just ensure the test runs
        let event_data = json!({{ "test": "memory" }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);
        
        let _result = handler(event).await;
        // Memory assertions would go here
    }}

    #[tokio::test]
    async fn test_concurrent_invocations() {{
        use std::sync::Arc;
        use tokio::sync::Semaphore;
        
        let concurrency = 10;
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let mut handles = Vec::new();

        let start_time = Instant::now();
        
        for i in 0..concurrency {{
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let handle = tokio::spawn(async move {{
                let _permit = permit;
                let event_data = json!({{ "test": "concurrent", "id": i }});
                let context = create_test_context();
                let event = LambdaEvent::new(event_data, context);
                handler(event).await
            }});
            handles.push(handle);
        }}

        // Wait for all invocations to complete
        for handle in handles {{
            let result = handle.await.unwrap();
            assert!(result.is_ok(), "Concurrent invocation failed: {{:?}}", result.err());
        }}

        let total_duration = start_time.elapsed();
        let throughput = concurrency as f64 / total_duration.as_secs_f64();
        
        println!("Throughput: {{:.2}} RPS", throughput);
        assert!(
            throughput >= {} as f64,
            "Throughput too low: {{:.2}} RPS",
            throughput
        );
    }}

"#,
            self.performance_benchmarks.max_cold_start_ms,
            self.performance_benchmarks.max_warm_start_ms,
            self.performance_benchmarks.min_throughput_rps
        )
    }

    fn generate_integration_tests(&self) -> String {
        r#"    #[tokio::test]
    async fn test_error_handling() {{
        // Test with invalid event data to ensure proper error handling
        let invalid_event = json!({{ "invalid": "data" }});
        let context = create_test_context();
        let event = LambdaEvent::new(invalid_event, context);

        let result = handler(event).await;
        
        // Depending on your error handling strategy, adjust this assertion
        // For now, we'll just ensure it doesn't panic
        println!("Error handling result: {{:?}}", result);
    }}

    #[tokio::test]
    async fn test_timeout_handling() {{
        // Test timeout behavior (if applicable)
        let event_data = json!({{ "test": "timeout" }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);

        // Set a very short timeout to test timeout handling
        let result = run_with_timeout(
            async {{ handler(event).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>) }},
            1 // 1ms timeout
        ).await;

        // This should timeout
        assert!(result.is_err(), "Handler should have timed out");
    }}

    #[tokio::test]
    async fn test_large_payload() {{
        // Test with a large payload to ensure memory efficiency
        let large_data = "x".repeat(1024 * 100); // 100KB string
        let event_data = json!({{ "large_data": large_data }});
        let context = create_test_context();
        let event = LambdaEvent::new(event_data, context);

        let start_time = Instant::now();
        let result = handler(event).await;
        let duration = start_time.elapsed();

        assert!(result.is_ok(), "Large payload test failed: {{:?}}", result.err());
        println!("Large payload processing time: {{:?}}", duration);
    }}
}}
"#
        .to_string()
    }

    /// Generate a complete test script for cargo lambda test
    pub fn generate_cargo_lambda_test_script(
        &self,
        annotations: &LambdaAnnotations,
    ) -> Result<String> {
        let mut script = String::from("#!/bin/bash\n");
        script.push_str("# Generated test script for cargo-lambda\n\n");
        script.push_str("set -e\n\n");

        // Build first
        script.push_str("echo \"Building Lambda function...\"\n");
        script.push_str("cargo lambda build --release\n\n");

        // Run unit tests
        script.push_str("echo \"Running unit tests...\"\n");
        script.push_str("cargo test\n\n");

        // Run integration tests with cargo-lambda
        script.push_str("echo \"Running integration tests with cargo-lambda...\"\n");

        if let Some(ref event_type) = annotations.event_type {
            if let Some(events) = self.test_events.get(event_type) {
                for event in events {
                    script.push_str(&format!("echo \"Testing event: {}\"\n", event.name));

                    // Create temporary event file
                    script.push_str(&format!(
                        "cat > /tmp/test_event_{}.json << 'EOF'\n{}\nEOF\n",
                        event.name,
                        serde_json::to_string_pretty(&event.event_data)?
                    ));

                    // Invoke with cargo lambda
                    script.push_str(&format!(
                        "cargo lambda invoke --data-file /tmp/test_event_{}.json\n",
                        event.name
                    ));

                    script.push_str(&format!("rm /tmp/test_event_{}.json\n\n", event.name));
                }
            }
        }

        // Performance benchmarks
        script.push_str("echo \"Running performance benchmarks...\"\n");
        script.push_str("if command -v hyperfine > /dev/null; then\n");
        script.push_str("    echo \"Benchmarking cold start performance...\"\n");
        script.push_str(
            "    hyperfine 'cargo lambda invoke --data-ascii \\'{}\\'' --warmup 1 --min-runs 10\n",
        );
        script.push_str("else\n");
        script.push_str("    echo \"hyperfine not installed, skipping performance benchmarks\"\n");
        script.push_str("fi\n\n");

        script.push_str("echo \"All tests completed successfully!\"\n");

        Ok(script)
    }

    /// Generate a GitHub Actions workflow for Lambda testing
    pub fn generate_github_actions_workflow(
        &self,
        annotations: &LambdaAnnotations,
    ) -> Result<String> {
        Ok(format!(
            r#"name: Lambda Function Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        components: rustfmt, clippy
    
    - name: Install cargo-lambda
      run: |
        pip install cargo-lambda
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{{{ runner.os }}}}-cargo-registry-${{{{ hashFiles('**/Cargo.lock') }}}}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{{{ runner.os }}}}-cargo-index-${{{{ hashFiles('**/Cargo.lock') }}}}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{{{ runner.os }}}}-cargo-build-target-${{{{ hashFiles('**/Cargo.lock') }}}}

    - name: Format check
      run: cargo fmt --check

    - name: Clippy check
      run: cargo clippy -- -D warnings

    - name: Run tests
      run: cargo test --verbose

    - name: Build Lambda function
      run: cargo lambda build --release{}

    - name: Test Lambda function locally
      run: |
        # Create test events
{}
        
        # Run integration tests
        cargo lambda invoke --data-file test_events/basic_test.json

    - name: Performance benchmarks
      if: github.event_name == 'push'
      run: |
        if command -v hyperfine > /dev/null; then
          hyperfine 'cargo lambda invoke --data-ascii "{{\"test\":\"benchmark\"}}"' --warmup 1
        fi

    - name: Security audit
      run: |
        cargo install --force cargo-audit
        cargo audit

    - name: Check binary size
      run: |
        BINARY_SIZE=$(du -h target/lambda/*/bootstrap | cut -f1)
        echo "Binary size: $BINARY_SIZE"
        # Add size threshold check here if needed
"#,
            match annotations.architecture {
                depyler_annotations::Architecture::Arm64 => " --arm64",
                depyler_annotations::Architecture::X86_64 => " --x86-64",
            },
            self.generate_test_events_yaml(annotations)?
        ))
    }

    fn generate_test_events_yaml(&self, annotations: &LambdaAnnotations) -> Result<String> {
        let mut yaml = String::from("        mkdir -p test_events\n");

        if let Some(ref event_type) = annotations.event_type {
            if let Some(events) = self.test_events.get(event_type) {
                for (i, event) in events.iter().enumerate() {
                    yaml.push_str(&format!(
                        "        cat > test_events/test_{}.json << 'EOF'\n{}\n        EOF\n",
                        i,
                        serde_json::to_string_pretty(&event.event_data)?
                    ));
                }
            }
        }

        yaml.push_str("        cat > test_events/basic_test.json << 'EOF'\n{\"test\": \"basic\"}\n        EOF\n");

        Ok(yaml)
    }

    /// Generate local development testing script
    pub fn generate_local_dev_script(&self) -> String {
        r#"#!/bin/bash
# Local development testing script

set -e

echo "🚀 Starting local Lambda development testing..."

# Build the function
echo "📦 Building Lambda function..."
cargo lambda build --release

# Run unit tests
echo "🧪 Running unit tests..."
cargo test

# Start local development server (if available)
if command -v cargo-lambda &> /dev/null; then
    echo "🌐 Starting local Lambda server..."
    echo "You can test your function at: http://localhost:9000/lambda-url/function_name/"
    
    # Start the server in background
    cargo lambda start &
    SERVER_PID=$!
    
    # Wait a moment for server to start
    sleep 2
    
    # Test basic invocation
    echo "🔍 Testing basic invocation..."
    curl -X POST \
        -H "Content-Type: application/json" \
        -d '{{"test": "local_development"}}' \
        http://localhost:9000/2015-03-31/functions/function/invocations
    
    echo -e "\n✅ Local testing completed!"
    echo "🛑 Stopping local server..."
    kill $SERVER_PID
else
    echo "⚠️ cargo-lambda not found. Install with: pip install cargo-lambda"
fi

echo "🎉 Development testing finished!"
"#
        .to_string()
    }

    /// Generate load testing script
    pub fn generate_load_test_script(&self, annotations: &LambdaAnnotations) -> Result<String> {
        Ok(format!(
            r#"#!/bin/bash
# Load testing script for Lambda function

set -e

echo "🔥 Starting load testing..."

# Configuration
CONCURRENCY=10
DURATION=30
MEMORY_SIZE={}

# Build first
cargo lambda build --release

# Create test event
cat > load_test_event.json << 'EOF'
{{"test": "load_test", "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"}}
EOF

# Run load test
if command -v hey > /dev/null; then
    echo "Running load test with hey..."
    
    # Start local server
    cargo lambda start &
    SERVER_PID=$!
    sleep 2
    
    # Run load test
    hey -z ${{DURATION}}s -c ${{CONCURRENCY}} \
        -m POST \
        -H "Content-Type: application/json" \
        -D load_test_event.json \
        http://localhost:9000/2015-03-31/functions/function/invocations
    
    # Cleanup
    kill $SERVER_PID
    rm load_test_event.json
    
elif command -v ab > /dev/null; then
    echo "Running load test with Apache Bench..."
    
    # Start local server
    cargo lambda start &
    SERVER_PID=$!
    sleep 2
    
    # Run ab test
    ab -n 1000 -c ${{CONCURRENCY}} \
        -p load_test_event.json \
        -T application/json \
        http://localhost:9000/2015-03-31/functions/function/invocations
    
    # Cleanup
    kill $SERVER_PID
    rm load_test_event.json
    
else
    echo "No load testing tool found. Install 'hey' or 'ab' (Apache Bench)"
    exit 1
fi

echo "✅ Load testing completed!"
"#,
            annotations.memory_size
        ))
    }
}

impl TestContext {
    // Note: This would be available when using the actual lambda_runtime crate
    // pub fn to_lambda_context(&self) -> Context {
    //     Context {
    //         request_id: self.aws_request_id.clone(),
    //         deadline: std::time::Instant::now() + Duration::from_millis(self.timeout_ms),
    //         invoked_function_arn: self.invoked_function_arn.clone(),
    //         xray_trace_id: None,
    //         client_context: None,
    //         identity: None,
    //         env_config: lambda_runtime::Config::default(),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_creation() {
        let harness = LambdaTestHarness::new();
        assert!(!harness.test_events.is_empty());
        assert!(harness
            .test_events
            .contains_key(&LambdaEventType::ApiGatewayProxyRequest));
        assert!(harness.test_events.contains_key(&LambdaEventType::S3Event));
        assert!(harness.test_events.contains_key(&LambdaEventType::SqsEvent));
    }

    #[test]
    fn test_custom_test_event() {
        let mut harness = LambdaTestHarness::new();

        let custom_event = TestEvent {
            name: "custom_test".to_string(),
            event_data: json!({"custom": "data"}),
            expected_response: None,
            should_succeed: true,
            description: "Custom test event".to_string(),
        };

        harness.add_test_event(LambdaEventType::ApiGatewayProxyRequest, custom_event);

        let events = harness
            .test_events
            .get(&LambdaEventType::ApiGatewayProxyRequest)
            .unwrap();
        assert!(events.iter().any(|e| e.name == "custom_test"));
    }

    #[test]
    fn test_test_suite_generation() {
        let harness = LambdaTestHarness::new();
        let annotations = depyler_annotations::LambdaAnnotations {
            event_type: Some(LambdaEventType::ApiGatewayProxyRequest),
            ..Default::default()
        };

        let test_suite = harness.generate_test_suite(&annotations).unwrap();

        assert!(test_suite.contains("#[tokio::test]"));
        assert!(test_suite.contains("test_basic_get_request"));
        assert!(test_suite.contains("test_cold_start_performance"));
    }

    #[test]
    fn test_github_actions_workflow() {
        let harness = LambdaTestHarness::new();
        let annotations = depyler_annotations::LambdaAnnotations::default();

        let workflow = harness
            .generate_github_actions_workflow(&annotations)
            .unwrap();

        assert!(workflow.contains("name: Lambda Function Tests"));
        assert!(workflow.contains("cargo lambda build"));
        assert!(workflow.contains("cargo test"));
    }

    #[test]
    fn test_cargo_lambda_script() {
        let harness = LambdaTestHarness::new();
        let annotations = depyler_annotations::LambdaAnnotations {
            event_type: Some(LambdaEventType::S3Event),
            ..Default::default()
        };

        let script = harness
            .generate_cargo_lambda_test_script(&annotations)
            .unwrap();

        assert!(script.contains("cargo lambda build"));
        assert!(script.contains("cargo lambda invoke"));
        assert!(script.contains("s3_object_created"));
    }

    #[test]
    fn test_performance_benchmarks_configuration() {
        let benchmarks = PerformanceBenchmarks {
            max_cold_start_ms: 50,
            max_warm_start_ms: 5,
            max_memory_usage_mb: 32,
            min_throughput_rps: 200,
        };

        let harness = LambdaTestHarness::new().with_benchmarks(benchmarks);

        assert_eq!(harness.performance_benchmarks.max_cold_start_ms, 50);
        assert_eq!(harness.performance_benchmarks.min_throughput_rps, 200);
    }

    #[test]
    fn test_context_conversion() {
        let test_context = TestContext {
            function_name: "test-func".to_string(),
            aws_request_id: "test-123".to_string(),
            timeout_ms: 5000,
            ..TestContext::default()
        };

        // Note: Context conversion would be available when using the actual lambda_runtime crate
        assert_eq!(test_context.aws_request_id, "test-123");
    }
}
