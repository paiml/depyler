//! Training state tracking for continuous oracle retraining.
//!
//! Issue #211: Tracks codebase changes to trigger automatic retraining
//! when the corpus or model architecture changes.
//!
//! **DEPRECATED**: This module is superseded by `oracle_lineage` (Issue #212).
//! Use `OracleLineage` from `entrenar::monitor::ModelLineage` instead.
//! Benefits: lineage chains, regression detection, PAIML stack alignment.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Tracks the state of oracle training for change detection.
///
/// Stored in `.depyler/oracle_state.json` to persist across sessions.
///
/// # Deprecated
///
/// Use [`OracleLineage`](crate::oracle_lineage::OracleLineage) instead.
/// This struct is maintained for backward compatibility.
#[deprecated(since = "3.22.0", note = "Use OracleLineage instead (Issue #212)")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrainingState {
    /// Git commit SHA when the oracle was last trained
    pub last_trained_commit_sha: String,
    /// Hash of the training corpus content
    pub corpus_hash: String,
    /// Timestamp of last training (ISO 8601)
    pub last_trained_at: String,
    /// Number of samples used in training
    pub sample_count: usize,
    /// Model version for architecture changes
    pub model_version: String,
}

#[allow(deprecated)]
impl Default for TrainingState {
    fn default() -> Self {
        Self {
            last_trained_commit_sha: String::new(),
            corpus_hash: String::new(),
            last_trained_at: String::new(),
            sample_count: 0,
            model_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Default state file location
const STATE_DIR: &str = ".depyler";
const STATE_FILE: &str = "oracle_state.json";

#[allow(deprecated)]
impl TrainingState {
    /// Create a new training state with current values.
    #[must_use]
    pub fn new(commit_sha: String, corpus_hash: String, sample_count: usize) -> Self {
        Self {
            last_trained_commit_sha: commit_sha,
            corpus_hash,
            last_trained_at: chrono::Utc::now().to_rfc3339(),
            sample_count,
            model_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Get the default state file path (`.depyler/oracle_state.json` in project root).
    #[must_use]
    pub fn default_state_path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        for _ in 0..5 {
            if path.join("Cargo.toml").exists() {
                return path.join(STATE_DIR).join(STATE_FILE);
            }
            if !path.pop() {
                break;
            }
        }
        PathBuf::from(STATE_DIR).join(STATE_FILE)
    }

    /// Load training state from file, returning None if not found.
    ///
    /// # Errors
    ///
    /// Returns error if file exists but cannot be parsed.
    pub fn load(path: &Path) -> Result<Option<Self>, std::io::Error> {
        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(path)?;
        let state: Self = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(state))
    }

    /// Load from default path.
    pub fn load_default() -> Result<Option<Self>, std::io::Error> {
        Self::load(&Self::default_state_path())
    }

    /// Save training state to file.
    ///
    /// Creates the parent directory if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns error if file cannot be written.
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }

    /// Save to default path.
    pub fn save_default(&self) -> Result<(), std::io::Error> {
        self.save(&Self::default_state_path())
    }

    /// Check if retraining is needed based on current state.
    ///
    /// Returns `true` if:
    /// - Commit SHA has changed (codebase modified)
    /// - Corpus hash has changed (training data modified)
    /// - Model version has changed (architecture updated)
    #[must_use]
    pub fn needs_retraining(&self, current_sha: &str, current_corpus_hash: &str) -> bool {
        // Empty state means never trained
        if self.last_trained_commit_sha.is_empty() {
            return true;
        }

        // Check commit SHA change
        if self.last_trained_commit_sha != current_sha {
            return true;
        }

        // Check corpus hash change
        if self.corpus_hash != current_corpus_hash {
            return true;
        }

        // Check model version change
        let current_version = env!("CARGO_PKG_VERSION");
        if self.model_version != current_version {
            return true;
        }

        false
    }

    /// Get the current git HEAD commit SHA.
    ///
    /// Returns empty string if not in a git repo.
    #[must_use]
    pub fn get_current_commit_sha() -> String {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    /// Compute a hash of the training corpus for change detection.
    ///
    /// Uses a simple hash of corpus file paths and modification times.
    #[must_use]
    pub fn compute_corpus_hash(corpus_paths: &[PathBuf]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        for path in corpus_paths {
            path.hash(&mut hasher);
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    modified.hash(&mut hasher);
                }
            }
        }

        format!("{:016x}", hasher.finish())
    }
}

#[cfg(test)]
#[allow(deprecated)] // Testing deprecated TrainingState for backward compatibility
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ============================================================
    // RED PHASE: Tests written FIRST before implementation
    // ============================================================

    #[test]
    fn test_training_state_creation() {
        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        assert_eq!(state.last_trained_commit_sha, "abc123");
        assert_eq!(state.corpus_hash, "hash456");
        assert_eq!(state.sample_count, 1000);
        assert!(!state.last_trained_at.is_empty());
        assert!(!state.model_version.is_empty());
    }

    #[test]
    fn test_training_state_default() {
        let state = TrainingState::default();

        assert!(state.last_trained_commit_sha.is_empty());
        assert!(state.corpus_hash.is_empty());
        assert!(state.last_trained_at.is_empty());
        assert_eq!(state.sample_count, 0);
        assert!(!state.model_version.is_empty());
    }

