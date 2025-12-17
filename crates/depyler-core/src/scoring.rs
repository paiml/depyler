//! DEPYLER-SCORE-001: 100-Point Single-Shot Compile Score
//!
//! Quantifies transpilation quality across multiple orthogonal dimensions:
//! - A. Compilation Success (40 points)
//! - B. Type Inference Quality (25 points)
//! - C. Test Coverage (15 points)
//! - D. Code Quality (10 points)
//! - E. Semantic Equivalence (10 points)
//!
//! Academic Foundation:
//! - Jia & Harman (2011): Mutation testing for quality assessment
//! - Pierce (2002): Type systems and Hindley-Milner inference
//! - Leroy (2009): CompCert formal verification
//! - Chidamber & Kemerer (1994): CK metrics suite
//! - Sculley et al. (2015): ML feedback loops

use std::collections::HashMap;
use std::path::PathBuf;

/// Scoring mode determines which checks are performed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScoringMode {
    /// Quick mode: <10s, A1-A3 only (filesystem + rustc)
    Quick,
    /// Fast mode: <60s, A + B + D1 (compile + clippy)
    #[default]
    Fast,
    /// Full mode: <5m, all categories (complete verification)
    Full,
}

/// The 100-point score with category breakdown
#[derive(Debug, Clone, Default)]
pub struct SingleShotScore {
    /// Total score (0-100)
    pub total: u8,
    /// Category A: Compilation Success (0-40)
    pub compilation: u8,
    /// Category B: Type Inference Quality (0-25)
    pub type_inference: u8,
    /// Category C: Test Coverage (0-15)
    pub test_coverage: u8,
    /// Category D: Code Quality (0-10)
    pub code_quality: u8,
    /// Category E: Semantic Equivalence (0-10)
    pub semantic_equivalence: u8,
    /// Whether the gateway (A >= 24) passed
    pub gateway_passed: bool,
    /// Which scoring mode was used
    pub mode: ScoringMode,
}

/// Detailed breakdown of each subcategory
#[derive(Debug, Clone, Default)]
pub struct CategoryBreakdown {
    // Category A: Compilation Success
    /// A1: Parse success (0-10)
    pub a1_parse: u8,
    /// A2: Type check success (0-15)
    pub a2_type_check: u8,
    /// A3: Cargo build success (0-15)
    pub a3_cargo_build: u8,

    // Category B: Type Inference Quality
    /// B1: No E0308 errors (0-10)
    pub b1_no_e0308: u8,
    /// B2: No E0599 errors (0-8)
    pub b2_no_e0599: u8,
    /// B3: No E0425 errors (0-7)
    pub b3_no_e0425: u8,

    // Category C: Test Coverage
    /// C1: Doctest pass (0-5)
    pub c1_doctest: u8,
    /// C2: Unit test pass (0-5)
    pub c2_unit_test: u8,
    /// C3: Property test pass (0-5)
    pub c3_property_test: u8,

    // Category D: Code Quality
    /// D1: Clippy clean (0-5)
    pub d1_clippy: u8,
    /// D2: TDG score >= B (0-3)
    pub d2_tdg: u8,
    /// D3: Complexity <= 10 (0-2)
    pub d3_complexity: u8,

    // Category E: Semantic Equivalence
    /// E1: Golden trace match (0-5)
    pub e1_trace_match: u8,
    /// E2: Output equivalence (0-5)
    pub e2_output_equiv: u8,
}

/// Compilation error details for training
#[derive(Debug, Clone)]
pub struct CompilationError {
    /// Error code (e.g., "E0308")
    pub code: String,
    /// Error message
    pub message: String,
    /// File location
    pub location: Option<String>,
    /// Line number
    pub line: Option<u32>,
}

/// A transpiler decision that can be correlated with outcomes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranspilerDecision {
    /// Type inference decision
    TypeInference { variable: String, inferred_type: String },
    /// Method translation decision
    MethodTranslation { python_method: String, rust_method: String },
    /// Import mapping decision
    ImportMapping { python_import: String, rust_import: String },
    /// Fallback to serde_json::Value
    ValueFallback { context: String },
    /// Other decision
    Other(String),
}

