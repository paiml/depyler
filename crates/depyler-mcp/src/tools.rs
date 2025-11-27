use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct TranspileRequest {
    pub source: String,
    #[serde(default = "default_mode")]
    pub mode: Mode,
    #[serde(default)]
    pub options: TranspileOptions,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Inline,
    File,
    Project,
}

fn default_mode() -> Mode {
    Mode::Inline
}

#[derive(Debug, Deserialize)]
pub struct TranspileOptions {
    #[serde(default = "default_optimization")]
    pub optimization_level: OptimizationLevel,
    #[serde(default = "default_type_inference")]
    pub type_inference: TypeInference,
    #[serde(default = "default_memory_model")]
    pub memory_model: MemoryModel,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationLevel {
    Size,
    Speed,
    #[default]
    Energy,
}

fn default_optimization() -> OptimizationLevel {
    OptimizationLevel::Energy
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TypeInference {
    #[default]
    Conservative,
    Aggressive,
    MlAssisted,
}

fn default_type_inference() -> TypeInference {
    TypeInference::Conservative
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MemoryModel {
    #[default]
    StackPreferred,
    Arena,
    RcRefcell,
}

fn default_memory_model() -> MemoryModel {
    MemoryModel::StackPreferred
}

impl Default for TranspileOptions {
    fn default() -> Self {
        Self {
            optimization_level: default_optimization(),
            type_inference: default_type_inference(),
            memory_model: default_memory_model(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranspileResponse {
    pub rust_code: String,
    pub metrics: TranspileMetrics,
    pub warnings: Vec<String>,
    pub compilation_command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranspileMetrics {
    pub estimated_energy_reduction: f64,
    pub memory_safety_score: f64,
    pub lines_of_code: usize,
    pub complexity_delta: f64,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub project_path: String,
    #[serde(default = "default_analysis_depth")]
    pub analysis_depth: AnalysisDepth,
    #[serde(default = "default_include_patterns")]
    pub include_patterns: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisDepth {
    Surface,
    Standard,
    Deep,
}

fn default_analysis_depth() -> AnalysisDepth {
    AnalysisDepth::Standard
}

fn default_include_patterns() -> Vec<String> {
    vec!["**/*.py".to_string()]
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub complexity_score: f64,
    pub total_python_loc: usize,
    pub estimated_rust_loc: usize,
    pub migration_strategy: MigrationStrategy,
    pub high_risk_components: Vec<RiskComponent>,
    pub type_inference_coverage: f64,
    pub external_dependencies: Vec<String>,
    pub recommended_rust_crates: Vec<CrateRecommendation>,
    pub estimated_effort_hours: f64,
}

#[derive(Debug, Serialize)]
pub struct MigrationStrategy {
    pub phases: Vec<MigrationPhase>,
    pub recommended_order: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MigrationPhase {
    pub name: String,
    pub description: String,
    pub components: Vec<String>,
    pub estimated_effort: f64,
}

#[derive(Debug, Serialize)]
pub struct RiskComponent {
    pub name: String,
    pub risk_level: RiskLevel,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)] // Variants will be used in future implementations
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize)]
pub struct CrateRecommendation {
    pub python_package: String,
    pub rust_crate: String,
    pub confidence: f64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub python_source: String,
    pub rust_source: String,
    #[serde(default)]
    pub test_cases: Vec<TestCase>,
    #[serde(default = "default_verification_level")]
    pub verification_level: VerificationLevel,
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub input: Value,
    pub expected_output: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationLevel {
    Basic,
    Comprehensive,
    Formal,
}

fn default_verification_level() -> VerificationLevel {
    VerificationLevel::Comprehensive
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub verification_passed: bool,
    pub semantic_equivalence_score: f64,
    pub test_results: TestResults,
    pub safety_guarantees: SafetyGuarantees,
    pub performance_comparison: PerformanceComparison,
    pub optimization_suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TestResults {
    pub passed: usize,
    pub failed: usize,
    pub property_tests_generated: usize,
}

#[derive(Debug, Serialize)]
pub struct SafetyGuarantees {
    pub memory_safe: bool,
    pub thread_safe: bool,
    pub no_undefined_behavior: bool,
}

#[derive(Debug, Serialize)]
pub struct PerformanceComparison {
    pub execution_time_ratio: f64,
    pub memory_usage_ratio: f64,
    pub energy_consumption_ratio: f64,
}
