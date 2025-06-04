pub mod complexity;
pub mod metrics;
pub mod type_flow;

use anyhow::Result;
use depyler_core::hir::{HirFunction, HirModule};
use serde::{Deserialize, Serialize};

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
            .all(|(_, ty)| !matches!(ty, depyler_core::hir::Type::Unknown));
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
                .filter(|(_, ty)| !matches!(ty, depyler_core::hir::Type::Unknown))
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
