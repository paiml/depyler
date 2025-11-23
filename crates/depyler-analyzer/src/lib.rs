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
        Self {
            enable_type_inference: true,
        }
    }

    pub fn analyze(&self, module: &HirModule) -> Result<AnalysisResult> {
        let function_metrics: Vec<FunctionMetrics> = module
            .functions
            .iter()
            .map(|f| self.analyze_function(f))
            .collect::<Result<Vec<_>>>()?;

        let module_metrics = self.calculate_module_metrics(&function_metrics);
        let type_coverage = self.calculate_type_coverage(module);

        Ok(AnalysisResult {
            module_metrics,
            function_metrics,
            type_coverage,
        })
    }

    fn analyze_function(&self, func: &HirFunction) -> Result<FunctionMetrics> {
        let cyclomatic = complexity::calculate_cyclomatic(&func.body);
        let cognitive = complexity::calculate_cognitive(&func.body);
        let max_nesting = complexity::calculate_max_nesting(&func.body);
        let loc = complexity::count_statements(&func.body);

        let has_type_annotations = func
            .params
            .iter()
            .all(|param| !matches!(param.ty, depyler_core::hir::Type::Unknown));
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
            functions
                .iter()
                .map(|f| f.cyclomatic_complexity as f64)
                .sum::<f64>()
                / total_functions as f64
        } else {
            0.0
        };

        let max_cyclomatic = functions
            .iter()
            .map(|f| f.cyclomatic_complexity)
            .max()
            .unwrap_or(0);

        let avg_cognitive = if total_functions > 0 {
            functions
                .iter()
                .map(|f| f.cognitive_complexity as f64)
                .sum::<f64>()
                / total_functions as f64
        } else {
            0.0
        };

        let max_cognitive = functions
            .iter()
            .map(|f| f.cognitive_complexity)
            .max()
            .unwrap_or(0);

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
        };

        let coverage = analyzer.calculate_type_coverage(&module);
        assert_eq!(coverage.total_parameters, 2);
        assert_eq!(coverage.annotated_parameters, 1);
        assert_eq!(coverage.total_functions, 2);
        assert_eq!(coverage.functions_with_return_type, 1);
        assert_eq!(coverage.coverage_percentage, 50.0); // 2 annotations out of 4 possible
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
