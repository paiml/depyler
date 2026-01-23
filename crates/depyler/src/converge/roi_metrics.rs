//! Oracle ROI Metrics - DEPYLER-1301
//!
//! Tracks Oracle suggestion acceptance/rejection rates for continuous
//! improvement of the ML-based error classification system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Baseline metrics before Oracle intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    /// Number of files processed
    pub files_processed: usize,
    /// Number of successful transpilations
    pub transpile_success: usize,
    /// Transpilation success rate (0.0-1.0)
    pub transpile_rate: f64,
    /// Total number of compile errors
    pub compile_errors: usize,
}

/// Oracle performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OraclePerformance {
    /// Confidence for type mismatch errors
    pub type_mismatch_confidence: f64,
    /// Confidence for trait bound errors
    pub trait_bound_confidence: f64,
    /// Confidence for borrow checker errors
    pub borrow_checker_confidence: f64,
    /// Confidence for missing import errors
    pub missing_import_confidence: f64,
}

impl Default for OraclePerformance {
    fn default() -> Self {
        Self {
            type_mismatch_confidence: 0.0,
            trait_bound_confidence: 0.0,
            borrow_checker_confidence: 0.0,
            missing_import_confidence: 0.0,
        }
    }
}

/// ROI (Return on Investment) metrics for Oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoiMetrics {
    /// Number of high-confidence errors (>0.8)
    pub high_confidence_errors: usize,
    /// Number of medium-confidence errors (0.5-0.8)
    pub medium_confidence_errors: usize,
    /// Total classifiable errors
    pub total_classifiable: usize,
    /// Rate of classifiable errors (0.0-1.0)
    pub classifiable_rate: f64,
    /// Estimated cost savings in cents
    pub estimated_savings_cents: u64,
}

impl Default for RoiMetrics {
    fn default() -> Self {
        Self {
            high_confidence_errors: 0,
            medium_confidence_errors: 0,
            total_classifiable: 0,
            classifiable_rate: 0.0,
            estimated_savings_cents: 0,
        }
    }
}

/// Complete Oracle ROI metrics document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleRoiMetrics {
    /// Timestamp of metrics collection
    pub timestamp: DateTime<Utc>,
    /// Session identifier
    pub session: String,
    /// Baseline metrics before Oracle
    pub baseline: BaselineMetrics,
    /// Error distribution by code
    pub error_distribution: HashMap<String, usize>,
    /// Oracle performance metrics
    pub oracle_performance: OraclePerformance,
    /// ROI metrics
    pub roi_metrics: RoiMetrics,
    /// Associated issue number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
}

impl OracleRoiMetrics {
    /// Create new metrics from convergence state
    pub fn from_convergence(
        state: &super::ConvergenceState,
        classifications: &[super::ErrorClassification],
        session_name: &str,
    ) -> Self {
        let total_files = state.examples.len();
        let passing_files = state.examples.iter().filter(|e| e.compiles).count();

        // Count total errors
        let total_errors: usize = state.examples.iter().map(|e| e.errors.len()).sum();

        // Build error distribution
        let mut error_distribution: HashMap<String, usize> = HashMap::new();
        for classification in classifications {
            let key = format!(
                "{}_{}",
                classification.error.code,
                classification.subcategory.replace(' ', "_")
            );
            *error_distribution.entry(key).or_insert(0) += 1;
        }

        // Calculate confidence buckets
        let high_conf = classifications.iter().filter(|c| c.confidence > 0.8).count();
        let medium_conf = classifications
            .iter()
            .filter(|c| c.confidence > 0.5 && c.confidence <= 0.8)
            .count();
        let total_classifiable = high_conf + medium_conf;

        // Estimate savings: $0.04 per error avoided (based on LLM API costs)
        let estimated_savings = (high_conf * 4) as u64; // 4 cents per high-conf error

        // Calculate average confidence per category
        let mut category_confidences: HashMap<String, (f64, usize)> = HashMap::new();
        for c in classifications {
            let entry = category_confidences
                .entry(c.subcategory.clone())
                .or_insert((0.0, 0));
            entry.0 += c.confidence;
            entry.1 += 1;
        }

        let avg_confidence = |key: &str| -> f64 {
            category_confidences
                .get(key)
                .map(|(sum, count)| if *count > 0 { sum / *count as f64 } else { 0.0 })
                .unwrap_or(0.0)
        };

        Self {
            timestamp: Utc::now(),
            session: session_name.to_string(),
            baseline: BaselineMetrics {
                files_processed: total_files,
                transpile_success: passing_files,
                transpile_rate: if total_files > 0 {
                    passing_files as f64 / total_files as f64
                } else {
                    0.0
                },
                compile_errors: total_errors,
            },
            error_distribution,
            oracle_performance: OraclePerformance {
                type_mismatch_confidence: avg_confidence("type_inference"),
                trait_bound_confidence: avg_confidence("trait_bound"),
                borrow_checker_confidence: avg_confidence("borrow_checker"),
                missing_import_confidence: avg_confidence("missing_import"),
            },
            roi_metrics: RoiMetrics {
                high_confidence_errors: high_conf,
                medium_confidence_errors: medium_conf,
                total_classifiable,
                classifiable_rate: if !classifications.is_empty() {
                    total_classifiable as f64 / classifications.len() as f64
                } else {
                    0.0
                },
                estimated_savings_cents: estimated_savings,
            },
            issue: None,
        }
    }

