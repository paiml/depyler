//! Corpus registry module.
//!
//! Defines known corpora for convergence testing and validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A registered corpus for convergence testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusEntry {
    /// Human-readable name.
    pub name: String,
    /// Description of the corpus.
    pub description: String,
    /// Path to the corpus directory.
    pub path: PathBuf,
    /// Include patterns for Python files.
    pub include: Vec<String>,
    /// Exclude patterns.
    pub exclude: Vec<String>,
    /// GitHub repository URL.
    pub github: Option<String>,
    /// Approximate file count.
    pub file_count: Option<usize>,
    /// Target single-shot compilation rate.
    pub target_rate: f64,
    /// TDG score (if measured).
    pub tdg_score: Option<f64>,
    /// Quality grade (A, B, C, etc.).
    pub grade: Option<String>,
    /// Number of tests.
    pub tests: Option<usize>,
    /// Test coverage percentage.
    pub coverage: Option<f64>,
}

impl CorpusEntry {
    /// Create a new corpus entry.
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            path,
            include: vec!["**/*.py".to_string()],
            exclude: vec![
                "**/__pycache__/**".to_string(),
                "**/test_*.py".to_string(),
                "**/__init__.py".to_string(),
            ],
            github: None,
            file_count: None,
            target_rate: 80.0,
            tdg_score: None,
            grade: None,
            tests: None,
            coverage: None,
        }
    }

    /// Set the description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set the GitHub URL.
    pub fn with_github(mut self, url: &str) -> Self {
        self.github = Some(url.to_string());
        self
    }

    /// Set quality metrics.
    pub fn with_quality(mut self, tdg: f64, grade: &str, tests: usize, coverage: f64) -> Self {
        self.tdg_score = Some(tdg);
        self.grade = Some(grade.to_string());
        self.tests = Some(tests);
        self.coverage = Some(coverage);
        self
    }

    /// Check if the corpus path exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

/// Registry of known corpora.
#[derive(Debug, Clone, Default)]
pub struct CorpusRegistry {
    corpora: HashMap<String, CorpusEntry>,
}

