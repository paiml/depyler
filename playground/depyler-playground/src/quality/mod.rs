use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Configuration-driven quality metrics following PMAT principles
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PmatConfiguration {
    /// Target metrics derived from empirical analysis
    pub targets: QualityTargets,
    /// Non-linear scoring functions
    pub scoring: ScoringFunctions,
    /// Continuous improvement thresholds
    pub kaizen_thresholds: KaizenThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualityTargets {
    /// Time to First Meaningful Paint target (P50)
    pub ttfmp_p50_ms: f64,
    /// Time to Interactive target (P90)
    pub tti_p90_ms: f64,
    /// WASM size budget (gzipped)
    pub wasm_size_budget_kb: f64,
    /// Transpilation latency targets by complexity
    pub transpile_targets: TranspilationTargets,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranspilationTargets {
    /// Simple functions (<10 lines)
    pub simple_p95_ms: f64,
    /// Medium complexity (10-50 lines)
    pub medium_p95_ms: f64,
    /// Complex functions (>50 lines)
    pub complex_p95_ms: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KaizenThresholds {
    /// Minimum improvement to trigger kaizen event
    pub min_improvement_percent: f64,
    /// Maximum regression allowed before alert
    pub max_regression_percent: f64,
    /// Number of samples needed for trend analysis
    pub trend_sample_size: usize,
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidTarget(String),
    InvalidRange(String),
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidTarget(msg) => write!(f, "Invalid target: {}", msg),
            ConfigError::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl PmatConfiguration {
    /// Load from embedded configuration with validation
    pub fn load() -> Result<Self, ConfigError> {
        const CONFIG_TOML: &str = include_str!("../config/pmat.toml");
        let config: Self =
            toml::from_str(CONFIG_TOML).map_err(|e| ConfigError::ParseError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Create default configuration for playground
    pub fn default_playground() -> Self {
        Self {
            targets: QualityTargets {
                ttfmp_p50_ms: 800.0,
                tti_p90_ms: 2000.0,
                wasm_size_budget_kb: 1500.0,
                transpile_targets: TranspilationTargets {
                    simple_p95_ms: 50.0,
                    medium_p95_ms: 200.0,
                    complex_p95_ms: 1000.0,
                },
            },
            scoring: ScoringFunctions {
                sigmoid_steepness: 3.0,
                exponential_decay_rate: 0.5,
            },
            kaizen_thresholds: KaizenThresholds {
                min_improvement_percent: 5.0,
                max_regression_percent: 10.0,
                trend_sample_size: 10,
            },
        }
    }

    /// Validate configuration against invariants
    fn validate(&self) -> Result<(), ConfigError> {
        if self.targets.ttfmp_p50_ms <= 0.0 {
            return Err(ConfigError::InvalidTarget(
                "ttfmp_p50_ms must be positive".to_string(),
            ));
        }
        if self.targets.tti_p90_ms <= self.targets.ttfmp_p50_ms {
            return Err(ConfigError::InvalidRange(
                "tti_p90_ms must be greater than ttfmp_p50_ms".to_string(),
            ));
        }
        if self.targets.wasm_size_budget_kb <= 0.0 {
            return Err(ConfigError::InvalidTarget(
                "wasm_size_budget_kb must be positive".to_string(),
            ));
        }
        if self.scoring.sigmoid_steepness <= 0.0 {
            return Err(ConfigError::InvalidTarget(
                "sigmoid_steepness must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

/// Actual runtime metrics collected
#[derive(Debug, Clone)]
pub struct PlaygroundMetrics {
    pub page_load: PageLoadMetrics,
    pub transpilation: TranspilationMetrics,
    pub execution: ExecutionMetrics,
    pub quality_events: Vec<QualityEvent>,
}

#[derive(Debug, Clone)]
pub struct PageLoadMetrics {
    pub ttfmp_ms: f64,
    pub tti_ms: f64,
    pub wasm_load_ms: f64,
    pub wasm_size_kb: f64,
}

#[derive(Debug, Clone)]
pub struct TranspilationMetrics {
    pub latency_p95_ms: f64,
    pub complexity_bucket: ComplexityBucket,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone)]
pub enum ComplexityBucket {
    Simple,
    Medium,
    Complex,
}

#[derive(Debug, Clone)]
pub struct ExecutionMetrics {
    pub rust_execution_ms: f64,
    pub python_execution_ms: f64,
    pub energy_savings_percent: f64,
    pub memory_usage_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityEvent {
    pub timestamp: SystemTime,
    pub event_type: QualityEventType,
    pub severity: QualityEventSeverity,
    pub message: String,
    pub metrics_snapshot: Option<PmatScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityEventType {
    PerformanceRegression,
    PerformanceImprovement,
    ErrorThresholdExceeded,
    CacheEfficiencyDrop,
    EnergyEfficiencyImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityEventSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmatScore {
    pub productivity: f64,
    pub maintainability: f64,
    pub accessibility: f64,
    pub testability: f64,
    pub tdg: f64,
    pub timestamp: SystemTime,
}

impl PlaygroundMetrics {
    /// Calculate PMAT score using configuration
    pub fn calculate_pmat(&self, config: &PmatConfiguration) -> PmatScore {
        let productivity = self.calculate_productivity(&config.targets, &config.scoring);
        let maintainability = self.calculate_maintainability();
        let accessibility = self.calculate_accessibility(&config.targets);
        let testability = self.calculate_testability();

        PmatScore {
            productivity,
            maintainability,
            accessibility,
            testability,
            tdg: (productivity + maintainability + accessibility + testability) / 4.0,
            timestamp: SystemTime::now(),
        }
    }

    fn calculate_productivity(&self, targets: &QualityTargets, scoring: &ScoringFunctions) -> f64 {
        // Non-linear scoring based on empirical distribution
        let load_score = scoring.exponential_decay(
            self.page_load.tti_ms,
            targets.tti_p90_ms,
            0.5, // decay rate
        );

        let transpile_score = match self.transpilation.complexity_bucket {
            ComplexityBucket::Simple => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.simple_p95_ms,
            ),
            ComplexityBucket::Medium => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.medium_p95_ms,
            ),
            ComplexityBucket::Complex => scoring.sigmoid(
                self.transpilation.latency_p95_ms,
                targets.transpile_targets.complex_p95_ms,
            ),
        };

        // Weighted combination based on user impact
        0.3 * load_score + 0.7 * transpile_score
    }

    fn calculate_maintainability(&self) -> f64 {
        // Based on cache hit rate and error rate
        let cache_score = self.transpilation.cache_hit_rate;
        let error_score = 1.0 - self.transpilation.error_rate.min(1.0);

        (cache_score + error_score) / 2.0
    }

    fn calculate_accessibility(&self, targets: &QualityTargets) -> f64 {
        // Based on load time and WASM size
        let load_accessibility = if self.page_load.ttfmp_ms <= targets.ttfmp_p50_ms {
            1.0
        } else {
            targets.ttfmp_p50_ms / self.page_load.ttfmp_ms
        };

        let size_accessibility = if self.page_load.wasm_size_kb <= targets.wasm_size_budget_kb {
            1.0
        } else {
            targets.wasm_size_budget_kb / self.page_load.wasm_size_kb
        };

        (load_accessibility + size_accessibility) / 2.0
    }

    fn calculate_testability(&self) -> f64 {
        // Based on energy efficiency and execution performance
        let energy_score = self.execution.energy_savings_percent / 100.0;
        let performance_ratio = if self.execution.python_execution_ms > 0.0 {
            self.execution.rust_execution_ms / self.execution.python_execution_ms
        } else {
            1.0
        };

        // Better testability when Rust is faster and more energy efficient
        let speed_score = if performance_ratio < 1.0 {
            1.0 - performance_ratio
        } else {
            0.5
        };

        (energy_score + speed_score) / 2.0
    }
}

/// Non-linear scoring functions based on empirical data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScoringFunctions {
    pub sigmoid_steepness: f64,
    pub exponential_decay_rate: f64,
}

impl ScoringFunctions {
    /// Sigmoid function for smooth transitions
    pub fn sigmoid(&self, actual: f64, target: f64) -> f64 {
        let x = (target - actual) / target;
        1.0 / (1.0 + (-self.sigmoid_steepness * x).exp())
    }

    /// Exponential decay for time-based metrics
    pub fn exponential_decay(&self, actual: f64, target: f64, rate: f64) -> f64 {
        if actual <= target {
            1.0
        } else {
            (-rate * (actual - target) / target).exp()
        }
    }
}

/// Quality monitoring and trend analysis
pub struct QualityMonitor {
    config: PmatConfiguration,
    history: Vec<PmatScore>,
    events: Vec<QualityEvent>,
}

impl QualityMonitor {
    pub fn new(config: PmatConfiguration) -> Self {
        Self {
            config,
            history: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn record_metrics(&mut self, metrics: &PlaygroundMetrics) -> PmatScore {
        let score = metrics.calculate_pmat(&self.config);

        // Check for quality events
        if let Some(event) = self.detect_quality_event(&score) {
            self.events.push(event);
        }

        self.history.push(score.clone());

        // Keep only recent history for trend analysis
        if self.history.len() > self.config.kaizen_thresholds.trend_sample_size * 2 {
            self.history.remove(0);
        }

        score
    }

    fn detect_quality_event(&self, current_score: &PmatScore) -> Option<QualityEvent> {
        if self.history.is_empty() {
            return None;
        }

        let recent_scores: Vec<_> = self
            .history
            .iter()
            .rev()
            .take(self.config.kaizen_thresholds.trend_sample_size)
            .collect();

        if recent_scores.len() < 2 {
            return None;
        }

        let avg_recent =
            recent_scores.iter().map(|s| s.tdg).sum::<f64>() / recent_scores.len() as f64;
        let change_percent = ((current_score.tdg - avg_recent) / avg_recent * 100.0).abs();

        if change_percent >= self.config.kaizen_thresholds.min_improvement_percent {
            if current_score.tdg > avg_recent {
                Some(QualityEvent {
                    timestamp: SystemTime::now(),
                    event_type: QualityEventType::PerformanceImprovement,
                    severity: QualityEventSeverity::Info,
                    message: format!("TDG improved by {:.1}%", change_percent),
                    metrics_snapshot: Some(current_score.clone()),
                })
            } else if change_percent >= self.config.kaizen_thresholds.max_regression_percent {
                Some(QualityEvent {
                    timestamp: SystemTime::now(),
                    event_type: QualityEventType::PerformanceRegression,
                    severity: QualityEventSeverity::Warning,
                    message: format!("TDG regressed by {:.1}%", change_percent),
                    metrics_snapshot: Some(current_score.clone()),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_recent_events(&self, duration: Duration) -> Vec<&QualityEvent> {
        let cutoff = SystemTime::now() - duration;
        self.events
            .iter()
            .filter(|e| e.timestamp >= cutoff)
            .collect()
    }

    pub fn get_trend_analysis(&self) -> TrendAnalysis {
        if self.history.len() < self.config.kaizen_thresholds.trend_sample_size {
            return TrendAnalysis::InsufficientData;
        }

        let recent_scores: Vec<f64> = self
            .history
            .iter()
            .rev()
            .take(self.config.kaizen_thresholds.trend_sample_size)
            .map(|s| s.tdg)
            .collect();

        let older_scores: Vec<f64> = self
            .history
            .iter()
            .rev()
            .skip(self.config.kaizen_thresholds.trend_sample_size)
            .take(self.config.kaizen_thresholds.trend_sample_size)
            .map(|s| s.tdg)
            .collect();

        if older_scores.is_empty() {
            return TrendAnalysis::InsufficientData;
        }

        let recent_avg = recent_scores.iter().sum::<f64>() / recent_scores.len() as f64;
        let older_avg = older_scores.iter().sum::<f64>() / older_scores.len() as f64;

        let change_percent = (recent_avg - older_avg) / older_avg * 100.0;

        if change_percent.abs() < self.config.kaizen_thresholds.min_improvement_percent {
            TrendAnalysis::Stable { change_percent }
        } else if change_percent > 0.0 {
            TrendAnalysis::Improving { change_percent }
        } else {
            TrendAnalysis::Declining { change_percent }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrendAnalysis {
    InsufficientData,
    Stable { change_percent: f64 },
    Improving { change_percent: f64 },
    Declining { change_percent: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pmat_configuration_default() {
        let config = PmatConfiguration::default_playground();
        assert!(config.validate().is_ok());
        assert!(config.targets.ttfmp_p50_ms > 0.0);
        assert!(config.targets.tti_p90_ms > config.targets.ttfmp_p50_ms);
    }

    #[test]
    fn test_scoring_functions() {
        let scoring = ScoringFunctions {
            sigmoid_steepness: 3.0,
            exponential_decay_rate: 0.5,
        };

        // Test sigmoid - should be 1.0 when actual <= target
        assert!(scoring.sigmoid(50.0, 100.0) > 0.5);
        assert!(scoring.sigmoid(100.0, 100.0) >= 0.5);
        assert!(scoring.sigmoid(150.0, 100.0) < 0.5);

        // Test exponential decay - should be 1.0 when actual <= target
        assert_eq!(scoring.exponential_decay(50.0, 100.0, 0.5), 1.0);
        assert_eq!(scoring.exponential_decay(100.0, 100.0, 0.5), 1.0);
        assert!(scoring.exponential_decay(150.0, 100.0, 0.5) < 1.0);
    }

    #[test]
    fn test_quality_monitor() {
        let config = PmatConfiguration::default_playground();
        let mut monitor = QualityMonitor::new(config);

        let metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        let score = monitor.record_metrics(&metrics);
        assert!(score.tdg > 0.0);
        assert!(score.tdg <= 1.0);
        assert!(score.productivity > 0.0);
        assert!(score.maintainability > 0.0);
        assert!(score.accessibility > 0.0);
        assert!(score.testability > 0.0);
    }

    #[test]
    fn test_configuration_validation() {
        let mut config = PmatConfiguration::default_playground();
        assert!(config.validate().is_ok());

        // Test invalid configurations
        config.targets.ttfmp_p50_ms = 0.0;
        assert!(config.validate().is_err());

        config.targets.ttfmp_p50_ms = 100.0;
        config.targets.tti_p90_ms = 50.0; // Less than ttfmp
        assert!(config.validate().is_err());

        config.targets.tti_p90_ms = 200.0;
        config.targets.wasm_size_budget_kb = 0.0;
        assert!(config.validate().is_err());

        config.targets.wasm_size_budget_kb = 1000.0;
        config.scoring.sigmoid_steepness = 0.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_trend_analysis() {
        let config = PmatConfiguration::default_playground();
        let mut monitor = QualityMonitor::new(config.clone());

        // Test insufficient data case
        assert!(matches!(
            monitor.get_trend_analysis(),
            TrendAnalysis::InsufficientData
        ));

        // Add enough data for trend analysis
        let base_metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        // Add older scores
        for _ in 0..config.kaizen_thresholds.trend_sample_size {
            monitor.record_metrics(&base_metrics);
        }

        // Add recent scores with improvement
        let mut improved_metrics = base_metrics.clone();
        improved_metrics.transpilation.latency_p95_ms = 20.0; // Better latency
        for _ in 0..config.kaizen_thresholds.trend_sample_size {
            monitor.record_metrics(&improved_metrics);
        }

        // Should detect improvement
        let trend = monitor.get_trend_analysis();
        match trend {
            TrendAnalysis::Improving { change_percent } => {
                assert!(change_percent > 0.0);
            }
            _ => panic!("Expected improving trend"),
        }
    }

    #[test]
    fn test_quality_events() {
        let mut config = PmatConfiguration::default_playground();
        config.kaizen_thresholds.min_improvement_percent = 5.0;
        config.kaizen_thresholds.max_regression_percent = 10.0;
        let mut monitor = QualityMonitor::new(config);

        let base_metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 50.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        // Build baseline
        for _ in 0..5 {
            monitor.record_metrics(&base_metrics);
        }

        // Trigger improvement event
        let mut improved = base_metrics.clone();
        improved.transpilation.latency_p95_ms = 25.0; // 50% improvement
        monitor.record_metrics(&improved);

        let events = monitor.get_recent_events(std::time::Duration::from_secs(60));
        assert!(!events.is_empty());
        assert!(matches!(
            events[0].event_type,
            QualityEventType::PerformanceImprovement
        ));
    }

    #[test]
    fn test_complexity_bucket_calculations() {
        let config = PmatConfiguration::default_playground();
        let scoring = &config.scoring;

        let metrics_simple = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        let score_simple = metrics_simple.calculate_productivity(&config.targets, scoring);
        assert!(score_simple > 0.0 && score_simple <= 1.0);

        // Test medium complexity
        let mut metrics_medium = metrics_simple.clone();
        metrics_medium.transpilation.complexity_bucket = ComplexityBucket::Medium;
        metrics_medium.transpilation.latency_p95_ms = 150.0;
        let score_medium = metrics_medium.calculate_productivity(&config.targets, scoring);
        assert!(score_medium > 0.0 && score_medium <= 1.0);

        // Test complex
        let mut metrics_complex = metrics_simple.clone();
        metrics_complex.transpilation.complexity_bucket = ComplexityBucket::Complex;
        metrics_complex.transpilation.latency_p95_ms = 800.0;
        let score_complex = metrics_complex.calculate_productivity(&config.targets, scoring);
        assert!(score_complex > 0.0 && score_complex <= 1.0);
    }

    #[test]
    fn test_edge_cases() {
        let config = PmatConfiguration::default_playground();

        // Test with zero python execution time
        let metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.0, // No cache hits
                error_rate: 1.0,     // 100% error rate
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 0.0, // Edge case
                energy_savings_percent: 0.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        let score = metrics.calculate_pmat(&config);
        assert!(score.tdg >= 0.0);
        assert!(score.tdg <= 1.0);
        assert!(score.maintainability == 0.0); // Due to 100% error rate
    }

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::InvalidTarget("Test error".to_string());
        assert_eq!(format!("{}", error), "Invalid target: Test error");

        let error = ConfigError::InvalidRange("Range error".to_string());
        assert_eq!(format!("{}", error), "Invalid range: Range error");

        let error = ConfigError::ParseError("Parse error".to_string());
        assert_eq!(format!("{}", error), "Parse error: Parse error");
    }

    #[test]
    fn test_regression_detection() {
        let mut config = PmatConfiguration::default_playground();
        config.kaizen_thresholds.min_improvement_percent = 5.0;
        config.kaizen_thresholds.max_regression_percent = 10.0;
        let mut monitor = QualityMonitor::new(config);

        let good_metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        // Build baseline with good metrics
        for _ in 0..5 {
            monitor.record_metrics(&good_metrics);
        }

        // Trigger regression event (worse performance)
        let mut bad_metrics = good_metrics.clone();
        bad_metrics.transpilation.latency_p95_ms = 100.0; // Much worse
        bad_metrics.transpilation.error_rate = 0.5; // 50% errors
        monitor.record_metrics(&bad_metrics);

        let events = monitor.get_recent_events(std::time::Duration::from_secs(60));
        assert!(!events.is_empty());
        assert!(matches!(
            events[0].event_type,
            QualityEventType::PerformanceRegression
        ));
        assert!(matches!(events[0].severity, QualityEventSeverity::Warning));
    }

    #[test]
    fn test_trend_stable() {
        let config = PmatConfiguration::default_playground();
        let mut monitor = QualityMonitor::new(config.clone());

        let metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        // Add enough consistent data for stable trend
        for _ in 0..(config.kaizen_thresholds.trend_sample_size * 2) {
            monitor.record_metrics(&metrics);
        }

        let trend = monitor.get_trend_analysis();
        match trend {
            TrendAnalysis::Stable { change_percent } => {
                assert!(change_percent.abs() < config.kaizen_thresholds.min_improvement_percent);
            }
            _ => panic!("Expected stable trend"),
        }
    }

    #[test]
    fn test_declining_trend() {
        let config = PmatConfiguration::default_playground();
        let mut monitor = QualityMonitor::new(config.clone());

        let good_metrics = PlaygroundMetrics {
            page_load: PageLoadMetrics {
                ttfmp_ms: 500.0,
                tti_ms: 1000.0,
                wasm_load_ms: 200.0,
                wasm_size_kb: 800.0,
            },
            transpilation: TranspilationMetrics {
                latency_p95_ms: 30.0,
                complexity_bucket: ComplexityBucket::Simple,
                cache_hit_rate: 0.8,
                error_rate: 0.02,
            },
            execution: ExecutionMetrics {
                rust_execution_ms: 10.0,
                python_execution_ms: 100.0,
                energy_savings_percent: 85.0,
                memory_usage_mb: 5.0,
            },
            quality_events: vec![],
        };

        // Add good metrics first
        for _ in 0..config.kaizen_thresholds.trend_sample_size {
            monitor.record_metrics(&good_metrics);
        }

        // Add worse metrics
        let mut worse_metrics = good_metrics.clone();
        worse_metrics.transpilation.latency_p95_ms = 60.0; // Worse latency
        worse_metrics.transpilation.error_rate = 0.1; // Higher error rate
        for _ in 0..config.kaizen_thresholds.trend_sample_size {
            monitor.record_metrics(&worse_metrics);
        }

        let trend = monitor.get_trend_analysis();
        match trend {
            TrendAnalysis::Declining { change_percent } => {
                assert!(change_percent < 0.0);
            }
            _ => panic!("Expected declining trend"),
        }
    }
}
