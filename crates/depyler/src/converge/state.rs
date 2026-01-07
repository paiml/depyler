//! Convergence state management
//!
//! Tracks the state of the convergence loop including examples, errors,
//! clusters, and applied fixes.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Display mode for convergence output (DEPYLER-CONVERGE-RICH)
/// Mirrors UTOL's DisplayMode for consistency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    /// Rich TUI with progress bars and sparklines
    #[default]
    Rich,
    /// Minimal single-line output (CI-friendly)
    Minimal,
    /// JSON output (automation)
    Json,
    /// No output (silent mode)
    Silent,
}

impl std::str::FromStr for DisplayMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "rich" => Self::Rich,
            "minimal" => Self::Minimal,
            "json" => Self::Json,
            "silent" => Self::Silent,
            _ => Self::Rich, // Default to rich
        })
    }
}

impl DisplayMode {
    /// Parse display mode from string (convenience method)
    pub fn parse(s: &str) -> Self {
        s.parse().unwrap_or_default()
    }
}

/// Configuration for the convergence loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Directory containing Python examples
    pub input_dir: PathBuf,
    /// Target compilation rate (0-100)
    pub target_rate: f64,
    /// Maximum iterations before stopping
    pub max_iterations: usize,
    /// Automatically apply transpiler fixes
    pub auto_fix: bool,
    /// Show what would be fixed without applying
    pub dry_run: bool,
    /// Show detailed progress
    pub verbose: bool,
    /// Minimum confidence for auto-fix
    pub fix_confidence_threshold: f64,
    /// Directory to save/resume state
    pub checkpoint_dir: Option<PathBuf>,
    /// Number of parallel compilation jobs
    pub parallel_jobs: usize,
    /// Display mode (rich, minimal, json, silent)
    #[serde(default)]
    pub display_mode: DisplayMode,
    /// Enable Oracle ML-based error classification
    #[serde(default)]
    pub oracle: bool,
    /// Enable explainability traces for transpiler decisions
    #[serde(default)]
    pub explain: bool,
    /// Enable O(1) compilation cache for unchanged files
    #[serde(default = "default_use_cache")]
    pub use_cache: bool,
}

fn default_use_cache() -> bool {
    true
}

impl ConvergenceConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.target_rate < 0.0 || self.target_rate > 100.0 {
            bail!("target_rate must be between 0 and 100");
        }
        if self.max_iterations == 0 {
            bail!("max_iterations must be > 0");
        }
        if self.fix_confidence_threshold < 0.0 || self.fix_confidence_threshold > 1.0 {
            bail!("fix_confidence_threshold must be between 0 and 1");
        }
        if self.parallel_jobs == 0 {
            bail!("parallel_jobs must be > 0");
        }
        Ok(())
    }
}

/// State of a single example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleState {
    /// Path to the Python file
    pub path: PathBuf,
    /// Whether compilation succeeded
    pub compiles: bool,
    /// Compilation errors (if any)
    pub errors: Vec<String>,
    /// Last compilation time
    pub last_compiled: Option<std::time::SystemTime>,
}

impl ExampleState {
    /// Create new example state
    pub fn new(path: PathBuf, compiles: bool) -> Self {
        Self {
            path,
            compiles,
            errors: vec![],
            last_compiled: None,
        }
    }
}

/// Record of an applied fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFix {
    /// Iteration when fix was applied
    pub iteration: usize,
    /// Error code that was fixed
    pub error_code: String,
    /// Description of the fix
    pub description: String,
    /// File that was modified
    pub file_modified: PathBuf,
    /// Git commit hash (if committed)
    pub commit_hash: Option<String>,
    /// Whether the fix was verified
    pub verified: bool,
}

/// Main convergence state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceState {
    /// Configuration
    pub config: ConvergenceConfig,
    /// Current iteration number
    pub iteration: usize,
    /// Current compilation rate (0-100)
    pub compilation_rate: f64,
    /// State of each example
    pub examples: Vec<ExampleState>,
    /// Error clusters from current iteration
    pub error_clusters: Vec<super::ErrorCluster>,
    /// Fixes that have been applied
    pub fixes_applied: Vec<AppliedFix>,
}