    /// Write metrics to the standard docs location
    pub fn write_to_docs(&self) -> anyhow::Result<()> {
        let docs_path = Path::new("docs/oracle_roi_metrics.json");
        self.write_to(docs_path)
    }

    /// Write metrics to a specific path
    pub fn write_to(&self, path: &Path) -> anyhow::Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;

        tracing::info!(
            "Wrote Oracle ROI metrics to {}: {} high-conf, {} classifiable ({}%)",
            path.display(),
            self.roi_metrics.high_confidence_errors,
            self.roi_metrics.total_classifiable,
            (self.roi_metrics.classifiable_rate * 100.0).round() as u32
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_config() -> super::super::ConvergenceConfig {
        super::super::ConvergenceConfig {
            input_dir: PathBuf::from("/tmp"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: super::super::DisplayMode::Silent,
            oracle: false,
            explain: false,
            use_cache: true,
        }
    }

    #[test]
    fn test_oracle_roi_metrics_from_empty_state() {
        let config = test_config();
        let state = super::super::ConvergenceState::new(config);
        let classifications: Vec<super::super::ErrorClassification> = vec![];

        let metrics = OracleRoiMetrics::from_convergence(&state, &classifications, "test-session");

        assert_eq!(metrics.session, "test-session");
        assert_eq!(metrics.baseline.files_processed, 0);
        assert_eq!(metrics.roi_metrics.total_classifiable, 0);
    }

    #[test]
    fn test_oracle_roi_metrics_serialization() {
        let metrics = OracleRoiMetrics {
            timestamp: Utc::now(),
            session: "test".to_string(),
            baseline: BaselineMetrics {
                files_processed: 10,
                transpile_success: 7,
                transpile_rate: 0.7,
                compile_errors: 15,
            },
            error_distribution: HashMap::new(),
            oracle_performance: OraclePerformance::default(),
            roi_metrics: RoiMetrics::default(),
            issue: Some("#172".to_string()),
        };

        let json = serde_json::to_string_pretty(&metrics).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("files_processed"));

        let parsed: OracleRoiMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.baseline.files_processed, 10);
    }

    #[test]
    fn test_oracle_roi_metrics_write_to_temp() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("roi_metrics.json");

        let metrics = OracleRoiMetrics {
            timestamp: Utc::now(),
            session: "test-write".to_string(),
            baseline: BaselineMetrics {
                files_processed: 5,
                transpile_success: 3,
                transpile_rate: 0.6,
                compile_errors: 10,
            },
            error_distribution: HashMap::new(),
            oracle_performance: OraclePerformance::default(),
            roi_metrics: RoiMetrics::default(),
            issue: None,
        };

        metrics.write_to(&path).unwrap();

        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("test-write"));
    }

    #[test]
    fn test_baseline_metrics_default() {
        let baseline = BaselineMetrics {
            files_processed: 0,
            transpile_success: 0,
            transpile_rate: 0.0,
            compile_errors: 0,
        };
        assert_eq!(baseline.files_processed, 0);
    }

    #[test]
    fn test_roi_metrics_default() {
        let roi = RoiMetrics::default();
        assert_eq!(roi.high_confidence_errors, 0);
        assert_eq!(roi.estimated_savings_cents, 0);
    }

    #[test]
    fn test_oracle_performance_default() {
        let perf = OraclePerformance::default();
        assert_eq!(perf.type_mismatch_confidence, 0.0);
    }
}
