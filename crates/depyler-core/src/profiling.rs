//! Profiling integration for performance analysis of transpiled code
//!
//! This module provides tools to profile and analyze the performance characteristics
//! of Python code and its transpiled Rust equivalent, helping developers understand
//! performance improvements and bottlenecks.

use crate::hir::{HirExpr, HirFunction, HirProgram, HirStmt};
use colored::Colorize;
use std::collections::HashMap;

/// Profiling configuration and results collector
pub struct Profiler {
    /// Configuration for profiling
    config: ProfileConfig,
    /// Collected metrics
    metrics: HashMap<String, FunctionMetrics>,
    /// Hot path analysis results
    hot_paths: Vec<HotPath>,
    /// Performance predictions
    predictions: Vec<PerformancePrediction>,
}

#[derive(Debug, Clone)]
pub struct ProfileConfig {
    /// Enable instruction counting
    pub count_instructions: bool,
    /// Enable memory allocation tracking
    pub track_allocations: bool,
    /// Enable hot path detection
    pub detect_hot_paths: bool,
    /// Minimum samples for hot path detection
    pub hot_path_threshold: usize,
    /// Generate flame graph data
    pub generate_flamegraph: bool,
    /// Include performance hints
    pub include_hints: bool,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            count_instructions: true,
            track_allocations: true,
            detect_hot_paths: true,
            hot_path_threshold: 100,
            generate_flamegraph: false,
            include_hints: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionMetrics {
    /// Function name
    pub name: String,
    /// Estimated instruction count
    pub instruction_count: usize,
    /// Estimated memory allocations
    pub allocation_count: usize,
    /// Estimated execution time (relative)
    pub estimated_time: f64,
    /// Number of times called (if detectable)
    pub call_count: usize,
    /// Percentage of total program time
    pub time_percentage: f64,
    /// Whether this is a hot function
    pub is_hot: bool,
}

#[derive(Debug, Clone)]
pub struct HotPath {
    /// Functions in the call chain
    pub call_chain: Vec<String>,
    /// Estimated percentage of execution time
    pub time_percentage: f64,
    /// Loop depth in the path
    pub loop_depth: usize,
    /// Whether path contains I/O
    pub has_io: bool,
}

#[derive(Debug, Clone)]
pub struct PerformancePrediction {
    /// Type of prediction
    pub category: PredictionCategory,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Predicted speedup factor
    pub speedup_factor: f64,
    /// Explanation
    pub explanation: String,
    /// Affected functions
    pub functions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PredictionCategory {
    /// Type system eliminates runtime checks
    TypeSystemOptimization,
    /// Zero-cost abstractions in Rust
    ZeroCostAbstraction,
    /// Memory layout improvements
    MemoryLayoutOptimization,
    /// Iterator fusion and optimization
    IteratorOptimization,
    /// String handling improvements
    StringOptimization,
    /// Parallelization opportunities
    ParallelizationOpportunity,
}

/// Profiling annotations that can be added to generated code
#[derive(Debug, Clone)]
pub struct ProfilingAnnotation {
    /// Annotation type
    pub kind: AnnotationKind,
    /// Target function or location
    pub target: String,
    /// Annotation value
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum AnnotationKind {
    /// Instrument with timing
    TimingProbe,
    /// Count allocations
    AllocationCounter,
    /// Mark as hot path
    HotPathMarker,
    /// Performance hint
    PerformanceHint,
}

impl Profiler {
    pub fn new(config: ProfileConfig) -> Self {
        Self {
            config,
            metrics: HashMap::new(),
            hot_paths: Vec::new(),
            predictions: Vec::new(),
        }
    }

    /// Analyze a program for profiling insights
    pub fn analyze_program(&mut self, program: &HirProgram) -> ProfilingReport {
        // Clear previous results
        self.metrics.clear();
        self.hot_paths.clear();
        self.predictions.clear();

        // Analyze each function
        let mut total_instructions = 0;
        let mut total_allocations = 0;

        for func in &program.functions {
            let metrics = self.analyze_function(func);
            total_instructions += metrics.instruction_count;
            total_allocations += metrics.allocation_count;
            self.metrics.insert(func.name.clone(), metrics);
        }

        // Calculate time percentages
        for metrics in self.metrics.values_mut() {
            metrics.time_percentage = (metrics.estimated_time / total_instructions as f64) * 100.0;
            metrics.is_hot = metrics.time_percentage > 10.0;
        }

        // Detect hot paths
        if self.config.detect_hot_paths {
            self.detect_hot_paths(program);
        }

        // Generate performance predictions
        self.generate_predictions(program);

        // Create report
        ProfilingReport {
            metrics: self.metrics.clone(),
            hot_paths: self.hot_paths.clone(),
            predictions: self.predictions.clone(),
            total_instructions,
            total_allocations,
            annotations: self.generate_annotations(),
        }
    }

