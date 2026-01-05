//! Hunt Mode: Automated Calibration Subsystem for Depyler
//!
//! Implements Toyota Production System principles for continuous compiler improvement:
//! - Kaizen (改善): Continuous incremental improvement tracking
//! - Jidoka (自働化): Automation with quality gates
//! - Andon (行灯): Visual status and stop-the-line signaling
//! - Hansei (反省): Reflection and lessons learned
//! - Genchi Genbutsu (現地現物): Direct observation of failures
//!
//! # Architecture
//!
//! Hunt Mode operates as a PDCA (Plan-Do-Check-Act) cycle:
//! 1. PLAN (Hunt): Classify and prioritize compilation failures
//! 2. DO (Isolate): Synthesize minimal reproduction cases
//! 3. CHECK (Repair): Apply heuristic fixes with quality thresholds
//! 4. ACT (Verify): Validate and commit successful fixes
//!
//! # References
//!
//! - Liker, J.K. (2004). The Toyota Way
//! - Ohno, T. (1988). Toyota Production System
//! - Beck, K. (2002). Test Driven Development

pub mod kaizen;
pub mod planner;
pub mod isolator;
pub mod repair;
pub mod verifier;
pub mod hansei;
pub mod five_whys;
pub mod hunt_shim;

// Re-exports for convenience
pub use kaizen::KaizenMetrics;
pub use planner::{HuntPlanner, FailurePattern, ErrorCluster};
pub use isolator::{MinimalReproducer, ReproCase};
pub use repair::{JidokaRepairEngine, RepairResult, Mutator};
pub use verifier::{AndonVerifier, AndonStatus, VerifyResult};
pub use hansei::{HanseiReflector, Lesson, CycleOutcome};
pub use five_whys::{FiveWhysAnalyzer, RootCauseChain, WhyStep};

use std::path::PathBuf;

/// Configuration for Hunt Mode operation
#[derive(Debug, Clone)]
pub struct HuntConfig {
    /// Maximum number of cycles to run
    pub max_cycles: u32,
    /// Minimum confidence threshold for auto-applying fixes (Jidoka)
    pub quality_threshold: f64,
    /// Target compilation rate (Kaizen goal)
    pub target_rate: f64,
    /// Stop if no improvement after this many cycles
    pub plateau_threshold: u32,
    /// Enable Five Whys analysis
    pub enable_five_whys: bool,
    /// Path to lessons database
    pub lessons_database: PathBuf,
}

impl Default for HuntConfig {
    fn default() -> Self {
        Self {
            max_cycles: 100,
            quality_threshold: 0.85,
            target_rate: 0.80,
            plateau_threshold: 5,
            enable_five_whys: true,
            lessons_database: PathBuf::from(".depyler/lessons.db"),
        }
    }
}

/// Main Hunt Mode engine that orchestrates the PDCA cycle
pub struct HuntEngine {
    config: HuntConfig,
    metrics: KaizenMetrics,
    planner: HuntPlanner,
    reproducer: MinimalReproducer,
    repair_engine: JidokaRepairEngine,
    verifier: AndonVerifier,
    reflector: HanseiReflector,
}

impl HuntEngine {
    /// Create a new Hunt Mode engine with the given configuration
    pub fn new(config: HuntConfig) -> Self {
        Self {
            metrics: KaizenMetrics::new(),
            planner: HuntPlanner::new(),
            reproducer: MinimalReproducer::new(),
            repair_engine: JidokaRepairEngine::new(config.quality_threshold),
            verifier: AndonVerifier::new(),
            reflector: HanseiReflector::new(),
            config,
        }
    }

    /// Run a single PDCA cycle
    ///
    /// Returns the outcome of the cycle for Hansei reflection
    pub fn run_cycle(&mut self) -> anyhow::Result<CycleOutcome> {
        // PLAN: Select highest-priority failure pattern
        let pattern = self.planner.select_next_target()
            .ok_or_else(|| anyhow::anyhow!("No failure patterns to process"))?;

        // DO: Synthesize minimal reproduction case
        let repro = self.reproducer.synthesize_repro(&pattern)?;

        // CHECK: Attempt repair with Jidoka quality gates
        let repair_result = self.repair_engine.attempt_repair(&repro)?;

        // ACT: Verify and commit if successful
        let verify_result = match repair_result {
            RepairResult::Success(fix) => {
                self.verifier.verify_and_commit(&fix, &repro)?
            }
            RepairResult::NeedsHumanReview { fix, confidence, reason } => {
                VerifyResult::NeedsReview { fix, confidence, reason }
            }
            RepairResult::NoFixFound => {
                VerifyResult::NoFixAvailable
            }
        };

        // Create outcome for Hansei reflection
        let outcome = CycleOutcome {
            pattern,
            repro,
            verify_result,
            metrics_snapshot: self.metrics.clone(),
        };

        // Hansei: Reflect and learn
        let _lessons = self.reflector.reflect_on_cycle(&outcome);

        // Update Kaizen metrics
        self.metrics.record_cycle(&outcome);

        Ok(outcome)
    }

