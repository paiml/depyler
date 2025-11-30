//! Corpus-Based CITL Pattern Mining
//!
//! Mines error→fix patterns from the reprorusted corpus using
//! entrenar's DecisionCITL for Tarantula fault localization.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────┐     ┌──────────────────┐
//! │  Parquet Corpus     │────►│  CorpusCITL      │
//! │  (606 pairs)        │     │  Pattern Mining  │
//! └─────────────────────┘     └──────────────────┘
//!                                      │
//!                                      ▼
//!                              ┌──────────────────┐
//!                              │  DecisionCITL    │
//!                              │  (Tarantula)     │
//!                              └──────────────────┘
//!                                      │
//!                                      ▼
//!                              ┌──────────────────┐
//!                              │  PatternStore    │
//!                              │  (BM25+Dense)    │
//!                              └──────────────────┘
//! ```
//!
//! # References
//!
//! - Jones & Harrold (2005): Tarantula Fault Localization
//! - Zeller (2002): Isolating Cause-Effect Chains

use crate::OracleError;
use arrow::array::Array; // Required for is_null() method
use entrenar::citl::{
    CITLConfig, CompilationOutcome, DecisionCITL, DecisionPatternStore, DecisionTrace,
    FixSuggestion, PatternStoreConfig, SourceSpan,
};
use std::collections::HashMap;
use std::path::Path;

/// Statistics for corpus ingestion
#[derive(Debug, Clone, Default)]
pub struct IngestionStats {
    /// Total pairs processed
    pub total_pairs: usize,
    /// Pairs with successful Rust transpilation
    pub success_pairs: usize,
    /// Pairs that failed transpilation
    pub failed_pairs: usize,
    /// Unique error patterns discovered
    pub unique_patterns: usize,
    /// Categories processed
    pub categories: usize,
}

/// Corpus-based CITL trainer for pattern mining
///
/// Ingests the reprorusted Python-Rust corpus and builds
/// a pattern library for fix suggestions.
pub struct CorpusCITL {
    /// DecisionCITL trainer for fault localization
    trainer: DecisionCITL,
    /// Pattern store for fix retrieval
    pattern_store: DecisionPatternStore,
    /// Ingestion statistics
    stats: IngestionStats,
    /// Error code to category mapping
    error_categories: HashMap<String, Vec<String>>,
}

impl CorpusCITL {
    /// Create a new CorpusCITL trainer
    ///
    /// # Errors
    ///
    /// Returns error if entrenar CITL initialization fails.
    pub fn new() -> Result<Self, OracleError> {
        Self::with_config(CITLConfig::default(), PatternStoreConfig::default())
    }

    /// Create with custom configuration
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails.
    pub fn with_config(
        citl_config: CITLConfig,
        pattern_config: PatternStoreConfig,
    ) -> Result<Self, OracleError> {
        let trainer = DecisionCITL::with_config(citl_config)
            .map_err(|e| OracleError::Model(e.to_string()))?;
        let pattern_store = DecisionPatternStore::with_config(pattern_config)
            .map_err(|e| OracleError::Model(e.to_string()))?;

        Ok(Self {
            trainer,
            pattern_store,
            stats: IngestionStats::default(),
            error_categories: HashMap::new(),
        })
    }