impl CorpusRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            corpora: HashMap::new(),
        }
    }

    /// Create a registry with built-in known corpora.
    #[must_use]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Corpus 1: reprorusted-python-cli (Original)
        let mut entry1 = CorpusEntry::new(
            "reprorusted-python-cli",
            PathBuf::from("/home/noah/src/reprorusted-python-cli"),
        )
        .with_description("Original Python CLI examples - mixed type annotations")
        .with_github("https://github.com/paiml/reprorusted-python-cli");
        entry1.file_count = Some(601);
        entry1.include = vec!["examples/**/*.py".to_string()];
        registry.register(entry1);

        // Corpus 2: reprorusted-std-only (Stdlib)
        let mut entry2 = CorpusEntry::new(
            "reprorusted-std-only",
            PathBuf::from("/home/noah/src/reprorusted-std-only"),
        )
        .with_description("Python stdlib examples - std-only transpilation targets")
        .with_github("https://github.com/paiml/reprorusted-std-only")
        .with_quality(91.9, "A", 182, 100.0);
        entry2.file_count = Some(1382);
        entry2.include = vec!["src/**/*.py".to_string()];
        registry.register(entry2);

        // Corpus 3: fully-typed-reprorusted-python-cli (Typed CLI)
        let mut entry3 = CorpusEntry::new(
            "fully-typed-reprorusted-python-cli",
            PathBuf::from("/home/noah/src/fully-typed-reprorusted-python-cli"),
        )
        .with_description("Fully typed Python CLI utilities - strict type annotations")
        .with_github("https://github.com/paiml/fully-typed-reprorusted-python-cli")
        .with_quality(90.5, "A", 152, 100.0);
        entry3.file_count = Some(1839);
        entry3.include = vec!["src/**/*.py".to_string()];
        registry.register(entry3);

        registry
    }

    /// Register a corpus.
    pub fn register(&mut self, entry: CorpusEntry) {
        self.corpora.insert(entry.name.clone(), entry);
    }

    /// Get a corpus by name.
    pub fn get(&self, name: &str) -> Option<&CorpusEntry> {
        self.corpora.get(name)
    }

    /// List all registered corpora.
    pub fn list(&self) -> Vec<&CorpusEntry> {
        let mut entries: Vec<_> = self.corpora.values().collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// List only corpora that exist on disk.
    pub fn list_available(&self) -> Vec<&CorpusEntry> {
        self.list().into_iter().filter(|e| e.exists()).collect()
    }

    /// Get total file count across all corpora.
    pub fn total_files(&self) -> usize {
        self.corpora
            .values()
            .filter_map(|e| e.file_count)
            .sum()
    }

    /// Load registry from TOML file.
    ///
    /// # Errors
    ///
    /// Returns error if the file cannot be read or parsed.
    pub fn from_toml(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let parsed: toml::Value = toml::from_str(&content)?;

        let mut registry = Self::new();

        if let Some(corpora) = parsed.get("corpora").and_then(|v| v.as_table()) {
            for (name, value) in corpora {
                if let Ok(entry) = parse_corpus_entry(name, value) {
                    registry.register(entry);
                }
            }
        }

        Ok(registry)
    }

    /// Save registry to TOML file.
    ///
    /// # Errors
    ///
    /// Returns error if the file cannot be written.
    pub fn to_toml(&self, path: &Path) -> anyhow::Result<()> {
        let mut content = String::from(
            "# Depyler Corpus Registry\n\
             #\n\
             # Known corpora for convergence testing and validation.\n\
             # Use: depyler converge --corpus <name>\n\n\
             [corpora]\n\n",
        );

        for entry in self.list() {
            content.push_str(&format!("[corpora.{}]\n", entry.name));
            content.push_str(&format!("name = \"{}\"\n", entry.name));
            content.push_str(&format!("description = \"{}\"\n", entry.description));
            content.push_str(&format!("path = \"{}\"\n", entry.path.display()));
            content.push_str(&format!("include = {:?}\n", entry.include));
            content.push_str(&format!("exclude = {:?}\n", entry.exclude));

            if let Some(ref github) = entry.github {
                content.push_str(&format!("github = \"{github}\"\n"));
            }
            if let Some(count) = entry.file_count {
                content.push_str(&format!("file_count = {count}\n"));
            }
            content.push_str(&format!("target_rate = {}\n", entry.target_rate));
            if let Some(tdg) = entry.tdg_score {
                content.push_str(&format!("tdg_score = {tdg}\n"));
            }
            if let Some(ref grade) = entry.grade {
                content.push_str(&format!("grade = \"{grade}\"\n"));
            }
            if let Some(tests) = entry.tests {
                content.push_str(&format!("tests = {tests}\n"));
            }
            if let Some(coverage) = entry.coverage {
                content.push_str(&format!("coverage = {coverage}\n"));
            }
            content.push('\n');
        }

        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Parse a corpus entry from TOML value.
fn parse_corpus_entry(name: &str, value: &toml::Value) -> anyhow::Result<CorpusEntry> {
    let table = value
        .as_table()
        .ok_or_else(|| anyhow::anyhow!("Expected table for corpus {name}"))?;

    let path = table
        .get("path")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut entry = CorpusEntry::new(name, path);

    if let Some(desc) = table.get("description").and_then(|v| v.as_str()) {
        entry.description = desc.to_string();
    }

    if let Some(github) = table.get("github").and_then(|v| v.as_str()) {
        entry.github = Some(github.to_string());
    }

    if let Some(include) = table.get("include").and_then(|v| v.as_array()) {
        entry.include = include
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    if let Some(exclude) = table.get("exclude").and_then(|v| v.as_array()) {
        entry.exclude = exclude
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();
    }

    if let Some(count) = table.get("file_count").and_then(|v| v.as_integer()) {
        entry.file_count = Some(count as usize);
    }

    if let Some(rate) = table.get("target_rate").and_then(|v| v.as_float()) {
        entry.target_rate = rate;
    }

    if let Some(tdg) = table.get("tdg_score").and_then(|v| v.as_float()) {
        entry.tdg_score = Some(tdg);
    }

    if let Some(grade) = table.get("grade").and_then(|v| v.as_str()) {
        entry.grade = Some(grade.to_string());
    }

    if let Some(tests) = table.get("tests").and_then(|v| v.as_integer()) {
        entry.tests = Some(tests as usize);
    }

    if let Some(coverage) = table.get("coverage").and_then(|v| v.as_float()) {
        entry.coverage = Some(coverage);
    }

    Ok(entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus_entry_new() {
        let entry = CorpusEntry::new("test", PathBuf::from("/tmp/test"));
        assert_eq!(entry.name, "test");
        assert_eq!(entry.target_rate, 80.0);
    }

    #[test]
    fn test_corpus_entry_with_quality() {
        let entry = CorpusEntry::new("test", PathBuf::from("/tmp"))
            .with_quality(91.9, "A", 182, 100.0);

        assert_eq!(entry.tdg_score, Some(91.9));
        assert_eq!(entry.grade, Some("A".to_string()));
        assert_eq!(entry.tests, Some(182));
        assert_eq!(entry.coverage, Some(100.0));
    }

    #[test]
    fn test_registry_with_defaults() {
        let registry = CorpusRegistry::with_defaults();
        assert_eq!(registry.corpora.len(), 3);

        assert!(registry.get("reprorusted-python-cli").is_some());
        assert!(registry.get("reprorusted-std-only").is_some());
        assert!(registry.get("fully-typed-reprorusted-python-cli").is_some());
    }

    #[test]
    fn test_registry_list() {
        let registry = CorpusRegistry::with_defaults();
        let list = registry.list();
        assert_eq!(list.len(), 3);

        // Should be sorted by name
        assert_eq!(list[0].name, "fully-typed-reprorusted-python-cli");
        assert_eq!(list[1].name, "reprorusted-python-cli");
        assert_eq!(list[2].name, "reprorusted-std-only");
    }

    #[test]
    fn test_registry_get() {
        let registry = CorpusRegistry::with_defaults();

        let entry = registry.get("reprorusted-std-only").unwrap();
        assert_eq!(entry.tdg_score, Some(91.9));
        assert_eq!(entry.grade, Some("A".to_string()));
    }

    #[test]
    fn test_corpus_entry_exists() {
        let entry = CorpusEntry::new("nonexistent", PathBuf::from("/nonexistent/path"));
        assert!(!entry.exists());

        let entry = CorpusEntry::new("tmp", PathBuf::from("/tmp"));
        assert!(entry.exists());
    }
}
