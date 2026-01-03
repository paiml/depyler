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
}
