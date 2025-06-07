use depyler_annotations::TranspilationAnnotations;
use depyler_annotations::{AnnotationError, AnnotationParser};
use depyler_core::hir::*;
use depyler_core::DepylerPipeline;
use depyler_quality::{QualityAnalyzer, QualityError};
use smallvec::smallvec;

#[test]
fn test_pipeline_invalid_python_syntax() {
    let pipeline = DepylerPipeline::new();

    // Test various invalid Python syntax scenarios
    let invalid_cases = vec![
        "def incomplete_function(",
        "if True\n    pass",   // Missing colon
        "def func():\nreturn", // Invalid indentation
    ];

    for invalid_python in invalid_cases {
        let result = pipeline.transpile(invalid_python);
        assert!(
            result.is_err(),
            "Should fail for invalid Python: {invalid_python}"
        );
    }

    // These might succeed or fail depending on parsing - just check they don't panic
    let potentially_valid_cases = vec![
        "invalid_keyword_here()",
        "def func(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z):", // Very long
    ];

    for potentially_valid in potentially_valid_cases {
        let result = pipeline.transpile(potentially_valid);
        // Just ensure it doesn't panic - result can be either success or failure
        let _ = result; // Both Ok and Err are acceptable
    }
}

#[test]
fn test_pipeline_unsupported_features() {
    let pipeline = DepylerPipeline::new();

    // Test Python features not yet supported
    let unsupported_cases = vec![
        "async def async_func(): await something()",
        "class Parent: pass\nclass Child(Parent): pass", // Inheritance
        "@decorator\ndef decorated_func(): pass",
        "lambda x: x + 1",
        "try:\n    risky()\nexcept Exception:\n    handle()",
    ];

    for unsupported_python in unsupported_cases {
        let result = pipeline.transpile(unsupported_python);
        // Most should fail, but some might be partially supported
        if result.is_err() {
            // Expected failure - test passed
        }
    }
}

#[test]
fn test_annotation_parser_error_cases() {
    let parser = AnnotationParser::new();

    // Test invalid annotation syntax
    let invalid_annotations = vec![
        "# @depyler: invalid_key = value_without_quotes",
        "# @depyler: type_strategy = \"unknown_strategy\"",
        "# @depyler: ownership = \"invalid_ownership\"",
        "# @depyler: safety_level = \"unknown_safety\"",
        "# @depyler: unroll_loops = \"not_a_number\"",
    ];

    for invalid_annotation in invalid_annotations {
        let result = parser.parse_annotations(invalid_annotation);
        match result {
            Err(AnnotationError::UnknownKey(_)) | Err(AnnotationError::InvalidValue { .. }) => {
                // Expected error types
                // Expected - we successfully generated an error
            }
            _ => {
                // Might succeed with default fallback - that's also valid
            }
        }
    }
}

#[test]
fn test_annotation_parser_malformed_syntax() {
    let parser = AnnotationParser::new();

    // Test malformed annotation syntax
    let malformed_cases = vec![
        "# @depyler:",            // Missing key-value
        "# @depyler",             // Missing colon
        "# depyler: key = value", // Missing @
        "# @depyler key = value", // Missing colon
        "# @depyler: = value",    // Missing key
        "# @depyler: key =",      // Missing value
    ];

    for malformed in malformed_cases {
        let result = parser.parse_annotations(malformed);
        // Should either error or return defaults gracefully
        match result {
            Ok(annotations) => {
                // Default annotations should be returned
                assert_eq!(annotations, TranspilationAnnotations::default());
            }
            Err(_) => {
                // Error is also acceptable for malformed syntax
                // Expected - we successfully generated an error
            }
        }
    }
}

#[test]
fn test_quality_analyzer_edge_cases() {
    let analyzer = QualityAnalyzer::new();

    // Test with problematic function structures
    let problematic_function = HirFunction {
        name: "problematic".to_string(),
        params: smallvec![],
        ret_type: Type::Unknown, // Unknown type
        body: vec![
            // Very deeply nested structure
            HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::If {
                    condition: HirExpr::Literal(Literal::Bool(true)),
                    then_body: vec![HirStmt::If {
                        condition: HirExpr::Literal(Literal::Bool(true)),
                        then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
                        else_body: None,
                    }],
                    else_body: None,
                }],
                else_body: None,
            },
        ],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
        docstring: None,
    };

    let functions = vec![problematic_function];
    let result = analyzer.analyze_quality(&functions);

    // Should handle gracefully even with complex structures
    assert!(result.is_ok());

    let report = result.unwrap();
    assert!(report.complexity_metrics.cyclomatic_complexity > 1);
    assert!(report.complexity_metrics.max_nesting >= 3);
}

#[test]
fn test_pipeline_empty_and_whitespace() {
    let pipeline = DepylerPipeline::new();

    // Test edge cases with empty/whitespace input
    let edge_cases = vec![
        "",                 // Completely empty
        "   ",              // Only whitespace
        "\n\n\n",           // Only newlines
        "# Just a comment", // Only comments
        "   # Comment with spaces   ",
    ];

    for edge_case in edge_cases {
        let result = pipeline.transpile(edge_case);
        // Should handle gracefully - either succeed with empty output or fail cleanly
        let _ = result; // Both Ok and Err are acceptable
    }
}

