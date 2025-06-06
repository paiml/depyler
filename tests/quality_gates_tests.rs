use depyler_annotations::TranspilationAnnotations;
use depyler_core::hir::*;
use depyler_quality::*;
use smallvec::smallvec;

fn create_simple_function() -> HirFunction {
    HirFunction {
        name: "simple".to_string(),
        params: smallvec![("x".to_string(), Type::Int)],
        ret_type: Type::Int,
        body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
    }
}

fn create_complex_function() -> HirFunction {
    HirFunction {
        name: "complex".to_string(),
        params: smallvec![
            ("a".to_string(), Type::Int),
            ("b".to_string(), Type::Int),
            ("c".to_string(), Type::Int)
        ],
        ret_type: Type::Int,
        body: vec![HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::If {
                condition: HirExpr::Binary {
                    op: BinOp::Gt,
                    left: Box::new(HirExpr::Var("b".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::Int(0))),
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Binary {
                    op: BinOp::Add,
                    left: Box::new(HirExpr::Var("a".to_string())),
                    right: Box::new(HirExpr::Var("b".to_string())),
                }))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Var("a".to_string())))]),
            }],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(
                0,
            ))))]),
        }],
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
    }
}

#[test]
fn test_pmat_calculation_simple_function() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_simple_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Simple function should have good PMAT scores
    assert!(report.pmat_metrics.productivity_score > 0.0);
    assert!(report.pmat_metrics.maintainability_score > 0.0);
    assert!(report.pmat_metrics.tdg >= 1.0 && report.pmat_metrics.tdg <= 2.0);
}

#[test]
fn test_pmat_calculation_complex_function() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_complex_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Complex function should have lower PMAT scores
    assert!(report.pmat_metrics.productivity_score < 50.0);
    assert!(report.pmat_metrics.maintainability_score < 50.0);
    assert!(report.complexity_metrics.cyclomatic_complexity >= 3);
}

#[test]
fn test_quality_gates_all_pass() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_simple_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Simple function should pass all quality gates
    assert_eq!(report.overall_status, QualityStatus::Passed);
    assert!(!report.gates_passed.is_empty());
    assert!(report.gates_failed.is_empty());
}

#[test]
fn test_quality_gates_complexity_fail() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_complex_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Complex function might fail some gates
    assert!(report.complexity_metrics.cyclomatic_complexity > 1);
    assert!(report.complexity_metrics.cognitive_complexity > 0);
}

#[test]
fn test_empty_function_list() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Empty function list should handle gracefully
    assert_eq!(report.complexity_metrics.cyclomatic_complexity, 0);
    assert_eq!(report.complexity_metrics.cognitive_complexity, 0);
    assert!(report.pmat_metrics.tdg > 1.0); // Should be reasonable default score
}

#[test]
fn test_multiple_functions_analysis() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_simple_function(), create_complex_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Should analyze both functions and take max complexity
    assert!(report.complexity_metrics.cyclomatic_complexity >= 3);
    assert!(report.complexity_metrics.statement_count > 5);
}

#[test]
fn test_quality_requirements_evaluation() {
    let _analyzer = QualityAnalyzer::new();

    // Test each quality requirement type
    let req_coverage = QualityRequirement::MinTestCoverage(0.8);
    let req_complexity = QualityRequirement::MaxComplexity(10);
    let req_compilation = QualityRequirement::CompilationSuccess;
    let req_clippy = QualityRequirement::ClippyClean;
    let req_panic_free = QualityRequirement::PanicFree;
    let req_energy = QualityRequirement::EnergyEfficient(0.75);
    let req_min_tdg = QualityRequirement::MinPmatTdg(1.0);
    let req_max_tdg = QualityRequirement::MaxPmatTdg(2.0);

    // Just ensure they can be created and used
    assert!(matches!(
        req_coverage,
        QualityRequirement::MinTestCoverage(_)
    ));
    assert!(matches!(
        req_complexity,
        QualityRequirement::MaxComplexity(_)
    ));
    assert!(matches!(
        req_compilation,
        QualityRequirement::CompilationSuccess
    ));
    assert!(matches!(req_clippy, QualityRequirement::ClippyClean));
    assert!(matches!(req_panic_free, QualityRequirement::PanicFree));
    assert!(matches!(req_energy, QualityRequirement::EnergyEfficient(_)));
    assert!(matches!(req_min_tdg, QualityRequirement::MinPmatTdg(_)));
    assert!(matches!(req_max_tdg, QualityRequirement::MaxPmatTdg(_)));
}