impl ConvergenceState {
    /// Create new state from config
    pub fn new(config: ConvergenceConfig) -> Self {
        Self {
            config,
            iteration: 0,
            compilation_rate: 0.0,
            examples: vec![],
            error_clusters: vec![],
            fixes_applied: vec![],
        }
    }

    /// Update examples from compilation results
    pub fn update_examples(&mut self, results: &[super::CompilationResult]) {
        self.examples = results
            .iter()
            .map(|r| ExampleState {
                path: r.source_file.clone(),
                compiles: r.success,
                errors: r.errors.iter().map(|e| e.message.clone()).collect(),
                last_compiled: Some(std::time::SystemTime::now()),
            })
            .collect();
    }

    /// Update compilation rate based on examples
    pub fn update_compilation_rate(&mut self) {
        if self.examples.is_empty() {
            self.compilation_rate = 0.0;
        } else {
            let passing = self.examples.iter().filter(|e| e.compiles).count();
            self.compilation_rate = (passing as f64 / self.examples.len() as f64) * 100.0;
        }
    }

    /// Save state to checkpoint directory
    pub fn save_checkpoint(&self, dir: &Path) -> Result<()> {
        let checkpoint_file = dir.join("convergence_checkpoint.json");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(checkpoint_file, json)?;
        Ok(())
    }

