//! Depyler Corpus Analysis
//!
//! Deterministic scientific corpus analysis for measuring Python-to-Rust
//! transpilation quality. Implements the specification from
//! `docs/specifications/deterministic-scientific-corpus-report-using-rust-pmat-datascience-stack.md`
//!
//! # Toyota Way Principles
//!
//! - **自働化 (Jidoka)**: Automatic detection of compilation failures
//! - **現地現物 (Genchi Genbutsu)**: Direct analysis of compiler errors
//! - **改善 (Kaizen)**: Iterative PDCA cycles for improvement
//! - **5S**: Clean artifact state before analysis
//!
//! # Architecture
//!
//! ```text
//! Phase 1: Artifact Clearing (cleaner)
//!    ↓
//! Phase 2: Transpilation (transpiler)
//!    ↓
//! Phase 3: Compilation Verification (compiler)
//!    ↓
//! Phase 4: Error Analysis (taxonomy, statistics)
//!    ↓
//! Phase 5: Report Generation (report)
//! ```

pub mod cleaner;
pub mod clustering;
pub mod compiler;
pub mod config;
pub mod graph;
pub mod html_report;
pub mod manifest;
pub mod report;
pub mod semantic;
pub mod statistics;
pub mod taxonomy;
pub mod transpiler;

// Re-exports for convenience
pub use cleaner::ArtifactCleaner;
pub use clustering::{ClusterAnalyzer, ErrorCluster};
pub use compiler::{CompilationResult, CompilationVerifier};
pub use config::CorpusConfig;
pub use graph::{ErrorGraph, GraphAnalyzer};
pub use html_report::HtmlReportGenerator;
pub use manifest::ReportManifest;
pub use report::{CorpusReport, ReportFormat};
pub use semantic::{PythonDomain, SemanticClassification, SemanticClassifier};
pub use statistics::StatisticalAnalysis;
pub use taxonomy::{ErrorCategory, ErrorTaxonomy, RustError};
pub use transpiler::{TranspilationResult, TranspileRunner};

/// The main corpus analyzer that orchestrates all phases.
///
/// # Example
///
/// ```no_run
/// use depyler_corpus::{CorpusAnalyzer, CorpusConfig};
/// use std::path::PathBuf;
///
/// let config = CorpusConfig::default()
///     .with_corpus_path(PathBuf::from("../reprorusted-python-cli"));
///
/// let analyzer = CorpusAnalyzer::new(config);
/// let report = analyzer.analyze().expect("Analysis failed");
///
/// println!("Single-shot rate: {:.1}%", report.single_shot_rate());
/// ```
pub struct CorpusAnalyzer {
    config: CorpusConfig,
}