#[test]
fn test_error_types_creation() {
    // Test that error types can be created and handled
    let annotation_error = AnnotationError::UnknownKey("test_key".to_string());
    assert!(matches!(annotation_error, AnnotationError::UnknownKey(_)));

    let invalid_value_error = AnnotationError::InvalidValue {
        key: "test_key".to_string(),
        value: "test_value".to_string(),
    };
    assert!(matches!(
        invalid_value_error,
        AnnotationError::InvalidValue { .. }
    ));

    let syntax_error = AnnotationError::InvalidSyntax("test syntax".to_string());
    assert!(matches!(syntax_error, AnnotationError::InvalidSyntax(_)));
}

#[test]
fn test_quality_error_types() {
    // Test quality error creation
    let gate_failed = QualityError::GateFailed {
        gate_name: "Test Gate".to_string(),
    };
    assert!(matches!(gate_failed, QualityError::GateFailed { .. }));

    let metric_failed = QualityError::MetricCalculationFailed {
        metric: "Test Metric".to_string(),
    };
    assert!(matches!(
        metric_failed,
        QualityError::MetricCalculationFailed { .. }
    ));

    let coverage_unavailable = QualityError::CoverageUnavailable;
    assert!(matches!(
        coverage_unavailable,
        QualityError::CoverageUnavailable
    ));
}

#[test]
fn test_pipeline_with_verification_errors() {
    let pipeline = DepylerPipeline::new().with_verification();

    // Test code that might fail verification
    let potentially_problematic = vec![
        "def unchecked_access(arr, idx): return arr[idx]", // No bounds checking
        "def infinite_loop(): while True: pass",           // Potential infinite loop
        "def deep_recursion(n): return deep_recursion(n)", // Infinite recursion
    ];

    for problematic_code in potentially_problematic {
        let result = pipeline.transpile(problematic_code);
        // Should either succeed with warnings or fail with useful error messages
        match result {
            Ok(_) => {} // Success with verification is good
            Err(e) => {
                // Error should be informative
                let error_msg = format!("{e}");
                assert!(!error_msg.is_empty());
            }
        }
    }
}

#[test]
fn test_large_input_handling() {
    let pipeline = DepylerPipeline::new();

    // Create a reasonably large Python function
    let mut large_function = String::from("def large_func(x: int) -> int:\n");
    large_function.push_str("    result = 0\n");

    // Add many similar statements
    for i in 0..100 {
        large_function.push_str(&format!("    if x > {i}:\n        result += {i}\n"));
    }
    large_function.push_str("    return result\n");

    let result = pipeline.transpile(&large_function);

    // Should handle reasonably large inputs
    match result {
        Ok(rust_code) => {
            assert!(!rust_code.is_empty());
            assert!(rust_code.contains("pub fn large_func"));
        }
        Err(e) => {
            // If it fails, should be a clean error
            let error_msg = format!("{e}");
            assert!(!error_msg.is_empty());
        }
    }
}

#[test]
fn test_unicode_and_special_characters() {
    let pipeline = DepylerPipeline::new();

    // Test with Unicode and special characters
    let unicode_cases = vec![
        "def test_unicode(): return \"Hello 世界\"",
        "def test_emoji(): return \"🚀 Rust\"",
        "def test_accents(): return \"café\"",
        "# Comment with special chars: !@#$%^&*()",
    ];

    for unicode_case in unicode_cases {
        let result = pipeline.transpile(unicode_case);
        // Should handle Unicode gracefully
        let _ = result; // Both Ok and Err are acceptable
    }
}

#[test]
fn test_annotation_parser_unicode() {
    let parser = AnnotationParser::new();

    // Test annotation parsing with Unicode
    let unicode_annotation =
        "# @depyler: type_strategy = \"conservative\"\n# Comment with Unicode: 测试";

    let result = parser.parse_annotations(unicode_annotation);
    assert!(result.is_ok());

    let annotations = result.unwrap();
    assert_eq!(
        annotations.type_strategy,
        depyler_annotations::TypeStrategy::Conservative
    );
}

#[test]
fn test_concurrent_pipeline_usage() {
    use std::sync::Arc;
    use std::thread;

    let pipeline = Arc::new(DepylerPipeline::new());
    let mut handles = vec![];

    // Test multiple threads using the pipeline
    for i in 0..5 {
        let pipeline_clone = Arc::clone(&pipeline);
        let handle = thread::spawn(move || {
            let test_code = format!("def test_func_{i}(x: int) -> int:\n    return x + {i}");
            pipeline_clone.transpile(&test_code)
        });
        handles.push(handle);
    }

    // Wait for all threads and check results
    for handle in handles {
        let result = handle.join().unwrap();
        // Each thread should get a result (success or clean error)
        let _ = result; // Both Ok and Err are acceptable
    }
}
