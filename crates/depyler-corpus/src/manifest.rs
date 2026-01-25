//! Report manifest module.
//!
//! Generates deterministic manifest files for reproducibility verification.

use blake3::Hasher;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Report manifest for determinism verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportManifest {
    /// Manifest version.
    pub version: String,
    /// Execution metadata.
    pub execution: ExecutionMetadata,
    /// Corpus metadata.
    pub corpus: CorpusMetadata,
    /// Generated artifacts.
    pub artifacts: Vec<ArtifactInfo>,
    /// Checksum information.
    pub checksums: ChecksumInfo,
}

/// Execution metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Execution timestamp (ISO 8601).
    pub timestamp: DateTime<Utc>,
    /// Machine identifier (anonymous hash).
    pub machine_id: String,
    /// Depyler version.
    pub depyler_version: String,
    /// Rust compiler version.
    pub rust_version: String,
}

/// Corpus metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusMetadata {
    /// Corpus name.
    pub name: String,
    /// Corpus path.
    pub path: String,
    /// Number of Python files.
    pub python_files: usize,
    /// Hash of all source files.
    pub source_hash: String,
}

/// Artifact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactInfo {
    /// Artifact path.
    pub path: String,
    /// BLAKE3 hash.
    pub hash: String,
    /// MIME type.
    pub format: String,
}

/// Checksum information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksumInfo {
    /// Hash algorithm used.
    pub algorithm: String,
    /// Hash of the manifest itself (excluding this field).
    pub manifest_hash: String,
}

impl ReportManifest {
    /// Create a new manifest.
    pub fn new(corpus_name: &str, corpus_path: &Path, python_file_count: usize) -> Self {
        Self {
            version: "1.0.0".to_string(),
            execution: ExecutionMetadata {
                timestamp: Utc::now(),
                machine_id: Self::compute_machine_id(),
                depyler_version: env!("CARGO_PKG_VERSION").to_string(),
                rust_version: Self::get_rust_version(),
            },
            corpus: CorpusMetadata {
                name: corpus_name.to_string(),
                path: corpus_path.to_string_lossy().to_string(),
                python_files: python_file_count,
                source_hash: String::new(), // Computed later
            },
            artifacts: Vec::new(),
            checksums: ChecksumInfo {
                algorithm: "BLAKE3".to_string(),
                manifest_hash: String::new(), // Computed at finalization
            },
        }
    }

    /// Compute the source hash from Python files.
    pub fn compute_source_hash(
        &mut self,
        python_files: &[std::path::PathBuf],
    ) -> anyhow::Result<()> {
        let mut hasher = Hasher::new();

        // Sort files for deterministic ordering
        let mut sorted_files = python_files.to_vec();
        sorted_files.sort();

        for file in &sorted_files {
            if let Ok(content) = std::fs::read(file) {
                hasher.update(&content);
            }
        }

        self.corpus.source_hash = format!("blake3:{}", hasher.finalize().to_hex());
        Ok(())
    }

    /// Add an artifact to the manifest.
    pub fn add_artifact(&mut self, path: &Path, content: &[u8], format: &str) {
        let hash = Self::compute_hash(content);
        self.artifacts.push(ArtifactInfo {
            path: path.to_string_lossy().to_string(),
            hash,
            format: format.to_string(),
        });
    }

    /// Finalize the manifest by computing its own hash.
    pub fn finalize(&mut self) {
        // Create a copy without the manifest_hash field for hashing
        let mut temp = self.clone();
        temp.checksums.manifest_hash = String::new();

        let json = serde_json::to_string(&temp).unwrap_or_default();
        self.checksums.manifest_hash = Self::compute_hash(json.as_bytes());
    }

    /// Compute BLAKE3 hash of content.
    pub fn compute_hash(content: &[u8]) -> String {
        let hash = blake3::hash(content);
        format!("blake3:{}", hash.to_hex())
    }

    fn compute_machine_id() -> String {
        // Use hostname + username hash for anonymous identification
        let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
        let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let input = format!("{hostname}:{user}");
        let hash = blake3::hash(input.as_bytes());
        format!("sha256:{}", &hash.to_hex()[..16]) // Truncated for privacy
    }

    fn get_rust_version() -> String {
        std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Serialize to JSON.
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Serialize to YAML-like format (using JSON for simplicity).
    pub fn to_yaml(&self) -> anyhow::Result<String> {
        // For simplicity, use JSON with different formatting
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_manifest_creation() {
        let manifest = ReportManifest::new("test-corpus", Path::new("/tmp/test"), 100);

        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.corpus.name, "test-corpus");
        assert_eq!(manifest.corpus.python_files, 100);
        assert!(!manifest.execution.machine_id.is_empty());
    }

    #[test]
    fn test_compute_hash() {
        let hash = ReportManifest::compute_hash(b"hello world");
        assert!(hash.starts_with("blake3:"));
        assert!(hash.len() > 10);
    }

    #[test]
    fn test_add_artifact() {
        let mut manifest = ReportManifest::new("test", Path::new("."), 0);
        manifest.add_artifact(
            Path::new("report.json"),
            b"test content",
            "application/json",
        );

        assert_eq!(manifest.artifacts.len(), 1);
        assert_eq!(manifest.artifacts[0].path, "report.json");
        assert!(manifest.artifacts[0].hash.starts_with("blake3:"));
    }

    #[test]
    fn test_finalize() {
        let mut manifest = ReportManifest::new("test", Path::new("."), 0);
        manifest.finalize();

        assert!(!manifest.checksums.manifest_hash.is_empty());
        assert!(manifest.checksums.manifest_hash.starts_with("blake3:"));
    }

    #[test]
    fn test_source_hash_determinism() {
        let dir = TempDir::new().unwrap();
        let file1 = dir.path().join("a.py");
        let file2 = dir.path().join("b.py");
        std::fs::write(&file1, "print('a')").unwrap();
        std::fs::write(&file2, "print('b')").unwrap();

        let files = vec![file1.clone(), file2.clone()];

        // Compute hash twice
        let mut manifest1 = ReportManifest::new("test", dir.path(), 2);
        manifest1.compute_source_hash(&files).unwrap();

        let mut manifest2 = ReportManifest::new("test", dir.path(), 2);
        manifest2.compute_source_hash(&files).unwrap();

        // Should be identical
        assert_eq!(manifest1.corpus.source_hash, manifest2.corpus.source_hash);
    }

    #[test]
    fn test_to_json() {
        let manifest = ReportManifest::new("test", Path::new("."), 0);
        let json = manifest.to_json().unwrap();

        assert!(json.contains("\"version\""));
        assert!(json.contains("\"execution\""));
        assert!(json.contains("\"corpus\""));
    }
}
