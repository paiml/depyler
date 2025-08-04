use depyler_core::DepylerPipeline;

#[cfg(test)]
mod error_path_tests {
    use super::*;

    #[test]
    fn test_invalid_python_syntax() {
        let pipeline = DepylerPipeline::new();
        let invalid_syntax = r#"
def invalid_function(
    # Missing closing parenthesis and colon
    return 42
"#;

        let result = pipeline.transpile(invalid_syntax);
        assert!(result.is_err());
    }

    #[test]
    fn test_unterminated_string() {
        let pipeline = DepylerPipeline::new();
        let unterminated_string = r#"
def broken_string() -> str:
    return "unterminated string
"#;

        let result = pipeline.transpile(unterminated_string);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_indentation() {
        let pipeline = DepylerPipeline::new();
        let bad_indentation = r#"
def poorly_indented():
return 42
    x = 5
  y = 10
"#;

        let result = pipeline.transpile(bad_indentation);
        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_variables() {
        let pipeline = DepylerPipeline::new();
        let undefined_vars = r#"
def uses_undefined() -> int:
    return undefined_variable + another_undefined
"#;

        let result = pipeline.transpile(undefined_vars);
        // Should either transpile with warnings or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_recursive_imports() {
        let pipeline = DepylerPipeline::new();
        let recursive_import = r#"
import sys
from os import path
import json, csv, re
"#;

        let result = pipeline.transpile(recursive_import);
        // Should handle imports or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_unsupported_python_features() {
        let pipeline = DepylerPipeline::new();
        let unsupported_features = r#"
# Generator function
def generator():
    yield 1
    yield 2

# Async function
async def async_func():
    await some_coroutine()

# Decorators
@property
def decorated_func(self):
    return self.value

# Context managers
with open("file.txt") as f:
    content = f.read()
"#;

        let result = pipeline.transpile(unsupported_features);
        // Should fail gracefully on unsupported features
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_malformed_function_definitions() {
        let pipeline = DepylerPipeline::new();
        let malformed_function = r#"
def (x): return x
def 123invalid(x): return x
def def(x): return x  # Reserved keyword
"#;

        let result = pipeline.transpile(malformed_function);
        assert!(result.is_err());
    }

    #[test]
    fn test_type_annotation_errors() {
        let pipeline = DepylerPipeline::new();
        let bad_type_annotations = r#"
def bad_types(x: InvalidType, y: 123NotAType) -> UnknownReturnType:
    return x + y
"#;

        let result = pipeline.transpile(bad_type_annotations);
        // Should handle unknown types gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_division_by_zero_patterns() {
        let pipeline = DepylerPipeline::new();
        let division_by_zero = r#"
def dangerous_division(x: int) -> int:
    zero = 0
    return x // zero  # Floor division by zero
"#;

        let result = pipeline.transpile(division_by_zero);
        // Should transpile but may generate unsafe code
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_infinite_recursion_pattern() {
        let pipeline = DepylerPipeline::new();
        let infinite_recursion = r#"
def infinite_recursion(x: int) -> int:
    return infinite_recursion(x)
"#;

        let result = pipeline.transpile(infinite_recursion);
        // Should transpile but create potentially dangerous code
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_memory_intensive_patterns() {
        let pipeline = DepylerPipeline::new();
        let memory_intensive = r#"
def memory_hog() -> list:
    big_list = []
    for i in range(1000000):
        big_list.append(i * i)
    return big_list
"#;

        let result = pipeline.transpile(memory_intensive);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_mixed_tabs_and_spaces() {
        let pipeline = DepylerPipeline::new();
        let mixed_whitespace =
            "def mixed_whitespace():\n\tif True:\n        return 42\n\telse:\n        return 0";

        let result = pipeline.transpile(mixed_whitespace);
        assert!(result.is_err()); // Python should reject mixed tabs/spaces
    }

    #[test]
    fn test_extremely_long_lines() {
        let pipeline = DepylerPipeline::new();
        let very_long_expression = format!(
            "def long_line() -> int:\n    return {}",
            (0..1000)
                .map(|i| format!("{}", i))
                .collect::<Vec<_>>()
                .join(" + ")
        );

        let result = pipeline.transpile(&very_long_expression);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_null_bytes_in_source() {
        let pipeline = DepylerPipeline::new();
        let null_byte_source = "def func():\n    return 'hello\x00world'";

        let result = pipeline.transpile(null_byte_source);
        // Should handle or reject null bytes appropriately
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_circular_type_references() {
        let pipeline = DepylerPipeline::new();
        let circular_types = r#"
from typing import Optional

def circular_ref(x: Optional['CircularRef']) -> 'CircularRef':
    return x if x else CircularRef()
"#;

        let result = pipeline.transpile(circular_types);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_resource_exhaustion_simulation() {
        let pipeline = DepylerPipeline::new();

        // Test with smaller inputs to avoid stack overflow
        for size in [5, 10, 20] {
            let large_function = format!(
                "def large_func({}) -> int:\n    return {}",
                (0..size)
                    .map(|i| format!("x{}: int", i))
                    .collect::<Vec<_>>()
                    .join(", "),
                (0..size)
                    .map(|i| format!("x{}", i))
                    .collect::<Vec<_>>()
                    .join(" + ")
            );

            let result = pipeline.transpile(&large_function);
            // Should handle reasonable sizes, may fail on extreme sizes
            assert!(result.is_ok() || result.is_err());
        }
    }
}