impl CorpusAnalyzer {
    /// Create a new corpus analyzer with the given configuration.
    pub fn new(config: CorpusConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the configuration.
    pub fn config(&self) -> &CorpusConfig {
        &self.config
    }

    /// Run the full corpus analysis pipeline.
    ///
    /// # Phases
    ///
    /// 1. **Clean**: Remove all generated artifacts (5S methodology)
    /// 2. **Transpile**: Convert all Python files to Rust
    /// 3. **Compile**: Verify each generated Rust file compiles
    /// 4. **Analyze**: Classify and analyze compilation errors
    /// 5. **Report**: Generate deterministic report
    ///
    /// # Errors
    ///
    /// Returns an error if any phase fails critically.
    pub fn analyze(&self) -> anyhow::Result<CorpusReport> {
        // Phase 1: Clean artifacts
        if !self.config.skip_clean {
            let cleaner = ArtifactCleaner::new(&self.config.corpus_path);
            cleaner.clean()?;
        }

        // Phase 2: Transpile
        let transpiler = TranspileRunner::new(&self.config);
        let transpile_results = transpiler.run()?;

        // Phase 3: Compile
        let verifier = CompilationVerifier::new(&self.config);
        let compile_results = verifier.verify(&transpile_results)?;

        // Phase 4: Analyze
        let taxonomy = ErrorTaxonomy::classify(&compile_results);
        let statistics = StatisticalAnalysis::compute(&compile_results, &taxonomy);

        // Phase 5: Generate report
        let report = CorpusReport::new(
            &self.config,
            transpile_results,
            compile_results,
            taxonomy,
            statistics,
        );

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use taxonomy::BlockerPriority;

    #[test]
    fn test_corpus_analyzer_creation() {
        let config = CorpusConfig::default();
        let analyzer = CorpusAnalyzer::new(config);
        assert!(!analyzer.config.skip_clean);
    }

    #[test]
    fn test_corpus_analyzer_config_accessor() {
        let config = CorpusConfig {
            skip_clean: true,
            ..Default::default()
        };
        let analyzer = CorpusAnalyzer::new(config);
        assert!(analyzer.config().skip_clean);
    }

    #[test]
    fn test_corpus_config_with_corpus_path() {
        let config = CorpusConfig::default().with_corpus_path(PathBuf::from("/tmp/test"));
        assert_eq!(config.corpus_path, PathBuf::from("/tmp/test"));
    }

    #[test]
    fn test_corpus_config_skip_clean() {
        let config = CorpusConfig {
            skip_clean: true,
            ..Default::default()
        };
        assert!(config.skip_clean);
    }

    #[test]
    fn test_error_category_all_variants() {
        assert_eq!(
            ErrorCategory::from_error_code("E0308"),
            ErrorCategory::TypeMismatch
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0412"),
            ErrorCategory::UndefinedType
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0425"),
            ErrorCategory::UndefinedValue
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0282"),
            ErrorCategory::TypeAnnotation
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0277"),
            ErrorCategory::TraitBound
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0502"),
            ErrorCategory::BorrowCheck
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0503"),
            ErrorCategory::BorrowCheck
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0505"),
            ErrorCategory::BorrowCheck
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0106"),
            ErrorCategory::Lifetime
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0621"),
            ErrorCategory::Lifetime
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0061"),
            ErrorCategory::Syntax
        );
        assert_eq!(
            ErrorCategory::from_error_code("E0433"),
            ErrorCategory::Syntax
        );
        assert_eq!(
            ErrorCategory::from_error_code("E9999"),
            ErrorCategory::Other
        );
    }

    #[test]
    fn test_error_category_descriptions() {
        assert!(!ErrorCategory::TypeMismatch.description().is_empty());
        assert!(!ErrorCategory::UndefinedType.description().is_empty());
        assert!(!ErrorCategory::UndefinedValue.description().is_empty());
        assert!(!ErrorCategory::TypeAnnotation.description().is_empty());
        assert!(!ErrorCategory::TraitBound.description().is_empty());
        assert!(!ErrorCategory::BorrowCheck.description().is_empty());
        assert!(!ErrorCategory::Lifetime.description().is_empty());
        assert!(!ErrorCategory::Syntax.description().is_empty());
        assert!(!ErrorCategory::Other.description().is_empty());
    }

    #[test]
    fn test_rust_error_parse_all_categories() {
        let e0308 = RustError::parse("error[E0308]: type mismatch").unwrap();
        assert_eq!(e0308.category, ErrorCategory::TypeMismatch);

        let e0277 = RustError::parse("error[E0277]: trait not satisfied").unwrap();
        assert_eq!(e0277.category, ErrorCategory::TraitBound);
    }

    #[test]
    fn test_blocker_priority_edge_cases() {
        assert_eq!(
            BlockerPriority::from_frequency(0, 0),
            BlockerPriority::P3Low
        );
        assert_eq!(
            BlockerPriority::from_frequency(100, 100),
            BlockerPriority::P0Critical
        );
        assert_eq!(
            BlockerPriority::from_frequency(11, 100),
            BlockerPriority::P1High
        );
        assert_eq!(
            BlockerPriority::from_frequency(6, 100),
            BlockerPriority::P2Medium
        );
    }

    #[test]
    fn test_semantic_classifier_reexport() {
        let _classifier = SemanticClassifier::default();
    }

    #[test]
    fn test_artifact_cleaner_new() {
        let cleaner = ArtifactCleaner::new(&PathBuf::from("/tmp"));
        // Just verify it creates without panic
        let _ = cleaner;
    }
}
