//! # Doctest Extractor for CITL Training Pipeline
//!
//! GH-173: Extracts Python `>>>` doctest examples and converts them to
//! structured format for CITL training and Rust doc test generation.
//!
//! ## Overview
//!
//! This module implements Phase 1 of the doctest transpilation spec:
//! - Parse Python docstrings to extract `>>>` blocks
//! - Extract input expression and expected output
//! - Handle multi-line continuations (`...`)
//! - Output structured format: `{function, input, expected, line}`
//!
//! ## Example
//!
//! ```rust
//! use depyler_core::doctest_extractor::{DoctestExtractor, Doctest};
//!
//! let source = r#"
//! def square(x: int) -> int:
//!     """Compute square.
//!
//!     >>> square(4)
//!     16
//!     >>> square(-3)
//!     9
//!     """
//!     return x * x
//! "#;
//!
//! let extractor = DoctestExtractor::new();
//! let doctests = extractor.extract(source).unwrap();
//!
//! assert_eq!(doctests.len(), 2);
//! assert_eq!(doctests[0].input, "square(4)");
//! assert_eq!(doctests[0].expected, "16");
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A single extracted doctest example
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Doctest {
    /// The function this doctest belongs to
    pub function: String,
    /// The input expression (after `>>>`)
    pub input: String,
    /// The expected output
    pub expected: String,
    /// Line number in source file
    pub line: usize,
}

/// Result of extracting doctests from a module
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DoctestResult {
    /// Source file or module name
    pub source: String,
    /// Module path (e.g., "os.path")
    pub module: String,
    /// Extracted doctests grouped by function
    pub doctests: Vec<FunctionDoctests>,
}

/// Doctests for a single function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoctests {
    /// Function name
    pub function: String,
    /// Function signature if available
    pub signature: Option<String>,
    /// Docstring text
    pub docstring: Option<String>,
    /// Extracted doctest examples
    pub examples: Vec<Doctest>,
}

/// Extracts doctest examples from Python source code
#[derive(Debug, Clone, Default)]
pub struct DoctestExtractor {
    /// Whether to include module-level doctests
    pub include_module_doctests: bool,
    /// Whether to include class method doctests
    pub include_class_methods: bool,
}

impl DoctestExtractor {
    /// Creates a new DoctestExtractor with default settings
    pub fn new() -> Self {
        Self {
            include_module_doctests: true,
            include_class_methods: true,
        }
    }

    /// Configure whether to include module-level doctests
    pub fn with_module_doctests(mut self, include: bool) -> Self {
        self.include_module_doctests = include;
        self
    }

    /// Configure whether to include class method doctests
    pub fn with_class_methods(mut self, include: bool) -> Self {
        self.include_class_methods = include;
        self
    }

    /// Extract all doctests from Python source code
    pub fn extract(&self, source: &str) -> Result<Vec<Doctest>> {
        let mut doctests = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut current_function: Option<String> = None;
        let mut in_docstring = false;
        let mut docstring_delim: Option<&str> = None;
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // Track function definitions
            if trimmed.starts_with("def ") {
                if let Some(name) = Self::extract_function_name(trimmed) {
                    current_function = Some(name);
                }
            }

            // Track docstring boundaries
            if !in_docstring {
                if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
                    in_docstring = true;
                    docstring_delim = Some(if trimmed.starts_with("\"\"\"") {
                        "\"\"\""
                    } else {
                        "'''"
                    });
                    // Check if docstring ends on same line
                    let rest = &trimmed[3..];
                    if rest.contains(docstring_delim.unwrap()) {
                        in_docstring = false;
                        docstring_delim = None;
                    }
                }
            } else {
                // Inside docstring - look for >>> lines
                if trimmed.starts_with(">>>") {
                    let (doctest, consumed) =
                        self.parse_doctest(&lines, i, current_function.as_deref())?;
                    if let Some(dt) = doctest {
                        doctests.push(dt);
                    }
                    i += consumed.saturating_sub(1);
                }

                // Check for docstring end
                if let Some(delim) = docstring_delim {
                    if trimmed.ends_with(delim) && trimmed.len() >= 3 {
                        in_docstring = false;
                        docstring_delim = None;
                    }
                }
            }

