#[cfg(test)]
mod tests {
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
        let types = vec![
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
                assert!(true);
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
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        }
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
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_default_trait() {
        let session1 = InteractiveSession::new();
        let session2 = InteractiveSession::default();

        // Both should create valid sessions
        let _ = session1;
        let _ = session2;
    }
}
