//! Oracle ROI Metrics - DEPYLER-1301
//!
//! Tracks Oracle suggestion acceptance/rejection rates for continuous
//! improvement of the ML-based error classification system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

/// DEPYLER-1321 (Popper): Global escape rate tracker
/// Tracks DepylerValue usage vs concrete type usage during codegen
///
/// This implements Popper's "falsification criterion" for the type system:
/// If escape_rate > 20%, the type inference is immunizing against falsification
/// rather than genuinely solving the type inference problem.
#[derive(Debug, Default)]
pub struct EscapeRateTracker {
    /// Number of concrete type annotations (i32, String, Vec<T>, HashMap<K,V>, etc.)
    pub concrete_usages: AtomicUsize,
    /// Number of DepylerValue fallback usages
    pub depyler_value_usages: AtomicUsize,
}

impl EscapeRateTracker {
    /// Create a new tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a concrete type usage (not DepylerValue)
    #[inline]
    pub fn record_concrete(&self) {
        self.concrete_usages.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a DepylerValue fallback usage
    #[inline]
    pub fn record_depyler_value(&self) {
        self.depyler_value_usages.fetch_add(1, Ordering::Relaxed);
    }

    /// Get the current escape rate (0.0 - 1.0)
    /// Returns 0.0 if no types have been tracked yet
    pub fn escape_rate(&self) -> f64 {
        let concrete = self.concrete_usages.load(Ordering::Relaxed);
        let dv = self.depyler_value_usages.load(Ordering::Relaxed);
        let total = concrete + dv;
        if total == 0 {
            0.0
        } else {
            dv as f64 / total as f64
        }
    }

    /// Check if escape rate exceeds falsification threshold (20%)
    pub fn is_falsified(&self) -> bool {
        self.escape_rate() > ESCAPE_RATE_FALSIFICATION_THRESHOLD
    }

    /// Get current counts
    pub fn counts(&self) -> (usize, usize) {
        (
            self.concrete_usages.load(Ordering::Relaxed),
            self.depyler_value_usages.load(Ordering::Relaxed),
        )
    }

    /// Reset the tracker
    pub fn reset(&self) {
        self.concrete_usages.store(0, Ordering::Relaxed);
        self.depyler_value_usages.store(0, Ordering::Relaxed);
    }
}

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
    /// DEPYLER-1304: Number of errors with suggested fixes available
    #[serde(default)]
    pub fixes_available: usize,
    /// DEPYLER-1304: Fix availability rate (0.0-1.0)
    #[serde(default)]
    pub fix_availability_rate: f64,
    /// DEPYLER-1321 (Popper): Total concrete type annotations generated
    /// (native Rust types like i32, String, Vec<T>, HashMap<K,V>)
    #[serde(default)]
    pub concrete_type_usages: usize,
    /// DEPYLER-1321 (Popper): Number of DepylerValue fallback usages
    /// (the universal type that absorbs type mismatches)
    #[serde(default)]
    pub depyler_value_usages: usize,
    /// DEPYLER-1321 (Popper): Escape rate = depyler_value_usages / total_type_usages
    /// Per Popper's critique: if escape_rate > 0.20 (20%), type inference is failing
    /// The DepylerValue is a "protective belt" that immunizes against falsification
    #[serde(default)]
    pub escape_rate: f64,
    /// DEPYLER-1321 (Popper): Whether escape rate exceeds falsification threshold (20%)
    /// TRUE means type inference is evading, not solving
    #[serde(default)]
    pub escape_rate_falsified: bool,
}

/// DEPYLER-1321 (Popper): Threshold for escape rate falsification
/// If escape_rate exceeds this, type inference is failing (immunizing against falsification)
pub const ESCAPE_RATE_FALSIFICATION_THRESHOLD: f64 = 0.20;