            i += 1;
        }

        Ok(doctests)
    }

    /// Extract function name from a def line
    fn extract_function_name(line: &str) -> Option<String> {
        // "def function_name(args):" -> "function_name"
        let after_def = line.strip_prefix("def ")?.trim();
        let paren_idx = after_def.find('(')?;
        Some(after_def[..paren_idx].to_string())
    }

    /// Parse a single doctest starting at the given line
    fn parse_doctest(
        &self,
        lines: &[&str],
        start_line: usize,
        function: Option<&str>,
    ) -> Result<(Option<Doctest>, usize)> {
        let first_line = lines
            .get(start_line)
            .ok_or_else(|| anyhow::anyhow!("Invalid line index: {}", start_line))?;

        let trimmed = first_line.trim();
        if !trimmed.starts_with(">>>") {
            return Ok((None, 1));
        }

        // Extract input expression (may span multiple lines with ...)
        let mut input = trimmed
            .strip_prefix(">>> ")
            .unwrap_or(&trimmed[3..])
            .to_string();
        let mut consumed = 1;
        let mut next_idx = start_line + 1;

        // Handle multi-line input with ... continuation
        while next_idx < lines.len() {
            let next_line = lines[next_idx].trim();
            if let Some(stripped) = next_line.strip_prefix("...") {
                let continuation = stripped.strip_prefix(' ').unwrap_or(stripped);
                input.push('\n');
                input.push_str(continuation);
                consumed += 1;
                next_idx += 1;
            } else {
                break;
            }
        }

        // Extract expected output (all lines until next >>> or end of docstring)
        let mut expected_lines = Vec::new();
        while next_idx < lines.len() {
            let next_line = lines[next_idx].trim();

            // Stop conditions
            if next_line.starts_with(">>>")
                || next_line.starts_with("\"\"\"")
                || next_line.starts_with("'''")
                || next_line.is_empty()
                    && next_idx + 1 < lines.len()
                    && (lines[next_idx + 1].trim().starts_with(">>>")
                        || lines[next_idx + 1].trim().starts_with("\"\"\"")
                        || lines[next_idx + 1].trim().starts_with("'''"))
            {
                break;
            }

            // Skip empty lines at the start of expected output
            if expected_lines.is_empty() && next_line.is_empty() {
                consumed += 1;
                next_idx += 1;
                continue;
            }

            expected_lines.push(next_line);
            consumed += 1;
            next_idx += 1;
        }

        let expected = expected_lines.join("\n");

        // Skip doctests with no expected output (statements like assignments)
        if expected.is_empty() {
            return Ok((None, consumed));
        }

        Ok((
            Some(Doctest {
                function: function.unwrap_or("<module>").to_string(),
                input,
                expected,
                line: start_line + 1, // 1-indexed
            }),
            consumed,
        ))
    }

    /// Extract doctests to the JSON format specified in the spec
    pub fn extract_to_result(&self, source: &str, module: &str) -> Result<DoctestResult> {
        let doctests = self.extract(source)?;

        // Group by function
        let mut by_function: std::collections::HashMap<String, Vec<Doctest>> =
            std::collections::HashMap::new();

        for dt in doctests {
            by_function.entry(dt.function.clone()).or_default().push(dt);
        }

        let function_doctests: Vec<FunctionDoctests> = by_function
            .into_iter()
            .map(|(function, examples)| FunctionDoctests {
                function,
                signature: None,
                docstring: None,
                examples,
            })
            .collect();

        Ok(DoctestResult {
            source: module.to_string(),
            module: module.to_string(),
            doctests: function_doctests,
        })
    }
}

/// Convert a doctest to a Rust doc test assertion
pub fn doctest_to_rust_assertion(doctest: &Doctest) -> String {
    // Simple conversion: >>> f(x) + expected -> assert_eq!(f(x), expected);
    format!("assert_eq!({}, {});", doctest.input, doctest.expected)
}

