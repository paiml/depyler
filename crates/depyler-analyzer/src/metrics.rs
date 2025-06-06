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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_complexity_distribution_new() {
        let dist = ComplexityDistribution::new();
        assert_eq!(dist.low, 0);
        assert_eq!(dist.medium, 0);
        assert_eq!(dist.high, 0);
        assert_eq!(dist.very_high, 0);
    }

    #[test]
    fn test_complexity_distribution_default() {
        let dist = ComplexityDistribution::default();
        assert_eq!(dist.low, 0);
        assert_eq!(dist.medium, 0);
        assert_eq!(dist.high, 0);
        assert_eq!(dist.very_high, 0);
    }

    #[test]
    fn test_complexity_distribution_add() {
        let mut dist = ComplexityDistribution::new();

        // Test low complexity (0-5)
        dist.add(0);
        dist.add(3);
        dist.add(5);
        assert_eq!(dist.low, 3);

        // Test medium complexity (6-10)
        dist.add(6);
        dist.add(8);
        dist.add(10);
        assert_eq!(dist.medium, 3);

        // Test high complexity (11-20)
        dist.add(11);
        dist.add(15);
        dist.add(20);
        assert_eq!(dist.high, 3);

        // Test very high complexity (>20)
        dist.add(21);
        dist.add(30);
        dist.add(100);
        assert_eq!(dist.very_high, 3);
    }

    #[test]
    fn test_complexity_distribution_total() {
        let mut dist = ComplexityDistribution::new();
        assert_eq!(dist.total(), 0);

        dist.add(1);
        dist.add(7);
        dist.add(15);
        dist.add(25);
        assert_eq!(dist.total(), 4);
    }

    #[test]
    fn test_complexity_distribution_average() {
        let mut dist = ComplexityDistribution::new();

        // Empty distribution should return 0
        assert_eq!(dist.average(), 0.0);

        // Add various complexities
        dist.add(3); // low (weighted as 3)
        dist.add(8); // medium (weighted as 8)
        dist.add(15); // high (weighted as 15)
        dist.add(25); // very high (weighted as 25)

        // Expected: (3 + 8 + 15 + 25) / 4 = 12.75
        assert!((dist.average() - 12.75).abs() < 0.01);
    }

    #[test]
    fn test_performance_profile_calculation() {
        let metrics = TranspilationMetrics {
            parse_time: Duration::from_millis(100),
            analysis_time: Duration::from_millis(200),
            transpilation_time: Duration::from_millis(300),
            total_time: Duration::from_millis(600),
            source_size_bytes: 1024 * 1024, // 1 MB
            output_size_bytes: 512 * 1024,  // 0.5 MB
            functions_transpiled: 10,
            direct_transpilation_rate: 0.8,
            mcp_fallback_count: 2,
        };

        let memory_peak_bytes = 2 * 1024 * 1024; // 2 MB

        let profile = PerformanceProfile::calculate(&metrics, memory_peak_bytes);

        // 1 MB / 0.1 seconds = 10 MB/s
        assert!((profile.parsing_throughput_mbps - 10.0).abs() < 0.01);

        // 1 MB / 0.2 seconds = 5 MB/s
        assert!((profile.hir_generation_throughput_mbps - 5.0).abs() < 0.01);

        // 1 MB / 0.3 seconds = 3.33 MB/s
        assert!((profile.transpilation_throughput_mbps - 3.333333333333333).abs() < 0.01);

        // 2 MB peak memory
        assert!((profile.memory_peak_mb - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_performance_profile_zero_time() {
        let metrics = TranspilationMetrics {
            parse_time: Duration::from_millis(0),
            analysis_time: Duration::from_millis(0),
            transpilation_time: Duration::from_millis(0),
            total_time: Duration::from_millis(0),
            source_size_bytes: 1024,
            output_size_bytes: 512,
            functions_transpiled: 1,
            direct_transpilation_rate: 1.0,
            mcp_fallback_count: 0,
        };

        let profile = PerformanceProfile::calculate(&metrics, 1024);

        // All throughputs should be 0 when time is 0
        assert_eq!(profile.parsing_throughput_mbps, 0.0);
        assert_eq!(profile.hir_generation_throughput_mbps, 0.0);
        assert_eq!(profile.transpilation_throughput_mbps, 0.0);
        assert!((profile.memory_peak_mb - 0.0009765625).abs() < 0.001); // 1024 bytes = ~0.001 MB
    }

    #[test]
    fn test_transpilation_metrics_creation() {
        let metrics = TranspilationMetrics {
            parse_time: Duration::from_millis(50),
            analysis_time: Duration::from_millis(100),
            transpilation_time: Duration::from_millis(150),
            total_time: Duration::from_millis(300),
            source_size_bytes: 2048,
            output_size_bytes: 1024,
            functions_transpiled: 5,
            direct_transpilation_rate: 0.6,
            mcp_fallback_count: 2,
        };

        assert_eq!(metrics.parse_time, Duration::from_millis(50));
        assert_eq!(metrics.analysis_time, Duration::from_millis(100));
        assert_eq!(metrics.transpilation_time, Duration::from_millis(150));
        assert_eq!(metrics.total_time, Duration::from_millis(300));
        assert_eq!(metrics.source_size_bytes, 2048);
        assert_eq!(metrics.output_size_bytes, 1024);
        assert_eq!(metrics.functions_transpiled, 5);
        assert_eq!(metrics.direct_transpilation_rate, 0.6);
        assert_eq!(metrics.mcp_fallback_count, 2);
    }

    #[test]
    fn test_quality_metrics_creation() {
        let cyclomatic_dist = ComplexityDistribution {
            low: 5,
            medium: 3,
            high: 2,
            very_high: 1,
        };

        let cognitive_dist = ComplexityDistribution {
            low: 6,
            medium: 2,
            high: 2,
            very_high: 1,
        };

        let quality_metrics = QualityMetrics {
            cyclomatic_distribution: cyclomatic_dist.clone(),
            cognitive_distribution: cognitive_dist.clone(),
            type_coverage: 0.85,
            panic_free_functions: 8,
            terminating_functions: 10,
            pure_functions: 7,
        };

        assert_eq!(quality_metrics.cyclomatic_distribution.low, 5);
        assert_eq!(quality_metrics.cognitive_distribution.low, 6);
        assert_eq!(quality_metrics.type_coverage, 0.85);
        assert_eq!(quality_metrics.panic_free_functions, 8);
        assert_eq!(quality_metrics.terminating_functions, 10);
        assert_eq!(quality_metrics.pure_functions, 7);
    }

    #[test]
    fn test_complexity_distribution_serialization() {
        let dist = ComplexityDistribution {
            low: 10,
            medium: 5,
            high: 2,
            very_high: 1,
        };

        // Test that it can be serialized to JSON
        let json = serde_json::to_string(&dist).unwrap();
        assert!(json.contains("\"low\":10"));
        assert!(json.contains("\"medium\":5"));
        assert!(json.contains("\"high\":2"));
        assert!(json.contains("\"very_high\":1"));

        // Test that it can be deserialized back
        let deserialized: ComplexityDistribution = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.low, 10);
        assert_eq!(deserialized.medium, 5);
        assert_eq!(deserialized.high, 2);
        assert_eq!(deserialized.very_high, 1);
    }

    #[test]
    fn test_weighted_average_calculation() {
        let mut dist = ComplexityDistribution::new();

        // Add multiple of each complexity level
        for _ in 0..2 {
            dist.add(3);
        } // 2 low (weight 3 each)
        for _ in 0..3 {
            dist.add(8);
        } // 3 medium (weight 8 each)
        for _ in 0..1 {
            dist.add(15);
        } // 1 high (weight 15 each)
        for _ in 0..1 {
            dist.add(25);
        } // 1 very high (weight 25 each)

        // Expected: (2*3 + 3*8 + 1*15 + 1*25) / 7 = (6 + 24 + 15 + 25) / 7 = 70/7 = 10
        assert!((dist.average() - 10.0).abs() < 0.01);
    }
}