    fn analyze_function(&self, func: &HirFunction) -> FunctionMetrics {
        let mut instruction_count = 0;
        let mut allocation_count = 0;
        let mut loop_multiplier = 1.0;

        // Analyze function body
        for stmt in &func.body {
            let (inst, alloc, loop_factor) = self.analyze_stmt(stmt, 1);
            instruction_count += inst;
            allocation_count += alloc;
            loop_multiplier *= loop_factor;
        }

        // Estimate execution time based on instruction count and loop factors
        let estimated_time = instruction_count as f64 * loop_multiplier;

        FunctionMetrics {
            name: func.name.clone(),
            instruction_count,
            allocation_count,
            estimated_time,
            call_count: 0,        // Would need call graph analysis
            time_percentage: 0.0, // Calculated later
            is_hot: false,        // Determined later
        }
    }

    fn analyze_stmt(&self, stmt: &HirStmt, loop_depth: usize) -> (usize, usize, f64) {
        match stmt {
            HirStmt::Assign { value, .. } => self.analyze_assign(value),
            HirStmt::Expr(expr) => self.analyze_expr_stmt(expr),
            HirStmt::Return(Some(expr)) => self.analyze_return_with_value(expr),
            HirStmt::Return(None) => (1, 0, 1.0),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.analyze_if(condition, then_body, else_body.as_deref(), loop_depth),
            HirStmt::While { condition, body } => self.analyze_while(condition, body, loop_depth),
            HirStmt::For { iter, body, .. } => self.analyze_for(iter, body, loop_depth),
            _ => (1, 0, 1.0),
        }
    }

    fn analyze_assign(&self, value: &HirExpr) -> (usize, usize, f64) {
        let (inst, alloc) = self.analyze_expr(value);
        (inst + 1, alloc, 1.0)
    }

    fn analyze_expr_stmt(&self, expr: &HirExpr) -> (usize, usize, f64) {
        let (inst, alloc) = self.analyze_expr(expr);
        (inst, alloc, 1.0)
    }

    fn analyze_return_with_value(&self, expr: &HirExpr) -> (usize, usize, f64) {
        let (inst, alloc) = self.analyze_expr(expr);
        (inst + 1, alloc, 1.0)
    }

    fn analyze_if(
        &self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: Option<&[HirStmt]>,
        loop_depth: usize,
    ) -> (usize, usize, f64) {
        let (cond_inst, cond_alloc) = self.analyze_expr(condition);
        let mut total_inst = cond_inst + 1;
        let mut total_alloc = cond_alloc;

        for stmt in then_body {
            let (inst, alloc, _) = self.analyze_stmt(stmt, loop_depth);
            total_inst += inst;
            total_alloc += alloc;
        }

        if let Some(else_stmts) = else_body {
            for stmt in else_stmts {
                let (inst, alloc, _) = self.analyze_stmt(stmt, loop_depth);
                total_inst += inst / 2;
                total_alloc += alloc / 2;
            }
        }

        (total_inst, total_alloc, 1.0)
    }

    fn analyze_while(
        &self,
        condition: &HirExpr,
        body: &[HirStmt],
        loop_depth: usize,
    ) -> (usize, usize, f64) {
        let (cond_inst, cond_alloc) = self.analyze_expr(condition);
        let (body_inst, body_alloc) = self.analyze_body(body, loop_depth + 1);
        let loop_factor = 10.0_f64.powi(loop_depth as i32);
        (
            cond_inst + (body_inst * 10),
            cond_alloc + (body_alloc * 10),
            loop_factor,
        )
    }

    fn analyze_for(
        &self,
        iter: &HirExpr,
        body: &[HirStmt],
        loop_depth: usize,
    ) -> (usize, usize, f64) {
        let (iter_inst, iter_alloc) = self.analyze_expr(iter);
        let (body_inst, body_alloc) = self.analyze_body(body, loop_depth + 1);
        let loop_factor = 10.0_f64.powi(loop_depth as i32);
        (
            iter_inst + (body_inst * 10),
            iter_alloc + (body_alloc * 10),
            loop_factor,
        )
    }

    fn analyze_body(&self, body: &[HirStmt], loop_depth: usize) -> (usize, usize) {
        let mut body_inst = 0;
        let mut body_alloc = 0;
        for stmt in body {
            let (inst, alloc, _) = self.analyze_stmt(stmt, loop_depth);
            body_inst += inst;
            body_alloc += alloc;
        }
        (body_inst, body_alloc)
    }

    fn analyze_expr(&self, expr: &HirExpr) -> (usize, usize) {
        analyze_expr_inner(expr)
    }

    fn detect_hot_paths(&mut self, _program: &HirProgram) {
        // Find functions that consume > 10% of time
        let hot_functions: Vec<_> = self
            .metrics
            .values()
            .filter(|m| m.is_hot)
            .map(|m| m.name.clone())
            .collect();

        // For now, create simple hot paths from hot functions
        for func_name in hot_functions {
            if let Some(metrics) = self.metrics.get(&func_name) {
                self.hot_paths.push(HotPath {
                    call_chain: vec![func_name],
                    time_percentage: metrics.time_percentage,
                    loop_depth: 0, // Would need more analysis
                    has_io: false, // Would need I/O detection
                });
            }
        }
    }

