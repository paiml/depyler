//! # Pytest Assertion Extractor for CITL Training Pipeline
//!
//! GH-174: Extracts simple `assert` statements from `test_*.py` files
//! as additional CITL training signal.
//!
//! ## Overview
//!
//! Many test files contain simple I/O assertions equivalent to doctests:
//!
//! ```python
//! def test_square():
//!     assert square(4) == 16
//!     assert square(-3) == 9
//! ```
//!
//! This module extracts these patterns into the same format as doctests.
//!
//! ## Scope
//!
//! Extract **only** simple patterns:
//! - `assert f(x) == y`
//! - `assert f(x, y) == z`
//! - `assert f(x) == [a, b, c]`
//!
//! **Ignore** complex patterns:
//! - Fixtures, mocks, parametrize
//! - Exception testing (`pytest.raises`)
//! - Approximate comparisons (`pytest.approx`)

use crate::doctest_extractor::Doctest;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Result of extracting pytest assertions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PytestResult {
    /// Source file
    pub source: String,
    /// Extracted assertions as doctests
    pub assertions: Vec<Doctest>,
}

/// Extracts simple assert statements from pytest files
#[derive(Debug, Clone, Default)]
pub struct PytestExtractor {
    /// Only extract from test_*.py files
    pub strict_test_files: bool,
}

impl PytestExtractor {
    /// Creates a new PytestExtractor with default settings
    pub fn new() -> Self {
        Self {
            strict_test_files: true,
        }
    }

    /// Configure whether to only extract from test_*.py files
    pub fn with_strict_test_files(mut self, strict: bool) -> Self {
        self.strict_test_files = strict;
        self
    }

    /// Extract all simple assertions from Python test source code
    pub fn extract(&self, source: &str) -> Result<Vec<Doctest>> {
        let mut assertions = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut current_function: Option<String> = None;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Track test function definitions
            if trimmed.starts_with("def test_") {
                if let Some(name) = Self::extract_function_name(trimmed) {
                    current_function = Some(name);
                }
            } else if trimmed.starts_with("def ") && !trimmed.starts_with("def test_") {
                // Non-test function, clear context
                current_function = None;
            }

            // Look for simple assert statements
            if trimmed.starts_with("assert ") {
                if let Some(doctest) = self.parse_assert(trimmed, line_num + 1, &current_function) {
                    assertions.push(doctest);
                }
            }
        }