    /// Run Hunt Mode until target rate achieved or plateau detected
    pub fn run_until_complete(&mut self) -> anyhow::Result<Vec<CycleOutcome>> {
        let mut outcomes = Vec::new();

        for cycle in 0..self.config.max_cycles {
            // Check if target achieved (Kaizen goal)
            if self.metrics.compilation_rate >= self.config.target_rate {
                tracing::info!(
                    "Target rate {:.1}% achieved after {} cycles",
                    self.config.target_rate * 100.0,
                    cycle
                );
                break;
            }

            // Check for plateau (Andon: stop if no progress)
            if self.metrics.cycles_since_improvement >= self.config.plateau_threshold {
                tracing::warn!(
                    "Plateau detected after {} cycles without improvement",
                    self.metrics.cycles_since_improvement
                );
                break;
            }

            match self.run_cycle() {
                Ok(outcome) => outcomes.push(outcome),
                Err(e) => {
                    tracing::error!("Cycle {} failed: {}", cycle, e);
                    // Andon: Don't stop completely, log and continue
                }
            }
        }

        Ok(outcomes)
    }

    /// Get current Andon status
    pub fn andon_status(&self) -> &AndonStatus {
        self.verifier.status()
    }

    /// Get current Kaizen metrics
    pub fn metrics(&self) -> &KaizenMetrics {
        &self.metrics
    }

    /// Export Hansei lessons learned
    pub fn export_lessons(&self) -> Vec<Lesson> {
        self.reflector.lessons().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunt_config_default() {
        let config = HuntConfig::default();
        assert_eq!(config.max_cycles, 100);
        assert!((config.quality_threshold - 0.85).abs() < f64::EPSILON);
        assert!((config.target_rate - 0.80).abs() < f64::EPSILON);
        assert_eq!(config.plateau_threshold, 5);
        assert!(config.enable_five_whys);
    }

    #[test]
    fn test_hunt_config_custom() {
        let config = HuntConfig {
            max_cycles: 50,
            quality_threshold: 0.90,
            target_rate: 0.95,
            plateau_threshold: 10,
            enable_five_whys: false,
            lessons_database: PathBuf::from("/custom/path"),
        };
        assert_eq!(config.max_cycles, 50);
        assert!((config.quality_threshold - 0.90).abs() < f64::EPSILON);
        assert!((config.target_rate - 0.95).abs() < f64::EPSILON);
        assert_eq!(config.plateau_threshold, 10);
        assert!(!config.enable_five_whys);
        assert_eq!(config.lessons_database.to_str(), Some("/custom/path"));
    }

    #[test]
    fn test_hunt_config_clone() {
        let config = HuntConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.max_cycles, config.max_cycles);
        assert!((cloned.quality_threshold - config.quality_threshold).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hunt_config_debug() {
        let config = HuntConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("HuntConfig"));
        assert!(debug_str.contains("max_cycles"));
        assert!(debug_str.contains("quality_threshold"));
    }

    #[test]
    fn test_hunt_engine_creation() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        assert_eq!(engine.metrics().compilation_rate, 0.0);
        assert_eq!(engine.metrics().cumulative_fixes, 0);
    }