    fn generate_predictions(&mut self, program: &HirProgram) {
        // Type system optimization prediction
        let type_checks_removed = self.count_type_checks(program);
        if type_checks_removed > 0 {
            self.predictions.push(PerformancePrediction {
                category: PredictionCategory::TypeSystemOptimization,
                confidence: 0.9,
                speedup_factor: 1.0 + (type_checks_removed as f64 * 0.1),
                explanation: format!(
                    "Rust's type system eliminates {} runtime type checks",
                    type_checks_removed
                ),
                functions: vec![],
            });
        }

        // Iterator optimization prediction
        let iterator_opportunities = self.count_iterator_opportunities(program);
        if iterator_opportunities > 0 {
            self.predictions.push(PerformancePrediction {
                category: PredictionCategory::IteratorOptimization,
                confidence: 0.8,
                speedup_factor: 1.2,
                explanation: "Rust's iterator fusion can optimize chained operations".to_string(),
                functions: vec![],
            });
        }

        // Memory layout optimization
        self.predictions.push(PerformancePrediction {
            category: PredictionCategory::MemoryLayoutOptimization,
            confidence: 0.7,
            speedup_factor: 1.3,
            explanation: "Rust's memory layout is more cache-friendly than Python".to_string(),
            functions: vec![],
        });
    }

    fn count_type_checks(&self, program: &HirProgram) -> usize {
        let mut count = 0;
        for func in &program.functions {
            for stmt in &func.body {
                count += self.count_type_checks_in_stmt(stmt);
            }
        }
        count
    }

    fn count_type_checks_in_stmt(&self, stmt: &HirStmt) -> usize {
        match stmt {
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let mut count = 0;
                if self.is_type_check_expr(condition) {
                    count += 1;
                }
                for s in then_body {
                    count += self.count_type_checks_in_stmt(s);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        count += self.count_type_checks_in_stmt(s);
                    }
                }
                count
            }
            _ => 0,
        }
    }

    fn is_type_check_expr(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Call { func, .. } = expr {
            func == "isinstance" || func == "type"
        } else {
            false
        }
    }

    fn count_iterator_opportunities(&self, program: &HirProgram) -> usize {
        let mut count = 0;
        for func in &program.functions {
            for stmt in &func.body {
                if matches!(stmt, HirStmt::For { .. }) {
                    count += 1;
                }
            }
        }
        count
    }

    fn generate_annotations(&self) -> Vec<ProfilingAnnotation> {
        let mut annotations = Vec::new();

        // Add timing probes for hot functions
        for (name, metrics) in &self.metrics {
            if metrics.is_hot {
                annotations.push(ProfilingAnnotation {
                    kind: AnnotationKind::TimingProbe,
                    target: name.clone(),
                    value: format!("hot_function_{}", name),
                });
            }
        }

        // Add allocation counters for functions with high allocation
        for (name, metrics) in &self.metrics {
            if metrics.allocation_count > 10 {
                annotations.push(ProfilingAnnotation {
                    kind: AnnotationKind::AllocationCounter,
                    target: name.clone(),
                    value: format!("alloc_count_{}", metrics.allocation_count),
                });
            }
        }

        annotations
    }
}

/// Profiling report containing all analysis results
#[derive(Debug, Clone)]
pub struct ProfilingReport {
    /// Function-level metrics
    pub metrics: HashMap<String, FunctionMetrics>,
    /// Detected hot paths
    pub hot_paths: Vec<HotPath>,
    /// Performance predictions
    pub predictions: Vec<PerformancePrediction>,
    /// Total instruction count estimate
    pub total_instructions: usize,
    /// Total allocation count estimate
    pub total_allocations: usize,
    /// Profiling annotations for code generation
    pub annotations: Vec<ProfilingAnnotation>,
}

impl ProfilingReport {
    /// Format the report for display
    pub fn format_report(&self) -> String {
        let mut output = String::new();
        self.format_header(&mut output);
        self.format_summary(&mut output);
        self.format_hot_paths(&mut output);
        self.format_function_metrics(&mut output);
        self.format_predictions(&mut output);
        self.format_overall_speedup(&mut output);
        output
    }

    fn format_header(&self, output: &mut String) {
        output.push_str(&format!("\n{}\n", "Profiling Report".bold().blue()));
        output.push_str(&format!("{}\n\n", "═".repeat(50)));
    }

    fn format_summary(&self, output: &mut String) {
        output.push_str(&format!("{}\n", "Summary".bold()));
        output.push_str(&format!(
            "  Total estimated instructions: {}\n",
            self.total_instructions.to_string().yellow()
        ));
        output.push_str(&format!(
            "  Total estimated allocations: {}\n",
            self.total_allocations.to_string().yellow()
        ));
        output.push_str(&format!(
            "  Functions analyzed: {}\n\n",
            self.metrics.len().to_string().yellow()
        ));
    }