        Ok(assertions)
    }

    /// Extract function name from a def line
    fn extract_function_name(line: &str) -> Option<String> {
        let after_def = line.strip_prefix("def ")?.trim();
        let paren_idx = after_def.find('(')?;
        Some(after_def[..paren_idx].to_string())
    }

    /// Parse an assert statement into a Doctest if it's a simple pattern
    fn parse_assert(
        &self,
        line: &str,
        line_num: usize,
        _current_function: &Option<String>,
    ) -> Option<Doctest> {
        // Remove "assert " prefix
        let assertion = line.strip_prefix("assert ")?.trim();

        // Skip complex patterns
        if self.is_complex_assertion(assertion) {
            return None;
        }

        // Look for == comparison
        let eq_idx = assertion.find(" == ")?;
        let left = assertion[..eq_idx].trim();
        let right = assertion[eq_idx + 4..].trim();

        // Left side should be a function call
        if !left.contains('(') || !left.contains(')') {
            return None;
        }

        // Extract the function being tested (from the call)
        let func_name = self.extract_called_function(left)?;

        // Clean up right side (remove trailing comments, etc.)
        let expected = self.clean_expected(right);

        Some(Doctest {
            function: func_name,
            input: left.to_string(),
            expected,
            line: line_num,
        })
    }

    /// Check if an assertion is too complex to extract
    fn is_complex_assertion(&self, assertion: &str) -> bool {
        // Skip pytest-specific patterns
        if assertion.contains("pytest.") {
            return true;
        }

        // Skip approximate comparisons
        if assertion.contains("approx(") {
            return true;
        }

        // Skip assertions with 'in' operator
        if assertion.contains(" in ") && !assertion.contains(" == ") {
            return true;
        }

        // Skip assertions with 'is' operator (identity checks)
        if assertion.contains(" is ") && !assertion.contains(" == ") {
            return true;
        }

        // Skip assertions with 'not' operator at the start
        if assertion.starts_with("not ") {
            return true;
        }

        // Skip multi-condition assertions
        if assertion.contains(" and ") || assertion.contains(" or ") {
            return true;
        }

        // Skip assertions with lambda
        if assertion.contains("lambda") {
            return true;
        }

        // Skip type checks
        if assertion.contains("isinstance(") || assertion.contains("type(") {
            return true;
        }

        false
    }

    /// Extract the function name being called
    fn extract_called_function(&self, call_expr: &str) -> Option<String> {
        let paren_idx = call_expr.find('(')?;
        let func_part = &call_expr[..paren_idx];

        // Handle method calls like obj.method()
        if let Some(dot_idx) = func_part.rfind('.') {
            Some(func_part[dot_idx + 1..].to_string())
        } else {
            Some(func_part.to_string())
        }
    }

    /// Clean up the expected value
    fn clean_expected(&self, expected: &str) -> String {
        let mut result = expected.to_string();

        // Remove trailing comments
        if let Some(hash_idx) = result.find('#') {
            result = result[..hash_idx].trim().to_string();
        }

        // Remove trailing comma (from tuple unpacking)
        result = result.trim_end_matches(',').trim().to_string();

        result
    }

    /// Extract assertions to the same format as doctest results
    pub fn extract_to_result(&self, source: &str, filename: &str) -> Result<PytestResult> {
        let assertions = self.extract(source)?;
        Ok(PytestResult {
            source: filename.to_string(),
            assertions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RED TESTS - These define the expected behavior (GH-174)
    // =========================================================================

    #[test]
    fn test_extract_simple_assert_eq() {
        let source = r#"
def test_square():
    assert square(4) == 16
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].function, "square");
        assert_eq!(assertions[0].input, "square(4)");
        assert_eq!(assertions[0].expected, "16");
    }

    #[test]
    fn test_extract_multiple_assertions() {
        let source = r#"
def test_square():
    assert square(4) == 16
    assert square(-3) == 9
    assert square(0) == 0
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 3);
        assert_eq!(assertions[0].expected, "16");
        assert_eq!(assertions[1].expected, "9");
        assert_eq!(assertions[2].expected, "0");
    }

    #[test]
    fn test_extract_multiple_args() {
        let source = r#"
def test_add():
    assert add(1, 2) == 3
    assert add(-1, 1) == 0
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 2);
        assert_eq!(assertions[0].input, "add(1, 2)");
        assert_eq!(assertions[0].expected, "3");
    }

    #[test]
    fn test_extract_string_expected() {
        let source = r#"
def test_greet():
    assert greet("World") == "Hello, World!"
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "\"Hello, World!\"");
    }

    #[test]
    fn test_extract_list_expected() {
        let source = r#"
def test_range_list():
    assert range_list(3) == [0, 1, 2]
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "[0, 1, 2]");
    }

    #[test]
    fn test_extract_dict_expected() {
        let source = r#"
def test_make_dict():
    assert make_dict("a", 1) == {"a": 1}
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "{\"a\": 1}");
    }

    #[test]
    fn test_extract_boolean_expected() {
        let source = r#"
def test_is_even():
    assert is_even(4) == True
    assert is_even(3) == False
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 2);
        assert_eq!(assertions[0].expected, "True");
        assert_eq!(assertions[1].expected, "False");
    }

    #[test]
    fn test_skip_pytest_raises() {
        let source = r#"
def test_error():
    with pytest.raises(ValueError):
        divide(1, 0)
    assert divide(10, 2) == 5
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        // Should only extract the simple assertion
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "divide(10, 2)");
    }

    #[test]
    fn test_skip_pytest_approx() {
        let source = r#"
def test_float():
    assert divide(10, 3) == pytest.approx(3.333, rel=0.01)
    assert multiply(2, 3) == 6
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        // Should only extract the exact comparison
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "multiply(2, 3)");
    }

    #[test]
    fn test_skip_complex_and_or() {
        let source = r#"
def test_complex():
    assert foo(1) == 1 and bar(2) == 2
    assert simple(3) == 3
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        // Should only extract the simple assertion
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "simple(3)");
    }

    #[test]
    fn test_skip_isinstance() {
        let source = r#"
def test_types():
    assert isinstance(foo(), int)
    assert bar() == 42
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "bar()");
    }

    #[test]
    fn test_skip_in_operator() {
        let source = r#"
def test_membership():
    assert 1 in get_list()
    assert get_first() == 1
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "get_first()");
    }

    #[test]
    fn test_method_call() {
        let source = r#"
def test_method():
    obj = MyClass()
    assert obj.compute(5) == 25
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].function, "compute");
        assert_eq!(assertions[0].input, "obj.compute(5)");
    }

    #[test]
    fn test_line_numbers() {
        let source = r#"
def test_foo():
    x = 1
    assert foo(1) == 1
    y = 2
    assert foo(2) == 4
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 2);
        assert_eq!(assertions[0].line, 4);
        assert_eq!(assertions[1].line, 6);
    }

    #[test]
    fn test_extract_to_result() {
        let source = r#"
def test_square():
    assert square(4) == 16
"#;

        let extractor = PytestExtractor::new();
        let result = extractor.extract_to_result(source, "test_math.py").unwrap();

        assert_eq!(result.source, "test_math.py");
        assert_eq!(result.assertions.len(), 1);
    }

    #[test]
    fn test_empty_source() {
        let source = "";
        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();
        assert!(assertions.is_empty());
    }

    #[test]
    fn test_no_assertions() {
        let source = r#"
def test_foo():
    x = compute()
    print(x)
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();
        assert!(assertions.is_empty());
    }

    #[test]
    fn test_non_function_call_lhs() {
        let source = r#"
def test_foo():
    assert x == 1
    assert foo() == 2
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        // Should only extract the function call assertion
        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].input, "foo()");
    }

    #[test]
    fn test_trailing_comment() {
        let source = r#"
def test_foo():
    assert foo(1) == 1  # This tests the basic case
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "1");
    }

    #[test]
    fn test_none_expected() {
        let source = r#"
def test_returns_none():
    assert returns_none() == None
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "None");
    }

    #[test]
    fn test_tuple_expected() {
        let source = r#"
def test_tuple():
    assert get_tuple() == (1, 2, 3)
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "(1, 2, 3)");
    }

    #[test]
    fn test_float_expected() {
        let source = r#"
def test_float():
    assert divide(10, 4) == 2.5
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "2.5");
    }

    #[test]
    fn test_negative_number_expected() {
        let source = r#"
def test_negative():
    assert negate(5) == -5
"#;

        let extractor = PytestExtractor::new();
        let assertions = extractor.extract(source).unwrap();

        assert_eq!(assertions.len(), 1);
        assert_eq!(assertions[0].expected, "-5");
    }
}
