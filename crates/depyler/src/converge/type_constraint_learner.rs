//! E0308 Type Constraint Learning (DEPYLER-1101)
//!
//! Enhanced Oracle Loop Phase 1: Parse E0308 error messages to extract
//! type constraints and learn from compiler feedback.
//!
//! Strategic recommendation from DEPYLER-1100:
//! - Parse `expected X, found Y` messages systematically
//! - Store learned type constraints
//! - Re-transpile with learned constraints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A learned type constraint from E0308 error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeConstraint {
    /// Variable or expression that needs the constraint
    pub target: String,
    /// Expected type (from rustc)
    pub expected_type: String,
    /// Found type (what we generated)
    pub found_type: String,
    /// Source file where constraint was learned
    pub source_file: PathBuf,
    /// Line number in generated Rust
    pub line: usize,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Type constraint store for learning from E0308 errors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeConstraintStore {
    /// Variable-to-type constraints keyed by (file, variable_name)
    pub variable_constraints: HashMap<(String, String), TypeConstraint>,
    /// Expression pattern constraints (pattern -> expected_type)
    pub pattern_constraints: HashMap<String, String>,
    /// Statistics
    pub stats: ConstraintStats,
}

/// Statistics about learned constraints
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConstraintStats {
    /// Total E0308 errors parsed
    pub total_parsed: usize,
    /// Successfully extracted constraints
    pub constraints_extracted: usize,
    /// Constraints that improved compilation
    pub constraints_effective: usize,
}

impl TypeConstraintStore {
    /// Create a new empty store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a constraint to the store
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        let key = (
            constraint.source_file.to_string_lossy().to_string(),
            constraint.target.clone(),
        );
        self.variable_constraints.insert(key, constraint);
        self.stats.constraints_extracted += 1;
    }

    /// Get constraint for a variable in a file
    pub fn get_constraint(&self, file: &str, variable: &str) -> Option<&TypeConstraint> {
        self.variable_constraints
            .get(&(file.to_string(), variable.to_string()))
    }

    /// Get all constraints for a file
    pub fn get_file_constraints(&self, file: &str) -> Vec<&TypeConstraint> {
        self.variable_constraints
            .iter()
            .filter(|((f, _), _)| f == file)
            .map(|(_, c)| c)
            .collect()
    }

    /// Save to JSON file
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from JSON file
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let store = serde_json::from_str(&json)?;
        Ok(store)
    }
}

/// Parse E0308 error message to extract type constraint
///
/// E0308 format: "expected `X`, found `Y`" or "mismatched types"
/// Example: "expected `f64`, found `{integer}`"
/// Example: "expected `Option<String>`, found `String`"
pub fn parse_e0308_constraint(
    error_message: &str,
    source_file: &Path,
    line: usize,
) -> Option<TypeConstraint> {
    // Pattern 1: "expected `X`, found `Y`"
    if let Some((expected, found)) = extract_expected_found(error_message) {
        // Try to extract variable name from error message
        let target =
            extract_target_variable(error_message).unwrap_or_else(|| "unknown".to_string());

        return Some(TypeConstraint {
            target,
            expected_type: normalize_type(&expected),
            found_type: normalize_type(&found),
            source_file: source_file.to_path_buf(),
            line,
            confidence: 0.9,
        });
    }

    None
}

/// Extract "expected X, found Y" from error message
fn extract_expected_found(message: &str) -> Option<(String, String)> {
    // Pattern: "expected `X`, found `Y`"
    let expected_start = message.find("expected `")?;
    let expected_end = message[expected_start + 10..].find('`')?;
    let expected = message[expected_start + 10..expected_start + 10 + expected_end].to_string();

    let found_start = message.find("found `")?;
    let found_end = message[found_start + 7..].find('`')?;
    let found = message[found_start + 7..found_start + 7 + found_end].to_string();

    Some((expected, found))
}

/// Extract target variable name from error context
fn extract_target_variable(message: &str) -> Option<String> {
    // Pattern: look for variable name before ":"
    // Example: "let x: i32 = ..." -> "x"

    // Try to find "for `variable`" pattern
    if let Some(start) = message.find("for `") {
        if let Some(end) = message[start + 5..].find('`') {
            return Some(message[start + 5..start + 5 + end].to_string());
        }
    }

    // Try "in expression `variable`"
    if let Some(start) = message.find("in expression `") {
        if let Some(end) = message[start + 15..].find('`') {
            return Some(message[start + 15..start + 15 + end].to_string());
        }
    }

    None
}

/// Normalize Rust type names for consistency
fn normalize_type(type_str: &str) -> String {
    let mut result = type_str.to_string();

    // Normalize integer literals
    if result == "{integer}" {
        result = "i64".to_string();
    }
    if result == "{float}" {
        result = "f64".to_string();
    }

    // Remove lifetime annotations for comparison
    result = result.replace("'static ", "");
    result = result.replace("'a ", "");
    result = result.replace("'_ ", "");

    result
}

/// Batch parse E0308 errors from compiler output
pub fn parse_e0308_batch(
    errors: &[(String, String, usize)], // (code, message, line)
    source_file: &Path,
) -> Vec<TypeConstraint> {
    errors
        .iter()
        .filter(|(code, _, _)| code == "E0308")
        .filter_map(|(_, message, line)| parse_e0308_constraint(message, source_file, *line))
        .collect()
}

