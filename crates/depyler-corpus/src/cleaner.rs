//! Artifact cleaner module (Phase 1).
//!
//! Implements the 5S methodology for deterministic artifact clearing:
//! - 整理 (Seiri) - Sort: Identify artifacts
//! - 整頓 (Seiton) - Set in Order: Record state
//! - 清掃 (Seiso) - Shine: Clean artifacts
//! - 清潔 (Seiketsu) - Standardize: Verify clean state
//! - 躾 (Shitsuke) - Sustain: Maintain determinism

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Result of the cleaning operation.
#[derive(Debug, Clone, Default)]
pub struct CleanResult {
    /// Number of .rs files removed.
    pub rs_files_removed: usize,
    /// Number of Cargo.toml files removed.
    pub cargo_toml_removed: usize,
    /// Number of Cargo.lock files removed.
    pub cargo_lock_removed: usize,
    /// Number of target directories removed.
    pub target_dirs_removed: usize,
    /// Whether the clean state was verified.
    pub verified: bool,
}

/// Artifact cleaner for corpus directories.
///
/// Removes all generated artifacts to ensure deterministic analysis.
pub struct ArtifactCleaner {
    corpus_path: PathBuf,
}

impl ArtifactCleaner {
    /// Create a new artifact cleaner for the given corpus path.
    pub fn new(corpus_path: &Path) -> Self {
        Self {
            corpus_path: corpus_path.to_path_buf(),
        }
    }

    /// Execute the full 5S cleaning protocol.
    ///
    /// # Errors
    ///
    /// Returns an error if cleaning or verification fails.
    pub fn clean(&self) -> Result<CleanResult> {
        let mut result = CleanResult::default();

        // 整理 (Seiri) - Sort: Identify artifacts
        let rs_files = self.find_rs_files()?;
        let cargo_tomls = self.find_cargo_tomls()?;
        let cargo_locks = self.find_cargo_locks()?;
        let target_dirs = self.find_target_dirs()?;

        // 整頓 (Seiton) - Set in Order: Record state (logging)
        tracing::info!(
            "Found artifacts: {} .rs, {} Cargo.toml, {} Cargo.lock, {} target/",
            rs_files.len(),
            cargo_tomls.len(),
            cargo_locks.len(),
            target_dirs.len()
        );

        // 清掃 (Seiso) - Shine: Clean artifacts
        for file in &rs_files {
            std::fs::remove_file(file)
                .with_context(|| format!("Failed to remove {}", file.display()))?;
            result.rs_files_removed += 1;
        }

        for file in &cargo_tomls {
            std::fs::remove_file(file)
                .with_context(|| format!("Failed to remove {}", file.display()))?;
            result.cargo_toml_removed += 1;
        }

        for file in &cargo_locks {
            std::fs::remove_file(file)
                .with_context(|| format!("Failed to remove {}", file.display()))?;
            result.cargo_lock_removed += 1;
        }

        for dir in &target_dirs {
            std::fs::remove_dir_all(dir)
                .with_context(|| format!("Failed to remove {}", dir.display()))?;
            result.target_dirs_removed += 1;
        }

        // 清潔 (Seiketsu) - Standardize: Verify clean state
        result.verified = self.verify_clean_state()?;

        if !result.verified {
            anyhow::bail!("Clean state verification failed");
        }

        Ok(result)
    }

    /// Find all .rs files in the corpus.
    pub fn find_rs_files(&self) -> Result<Vec<PathBuf>> {
        self.find_files_by_extension("rs")
    }

    /// Find all Cargo.toml files in the corpus.
    pub fn find_cargo_tomls(&self) -> Result<Vec<PathBuf>> {
        self.find_files_by_name("Cargo.toml")
    }

    /// Find all Cargo.lock files in the corpus.
    pub fn find_cargo_locks(&self) -> Result<Vec<PathBuf>> {
        self.find_files_by_name("Cargo.lock")
    }

    /// Find all target directories in the corpus.
    pub fn find_target_dirs(&self) -> Result<Vec<PathBuf>> {
        let mut dirs = Vec::new();
        for entry in WalkDir::new(&self.corpus_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() && entry.file_name() == "target" {
                dirs.push(entry.path().to_path_buf());
            }
        }
        Ok(dirs)
    }