/// Generate Rust doc comment with doc tests
pub fn generate_rust_doc_tests(doctests: &[Doctest]) -> String {
    if doctests.is_empty() {
        return String::new();
    }

    let mut lines = vec!["/// ```".to_string()];
    for dt in doctests {
        lines.push(format!("/// {}", doctest_to_rust_assertion(dt)));
    }
    lines.push("/// ```".to_string());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RED TESTS - These define the expected behavior (GH-173)
    // =========================================================================

    #[test]
    fn test_extract_simple_doctest() {
        let source = r#"
def square(x: int) -> int:
    """Compute square.

    >>> square(4)
    16
    """
    return x * x
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].function, "square");
        assert_eq!(doctests[0].input, "square(4)");
        assert_eq!(doctests[0].expected, "16");
    }

    #[test]
    fn test_extract_multiple_doctests() {
        let source = r#"
def square(x: int) -> int:
    """Compute square.

    >>> square(4)
    16
    >>> square(-3)
    9
    """
    return x * x
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 2);
        assert_eq!(doctests[0].input, "square(4)");
        assert_eq!(doctests[0].expected, "16");
        assert_eq!(doctests[1].input, "square(-3)");
        assert_eq!(doctests[1].expected, "9");
    }

    #[test]
    fn test_extract_multiline_continuation() {
        let source = r#"
def add_all(a, b, c, d):
    """Add numbers.

    >>> add_all(1,
    ...         2,
    ...         3,
    ...         4)
    10
    """
    return a + b + c + d
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert!(doctests[0].input.contains("add_all(1,"));
        assert!(doctests[0].input.contains("2,"));
        assert_eq!(doctests[0].expected, "10");
    }

    #[test]
    fn test_extract_string_output() {
        let source = r#"
def greet(name: str) -> str:
    """Greet someone.

    >>> greet("World")
    'Hello, World!'
    """
    return f"Hello, {name}!"
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].input, "greet(\"World\")");
        assert_eq!(doctests[0].expected, "'Hello, World!'");
    }

    #[test]
    fn test_extract_multiple_functions() {
        let source = r#"
def add(a: int, b: int) -> int:
    """Add two numbers.

    >>> add(1, 2)
    3
    """
    return a + b

def multiply(a: int, b: int) -> int:
    """Multiply two numbers.

    >>> multiply(3, 4)
    12
    """
    return a * b
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 2);
        assert_eq!(doctests[0].function, "add");
        assert_eq!(doctests[0].input, "add(1, 2)");
        assert_eq!(doctests[1].function, "multiply");
        assert_eq!(doctests[1].input, "multiply(3, 4)");
    }

    #[test]
    fn test_extract_list_output() {
        let source = r#"
def range_list(n: int) -> list:
    """Create range list.

    >>> range_list(3)
    [0, 1, 2]
    """
    return list(range(n))
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].expected, "[0, 1, 2]");
    }

    #[test]
    fn test_extract_dict_output() {
        let source = r#"
def make_dict(key, value):
    """Create dict.

    >>> make_dict('a', 1)
    {'a': 1}
    """
    return {key: value}
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].expected, "{'a': 1}");
    }

    #[test]
    fn test_extract_multiline_output() {
        let source = r#"
def describe(x):
    """Describe value.

    >>> describe(42)
    Value: 42
    Type: int
    """
    print(f"Value: {x}")
    print(f"Type: {type(x).__name__}")
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert!(doctests[0].expected.contains("Value: 42"));
        assert!(doctests[0].expected.contains("Type: int"));
    }

    #[test]
    fn test_skip_doctests_without_output() {
        let source = r#"
def side_effect():
    """Do something.

    >>> x = side_effect()
    >>> print(x)
    42
    """
    return 42
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        // Should only capture the print(x) -> 42 doctest
        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].input, "print(x)");
        assert_eq!(doctests[0].expected, "42");
    }

    #[test]
    fn test_single_quote_docstring() {
        let source = r#"
def foo():
    '''Single quote docstring.

    >>> foo()
    'bar'
    '''
    return 'bar'
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].expected, "'bar'");
    }

    #[test]
    fn test_extract_to_result_json_format() {
        let source = r#"
def square(x: int) -> int:
    """Compute square.

    >>> square(4)
    16
    """
    return x * x
"#;

        let extractor = DoctestExtractor::new();
        let result = extractor.extract_to_result(source, "math_utils").unwrap();

        assert_eq!(result.module, "math_utils");
        assert_eq!(result.doctests.len(), 1);
        assert_eq!(result.doctests[0].function, "square");
        assert_eq!(result.doctests[0].examples.len(), 1);
    }

    #[test]
    fn test_doctest_to_rust_assertion() {
        let dt = Doctest {
            function: "square".to_string(),
            input: "square(4)".to_string(),
            expected: "16".to_string(),
            line: 5,
        };

        let rust = doctest_to_rust_assertion(&dt);
        assert_eq!(rust, "assert_eq!(square(4), 16);");
    }

    #[test]
    fn test_generate_rust_doc_tests() {
        let doctests = vec![
            Doctest {
                function: "square".to_string(),
                input: "square(4)".to_string(),
                expected: "16".to_string(),
                line: 5,
            },
            Doctest {
                function: "square".to_string(),
                input: "square(-3)".to_string(),
                expected: "9".to_string(),
                line: 7,
            },
        ];

        let rust_doc = generate_rust_doc_tests(&doctests);
        assert!(rust_doc.contains("/// ```"));
        assert!(rust_doc.contains("assert_eq!(square(4), 16);"));
        assert!(rust_doc.contains("assert_eq!(square(-3), 9);"));
    }

    #[test]
    fn test_line_numbers_are_correct() {
        let source = r#"
def foo():
    """Test.

    >>> foo()
    42
    """
    return 42
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        // >>> foo() is on line 5 (1-indexed)
        assert_eq!(doctests[0].line, 5);
    }

    #[test]
    fn test_empty_source() {
        let source = "";
        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();
        assert!(doctests.is_empty());
    }

    #[test]
    fn test_no_doctests() {
        let source = r#"
def foo():
    """No doctests here."""
    return 42
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();
        assert!(doctests.is_empty());
    }

    #[test]
    fn test_real_stdlib_example_len() {
        // Simulated stdlib-style doctest from str.len
        let source = r#"
def length(s: str) -> int:
    """Return the length of s.

    >>> length("hello")
    5
    >>> length("")
    0
    >>> length("日本語")
    3
    """
    return len(s)
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 3);
        assert_eq!(doctests[0].expected, "5");
        assert_eq!(doctests[1].expected, "0");
        assert_eq!(doctests[2].expected, "3");
    }

    #[test]
    fn test_boolean_output() {
        let source = r#"
def is_even(n: int) -> bool:
    """Check if n is even.

    >>> is_even(4)
    True
    >>> is_even(3)
    False
    """
    return n % 2 == 0
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 2);
        assert_eq!(doctests[0].expected, "True");
        assert_eq!(doctests[1].expected, "False");
    }

    #[test]
    fn test_none_output() {
        let source = r#"
def returns_none():
    """Return None.

    >>> returns_none()

    >>> returns_none() is None
    True
    """
    return None
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        // First doctest has no output (None doesn't print), second has True
        assert!(!doctests.is_empty());
        assert!(doctests.iter().any(|dt| dt.expected == "True"));
    }

    #[test]
    fn test_float_output() {
        let source = r#"
def divide(a: float, b: float) -> float:
    """Divide a by b.

    >>> divide(10.0, 4.0)
    2.5
    """
    return a / b
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        assert_eq!(doctests[0].expected, "2.5");
    }

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_doctest_struct_debug() {
        let dt = Doctest {
            function: "test_func".to_string(),
            input: "test_func(1)".to_string(),
            expected: "42".to_string(),
            line: 10,
        };

        let debug = format!("{:?}", dt);
        assert!(debug.contains("Doctest"));
        assert!(debug.contains("test_func"));
        assert!(debug.contains("42"));
        assert!(debug.contains("10"));
    }

    #[test]
    fn test_doctest_struct_clone() {
        let dt = Doctest {
            function: "original".to_string(),
            input: "original()".to_string(),
            expected: "1".to_string(),
            line: 5,
        };

        let cloned = dt.clone();
        assert_eq!(cloned.function, "original");
        assert_eq!(cloned.input, "original()");
        assert_eq!(cloned.expected, "1");
        assert_eq!(cloned.line, 5);
    }

    #[test]
    fn test_doctest_struct_partial_eq() {
        let dt1 = Doctest {
            function: "f".to_string(),
            input: "f()".to_string(),
            expected: "1".to_string(),
            line: 1,
        };

        let dt2 = Doctest {
            function: "f".to_string(),
            input: "f()".to_string(),
            expected: "1".to_string(),
            line: 1,
        };

        let dt3 = Doctest {
            function: "g".to_string(),
            input: "g()".to_string(),
            expected: "2".to_string(),
            line: 2,
        };

        assert_eq!(dt1, dt2);
        assert_ne!(dt1, dt3);
    }

    #[test]
    fn test_doctest_result_default() {
        let result = DoctestResult::default();
        assert!(result.source.is_empty());
        assert!(result.module.is_empty());
        assert!(result.doctests.is_empty());
    }

    #[test]
    fn test_doctest_result_debug() {
        let result = DoctestResult {
            source: "test.py".to_string(),
            module: "test_module".to_string(),
            doctests: vec![],
        };

        let debug = format!("{:?}", result);
        assert!(debug.contains("DoctestResult"));
        assert!(debug.contains("test.py"));
        assert!(debug.contains("test_module"));
    }

    #[test]
    fn test_doctest_result_clone() {
        let result = DoctestResult {
            source: "source.py".to_string(),
            module: "module".to_string(),
            doctests: vec![FunctionDoctests {
                function: "func".to_string(),
                signature: Some("func(x: int) -> int".to_string()),
                docstring: Some("Doc".to_string()),
                examples: vec![],
            }],
        };

        let cloned = result.clone();
        assert_eq!(cloned.source, "source.py");
        assert_eq!(cloned.module, "module");
        assert_eq!(cloned.doctests.len(), 1);
    }

    #[test]
    fn test_function_doctests_debug() {
        let fd = FunctionDoctests {
            function: "my_func".to_string(),
            signature: Some("my_func() -> None".to_string()),
            docstring: Some("Docstring text".to_string()),
            examples: vec![],
        };

        let debug = format!("{:?}", fd);
        assert!(debug.contains("FunctionDoctests"));
        assert!(debug.contains("my_func"));
    }

    #[test]
    fn test_function_doctests_clone() {
        let fd = FunctionDoctests {
            function: "func".to_string(),
            signature: None,
            docstring: None,
            examples: vec![Doctest {
                function: "func".to_string(),
                input: "func()".to_string(),
                expected: "42".to_string(),
                line: 1,
            }],
        };

        let cloned = fd.clone();
        assert_eq!(cloned.function, "func");
        assert!(cloned.signature.is_none());
        assert!(cloned.docstring.is_none());
        assert_eq!(cloned.examples.len(), 1);
    }

    #[test]
    fn test_doctest_extractor_default() {
        // Default derive uses false for bools, new() uses true
        let extractor: DoctestExtractor = Default::default();
        assert!(!extractor.include_module_doctests); // Default is false
        assert!(!extractor.include_class_methods); // Default is false

        // new() sets them to true
        let extractor_new = DoctestExtractor::new();
        assert!(extractor_new.include_module_doctests);
        assert!(extractor_new.include_class_methods);
    }

    #[test]
    fn test_doctest_extractor_debug() {
        let extractor = DoctestExtractor::new();
        let debug = format!("{:?}", extractor);
        assert!(debug.contains("DoctestExtractor"));
        assert!(debug.contains("include_module_doctests"));
        assert!(debug.contains("include_class_methods"));
    }

    #[test]
    fn test_doctest_extractor_clone() {
        let extractor = DoctestExtractor::new()
            .with_module_doctests(false)
            .with_class_methods(false);

        let cloned = extractor.clone();
        assert!(!cloned.include_module_doctests);
        assert!(!cloned.include_class_methods);
    }

    #[test]
    fn test_with_module_doctests_builder() {
        let extractor = DoctestExtractor::new().with_module_doctests(false);
        assert!(!extractor.include_module_doctests);
        assert!(extractor.include_class_methods); // Unchanged

        let extractor2 = DoctestExtractor::new().with_module_doctests(true);
        assert!(extractor2.include_module_doctests);
    }

    #[test]
    fn test_with_class_methods_builder() {
        let extractor = DoctestExtractor::new().with_class_methods(false);
        assert!(extractor.include_module_doctests); // Unchanged
        assert!(!extractor.include_class_methods);

        let extractor2 = DoctestExtractor::new().with_class_methods(true);
        assert!(extractor2.include_class_methods);
    }

    #[test]
    fn test_builder_chaining() {
        let extractor = DoctestExtractor::new()
            .with_module_doctests(false)
            .with_class_methods(false);

        assert!(!extractor.include_module_doctests);
        assert!(!extractor.include_class_methods);
    }

    #[test]
    fn test_generate_rust_doc_tests_empty() {
        let doctests: Vec<Doctest> = vec![];
        let result = generate_rust_doc_tests(&doctests);
        assert!(result.is_empty());
    }

    #[test]
    fn test_generate_rust_doc_tests_single() {
        let doctests = vec![Doctest {
            function: "f".to_string(),
            input: "f(1)".to_string(),
            expected: "2".to_string(),
            line: 1,
        }];

        let result = generate_rust_doc_tests(&doctests);
        assert!(result.contains("/// ```"));
        assert!(result.contains("assert_eq!(f(1), 2);"));
    }

    #[test]
    fn test_extract_function_name_simple() {
        let result = DoctestExtractor::extract_function_name("def foo():");
        assert_eq!(result, Some("foo".to_string()));
    }

    #[test]
    fn test_extract_function_name_with_args() {
        let result = DoctestExtractor::extract_function_name("def bar(x: int, y: str) -> bool:");
        assert_eq!(result, Some("bar".to_string()));
    }

    #[test]
    fn test_extract_function_name_underscore() {
        let result = DoctestExtractor::extract_function_name("def _private_func(arg):");
        assert_eq!(result, Some("_private_func".to_string()));
    }

    #[test]
    fn test_extract_function_name_invalid() {
        let result = DoctestExtractor::extract_function_name("class Foo:");
        assert!(result.is_none());

        let result2 = DoctestExtractor::extract_function_name("x = 1");
        assert!(result2.is_none());
    }

    #[test]
    fn test_doctest_serialization() {
        let dt = Doctest {
            function: "test".to_string(),
            input: "test()".to_string(),
            expected: "42".to_string(),
            line: 5,
        };

        let json = serde_json::to_string(&dt).unwrap();
        assert!(json.contains("\"function\":\"test\""));
        assert!(json.contains("\"input\":\"test()\""));
        assert!(json.contains("\"expected\":\"42\""));
        assert!(json.contains("\"line\":5"));
    }

    #[test]
    fn test_doctest_deserialization() {
        let json = r#"{"function":"f","input":"f()","expected":"1","line":10}"#;
        let dt: Doctest = serde_json::from_str(json).unwrap();

        assert_eq!(dt.function, "f");
        assert_eq!(dt.input, "f()");
        assert_eq!(dt.expected, "1");
        assert_eq!(dt.line, 10);
    }

    #[test]
    fn test_doctest_result_serialization() {
        let result = DoctestResult {
            source: "test.py".to_string(),
            module: "test".to_string(),
            doctests: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"source\":\"test.py\""));
        assert!(json.contains("\"module\":\"test\""));
    }

    #[test]
    fn test_extract_to_result_grouping() {
        let source = r#"
def foo():
    """Foo.

    >>> foo()
    1
    """
    return 1

def bar():
    """Bar.

    >>> bar()
    2
    """
    return 2
"#;

        let extractor = DoctestExtractor::new();
        let result = extractor.extract_to_result(source, "test_mod").unwrap();

        assert_eq!(result.module, "test_mod");
        assert_eq!(result.source, "test_mod");
        assert_eq!(result.doctests.len(), 2);
    }

    #[test]
    fn test_module_level_doctest() {
        let source = r#"
"""Module docstring.

>>> 1 + 1
2
"""

def foo():
    pass
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        // Module-level doctest should be captured with <module> function name
        assert!(!doctests.is_empty());
        assert!(doctests.iter().any(|dt| dt.function == "<module>"));
    }

    #[test]
    fn test_doctest_with_whitespace() {
        let source = r#"
def foo():
    """Test with whitespace.

    >>>    foo()
    42
    """
    return 42
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        assert_eq!(doctests.len(), 1);
        // Input should have leading spaces stripped
        assert_eq!(doctests[0].input.trim(), "foo()");
    }

    #[test]
    fn test_inline_docstring() {
        let source = r#"
def foo():
    """Inline docstring. >>> foo() should not be parsed here."""
    return 42
"#;

        let extractor = DoctestExtractor::new();
        let doctests = extractor.extract(source).unwrap();

        // Inline >>> in docstring text should not be parsed
        assert!(doctests.is_empty());
    }
}