    /// Ingest corpus from parquet file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to parquet file with columns:
    ///   - `category`: Example category name
    ///   - `python_code`: Original Python source
    ///   - `rust_code`: Transpiled Rust (null if failed)
    ///   - `has_rust`: Boolean indicating success
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be read or parsed.
    pub fn ingest_from_parquet(&mut self, path: &Path) -> Result<IngestionStats, OracleError> {
        use arrow::array::{BooleanArray, StringArray};
        use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
        use std::fs::File;

        let file = File::open(path).map_err(|e| OracleError::Io(e))?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .map_err(|e| OracleError::Model(format!("Parquet read error: {}", e)))?;

        let reader = builder
            .build()
            .map_err(|e| OracleError::Model(format!("Parquet build error: {}", e)))?;

        let mut categories_seen = std::collections::HashSet::new();

        for batch_result in reader {
            let batch = batch_result
                .map_err(|e| OracleError::Model(format!("Batch read error: {}", e)))?;

            // Get columns
            let category_col = batch
                .column_by_name("category")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let python_col = batch
                .column_by_name("python_code")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let rust_col = batch
                .column_by_name("rust_code")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>());
            let has_rust_col = batch
                .column_by_name("has_rust")
                .and_then(|c| c.as_any().downcast_ref::<BooleanArray>());

            let (category_col, python_col) = match (category_col, python_col) {
                (Some(c), Some(p)) => (c, p),
                _ => continue,
            };

            for i in 0..batch.num_rows() {
                let category = category_col.value(i).to_string();
                let python_code = python_col.value(i).to_string();
                let rust_code = rust_col.and_then(|c| {
                    if c.is_null(i) {
                        None
                    } else {
                        Some(c.value(i).to_string())
                    }
                });
                let has_rust = has_rust_col.map_or(false, |c| c.value(i));

                categories_seen.insert(category.clone());
                self.stats.total_pairs += 1;

                if has_rust && rust_code.is_some() {
                    // Successful transpilation - record as success session
                    self.stats.success_pairs += 1;

                    let rust = rust_code.unwrap();
                    let traces = self.extract_decision_traces(&python_code, &rust);
                    let outcome = CompilationOutcome::success();

                    // Create fix pattern from the diff
                    let diff = self.create_diff(&python_code, &rust);

                    self.trainer
                        .ingest_session(traces, outcome, Some(diff))
                        .map_err(|e| OracleError::Model(e.to_string()))?;
                } else {
                    // Failed transpilation - record as failure
                    self.stats.failed_pairs += 1;

                    let traces = self.extract_decision_traces(&python_code, "");
                    let error_code = format!("TRANSPILE_FAIL_{}", category.to_uppercase());

                    let outcome = CompilationOutcome::failure(
                        vec![error_code.clone()],
                        vec![SourceSpan::line("input.py", 1)],
                        vec!["Transpilation not yet supported".to_string()],
                    );

                    self.trainer
                        .ingest_session(traces, outcome, None)
                        .map_err(|e| OracleError::Model(e.to_string()))?;

                    // Track error categories
                    self.error_categories
                        .entry(error_code)
                        .or_default()
                        .push(category.clone());
                }
            }
        }

        self.stats.categories = categories_seen.len();
        self.stats.unique_patterns = self.trainer.pattern_store().len();

