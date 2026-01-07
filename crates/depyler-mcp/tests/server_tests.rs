//! Comprehensive tests for DepylerMcpServer module
//!
//! Coverage target: server.rs from 67.78% to 95%+

use depyler_core::DepylerPipeline;
use depyler_mcp::tools::*;
use depyler_mcp::{AnalyzeTool, DepylerMcpServer, PmatQualityTool, TranspileTool, VerifyTool};
use pmcp::{RequestHandlerExtra, ToolHandler};
use serde_json::json;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Arc;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

mod server_construction {
    use super::*;

    #[test]
    fn test_server_new() {
        let server = DepylerMcpServer::new();
        // Should create successfully without panicking
        drop(server);
    }

    #[test]
    fn test_server_default_impl() {
        let server = DepylerMcpServer::default();
        drop(server);
    }

    #[tokio::test]
    async fn test_create_server() {
        let result = DepylerMcpServer::create_server().await;
        assert!(result.is_ok());
    }
}

mod transpile_tool {
    use super::*;

    fn create_tool() -> TranspileTool {
        TranspileTool::new(Arc::new(DepylerPipeline::new()))
    }

    fn create_extra() -> RequestHandlerExtra {
        RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new())
    }

    #[tokio::test]
    async fn test_transpile_inline_mode() {
        let tool = create_tool();
        let args = json!({
            "source": "def add(a: int, b: int) -> int:\n    return a + b",
            "mode": "inline"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("rust_code").is_some());
        assert!(response.get("metrics").is_some());
        assert!(response.get("warnings").is_some());
        assert!(response.get("compilation_command").is_some());
    }

    #[tokio::test]
    async fn test_transpile_inline_with_simple_code() {
        let tool = create_tool();
        let args = json!({
            "source": "x = 1",
            "mode": "inline"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transpile_file_mode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "def hello() -> str:").unwrap();
        writeln!(file, "    return 'hello'").unwrap();

        // Change to temp directory for the test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "source": "test.py",
            "mode": "file"
        });

        let result = tool.handle(args, create_extra()).await;

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transpile_project_mode() {
        let temp_dir = TempDir::new().unwrap();
        let main_path = temp_dir.path().join("main.py");
        let mut file = File::create(&main_path).unwrap();
        writeln!(file, "def main():").unwrap();
        writeln!(file, "    print('hello')").unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "source": ".",
            "mode": "project"
        });

        let result = tool.handle(args, create_extra()).await;
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transpile_invalid_request() {
        let tool = create_tool();
        let args = json!({
            "invalid": "request"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transpile_file_outside_project_root() {
        let tool = create_tool();
        let args = json!({
            "source": "/etc/passwd",
            "mode": "file"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transpile_metrics_structure() {
        let tool = create_tool();
        let args = json!({
            "source": "def foo(): pass",
            "mode": "inline"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let metrics = result.get("metrics").unwrap();

        assert!(metrics.get("estimated_energy_reduction").is_some());
        assert!(metrics.get("memory_safety_score").is_some());
        assert!(metrics.get("lines_of_code").is_some());
        assert!(metrics.get("complexity_delta").is_some());
    }
}

mod analyze_tool {
    use super::*;

    fn create_tool() -> AnalyzeTool {
        AnalyzeTool::new(Arc::new(DepylerPipeline::new()))
    }

    fn create_extra() -> RequestHandlerExtra {
        RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new())
    }

    #[tokio::test]
    async fn test_analyze_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        let mut file = File::create(&file_path).unwrap();
        for i in 0..50 {
            writeln!(file, "def func{}(): pass", i).unwrap();
        }

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "project_path": "test.py"
        });

        let result = tool.handle(args, create_extra()).await;
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("complexity_score").is_some());
        assert!(response.get("total_python_loc").is_some());
    }

    #[tokio::test]
    async fn test_analyze_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple Python files
        for i in 0..3 {
            let file_path = temp_dir.path().join(format!("file{}.py", i));
            let mut file = File::create(&file_path).unwrap();
            for j in 0..10 {
                writeln!(file, "x{} = {}", j, j).unwrap();
            }
        }

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "project_path": "."
        });

        let result = tool.handle(args, create_extra()).await;
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());

        let response = result.unwrap();
        let loc = response.get("total_python_loc").unwrap().as_u64().unwrap();
        assert!(loc >= 30); // At least 30 lines across 3 files
    }

    #[tokio::test]
    async fn test_analyze_invalid_path() {
        let tool = create_tool();
        let args = json!({
            "project_path": "/nonexistent/path/to/project"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_analyze_response_structure() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "x = 1").unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "project_path": "test.py"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.get("complexity_score").is_some());
        assert!(result.get("total_python_loc").is_some());
        assert!(result.get("estimated_rust_loc").is_some());
        assert!(result.get("migration_strategy").is_some());
        assert!(result.get("type_inference_coverage").is_some());
        assert!(result.get("external_dependencies").is_some());
        assert!(result.get("recommended_rust_crates").is_some());
        assert!(result.get("estimated_effort_hours").is_some());
    }

    #[tokio::test]
    async fn test_analyze_migration_strategy_structure() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.py");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "x = 1").unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let tool = create_tool();
        let args = json!({
            "project_path": "test.py"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        let strategy = result.get("migration_strategy").unwrap();
        assert!(strategy.get("phases").is_some());
        assert!(strategy.get("recommended_order").is_some());
    }
}

