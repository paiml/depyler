//! Kaizen (改善) - Continuous Improvement Tracking
//!
//! Implements Toyota's philosophy that "no process can be considered perfect
//! but can always be improved" (Imai, 1986).
//!
//! Tracks compilation rate improvements across Hunt Mode cycles,
//! detects plateaus, and measures cumulative progress.

use super::hansei::CycleOutcome;
use super::verifier::VerifyResult;

/// Metrics for tracking continuous improvement (Kaizen)
///
/// Each TDD cycle should improve the compilation rate by at least 0.1%.
/// The system tracks both absolute rate and improvement velocity.
#[derive(Debug, Clone, Default)]
pub struct KaizenMetrics {
    /// Current single-shot compilation rate (0.0 - 1.0)
    pub compilation_rate: f64,
    /// Improvement since last cycle
    pub rate_delta: f64,
    /// Cycles since last improvement (for plateau detection)
    pub cycles_since_improvement: u32,
    /// Total fixes applied across all cycles
    pub cumulative_fixes: u32,
    /// Total cycles executed
    pub total_cycles: u32,
    /// Historical compilation rates for trend analysis
    rate_history: Vec<f64>,
}

impl KaizenMetrics {
    /// Create new metrics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Create metrics with initial compilation rate
    pub fn with_initial_rate(rate: f64) -> Self {
        Self {
            compilation_rate: rate,
            rate_history: vec![rate],
            ..Default::default()
        }
    }

    /// Record the outcome of a Hunt Mode cycle
    ///
    /// Updates metrics based on whether the cycle improved compilation rate.
    pub fn record_cycle(&mut self, outcome: &CycleOutcome) {
        self.total_cycles += 1;

        // Check if this cycle resulted in a successful fix
        let fix_applied = matches!(outcome.verify_result, VerifyResult::Success);

        if fix_applied {
            self.cumulative_fixes += 1;
        }

        // Calculate new compilation rate (would come from actual measurement)
        // For now, estimate based on fix success
        let new_rate = if fix_applied {
            // Assume each fix improves rate by ~0.5% on average
            (self.compilation_rate + 0.005).min(1.0)
        } else {
            self.compilation_rate
        };

        self.rate_delta = new_rate - self.compilation_rate;

        if self.rate_delta > 0.001 {
            self.cycles_since_improvement = 0;
        } else {
            self.cycles_since_improvement += 1;
        }

        self.compilation_rate = new_rate;
        self.rate_history.push(new_rate);
    }

    /// Update compilation rate from external measurement
    ///
    /// Called after measuring actual rustc success rate on the corpus.
    pub fn update_rate(&mut self, measured_rate: f64) {
        self.rate_delta = measured_rate - self.compilation_rate;

        if self.rate_delta > 0.001 {
            self.cycles_since_improvement = 0;
        } else {
            self.cycles_since_improvement += 1;
        }

        self.compilation_rate = measured_rate;
        self.rate_history.push(measured_rate);
    }

    /// Check if the system is still making progress
    ///
    /// Toyota Way: Small, incremental improvements compound.
    /// Each TDD cycle should improve rate by at least 0.1%.
    pub fn is_improving(&self) -> bool {
        self.rate_delta > 0.001 || self.cycles_since_improvement < 5
    }

    /// Check if plateau has been reached
    ///
    /// Andon principle: Signal when progress has stalled.
    pub fn is_plateaued(&self, threshold: u32) -> bool {
        self.cycles_since_improvement >= threshold
    }

    /// Calculate improvement velocity (rate change per cycle)
    pub fn improvement_velocity(&self) -> f64 {
        if self.total_cycles == 0 {
            return 0.0;
        }

        // Calculate average improvement per cycle
        if self.rate_history.len() < 2 {
            return 0.0;
        }

        let first = self.rate_history.first().unwrap_or(&0.0);
        let last = self.rate_history.last().unwrap_or(&0.0);
        (last - first) / self.total_cycles as f64
    }

    /// Estimate cycles needed to reach target rate
    pub fn estimate_cycles_to_target(&self, target_rate: f64) -> Option<u32> {
        let remaining = target_rate - self.compilation_rate;

        // Already at or above target
        if remaining <= 0.0 {
            return Some(0);
        }

        let velocity = self.improvement_velocity();

        // Not improving, can't estimate
        if velocity <= 0.0 {
            return None;
        }

        Some((remaining / velocity).ceil() as u32)
    }