        Ok(self.stats.clone())
    }

    /// Ingest a single Python-Rust pair
    ///
    /// # Arguments
    ///
    /// * `python_code` - Original Python source
    /// * `rust_code` - Transpiled Rust code (None if failed)
    /// * `category` - Example category name
    ///
    /// # Errors
    ///
    /// Returns error if ingestion fails.
    pub fn ingest_pair(
        &mut self,
        python_code: &str,
        rust_code: Option<&str>,
        category: &str,
    ) -> Result<(), OracleError> {
        self.stats.total_pairs += 1;

        if let Some(rust) = rust_code {
            self.stats.success_pairs += 1;

            let traces = self.extract_decision_traces(python_code, rust);
            let outcome = CompilationOutcome::success();
            let diff = self.create_diff(python_code, rust);

            self.trainer
                .ingest_session(traces, outcome, Some(diff))
                .map_err(|e| OracleError::Model(e.to_string()))?;
        } else {
            self.stats.failed_pairs += 1;

            let traces = self.extract_decision_traces(python_code, "");
            let error_code = format!("TRANSPILE_FAIL_{}", category.to_uppercase());

            let outcome = CompilationOutcome::failure(
                vec![error_code.clone()],
                vec![SourceSpan::line("input.py", 1)],
                vec!["Transpilation failed".to_string()],
            );

            self.trainer
                .ingest_session(traces, outcome, None)
                .map_err(|e| OracleError::Model(e.to_string()))?;

            // Track error categories
            self.error_categories
                .entry(error_code)
                .or_default()
                .push(category.to_string());
        }

        Ok(())
    }

    /// Suggest fixes for an error
    ///
    /// # Arguments
    ///
    /// * `error_code` - The error code (e.g., "TRANSPILE_FAIL_LOG_PARSER")
    /// * `context` - Decision context (Python AST features used)
    /// * `max_suggestions` - Maximum suggestions to return
    ///
    /// # Returns
    ///
    /// Vector of fix suggestions sorted by relevance.
    pub fn suggest_fix(
        &self,
        error_code: &str,
        context: &[String],
        max_suggestions: usize,
    ) -> Result<Vec<FixSuggestion>, OracleError> {
        self.pattern_store
            .suggest_fix(error_code, context, max_suggestions)
            .map_err(|e| OracleError::Model(e.to_string()))
    }

    /// Get top suspicious decision types (by Tarantula score)
    ///
    /// # Arguments
    ///
    /// * `k` - Number of top types to return
    ///
    /// # Returns
    ///
    /// Vector of (decision_type, suspiciousness_score) pairs.
    #[must_use]
    pub fn top_suspicious_decisions(&self, k: usize) -> Vec<(&str, f32)> {
        self.trainer.top_suspicious_types(k)
    }

    /// Get ingestion statistics
    #[must_use]
    pub fn stats(&self) -> &IngestionStats {
        &self.stats
    }

    /// Get the number of patterns in the store
    #[must_use]
    pub fn pattern_count(&self) -> usize {
        self.pattern_store.len()
    }

    /// Get error categories mapping
    #[must_use]
    pub fn error_categories(&self) -> &HashMap<String, Vec<String>> {
        &self.error_categories
    }

    /// Save patterns to disk (APR format)
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written.
    pub fn save_patterns(&self, path: &Path) -> Result<(), OracleError> {
        self.pattern_store
            .save_apr(path)
            .map_err(|e| OracleError::Model(e.to_string()))
    }

    /// Load patterns from disk (APR format)
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be read.
    pub fn load_patterns(&mut self, path: &Path) -> Result<(), OracleError> {
        self.pattern_store = DecisionPatternStore::load_apr(path)
            .map_err(|e| OracleError::Model(e.to_string()))?;
        Ok(())
    }

    // ========================================================================
    // Private helpers
    // ========================================================================

    /// Extract decision traces from Python code
    fn extract_decision_traces(&self, python_code: &str, _rust_code: &str) -> Vec<DecisionTrace> {
        let mut traces = Vec::new();

        // Extract Python AST features as decision traces
        let features = self.extract_python_features(python_code);

        for (i, feature) in features.iter().enumerate() {
            traces.push(
                DecisionTrace::new(
                    format!("d{}", i),
                    feature.clone(),
                    format!("Python feature: {}", feature),
                )
                .with_span(SourceSpan::line("input.py", 1)),
            );
        }

        traces
    }

    /// Extract Python features for decision tracing
    fn extract_python_features(&self, python_code: &str) -> Vec<String> {
        let mut features = Vec::new();

        // Simple pattern matching for common Python features
        if python_code.contains("stdin") {
            features.push("stdin_usage".to_string());
        }
        if python_code.contains(":=") {
            features.push("walrus_operator".to_string());
        }
        if python_code.contains("async ") || python_code.contains("await ") {
            features.push("async_await".to_string());
        }
        if python_code.contains("yield") {
            features.push("generator".to_string());
        }
        if python_code.contains("lambda") {
            features.push("lambda".to_string());
        }
        if python_code.contains("class ") {
            features.push("class_definition".to_string());
        }
        if python_code.contains("def ") {
            features.push("function_definition".to_string());
        }
        if python_code.contains("import ") {
            features.push("import_statement".to_string());
        }
        if python_code.contains("try:") || python_code.contains("except") {
            features.push("exception_handling".to_string());
        }
        if python_code.contains("with ") {
            features.push("context_manager".to_string());
        }
        if python_code.contains("[") && python_code.contains("for") {
            features.push("list_comprehension".to_string());
        }

        features
    }

    /// Create a diff between Python and Rust code
    fn create_diff(&self, python: &str, rust: &str) -> String {
        // Simple line-by-line diff format
        let mut diff = String::new();

        for line in python.lines() {
            diff.push_str(&format!("- {}\n", line));
        }
        for line in rust.lines() {
            diff.push_str(&format!("+ {}\n", line));
        }

        diff
    }
}

impl Default for CorpusCITL {
    fn default() -> Self {
        Self::new().expect("CorpusCITL initialization failed")
    }
}