/// Analyze constraint patterns to identify common type mismatches
pub fn analyze_constraint_patterns(constraints: &[TypeConstraint]) -> HashMap<String, usize> {
    let mut patterns: HashMap<String, usize> = HashMap::new();

    for c in constraints {
        let pattern = format!("{} -> {}", c.found_type, c.expected_type);
        *patterns.entry(pattern).or_insert(0) += 1;
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_e0308_expected_found_basic() {
        let message = "expected `f64`, found `{integer}`";
        let result = extract_expected_found(message);
        assert!(result.is_some());
        let (expected, found) = result.unwrap();
        assert_eq!(expected, "f64");
        assert_eq!(found, "{integer}");
    }

    #[test]
    fn test_parse_e0308_expected_found_complex() {
        let message = "expected `Option<String>`, found `String`";
        let result = extract_expected_found(message);
        assert!(result.is_some());
        let (expected, found) = result.unwrap();
        assert_eq!(expected, "Option<String>");
        assert_eq!(found, "String");
    }

    #[test]
    fn test_parse_e0308_constraint() {
        let message = "expected `f64`, found `{integer}`";
        let source = PathBuf::from("test.rs");
        let result = parse_e0308_constraint(message, &source, 42);
        assert!(result.is_some());
        let constraint = result.unwrap();
        assert_eq!(constraint.expected_type, "f64");
        assert_eq!(constraint.found_type, "i64"); // normalized
        assert_eq!(constraint.line, 42);
    }

    #[test]
    fn test_normalize_type_integer() {
        assert_eq!(normalize_type("{integer}"), "i64");
        assert_eq!(normalize_type("{float}"), "f64");
    }

    #[test]
    fn test_normalize_type_lifetimes() {
        assert_eq!(normalize_type("'static str"), "str");
        assert_eq!(normalize_type("'a Vec<T>"), "Vec<T>");
    }

    #[test]
    fn test_type_constraint_store_basic() {
        let mut store = TypeConstraintStore::new();
        let constraint = TypeConstraint {
            target: "x".to_string(),
            expected_type: "f64".to_string(),
            found_type: "i32".to_string(),
            source_file: PathBuf::from("test.rs"),
            line: 10,
            confidence: 0.9,
        };
        store.add_constraint(constraint.clone());

        let retrieved = store.get_constraint("test.rs", "x");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().expected_type, "f64");
    }

    #[test]
    fn test_type_constraint_store_stats() {
        let mut store = TypeConstraintStore::new();
        assert_eq!(store.stats.constraints_extracted, 0);

        store.add_constraint(TypeConstraint {
            target: "x".to_string(),
            expected_type: "f64".to_string(),
            found_type: "i32".to_string(),
            source_file: PathBuf::from("test.rs"),
            line: 10,
            confidence: 0.9,
        });

        assert_eq!(store.stats.constraints_extracted, 1);
    }

    #[test]
    fn test_parse_e0308_batch() {
        let errors = vec![
            (
                "E0308".to_string(),
                "expected `f64`, found `{integer}`".to_string(),
                10,
            ),
            ("E0599".to_string(), "no method named `foo`".to_string(), 20),
            (
                "E0308".to_string(),
                "expected `String`, found `&str`".to_string(),
                30,
            ),
        ];
        let source = PathBuf::from("test.rs");
        let constraints = parse_e0308_batch(&errors, &source);

        assert_eq!(constraints.len(), 2); // Only E0308 errors
        assert_eq!(constraints[0].expected_type, "f64");
        assert_eq!(constraints[1].expected_type, "String");
    }

    #[test]
    fn test_analyze_constraint_patterns() {
        let constraints = vec![
            TypeConstraint {
                target: "x".to_string(),
                expected_type: "f64".to_string(),
                found_type: "i32".to_string(),
                source_file: PathBuf::from("a.rs"),
                line: 1,
                confidence: 0.9,
            },
            TypeConstraint {
                target: "y".to_string(),
                expected_type: "f64".to_string(),
                found_type: "i32".to_string(),
                source_file: PathBuf::from("b.rs"),
                line: 2,
                confidence: 0.9,
            },
            TypeConstraint {
                target: "z".to_string(),
                expected_type: "String".to_string(),
                found_type: "&str".to_string(),
                source_file: PathBuf::from("c.rs"),
                line: 3,
                confidence: 0.9,
            },
        ];

        let patterns = analyze_constraint_patterns(&constraints);
        assert_eq!(patterns.get("i32 -> f64"), Some(&2));
        assert_eq!(patterns.get("&str -> String"), Some(&1));
    }

    #[test]
    fn test_extract_target_variable_for_pattern() {
        let message = "expected `f64`, found `i32` for `count`";
        let result = extract_target_variable(message);
        assert_eq!(result, Some("count".to_string()));
    }

    #[test]
    fn test_get_file_constraints() {
        let mut store = TypeConstraintStore::new();
        store.add_constraint(TypeConstraint {
            target: "x".to_string(),
            expected_type: "f64".to_string(),
            found_type: "i32".to_string(),
            source_file: PathBuf::from("test.rs"),
            line: 10,
            confidence: 0.9,
        });
        store.add_constraint(TypeConstraint {
            target: "y".to_string(),
            expected_type: "String".to_string(),
            found_type: "&str".to_string(),
            source_file: PathBuf::from("test.rs"),
            line: 20,
            confidence: 0.9,
        });
        store.add_constraint(TypeConstraint {
            target: "z".to_string(),
            expected_type: "bool".to_string(),
            found_type: "i32".to_string(),
            source_file: PathBuf::from("other.rs"),
            line: 5,
            confidence: 0.9,
        });

        let test_constraints = store.get_file_constraints("test.rs");
        assert_eq!(test_constraints.len(), 2);

        let other_constraints = store.get_file_constraints("other.rs");
        assert_eq!(other_constraints.len(), 1);
    }
}