#[test]
fn test_quality_gate_severity_levels() {
    let error_gate = QualityGate {
        name: "Error Gate".to_string(),
        requirements: vec![QualityRequirement::MaxComplexity(5)],
        severity: Severity::Error,
    };

    let warning_gate = QualityGate {
        name: "Warning Gate".to_string(),
        requirements: vec![QualityRequirement::MinTestCoverage(0.9)],
        severity: Severity::Warning,
    };

    let info_gate = QualityGate {
        name: "Info Gate".to_string(),
        requirements: vec![QualityRequirement::EnergyEfficient(0.8)],
        severity: Severity::Info,
    };

    assert_eq!(error_gate.severity, Severity::Error);
    assert_eq!(warning_gate.severity, Severity::Warning);
    assert_eq!(info_gate.severity, Severity::Info);
}

#[test]
fn test_pmat_edge_cases() {
    let analyzer = QualityAnalyzer::new();

    // Test with function that has zero complexity
    let zero_complexity_func = HirFunction {
        name: "empty".to_string(),
        params: smallvec![],
        ret_type: Type::None,
        body: vec![], // Empty body
        properties: FunctionProperties::default(),
        annotations: TranspilationAnnotations::default(),
    };

    let functions = vec![zero_complexity_func];
    let report = analyzer.analyze_quality(&functions).unwrap();

    // Should handle zero complexity gracefully
    assert!(report.pmat_metrics.productivity_score > 0.0);
    assert!(report.pmat_metrics.tdg > 0.0);
}

#[test]
fn test_coverage_metrics_structure() {
    let coverage = CoverageMetrics {
        line_coverage: 0.85,
        branch_coverage: 0.75,
        function_coverage: 0.90,
    };

    assert_eq!(coverage.line_coverage, 0.85);
    assert_eq!(coverage.branch_coverage, 0.75);
    assert_eq!(coverage.function_coverage, 0.90);
}

#[test]
fn test_complexity_metrics_structure() {
    let complexity = ComplexityMetrics {
        cyclomatic_complexity: 15,
        cognitive_complexity: 12,
        max_nesting: 3,
        statement_count: 25,
    };

    assert_eq!(complexity.cyclomatic_complexity, 15);
    assert_eq!(complexity.cognitive_complexity, 12);
    assert_eq!(complexity.max_nesting, 3);
    assert_eq!(complexity.statement_count, 25);
}

#[test]
fn test_quality_status_enum() {
    assert_eq!(QualityStatus::Passed, QualityStatus::Passed);
    assert_eq!(QualityStatus::Failed, QualityStatus::Failed);
    assert_eq!(QualityStatus::Warning, QualityStatus::Warning);

    assert_ne!(QualityStatus::Passed, QualityStatus::Failed);
}

#[test]
fn test_quality_analyzer_creation() {
    let analyzer1 = QualityAnalyzer::new();
    let analyzer2 = QualityAnalyzer::default();

    // Both should create valid analyzers
    let functions = vec![create_simple_function()];

    assert!(analyzer1.analyze_quality(&functions).is_ok());
    assert!(analyzer2.analyze_quality(&functions).is_ok());
}

#[test]
fn test_print_quality_report() {
    let analyzer = QualityAnalyzer::new();
    let functions = vec![create_simple_function()];

    let report = analyzer.analyze_quality(&functions).unwrap();

    // Should not panic when printing report
    analyzer.print_quality_report(&report);
    // If we reach here, the print function didn't panic
    assert!(true);
}