    fn format_hot_paths(&self, output: &mut String) {
        if self.hot_paths.is_empty() {
            return;
        }
        output.push_str(&format!("{}\n", "Hot Paths".bold().red()));
        for (idx, path) in self.hot_paths.iter().enumerate() {
            output.push_str(&format!(
                "  [{}] {} ({:.1}% of execution time)\n",
                idx + 1,
                path.call_chain.join(" → "),
                path.time_percentage
            ));
        }
        output.push('\n');
    }

    fn format_function_metrics(&self, output: &mut String) {
        output.push_str(&format!("{}\n", "Function Metrics".bold()));
        let mut sorted_metrics: Vec<_> = self.metrics.values().collect();
        sorted_metrics.sort_by(|a, b| b.time_percentage.partial_cmp(&a.time_percentage).unwrap());

        for metrics in sorted_metrics.iter().take(10) {
            let hot_marker = if metrics.is_hot { "🔥" } else { "  " };
            output.push_str(&format!(
                "{} {:<30} {:>6.1}% time | {:>6} inst | {:>4} alloc\n",
                hot_marker,
                metrics.name,
                metrics.time_percentage,
                metrics.instruction_count,
                metrics.allocation_count
            ));
        }
        output.push('\n');
    }

    fn format_predictions(&self, output: &mut String) {
        if self.predictions.is_empty() {
            return;
        }
        output.push_str(&format!("{}\n", "Performance Predictions".bold().green()));
        for pred in &self.predictions {
            output.push_str(&format!(
                "  • {} ({}x speedup, {:.0}% confidence)\n",
                pred.explanation,
                format!("{:.1}", pred.speedup_factor).green(),
                pred.confidence * 100.0
            ));
        }
        output.push('\n');
    }

    fn format_overall_speedup(&self, output: &mut String) {
        let total_speedup: f64 = self.predictions.iter().map(|p| p.speedup_factor).product();
        if total_speedup > 1.0 {
            output.push_str(&format!(
                "{} Estimated overall speedup: {}x\n",
                "🚀".green(),
                format!("{:.1}", total_speedup).bold().green()
            ));
        }
    }

    /// Generate flame graph data in collapsed format
    pub fn generate_flamegraph_data(&self) -> String {
        let mut lines = Vec::new();

        for (func_name, metrics) in &self.metrics {
            // Simple format: function_name sample_count
            let sample_count = (metrics.time_percentage * 100.0) as usize;
            if sample_count > 0 {
                lines.push(format!("{} {}", func_name, sample_count));
            }
        }

        lines.join("\n")
    }

    /// Generate perf-compatible annotations
    pub fn generate_perf_annotations(&self) -> String {
        let annotations: Vec<String> = self
            .annotations
            .iter()
            .map(|annotation| self.format_annotation(annotation))
            .collect();
        annotations.join("\n")
    }

    fn format_annotation(&self, annotation: &ProfilingAnnotation) -> String {
        match annotation.kind {
            AnnotationKind::TimingProbe => self.format_timing_probe(&annotation.target),
            AnnotationKind::AllocationCounter => {
                self.format_allocation_counter(&annotation.target, &annotation.value)
            }
            AnnotationKind::HotPathMarker => self.format_hot_path_marker(&annotation.target),
            AnnotationKind::PerformanceHint => {
                self.format_performance_hint(&annotation.target, &annotation.value)
            }
        }
    }

    fn format_timing_probe(&self, target: &str) -> String {
        format!("# @probe {}: timing probe", target)
    }

    fn format_allocation_counter(&self, target: &str, value: &str) -> String {
        format!("# @probe {}: allocation counter = {}", target, value)
    }

    fn format_hot_path_marker(&self, target: &str) -> String {
        format!("# @hot {}: hot path marker", target)
    }

