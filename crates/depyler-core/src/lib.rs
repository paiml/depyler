pub mod annotation_aware_type_mapper;
pub mod ast_bridge;
pub mod codegen;
pub mod direct_rules;
pub mod error;
pub mod hir;
pub mod lambda_codegen;
pub mod lambda_errors;
pub mod lambda_inference;
pub mod lambda_optimizer;
pub mod lambda_testing;
pub mod lambda_types;
pub mod optimization;
pub mod rust_gen;
pub mod type_mapper;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepylerPipeline {
    analyzer: CoreAnalyzer,
    transpiler: DirectTranspiler,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifier: Option<PropertyVerifier>,
    #[serde(skip)]
    #[allow(dead_code)]
    mcp_client: LazyMcpClient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAnalyzer {
    pub metrics_enabled: bool,
    pub type_inference_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectTranspiler {
    pub type_mapper: type_mapper::TypeMapper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyVerifier {
    pub enable_quickcheck: bool,
    pub enable_contracts: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LazyMcpClient {
    #[allow(dead_code)]
    endpoint: Option<String>,
}

pub trait AnalyzableStage {
    type Input;
    type Output;
    type Metrics;

    fn execute(&self, input: Self::Input) -> Result<(Self::Output, Self::Metrics)>;
    fn validate(&self, output: &Self::Output) -> ValidationResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Default for DepylerPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl DepylerPipeline {
    pub fn new() -> Self {
        Self {
            analyzer: CoreAnalyzer {
                metrics_enabled: true,
                type_inference_enabled: true,
            },
            transpiler: DirectTranspiler {
                type_mapper: type_mapper::TypeMapper::default(),
            },
            verifier: None,
            mcp_client: LazyMcpClient::default(),
        }
    }

    pub fn with_verification(mut self) -> Self {
        self.verifier = Some(PropertyVerifier {
            enable_quickcheck: true,
            enable_contracts: true,
        });
        self
    }

    pub fn transpile(&self, python_source: &str) -> Result<String> {
        // Parse Python source
        let ast = self.parse_python(python_source)?;

        // Convert to HIR with annotation support
        let mut hir = ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // Apply optimization passes based on annotations
        optimization::optimize_module(&mut hir);

        // Generate Rust code using the unified generation system
        let rust_code = rust_gen::generate_rust_file(&hir, &self.transpiler.type_mapper)?;

        Ok(rust_code)
    }

    pub fn parse_to_hir(&self, source: &str) -> Result<hir::HirModule> {
        let ast = self.parse_python(source)?;
        ast_bridge::AstBridge::new()
            .with_source(source.to_string())
            .python_to_hir(ast)
    }

    pub fn analyze_to_typed_hir(&self, source: &str) -> Result<hir::HirModule> {
        // For now, just return the HIR without type analysis
        // In the future, this would add type inference
        self.parse_to_hir(source)
    }

    pub fn parse_python(&self, source: &str) -> Result<rustpython_ast::Mod> {
        use rustpython_ast::Suite;
        use rustpython_parser::Parse;

        let statements = Suite::parse(source, "<input>")
            .map_err(|e| anyhow::anyhow!("Python parse error: {}", e))?;

        Ok(rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub enable_verification: bool,
    pub enable_metrics: bool,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Default)]
pub enum OptimizationLevel {
    #[default]
    Debug,
    Release,
    Size,
}

impl DepylerPipeline {
    pub fn new_with_config(config: Config) -> Self {
        let mut pipeline = Self::new();
        pipeline.analyzer.metrics_enabled = config.enable_metrics;

        if config.enable_verification {
            pipeline = pipeline.with_verification();
        }

        pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = DepylerPipeline::new();
        assert!(pipeline.analyzer.metrics_enabled);
        assert!(pipeline.analyzer.type_inference_enabled);
        assert!(pipeline.verifier.is_none());
    }

    #[test]
    fn test_pipeline_with_verification() {
        let pipeline = DepylerPipeline::new().with_verification();
        assert!(pipeline.verifier.is_some());
        let verifier = pipeline.verifier.unwrap();
        assert!(verifier.enable_quickcheck);
        assert!(verifier.enable_contracts);
    }

    #[test]
    fn test_config_creation() {
        let config = Config {
            enable_verification: true,
            enable_metrics: false,
            optimization_level: OptimizationLevel::Release,
        };

        let pipeline = DepylerPipeline::new_with_config(config);
        assert!(pipeline.verifier.is_some());
        assert!(!pipeline.analyzer.metrics_enabled);
    }

    #[test]
    fn test_simple_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

        let result = pipeline.transpile(python_code);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("i32"));
    }

    #[test]
    fn test_parse_to_hir() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def test_func(x: int) -> str:
    return "hello"
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name, "test_func");
        assert_eq!(hir.functions[0].params[0].0, "x");
        assert_eq!(hir.functions[0].params[0].1, hir::Type::Int);
        assert_eq!(hir.functions[0].ret_type, hir::Type::String);
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec!["Warning message".to_string()],
        };

        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_invalid_python_syntax() {
        let pipeline = DepylerPipeline::new();
        let invalid_python = "def invalid_syntax(\n    return";

        let result = pipeline.transpile(invalid_python);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyzable_stage_trait() {
        // Test that the trait is properly defined
        struct TestStage;

        impl AnalyzableStage for TestStage {
            type Input = String;
            type Output = String;
            type Metrics = usize;

            fn execute(&self, input: Self::Input) -> Result<(Self::Output, Self::Metrics)> {
                Ok((input.clone(), input.len()))
            }

            fn validate(&self, _output: &Self::Output) -> ValidationResult {
                ValidationResult {
                    is_valid: true,
                    errors: vec![],
                    warnings: vec![],
                }
            }
        }

        let stage = TestStage;
        let (output, metrics) = stage.execute("test".to_string()).unwrap();
        assert_eq!(output, "test");
        assert_eq!(metrics, 4);

        let validation = stage.validate(&output);
        assert!(validation.is_valid);
    }

    #[test]
    fn test_complex_function_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

        let result = pipeline.transpile(python_code);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("fibonacci"));
        assert!(rust_code.contains("if"));
        assert!(rust_code.contains("return"));
    }

    #[test]
    fn test_type_annotations() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
from typing import List, Optional

def process_list(items: List[str]) -> Optional[str]:
    if items:
        return items[0]
    return None
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(
            func.params[0].1,
            hir::Type::List(Box::new(hir::Type::String))
        );
        assert_eq!(
            func.ret_type,
            hir::Type::Optional(Box::new(hir::Type::String))
        );
    }

