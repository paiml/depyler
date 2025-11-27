//! Parameter persistence for optimized generation parameters.
//!
//! This module handles saving and loading optimized `GenerationParams`
//! to/from `~/.depyler/oracle_params.json` for reuse across sessions.

use crate::self_supervised::GenerationParams;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Optimized parameters with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedParams {
    /// The generation parameters
    pub params: GenerationParams,
    /// Fitness score achieved during optimization
    pub fitness: f64,
    /// Number of evaluations used
    pub evaluations: usize,
    /// Whether curriculum learning was used
    pub curriculum: bool,
    /// Timestamp of optimization (Unix epoch seconds)
    pub timestamp: u64,
    /// Version of the optimizer
    pub version: String,
}

impl OptimizedParams {
    /// Create new optimized params from generation params and metadata.
    #[must_use]
    pub fn new(params: GenerationParams, fitness: f64, evaluations: usize, curriculum: bool) -> Self {
        Self {
            params,
            fitness,
            evaluations,
            curriculum,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Default path for oracle parameters file.
///
/// Returns `~/.depyler/oracle_params.json`
#[must_use]
pub fn default_params_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".depyler")
        .join("oracle_params.json")
}

/// Save optimized parameters to disk.
///
/// Creates the `.depyler` directory if it doesn't exist.
///
/// # Errors
///
/// Returns error if directory creation or file writing fails.
pub fn save_params(params: &OptimizedParams, path: Option<&PathBuf>) -> Result<PathBuf, std::io::Error> {
    let path = path.cloned().unwrap_or_else(default_params_path);

    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(params)
        .map_err(std::io::Error::other)?;

    fs::write(&path, json)?;
    Ok(path)
}

/// Load optimized parameters from disk.
///
/// # Errors
///
/// Returns error if file reading or parsing fails.
pub fn load_params(path: Option<&PathBuf>) -> Result<OptimizedParams, std::io::Error> {
    let path = path.cloned().unwrap_or_else(default_params_path);

    let json = fs::read_to_string(&path)?;

    serde_json::from_str(&json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Check if optimized parameters exist at the default or given path.
#[must_use]
pub fn params_exist(path: Option<&PathBuf>) -> bool {
    let path = path.cloned().unwrap_or_else(default_params_path);
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_optimized_params_creation() {
        let params = GenerationParams::default();
        let optimized = OptimizedParams::new(params, 0.85, 100, true);

        assert!((optimized.params.weight_docstring - 0.3).abs() < f64::EPSILON);
        assert!((optimized.fitness - 0.85).abs() < f64::EPSILON);
        assert_eq!(optimized.evaluations, 100);
        assert!(optimized.curriculum);
        assert!(optimized.timestamp > 0);
        assert!(!optimized.version.is_empty());
    }

    #[test]
    fn test_save_and_load_params() {
        let temp_dir = TempDir::new().unwrap();
        let params_path = temp_dir.path().join("oracle_params.json");

        let params = GenerationParams::default();
        let optimized = OptimizedParams::new(params, 0.9, 50, false);

        // Save
        let saved_path = save_params(&optimized, Some(&params_path)).unwrap();
        assert_eq!(saved_path, params_path);
        assert!(params_path.exists());

        // Load
        let loaded = load_params(Some(&params_path)).unwrap();
        assert!((loaded.params.weight_docstring - 0.3).abs() < f64::EPSILON);
        assert!((loaded.fitness - 0.9).abs() < f64::EPSILON);
        assert_eq!(loaded.evaluations, 50);
        assert!(!loaded.curriculum);
    }

    #[test]
    fn test_params_exist() {
        let temp_dir = TempDir::new().unwrap();
        let params_path = temp_dir.path().join("oracle_params.json");

        // Should not exist initially
        assert!(!params_exist(Some(&params_path)));

        // Save params
        let params = GenerationParams::default();
        let optimized = OptimizedParams::new(params, 0.75, 25, true);
        save_params(&optimized, Some(&params_path)).unwrap();

        // Should exist now
        assert!(params_exist(Some(&params_path)));
    }

    #[test]
    fn test_load_nonexistent_params() {
        let temp_dir = TempDir::new().unwrap();
        let params_path = temp_dir.path().join("nonexistent.json");

        let result = load_params(Some(&params_path));
        assert!(result.is_err());
    }

    #[test]
    fn test_default_params_path() {
        let path = default_params_path();
        assert!(path.to_string_lossy().contains(".depyler"));
        assert!(path.to_string_lossy().contains("oracle_params.json"));
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dir").join("params.json");

        let params = GenerationParams::default();
        let optimized = OptimizedParams::new(params, 0.8, 30, false);

        // Should create nested directories
        let result = save_params(&optimized, Some(&nested_path));
        assert!(result.is_ok());
        assert!(nested_path.exists());
    }
}