    /// Get trend indicator for Andon display
    pub fn trend_indicator(&self) -> TrendIndicator {
        if self.rate_delta > 0.01 {
            TrendIndicator::StronglyImproving
        } else if self.rate_delta > 0.001 {
            TrendIndicator::Improving
        } else if self.rate_delta > -0.001 {
            TrendIndicator::Stable
        } else {
            TrendIndicator::Regressing
        }
    }

    /// Get rate history for analysis
    pub fn history(&self) -> &[f64] {
        &self.rate_history
    }
}

/// Trend indicator for Andon dashboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendIndicator {
    /// Rate increasing by more than 1% per cycle
    StronglyImproving,
    /// Rate increasing (0.1% - 1% per cycle)
    Improving,
    /// Rate stable (within ±0.1%)
    Stable,
    /// Rate decreasing (regression detected)
    Regressing,
}

impl std::fmt::Display for TrendIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            TrendIndicator::StronglyImproving => "↑↑",
            TrendIndicator::Improving => "↑",
            TrendIndicator::Stable => "→",
            TrendIndicator::Regressing => "↓",
        };
        write!(f, "{}", symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaizen_metrics_new() {
        let metrics = KaizenMetrics::new();
        assert_eq!(metrics.compilation_rate, 0.0);
        assert_eq!(metrics.rate_delta, 0.0);
        assert_eq!(metrics.cycles_since_improvement, 0);
        assert_eq!(metrics.cumulative_fixes, 0);
        assert_eq!(metrics.total_cycles, 0);
    }

    #[test]
    fn test_kaizen_metrics_with_initial_rate() {
        let metrics = KaizenMetrics::with_initial_rate(0.236);
        assert!((metrics.compilation_rate - 0.236).abs() < f64::EPSILON);
        assert_eq!(metrics.history().len(), 1);
    }

    #[test]
    fn test_is_improving_when_rate_increasing() {
        let mut metrics = KaizenMetrics::with_initial_rate(0.20);
        metrics.update_rate(0.25);
        assert!(metrics.is_improving());
        assert_eq!(metrics.cycles_since_improvement, 0);
    }

    #[test]
    fn test_is_improving_within_grace_period() {
        let mut metrics = KaizenMetrics::with_initial_rate(0.20);
        // No improvement but within grace period
        for _ in 0..4 {
            metrics.update_rate(0.20);
        }
        assert!(metrics.is_improving()); // Still within 5 cycle grace
    }

    #[test]
    fn test_plateau_detection() {
        let mut metrics = KaizenMetrics::with_initial_rate(0.20);
        for _ in 0..5 {
            metrics.update_rate(0.20); // No improvement
        }
        assert!(metrics.is_plateaued(5));
        assert!(!metrics.is_improving());
    }

    #[test]
    fn test_improvement_velocity() {
        let mut metrics = KaizenMetrics::with_initial_rate(0.20);
        metrics.total_cycles = 10;
        metrics.update_rate(0.30); // 10% improvement over 10 cycles

        let velocity = metrics.improvement_velocity();
        assert!(velocity > 0.0);
    }

    #[test]
    fn test_trend_indicator() {
        let mut metrics = KaizenMetrics::new();

        // Strongly improving
        metrics.rate_delta = 0.02;
        assert_eq!(metrics.trend_indicator(), TrendIndicator::StronglyImproving);

        // Improving
        metrics.rate_delta = 0.005;
        assert_eq!(metrics.trend_indicator(), TrendIndicator::Improving);

        // Stable
        metrics.rate_delta = 0.0005;
        assert_eq!(metrics.trend_indicator(), TrendIndicator::Stable);

        // Regressing
        metrics.rate_delta = -0.005;
        assert_eq!(metrics.trend_indicator(), TrendIndicator::Regressing);
    }

    #[test]
    fn test_estimate_cycles_to_target() {
        let mut metrics = KaizenMetrics::with_initial_rate(0.20);
        metrics.total_cycles = 10;
        metrics.update_rate(0.30);

        // Should estimate cycles needed to reach 80%
        let estimate = metrics.estimate_cycles_to_target(0.80);
        assert!(estimate.is_some());
    }

    #[test]
    fn test_estimate_cycles_already_at_target() {
        let metrics = KaizenMetrics::with_initial_rate(0.85);
        let estimate = metrics.estimate_cycles_to_target(0.80);
        assert_eq!(estimate, Some(0));
    }

    #[test]
    fn test_estimate_cycles_not_improving() {
        let metrics = KaizenMetrics::with_initial_rate(0.20);
        // No cycles run, velocity is 0
        let estimate = metrics.estimate_cycles_to_target(0.80);
        assert!(estimate.is_none());
    }
}
