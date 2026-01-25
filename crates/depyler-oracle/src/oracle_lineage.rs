//! Oracle Lineage Tracking using Entrenar's ModelLineage (Issue #212)
//!
//! Replaces custom TrainingState with standardized ModelLineage component.
//! Benefits: Gains regression detection, lineage chains, and PAIML stack alignment.

use entrenar::monitor::{ChangeType, ModelLineage, ModelMetadata};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Default lineage file location
const LINEAGE_DIR: &str = ".depyler";
const LINEAGE_FILE: &str = "oracle_lineage.json";

/// Tag keys for oracle-specific metadata
const TAG_COMMIT_SHA: &str = "commit_sha";
const TAG_SAMPLE_COUNT: &str = "sample_count";
const TAG_CORPUS_HASH: &str = "corpus_hash";

/// Oracle lineage tracker using Entrenar's ModelLineage.
///
/// Maps depyler's training state to entrenar's lineage model:
/// - `commit_sha` → `ModelMetadata.tags["commit_sha"]`
/// - `corpus_hash` → `ModelMetadata.config_hash`
/// - `sample_count` → `ModelMetadata.tags["sample_count"]`
/// - `model_version` → `ModelMetadata.version`
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OracleLineage {
    inner: ModelLineage,
}

impl OracleLineage {
    /// Create a new empty lineage tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: ModelLineage::new(),
        }
    }

    /// Get the default lineage file path (`.depyler/oracle_lineage.json` in project root).
    #[must_use]
    pub fn default_lineage_path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        for _ in 0..5 {
            if path.join("Cargo.toml").exists() {
                return path.join(LINEAGE_DIR).join(LINEAGE_FILE);
            }
            if !path.pop() {
                break;
            }
        }
        PathBuf::from(LINEAGE_DIR).join(LINEAGE_FILE)
    }

    /// Load lineage from file, returning empty lineage if not found.
    ///
    /// # Errors
    ///
    /// Returns error if file exists but cannot be parsed.
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(path)?;
        let inner = ModelLineage::from_json(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Self { inner })
    }

    /// Load from default path.
    pub fn load_default() -> Result<Self, std::io::Error> {
        Self::load(&Self::default_lineage_path())
    }

    /// Save lineage to file.
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

        let content = self
            .inner
            .to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }

    /// Save to default path.
    pub fn save_default(&self) -> Result<(), std::io::Error> {
        self.save(&Self::default_lineage_path())
    }

    /// Get the latest model in the lineage (most recently added).
    #[must_use]
    pub fn latest_model(&self) -> Option<&ModelMetadata> {
        self.inner.all_models().max_by_key(|m| m.created_at)
    }

    /// Check if retraining is needed based on current state.
    ///
    /// Returns `true` if:
    /// - No models exist in lineage (never trained)
    /// - Commit SHA has changed (codebase modified)
    /// - Corpus hash has changed (training data modified)
    /// - Model version has changed (architecture updated)
    #[must_use]
    pub fn needs_retraining(&self, current_sha: &str, current_corpus_hash: &str) -> bool {
        let Some(latest) = self.latest_model() else {
            return true; // No models means never trained
        };

        // Check commit SHA change
        let stored_sha = latest
            .tags
            .get(TAG_COMMIT_SHA)
            .map(String::as_str)
            .unwrap_or("");
        if stored_sha != current_sha {
            return true;
        }

        // Check corpus hash change
        if latest.config_hash != current_corpus_hash {
            return true;
        }

        // Check model version change
        let current_version = env!("CARGO_PKG_VERSION");
        if latest.version != current_version {
            return true;
        }

        false
    }

    /// Record a new training run.
    ///
    /// Creates a new model entry and links it to the previous model if exists.
    pub fn record_training(
        &mut self,
        commit_sha: String,
        corpus_hash: String,
        sample_count: usize,
        accuracy: f64,
    ) -> String {
        // Use millisecond-precision timestamp for unique model IDs
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let model_id = format!("oracle-{}-{}", env!("CARGO_PKG_VERSION"), now);

        let mut tags = HashMap::new();
        tags.insert(TAG_COMMIT_SHA.to_string(), commit_sha);
        tags.insert(TAG_SAMPLE_COUNT.to_string(), sample_count.to_string());
        tags.insert(TAG_CORPUS_HASH.to_string(), corpus_hash.clone());

        let metadata = ModelMetadata {
            model_id: model_id.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            accuracy,
            created_at: now,
            config_hash: corpus_hash,
            tags,
        };

        // Get previous model for derivation BEFORE adding new model
        let parent_id = self.latest_model().map(|m| m.model_id.clone());

        // Add new model
        self.inner.add_model(metadata);

        // Add derivation edge if there's a parent
        if let Some(parent) = parent_id {
            self.inner.add_derivation(
                &parent,
                &model_id,
                ChangeType::Retrain,
                "Oracle retrained due to codebase changes",
            );
        }

        model_id
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

    /// Find if the latest training resulted in a regression.
    #[must_use]
    pub fn find_regression(&self) -> Option<(String, f64)> {
        let latest = self.latest_model()?;
        let derivation = self.inner.find_regression_source(&latest.model_id)?;
        let parent = self.inner.get_model(&derivation.parent_id)?;
        let delta = latest.accuracy - parent.accuracy;
        Some((derivation.description.clone(), delta))
    }

    /// Get the training lineage chain for the latest model.
    #[must_use]
    pub fn get_lineage_chain(&self) -> Vec<String> {
        self.latest_model()
            .map(|m| self.inner.get_lineage_chain(&m.model_id))
            .unwrap_or_default()
    }

    /// Get model count in lineage.
    #[must_use]
    pub fn model_count(&self) -> usize {
        self.inner.all_models().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ============================================================
    // RED PHASE: Tests written FIRST before implementation
    // Issue #212: Replace TrainingState with ModelLineage
    // ============================================================

    #[test]
    fn test_oracle_lineage_creation() {
        let lineage = OracleLineage::new();
        assert_eq!(lineage.model_count(), 0);
    }

    #[test]
    fn test_default_lineage_path() {
        let path = OracleLineage::default_lineage_path();
        assert!(path.to_string_lossy().contains(".depyler"));
        assert!(path.to_string_lossy().contains("oracle_lineage.json"));
    }

    #[test]
    fn test_needs_retraining_empty_lineage() {
        let lineage = OracleLineage::new();
        // Empty lineage should always need training
        assert!(lineage.needs_retraining("abc123", "hash456"));
    }

    #[test]
    fn test_needs_retraining_same_state() {
        let mut lineage = OracleLineage::new();
        let sha = "abc123def456789012345678901234567890abcd";
        let hash = "corpus_hash_123";

        lineage.record_training(sha.to_string(), hash.to_string(), 1000, 0.85);

        // Same commit and corpus hash should NOT need retraining
        assert!(!lineage.needs_retraining(sha, hash));
    }

    #[test]
    fn test_needs_retraining_commit_changed() {
        let mut lineage = OracleLineage::new();
        let sha = "abc123def456789012345678901234567890abcd";
        let hash = "corpus_hash_123";

        lineage.record_training(sha.to_string(), hash.to_string(), 1000, 0.85);

        // Different commit SHA should need retraining
        assert!(lineage.needs_retraining("different_sha_789", hash));
    }

    #[test]
    fn test_needs_retraining_corpus_changed() {
        let mut lineage = OracleLineage::new();
        let sha = "abc123def456789012345678901234567890abcd";
        let hash = "corpus_hash_123";

        lineage.record_training(sha.to_string(), hash.to_string(), 1000, 0.85);

        // Different corpus hash should need retraining
        assert!(lineage.needs_retraining(sha, "different_corpus_hash"));
    }

    #[test]
    fn test_record_training_creates_model() {
        let mut lineage = OracleLineage::new();
        assert_eq!(lineage.model_count(), 0);

        let model_id =
            lineage.record_training("abc123".to_string(), "hash456".to_string(), 1000, 0.85);

        assert_eq!(lineage.model_count(), 1);
        assert!(model_id.starts_with("oracle-"));
    }

    #[test]
    fn test_record_training_creates_derivation() {
        let mut lineage = OracleLineage::new();

        // First training
        lineage.record_training("sha1".to_string(), "hash1".to_string(), 1000, 0.80);

        // Small delay to ensure unique timestamps
        std::thread::sleep(std::time::Duration::from_millis(2));

        // Second training
        lineage.record_training("sha2".to_string(), "hash2".to_string(), 1500, 0.85);

        assert_eq!(lineage.model_count(), 2);

        // Lineage chain should have both models
        let chain = lineage.get_lineage_chain();
        assert_eq!(chain.len(), 2);
    }

    #[test]
    fn test_save_load_roundtrip() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let lineage_path = temp_dir.path().join(".depyler").join("oracle_lineage.json");

        let mut lineage = OracleLineage::new();
        lineage.record_training("abc123".to_string(), "hash456".to_string(), 1000, 0.85);

        // Save
        lineage.save(&lineage_path).expect("save should work");
        assert!(lineage_path.exists());

        // Load
        let loaded = OracleLineage::load(&lineage_path).expect("load should work");
        assert_eq!(loaded.model_count(), 1);

        // Verify latest model
        let latest = loaded.latest_model().expect("should have model");
        assert_eq!(latest.tags.get(TAG_COMMIT_SHA), Some(&"abc123".to_string()));
    }

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let lineage_path = temp_dir.path().join("nonexistent.json");

        let loaded = OracleLineage::load(&lineage_path).expect("load should not error");
        assert_eq!(loaded.model_count(), 0);
    }

    #[test]
    fn test_get_current_commit_sha() {
        let sha = OracleLineage::get_current_commit_sha();

        // Should return a non-empty SHA in a git repo
        // (40-character hex string or empty if not in git repo)
        if !sha.is_empty() {
            assert_eq!(sha.len(), 40, "Git SHA should be 40 characters");
            assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_compute_corpus_hash_empty() {
        let hash = OracleLineage::compute_corpus_hash(&[]);
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
        let hash1 = OracleLineage::compute_corpus_hash(&paths);
        let hash2 = OracleLineage::compute_corpus_hash(&paths);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_find_regression_no_regression() {
        let mut lineage = OracleLineage::new();

        // First training
        lineage.record_training("sha1".to_string(), "hash1".to_string(), 1000, 0.80);

        // Small delay to ensure unique timestamps
        std::thread::sleep(std::time::Duration::from_millis(2));

        // Second training with better accuracy (improvement)
        lineage.record_training("sha2".to_string(), "hash2".to_string(), 1500, 0.85);

        // No regression should be found
        assert!(lineage.find_regression().is_none());
    }

    #[test]
    fn test_find_regression_detected() {
        let mut lineage = OracleLineage::new();

        // First training with good accuracy
        lineage.record_training("sha1".to_string(), "hash1".to_string(), 1000, 0.85);

        // Small delay to ensure unique timestamps
        std::thread::sleep(std::time::Duration::from_millis(2));

        // Second training with worse accuracy (regression)
        lineage.record_training("sha2".to_string(), "hash2".to_string(), 1500, 0.75);

        // Verify we have 2 models
        assert_eq!(lineage.model_count(), 2);

        // Verify lineage chain exists
        let chain = lineage.get_lineage_chain();
        assert_eq!(chain.len(), 2, "Should have 2 models in chain");

        // Regression should be detected
        let regression = lineage.find_regression();
        assert!(
            regression.is_some(),
            "Should detect regression: 0.85 -> 0.75"
        );

        let (_, delta) = regression.unwrap();
        assert!(delta < 0.0, "Delta should be negative for regression");
    }

    #[test]
    fn test_latest_model_empty() {
        let lineage = OracleLineage::new();
        assert!(lineage.latest_model().is_none());
    }

    #[test]
    fn test_latest_model_returns_newest() {
        let mut lineage = OracleLineage::new();

        lineage.record_training("sha1".to_string(), "hash1".to_string(), 1000, 0.80);
        std::thread::sleep(std::time::Duration::from_millis(10));
        lineage.record_training("sha2".to_string(), "hash2".to_string(), 1500, 0.85);

        let latest = lineage.latest_model().expect("should have model");
        assert_eq!(latest.tags.get(TAG_COMMIT_SHA), Some(&"sha2".to_string()));
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
            accuracy in 0.0f64..1.0,
        ) {
            let mut lineage = OracleLineage::new();
            lineage.record_training(sha.clone(), hash.clone(), count, accuracy);

            let json = lineage.inner.to_json().unwrap();
            let loaded = ModelLineage::from_json(&json).unwrap();

            let latest = loaded.all_models().next();
            prop_assert!(latest.is_some());
        }

        #[test]
        fn prop_needs_retraining_reflexive(
            sha in "[a-f0-9]{40}",
            hash in "[a-f0-9]{16}",
        ) {
            let mut lineage = OracleLineage::new();
            lineage.record_training(sha.clone(), hash.clone(), 1000, 0.85);
            // Same values should NOT need retraining
            prop_assert!(!lineage.needs_retraining(&sha, &hash));
        }

        #[test]
        fn prop_needs_retraining_different_sha(
            sha1 in "[a-f0-9]{40}",
            sha2 in "[a-f0-9]{40}",
            hash in "[a-f0-9]{16}",
        ) {
            prop_assume!(sha1 != sha2);
            let mut lineage = OracleLineage::new();
            lineage.record_training(sha1, hash.clone(), 1000, 0.85);
            // Different SHA should need retraining
            prop_assert!(lineage.needs_retraining(&sha2, &hash));
        }

        #[test]
        fn prop_needs_retraining_different_hash(
            sha in "[a-f0-9]{40}",
            hash1 in "[a-f0-9]{16}",
            hash2 in "[a-f0-9]{16}",
        ) {
            prop_assume!(hash1 != hash2);
            let mut lineage = OracleLineage::new();
            lineage.record_training(sha.clone(), hash1, 1000, 0.85);
            // Different hash should need retraining
            prop_assert!(lineage.needs_retraining(&sha, &hash2));
        }
    }
}