    #[test]
    fn test_annotation_aware_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: optimization_level = "aggressive"
# @depyler: thread_safety = "required"
# @depyler: bounds_checking = "explicit"
def compute_sum(numbers: List[int]) -> int:
    total = 0
    for num in numbers:
        total += num
    return total
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify annotations were extracted
        assert_eq!(
            func.annotations.optimization_level,
            depyler_annotations::OptimizationLevel::Aggressive
        );
        assert_eq!(
            func.annotations.thread_safety,
            depyler_annotations::ThreadSafety::Required
        );
        assert_eq!(
            func.annotations.bounds_checking,
            depyler_annotations::BoundsChecking::Explicit
        );

        // Verify transpilation works
        let rust_code = pipeline.transpile(python_code).unwrap();
        assert!(rust_code.contains("compute_sum"));
    }

    #[test]
    fn test_string_strategy_annotation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: string_strategy = "zero_copy"
# @depyler: ownership = "borrowed"
def process_string(s: str) -> str:
    return s
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify string strategy was extracted
        assert_eq!(
            func.annotations.string_strategy,
            depyler_annotations::StringStrategy::ZeroCopy
        );
        assert_eq!(
            func.annotations.ownership_model,
            depyler_annotations::OwnershipModel::Borrowed
        );

        // The generated code should use borrowed strings
        let rust_code = pipeline.transpile(python_code).unwrap();
        assert!(rust_code.contains("process_string"));
    }

    #[test]
    fn test_hash_strategy_annotation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: hash_strategy = "fnv"
def create_map() -> Dict[str, int]:
    # TODO: Dictionary subscript assignment not yet supported
    # For now, just test that the annotation is parsed correctly
    return {}
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify hash strategy was extracted
        assert_eq!(
            func.annotations.hash_strategy,
            depyler_annotations::HashStrategy::Fnv
        );
    }
}
