//! Corpus configuration module.
//!
//! Defines configuration options for corpus analysis.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for corpus analysis.
///
/// # Example
///
/// ```
/// use depyler_corpus::CorpusConfig;
/// use std::path::PathBuf;
///
/// let config = CorpusConfig::default()
///     .with_corpus_path(PathBuf::from("../my-corpus"))
///     .with_output_dir(PathBuf::from("./reports"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusConfig {
    /// Path to the corpus directory containing Python files.
    pub corpus_path: PathBuf,

    /// Name of the corpus (for reporting).
    pub corpus_name: String,

    /// Output directory for generated reports.
    pub output_dir: PathBuf,

    /// Skip artifact cleaning phase.
    pub skip_clean: bool,

    /// Maximum number of parallel workers.
    pub parallel_workers: usize,

    /// Timeout per file in seconds.
    pub timeout_per_file_sec: u64,

    /// File patterns to include.
    pub include_patterns: Vec<String>,

    /// File patterns to exclude.
    pub exclude_patterns: Vec<String>,

    /// Target single-shot compilation rate (for Andon alerts).
    pub target_rate: f64,

    /// Output formats to generate.
    pub output_formats: Vec<OutputFormat>,

    /// Path to depyler binary (None = use PATH).
    pub depyler_path: Option<PathBuf>,
}

/// Output format for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// JSON (machine-readable, deterministic).
    Json,
    /// Markdown (human-readable).
    Markdown,
    /// PNG visualizations.
    Png,
    /// Terminal ASCII output.
    Terminal,
}

impl Default for CorpusConfig {
    fn default() -> Self {
        Self {
            corpus_path: PathBuf::from("../reprorusted-python-cli"),
            corpus_name: "reprorusted-python-cli".to_string(),
            output_dir: PathBuf::from("./reports"),
            skip_clean: false,
            parallel_workers: 4,
            timeout_per_file_sec: 30,
            include_patterns: vec!["examples/**/*.py".to_string()],
            exclude_patterns: vec![
                "**/__pycache__/**".to_string(),
                "**/test_*.py".to_string(),
                "**/__init__.py".to_string(),
            ],
            target_rate: 80.0,
            output_formats: vec![OutputFormat::Json, OutputFormat::Markdown],
            depyler_path: None,
        }
    }
}

impl CorpusConfig {
    /// Create a new config with the specified corpus path.
    pub fn with_corpus_path(mut self, path: PathBuf) -> Self {
        self.corpus_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        self.corpus_path = path;
        self
    }

    /// Set the output directory.
    pub fn with_output_dir(mut self, path: PathBuf) -> Self {
        self.output_dir = path;
        self
    }

    /// Skip the artifact cleaning phase.
    pub fn with_skip_clean(mut self, skip: bool) -> Self {
        self.skip_clean = skip;
        self
    }

    /// Set the target single-shot rate.
    pub fn with_target_rate(mut self, rate: f64) -> Self {
        self.target_rate = rate;
        self
    }

    /// Set the depyler binary path.
    pub fn with_depyler_path(mut self, path: PathBuf) -> Self {
        self.depyler_path = Some(path);
        self
    }

    /// Load configuration from a YAML file.
    pub fn from_yaml_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?; // Use JSON for simplicity
        Ok(config)
    }

    /// Get the effective depyler binary path.
    pub fn depyler_binary(&self) -> PathBuf {
        self.depyler_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("depyler"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CorpusConfig::default();
        assert_eq!(config.corpus_name, "reprorusted-python-cli");
        assert_eq!(config.target_rate, 80.0);
        assert!(!config.skip_clean);
    }

    #[test]
    fn test_config_builder() {
        let config = CorpusConfig::default()
            .with_corpus_path(PathBuf::from("/tmp/test-corpus"))
            .with_target_rate(90.0)
            .with_skip_clean(true);

        assert_eq!(config.corpus_name, "test-corpus");
        assert_eq!(config.target_rate, 90.0);
        assert!(config.skip_clean);
    }

    #[test]
    fn test_output_formats() {
        let config = CorpusConfig::default();
        assert!(config.output_formats.contains(&OutputFormat::Json));
        assert!(config.output_formats.contains(&OutputFormat::Markdown));
    }

    #[test]
    fn test_depyler_binary_default() {
        let config = CorpusConfig::default();
        assert_eq!(config.depyler_binary(), PathBuf::from("depyler"));
    }

    #[test]
    fn test_depyler_binary_custom() {
        let config =
            CorpusConfig::default().with_depyler_path(PathBuf::from("/usr/local/bin/depyler"));
        assert_eq!(
            config.depyler_binary(),
            PathBuf::from("/usr/local/bin/depyler")
        );
    }
}