    fn format_performance_hint(&self, target: &str, value: &str) -> String {
        format!("# @hint {}: {}", target, value)
    }
}

fn analyze_expr_inner(expr: &HirExpr) -> (usize, usize) {
    match expr {
        HirExpr::Literal(_) => (1, 0),
        HirExpr::Var(_) => (1, 0),
        HirExpr::Binary { left, right, .. } => analyze_binary_expr(left, right),
        HirExpr::Call { args, .. } => analyze_call_expr(args),
        HirExpr::List(items) => analyze_list_expr(items),
        HirExpr::Dict(pairs) => analyze_dict_expr(pairs),
        _ => (1, 0),
    }
}

fn analyze_binary_expr(left: &HirExpr, right: &HirExpr) -> (usize, usize) {
    let (l_inst, l_alloc) = analyze_expr_inner(left);
    let (r_inst, r_alloc) = analyze_expr_inner(right);
    (l_inst + r_inst + 1, l_alloc + r_alloc)
}

fn analyze_call_expr(args: &[HirExpr]) -> (usize, usize) {
    let mut total_inst = 10;
    let mut total_alloc = 0;
    for arg in args {
        let (inst, alloc) = analyze_expr_inner(arg);
        total_inst += inst;
        total_alloc += alloc;
    }
    (total_inst, total_alloc)
}

fn analyze_list_expr(items: &[HirExpr]) -> (usize, usize) {
    let mut total_inst = 1;
    let total_alloc = 1;
    for item in items {
        let (inst, _) = analyze_expr_inner(item);
        total_inst += inst;
    }
    (total_inst, total_alloc)
}

fn analyze_dict_expr(pairs: &[(HirExpr, HirExpr)]) -> (usize, usize) {
    let mut total_inst = 1;
    let total_alloc = 1;
    for (k, v) in pairs {
        let (k_inst, _) = analyze_expr_inner(k);
        let (v_inst, _) = analyze_expr_inner(v);
        total_inst += k_inst + v_inst + 2;
    }
    (total_inst, total_alloc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    fn create_test_function(name: &str, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    // === ProfileConfig tests ===

    #[test]
    fn test_profile_config_default() {
        let config = ProfileConfig::default();
        assert!(config.count_instructions);
        assert!(config.track_allocations);
        assert!(config.detect_hot_paths);
        assert_eq!(config.hot_path_threshold, 100);
        assert!(!config.generate_flamegraph);
        assert!(config.include_hints);
    }

    #[test]
    fn test_profile_config_clone() {
        let config = ProfileConfig {
            count_instructions: false,
            hot_path_threshold: 50,
            ..Default::default()
        };
        let cloned = config.clone();
        assert_eq!(cloned.count_instructions, false);
        assert_eq!(cloned.hot_path_threshold, 50);
    }

    #[test]
    fn test_profile_config_debug() {
        let config = ProfileConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("ProfileConfig"));
    }

    // === FunctionMetrics tests ===

    #[test]
    fn test_function_metrics_new() {
        let metrics = FunctionMetrics {
            name: "test_func".to_string(),
            instruction_count: 100,
            allocation_count: 5,
            estimated_time: 150.0,
            call_count: 10,
            time_percentage: 25.0,
            is_hot: true,
        };
        assert_eq!(metrics.name, "test_func");
        assert!(metrics.is_hot);
    }

    #[test]
    fn test_function_metrics_clone() {
        let metrics = FunctionMetrics {
            name: "clone_test".to_string(),
            instruction_count: 50,
            allocation_count: 2,
            estimated_time: 75.0,
            call_count: 5,
            time_percentage: 10.0,
            is_hot: false,
        };
        let cloned = metrics.clone();
        assert_eq!(metrics.name, cloned.name);
    }

    #[test]
    fn test_function_metrics_debug() {
        let metrics = FunctionMetrics {
            name: "debug".to_string(),
            instruction_count: 0,
            allocation_count: 0,
            estimated_time: 0.0,
            call_count: 0,
            time_percentage: 0.0,
            is_hot: false,
        };
        let debug = format!("{:?}", metrics);
        assert!(debug.contains("FunctionMetrics"));
    }

    // === HotPath tests ===

    #[test]
    fn test_hot_path_new() {
        let path = HotPath {
            call_chain: vec!["main".to_string(), "process".to_string()],
            time_percentage: 45.0,
            loop_depth: 2,
            has_io: true,
        };
        assert_eq!(path.call_chain.len(), 2);
        assert!(path.has_io);
    }

    #[test]
    fn test_hot_path_clone() {
        let path = HotPath {
            call_chain: vec!["func".to_string()],
            time_percentage: 30.0,
            loop_depth: 1,
            has_io: false,
        };
        let cloned = path.clone();
        assert_eq!(path.time_percentage, cloned.time_percentage);
    }

    #[test]
    fn test_hot_path_debug() {
        let path = HotPath {
            call_chain: vec![],
            time_percentage: 0.0,
            loop_depth: 0,
            has_io: false,
        };
        let debug = format!("{:?}", path);
        assert!(debug.contains("HotPath"));
    }

    // === PerformancePrediction tests ===

    #[test]
    fn test_performance_prediction_new() {
        let pred = PerformancePrediction {
            category: PredictionCategory::TypeSystemOptimization,
            confidence: 0.9,
            speedup_factor: 2.5,
            explanation: "Type checks removed".to_string(),
            functions: vec!["func1".to_string()],
        };
        assert_eq!(pred.confidence, 0.9);
        assert_eq!(pred.speedup_factor, 2.5);
    }

    #[test]
    fn test_performance_prediction_clone() {
        let pred = PerformancePrediction {
            category: PredictionCategory::IteratorOptimization,
            confidence: 0.8,
            speedup_factor: 1.5,
            explanation: "test".to_string(),
            functions: vec![],
        };
        let cloned = pred.clone();
        assert_eq!(pred.category, cloned.category);
    }

    #[test]
    fn test_performance_prediction_debug() {
        let pred = PerformancePrediction {
            category: PredictionCategory::StringOptimization,
            confidence: 0.5,
            speedup_factor: 1.0,
            explanation: "".to_string(),
            functions: vec![],
        };
        let debug = format!("{:?}", pred);
        assert!(debug.contains("PerformancePrediction"));
    }

    // === PredictionCategory tests ===

    #[test]
    fn test_prediction_category_variants() {
        let categories = [
            PredictionCategory::TypeSystemOptimization,
            PredictionCategory::ZeroCostAbstraction,
            PredictionCategory::MemoryLayoutOptimization,
            PredictionCategory::IteratorOptimization,
            PredictionCategory::StringOptimization,
            PredictionCategory::ParallelizationOpportunity,
        ];
        assert_eq!(categories.len(), 6);
    }

    #[test]
    fn test_prediction_category_eq() {
        assert_eq!(
            PredictionCategory::TypeSystemOptimization,
            PredictionCategory::TypeSystemOptimization
        );
        assert_ne!(
            PredictionCategory::TypeSystemOptimization,
            PredictionCategory::StringOptimization
        );
    }

    #[test]
    fn test_prediction_category_clone() {
        let cat = PredictionCategory::MemoryLayoutOptimization;
        let cloned = cat.clone();
        assert_eq!(cat, cloned);
    }

    #[test]
    fn test_prediction_category_debug() {
        let debug = format!("{:?}", PredictionCategory::ZeroCostAbstraction);
        assert!(debug.contains("ZeroCostAbstraction"));
    }

    // === ProfilingAnnotation tests ===

    #[test]
    fn test_profiling_annotation_new() {
        let annotation = ProfilingAnnotation {
            kind: AnnotationKind::TimingProbe,
            target: "func".to_string(),
            value: "probe_1".to_string(),
        };
        assert_eq!(annotation.target, "func");
    }

    #[test]
    fn test_profiling_annotation_clone() {
        let annotation = ProfilingAnnotation {
            kind: AnnotationKind::AllocationCounter,
            target: "test".to_string(),
            value: "100".to_string(),
        };
        let cloned = annotation.clone();
        assert_eq!(annotation.target, cloned.target);
    }

    #[test]
    fn test_profiling_annotation_debug() {
        let annotation = ProfilingAnnotation {
            kind: AnnotationKind::HotPathMarker,
            target: "".to_string(),
            value: "".to_string(),
        };
        let debug = format!("{:?}", annotation);
        assert!(debug.contains("ProfilingAnnotation"));
    }

    // === AnnotationKind tests ===

    #[test]
    fn test_annotation_kind_variants() {
        let kinds = [
            AnnotationKind::TimingProbe,
            AnnotationKind::AllocationCounter,
            AnnotationKind::HotPathMarker,
            AnnotationKind::PerformanceHint,
        ];
        assert_eq!(kinds.len(), 4);
    }

    #[test]
    fn test_annotation_kind_clone() {
        let kind = AnnotationKind::PerformanceHint;
        let cloned = kind.clone();
        assert!(matches!(cloned, AnnotationKind::PerformanceHint));
    }

    #[test]
    fn test_annotation_kind_debug() {
        let debug = format!("{:?}", AnnotationKind::TimingProbe);
        assert!(debug.contains("TimingProbe"));
    }

    // === Profiler tests ===

    #[test]
    fn test_profiler_creation() {
        let config = ProfileConfig::default();
        let profiler = Profiler::new(config);
        assert!(profiler.metrics.is_empty());
        assert!(profiler.hot_paths.is_empty());
    }

    #[test]
    fn test_profiler_empty_program() {
        let mut profiler = Profiler::new(ProfileConfig::default());
        let program = HirProgram {
            functions: vec![],
            classes: vec![],
            imports: vec![],
        };
        let report = profiler.analyze_program(&program);
        assert!(report.metrics.is_empty());
        assert_eq!(report.total_instructions, 0);
    }

    #[test]
    fn test_simple_function_profiling() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "simple",
            vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Literal(Literal::Int(42)),
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
            ],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert_eq!(report.metrics.len(), 1);
        assert!(report.total_instructions > 0);
    }