    /// Verify that no generated artifacts remain.
    pub fn verify_clean_state(&self) -> Result<bool> {
        let rs_count = self.find_rs_files()?.len();
        let cargo_count = self.find_cargo_tomls()?.len();

        Ok(rs_count == 0 && cargo_count == 0)
    }

    /// Count total artifacts (for reporting).
    pub fn count_artifacts(&self) -> Result<(usize, usize, usize, usize)> {
        Ok((
            self.find_rs_files()?.len(),
            self.find_cargo_tomls()?.len(),
            self.find_cargo_locks()?.len(),
            self.find_target_dirs()?.len(),
        ))
    }

    fn find_files_by_extension(&self, ext: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.corpus_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(file_ext) = entry.path().extension() {
                    if file_ext == ext {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
        Ok(files)
    }

    fn find_files_by_name(&self, name: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.corpus_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.file_name().to_str() == Some(name) {
                files.push(entry.path().to_path_buf());
            }
        }
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_corpus() -> TempDir {
        let dir = TempDir::new().unwrap();

        // Create some Python files
        std::fs::write(dir.path().join("example.py"), "print('hello')").unwrap();

        // Create some Rust artifacts
        std::fs::write(dir.path().join("example.rs"), "fn main() {}").unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        std::fs::write(dir.path().join("Cargo.lock"), "").unwrap();

        // Create a target directory
        std::fs::create_dir(dir.path().join("target")).unwrap();
        std::fs::write(dir.path().join("target/debug"), "").unwrap();

        dir
    }

    #[test]
    fn test_find_rs_files() {
        let dir = create_test_corpus();
        let cleaner = ArtifactCleaner::new(dir.path());

        let rs_files = cleaner.find_rs_files().unwrap();
        assert_eq!(rs_files.len(), 1);
        assert!(rs_files[0].ends_with("example.rs"));
    }

    #[test]
    fn test_find_cargo_tomls() {
        let dir = create_test_corpus();
        let cleaner = ArtifactCleaner::new(dir.path());

        let cargo_files = cleaner.find_cargo_tomls().unwrap();
        assert_eq!(cargo_files.len(), 1);
    }

    #[test]
    fn test_find_target_dirs() {
        let dir = create_test_corpus();
        let cleaner = ArtifactCleaner::new(dir.path());

        let target_dirs = cleaner.find_target_dirs().unwrap();
        assert_eq!(target_dirs.len(), 1);
    }

    #[test]
    fn test_clean_removes_artifacts() {
        let dir = create_test_corpus();
        let cleaner = ArtifactCleaner::new(dir.path());

        // Verify artifacts exist before cleaning
        let (rs, cargo, lock, target) = cleaner.count_artifacts().unwrap();
        assert_eq!(rs, 1);
        assert_eq!(cargo, 1);
        assert_eq!(lock, 1);
        assert_eq!(target, 1);

        // Clean
        let result = cleaner.clean().unwrap();

        // Verify results
        assert_eq!(result.rs_files_removed, 1);
        assert_eq!(result.cargo_toml_removed, 1);
        assert_eq!(result.cargo_lock_removed, 1);
        assert_eq!(result.target_dirs_removed, 1);
        assert!(result.verified);

        // Verify clean state
        let (rs, cargo, lock, target) = cleaner.count_artifacts().unwrap();
        assert_eq!(rs, 0);
        assert_eq!(cargo, 0);
        assert_eq!(lock, 0);
        assert_eq!(target, 0);

        // Python file should still exist
        assert!(dir.path().join("example.py").exists());
    }

    #[test]
    fn test_verify_clean_state_empty_dir() {
        let dir = TempDir::new().unwrap();
        let cleaner = ArtifactCleaner::new(dir.path());

        assert!(cleaner.verify_clean_state().unwrap());
    }

    #[test]
    fn test_verify_clean_state_with_artifacts() {
        let dir = create_test_corpus();
        let cleaner = ArtifactCleaner::new(dir.path());

        assert!(!cleaner.verify_clean_state().unwrap());
    }
}