/// Single file transpilation result with comprehensive scoring
#[derive(Debug, Clone)]
pub struct SingleShotResult {
    /// Path to the file
    pub file_path: PathBuf,
    /// The computed score
    pub score: SingleShotScore,
    /// Detailed category breakdown
    pub category_breakdown: CategoryBreakdown,
    /// List of compilation errors
    pub error_details: Vec<CompilationError>,
    /// Transpiler decisions made for this file
    pub transpiler_decisions: Vec<TranspilerDecision>,
}

/// Configuration for scoring
#[derive(Debug, Clone)]
pub struct ScoringConfig {
    /// Gateway threshold (default: 0.6 = 60%)
    pub gateway_threshold: f32,
    /// Category weights
    pub weights: CategoryWeights,
    /// Enable semantic check (requires Renacer)
    pub enable_semantic_check: bool,
    /// Send results to oracle for training
    pub oracle_feedback: bool,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            gateway_threshold: 0.6,
            weights: CategoryWeights::default(),
            enable_semantic_check: true,
            oracle_feedback: true,
        }
    }
}

/// Category weights (must sum to 1.0)
#[derive(Debug, Clone)]
pub struct CategoryWeights {
    /// Compilation weight (default: 0.40)
    pub compilation: f32,
    /// Type inference weight (default: 0.25)
    pub type_inference: f32,
    /// Test coverage weight (default: 0.15)
    pub test_coverage: f32,
    /// Code quality weight (default: 0.10)
    pub code_quality: f32,
    /// Semantic equivalence weight (default: 0.10)
    pub semantic_equiv: f32,
}

impl Default for CategoryWeights {
    fn default() -> Self {
        Self {
            compilation: 0.40,
            type_inference: 0.25,
            test_coverage: 0.15,
            code_quality: 0.10,
            semantic_equiv: 0.10,
        }
    }
}

/// Output format for score reports
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// Terminal-friendly table
    #[default]
    Human,
    /// Machine-readable JSON
    Json,
    /// Analytics/ML training Parquet
    Parquet,
    /// Documentation Markdown
    Markdown,
}

/// Corpus-level score report
#[derive(Debug, Clone)]
pub struct CorpusScoreReport {
    /// Individual file results
    pub results: Vec<SingleShotResult>,
    /// Aggregate score
    pub aggregate_score: f32,
    /// Letter grade
    pub grade: Grade,
    /// Category aggregates
    pub category_averages: CategoryBreakdown,
    /// Top blockers (Pareto analysis)
    pub top_blockers: Vec<Blocker>,
}

/// Letter grade mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    APlus,  // 95-100
    A,      // 90-94
    AMinus, // 85-89
    BPlus,  // 80-84
    B,      // 70-79
    C,      // 60-69
    D,      // 50-59
    F,      // 0-49
}

impl Grade {
    /// Convert score to grade
    pub fn from_score(score: f32) -> Self {
        match score as u8 {
            95..=100 => Grade::APlus,
            90..=94 => Grade::A,
            85..=89 => Grade::AMinus,
            80..=84 => Grade::BPlus,
            70..=79 => Grade::B,
            60..=69 => Grade::C,
            50..=59 => Grade::D,
            _ => Grade::F,
        }
    }

    /// Get display string
    pub fn as_str(&self) -> &'static str {
        match self {
            Grade::APlus => "A+",
            Grade::A => "A",
            Grade::AMinus => "A-",
            Grade::BPlus => "B+",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::F => "F",
        }
    }
}

/// A blocker identified by Pareto analysis
#[derive(Debug, Clone)]
pub struct Blocker {
    /// Error pattern or issue
    pub pattern: String,
    /// Number of files affected
    pub affected_files: usize,
    /// Average points lost
    pub avg_points_lost: f32,
}

/// Input data for calculating score breakdown from error analysis
#[derive(Debug, Clone, Default)]
pub struct BreakdownInput<'a> {
    /// Parse succeeded
    pub parse_ok: bool,
    /// Type check succeeded
    pub type_check_ok: bool,
    /// Cargo build succeeded
    pub build_ok: bool,
    /// Compilation errors
    pub errors: &'a [CompilationError],
    /// Doctests passed
    pub doctest_pass: bool,
    /// Unit tests passed
    pub unit_test_pass: bool,
    /// Property tests passed
    pub property_test_pass: bool,
    /// Clippy clean
    pub clippy_clean: bool,
    /// TDG grade B or better
    pub tdg_grade_b_or_better: bool,
    /// Complexity <= 10
    pub complexity_ok: bool,
    /// Trace match (Renacer)
    pub trace_match: bool,
    /// Output equivalence
    pub output_equiv: bool,
}