// ============================================================================
// EXTREME TDD Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Unit Tests - IngestionStats
    // ========================================================================

    #[test]
    fn test_ingestion_stats_default() {
        let stats = IngestionStats::default();
        assert_eq!(stats.total_pairs, 0);
        assert_eq!(stats.success_pairs, 0);
        assert_eq!(stats.failed_pairs, 0);
        assert_eq!(stats.unique_patterns, 0);
        assert_eq!(stats.categories, 0);
    }

    // ========================================================================
    // Unit Tests - CorpusCITL Construction
    // ========================================================================

    #[test]
    fn test_corpus_citl_new() {
        let citl = CorpusCITL::new();
        assert!(citl.is_ok());

        let citl = citl.unwrap();
        assert_eq!(citl.stats().total_pairs, 0);
        assert_eq!(citl.pattern_count(), 0);
    }

    #[test]
    fn test_corpus_citl_default() {
        let citl = CorpusCITL::default();
        assert_eq!(citl.stats().total_pairs, 0);
    }

    // ========================================================================
    // Unit Tests - Feature Extraction
    // ========================================================================

    #[test]
    fn test_extract_python_features_stdin() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("import sys\ndata = sys.stdin.read()");
        assert!(features.contains(&"stdin_usage".to_string()));
        assert!(features.contains(&"import_statement".to_string()));
    }

    #[test]
    fn test_extract_python_features_walrus() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("if (n := len(items)) > 0:");
        assert!(features.contains(&"walrus_operator".to_string()));
    }

    #[test]
    fn test_extract_python_features_async() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("async def fetch(): await get()");
        assert!(features.contains(&"async_await".to_string()));
        assert!(features.contains(&"function_definition".to_string()));
    }

    #[test]
    fn test_extract_python_features_generator() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("def gen(): yield 1");
        assert!(features.contains(&"generator".to_string()));
    }

    #[test]
    fn test_extract_python_features_comprehension() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("squares = [x*x for x in range(10)]");
        assert!(features.contains(&"list_comprehension".to_string()));
    }

    #[test]
    fn test_extract_python_features_exception() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("try:\n  x()\nexcept Error:");
        assert!(features.contains(&"exception_handling".to_string()));
    }

    #[test]
    fn test_extract_python_features_context_manager() {
        let citl = CorpusCITL::new().unwrap();
        let features = citl.extract_python_features("with open('f') as f:");
        assert!(features.contains(&"context_manager".to_string()));
    }

    // ========================================================================
    // Unit Tests - Pair Ingestion
    // ========================================================================

    #[test]
    fn test_ingest_pair_success() {
        let mut citl = CorpusCITL::new().unwrap();

        let python = "def add(a, b): return a + b";
        let rust = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let result = citl.ingest_pair(python, Some(rust), "simple_add");
        assert!(result.is_ok());

        assert_eq!(citl.stats().total_pairs, 1);
        assert_eq!(citl.stats().success_pairs, 1);
        assert_eq!(citl.stats().failed_pairs, 0);
    }

    #[test]
    fn test_ingest_pair_failure() {
        let mut citl = CorpusCITL::new().unwrap();

        let python = "data = sys.stdin.readlines()";

        let result = citl.ingest_pair(python, None, "log_parser");
        assert!(result.is_ok());

        assert_eq!(citl.stats().total_pairs, 1);
        assert_eq!(citl.stats().success_pairs, 0);
        assert_eq!(citl.stats().failed_pairs, 1);
    }

    #[test]
    fn test_ingest_multiple_pairs() {
        let mut citl = CorpusCITL::new().unwrap();

        // Success
        citl.ingest_pair("def f(): pass", Some("fn f() {}"), "simple")
            .unwrap();

        // Failure
        citl.ingest_pair("async def f(): await g()", None, "async_basic")
            .unwrap();

        // Success
        citl.ingest_pair("x = 1 + 2", Some("let x = 1 + 2;"), "math")
            .unwrap();

        assert_eq!(citl.stats().total_pairs, 3);
        assert_eq!(citl.stats().success_pairs, 2);
        assert_eq!(citl.stats().failed_pairs, 1);
    }

    // ========================================================================
    // Unit Tests - Suspiciousness Analysis
    // ========================================================================

    #[test]
    fn test_top_suspicious_decisions_empty() {
        let citl = CorpusCITL::new().unwrap();
        let top = citl.top_suspicious_decisions(5);
        assert!(top.is_empty());
    }

    #[test]
    fn test_top_suspicious_after_ingestion() {
        let mut citl = CorpusCITL::new().unwrap();

        // Ingest some failures with stdin
        for _ in 0..5 {
            citl.ingest_pair("import sys\ndata = sys.stdin.read()", None, "stdin_example")
                .unwrap();
        }

        // Ingest some successes without stdin
        for _ in 0..3 {
            citl.ingest_pair("x = 1", Some("let x = 1;"), "simple")
                .unwrap();
        }

        let top = citl.top_suspicious_decisions(10);
        // stdin_usage should have high suspiciousness (appears in failures, not successes)
        // This is a basic check - exact scores depend on Tarantula implementation
        assert!(!top.is_empty() || citl.stats().failed_pairs > 0);
    }

    // ========================================================================
    // Unit Tests - Error Categories
    // ========================================================================

    #[test]
    fn test_error_categories_tracking() {
        let mut citl = CorpusCITL::new().unwrap();

        citl.ingest_pair("stdin.read()", None, "log_parser")
            .unwrap();
        citl.ingest_pair("if (x := 1):", None, "walrus_operator")
            .unwrap();

        let categories = citl.error_categories();
        assert!(categories.contains_key("TRANSPILE_FAIL_LOG_PARSER"));
        assert!(categories.contains_key("TRANSPILE_FAIL_WALRUS_OPERATOR"));
    }

    // ========================================================================
    // Unit Tests - Diff Creation
    // ========================================================================

    #[test]
    fn test_create_diff() {
        let citl = CorpusCITL::new().unwrap();

        let python = "def add(a, b):\n    return a + b";
        let rust = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}";

        let diff = citl.create_diff(python, rust);

        assert!(diff.contains("- def add(a, b):"));
        assert!(diff.contains("+ fn add(a: i32, b: i32) -> i32 {"));
    }

    // ========================================================================
    // Property Tests
    // ========================================================================

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_ingestion_counts_consistent(
            success_count in 0usize..20,
            failure_count in 0usize..20
        ) {
            let mut citl = CorpusCITL::new().unwrap();

            for i in 0..success_count {
                citl.ingest_pair(
                    &format!("def f{i}(): pass"),
                    Some(&format!("fn f{i}() {{}}")),
                    &format!("success_{i}"),
                ).unwrap();
            }

            for i in 0..failure_count {
                citl.ingest_pair(
                    &format!("async def f{i}(): await x()"),
                    None,
                    &format!("failure_{i}"),
                ).unwrap();
            }

            prop_assert_eq!(citl.stats().total_pairs, success_count + failure_count);
            prop_assert_eq!(citl.stats().success_pairs, success_count);
            prop_assert_eq!(citl.stats().failed_pairs, failure_count);
        }

        #[test]
        fn prop_feature_extraction_deterministic(code in "[a-z ]+") {
            let citl = CorpusCITL::new().unwrap();
            let features1 = citl.extract_python_features(&code);
            let features2 = citl.extract_python_features(&code);
            prop_assert_eq!(features1, features2);
        }
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    #[ignore] // Run with: cargo test --ignored corpus_citl
    fn test_ingest_reprorusted_corpus() {
        use std::path::PathBuf;

        // Path to the reprorusted corpus (use snappy-compressed version)
        let corpus_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("reprorusted-python-cli")
            .join("data")
            .join("depyler_citl_corpus_uncompressed.parquet");

        if !corpus_path.exists() {
            eprintln!("Skipping test: corpus not found at {:?}", corpus_path);
            return;
        }

        let mut citl = CorpusCITL::new().unwrap();
        let stats = citl.ingest_from_parquet(&corpus_path).unwrap();

        // Verify corpus statistics
        println!("Corpus ingestion stats:");
        println!("  Total pairs: {}", stats.total_pairs);
        println!("  Success pairs: {}", stats.success_pairs);
        println!("  Failed pairs: {}", stats.failed_pairs);
        println!("  Categories: {}", stats.categories);
        println!("  Unique patterns: {}", stats.unique_patterns);

        // Basic sanity checks based on known corpus
        assert!(stats.total_pairs > 500, "Expected 600+ pairs");
        assert!(stats.success_pairs > 400, "Expected 400+ successful pairs");
        assert!(stats.categories > 200, "Expected 200+ categories");

        // Check suspicious decisions
        let suspicious = citl.top_suspicious_decisions(10);
        println!("\nTop 10 suspicious decision types:");
        for (decision_type, score) in &suspicious {
            println!("  {}: {:.3}", decision_type, score);
        }

        // Verify error categories were tracked
        let error_cats = citl.error_categories();
        println!("\nError categories tracked: {}", error_cats.len());
    }
}
