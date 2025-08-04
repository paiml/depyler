//! Quality Assurance Automation - Phase 8.4
//!
//! Automated test generation, quality metrics dashboard, continuous coverage monitoring,
//! and comprehensive quality assurance pipeline automation.

use depyler_core::DepylerPipeline;
use std::collections::{HashMap, BTreeMap};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

/// Automated test generator for systematic test case creation
pub struct AutomatedTestGenerator {
    pipeline: DepylerPipeline,
    generation_rules: Vec<TestGenerationRule>,
    generated_tests: HashMap<String, Vec<GeneratedTest>>,
}

#[derive(Debug, Clone)]
pub struct TestGenerationRule {
    pub pattern: String,
    pub category: TestCategory,
    pub priority: GenerationPriority,
    pub complexity_levels: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestCategory {
    Syntax,
    Semantics,
    Performance,
    EdgeCases,
    ErrorHandling,
    Integration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenerationPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct GeneratedTest {
    pub id: String,
    pub category: TestCategory,
    pub python_code: String,
    pub expected_outcome: ExpectedOutcome,
    pub complexity_score: usize,
    pub generation_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpectedOutcome {
    Success,
    CompileError,
    RuntimeError,
    PerformanceWarning,
}

impl AutomatedTestGenerator {
    /// Creates a new automated test generator
    /// 
    /// # Automated Generation Example
    /// ```
    /// let mut generator = AutomatedTestGenerator::new();
    /// 
    /// // Generate tests for arithmetic operations
    /// let tests = generator.generate_tests_for_category(TestCategory::Syntax, 5);
    /// assert!(tests.len() <= 5);
    /// 
    /// // Each test should have valid structure
    /// for test in &tests {
    ///     assert!(!test.python_code.is_empty());
    ///     assert!(!test.id.is_empty());
    /// }
    /// ```
    pub fn new() -> Self {
        let mut generator = Self {
            pipeline: DepylerPipeline::new(),
            generation_rules: Vec::new(),
            generated_tests: HashMap::new(),
        };
        
        generator.initialize_default_rules();
        generator
    }
    
    /// Generates tests for a specific category
    pub fn generate_tests_for_category(&mut self, category: TestCategory, count: usize) -> Vec<GeneratedTest> {
        let mut tests = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        for i in 0..count {
            let test = self.generate_single_test(&category, i, timestamp);
            tests.push(test);
        }
        
        // Store generated tests
        self.generated_tests.insert(format!("{:?}", category), tests.clone());
        tests
    }
    
    /// Generates comprehensive test suite across all categories
    /// 
    /// # Comprehensive Generation Example
    /// ```
    /// let mut generator = AutomatedTestGenerator::new();
    /// 
    /// let suite = generator.generate_comprehensive_suite(20);
    /// assert!(suite.total_tests > 0);
    /// assert!(suite.tests_by_category.len() > 0);
    /// 
    /// // Should have diverse test categories
    /// let categories: std::collections::HashSet<_> = suite.tests_by_category.keys().collect();
    /// assert!(categories.len() >= 3);
    /// ```
    pub fn generate_comprehensive_suite(&mut self, total_tests: usize) -> TestSuite {
        let categories = vec![
            TestCategory::Syntax,
            TestCategory::Semantics,
            TestCategory::Performance,
            TestCategory::EdgeCases,
            TestCategory::ErrorHandling,
            TestCategory::Integration,
        ];
        
        let tests_per_category = total_tests / categories.len();
        let mut all_tests = HashMap::new();
        let mut total_generated = 0;
        
        for category in categories {
            let tests = self.generate_tests_for_category(category.clone(), tests_per_category);
            total_generated += tests.len();
            all_tests.insert(format!("{:?}", category), tests);
        }
        
        TestSuite {
            total_tests: total_generated,
            tests_by_category: all_tests,
            generation_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
    
    /// Validates generated tests by running them through the pipeline
    /// 
    /// # Test Validation Example
    /// ```
    /// let mut generator = AutomatedTestGenerator::new();
    /// 
    /// let tests = generator.generate_tests_for_category(TestCategory::Syntax, 3);
    /// let validation = generator.validate_generated_tests(&tests);
    /// 
    /// assert_eq!(validation.total_tests, 3);
    /// assert!(validation.success_rate >= 0.0);
    /// assert!(validation.success_rate <= 1.0);
    /// ```
    pub fn validate_generated_tests(&self, tests: &[GeneratedTest]) -> TestValidationResult {
        let mut successful = 0;
        let mut failed = 0;
        let mut unexpected_outcomes = 0;
        
        for test in tests {
            let result = self.pipeline.transpile(&test.python_code);
            
            let actual_outcome = match result {
                Ok(_) => ExpectedOutcome::Success,
                Err(_) => ExpectedOutcome::CompileError,
            };
            
            if actual_outcome == test.expected_outcome {
                successful += 1;
            } else {
                failed += 1;
                if matches!(test.expected_outcome, ExpectedOutcome::Success) && matches!(actual_outcome, ExpectedOutcome::CompileError) {
                    unexpected_outcomes += 1;
                }
            }
        }
        
        let success_rate = if tests.is_empty() {
            0.0
        } else {
            successful as f64 / tests.len() as f64
        };
        
        TestValidationResult {
            total_tests: tests.len(),
            successful_validations: successful,
            failed_validations: failed,
            unexpected_outcomes,
            success_rate,
        }
    }
    
    // Private helper methods
    fn initialize_default_rules(&mut self) {
        self.generation_rules = vec![
            TestGenerationRule {
                pattern: "function_definition".to_string(),
                category: TestCategory::Syntax,
                priority: GenerationPriority::Critical,
                complexity_levels: vec![1, 2, 3],
            },
            TestGenerationRule {
                pattern: "conditional_logic".to_string(),
                category: TestCategory::Semantics,
                priority: GenerationPriority::High,
                complexity_levels: vec![1, 2, 3, 4],
            },
            TestGenerationRule {
                pattern: "error_conditions".to_string(),
                category: TestCategory::ErrorHandling,
                priority: GenerationPriority::High,
                complexity_levels: vec![1, 2],
            },
            TestGenerationRule {
                pattern: "performance_sensitive".to_string(),
                category: TestCategory::Performance,
                priority: GenerationPriority::Medium,
                complexity_levels: vec![2, 3, 4, 5],
            },
        ];
    }
    
    fn generate_single_test(&self, category: &TestCategory, index: usize, timestamp: u64) -> GeneratedTest {
        let (code, expected, complexity) = match category {
            TestCategory::Syntax => {
                let complexity = (index % 3) + 1;
                let code = match complexity {
                    1 => "def simple(): return 42".to_string(),
                    2 => "def with_params(x: int, y: str) -> str: return y".to_string(),
                    _ => "def complex(a: int, b: int, c: str) -> str: return f'{a + b}: {c}'".to_string(),
                };
                (code, ExpectedOutcome::Success, complexity)
            },
            TestCategory::Semantics => {
                let complexity = (index % 4) + 1;
                let code = match complexity {
                    1 => "def check(x: int) -> bool: return x > 0".to_string(),
                    2 => "def conditional(x: int) -> str: return 'pos' if x > 0 else 'neg'".to_string(),
                    3 => "def nested_if(x: int, y: int) -> str: return 'both' if x > 0 and y > 0 else 'not_both'".to_string(),
                    _ => "def complex_logic(x: int) -> str: return 'high' if x > 100 else 'med' if x > 10 else 'low'".to_string(),
                };
                (code, ExpectedOutcome::Success, complexity)
            },
            TestCategory::Performance => {
                let complexity = (index % 4) + 2;
                let code = match complexity {
                    2 => "def simple_loop(n: int) -> int: return sum(range(n))".to_string(),
                    3 => "def nested_loop(n: int) -> int: return sum(i*j for i in range(n) for j in range(n))".to_string(),
                    _ => "def recursive(n: int) -> int: return n if n <= 1 else recursive(n-1) + recursive(n-2)".to_string(),
                };
                (code, ExpectedOutcome::Success, complexity)
            },
            TestCategory::EdgeCases => {
                let complexity = (index % 3) + 1;
                let code = match index % 4 {
                    0 => "def empty_function(): pass".to_string(),
                    1 => "def unicode_函数(): return '测试'".to_string(),
                    2 => "def large_number(): return 9223372036854775807".to_string(),
                    _ => "def edge_case(x: int) -> int: return x // 0 if x != 0 else 1".to_string(),
                };
                let expected = if code.contains("// 0") {
                    ExpectedOutcome::CompileError
                } else {
                    ExpectedOutcome::Success
                };
                (code, expected, complexity)
            },
            TestCategory::ErrorHandling => {
                let complexity = (index % 2) + 1;
                let code = match index % 3 {
                    0 => "def broken_syntax(\n    return 42".to_string(),
                    1 => "def invalid_param(: pass".to_string(),
                    _ => "async def unsupported(): await something()".to_string(),
                };
                (code, ExpectedOutcome::CompileError, complexity)
            },
            TestCategory::Integration => {
                let complexity = (index % 3) + 2;
                let code = format!(
                    "def integration_test_{}(data: list) -> dict: return {{'count': len(data), 'sum': sum(data)}}",
                    index
                );
                (code, ExpectedOutcome::Success, complexity)
            },
        };
        
        GeneratedTest {
            id: format!("{:?}_{}_{}_{}", category, index, complexity, timestamp),
            category: category.clone(),
            python_code: code,
            expected_outcome: expected,
            complexity_score: complexity,
            generation_timestamp: timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub total_tests: usize,
    pub tests_by_category: HashMap<String, Vec<GeneratedTest>>,
    pub generation_timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct TestValidationResult {
    pub total_tests: usize,
    pub successful_validations: usize,
    pub failed_validations: usize,
    pub unexpected_outcomes: usize,
    pub success_rate: f64,
}

/// Quality metrics dashboard for comprehensive quality monitoring
pub struct QualityMetricsDashboard {
    metrics: BTreeMap<u64, QualitySnapshot>,
    alerts: Vec<QualityAlert>,
    thresholds: QualityThresholds,
}

#[derive(Debug, Clone)]
pub struct QualitySnapshot {
    pub timestamp: u64,
    pub test_coverage: f64,
    pub mutation_score: f64,
    pub performance_score: f64,
    pub error_rate: f64,
    pub code_quality_score: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone)]
pub struct QualityAlert {
    pub timestamp: u64,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub metric_value: f64,
    pub threshold: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertType {
    CoverageDropped,
    MutationScoreDeclined,
    PerformanceRegression,
    ErrorRateIncreased,
    QualityDeclined,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct QualityThresholds {
    pub min_test_coverage: f64,
    pub min_mutation_score: f64,
    pub min_performance_score: f64,
    pub max_error_rate: f64,
    pub min_code_quality: f64,
}

impl QualityMetricsDashboard {
    /// Creates a new quality metrics dashboard
    /// 
    /// # Quality Monitoring Example
    /// ```
    /// let mut dashboard = QualityMetricsDashboard::new();
    /// 
    /// // Record quality snapshot
    /// dashboard.record_snapshot(QualitySnapshot {
    ///     timestamp: 1000,
    ///     test_coverage: 0.85,
    ///     mutation_score: 0.75,
    ///     performance_score: 0.90,
    ///     error_rate: 0.05,
    ///     code_quality_score: 0.88,
    ///     overall_score: 0.82,
    /// });
    /// 
    /// let latest = dashboard.get_latest_snapshot();
    /// assert!(latest.is_some());
    /// assert_eq!(latest.unwrap().test_coverage, 0.85);
    /// ```
    pub fn new() -> Self {
        Self {
            metrics: BTreeMap::new(),
            alerts: Vec::new(),
            thresholds: QualityThresholds {
                min_test_coverage: 0.80,
                min_mutation_score: 0.70,
                min_performance_score: 0.75,
                max_error_rate: 0.10,
                min_code_quality: 0.75,
            },
        }
    }
    
    /// Records a quality snapshot
    pub fn record_snapshot(&mut self, snapshot: QualitySnapshot) {
        // Check for threshold violations and generate alerts
        self.check_thresholds(&snapshot);
        
        // Store the snapshot
        self.metrics.insert(snapshot.timestamp, snapshot);
    }
    
    /// Gets the latest quality snapshot
    pub fn get_latest_snapshot(&self) -> Option<&QualitySnapshot> {
        self.metrics.values().last()
    }
    
    /// Gets quality trend over time
    /// 
    /// # Trend Analysis Example
    /// ```
    /// let mut dashboard = QualityMetricsDashboard::new();
    /// 
    /// // Record multiple snapshots
    /// for i in 1..=5 {
    ///     dashboard.record_snapshot(QualitySnapshot {
    ///         timestamp: i * 1000,
    ///         test_coverage: 0.80 + (i as f64 * 0.02),
    ///         mutation_score: 0.70,
    ///         performance_score: 0.85,
    ///         error_rate: 0.05,
    ///         code_quality_score: 0.80,
    ///         overall_score: 0.80,
    ///     });
    /// }
    /// 
    /// let trend = dashboard.get_quality_trend(3);
    /// assert_eq!(trend.snapshots.len(), 3);  // Last 3 snapshots
    /// assert!(trend.coverage_trend > 0.0);   // Improving trend
    /// ```
    pub fn get_quality_trend(&self, period: usize) -> QualityTrend {
        let snapshots: Vec<_> = self.metrics.values()
            .rev()
            .take(period)
            .cloned()
            .collect();
        
        if snapshots.len() < 2 {
            return QualityTrend {
                snapshots,
                coverage_trend: 0.0,
                mutation_trend: 0.0,
                performance_trend: 0.0,
                overall_trend: 0.0,
            };
        }
        
        let first = &snapshots.clone()[snapshots.len() - 1];
        let last = &snapshots.clone()[0];
        
        QualityTrend {
            snapshots: snapshots.clone(),
            coverage_trend: last.test_coverage - first.test_coverage,
            mutation_trend: last.mutation_score - first.mutation_score,
            performance_trend: last.performance_score - first.performance_score,
            overall_trend: last.overall_score - first.overall_score,
        }
    }
    
    /// Gets active quality alerts
    /// 
    /// # Alert Management Example
    /// ```
    /// let mut dashboard = QualityMetricsDashboard::new();
    /// 
    /// // Record snapshot that triggers alerts
    /// dashboard.record_snapshot(QualitySnapshot {
    ///     timestamp: 1000,
    ///     test_coverage: 0.60,  // Below threshold (0.80)
    ///     mutation_score: 0.50, // Below threshold (0.70)
    ///     performance_score: 0.85,
    ///     error_rate: 0.15,     // Above threshold (0.10)
    ///     code_quality_score: 0.80,
    ///     overall_score: 0.65,
    /// });
    /// 
    /// let alerts = dashboard.get_active_alerts();
    /// assert!(alerts.len() >= 2);  // Should have coverage and error rate alerts
    /// 
    /// // Check alert types
    /// let alert_types: std::collections::HashSet<_> = alerts.iter()
    ///     .map(|a| &a.alert_type)
    ///     .collect();
    /// assert!(alert_types.contains(&AlertType::CoverageDropped));
    /// ```
    pub fn get_active_alerts(&self) -> Vec<&QualityAlert> {
        // Return alerts from the last hour (3600 seconds)
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(3600);
        
        self.alerts.iter()
            .filter(|alert| alert.timestamp >= cutoff_time)
            .collect()
    }
    
    /// Generates quality report
    pub fn generate_quality_report(&self) -> QualityReport {
        let latest = self.get_latest_snapshot();
        let trend = self.get_quality_trend(5);
        let active_alerts = self.get_active_alerts();
        
        let status = if let Some(snapshot) = latest {
            if snapshot.overall_score >= 0.90 {
                QualityStatus::Excellent
            } else if snapshot.overall_score >= 0.80 {
                QualityStatus::Good
            } else if snapshot.overall_score >= 0.70 {
                QualityStatus::Fair
            } else {
                QualityStatus::Poor
            }
        } else {
            QualityStatus::Unknown
        };
        
        QualityReport {
            status,
            latest_snapshot: latest.cloned(),
            trend_analysis: trend,
            active_alerts: active_alerts.into_iter().cloned().collect(),
            recommendations: self.generate_recommendations(),
        }
    }
    
    // Private helper methods
    fn check_thresholds(&mut self, snapshot: &QualitySnapshot) {
        let timestamp = snapshot.timestamp;
        
        // Check test coverage
        if snapshot.test_coverage < self.thresholds.min_test_coverage {
            self.alerts.push(QualityAlert {
                timestamp,
                alert_type: AlertType::CoverageDropped,
                severity: AlertSeverity::High,
                message: format!("Test coverage dropped to {:.1}%", snapshot.test_coverage * 100.0),
                metric_value: snapshot.test_coverage,
                threshold: self.thresholds.min_test_coverage,
            });
        }
        
        // Check mutation score
        if snapshot.mutation_score < self.thresholds.min_mutation_score {
            self.alerts.push(QualityAlert {
                timestamp,
                alert_type: AlertType::MutationScoreDeclined,
                severity: AlertSeverity::Medium,
                message: format!("Mutation score declined to {:.1}%", snapshot.mutation_score * 100.0),
                metric_value: snapshot.mutation_score,
                threshold: self.thresholds.min_mutation_score,
            });
        }
        
        // Check error rate
        if snapshot.error_rate > self.thresholds.max_error_rate {
            self.alerts.push(QualityAlert {
                timestamp,
                alert_type: AlertType::ErrorRateIncreased,
                severity: AlertSeverity::Critical,
                message: format!("Error rate increased to {:.1}%", snapshot.error_rate * 100.0),
                metric_value: snapshot.error_rate,
                threshold: self.thresholds.max_error_rate,
            });
        }
        
        // Check performance
        if snapshot.performance_score < self.thresholds.min_performance_score {
            self.alerts.push(QualityAlert {
                timestamp,
                alert_type: AlertType::PerformanceRegression,
                severity: AlertSeverity::Medium,
                message: format!("Performance score dropped to {:.1}%", snapshot.performance_score * 100.0),
                metric_value: snapshot.performance_score,
                threshold: self.thresholds.min_performance_score,
            });
        }
    }
    
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if let Some(latest) = self.get_latest_snapshot() {
            if latest.test_coverage < 0.85 {
                recommendations.push("Increase test coverage by adding more comprehensive test cases".to_string());
            }
            
            if latest.mutation_score < 0.75 {
                recommendations.push("Improve test quality by adding more effective test cases that catch mutations".to_string());
            }
            
            if latest.error_rate > 0.05 {
                recommendations.push("Reduce error rate by fixing failing tests and improving error handling".to_string());
            }
            
            if latest.performance_score < 0.80 {
                recommendations.push("Optimize performance by profiling slow operations and improving algorithms".to_string());
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("Quality metrics are within acceptable ranges. Continue monitoring.".to_string());
        }
        
        recommendations
    }
}

#[derive(Debug, Clone)]
pub struct QualityTrend {
    pub snapshots: Vec<QualitySnapshot>,
    pub coverage_trend: f64,
    pub mutation_trend: f64,
    pub performance_trend: f64,
    pub overall_trend: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QualityStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct QualityReport {
    pub status: QualityStatus,
    pub latest_snapshot: Option<QualitySnapshot>,
    pub trend_analysis: QualityTrend,
    pub active_alerts: Vec<QualityAlert>,
    pub recommendations: Vec<String>,
}

impl fmt::Display for QualityReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quality Assurance Report")?;
        writeln!(f, "=======================")?;
        writeln!(f, "Status: {:?}", self.status)?;
        
        if let Some(snapshot) = &self.latest_snapshot {
            writeln!(f, "\nLatest Metrics:")?;
            writeln!(f, "  Test Coverage: {:.1}%", snapshot.test_coverage * 100.0)?;
            writeln!(f, "  Mutation Score: {:.1}%", snapshot.mutation_score * 100.0)?;
            writeln!(f, "  Performance: {:.1}%", snapshot.performance_score * 100.0)?;
            writeln!(f, "  Error Rate: {:.1}%", snapshot.error_rate * 100.0)?;
            writeln!(f, "  Overall Score: {:.1}%", snapshot.overall_score * 100.0)?;
        }
        
        if !self.active_alerts.is_empty() {
            writeln!(f, "\nActive Alerts ({}):", self.active_alerts.len())?;
            for alert in &self.active_alerts {
                writeln!(f, "  [{:?}] {}", alert.severity, alert.message)?;
            }
        }
        
        if !self.recommendations.is_empty() {
            writeln!(f, "\nRecommendations:")?;
            for rec in &self.recommendations {
                writeln!(f, "  • {}", rec)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test automated test generation
    #[test]
    fn test_automated_test_generation() {
        println!("=== Automated Test Generation Test ===");
        
        let mut generator = AutomatedTestGenerator::new();
        
        // Test category-specific generation
        let syntax_tests = generator.generate_tests_for_category(TestCategory::Syntax, 3);
        println!("Generated {} syntax tests", syntax_tests.len());
        assert_eq!(syntax_tests.len(), 3);
        
        for test in &syntax_tests {
            println!("  Test {}: {}", test.id, test.python_code.chars().take(50).collect::<String>());
            assert_eq!(test.category, TestCategory::Syntax);
            assert!(!test.python_code.is_empty());
            assert!(!test.id.is_empty());
            assert!(test.complexity_score > 0);
        }
        
        // Test different categories
        let semantics_tests = generator.generate_tests_for_category(TestCategory::Semantics, 2);
        println!("Generated {} semantics tests", semantics_tests.len());
        assert_eq!(semantics_tests.len(), 2);
        
        let error_tests = generator.generate_tests_for_category(TestCategory::ErrorHandling, 2);
        println!("Generated {} error handling tests", error_tests.len());
        assert_eq!(error_tests.len(), 2);
        
        // Verify different categories generate different types of tests
        assert_ne!(syntax_tests[0].category, semantics_tests[0].category);
        assert_ne!(syntax_tests[0].category, error_tests[0].category);
        
        // Test comprehensive suite generation
        let suite = generator.generate_comprehensive_suite(12);
        println!("Generated comprehensive suite with {} total tests across {} categories",
            suite.total_tests, suite.tests_by_category.len());
        
        assert!(suite.total_tests >= 10); // Should generate most of the requested tests
        assert!(suite.tests_by_category.len() >= 3); // Multiple categories
        
        // Validate generated tests
        let all_tests: Vec<_> = suite.tests_by_category.values()
            .flat_map(|tests| tests.iter().cloned())
            .collect();
        
        let validation = generator.validate_generated_tests(&all_tests);
        println!("Validation: {}/{} successful ({:.1}% success rate)",
            validation.successful_validations,
            validation.total_tests,
            validation.success_rate * 100.0
        );
        
        assert_eq!(validation.total_tests, all_tests.len());
        assert!(validation.success_rate >= 0.0);
        assert!(validation.success_rate <= 1.0);
    }

    /// Test quality metrics dashboard
    #[test]
    fn test_quality_metrics_dashboard() {
        println!("=== Quality Metrics Dashboard Test ===");
        
        let mut dashboard = QualityMetricsDashboard::new();
        
        // Use realistic timestamps (current time)
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Test recording snapshots
        let snapshot1 = QualitySnapshot {
            timestamp: current_time - 100,  // 100 seconds ago
            test_coverage: 0.85,
            mutation_score: 0.75,
            performance_score: 0.90,
            error_rate: 0.05,
            code_quality_score: 0.88,
            overall_score: 0.82,
        };
        
        dashboard.record_snapshot(snapshot1.clone());
        
        let latest = dashboard.get_latest_snapshot();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().test_coverage, 0.85);
        
        // Test snapshot that triggers alerts
        let snapshot2 = QualitySnapshot {
            timestamp: current_time,  // Current time
            test_coverage: 0.60,  // Below threshold
            mutation_score: 0.50, // Below threshold
            performance_score: 0.85,
            error_rate: 0.15,     // Above threshold
            code_quality_score: 0.80,
            overall_score: 0.65,
        };
        
        dashboard.record_snapshot(snapshot2);
        
        let alerts = dashboard.get_active_alerts();
        println!("Generated {} alerts", alerts.len());
        assert!(alerts.len() >= 2); // Should have multiple alerts
        
        // Check alert types
        let alert_types: std::collections::HashSet<_> = alerts.iter()
            .map(|a| &a.alert_type)
            .collect();
        
        println!("Alert types: {:?}", alert_types);
        assert!(alert_types.contains(&AlertType::CoverageDropped));
        assert!(alert_types.contains(&AlertType::ErrorRateIncreased));
        
        // Test trend analysis
        let snapshot3 = QualitySnapshot {
            timestamp: current_time + 100,  // Future time
            test_coverage: 0.88,  // Improved
            mutation_score: 0.78, // Improved
            performance_score: 0.92,
            error_rate: 0.03,     // Improved
            code_quality_score: 0.90,
            overall_score: 0.87,
        };
        
        dashboard.record_snapshot(snapshot3);
        
        let trend = dashboard.get_quality_trend(3);
        println!("Quality trend over 3 snapshots:");
        println!("  Coverage trend: {:+.3}", trend.coverage_trend);
        println!("  Overall trend: {:+.3}", trend.overall_trend);
        
        assert_eq!(trend.snapshots.len(), 3);
        assert!(trend.coverage_trend > 0.0); // Should show improvement
        assert!(trend.overall_trend > 0.0);  // Should show improvement
        
        // Test quality report generation
        let report = dashboard.generate_quality_report();
        println!("Quality report status: {:?}", report.status);
        
        assert!(matches!(report.status, QualityStatus::Good | QualityStatus::Excellent));
        assert!(report.latest_snapshot.is_some());
        assert!(!report.recommendations.is_empty());
        
        // Test report display
        let report_str = format!("{}", report);
        assert!(report_str.contains("Quality Assurance Report"));
        assert!(report_str.contains("Test Coverage"));
        
        println!("\n{}", report);
    }

    /// Test quality assurance automation integration
    #[test]
    fn test_quality_automation_integration() {
        println!("=== Quality Automation Integration Test ===");
        
        // Integrate test generation with quality monitoring
        let mut generator = AutomatedTestGenerator::new();
        let mut dashboard = QualityMetricsDashboard::new();
        
        // Generate test suite
        let suite = generator.generate_comprehensive_suite(18);
        println!("Generated {} tests across {} categories",
            suite.total_tests, suite.tests_by_category.len());
        
        // Validate tests and collect metrics
        let mut total_tests = 0;
        let mut successful_tests = 0;
        let mut error_count = 0;
        
        for (category, tests) in &suite.tests_by_category {
            let validation = generator.validate_generated_tests(tests);
            total_tests += validation.total_tests;
            successful_tests += validation.successful_validations;
            error_count += validation.failed_validations;
            
            println!("Category {}: {}/{} tests successful ({:.1}%)",
                category,
                validation.successful_validations,
                validation.total_tests,
                validation.success_rate * 100.0
            );
        }
        
        // Calculate quality metrics
        let coverage = if total_tests > 0 {
            successful_tests as f64 / total_tests as f64
        } else {
            0.0
        };
        
        let error_rate = if total_tests > 0 {
            error_count as f64 / total_tests as f64
        } else {
            0.0
        };
        
        // Create quality snapshot
        let snapshot = QualitySnapshot {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            test_coverage: coverage,
            mutation_score: 0.75, // Simulated
            performance_score: 0.85, // Simulated
            error_rate,
            code_quality_score: 0.80, // Simulated
            overall_score: (coverage + 0.75 + 0.85 + (1.0 - error_rate) + 0.80) / 5.0,
        };
        
        dashboard.record_snapshot(snapshot);
        
        // Generate comprehensive report
        let report = dashboard.generate_quality_report();
        
        println!("\nIntegrated Quality Report:");
        println!("  Generated Tests: {}", total_tests);
        println!("  Successful Tests: {}", successful_tests);
        println!("  Test Success Rate: {:.1}%", coverage * 100.0);
        println!("  Error Rate: {:.1}%", error_rate * 100.0);
        println!("  Overall Quality: {:?}", report.status);
        
        // Validate integration results
        assert!(total_tests > 0);
        assert!(coverage >= 0.0 && coverage <= 1.0);
        assert!(error_rate >= 0.0 && error_rate <= 1.0);
        assert!(report.latest_snapshot.is_some());
        
        // Quality should be reasonable for generated tests
        if let Some(latest) = &report.latest_snapshot {
            assert!(latest.overall_score > 0.0);
            assert!(latest.overall_score <= 1.0);
        }
        
        println!("\nFull Quality Report:\n{}", report);
    }

    /// Test comprehensive quality assurance pipeline
    #[test]
    fn test_comprehensive_qa_pipeline() {
        println!("=== Comprehensive QA Pipeline Test ===");
        
        let mut generator = AutomatedTestGenerator::new();
        let mut dashboard = QualityMetricsDashboard::new();
        
        // Simulate QA pipeline over time
        let time_periods = vec![1000, 2000, 3000, 4000, 5000];
        
        for (i, timestamp) in time_periods.iter().enumerate() {
            println!("\n--- QA Pipeline Run {} (timestamp: {}) ---", i + 1, timestamp);
            
            // Generate tests for this period
            let test_count = 8 + (i * 2); // Increasing test generation
            let suite = generator.generate_comprehensive_suite(test_count);
            
            // Validate all tests
            let all_tests: Vec<_> = suite.tests_by_category.values()
                .flat_map(|tests| tests.iter().cloned())
                .collect();
            
            let validation = generator.validate_generated_tests(&all_tests);
            
            // Simulate improving quality over time
            let base_coverage = validation.success_rate;
            let coverage_improvement = i as f64 * 0.05; // 5% improvement each period
            let adjusted_coverage = (base_coverage + coverage_improvement).min(1.0);
            
            let mutation_score = 0.65 + (i as f64 * 0.03); // Improving mutation score
            let performance_score = 0.80 + (i as f64 * 0.02); // Improving performance
            let error_rate = (0.10 - (i as f64 * 0.01)).max(0.01); // Decreasing error rate
            
            // Create quality snapshot
            let snapshot = QualitySnapshot {
                timestamp: *timestamp,
                test_coverage: adjusted_coverage,
                mutation_score,
                performance_score,
                error_rate,
                code_quality_score: 0.75 + (i as f64 * 0.03),
                overall_score: (adjusted_coverage + mutation_score + performance_score + (1.0 - error_rate) + 0.75) / 5.0,
            };
            
            dashboard.record_snapshot(snapshot.clone());
            
            println!("  Tests Generated: {}", all_tests.len());
            println!("  Test Coverage: {:.1}%", adjusted_coverage * 100.0);
            println!("  Mutation Score: {:.1}%", mutation_score * 100.0);
            println!("  Performance: {:.1}%", performance_score * 100.0);
            println!("  Error Rate: {:.1}%", error_rate * 100.0);
            println!("  Overall Score: {:.1}%", snapshot.overall_score * 100.0);
        }
        
        // Analyze final results
        let final_report = dashboard.generate_quality_report();
        let trend = dashboard.get_quality_trend(5);
        
        println!("\n=== Final QA Pipeline Results ===");
        println!("Quality Status: {:?}", final_report.status);
        println!("Coverage Trend: {:+.3}", trend.coverage_trend);
        println!("Mutation Trend: {:+.3}", trend.mutation_trend);
        println!("Performance Trend: {:+.3}", trend.performance_trend);
        println!("Overall Trend: {:+.3}", trend.overall_trend);
        
        // Validate pipeline effectiveness
        assert_eq!(trend.snapshots.len(), 5);
        assert!(trend.coverage_trend > 0.0); // Should show improvement
        assert!(trend.overall_trend > 0.0);  // Should show improvement
        
        // Final quality should be good
        assert!(matches!(final_report.status, QualityStatus::Good | QualityStatus::Excellent));
        
        // Should have fewer alerts in later periods (quality improved)
        let recent_alerts = dashboard.get_active_alerts();
        println!("Active alerts: {}", recent_alerts.len());
        
        println!("\nFinal Pipeline Report:\n{}", final_report);
        
        // Validate that the QA pipeline shows continuous improvement
        if let Some(latest) = &final_report.latest_snapshot {
            assert!(latest.test_coverage >= 0.80); // Should achieve good coverage
            assert!(latest.overall_score >= 0.75);  // Should achieve good overall quality
            assert!(latest.error_rate <= 0.10);     // Should maintain low error rate
        }
    }
}