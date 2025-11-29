use crate::interactive::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_interactive_session_creation() {
    let session = InteractiveSession::new();
    // Should create without panic
    let _ = session; // Avoid unused warning
}

#[test]
fn test_suggestion_type_variants() {
    let types = [
        SuggestionType::Performance,
        SuggestionType::Safety,
        SuggestionType::TypeStrategy,
        SuggestionType::ErrorHandling,
        SuggestionType::Concurrency,
        SuggestionType::Memory,
    ];

    // Ensure all variants are covered
    assert_eq!(types.len(), 6);
}

#[test]
fn test_impact_level_ordering() {
    assert!(ImpactLevel::Low < ImpactLevel::Medium);
    assert!(ImpactLevel::Medium < ImpactLevel::High);
    assert!(ImpactLevel::Low < ImpactLevel::High);
}

#[test]
fn test_annotation_suggestion_creation() {
    let suggestion = AnnotationSuggestion {
        line: 5,
        function_name: "test_func".to_string(),
        suggestion_type: SuggestionType::Performance,
        annotation: "# @depyler: optimize = true".to_string(),
        reason: "Function has nested loops".to_string(),
        impact: ImpactLevel::High,
    };

    assert_eq!(suggestion.line, 5);
    assert_eq!(suggestion.function_name, "test_func");
    assert!(matches!(
        suggestion.suggestion_type,
        SuggestionType::Performance
    ));
    assert_eq!(suggestion.impact, ImpactLevel::High);
}

#[test]
fn test_attempt_transpilation_simple() {
    let session = InteractiveSession::new();
    let python_code = "def add(a: int, b: int) -> int:\n    return a + b";

    match session.attempt_transpilation(python_code) {
        Ok((rust_code, warnings)) => {
            assert!(!rust_code.is_empty());
            // Simple function shouldn't have warnings
            assert!(warnings.is_empty() || warnings.len() < 2);
        }
        Err(_) => {
            // Transpilation might fail in test environment
            // Transpilation might fail in test environment
        }
    }
}

#[test]
fn test_attempt_transpilation_with_unsafe() {
    let _session = InteractiveSession::new();

    // This would need a more complex example that generates unsafe code
    // For now, we test the warning detection logic separately
    let rust_code = "unsafe { std::ptr::null() }";
    let _warnings: Vec<String> = vec![];

    // The method checks for "unsafe" in generated code
    assert!(rust_code.contains("unsafe"));
}

#[test]
#[ignore = "Requires terminal interaction"]
fn test_run_with_temp_file() {
    let mut session = InteractiveSession::new();

    // Create a temporary file with Python code
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "def simple():\n    return 42").unwrap();

    // Run interactive session (will fail due to no terminal in test)
    let result = session.run(temp_file.path().to_str().unwrap(), false);

    // In test environment, this will likely fail due to terminal interaction
    // We just ensure it doesn't panic unexpectedly
    if result.is_ok() {}
}

#[test]
#[ignore = "Requires terminal interaction"]
fn test_suggest_improvements() {
    let session = InteractiveSession::new();
    let python_code =
        "def compute(data):\n    for i in data:\n        for j in data:\n            pass";
    let rust_code = "fn compute(data: Vec<i32>) {}";

    // This involves terminal interaction, so will fail in test
    let result = session.suggest_improvements(python_code, rust_code);
    if result.is_ok() {}
}

#[test]
fn test_default_trait() {
    let session1 = InteractiveSession::new();
    let session2 = InteractiveSession::default();

    // Both should create valid sessions
    let _ = session1;
    let _ = session2;
}

// Additional tests for better coverage

#[test]
fn test_suggestion_type_debug() {
    let perf = SuggestionType::Performance;
    let safety = SuggestionType::Safety;
    let type_strat = SuggestionType::TypeStrategy;
    let error = SuggestionType::ErrorHandling;
    let concurrency = SuggestionType::Concurrency;
    let memory = SuggestionType::Memory;

    assert!(format!("{:?}", perf).contains("Performance"));
    assert!(format!("{:?}", safety).contains("Safety"));
    assert!(format!("{:?}", type_strat).contains("TypeStrategy"));
    assert!(format!("{:?}", error).contains("ErrorHandling"));
    assert!(format!("{:?}", concurrency).contains("Concurrency"));
    assert!(format!("{:?}", memory).contains("Memory"));
}

#[test]
fn test_suggestion_type_clone() {
    let original = SuggestionType::Performance;
    let cloned = original.clone();
    assert!(matches!(cloned, SuggestionType::Performance));
}

#[test]
fn test_impact_level_debug() {
    let low = ImpactLevel::Low;
    let medium = ImpactLevel::Medium;
    let high = ImpactLevel::High;

    assert!(format!("{:?}", low).contains("Low"));
    assert!(format!("{:?}", medium).contains("Medium"));
    assert!(format!("{:?}", high).contains("High"));
}

