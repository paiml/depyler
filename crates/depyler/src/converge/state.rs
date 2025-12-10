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