    /// Load state from checkpoint directory
    pub fn load_checkpoint(dir: &Path) -> Result<Self> {
        let checkpoint_file = dir.join("convergence_checkpoint.json");
        let json = std::fs::read_to_string(checkpoint_file)?;
        let state: Self = serde_json::from_str(&json)?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_mode_from_str() {
        assert_eq!("rich".parse::<DisplayMode>().unwrap(), DisplayMode::Rich);
        assert_eq!("RICH".parse::<DisplayMode>().unwrap(), DisplayMode::Rich);
        assert_eq!("minimal".parse::<DisplayMode>().unwrap(), DisplayMode::Minimal);
        assert_eq!("json".parse::<DisplayMode>().unwrap(), DisplayMode::Json);
        assert_eq!("silent".parse::<DisplayMode>().unwrap(), DisplayMode::Silent);
        assert_eq!("unknown".parse::<DisplayMode>().unwrap(), DisplayMode::Rich);
    }

    #[test]
    fn test_display_mode_parse() {
        assert_eq!(DisplayMode::parse("rich"), DisplayMode::Rich);
        assert_eq!(DisplayMode::parse("minimal"), DisplayMode::Minimal);
        assert_eq!(DisplayMode::parse("json"), DisplayMode::Json);
        assert_eq!(DisplayMode::parse("silent"), DisplayMode::Silent);
        assert_eq!(DisplayMode::parse("invalid"), DisplayMode::Rich);
    }

    #[test]
    fn test_display_mode_default() {
        assert_eq!(DisplayMode::default(), DisplayMode::Rich);
    }

    fn test_config() -> ConvergenceConfig {
        ConvergenceConfig {
            input_dir: PathBuf::from("/tmp/test"),
            target_rate: 80.0,
            max_iterations: 10,
            auto_fix: false,
            dry_run: false,
            verbose: false,
            fix_confidence_threshold: 0.8,
            checkpoint_dir: None,
            parallel_jobs: 4,
            display_mode: DisplayMode::Rich,
            oracle: false,
            explain: false,
            use_cache: true,
        }
    }

    #[test]
    fn test_convergence_config_validate_valid() {
        let config = test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_convergence_config_validate_invalid_target_rate_negative() {
        let mut config = test_config();
        config.target_rate = -1.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_convergence_config_validate_invalid_target_rate_over_100() {
        let mut config = test_config();
        config.target_rate = 101.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_convergence_config_validate_invalid_max_iterations() {
        let mut config = test_config();
        config.max_iterations = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_convergence_config_validate_invalid_confidence_threshold_negative() {
        let mut config = test_config();
        config.fix_confidence_threshold = -0.1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_convergence_config_validate_invalid_confidence_threshold_over_1() {
        let mut config = test_config();
        config.fix_confidence_threshold = 1.1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_convergence_config_validate_invalid_parallel_jobs() {
        let mut config = test_config();
        config.parallel_jobs = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_example_state_new() {
        let state = ExampleState::new(PathBuf::from("test.py"), true);
        assert_eq!(state.path, PathBuf::from("test.py"));
        assert!(state.compiles);
        assert!(state.errors.is_empty());
        assert!(state.last_compiled.is_none());
    }

    #[test]
    fn test_convergence_state_new() {
        let config = test_config();
        let state = ConvergenceState::new(config.clone());
        assert_eq!(state.iteration, 0);
        assert_eq!(state.compilation_rate, 0.0);
        assert!(state.examples.is_empty());
        assert!(state.error_clusters.is_empty());
        assert!(state.fixes_applied.is_empty());
    }

    #[test]
    fn test_convergence_state_update_compilation_rate_empty() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);
        state.update_compilation_rate();
        assert_eq!(state.compilation_rate, 0.0);
    }

    #[test]
    fn test_convergence_state_update_compilation_rate_all_pass() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);
        state.examples = vec![
            ExampleState::new(PathBuf::from("a.py"), true),
            ExampleState::new(PathBuf::from("b.py"), true),
        ];
        state.update_compilation_rate();
        assert_eq!(state.compilation_rate, 100.0);
    }

    #[test]
    fn test_convergence_state_update_compilation_rate_mixed() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);
        state.examples = vec![
            ExampleState::new(PathBuf::from("a.py"), true),
            ExampleState::new(PathBuf::from("b.py"), false),
            ExampleState::new(PathBuf::from("c.py"), true),
            ExampleState::new(PathBuf::from("d.py"), false),
        ];
        state.update_compilation_rate();
        assert_eq!(state.compilation_rate, 50.0);
    }

    #[test]
    fn test_convergence_state_checkpoint_roundtrip() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);
        state.iteration = 5;
        state.compilation_rate = 75.0;
        state.examples = vec![
            ExampleState::new(PathBuf::from("test.py"), true),
        ];

        let temp_dir = tempfile::tempdir().unwrap();
        state.save_checkpoint(temp_dir.path()).unwrap();

        let loaded = ConvergenceState::load_checkpoint(temp_dir.path()).unwrap();
        assert_eq!(loaded.iteration, 5);
        assert_eq!(loaded.compilation_rate, 75.0);
        assert_eq!(loaded.examples.len(), 1);
    }

