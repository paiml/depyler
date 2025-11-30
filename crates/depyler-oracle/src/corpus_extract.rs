//! Corpus extraction for oracle training data
//!
//! This module handles extraction and deduplication of training errors
//! from Python transpilation attempts. Replaces the bash script with
//! type-safe Rust implementation.
//!
//! # Design (Extreme TDD)
//!
//! Tests written FIRST, implementation follows.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// A training error extracted from transpilation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrainingError {
    /// Error code (e.g., "E0308", "TRANS")
    pub error_code: String,
    /// Error message
    pub message: String,
    /// Context (code snippet)
    pub context: String,
    /// Source file path
    pub file: String,
    /// Hash for deduplication (md5 of error_code + message)
    pub hash: String,
    /// Timestamp of extraction
    pub timestamp: String,
    /// Cycle number (for accumulation mode)
    pub cycle: u32,
}

impl TrainingError {
    /// Create a new training error with auto-generated hash
    pub fn new(
        error_code: impl Into<String>,
        message: impl Into<String>,
        context: impl Into<String>,
        file: impl Into<String>,
        cycle: u32,
    ) -> Self {
        let error_code = error_code.into();
        let message = message.into();
        let hash = Self::compute_hash(&error_code, &message);
        Self {
            error_code,
            message,
            context: context.into(),
            file: file.into(),
            hash,
            timestamp: chrono::Utc::now().to_rfc3339(),
            cycle,
        }
    }

    /// Compute deduplication hash from error_code and message
    pub fn compute_hash(error_code: &str, message: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        error_code.hash(&mut hasher);
        message.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }
}

/// Corpus of training errors with deduplication
#[derive(Debug, Default)]
pub struct TrainingCorpus {
    errors: Vec<TrainingError>,
    seen_hashes: HashSet<String>,
}

impl TrainingCorpus {
    /// Create empty corpus
    pub fn new() -> Self {
        Self::default()
    }