impl Default for RoiMetrics {
    fn default() -> Self {
        Self {
            high_confidence_errors: 0,
            medium_confidence_errors: 0,
            total_classifiable: 0,
            classifiable_rate: 0.0,
            estimated_savings_cents: 0,
            fixes_available: 0,
            fix_availability_rate: 0.0,
            // DEPYLER-1321 (Popper): Escape rate metrics
            concrete_type_usages: 0,
            depyler_value_usages: 0,
            escape_rate: 0.0,
            escape_rate_falsified: false,
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
        let high_conf = classifications
            .iter()
            .filter(|c| c.confidence > 0.8)
            .count();
        let medium_conf = classifications
            .iter()
            .filter(|c| c.confidence > 0.5 && c.confidence <= 0.8)
            .count();
        let total_classifiable = high_conf + medium_conf;

        // Estimate savings: $0.04 per error avoided (based on LLM API costs)
        let estimated_savings = (high_conf * 4) as u64; // 4 cents per high-conf error

        // DEPYLER-1304: Count errors with suggested fixes available
        let fixes_available = classifications
            .iter()
            .filter(|c| c.suggested_fix.is_some())
            .count();
        let fix_availability_rate = if !classifications.is_empty() {
            fixes_available as f64 / classifications.len() as f64
        } else {
            0.0
        };

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
                fixes_available,
                fix_availability_rate,
                // DEPYLER-1321 (Popper): Default escape rate values
                // Will be populated by from_convergence_with_escape_rate()
                concrete_type_usages: 0,
                depyler_value_usages: 0,
                escape_rate: 0.0,
                escape_rate_falsified: false,
            },
            issue: None,
        }
    }

    /// Create new metrics from convergence state with escape rate tracking
    /// DEPYLER-1321 (Popper): This variant includes escape rate metrics
    pub fn from_convergence_with_escape_rate(
        state: &super::ConvergenceState,
        classifications: &[super::ErrorClassification],
        session_name: &str,
        escape_tracker: Option<&EscapeRateTracker>,
    ) -> Self {
        let mut metrics = Self::from_convergence(state, classifications, session_name);

        // DEPYLER-1321: Populate escape rate metrics if tracker is provided
        if let Some(tracker) = escape_tracker {
            let (concrete, dv) = tracker.counts();
            let escape_rate = tracker.escape_rate();
            let falsified = tracker.is_falsified();

            metrics.roi_metrics.concrete_type_usages = concrete;
            metrics.roi_metrics.depyler_value_usages = dv;
            metrics.roi_metrics.escape_rate = escape_rate;
            metrics.roi_metrics.escape_rate_falsified = falsified;
        }

        metrics
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

        // DEPYLER-1321 (Popper): Include escape rate status in log
        let escape_status = if self.roi_metrics.escape_rate_falsified {
            "⚠️ FALSIFIED (>20%)"
        } else {
            "✅ OK"
        };

        tracing::info!(
            "Wrote Oracle ROI metrics to {}: {} high-conf, {} classifiable ({}%), {} fixes available ({}%), escape_rate: {:.1}% {}",
            path.display(),
            self.roi_metrics.high_confidence_errors,
            self.roi_metrics.total_classifiable,
            (self.roi_metrics.classifiable_rate * 100.0).round() as u32,
            self.roi_metrics.fixes_available,
            (self.roi_metrics.fix_availability_rate * 100.0).round() as u32,
            self.roi_metrics.escape_rate * 100.0,
            escape_status
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
            patch_transpiler: false,
            apr_file: None,
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

    // ========================================================================
    // DEPYLER-1321 (Popper): Escape Rate Tracker Tests
    // ========================================================================

    #[test]
    fn test_escape_rate_tracker_new() {
        let tracker = EscapeRateTracker::new();
        let (concrete, dv) = tracker.counts();
        assert_eq!(concrete, 0);
        assert_eq!(dv, 0);
        assert_eq!(tracker.escape_rate(), 0.0);
        assert!(!tracker.is_falsified());
    }

    #[test]
    fn test_escape_rate_tracker_record_concrete() {
        let tracker = EscapeRateTracker::new();
        tracker.record_concrete();
        tracker.record_concrete();
        tracker.record_concrete();

        let (concrete, dv) = tracker.counts();
        assert_eq!(concrete, 3);
        assert_eq!(dv, 0);
        assert_eq!(tracker.escape_rate(), 0.0); // 0 / 3 = 0
        assert!(!tracker.is_falsified());
    }

    #[test]
    fn test_escape_rate_tracker_record_depyler_value() {
        let tracker = EscapeRateTracker::new();
        tracker.record_depyler_value();
        tracker.record_depyler_value();

        let (concrete, dv) = tracker.counts();
        assert_eq!(concrete, 0);
        assert_eq!(dv, 2);
        assert_eq!(tracker.escape_rate(), 1.0); // 2 / 2 = 1.0 (100% escape!)
        assert!(tracker.is_falsified()); // 100% > 20%
    }

    #[test]
    fn test_escape_rate_tracker_mixed_usage() {
        let tracker = EscapeRateTracker::new();
        // 80 concrete, 20 DepylerValue = 20% escape rate = exactly at threshold
        for _ in 0..80 {
            tracker.record_concrete();
        }
        for _ in 0..20 {
            tracker.record_depyler_value();
        }

        let (concrete, dv) = tracker.counts();
        assert_eq!(concrete, 80);
        assert_eq!(dv, 20);
        assert!((tracker.escape_rate() - 0.20).abs() < 0.001); // 20 / 100 = 0.20
        assert!(!tracker.is_falsified()); // 20% is not > 20%
    }

    #[test]
    fn test_escape_rate_tracker_falsification_threshold() {
        let tracker = EscapeRateTracker::new();
        // 79 concrete, 21 DepylerValue = 21% escape rate = exceeds threshold
        for _ in 0..79 {
            tracker.record_concrete();
        }
        for _ in 0..21 {
            tracker.record_depyler_value();
        }

        assert!(tracker.escape_rate() > ESCAPE_RATE_FALSIFICATION_THRESHOLD);
        assert!(tracker.is_falsified()); // 21% > 20%
    }

    #[test]
    fn test_escape_rate_tracker_reset() {
        let tracker = EscapeRateTracker::new();
        tracker.record_concrete();
        tracker.record_depyler_value();
        tracker.reset();

        let (concrete, dv) = tracker.counts();
        assert_eq!(concrete, 0);
        assert_eq!(dv, 0);
    }

    #[test]
    fn test_roi_metrics_escape_rate_fields() {
        let roi = RoiMetrics::default();
        // DEPYLER-1321: Verify escape rate fields exist and are initialized
        assert_eq!(roi.concrete_type_usages, 0);
        assert_eq!(roi.depyler_value_usages, 0);
        assert_eq!(roi.escape_rate, 0.0);
        assert!(!roi.escape_rate_falsified);
    }

    #[test]
    fn test_roi_metrics_escape_rate_serialization() {
        let roi = RoiMetrics {
            high_confidence_errors: 10,
            medium_confidence_errors: 5,
            total_classifiable: 15,
            classifiable_rate: 0.75,
            estimated_savings_cents: 40,
            fixes_available: 8,
            fix_availability_rate: 0.53,
            // DEPYLER-1321: Escape rate fields
            concrete_type_usages: 80,
            depyler_value_usages: 20,
            escape_rate: 0.20,
            escape_rate_falsified: false,
        };

        let json = serde_json::to_string_pretty(&roi).unwrap();
        assert!(json.contains("concrete_type_usages"));
        assert!(json.contains("depyler_value_usages"));
        assert!(json.contains("escape_rate"));
        assert!(json.contains("escape_rate_falsified"));

        let parsed: RoiMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.concrete_type_usages, 80);
        assert_eq!(parsed.depyler_value_usages, 20);
        assert!((parsed.escape_rate - 0.20).abs() < 0.001);
        assert!(!parsed.escape_rate_falsified);
    }

    #[test]
    fn test_from_convergence_with_escape_rate() {
        let config = test_config();
        let state = super::super::ConvergenceState::new(config);
        let classifications: Vec<super::super::ErrorClassification> = vec![];

        // Create tracker with some usage
        let tracker = EscapeRateTracker::new();
        for _ in 0..70 {
            tracker.record_concrete();
        }
        for _ in 0..30 {
            tracker.record_depyler_value();
        }

        let metrics = OracleRoiMetrics::from_convergence_with_escape_rate(
            &state,
            &classifications,
            "escape-test",
            Some(&tracker),
        );

        assert_eq!(metrics.roi_metrics.concrete_type_usages, 70);
        assert_eq!(metrics.roi_metrics.depyler_value_usages, 30);
        assert!((metrics.roi_metrics.escape_rate - 0.30).abs() < 0.001);
        assert!(metrics.roi_metrics.escape_rate_falsified); // 30% > 20%
    }

    #[test]
    fn test_from_convergence_without_escape_tracker() {
        let config = test_config();
        let state = super::super::ConvergenceState::new(config);
        let classifications: Vec<super::super::ErrorClassification> = vec![];

        let metrics = OracleRoiMetrics::from_convergence_with_escape_rate(
            &state,
            &classifications,
            "no-tracker",
            None,
        );

        // Without tracker, escape rate should be 0
        assert_eq!(metrics.roi_metrics.concrete_type_usages, 0);
        assert_eq!(metrics.roi_metrics.depyler_value_usages, 0);
        assert_eq!(metrics.roi_metrics.escape_rate, 0.0);
        assert!(!metrics.roi_metrics.escape_rate_falsified);
    }
}
