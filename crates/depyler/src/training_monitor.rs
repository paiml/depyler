//! Training Monitor Integration
//!
//! Integrates entrenar's real-time monitoring with depyler's overnight improvement loop.
//! Implements Toyota Way principles for quality assurance during transpilation training.

use entrenar::monitor::{
    AndonSystem, DriftDetector, HanseiAnalyzer, Metric, MetricsCollector,
};
use std::time::Instant;

/// Training monitor for overnight improvement runs
pub struct TrainingMonitor {
    collector: MetricsCollector,
    drift_detector: DriftDetector,
    andon: AndonSystem,
    start_time: Instant,
    last_compile_rate: f64,
}

impl TrainingMonitor {
    /// Create a new training monitor
    pub fn new() -> Self {
        Self {
            collector: MetricsCollector::new(),
            drift_detector: DriftDetector::new(10), // 10-epoch window
            andon: AndonSystem::new(),
            start_time: Instant::now(),
            last_compile_rate: 0.0,
        }
    }

    /// Record metrics for an epoch
    pub fn record_epoch(
        &mut self,
        epoch: usize,
        transpile_ok: usize,
        compile_ok: usize,
        total_files: usize,
        error_count: usize,
    ) {
        let transpile_rate = if total_files > 0 {
            transpile_ok as f64 / total_files as f64
        } else {
            0.0
        };

        let compile_rate = if total_files > 0 {
            compile_ok as f64 / total_files as f64
        } else {
            0.0
        };

        // Record metrics using entrenar's Metric::Custom
        self.collector
            .record(Metric::Custom("epoch".into()), epoch as f64);
        self.collector
            .record(Metric::Custom("transpile_rate".into()), transpile_rate);
        self.collector
            .record(Metric::Custom("compile_rate".into()), compile_rate);
        self.collector
            .record(Metric::Custom("error_count".into()), error_count as f64);
        self.collector
            .record(Metric::Custom("transpile_ok".into()), transpile_ok as f64);
        self.collector
            .record(Metric::Custom("compile_ok".into()), compile_ok as f64);

        // Check for drift (sudden drop in compile rate)
        let drift_status = self.drift_detector.check(compile_rate);
        if let entrenar::monitor::DriftStatus::Drift(z_score) = drift_status {
            self.andon.warning(format!(
                "Compile rate drift detected: {:.1}% (z={:.2})",
                compile_rate * 100.0,
                z_score
            ));
        }

        // Check for regression (compile rate dropping)
        if compile_rate < self.last_compile_rate * 0.9 && self.last_compile_rate > 0.0 {
            self.andon.warning(format!(
                "Compile rate regression: {:.1}% -> {:.1}%",
                self.last_compile_rate * 100.0,
                compile_rate * 100.0
            ));
        }

        self.last_compile_rate = compile_rate;
    }

    /// Record an error pattern
    pub fn record_error(&mut self, error_code: &str) {
        self.collector
            .record(Metric::Custom(format!("error_{}", error_code)), 1.0);
    }

    /// Check if training should stop
    pub fn should_stop(&self) -> bool {
        self.andon.should_stop()
    }

    /// Get all alerts
    pub fn get_alerts(&self) -> Vec<String> {
        self.andon
            .history()
            .iter()
            .map(|a| format!("[{}] {}", a.level.as_str(), a.message))
            .collect()
    }

    /// Generate Hansei (post-training) report
    pub fn generate_report(&self, training_id: &str) -> String {
        let duration = self.start_time.elapsed().as_secs_f64();
        let analyzer = HanseiAnalyzer::new();
        let report = analyzer.analyze(training_id, &self.collector, duration);
        analyzer.format_report(&report)
    }

    /// Get metrics summary as JSON
    pub fn summary_json(&self) -> Result<String, serde_json::Error> {
        // Convert HashMap<Metric, MetricStats> to HashMap<String, MetricStats> for JSON serialization
        let summary: std::collections::HashMap<String, _> = self
            .collector
            .summary()
            .into_iter()
            .map(|(k, v)| (k.as_str().to_string(), v))
            .collect();
        serde_json::to_string_pretty(&summary)
    }

    /// Get duration in seconds
    pub fn duration_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}

impl Default for TrainingMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_monitor_new() {
        let monitor = TrainingMonitor::new();
        assert!(!monitor.should_stop());
    }

    #[test]
    fn test_record_epoch() {
        let mut monitor = TrainingMonitor::new();
        monitor.record_epoch(1, 290, 280, 307, 27);

        let json = monitor.summary_json().unwrap();
        assert!(json.contains("compile_rate"));
        assert!(json.contains("transpile_rate"));
    }

    #[test]
    fn test_regression_detection() {
        let mut monitor = TrainingMonitor::new();

        // First epoch: 90% compile rate
        monitor.record_epoch(1, 307, 276, 307, 31);

        // Second epoch: 50% compile rate (>10% drop triggers warning)
        monitor.record_epoch(2, 307, 153, 307, 154);

        let alerts = monitor.get_alerts();
        assert!(
            alerts.iter().any(|a| a.contains("regression")),
            "Expected regression alert, got: {:?}",
            alerts
        );
    }

    #[test]
    fn test_generate_report() {
        let mut monitor = TrainingMonitor::new();
        monitor.record_epoch(1, 290, 280, 307, 27);
        monitor.record_epoch(2, 295, 285, 307, 22);

        let report = monitor.generate_report("test-run");
        assert!(report.contains("HANSEI POST-TRAINING REPORT"));
        assert!(report.contains("test-run"));
    }
}