/// Score calculator
pub struct ScoreCalculator {
    config: ScoringConfig,
}

impl ScoreCalculator {
    /// Create a new score calculator with default config
    pub fn new() -> Self {
        Self {
            config: ScoringConfig::default(),
        }
    }

    /// Create with custom config
    pub fn with_config(config: ScoringConfig) -> Self {
        Self { config }
    }

    /// Calculate score for a single file
    pub fn calculate(&self, breakdown: &CategoryBreakdown, mode: ScoringMode) -> SingleShotScore {
        // Calculate category totals
        let compilation = breakdown.a1_parse + breakdown.a2_type_check + breakdown.a3_cargo_build;
        let type_inference = breakdown.b1_no_e0308 + breakdown.b2_no_e0599 + breakdown.b3_no_e0425;
        let test_coverage = breakdown.c1_doctest + breakdown.c2_unit_test + breakdown.c3_property_test;
        let code_quality = breakdown.d1_clippy + breakdown.d2_tdg + breakdown.d3_complexity;
        let semantic_equivalence = breakdown.e1_trace_match + breakdown.e2_output_equiv;

        // Check gateway (Popper-inspired falsifiability)
        let gateway_threshold = (40.0 * self.config.gateway_threshold) as u8; // 24 by default
        let gateway_passed = compilation >= gateway_threshold;

        // Calculate total (0 if gateway failed)
        let total = if gateway_passed {
            compilation + type_inference + test_coverage + code_quality + semantic_equivalence
        } else {
            0
        };

        SingleShotScore {
            total,
            compilation,
            type_inference,
            test_coverage,
            code_quality,
            semantic_equivalence,
            gateway_passed,
            mode,
        }
    }

    /// Calculate breakdown from error analysis
    pub fn breakdown_from_errors(&self, input: &BreakdownInput<'_>) -> CategoryBreakdown {
        // Count error types
        let e0308_count = input.errors.iter().filter(|e| e.code == "E0308").count();
        let e0599_count = input.errors.iter().filter(|e| e.code == "E0599").count();
        let e0425_count = input.errors.iter().filter(|e| e.code == "E0425").count();
        let total_errors = input.errors.len().max(1); // Avoid division by zero

        // Calculate B subcategories based on error ratios
        let e0308_ratio = e0308_count as f32 / total_errors as f32;
        let e0599_ratio = e0599_count as f32 / total_errors as f32;
        let e0425_ratio = e0425_count as f32 / total_errors as f32;

        CategoryBreakdown {
            // Category A
            a1_parse: if input.parse_ok { 10 } else { 0 },
            a2_type_check: if input.type_check_ok { 15 } else { 0 },
            a3_cargo_build: if input.build_ok { 15 } else { 0 },

            // Category B (inversely proportional to error ratio)
            b1_no_e0308: ((1.0 - e0308_ratio) * 10.0) as u8,
            b2_no_e0599: ((1.0 - e0599_ratio) * 8.0) as u8,
            b3_no_e0425: ((1.0 - e0425_ratio) * 7.0) as u8,

            // Category C
            c1_doctest: if input.doctest_pass { 5 } else { 0 },
            c2_unit_test: if input.unit_test_pass { 5 } else { 0 },
            c3_property_test: if input.property_test_pass { 5 } else { 0 },

            // Category D
            d1_clippy: if input.clippy_clean { 5 } else { 0 },
            d2_tdg: if input.tdg_grade_b_or_better { 3 } else { 0 },
            d3_complexity: if input.complexity_ok { 2 } else { 0 },

            // Category E
            e1_trace_match: if input.trace_match { 5 } else { 0 },
            e2_output_equiv: if input.output_equiv { 5 } else { 0 },
        }
    }