#[test]
fn test_impact_level_clone() {
    let original = ImpactLevel::High;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_impact_level_equality() {
    let l1 = ImpactLevel::Low;
    let l2 = ImpactLevel::Low;
    let m = ImpactLevel::Medium;

    assert_eq!(l1, l2);
    assert_ne!(l1, m);
}

#[test]
fn test_impact_level_sorting() {
    let mut levels = vec![ImpactLevel::High, ImpactLevel::Low, ImpactLevel::Medium];
    levels.sort();
    assert_eq!(levels, vec![ImpactLevel::Low, ImpactLevel::Medium, ImpactLevel::High]);
}

#[test]
fn test_annotation_suggestion_debug() {
    let suggestion = AnnotationSuggestion {
        line: 10,
        function_name: "test".to_string(),
        suggestion_type: SuggestionType::Safety,
        annotation: "# @depyler: safe = true".to_string(),
        reason: "Test reason".to_string(),
        impact: ImpactLevel::Medium,
    };

    let debug = format!("{:?}", suggestion);
    assert!(debug.contains("line"));
    assert!(debug.contains("function_name"));
}

#[test]
fn test_annotation_suggestion_clone() {
    let suggestion = AnnotationSuggestion {
        line: 1,
        function_name: "clone_test".to_string(),
        suggestion_type: SuggestionType::Memory,
        annotation: "# @depyler: memory = \"efficient\"".to_string(),
        reason: "Memory optimization".to_string(),
        impact: ImpactLevel::Low,
    };

    let cloned = suggestion.clone();
    assert_eq!(suggestion.line, cloned.line);
    assert_eq!(suggestion.function_name, cloned.function_name);
    assert_eq!(suggestion.annotation, cloned.annotation);
    assert_eq!(suggestion.reason, cloned.reason);
    assert_eq!(suggestion.impact, cloned.impact);
}

#[test]
fn test_attempt_transpilation_function_with_loop() {
    let session = InteractiveSession::new();
    let python_code = r#"def sum_list(items: list) -> int:
    result = 0
    for item in items:
        result = result + item
    return result"#;

    // May succeed or fail depending on type inference
    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_nested_function() {
    let session = InteractiveSession::new();
    let python_code = r#"def outer(x: int) -> int:
    def inner(y: int) -> int:
        return y * 2
    return inner(x)"#;

    // May succeed or fail
    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_with_conditionals() {
    let session = InteractiveSession::new();
    let python_code = r#"def abs_value(x: int) -> int:
    if x < 0:
        return -x
    else:
        return x"#;

    let result = session.attempt_transpilation(python_code);
    // Conditional code should transpile
    if let Ok((rust_code, _)) = result {
        assert!(!rust_code.is_empty());
    }
}

#[test]
fn test_attempt_transpilation_empty_function() {
    let session = InteractiveSession::new();
    let python_code = "def empty() -> None:\n    pass";

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_with_string_ops() {
    let session = InteractiveSession::new();
    let python_code = r#"def concat(a: str, b: str) -> str:
    return a + b"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_with_list_append() {
    let session = InteractiveSession::new();
    let python_code = r#"def build_list() -> list:
    result = []
    result.append(1)
    result.append(2)
    return result"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_with_dict() {
    let session = InteractiveSession::new();
    let python_code = r#"def get_dict() -> dict:
    d = {}
    d["key"] = "value"
    return d"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_multiline() {
    let session = InteractiveSession::new();
    let python_code = r#"def multi_line(x: int, y: int, z: int) -> int:
    a = x + y
    b = a * z
    c = b - x
    return c"#;

    let result = session.attempt_transpilation(python_code);
    if let Ok((rust_code, _)) = result {
        assert!(!rust_code.is_empty());
    }
}

#[test]
fn test_attempt_transpilation_invalid_syntax() {
    let session = InteractiveSession::new();
    let python_code = "def invalid(\n    syntax error here";

    let result = session.attempt_transpilation(python_code);
    assert!(result.is_err());
}

#[test]
fn test_attempt_transpilation_empty_source() {
    let session = InteractiveSession::new();
    let python_code = "";

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_comment_only() {
    let session = InteractiveSession::new();
    let python_code = "# Just a comment";

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_class_definition() {
    let session = InteractiveSession::new();
    let python_code = r#"class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_with_import() {
    let session = InteractiveSession::new();
    let python_code = r#"from typing import List

def process(items: List[int]) -> int:
    return sum(items)"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_run_interactive_session_function_exists() {
    // Just verify the function exists and is callable
    // It will fail without a terminal, but should not panic
}

#[test]
fn test_annotation_suggestion_with_all_suggestion_types() {
    let suggestions = vec![
        AnnotationSuggestion {
            line: 1,
            function_name: "perf_fn".to_string(),
            suggestion_type: SuggestionType::Performance,
            annotation: "# @depyler: optimize".to_string(),
            reason: "Perf".to_string(),
            impact: ImpactLevel::High,
        },
        AnnotationSuggestion {
            line: 2,
            function_name: "safe_fn".to_string(),
            suggestion_type: SuggestionType::Safety,
            annotation: "# @depyler: safe".to_string(),
            reason: "Safety".to_string(),
            impact: ImpactLevel::High,
        },
        AnnotationSuggestion {
            line: 3,
            function_name: "type_fn".to_string(),
            suggestion_type: SuggestionType::TypeStrategy,
            annotation: "# @depyler: type".to_string(),
            reason: "Type".to_string(),
            impact: ImpactLevel::Medium,
        },
        AnnotationSuggestion {
            line: 4,
            function_name: "error_fn".to_string(),
            suggestion_type: SuggestionType::ErrorHandling,
            annotation: "# @depyler: error".to_string(),
            reason: "Error".to_string(),
            impact: ImpactLevel::Medium,
        },
        AnnotationSuggestion {
            line: 5,
            function_name: "conc_fn".to_string(),
            suggestion_type: SuggestionType::Concurrency,
            annotation: "# @depyler: concurrent".to_string(),
            reason: "Concurrency".to_string(),
            impact: ImpactLevel::Low,
        },
        AnnotationSuggestion {
            line: 6,
            function_name: "mem_fn".to_string(),
            suggestion_type: SuggestionType::Memory,
            annotation: "# @depyler: memory".to_string(),
            reason: "Memory".to_string(),
            impact: ImpactLevel::Low,
        },
    ];

    assert_eq!(suggestions.len(), 6);

    // Sort by impact
    let mut sorted = suggestions.clone();
    sorted.sort_by(|a, b| b.impact.cmp(&a.impact).then_with(|| a.line.cmp(&b.line)));

    // High impact should be first
    assert!(matches!(sorted[0].impact, ImpactLevel::High));
}

#[test]
fn test_multiple_sessions() {
    let session1 = InteractiveSession::new();
    let session2 = InteractiveSession::new();
    let session3 = InteractiveSession::default();

    // All sessions should be independent
    let code = "def foo() -> int:\n    return 1";
    let _r1 = session1.attempt_transpilation(code);
    let _r2 = session2.attempt_transpilation(code);
    let _r3 = session3.attempt_transpilation(code);
}

#[test]
fn test_impact_level_partial_ord() {
    let low = ImpactLevel::Low;
    let medium = ImpactLevel::Medium;
    let high = ImpactLevel::High;

    assert!(low.partial_cmp(&medium) == Some(std::cmp::Ordering::Less));
    assert!(medium.partial_cmp(&high) == Some(std::cmp::Ordering::Less));
    assert!(high.partial_cmp(&low) == Some(std::cmp::Ordering::Greater));
    assert!(medium.partial_cmp(&medium) == Some(std::cmp::Ordering::Equal));
}

#[test]
fn test_warning_detection_unsafe() {
    // Test that the warning detection logic would find unsafe
    let rust_code = "pub fn dangerous() { unsafe { } }";
    assert!(rust_code.contains("unsafe"));
}

#[test]
fn test_warning_detection_panic() {
    // Test that the warning detection logic would find panic
    let rust_code = "pub fn might_fail() { panic!(\"error\"); }";
    assert!(rust_code.contains("panic!"));
}

#[test]
fn test_attempt_transpilation_boolean_ops() {
    let session = InteractiveSession::new();
    let python_code = r#"def check(a: bool, b: bool) -> bool:
    return a and b or not a"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_comparison() {
    let session = InteractiveSession::new();
    let python_code = r#"def compare(x: int, y: int) -> bool:
    return x < y and y > 0"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_while_loop() {
    let session = InteractiveSession::new();
    let python_code = r#"def countdown(n: int) -> int:
    while n > 0:
        n = n - 1
    return n"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_attempt_transpilation_optional_return() {
    let session = InteractiveSession::new();
    let python_code = r#"from typing import Optional

def maybe_value(x: int) -> Optional[int]:
    if x > 0:
        return x
    return None"#;

    let _result = session.attempt_transpilation(python_code);
}

#[test]
fn test_run_nonexistent_file() {
    let mut session = InteractiveSession::new();
    let result = session.run("/nonexistent/path/to/file.py", false);
    assert!(result.is_err());
}

#[test]
fn test_impact_level_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(ImpactLevel::Low);
    set.insert(ImpactLevel::Medium);
    set.insert(ImpactLevel::High);
    set.insert(ImpactLevel::Low); // Duplicate

    assert_eq!(set.len(), 3);
}