    #[test]
    fn test_hunt_engine_andon_status() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        let status = engine.andon_status();
        // Just verify we can call the method and get a status
        // (the actual status value depends on initial state)
        format!("{:?}", status); // Ensure Debug works
    }

    #[test]
    fn test_hunt_engine_export_lessons_empty() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        let lessons = engine.export_lessons();
        assert!(lessons.is_empty());
    }

    #[test]
    fn test_hunt_engine_with_custom_config() {
        let config = HuntConfig {
            max_cycles: 10,
            quality_threshold: 0.99,
            target_rate: 0.5,
            plateau_threshold: 2,
            enable_five_whys: false,
            lessons_database: PathBuf::from("test.db"),
        };
        let engine = HuntEngine::new(config);
        assert_eq!(engine.metrics().compilation_rate, 0.0);
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_hunt_config_all_fields() {
        let config = HuntConfig::default();
        // Verify all fields are accessible
        assert_eq!(config.max_cycles, 100);
        assert!((config.quality_threshold - 0.85).abs() < f64::EPSILON);
        assert!((config.target_rate - 0.80).abs() < f64::EPSILON);
        assert_eq!(config.plateau_threshold, 5);
        assert!(config.enable_five_whys);
        assert_eq!(config.lessons_database, PathBuf::from(".depyler/lessons.db"));
    }

    #[test]
    fn test_hunt_config_lessons_database_custom() {
        let config = HuntConfig {
            lessons_database: PathBuf::from("/custom/db/lessons.sqlite"),
            ..Default::default()
        };
        assert_eq!(
            config.lessons_database,
            PathBuf::from("/custom/db/lessons.sqlite")
        );
    }

    #[test]
    fn test_hunt_engine_metrics_initial_state() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        let metrics = engine.metrics();

        assert_eq!(metrics.compilation_rate, 0.0);
        assert_eq!(metrics.cumulative_fixes, 0);
        assert_eq!(metrics.cycles_since_improvement, 0);
    }

    #[test]
    fn test_hunt_engine_export_lessons_type() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        let lessons: Vec<Lesson> = engine.export_lessons();

        // Initially empty
        assert!(lessons.is_empty());
        // Verify it's a Vec<Lesson>
        let _: &[Lesson] = &lessons;
    }

    #[test]
    fn test_hunt_engine_andon_status_type() {
        let config = HuntConfig::default();
        let engine = HuntEngine::new(config);
        let status: &AndonStatus = engine.andon_status();

        // Verify it's the right type and can be formatted
        let _debug = format!("{:?}", status);
    }

    #[test]
    fn test_hunt_config_debug_format() {
        let config = HuntConfig {
            max_cycles: 50,
            quality_threshold: 0.95,
            target_rate: 0.90,
            plateau_threshold: 3,
            enable_five_whys: false,
            lessons_database: PathBuf::from("test.db"),
        };

        let debug = format!("{:?}", config);
        assert!(debug.contains("max_cycles"));
        assert!(debug.contains("50"));
        assert!(debug.contains("quality_threshold"));
        assert!(debug.contains("0.95"));
        assert!(debug.contains("target_rate"));
        assert!(debug.contains("0.9"));
        assert!(debug.contains("plateau_threshold"));
        assert!(debug.contains("3"));
        assert!(debug.contains("enable_five_whys"));
        assert!(debug.contains("false"));
        assert!(debug.contains("lessons_database"));
        assert!(debug.contains("test.db"));
    }

    #[test]
    fn test_hunt_config_clone_independence() {
        let original = HuntConfig {
            max_cycles: 25,
            quality_threshold: 0.75,
            target_rate: 0.60,
            plateau_threshold: 8,
            enable_five_whys: true,
            lessons_database: PathBuf::from("original.db"),
        };

        let mut cloned = original.clone();
        cloned.max_cycles = 50;
        cloned.quality_threshold = 0.99;

        // Original unchanged
        assert_eq!(original.max_cycles, 25);
        assert!((original.quality_threshold - 0.75).abs() < f64::EPSILON);

        // Cloned has new values
        assert_eq!(cloned.max_cycles, 50);
        assert!((cloned.quality_threshold - 0.99).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hunt_config_edge_values() {
        // Test with edge/boundary values
        let config = HuntConfig {
            max_cycles: 0,
            quality_threshold: 0.0,
            target_rate: 1.0,
            plateau_threshold: u32::MAX,
            enable_five_whys: false,
            lessons_database: PathBuf::new(),
        };

        assert_eq!(config.max_cycles, 0);
        assert_eq!(config.quality_threshold, 0.0);
        assert_eq!(config.target_rate, 1.0);
        assert_eq!(config.plateau_threshold, u32::MAX);
        assert!(!config.enable_five_whys);
        assert_eq!(config.lessons_database, PathBuf::new());
    }

    #[test]
    fn test_hunt_engine_config_preserved() {
        let config = HuntConfig {
            max_cycles: 42,
            quality_threshold: 0.77,
            target_rate: 0.88,
            plateau_threshold: 7,
            enable_five_whys: true,
            lessons_database: PathBuf::from("preserved.db"),
        };

        let engine = HuntEngine::new(config);

        // Engine should be functional
        assert_eq!(engine.metrics().compilation_rate, 0.0);
        assert!(engine.export_lessons().is_empty());
    }

    #[test]
    fn test_hunt_engine_multiple_instances() {
        let config1 = HuntConfig::default();
        let config2 = HuntConfig {
            max_cycles: 10,
            ..Default::default()
        };

        let engine1 = HuntEngine::new(config1);
        let engine2 = HuntEngine::new(config2);

        // Both engines independent
        assert_eq!(engine1.metrics().compilation_rate, 0.0);
        assert_eq!(engine2.metrics().compilation_rate, 0.0);
    }

    #[test]
    fn test_re_exports_available() {
        // Verify re-exports work
        let _: KaizenMetrics = KaizenMetrics::new();
        let _: HuntPlanner = HuntPlanner::new();
        let _: MinimalReproducer = MinimalReproducer::new();
        let _: JidokaRepairEngine = JidokaRepairEngine::new(0.85);
        let _: AndonVerifier = AndonVerifier::new();
        let _: HanseiReflector = HanseiReflector::new();
        let _: FiveWhysAnalyzer = FiveWhysAnalyzer::new();
    }
}
