//! Semantic error classification module (DEPYLER-REPORT-V2).
//!
//! Classifies transpilation errors by Python domain:
//! - Core: Basic Python features (variables, functions, control flow)
//! - Stdlib: Standard library usage (os, json, datetime, etc.)
//! - External: Third-party packages (numpy, pandas, etc.)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Python domain classification for error sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PythonDomain {
    /// Core Python features (variables, functions, classes, control flow).
    Core,
    /// Python standard library (os, sys, json, datetime, collections, etc.).
    Stdlib,
    /// External/third-party packages (numpy, pandas, requests, etc.).
    External,
}

impl PythonDomain {
    /// Get human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Core => "Core Python (variables, functions, classes)",
            Self::Stdlib => "Python Standard Library (os, json, datetime)",
            Self::External => "External Packages (numpy, pandas, requests)",
        }
    }
}

/// Domain-specific pass rate statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStats {
    /// Total files in this domain.
    pub total: usize,
    /// Files that passed (compiled successfully).
    pub passed: usize,
    /// Pass rate as percentage.
    pub pass_rate: f64,
}

impl DomainStats {
    /// Create new domain stats.
    pub fn new(total: usize, passed: usize) -> Self {
        let pass_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        Self {
            total,
            passed,
            pass_rate,
        }
    }
}

/// Semantic error classification result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticClassification {
    /// Stats by domain.
    pub by_domain: HashMap<PythonDomain, DomainStats>,
    /// File-to-domain mapping.
    pub file_domains: HashMap<String, PythonDomain>,
    /// Overall classification confidence (0.0-1.0).
    pub confidence: f64,
}

/// Semantic classifier that analyzes Python files to determine their domain.
pub struct SemanticClassifier {
    /// Known stdlib modules.
    stdlib_modules: Vec<&'static str>,
    /// Known external packages.
    external_packages: Vec<&'static str>,
}

impl Default for SemanticClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticClassifier {
    /// Create a new semantic classifier with default module lists.
    pub fn new() -> Self {
        Self {
            stdlib_modules: vec![
                // I/O and filesystem
                "os",
                "sys",
                "io",
                "pathlib",
                "shutil",
                "tempfile",
                "glob",
                // Data formats
                "json",
                "csv",
                "xml",
                "html",
                "configparser",
                "pickle",
                "sqlite3",
                // Date/time
                "datetime",
                "time",
                "calendar",
                // Text processing
                "re",
                "string",
                "textwrap",
                "difflib",
                // Math and numbers
                "math",
                "random",
                "statistics",
                "decimal",
                "fractions",
                // Collections and data structures
                "collections",
                "heapq",
                "bisect",
                "array",
                "queue",
                // Functional
                "itertools",
                "functools",
                "operator",
                // Network
                "urllib",
                "http",
                "socket",
                "email",
                // Concurrent
                "threading",
                "multiprocessing",
                "asyncio",
                "subprocess",
                // Other common stdlib
                "typing",
                "enum",
                "dataclasses",
                "abc",
                "copy",
                "pprint",
                "logging",
                "argparse",
                "unittest",
                "hashlib",
                "base64",
                "uuid",
                "contextlib",
            ],
            external_packages: vec![
                // Data science
                "numpy",
                "pandas",
                "scipy",
                "matplotlib",
                "seaborn",
                "sklearn",
                "scikit-learn",
                // Web
                "requests",
                "flask",
                "django",
                "fastapi",
                "aiohttp",
                "httpx",
                // Database
                "sqlalchemy",
                "pymongo",
                "redis",
                "psycopg2",
                // Cloud/Infrastructure
                "boto3",
                "google-cloud",
                "azure",
                // Serialization
                "pydantic",
                "msgpack",
                "protobuf",
                // Testing
                "pytest",
                "hypothesis",
                "mock",
                // Other popular
                "click",
                "typer",
                "pyyaml",
                "toml",
                "pillow",
                "beautifulsoup4",
                "lxml",
                "cryptography",
                "jwt",
            ],
        }
    }

    /// Classify a Python file's domain based on its imports.
    pub fn classify_file(&self, python_source: &str) -> PythonDomain {
        let imports = self.extract_imports(python_source);

        // Check for external packages first (highest priority)
        for import in &imports {
            if self.is_external_package(import) {
                return PythonDomain::External;
            }
        }

        // Check for stdlib usage
        for import in &imports {
            if self.is_stdlib_module(import) {
                return PythonDomain::Stdlib;
            }
        }

        // Default to core Python
        PythonDomain::Core
    }

    /// Classify multiple files and compute domain statistics.
    pub fn classify_corpus(
        &self,
        files: &[(String, String, bool)], // (filename, source, passed)
    ) -> SemanticClassification {
        let mut by_domain: HashMap<PythonDomain, (usize, usize)> = HashMap::new();
        let mut file_domains: HashMap<String, PythonDomain> = HashMap::new();

        for (filename, source, passed) in files {
            let domain = self.classify_file(source);
            file_domains.insert(filename.clone(), domain);

            let entry = by_domain.entry(domain).or_insert((0, 0));
            entry.0 += 1; // total
            if *passed {
                entry.1 += 1; // passed
            }
        }

        let domain_stats: HashMap<PythonDomain, DomainStats> = by_domain
            .into_iter()
            .map(|(domain, (total, passed))| (domain, DomainStats::new(total, passed)))
            .collect();

        // Compute confidence based on how well we could classify
        let total_files = files.len();
        let classified_count = file_domains.len();
        let confidence = if total_files > 0 {
            classified_count as f64 / total_files as f64
        } else {
            1.0
        };

        SemanticClassification {
            by_domain: domain_stats,
            file_domains,
            confidence,
        }
    }