mod verify_tool {
    use super::*;

    fn create_tool() -> VerifyTool {
        VerifyTool::new(Arc::new(DepylerPipeline::new()))
    }

    fn create_extra() -> RequestHandlerExtra {
        RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new())
    }

    #[tokio::test]
    async fn test_verify_valid_rust() {
        let tool = create_tool();
        let args = json!({
            "python_source": "def add(a, b): return a + b",
            "rust_source": "fn add(a: i64, b: i64) -> i64 { a + b }"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response
            .get("verification_passed")
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[tokio::test]
    async fn test_verify_invalid_rust() {
        let tool = create_tool();
        let args = json!({
            "python_source": "def add(a, b): return a + b",
            "rust_source": "fn add(a: i64 b: i64 -> i64 { a + b"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response
            .get("verification_passed")
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[tokio::test]
    async fn test_verify_response_structure() {
        let tool = create_tool();
        let args = json!({
            "python_source": "x = 1",
            "rust_source": "let x = 1;"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();

        assert!(result.get("verification_passed").is_some());
        assert!(result.get("semantic_equivalence_score").is_some());
        assert!(result.get("test_results").is_some());
        assert!(result.get("safety_guarantees").is_some());
        assert!(result.get("performance_comparison").is_some());
        assert!(result.get("optimization_suggestions").is_some());
    }

    #[tokio::test]
    async fn test_verify_safety_guarantees() {
        let tool = create_tool();
        let args = json!({
            "python_source": "x = 1",
            "rust_source": "fn main() {}"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let safety = result.get("safety_guarantees").unwrap();

        assert!(safety.get("memory_safe").is_some());
        assert!(safety.get("thread_safe").is_some());
        assert!(safety.get("no_undefined_behavior").is_some());
    }

    #[tokio::test]
    async fn test_verify_invalid_request() {
        let tool = create_tool();
        let args = json!({
            "invalid": "request"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_err());
    }
}

mod pmat_quality_tool {
    use super::*;

    fn create_tool() -> PmatQualityTool {
        PmatQualityTool::new(Arc::new(DepylerPipeline::new()))
    }

    fn create_extra() -> RequestHandlerExtra {
        RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new())
    }

    #[tokio::test]
    async fn test_quality_valid_code() {
        let tool = create_tool();
        let args = json!({
            "rust_code": "fn main() {}"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("score").is_some());
        assert!(response.get("passes").is_some());
        assert!(response.get("metrics").is_some());
    }

    #[tokio::test]
    async fn test_quality_invalid_syntax() {
        let tool = create_tool();
        let args = json!({
            "rust_code": "fn main( {}"
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        // Score should be lower for invalid code
        let score = response.get("score").unwrap().as_f64().unwrap();
        assert!(score < 100.0);
    }

    #[tokio::test]
    async fn test_quality_with_tests() {
        let tool = create_tool();
        let args = json!({
            "rust_code": r#"
fn main() {}

#[test]
fn test_something() {}
"#
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let metrics = result.get("metrics").unwrap();
        assert!(metrics.get("has_tests").unwrap().as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_quality_with_docs() {
        let tool = create_tool();
        let args = json!({
            "rust_code": r#"
//! Module documentation

/// Function documentation
fn documented() {}
"#
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let metrics = result.get("metrics").unwrap();
        assert!(metrics.get("has_docs").unwrap().as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_quality_large_file_penalty() {
        let tool = create_tool();
        // Generate large code
        let large_code: String = (0..600)
            .map(|i| format!("fn func{}() {{}}\n", i))
            .collect();

        let args = json!({
            "rust_code": large_code
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let suggestions = result.get("suggestions").unwrap();
        assert!(suggestions
            .get("reduce_complexity")
            .unwrap()
            .as_bool()
            .unwrap());
    }

    #[tokio::test]
    async fn test_quality_no_rust_code() {
        let tool = create_tool();
        let args = json!({
            "rust_code": 123  // Not a string
        });

        let result = tool.handle(args, create_extra()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_quality_suggestions_structure() {
        let tool = create_tool();
        let args = json!({
            "rust_code": "fn main() {}"
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        let suggestions = result.get("suggestions").unwrap();

        assert!(suggestions.get("add_tests").is_some());
        assert!(suggestions.get("add_docs").is_some());
        assert!(suggestions.get("reduce_complexity").is_some());
    }

    #[tokio::test]
    async fn test_quality_passes_threshold() {
        let tool = create_tool();

        // Good code with tests and docs should pass
        let args = json!({
            "rust_code": r#"
//! Module docs
/// Doc comment
fn main() {}
#[test]
fn test_main() {}
"#
        });

        let result = tool.handle(args, create_extra()).await.unwrap();
        assert!(result.get("passes").unwrap().as_bool().unwrap());
    }
}

mod tools_types {
    use super::*;

    #[test]
    fn test_mode_enum_serialization() {
        let inline = Mode::Inline;
        let file = Mode::File;
        let project = Mode::Project;

        let json_inline = serde_json::to_string(&inline).unwrap();
        let json_file = serde_json::to_string(&file).unwrap();
        let json_project = serde_json::to_string(&project).unwrap();

        assert!(json_inline.contains("inline"));
        assert!(json_file.contains("file"));
        assert!(json_project.contains("project"));
    }

    #[test]
    fn test_transpile_request_deserialization() {
        let json = json!({
            "source": "x = 1",
            "mode": "inline"
        });

        let request: TranspileRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.source, "x = 1");
        assert!(matches!(request.mode, Mode::Inline));
    }

    #[test]
    fn test_transpile_response_serialization() {
        let response = TranspileResponse {
            rust_code: "let x = 1;".to_string(),
            metrics: TranspileMetrics {
                estimated_energy_reduction: 75.0,
                memory_safety_score: 0.95,
                lines_of_code: 1,
                complexity_delta: -0.1,
            },
            warnings: vec!["warning1".to_string()],
            compilation_command: "rustc -O".to_string(),
        };

        let json = serde_json::to_value(response).unwrap();
        assert_eq!(json["rust_code"], "let x = 1;");
        assert_eq!(json["metrics"]["lines_of_code"], 1);
    }

    #[test]
    fn test_analyze_request_deserialization() {
        let json = json!({
            "project_path": "/path/to/project"
        });

        let request: AnalyzeRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.project_path, "/path/to/project");
    }

    #[test]
    fn test_migration_phase_structure() {
        let phase = MigrationPhase {
            name: "Phase 1".to_string(),
            description: "First phase".to_string(),
            components: vec!["main.py".to_string()],
            estimated_effort: 10.0,
        };

        let json = serde_json::to_value(phase).unwrap();
        assert_eq!(json["name"], "Phase 1");
        assert_eq!(json["estimated_effort"], 10.0);
    }

    #[test]
    fn test_crate_recommendation_structure() {
        let rec = CrateRecommendation {
            python_package: "requests".to_string(),
            rust_crate: "reqwest".to_string(),
            confidence: 0.9,
        };

        let json = serde_json::to_value(rec).unwrap();
        assert_eq!(json["python_package"], "requests");
        assert_eq!(json["rust_crate"], "reqwest");
        assert_eq!(json["confidence"], 0.9);
    }

    #[test]
    fn test_verify_request_deserialization() {
        let json = json!({
            "python_source": "x = 1",
            "rust_source": "let x = 1;"
        });

        let request: VerifyRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.python_source, "x = 1");
        assert_eq!(request.rust_source, "let x = 1;");
    }

    #[test]
    fn test_test_results_structure() {
        let results = TestResults {
            passed: 10,
            failed: 2,
            property_tests_generated: 5,
        };

        let json = serde_json::to_value(results).unwrap();
        assert_eq!(json["passed"], 10);
        assert_eq!(json["failed"], 2);
    }

    #[test]
    fn test_safety_guarantees_structure() {
        let guarantees = SafetyGuarantees {
            memory_safe: true,
            thread_safe: true,
            no_undefined_behavior: true,
        };

        let json = serde_json::to_value(guarantees).unwrap();
        assert!(json["memory_safe"].as_bool().unwrap());
    }

    #[test]
    fn test_performance_comparison_structure() {
        let perf = PerformanceComparison {
            execution_time_ratio: 0.2,
            memory_usage_ratio: 0.15,
            energy_consumption_ratio: 0.25,
        };

        let json = serde_json::to_value(perf).unwrap();
        assert_eq!(json["execution_time_ratio"], 0.2);
    }
}

mod security_tests {
    use super::*;

    #[tokio::test]
    async fn test_path_traversal_blocked() {
        let temp_dir = TempDir::new().unwrap();
        let secret_path = temp_dir.path().join("secret.txt");
        {
            let mut file = File::create(&secret_path).unwrap();
            write!(file, "SECRET_DATA").unwrap();
        }

        let original_dir = std::env::current_dir().unwrap();
        let work_dir = TempDir::new().unwrap();
        std::env::set_current_dir(work_dir.path()).unwrap();

        let tool = TranspileTool::new(Arc::new(DepylerPipeline::new()));
        let args = json!({
            "source": secret_path.to_str().unwrap(),
            "mode": "file"
        });

        let extra = RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new());
        let result = tool.handle(args, extra).await;

        std::env::set_current_dir(original_dir).unwrap();

        // Should be blocked as file is outside project root
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_relative_path_traversal_blocked() {
        let tool = TranspileTool::new(Arc::new(DepylerPipeline::new()));
        let args = json!({
            "source": "../../../etc/passwd",
            "mode": "file"
        });

        let extra = RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new());
        let result = tool.handle(args, extra).await;

        assert!(result.is_err());
    }
}

mod edge_cases {
    use super::*;

    #[tokio::test]
    async fn test_empty_python_source() {
        let tool = TranspileTool::new(Arc::new(DepylerPipeline::new()));
        let args = json!({
            "source": "",
            "mode": "inline"
        });

        let extra = RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new());
        let result = tool.handle(args, extra).await;
        // Should handle gracefully (may succeed or fail, but shouldn't panic)
        drop(result);
    }

    #[tokio::test]
    async fn test_unicode_in_source() {
        let tool = TranspileTool::new(Arc::new(DepylerPipeline::new()));
        let args = json!({
            "source": "# 日本語コメント\nx = '🎉'",
            "mode": "inline"
        });

        let extra = RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new());
        let result = tool.handle(args, extra).await;
        // Should handle unicode gracefully
        drop(result);
    }

    #[tokio::test]
    async fn test_very_long_source() {
        let tool = TranspileTool::new(Arc::new(DepylerPipeline::new()));
        let long_source: String = (0..1000)
            .map(|i| format!("x{} = {}\n", i, i))
            .collect();

        let args = json!({
            "source": long_source,
            "mode": "inline"
        });

        let extra = RequestHandlerExtra::new("test-id".to_string(), CancellationToken::new());
        let result = tool.handle(args, extra).await;
        assert!(result.is_ok());
    }
}