    #[test]
    fn test_applied_fix_serialization() {
        let fix = AppliedFix {
            iteration: 1,
            error_code: "E0425".to_string(),
            description: "Fixed undefined variable".to_string(),
            file_modified: PathBuf::from("src/lib.rs"),
            commit_hash: Some("abc123".to_string()),
            verified: true,
        };
        let json = serde_json::to_string(&fix).unwrap();
        let parsed: AppliedFix = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.iteration, 1);
        assert_eq!(parsed.error_code, "E0425");
        assert!(parsed.verified);
    }

    // ============================================================================
    // Additional Tests for Coverage
    // ============================================================================

    #[test]
    fn test_applied_fix_without_commit_hash() {
        let fix = AppliedFix {
            iteration: 5,
            error_code: "E0308".to_string(),
            description: "Fixed type mismatch".to_string(),
            file_modified: PathBuf::from("type_mapper.rs"),
            commit_hash: None,
            verified: false,
        };
        let json = serde_json::to_string(&fix).unwrap();
        let parsed: AppliedFix = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.iteration, 5);
        assert!(parsed.commit_hash.is_none());
        assert!(!parsed.verified);
    }

    #[test]
    fn test_example_state_clone() {
        let state = ExampleState::new(PathBuf::from("test.py"), true);
        let cloned = state.clone();
        assert_eq!(state.path, cloned.path);
        assert_eq!(state.compiles, cloned.compiles);
    }

    #[test]
    fn test_example_state_with_errors() {
        let mut state = ExampleState::new(PathBuf::from("error.py"), false);
        state.errors = vec!["E0599: no method".to_string(), "E0308: type mismatch".to_string()];
        state.last_compiled = Some(std::time::SystemTime::now());

        let json = serde_json::to_string(&state).unwrap();
        let parsed: ExampleState = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.errors.len(), 2);
        assert!(!parsed.compiles);
        assert!(parsed.last_compiled.is_some());
    }

    #[test]
    fn test_example_state_debug() {
        let state = ExampleState::new(PathBuf::from("test.py"), true);
        let debug = format!("{:?}", state);
        assert!(debug.contains("test.py"));
        assert!(debug.contains("compiles"));
    }

    #[test]
    fn test_convergence_state_update_examples() {
        use super::super::compiler::{CompilationResult, CompilationError};

        let config = test_config();
        let mut state = ConvergenceState::new(config);

        // Create compilation results
        let results = vec![
            CompilationResult {
                source_file: PathBuf::from("a.py"),
                success: true,
                errors: vec![],
                rust_file: Some(PathBuf::from("a.rs")),
            },
            CompilationResult {
                source_file: PathBuf::from("b.py"),
                success: false,
                errors: vec![
                    CompilationError {
                        code: "E0599".to_string(),
                        message: "no method".to_string(),
                        file: PathBuf::from("b.rs"),
                        line: 10,
                        column: 5,
                    }
                ],
                rust_file: None,
            },
        ];

        state.update_examples(&results);

        assert_eq!(state.examples.len(), 2);
        assert!(state.examples[0].compiles);
        assert!(!state.examples[1].compiles);
        assert_eq!(state.examples[1].errors.len(), 1);
    }

    #[test]
    fn test_convergence_state_clone() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);
        state.iteration = 10;
        state.compilation_rate = 75.5;

        let cloned = state.clone();

        assert_eq!(state.iteration, cloned.iteration);
        assert!((state.compilation_rate - cloned.compilation_rate).abs() < 0.001);
    }

    #[test]
    fn test_convergence_state_debug() {
        let config = test_config();
        let state = ConvergenceState::new(config);
        let debug = format!("{:?}", state);
        assert!(debug.contains("iteration"));
        assert!(debug.contains("compilation_rate"));
    }

    #[test]
    fn test_convergence_config_clone() {
        let config = test_config();
        let cloned = config.clone();
        assert_eq!(config.target_rate, cloned.target_rate);
        assert_eq!(config.max_iterations, cloned.max_iterations);
    }

    #[test]
    fn test_convergence_config_debug() {
        let config = test_config();
        let debug = format!("{:?}", config);
        assert!(debug.contains("target_rate"));
        assert!(debug.contains("max_iterations"));
    }

    #[test]
    fn test_convergence_config_serialization() {
        let config = test_config();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ConvergenceConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.target_rate, parsed.target_rate);
        assert_eq!(config.max_iterations, parsed.max_iterations);
    }

    #[test]
    fn test_convergence_config_with_all_options() {
        let config = ConvergenceConfig {
            input_dir: PathBuf::from("/test"),
            target_rate: 95.0,
            max_iterations: 100,
            auto_fix: true,
            dry_run: true,
            verbose: true,
            fix_confidence_threshold: 0.9,
            checkpoint_dir: Some(PathBuf::from("/checkpoints")),
            parallel_jobs: 8,
            display_mode: DisplayMode::Minimal,
            oracle: true,
            explain: true,
            use_cache: false,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_display_mode_serialization() {
        let modes = vec![
            DisplayMode::Rich,
            DisplayMode::Minimal,
            DisplayMode::Json,
            DisplayMode::Silent,
        ];
        for mode in modes {
            let json = serde_json::to_string(&mode).unwrap();
            let parsed: DisplayMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_display_mode_copy() {
        let mode = DisplayMode::Minimal;
        let copy = mode; // Copy, not move
        assert_eq!(mode, copy);
    }

    #[test]
    fn test_display_mode_debug() {
        assert!(format!("{:?}", DisplayMode::Rich).contains("Rich"));
        assert!(format!("{:?}", DisplayMode::Minimal).contains("Minimal"));
        assert!(format!("{:?}", DisplayMode::Json).contains("Json"));
        assert!(format!("{:?}", DisplayMode::Silent).contains("Silent"));
    }

    #[test]
    fn test_convergence_state_checkpoint_load_nonexistent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = ConvergenceState::load_checkpoint(temp_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_convergence_state_with_error_clusters() {
        use super::super::clusterer::{ErrorCluster, RootCause};

        let config = test_config();
        let mut state = ConvergenceState::new(config);

        state.error_clusters.push(ErrorCluster {
            root_cause: RootCause::TranspilerGap {
                gap_type: "type_inference".to_string(),
                location: "type_mapper.rs".to_string(),
            },
            error_code: "E0308".to_string(),
            examples_blocked: vec![PathBuf::from("a.py"), PathBuf::from("b.py")],
            sample_errors: vec![],
            fix_confidence: 0.85,
            suggested_fix: None,
        });

        assert_eq!(state.error_clusters.len(), 1);
        assert_eq!(state.error_clusters[0].examples_blocked.len(), 2);
    }

    #[test]
    fn test_convergence_state_with_fixes() {
        let config = test_config();
        let mut state = ConvergenceState::new(config);

        state.fixes_applied.push(AppliedFix {
            iteration: 1,
            error_code: "E0599".to_string(),
            description: "Added missing method".to_string(),
            file_modified: PathBuf::from("expr_gen.rs"),
            commit_hash: Some("abc123".to_string()),
            verified: true,
        });

        state.fixes_applied.push(AppliedFix {
            iteration: 3,
            error_code: "E0308".to_string(),
            description: "Fixed type inference".to_string(),
            file_modified: PathBuf::from("type_mapper.rs"),
            commit_hash: None,
            verified: false,
        });

        assert_eq!(state.fixes_applied.len(), 2);

        // Checkpoint roundtrip with fixes
        let temp_dir = tempfile::tempdir().unwrap();
        state.save_checkpoint(temp_dir.path()).unwrap();

        let loaded = ConvergenceState::load_checkpoint(temp_dir.path()).unwrap();
        assert_eq!(loaded.fixes_applied.len(), 2);
    }

    #[test]
    fn test_convergence_config_edge_case_rates() {
        // Test boundary values
        let mut config = test_config();

        // 0% target rate
        config.target_rate = 0.0;
        assert!(config.validate().is_ok());

        // 100% target rate
        config.target_rate = 100.0;
        assert!(config.validate().is_ok());

        // Exact 0.0 and 1.0 confidence threshold
        config.target_rate = 80.0;
        config.fix_confidence_threshold = 0.0;
        assert!(config.validate().is_ok());

        config.fix_confidence_threshold = 1.0;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_default_use_cache() {
        // Test that default_use_cache function returns true
        assert!(default_use_cache());
    }

    #[test]
    fn test_applied_fix_clone() {
        let fix = AppliedFix {
            iteration: 1,
            error_code: "E0599".to_string(),
            description: "test".to_string(),
            file_modified: PathBuf::from("test.rs"),
            commit_hash: None,
            verified: true,
        };
        let cloned = fix.clone();
        assert_eq!(fix.iteration, cloned.iteration);
        assert_eq!(fix.verified, cloned.verified);
    }

    #[test]
    fn test_applied_fix_debug() {
        let fix = AppliedFix {
            iteration: 1,
            error_code: "E0599".to_string(),
            description: "test fix".to_string(),
            file_modified: PathBuf::from("test.rs"),
            commit_hash: Some("abc".to_string()),
            verified: true,
        };
        let debug = format!("{:?}", fix);
        assert!(debug.contains("iteration"));
        assert!(debug.contains("E0599"));
    }
}