    /// Extract import statements from Python source.
    fn extract_imports(&self, source: &str) -> Vec<String> {
        let mut imports = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();

            // Handle "import x" and "import x, y, z"
            if let Some(rest) = trimmed.strip_prefix("import ") {
                for part in rest.split(',') {
                    let module = part.split_whitespace().next().unwrap_or("");
                    let base = module.split('.').next().unwrap_or(module);
                    if !base.is_empty() {
                        imports.push(base.to_string());
                    }
                }
            }
            // Handle "from x import y"
            else if let Some(rest) = trimmed.strip_prefix("from ") {
                if let Some(module_part) = rest.split(" import").next() {
                    let module = module_part.trim();
                    let base = module.split('.').next().unwrap_or(module);
                    if !base.is_empty() {
                        imports.push(base.to_string());
                    }
                }
            }
        }

        imports
    }

    /// Check if a module is from the Python standard library.
    fn is_stdlib_module(&self, module: &str) -> bool {
        self.stdlib_modules
            .iter()
            .any(|&m| m.eq_ignore_ascii_case(module))
    }

    /// Check if a module is an external package.
    fn is_external_package(&self, module: &str) -> bool {
        self.external_packages
            .iter()
            .any(|&p| p.eq_ignore_ascii_case(module))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_core_python() {
        let classifier = SemanticClassifier::new();

        let source = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

class Calculator:
    def add(self, a: int, b: int) -> int:
        return a + b
"#;

        assert_eq!(classifier.classify_file(source), PythonDomain::Core);
    }

    #[test]
    fn test_classify_stdlib() {
        let classifier = SemanticClassifier::new();

        let source = r#"
import json
import os
from datetime import datetime

def load_config(path: str) -> dict:
    with open(path) as f:
        return json.load(f)
"#;

        assert_eq!(classifier.classify_file(source), PythonDomain::Stdlib);
    }

    #[test]
    fn test_classify_external() {
        let classifier = SemanticClassifier::new();

        let source = r#"
import numpy as np
import pandas as pd
from sklearn.linear_model import LinearRegression

def train_model(data: pd.DataFrame):
    model = LinearRegression()
    return model.fit(data)
"#;

        assert_eq!(classifier.classify_file(source), PythonDomain::External);
    }

    #[test]
    fn test_extract_imports() {
        let classifier = SemanticClassifier::new();

        let source = r#"
import os
import json, sys
from datetime import datetime
from collections.abc import Mapping
import numpy as np
"#;

        let imports = classifier.extract_imports(source);
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"json".to_string()));
        assert!(imports.contains(&"sys".to_string()));
        assert!(imports.contains(&"datetime".to_string()));
        assert!(imports.contains(&"collections".to_string()));
        assert!(imports.contains(&"numpy".to_string()));
    }

    #[test]
    fn test_classify_corpus() {
        let classifier = SemanticClassifier::new();

        let files = vec![
            ("core.py".to_string(), "def hello(): pass".to_string(), true),
            (
                "stdlib.py".to_string(),
                "import json\ndef load(): pass".to_string(),
                true,
            ),
            (
                "external.py".to_string(),
                "import pandas\ndef analyze(): pass".to_string(),
                false,
            ),
        ];

        let result = classifier.classify_corpus(&files);

        assert_eq!(
            result.file_domains.get("core.py"),
            Some(&PythonDomain::Core)
        );
        assert_eq!(
            result.file_domains.get("stdlib.py"),
            Some(&PythonDomain::Stdlib)
        );
        assert_eq!(
            result.file_domains.get("external.py"),
            Some(&PythonDomain::External)
        );

        // Check stats
        let core_stats = result.by_domain.get(&PythonDomain::Core).unwrap();
        assert_eq!(core_stats.total, 1);
        assert_eq!(core_stats.passed, 1);
        assert_eq!(core_stats.pass_rate, 100.0);

        let external_stats = result.by_domain.get(&PythonDomain::External).unwrap();
        assert_eq!(external_stats.total, 1);
        assert_eq!(external_stats.passed, 0);
        assert_eq!(external_stats.pass_rate, 0.0);
    }

    #[test]
    fn test_domain_description() {
        assert!(!PythonDomain::Core.description().is_empty());
        assert!(!PythonDomain::Stdlib.description().is_empty());
        assert!(!PythonDomain::External.description().is_empty());
    }

    #[test]
    fn test_domain_stats_calculation() {
        let stats = DomainStats::new(10, 7);
        assert_eq!(stats.total, 10);
        assert_eq!(stats.passed, 7);
        assert!((stats.pass_rate - 70.0).abs() < 0.01);

        // Edge case: zero total
        let empty_stats = DomainStats::new(0, 0);
        assert_eq!(empty_stats.pass_rate, 0.0);
    }

    #[test]
    fn test_external_priority_over_stdlib() {
        let classifier = SemanticClassifier::new();

        // File that uses both stdlib and external
        let source = r#"
import os
import json
import numpy as np
"#;

        // External should win over stdlib
        assert_eq!(classifier.classify_file(source), PythonDomain::External);
    }

    #[test]
    fn test_empty_file_is_core() {
        let classifier = SemanticClassifier::new();
        assert_eq!(classifier.classify_file(""), PythonDomain::Core);
    }
}