    #[test]
    fn test_loop_detection_increases_cost() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "with_loop",
            vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        let metrics = report.metrics.get("with_loop").unwrap();
        assert!(metrics.instruction_count > 10); // Loop body executed multiple times
    }

    #[test]
    fn test_while_loop_analysis() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "with_while",
            vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(report.metrics.get("with_while").is_some());
    }

    #[test]
    fn test_if_else_analysis() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "with_if",
            vec![HirStmt::If {
                condition: HirExpr::Literal(Literal::Bool(true)),
                then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
                else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(0))))]),
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(report.metrics.get("with_if").is_some());
    }

    #[test]
    fn test_hot_path_detection() {
        let mut profiler = Profiler::new(ProfileConfig {
            detect_hot_paths: true,
            ..Default::default()
        });

        // Create a function that will be "hot" (high percentage of time)
        let func = create_test_function(
            "hot_function",
            vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(1000))],
                    kwargs: vec![],
                },
                body: vec![HirStmt::For {
                    target: AssignTarget::Symbol("j".to_string()),
                    iter: HirExpr::Call {
                        func: "range".to_string(),
                        args: vec![HirExpr::Literal(Literal::Int(1000))],
                        kwargs: vec![],
                    },
                    body: vec![HirStmt::Expr(HirExpr::Binary {
                        op: BinOp::Add,
                        left: Box::new(HirExpr::Var("i".to_string())),
                        right: Box::new(HirExpr::Var("j".to_string())),
                    })],
                }],
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(!report.hot_paths.is_empty());
    }

    #[test]
    fn test_hot_path_disabled() {
        let mut profiler = Profiler::new(ProfileConfig {
            detect_hot_paths: false,
            ..Default::default()
        });

        let func = create_test_function("test", vec![]);
        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(report.hot_paths.is_empty());
    }

    #[test]
    fn test_performance_predictions() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        // Function with type check
        let func = create_test_function(
            "with_type_check",
            vec![HirStmt::If {
                condition: HirExpr::Call {
                    func: "isinstance".to_string(),
                    args: vec![
                        HirExpr::Var("x".to_string()),
                        HirExpr::Var("int".to_string()),
                    ],
                    kwargs: vec![],
                },
                then_body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
                else_body: None,
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(!report.predictions.is_empty());

        // Should have type system optimization prediction
        assert!(report
            .predictions
            .iter()
            .any(|p| p.category == PredictionCategory::TypeSystemOptimization));
    }

    #[test]
    fn test_iterator_optimization_prediction() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "with_for",
            vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![HirStmt::Expr(HirExpr::Var("i".to_string()))],
            }],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(report
            .predictions
            .iter()
            .any(|p| p.category == PredictionCategory::IteratorOptimization));
    }

    #[test]
    fn test_memory_layout_prediction() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function("empty", vec![]);
        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert!(report
            .predictions
            .iter()
            .any(|p| p.category == PredictionCategory::MemoryLayoutOptimization));
    }

    // === ProfilingReport tests ===

    #[test]
    fn test_report_formatting() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func = create_test_function(
            "test",
            vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
        );

        let program = HirProgram {
            functions: vec![func],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        let formatted = report.format_report();

        assert!(formatted.contains("Profiling Report"));
        assert!(formatted.contains("Summary"));
        assert!(formatted.contains("Function Metrics"));
    }

    #[test]
    fn test_report_with_hot_paths() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![HotPath {
                call_chain: vec!["main".to_string(), "process".to_string()],
                time_percentage: 50.0,
                loop_depth: 1,
                has_io: false,
            }],
            predictions: vec![],
            total_instructions: 100,
            total_allocations: 10,
            annotations: vec![],
        };

        let formatted = report.format_report();
        assert!(formatted.contains("Hot Paths"));
        assert!(formatted.contains("main"));
    }

    #[test]
    fn test_report_with_predictions() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![PerformancePrediction {
                category: PredictionCategory::TypeSystemOptimization,
                confidence: 0.9,
                speedup_factor: 2.0,
                explanation: "Type checks removed".to_string(),
                functions: vec![],
            }],
            total_instructions: 100,
            total_allocations: 10,
            annotations: vec![],
        };

        let formatted = report.format_report();
        assert!(formatted.contains("Performance Predictions"));
        assert!(formatted.contains("Type checks removed"));
    }

    #[test]
    fn test_report_clone() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 50,
            total_allocations: 5,
            annotations: vec![],
        };
        let cloned = report.clone();
        assert_eq!(report.total_instructions, cloned.total_instructions);
    }

    #[test]
    fn test_report_debug() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 0,
            total_allocations: 0,
            annotations: vec![],
        };
        let debug = format!("{:?}", report);
        assert!(debug.contains("ProfilingReport"));
    }

    #[test]
    fn test_generate_flamegraph_data() {
        let mut metrics = HashMap::new();
        metrics.insert(
            "func1".to_string(),
            FunctionMetrics {
                name: "func1".to_string(),
                instruction_count: 100,
                allocation_count: 5,
                estimated_time: 100.0,
                call_count: 10,
                time_percentage: 50.0,
                is_hot: true,
            },
        );

        let report = ProfilingReport {
            metrics,
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 200,
            total_allocations: 10,
            annotations: vec![],
        };

        let flamegraph = report.generate_flamegraph_data();
        assert!(flamegraph.contains("func1"));
    }

    #[test]
    fn test_generate_perf_annotations() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 0,
            total_allocations: 0,
            annotations: vec![
                ProfilingAnnotation {
                    kind: AnnotationKind::TimingProbe,
                    target: "func1".to_string(),
                    value: "probe".to_string(),
                },
                ProfilingAnnotation {
                    kind: AnnotationKind::AllocationCounter,
                    target: "func2".to_string(),
                    value: "100".to_string(),
                },
            ],
        };

        let perf = report.generate_perf_annotations();
        assert!(perf.contains("@probe func1"));
        assert!(perf.contains("allocation counter"));
    }

    #[test]
    fn test_format_annotation_hot_path() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 0,
            total_allocations: 0,
            annotations: vec![],
        };

        let annotation = ProfilingAnnotation {
            kind: AnnotationKind::HotPathMarker,
            target: "hot_func".to_string(),
            value: "".to_string(),
        };

        let formatted = report.format_annotation(&annotation);
        assert!(formatted.contains("@hot"));
        assert!(formatted.contains("hot_func"));
    }

    #[test]
    fn test_format_annotation_performance_hint() {
        let report = ProfilingReport {
            metrics: HashMap::new(),
            hot_paths: vec![],
            predictions: vec![],
            total_instructions: 0,
            total_allocations: 0,
            annotations: vec![],
        };

        let annotation = ProfilingAnnotation {
            kind: AnnotationKind::PerformanceHint,
            target: "func".to_string(),
            value: "Consider caching".to_string(),
        };

        let formatted = report.format_annotation(&annotation);
        assert!(formatted.contains("@hint"));
        assert!(formatted.contains("Consider caching"));
    }

    // === Expression analysis tests ===

    #[test]
    fn test_analyze_literal() {
        let (inst, alloc) = analyze_expr_inner(&HirExpr::Literal(Literal::Int(42)));
        assert_eq!(inst, 1);
        assert_eq!(alloc, 0);
    }

    #[test]
    fn test_analyze_var() {
        let (inst, alloc) = analyze_expr_inner(&HirExpr::Var("x".to_string()));
        assert_eq!(inst, 1);
        assert_eq!(alloc, 0);
    }

    #[test]
    fn test_analyze_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let (inst, alloc) = analyze_expr_inner(&expr);
        assert_eq!(inst, 3); // 1 + 1 + 1
        assert_eq!(alloc, 0);
    }

    #[test]
    fn test_analyze_call() {
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        let (inst, alloc) = analyze_expr_inner(&expr);
        assert!(inst > 10); // Base cost + arg
        assert_eq!(alloc, 0);
    }

    #[test]
    fn test_analyze_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let (inst, alloc) = analyze_expr_inner(&expr);
        assert!(inst >= 3); // Base + 2 items
        assert_eq!(alloc, 1); // List allocation
    }

    #[test]
    fn test_analyze_dict() {
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]);
        let (inst, alloc) = analyze_expr_inner(&expr);
        assert!(inst >= 4); // Base + key + value + overhead
        assert_eq!(alloc, 1); // Dict allocation
    }

    // === Multiple functions test ===

    #[test]
    fn test_multiple_functions() {
        let mut profiler = Profiler::new(ProfileConfig::default());

        let func1 = create_test_function("func1", vec![HirStmt::Return(None)]);
        let func2 = create_test_function(
            "func2",
            vec![HirStmt::Expr(HirExpr::Literal(Literal::Int(1)))],
        );

        let program = HirProgram {
            functions: vec![func1, func2],
            classes: vec![],
            imports: vec![],
        };

        let report = profiler.analyze_program(&program);
        assert_eq!(report.metrics.len(), 2);
        assert!(report.metrics.contains_key("func1"));
        assert!(report.metrics.contains_key("func2"));
    }

    // === Type check detection tests ===

    #[test]
    fn test_type_check_isinstance() {
        let profiler = Profiler::new(ProfileConfig::default());
        let expr = HirExpr::Call {
            func: "isinstance".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(profiler.is_type_check_expr(&expr));
    }

    #[test]
    fn test_type_check_type() {
        let profiler = Profiler::new(ProfileConfig::default());
        let expr = HirExpr::Call {
            func: "type".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(profiler.is_type_check_expr(&expr));
    }

    #[test]
    fn test_not_type_check() {
        let profiler = Profiler::new(ProfileConfig::default());
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!profiler.is_type_check_expr(&expr));
    }

    #[test]
    fn test_not_type_check_non_call() {
        let profiler = Profiler::new(ProfileConfig::default());
        let expr = HirExpr::Var("isinstance".to_string());
        assert!(!profiler.is_type_check_expr(&expr));
    }

    // === Annotations generation tests ===

    #[test]
    fn test_generate_annotations_for_hot_function() {
        let mut profiler = Profiler::new(ProfileConfig::default());
        profiler.metrics.insert(
            "hot".to_string(),
            FunctionMetrics {
                name: "hot".to_string(),
                instruction_count: 1000,
                allocation_count: 5,
                estimated_time: 1000.0,
                call_count: 100,
                time_percentage: 80.0,
                is_hot: true,
            },
        );

        let annotations = profiler.generate_annotations();
        assert!(annotations.iter().any(|a| matches!(a.kind, AnnotationKind::TimingProbe)));
    }

    #[test]
    fn test_generate_annotations_for_high_allocation() {
        let mut profiler = Profiler::new(ProfileConfig::default());
        profiler.metrics.insert(
            "alloc_heavy".to_string(),
            FunctionMetrics {
                name: "alloc_heavy".to_string(),
                instruction_count: 100,
                allocation_count: 50,
                estimated_time: 100.0,
                call_count: 10,
                time_percentage: 10.0,
                is_hot: false,
            },
        );

        let annotations = profiler.generate_annotations();
        assert!(annotations
            .iter()
            .any(|a| matches!(a.kind, AnnotationKind::AllocationCounter)));
    }
}