    /// Aggregate corpus results
    pub fn aggregate(&self, results: &[SingleShotResult]) -> CorpusScoreReport {
        if results.is_empty() {
            return CorpusScoreReport {
                results: vec![],
                aggregate_score: 0.0,
                grade: Grade::F,
                category_averages: CategoryBreakdown::default(),
                top_blockers: vec![],
            };
        }

        let n = results.len() as f32;

        // Calculate averages
        let aggregate_score: f32 = results.iter().map(|r| r.score.total as f32).sum::<f32>() / n;

        let category_averages = CategoryBreakdown {
            a1_parse: (results.iter().map(|r| r.category_breakdown.a1_parse as f32).sum::<f32>() / n) as u8,
            a2_type_check: (results.iter().map(|r| r.category_breakdown.a2_type_check as f32).sum::<f32>() / n) as u8,
            a3_cargo_build: (results.iter().map(|r| r.category_breakdown.a3_cargo_build as f32).sum::<f32>() / n) as u8,
            b1_no_e0308: (results.iter().map(|r| r.category_breakdown.b1_no_e0308 as f32).sum::<f32>() / n) as u8,
            b2_no_e0599: (results.iter().map(|r| r.category_breakdown.b2_no_e0599 as f32).sum::<f32>() / n) as u8,
            b3_no_e0425: (results.iter().map(|r| r.category_breakdown.b3_no_e0425 as f32).sum::<f32>() / n) as u8,
            c1_doctest: (results.iter().map(|r| r.category_breakdown.c1_doctest as f32).sum::<f32>() / n) as u8,
            c2_unit_test: (results.iter().map(|r| r.category_breakdown.c2_unit_test as f32).sum::<f32>() / n) as u8,
            c3_property_test: (results.iter().map(|r| r.category_breakdown.c3_property_test as f32).sum::<f32>() / n) as u8,
            d1_clippy: (results.iter().map(|r| r.category_breakdown.d1_clippy as f32).sum::<f32>() / n) as u8,
            d2_tdg: (results.iter().map(|r| r.category_breakdown.d2_tdg as f32).sum::<f32>() / n) as u8,
            d3_complexity: (results.iter().map(|r| r.category_breakdown.d3_complexity as f32).sum::<f32>() / n) as u8,
            e1_trace_match: (results.iter().map(|r| r.category_breakdown.e1_trace_match as f32).sum::<f32>() / n) as u8,
            e2_output_equiv: (results.iter().map(|r| r.category_breakdown.e2_output_equiv as f32).sum::<f32>() / n) as u8,
        };

        // Pareto analysis for blockers
        let mut error_counts: HashMap<String, (usize, f32)> = HashMap::new();
        for result in results {
            for error in &result.error_details {
                let entry = error_counts.entry(error.code.clone()).or_insert((0, 0.0));
                entry.0 += 1;
                entry.1 += 100.0 - result.score.total as f32;
            }
        }

        let mut top_blockers: Vec<Blocker> = error_counts
            .into_iter()
            .map(|(code, (count, total_lost))| Blocker {
                pattern: code,
                affected_files: count,
                avg_points_lost: total_lost / count as f32,
            })
            .collect();

        top_blockers.sort_by(|a, b| b.affected_files.cmp(&a.affected_files));
        top_blockers.truncate(5);

        CorpusScoreReport {
            results: results.to_vec(),
            aggregate_score,
            grade: Grade::from_score(aggregate_score),
            category_averages,
            top_blockers,
        }
    }
}

impl Default for ScoreCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Tarantula fault localization score
#[derive(Debug, Clone)]
pub struct TarantulaScore {
    /// Suspiciousness score (0.0 - 1.0)
    pub suspiciousness: f32,
    /// Number of failed tests with this decision
    pub failed_count: usize,
    /// Number of passed tests with this decision
    pub passed_count: usize,
}

/// Statistics for a transpiler decision
#[derive(Debug, Clone, Default)]
pub struct DecisionStats {
    pub failed_count: usize,
    pub passed_count: usize,
}

