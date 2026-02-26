pub mod complexity;
pub mod metrics;
pub mod type_flow;

// Re-export complexity functions for easier use
pub use complexity::{
    calculate_cognitive, calculate_cyclomatic, calculate_max_nesting, count_statements,
};

use anyhow::Result;
use depyler_core::hir::{HirFunction, HirModule};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use depyler_annotations::TranspilationAnnotations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub module_metrics: ModuleMetrics,
    pub function_metrics: Vec<FunctionMetrics>,
    pub type_coverage: TypeCoverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetrics {
    pub total_functions: usize,
    pub total_lines: usize,
    pub avg_cyclomatic_complexity: f64,
    pub max_cyclomatic_complexity: u32,
    pub avg_cognitive_complexity: f64,
    pub max_cognitive_complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetrics {
    pub name: String,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub lines_of_code: usize,
    pub parameters: usize,
    pub max_nesting_depth: usize,
    pub has_type_annotations: bool,
    pub return_type_annotated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCoverage {
    pub total_parameters: usize,
    pub annotated_parameters: usize,
    pub total_functions: usize,
    pub functions_with_return_type: usize,
    pub coverage_percentage: f64,
}

pub struct Analyzer {
    #[allow(dead_code)]
    enable_type_inference: bool,
}

impl Analyzer {
    pub fn new() -> Self {
        Self { enable_type_inference: true }
    }

    pub fn analyze(&self, module: &HirModule) -> Result<AnalysisResult> {
        let function_metrics: Vec<FunctionMetrics> = module
            .functions
            .iter()
            .map(|f| self.analyze_function(f))
            .collect::<Result<Vec<_>>>()?;

        let module_metrics = self.calculate_module_metrics(&function_metrics);
        let type_coverage = self.calculate_type_coverage(module);

        Ok(AnalysisResult { module_metrics, function_metrics, type_coverage })
    }

    fn analyze_function(&self, func: &HirFunction) -> Result<FunctionMetrics> {
        let cyclomatic = complexity::calculate_cyclomatic(&func.body);
        let cognitive = complexity::calculate_cognitive(&func.body);
        let max_nesting = complexity::calculate_max_nesting(&func.body);
        let loc = complexity::count_statements(&func.body);

        let has_type_annotations =
            func.params.iter().all(|param| !matches!(param.ty, depyler_core::hir::Type::Unknown));
        let return_type_annotated = !matches!(func.ret_type, depyler_core::hir::Type::Unknown);

        Ok(FunctionMetrics {
            name: func.name.clone(),
            cyclomatic_complexity: cyclomatic,
            cognitive_complexity: cognitive,
            lines_of_code: loc,
            parameters: func.params.len(),
            max_nesting_depth: max_nesting,
            has_type_annotations,
            return_type_annotated,
        })
    }

    fn calculate_module_metrics(&self, functions: &[FunctionMetrics]) -> ModuleMetrics {
        let total_functions = functions.len();
        let total_lines: usize = functions.iter().map(|f| f.lines_of_code).sum();

        let avg_cyclomatic = if total_functions > 0 {
            functions.iter().map(|f| f.cyclomatic_complexity as f64).sum::<f64>()
                / total_functions as f64
        } else {
            0.0
        };

        let max_cyclomatic = functions.iter().map(|f| f.cyclomatic_complexity).max().unwrap_or(0);

        let avg_cognitive = if total_functions > 0 {
            functions.iter().map(|f| f.cognitive_complexity as f64).sum::<f64>()
                / total_functions as f64
        } else {
            0.0
        };

        let max_cognitive = functions.iter().map(|f| f.cognitive_complexity).max().unwrap_or(0);

        ModuleMetrics {
            total_functions,
            total_lines,
            avg_cyclomatic_complexity: avg_cyclomatic,
            max_cyclomatic_complexity: max_cyclomatic,
            avg_cognitive_complexity: avg_cognitive,
            max_cognitive_complexity: max_cognitive,
        }
    }

    fn calculate_type_coverage(&self, module: &HirModule) -> TypeCoverage {
        let mut total_parameters = 0;
        let mut annotated_parameters = 0;
        let mut functions_with_return_type = 0;

        for func in &module.functions {
            total_parameters += func.params.len();
            annotated_parameters += func
                .params
                .iter()
                .filter(|param| !matches!(param.ty, depyler_core::hir::Type::Unknown))
                .count();

            if !matches!(func.ret_type, depyler_core::hir::Type::Unknown) {
                functions_with_return_type += 1;
            }
        }

        let total_annotations = annotated_parameters + functions_with_return_type;
        let total_possible = total_parameters + module.functions.len();
        let coverage_percentage = if total_possible > 0 {
            (total_annotations as f64 / total_possible as f64) * 100.0
        } else {
            100.0
        };

        TypeCoverage {
            total_parameters,
            annotated_parameters,
            total_functions: module.functions.len(),
            functions_with_return_type,
            coverage_percentage,
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use depyler_core::hir::*;

    fn create_test_function() -> HirFunction {
        use smallvec::smallvec;
        HirFunction {
            name: "test_func".to_string(),
            params: smallvec![
                HirParam {
                    name: Symbol::from("x"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("y"),
                    ty: Type::String,
                    default: None,
                    is_vararg: false,
                }
            ],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_analyzer_creation() {
        let analyzer = Analyzer::new();
        assert!(analyzer.enable_type_inference);

        let default_analyzer = Analyzer::default();
        assert!(default_analyzer.enable_type_inference);
    }

    #[test]
    fn test_analyze_empty_module() {
        let analyzer = Analyzer::new();
        let module = HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let result = analyzer.analyze(&module).unwrap();
        assert_eq!(result.module_metrics.total_functions, 0);
        assert_eq!(result.function_metrics.len(), 0);
        assert_eq!(result.type_coverage.total_functions, 0);
        assert_eq!(result.type_coverage.coverage_percentage, 100.0);
    }

    #[test]
    fn test_analyze_single_function() {
        let analyzer = Analyzer::new();
        let func = create_test_function();
        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let result = analyzer.analyze(&module).unwrap();
        assert_eq!(result.module_metrics.total_functions, 1);
        assert_eq!(result.function_metrics.len(), 1);

        let func_metrics = &result.function_metrics[0];
        assert_eq!(func_metrics.name, "test_func");
        assert_eq!(func_metrics.parameters, 2);
        assert!(func_metrics.has_type_annotations);
        assert!(func_metrics.return_type_annotated);
    }

    #[test]
    fn test_type_coverage_calculation() {
        let analyzer = Analyzer::new();
        use smallvec::smallvec;
        let func_with_types = HirFunction {
            name: "typed_func".to_string(),
            params: smallvec![HirParam {
                name: Symbol::from("x"),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::String,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let func_without_types = HirFunction {
            name: "untyped_func".to_string(),
            params: smallvec![HirParam {
                name: Symbol::from("y"),
                ty: Type::Unknown,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Unknown,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let module = HirModule {
            functions: vec![func_with_types, func_without_types],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };

        let coverage = analyzer.calculate_type_coverage(&module);
        assert_eq!(coverage.total_parameters, 2);
        assert_eq!(coverage.annotated_parameters, 1);
        assert_eq!(coverage.total_functions, 2);
        assert_eq!(coverage.functions_with_return_type, 1);
        assert_eq!(coverage.coverage_percentage, 50.0); // 2 annotations out of 4 possible
    }

    // ========================================================================
    // Additional coverage tests
    // ========================================================================

    #[test]
    fn test_analyze_function_with_unknown_types() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "untyped".to_string(),
            params: smallvec![HirParam {
                name: Symbol::from("a"),
                ty: Type::Unknown,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Unknown,
            body: vec![HirStmt::Return(None)],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let result = analyzer.analyze(&module).unwrap();
        let fm = &result.function_metrics[0];
        assert!(!fm.has_type_annotations);
        assert!(!fm.return_type_annotated);
    }

    #[test]
    fn test_analyze_multiple_functions() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func1 = create_test_function();
        let func2 = HirFunction {
            name: "func2".to_string(),
            params: smallvec![],
            ret_type: Type::Bool,
            body: vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(true))))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Bool(
                    false,
                ))))]),
            }],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let module = HirModule {
            functions: vec![func1, func2],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let result = analyzer.analyze(&module).unwrap();
        assert_eq!(result.module_metrics.total_functions, 2);
        assert!(result.module_metrics.max_cyclomatic_complexity >= 2);
    }

    #[test]
    fn test_analysis_result_serialization() {
        let result = AnalysisResult {
            module_metrics: ModuleMetrics {
                total_functions: 1,
                total_lines: 10,
                avg_cyclomatic_complexity: 2.0,
                max_cyclomatic_complexity: 2,
                avg_cognitive_complexity: 1.0,
                max_cognitive_complexity: 1,
            },
            function_metrics: vec![FunctionMetrics {
                name: "test".to_string(),
                cyclomatic_complexity: 2,
                cognitive_complexity: 1,
                lines_of_code: 10,
                parameters: 1,
                max_nesting_depth: 1,
                has_type_annotations: true,
                return_type_annotated: true,
            }],
            type_coverage: TypeCoverage {
                total_parameters: 1,
                annotated_parameters: 1,
                total_functions: 1,
                functions_with_return_type: 1,
                coverage_percentage: 100.0,
            },
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("test"));
        let deserialized: AnalysisResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.module_metrics.total_functions, 1);
    }

    #[test]
    fn test_module_metrics_debug_clone() {
        let metrics = ModuleMetrics {
            total_functions: 5,
            total_lines: 50,
            avg_cyclomatic_complexity: 3.0,
            max_cyclomatic_complexity: 8,
            avg_cognitive_complexity: 2.5,
            max_cognitive_complexity: 6,
        };
        let debug = format!("{:?}", metrics);
        assert!(debug.contains("ModuleMetrics"));
        let cloned = metrics.clone();
        assert_eq!(cloned.total_functions, 5);
    }

    #[test]
    fn test_function_metrics_debug_clone() {
        let fm = FunctionMetrics {
            name: "my_func".to_string(),
            cyclomatic_complexity: 3,
            cognitive_complexity: 2,
            lines_of_code: 15,
            parameters: 2,
            max_nesting_depth: 3,
            has_type_annotations: false,
            return_type_annotated: true,
        };
        let debug = format!("{:?}", fm);
        assert!(debug.contains("my_func"));
        let cloned = fm.clone();
        assert_eq!(cloned.name, "my_func");
    }

    #[test]
    fn test_type_coverage_debug_clone() {
        let tc = TypeCoverage {
            total_parameters: 10,
            annotated_parameters: 7,
            total_functions: 5,
            functions_with_return_type: 3,
            coverage_percentage: 66.67,
        };
        let debug = format!("{:?}", tc);
        assert!(debug.contains("TypeCoverage"));
        let cloned = tc.clone();
        assert_eq!(cloned.total_parameters, 10);
    }

    #[test]
    fn test_module_metrics_empty_functions() {
        let analyzer = Analyzer::new();
        let empty: Vec<FunctionMetrics> = vec![];
        let metrics = analyzer.calculate_module_metrics(&empty);
        assert_eq!(metrics.total_functions, 0);
        assert_eq!(metrics.total_lines, 0);
        assert_eq!(metrics.avg_cyclomatic_complexity, 0.0);
        assert_eq!(metrics.max_cyclomatic_complexity, 0);
        assert_eq!(metrics.avg_cognitive_complexity, 0.0);
        assert_eq!(metrics.max_cognitive_complexity, 0);
    }

    #[test]
    fn test_type_coverage_all_annotated() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "all_typed".to_string(),
            params: smallvec![
                HirParam {
                    name: Symbol::from("a"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("b"),
                    ty: Type::String,
                    default: None,
                    is_vararg: false,
                },
            ],
            ret_type: Type::Bool,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let coverage = analyzer.calculate_type_coverage(&module);
        assert_eq!(coverage.coverage_percentage, 100.0);
    }

    // ========================================================================
    // S9B7: Additional coverage tests for analyzer edge cases
    // ========================================================================

    #[test]
    fn test_s9b7_analyze_function_with_nested_if() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "nested".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::If {
                    condition: HirExpr::Literal(Literal::Bool(false)),
                    then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                    else_body: None,
                }],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(2))))]),
            }],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let result = analyzer.analyze(&module).unwrap();
        let fm = &result.function_metrics[0];
        assert!(fm.cyclomatic_complexity >= 3);
        assert!(fm.max_nesting_depth >= 2);
    }

    #[test]
    fn test_s9b7_type_coverage_no_functions() {
        let analyzer = Analyzer::new();
        let module = HirModule {
            functions: vec![],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let coverage = analyzer.calculate_type_coverage(&module);
        assert_eq!(coverage.total_parameters, 0);
        assert_eq!(coverage.annotated_parameters, 0);
        assert_eq!(coverage.total_functions, 0);
        assert_eq!(coverage.functions_with_return_type, 0);
        assert_eq!(coverage.coverage_percentage, 100.0);
    }

    #[test]
    fn test_s9b7_analyze_function_empty_body() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "empty_body".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let result = analyzer.analyze_function(&func).unwrap();
        assert_eq!(result.name, "empty_body");
        assert_eq!(result.lines_of_code, 0);
        assert_eq!(result.parameters, 0);
        assert_eq!(result.max_nesting_depth, 0);
        assert!(result.has_type_annotations);
        assert!(result.return_type_annotated);
    }

    #[test]
    fn test_s9b7_type_coverage_mixed_annotations() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "mixed".to_string(),
            params: smallvec![
                HirParam {
                    name: Symbol::from("a"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("b"),
                    ty: Type::Unknown,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("c"),
                    ty: Type::Float,
                    default: None,
                    is_vararg: false,
                },
            ],
            ret_type: Type::Unknown,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let module = HirModule {
            functions: vec![func],
            imports: vec![],
            type_aliases: vec![],
            protocols: vec![],
            classes: vec![],
            constants: vec![],
            top_level_stmts: vec![],
        };
        let coverage = analyzer.calculate_type_coverage(&module);
        assert_eq!(coverage.total_parameters, 3);
        assert_eq!(coverage.annotated_parameters, 2);
        assert_eq!(coverage.functions_with_return_type, 0);
        // 2 annotated params + 0 return types out of 3 params + 1 function = 2/4 = 50%
        assert!((coverage.coverage_percentage - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_s9b7_module_metrics_single_function() {
        let analyzer = Analyzer::new();
        let metrics = vec![FunctionMetrics {
            name: "solo".to_string(),
            cyclomatic_complexity: 5,
            cognitive_complexity: 3,
            lines_of_code: 10,
            parameters: 2,
            max_nesting_depth: 1,
            has_type_annotations: true,
            return_type_annotated: true,
        }];
        let mm = analyzer.calculate_module_metrics(&metrics);
        assert_eq!(mm.total_functions, 1);
        assert_eq!(mm.total_lines, 10);
        assert_eq!(mm.avg_cyclomatic_complexity, 5.0);
        assert_eq!(mm.max_cyclomatic_complexity, 5);
        assert_eq!(mm.avg_cognitive_complexity, 3.0);
        assert_eq!(mm.max_cognitive_complexity, 3);
    }

    #[test]
    fn test_s9b7_analysis_result_debug_clone() {
        let result = AnalysisResult {
            module_metrics: ModuleMetrics {
                total_functions: 2,
                total_lines: 20,
                avg_cyclomatic_complexity: 3.0,
                max_cyclomatic_complexity: 4,
                avg_cognitive_complexity: 2.0,
                max_cognitive_complexity: 3,
            },
            function_metrics: vec![],
            type_coverage: TypeCoverage {
                total_parameters: 0,
                annotated_parameters: 0,
                total_functions: 0,
                functions_with_return_type: 0,
                coverage_percentage: 100.0,
            },
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("AnalysisResult"));
        let cloned = result.clone();
        assert_eq!(cloned.module_metrics.total_functions, 2);
    }

    #[test]
    fn test_s9b7_analyze_function_partial_type_annotations() {
        use smallvec::smallvec;
        let analyzer = Analyzer::new();
        let func = HirFunction {
            name: "partial".to_string(),
            params: smallvec![
                HirParam {
                    name: Symbol::from("x"),
                    ty: Type::Int,
                    default: None,
                    is_vararg: false,
                },
                HirParam {
                    name: Symbol::from("y"),
                    ty: Type::Unknown,
                    default: None,
                    is_vararg: false,
                },
            ],
            ret_type: Type::String,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };
        let result = analyzer.analyze_function(&func).unwrap();
        // Not all params have type annotations
        assert!(!result.has_type_annotations);
        assert!(result.return_type_annotated);
    }

    #[test]
    fn test_module_metrics_calculation() {
        let analyzer = Analyzer::new();
        let metrics = vec![
            FunctionMetrics {
                name: "func1".to_string(),
                cyclomatic_complexity: 2,
                cognitive_complexity: 3,
                lines_of_code: 5,
                parameters: 1,
                max_nesting_depth: 1,
                has_type_annotations: true,
                return_type_annotated: true,
            },
            FunctionMetrics {
                name: "func2".to_string(),
                cyclomatic_complexity: 4,
                cognitive_complexity: 6,
                lines_of_code: 10,
                parameters: 2,
                max_nesting_depth: 2,
                has_type_annotations: false,
                return_type_annotated: false,
            },
        ];

        let module_metrics = analyzer.calculate_module_metrics(&metrics);
        assert_eq!(module_metrics.total_functions, 2);
        assert_eq!(module_metrics.total_lines, 15);
        assert_eq!(module_metrics.avg_cyclomatic_complexity, 3.0);
        assert_eq!(module_metrics.max_cyclomatic_complexity, 4);
        assert_eq!(module_metrics.avg_cognitive_complexity, 4.5);
        assert_eq!(module_metrics.max_cognitive_complexity, 6);
    }
}