    /// Load corpus from JSONL file
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let mut corpus = Self::new();
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            for line in content.lines() {
                if let Ok(error) = serde_json::from_str::<TrainingError>(line) {
                    corpus.insert(error);
                }
            }
        }
        Ok(corpus)
    }

    /// Save corpus to JSONL file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        for error in &self.errors {
            writeln!(file, "{}", serde_json::to_string(error).unwrap())?;
        }
        Ok(())
    }

    /// Insert error if not duplicate (by hash)
    /// Returns true if inserted, false if duplicate
    pub fn insert(&mut self, error: TrainingError) -> bool {
        if self.seen_hashes.contains(&error.hash) {
            false
        } else {
            self.seen_hashes.insert(error.hash.clone());
            self.errors.push(error);
            true
        }
    }

    /// Number of unique errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get all errors
    pub fn errors(&self) -> &[TrainingError] {
        &self.errors
    }

    /// Merge another corpus (deduplicating)
    /// Returns count of new unique errors added
    pub fn merge(&mut self, other: TrainingCorpus) -> usize {
        let before = self.len();
        for error in other.errors {
            self.insert(error);
        }
        self.len() - before
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    // ============================================================
    // TDD RED: Tests written FIRST
    // ============================================================

    #[test]
    fn test_training_error_hash_deterministic() {
        // Same inputs should produce same hash
        let hash1 = TrainingError::compute_hash("E0308", "mismatched types");
        let hash2 = TrainingError::compute_hash("E0308", "mismatched types");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_training_error_hash_different_for_different_inputs() {
        let hash1 = TrainingError::compute_hash("E0308", "mismatched types");
        let hash2 = TrainingError::compute_hash("E0308", "different message");
        let hash3 = TrainingError::compute_hash("E0599", "mismatched types");
        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_training_error_new_generates_hash() {
        let error = TrainingError::new("E0308", "mismatched types", "ctx", "file.py", 0);
        assert!(!error.hash.is_empty());
        assert_eq!(error.hash.len(), 16); // 64-bit hash as hex
    }

    #[test]
    fn test_corpus_deduplicates_by_hash() {
        let mut corpus = TrainingCorpus::new();

        let e1 = TrainingError::new("E0308", "mismatched types", "ctx", "file1.py", 0);
        let e2 = TrainingError::new("E0308", "mismatched types", "ctx", "file2.py", 1); // Same error, different file/cycle
        let e3 = TrainingError::new("E0599", "method not found", "ctx", "file3.py", 0);

        assert!(corpus.insert(e1)); // First insert succeeds
        assert!(!corpus.insert(e2)); // Duplicate hash, rejected
        assert!(corpus.insert(e3)); // Different error, succeeds

        assert_eq!(corpus.len(), 2);
    }

    #[test]
    fn test_corpus_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("corpus.jsonl");

        // Create and save
        let mut corpus = TrainingCorpus::new();
        corpus.insert(TrainingError::new("E0308", "type error", "ctx", "a.py", 0));
        corpus.insert(TrainingError::new("E0599", "method error", "ctx", "b.py", 0));
        corpus.save(&path).unwrap();

        // Load and verify
        let loaded = TrainingCorpus::load(&path).unwrap();
        assert_eq!(loaded.len(), 2);

        // Verify content preserved
        let errors = loaded.errors();
        assert!(errors.iter().any(|e| e.error_code == "E0308"));
        assert!(errors.iter().any(|e| e.error_code == "E0599"));
    }

    #[test]
    fn test_corpus_load_deduplicates_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("corpus.jsonl");

        // Write file with duplicates (simulating broken bash script)
        std::fs::write(
            &path,
            r#"{"error_code":"E0308","message":"type error","context":"","file":"a.py","hash":"abc123","timestamp":"","cycle":0}
{"error_code":"E0308","message":"type error","context":"","file":"a.py","hash":"abc123","timestamp":"","cycle":1}
{"error_code":"E0308","message":"type error","context":"","file":"a.py","hash":"abc123","timestamp":"","cycle":2}
"#,
        )
        .unwrap();

        // Load should dedupe
        let corpus = TrainingCorpus::load(&path).unwrap();
        assert_eq!(corpus.len(), 1); // Deduped to 1
    }

    #[test]
    fn test_corpus_merge_returns_new_count() {
        let mut corpus1 = TrainingCorpus::new();
        corpus1.insert(TrainingError::new("E0308", "error1", "", "", 0));
        corpus1.insert(TrainingError::new("E0599", "error2", "", "", 0));

        let mut corpus2 = TrainingCorpus::new();
        corpus2.insert(TrainingError::new("E0599", "error2", "", "", 1)); // Duplicate
        corpus2.insert(TrainingError::new("E0277", "error3", "", "", 1)); // New

        let new_count = corpus1.merge(corpus2);
        assert_eq!(new_count, 1); // Only error3 was new
        assert_eq!(corpus1.len(), 3);
    }

    #[test]
    fn test_corpus_empty_file_loads_ok() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.jsonl");
        std::fs::write(&path, "").unwrap();

        let corpus = TrainingCorpus::load(&path).unwrap();
        assert!(corpus.is_empty());
    }

    #[test]
    fn test_corpus_nonexistent_file_loads_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.jsonl");

        let corpus = TrainingCorpus::load(&path).unwrap();
        assert!(corpus.is_empty());
    }

    #[test]
    fn test_corpus_handles_malformed_json_lines() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("corpus.jsonl");

        // Mix of valid and invalid lines
        std::fs::write(
            &path,
            r#"{"error_code":"E0308","message":"valid","context":"","file":"","hash":"abc","timestamp":"","cycle":0}
not valid json
{"error_code":"E0599","message":"also valid","context":"","file":"","hash":"def","timestamp":"","cycle":0}
"#,
        )
        .unwrap();

        let corpus = TrainingCorpus::load(&path).unwrap();
        assert_eq!(corpus.len(), 2); // Only valid lines loaded
    }
}