impl DecisionStats {
    /// Calculate Tarantula suspiciousness score
    pub fn tarantula_score(&self, total_failed: usize, total_passed: usize) -> TarantulaScore {
        let failed_ratio = if total_failed > 0 {
            self.failed_count as f32 / total_failed as f32
        } else {
            0.0
        };

        let passed_ratio = if total_passed > 0 {
            self.passed_count as f32 / total_passed as f32
        } else {
            0.0
        };

        let suspiciousness = if failed_ratio + passed_ratio > 0.0 {
            failed_ratio / (failed_ratio + passed_ratio)
        } else {
            0.0
        };

        TarantulaScore {
            suspiciousness,
            failed_count: self.failed_count,
            passed_count: self.passed_count,
        }
    }
}

/// Analyze score failures using Tarantula fault localization
pub fn analyze_score_failures(
    results: &[SingleShotResult],
) -> HashMap<TranspilerDecision, TarantulaScore> {
    let mut stats: HashMap<TranspilerDecision, DecisionStats> = HashMap::new();
    let mut total_failed = 0;
    let mut total_passed = 0;

    for result in results {
        let failed = result.score.total < 80;
        if failed {
            total_failed += 1;
        } else {
            total_passed += 1;
        }

        for decision in &result.transpiler_decisions {
            let entry = stats.entry(decision.clone()).or_default();
            if failed {
                entry.failed_count += 1;
            } else {
                entry.passed_count += 1;
            }
        }
    }

    stats
        .into_iter()
        .map(|(d, s)| (d, s.tarantula_score(total_failed, total_passed)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation_perfect() {
        let calculator = ScoreCalculator::new();
        let breakdown = CategoryBreakdown {
            a1_parse: 10,
            a2_type_check: 15,
            a3_cargo_build: 15,
            b1_no_e0308: 10,
            b2_no_e0599: 8,
            b3_no_e0425: 7,
            c1_doctest: 5,
            c2_unit_test: 5,
            c3_property_test: 5,
            d1_clippy: 5,
            d2_tdg: 3,
            d3_complexity: 2,
            e1_trace_match: 5,
            e2_output_equiv: 5,
        };

        let score = calculator.calculate(&breakdown, ScoringMode::Full);

        assert_eq!(score.total, 100);
        assert_eq!(score.compilation, 40);
        assert_eq!(score.type_inference, 25);
        assert_eq!(score.test_coverage, 15);
        assert_eq!(score.code_quality, 10);
        assert_eq!(score.semantic_equivalence, 10);
        assert!(score.gateway_passed);
    }

    #[test]
    fn test_gateway_blocks_when_compilation_fails() {
        let calculator = ScoreCalculator::new();
        let breakdown = CategoryBreakdown {
            a1_parse: 10,
            a2_type_check: 5, // Partial failure
            a3_cargo_build: 0, // Failed
            b1_no_e0308: 10,
            b2_no_e0599: 8,
            b3_no_e0425: 7,
            c1_doctest: 5,
            c2_unit_test: 5,
            c3_property_test: 5,
            d1_clippy: 5,
            d2_tdg: 3,
            d3_complexity: 2,
            e1_trace_match: 5,
            e2_output_equiv: 5,
        };

        let score = calculator.calculate(&breakdown, ScoringMode::Full);

        // Gateway threshold is 24 (60% of 40)
        // compilation = 10 + 5 + 0 = 15 < 24
        assert_eq!(score.compilation, 15);
        assert!(!score.gateway_passed);
        assert_eq!(score.total, 0); // Total is 0 when gateway fails
    }

    #[test]
    fn test_gateway_passes_at_threshold() {
        let calculator = ScoreCalculator::new();
        let breakdown = CategoryBreakdown {
            a1_parse: 10,
            a2_type_check: 14, // Just enough
            a3_cargo_build: 0, // Failed
            ..Default::default()
        };

        let score = calculator.calculate(&breakdown, ScoringMode::Quick);

        // compilation = 10 + 14 + 0 = 24 >= 24
        assert_eq!(score.compilation, 24);
        assert!(score.gateway_passed);
    }

    #[test]
    fn test_grade_mapping() {
        assert_eq!(Grade::from_score(100.0), Grade::APlus);
        assert_eq!(Grade::from_score(95.0), Grade::APlus);
        assert_eq!(Grade::from_score(94.0), Grade::A);
        assert_eq!(Grade::from_score(90.0), Grade::A);
        assert_eq!(Grade::from_score(89.0), Grade::AMinus);
        assert_eq!(Grade::from_score(85.0), Grade::AMinus);
        assert_eq!(Grade::from_score(84.0), Grade::BPlus);
        assert_eq!(Grade::from_score(80.0), Grade::BPlus);
        assert_eq!(Grade::from_score(79.0), Grade::B);
        assert_eq!(Grade::from_score(70.0), Grade::B);
        assert_eq!(Grade::from_score(69.0), Grade::C);
        assert_eq!(Grade::from_score(60.0), Grade::C);
        assert_eq!(Grade::from_score(59.0), Grade::D);
        assert_eq!(Grade::from_score(50.0), Grade::D);
        assert_eq!(Grade::from_score(49.0), Grade::F);
        assert_eq!(Grade::from_score(0.0), Grade::F);
    }

    #[test]
    fn test_breakdown_from_errors() {
        let calculator = ScoreCalculator::new();
        let errors = vec![
            CompilationError {
                code: "E0308".to_string(),
                message: "type mismatch".to_string(),
                location: None,
                line: None,
            },
            CompilationError {
                code: "E0308".to_string(),
                message: "type mismatch 2".to_string(),
                location: None,
                line: None,
            },
            CompilationError {
                code: "E0599".to_string(),
                message: "method not found".to_string(),
                location: None,
                line: None,
            },
        ];

        let breakdown = calculator.breakdown_from_errors(&BreakdownInput {
            parse_ok: true,
            type_check_ok: false,
            build_ok: false,
            errors: &errors,
            doctest_pass: false,
            unit_test_pass: false,
            property_test_pass: false,
            clippy_clean: false,
            tdg_grade_b_or_better: false,
            complexity_ok: true,
            trace_match: false,
            output_equiv: false,
        });

        assert_eq!(breakdown.a1_parse, 10);
        assert_eq!(breakdown.a2_type_check, 0);
        assert_eq!(breakdown.a3_cargo_build, 0);

        // E0308 ratio = 2/3 ≈ 0.67, so b1 = (1 - 0.67) * 10 ≈ 3
        assert!(breakdown.b1_no_e0308 <= 4);

        // E0599 ratio = 1/3 ≈ 0.33, so b2 = (1 - 0.33) * 8 ≈ 5
        assert!(breakdown.b2_no_e0599 >= 5);

        // E0425 ratio = 0/3 = 0, so b3 = (1 - 0) * 7 = 7
        assert_eq!(breakdown.b3_no_e0425, 7);

        assert_eq!(breakdown.d3_complexity, 2);
    }

    #[test]
    fn test_tarantula_score() {
        let stats = DecisionStats {
            failed_count: 8,
            passed_count: 2,
        };

        let tarantula = stats.tarantula_score(10, 10);

        // failed_ratio = 8/10 = 0.8
        // passed_ratio = 2/10 = 0.2
        // suspiciousness = 0.8 / (0.8 + 0.2) = 0.8
        assert!((tarantula.suspiciousness - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_corpus_aggregation() {
        let calculator = ScoreCalculator::new();

        let results = vec![
            SingleShotResult {
                file_path: PathBuf::from("a.py"),
                score: SingleShotScore {
                    total: 80,
                    compilation: 40,
                    type_inference: 20,
                    test_coverage: 10,
                    code_quality: 5,
                    semantic_equivalence: 5,
                    gateway_passed: true,
                    mode: ScoringMode::Fast,
                },
                category_breakdown: CategoryBreakdown::default(),
                error_details: vec![],
                transpiler_decisions: vec![],
            },
            SingleShotResult {
                file_path: PathBuf::from("b.py"),
                score: SingleShotScore {
                    total: 60,
                    compilation: 30,
                    type_inference: 15,
                    test_coverage: 10,
                    code_quality: 3,
                    semantic_equivalence: 2,
                    gateway_passed: true,
                    mode: ScoringMode::Fast,
                },
                category_breakdown: CategoryBreakdown::default(),
                error_details: vec![],
                transpiler_decisions: vec![],
            },
        ];

        let report = calculator.aggregate(&results);

        assert!((report.aggregate_score - 70.0).abs() < 0.01);
        assert_eq!(report.grade, Grade::B);
    }
}
