use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspilationMetrics {
    pub parse_time: Duration,
    pub analysis_time: Duration,
    pub transpilation_time: Duration,
    pub total_time: Duration,
    pub source_size_bytes: usize,
    pub output_size_bytes: usize,
    pub functions_transpiled: usize,
    pub direct_transpilation_rate: f64,
    pub mcp_fallback_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub cyclomatic_distribution: ComplexityDistribution,
    pub cognitive_distribution: ComplexityDistribution,
    pub type_coverage: f64,
    pub panic_free_functions: usize,
    pub terminating_functions: usize,
    pub pure_functions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityDistribution {
    pub low: usize,       // complexity <= 5
    pub medium: usize,    // 5 < complexity <= 10
    pub high: usize,      // 10 < complexity <= 20
    pub very_high: usize, // complexity > 20
}

impl Default for ComplexityDistribution {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplexityDistribution {
    pub fn new() -> Self {
        Self {
            low: 0,
            medium: 0,
            high: 0,
            very_high: 0,
        }
    }

    pub fn add(&mut self, complexity: u32) {
        match complexity {
            0..=5 => self.low += 1,
            6..=10 => self.medium += 1,
            11..=20 => self.high += 1,
            _ => self.very_high += 1,
        }
    }

    pub fn total(&self) -> usize {
        self.low + self.medium + self.high + self.very_high
    }

    pub fn average(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            return 0.0;
        }

        let weighted_sum =
            (self.low * 3) + (self.medium * 8) + (self.high * 15) + (self.very_high * 25);
        weighted_sum as f64 / total as f64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub parsing_throughput_mbps: f64,
    pub hir_generation_throughput_mbps: f64,
    pub transpilation_throughput_mbps: f64,
    pub memory_peak_mb: f64,
}

impl PerformanceProfile {
    pub fn calculate(metrics: &TranspilationMetrics, memory_peak_bytes: usize) -> Self {
        let source_mb = metrics.source_size_bytes as f64 / (1024.0 * 1024.0);

        Self {
            parsing_throughput_mbps: if metrics.parse_time.as_secs_f64() > 0.0 {
                source_mb / metrics.parse_time.as_secs_f64()
            } else {
                0.0
            },
            hir_generation_throughput_mbps: if metrics.analysis_time.as_secs_f64() > 0.0 {
                source_mb / metrics.analysis_time.as_secs_f64()
            } else {
                0.0
            },
            transpilation_throughput_mbps: if metrics.transpilation_time.as_secs_f64() > 0.0 {
                source_mb / metrics.transpilation_time.as_secs_f64()
            } else {
                0.0
            },
            memory_peak_mb: memory_peak_bytes as f64 / (1024.0 * 1024.0),
        }
    }
}