    #[test]
    fn test_training_state_serialization() {
        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        let json = serde_json::to_string(&state).expect("serialization should work");
        let deserialized: TrainingState =
            serde_json::from_str(&json).expect("deserialization should work");

        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_training_state_save_load() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let state_path = temp_dir.path().join(".depyler").join("oracle_state.json");

        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        // Save
        state.save(&state_path).expect("save should work");
        assert!(state_path.exists());

        // Load
        let loaded = TrainingState::load(&state_path)
            .expect("load should work")
            .expect("state should exist");

        assert_eq!(state.last_trained_commit_sha, loaded.last_trained_commit_sha);
        assert_eq!(state.corpus_hash, loaded.corpus_hash);
        assert_eq!(state.sample_count, loaded.sample_count);
    }

    #[test]
    fn test_training_state_load_nonexistent() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let state_path = temp_dir.path().join("nonexistent.json");

        let loaded = TrainingState::load(&state_path).expect("load should not error");
        assert!(loaded.is_none());
    }

    #[test]
    fn test_needs_retraining_empty_state() {
        let state = TrainingState::default();

        assert!(state.needs_retraining("abc123", "hash456"));
    }

    #[test]
    fn test_needs_retraining_same_state() {
        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        // Same commit and corpus hash should NOT need retraining
        assert!(!state.needs_retraining("abc123", "hash456"));
    }

    #[test]
    fn test_needs_retraining_commit_changed() {
        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        // Different commit SHA should need retraining
        assert!(state.needs_retraining("def789", "hash456"));
    }

    #[test]
    fn test_needs_retraining_corpus_changed() {
        let state = TrainingState::new(
            "abc123".to_string(),
            "hash456".to_string(),
            1000,
        );

        // Different corpus hash should need retraining
        assert!(state.needs_retraining("abc123", "different_hash"));
    }

    #[test]
    fn test_get_current_commit_sha() {
        let sha = TrainingState::get_current_commit_sha();

        // Should return a non-empty SHA in a git repo
        // (40-character hex string or empty if not in git repo)
        if !sha.is_empty() {
            assert_eq!(sha.len(), 40, "Git SHA should be 40 characters");
            assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_compute_corpus_hash_empty() {
        let hash = TrainingState::compute_corpus_hash(&[]);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16, "Hash should be 16 hex characters");
    }

    #[test]
    fn test_compute_corpus_hash_deterministic() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        std::fs::write(&file1, "content1").expect("write file1");
        std::fs::write(&file2, "content2").expect("write file2");

        let paths = vec![file1.clone(), file2.clone()];
        let hash1 = TrainingState::compute_corpus_hash(&paths);
        let hash2 = TrainingState::compute_corpus_hash(&paths);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_compute_corpus_hash_order_matters() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        std::fs::write(&file1, "content1").expect("write file1");
        std::fs::write(&file2, "content2").expect("write file2");

        let hash_ab = TrainingState::compute_corpus_hash(&[file1.clone(), file2.clone()]);
        let hash_ba = TrainingState::compute_corpus_hash(&[file2, file1]);

        // Order should matter (different hashes)
        assert_ne!(hash_ab, hash_ba);
    }

    #[test]
    fn test_default_state_path() {
        let path = TrainingState::default_state_path();
        assert!(path.to_string_lossy().contains(".depyler"));
        assert!(path.to_string_lossy().contains("oracle_state.json"));
    }

    // ============================================================
    // Property-based tests
    // ============================================================

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_serialization_roundtrip(
            sha in "[a-f0-9]{40}",
            hash in "[a-f0-9]{16}",
            count in 0usize..100000,
        ) {
            let state = TrainingState::new(sha.clone(), hash.clone(), count);
            let json = serde_json::to_string(&state).unwrap();
            let deserialized: TrainingState = serde_json::from_str(&json).unwrap();

            prop_assert_eq!(state.last_trained_commit_sha, deserialized.last_trained_commit_sha);
            prop_assert_eq!(state.corpus_hash, deserialized.corpus_hash);
            prop_assert_eq!(state.sample_count, deserialized.sample_count);
        }

        #[test]
        fn prop_needs_retraining_reflexive(
            sha in "[a-f0-9]{40}",
            hash in "[a-f0-9]{16}",
        ) {
            let state = TrainingState::new(sha.clone(), hash.clone(), 1000);
            // Same values should NOT need retraining
            prop_assert!(!state.needs_retraining(&sha, &hash));
        }

        #[test]
        fn prop_needs_retraining_different_sha(
            sha1 in "[a-f0-9]{40}",
            sha2 in "[a-f0-9]{40}",
            hash in "[a-f0-9]{16}",
        ) {
            prop_assume!(sha1 != sha2);
            let state = TrainingState::new(sha1, hash.clone(), 1000);
            // Different SHA should need retraining
            prop_assert!(state.needs_retraining(&sha2, &hash));
        }

        #[test]
        fn prop_needs_retraining_different_hash(
            sha in "[a-f0-9]{40}",
            hash1 in "[a-f0-9]{16}",
            hash2 in "[a-f0-9]{16}",
        ) {
            prop_assume!(hash1 != hash2);
            let state = TrainingState::new(sha.clone(), hash1, 1000);
            // Different hash should need retraining
            prop_assert!(state.needs_retraining(&sha, &hash2));
        }
    }
}
