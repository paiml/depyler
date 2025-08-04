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
            HirStmt::Assign { value, .. } => {
                let (inst, alloc) = self.analyze_expr(value);
                (inst + 1, alloc, 1.0)
            }
            HirStmt::Expr(expr) => {
                let (inst, alloc) = self.analyze_expr(expr);
                (inst, alloc, 1.0)
            }
            HirStmt::Return(Some(expr)) => {
                let (inst, alloc) = self.analyze_expr(expr);
                (inst + 1, alloc, 1.0)
            }
            HirStmt::Return(None) => (1, 0, 1.0),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
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
                        total_inst += inst / 2; // Assume 50% probability
                        total_alloc += alloc / 2;
                    }
                }

                (total_inst, total_alloc, 1.0)
            }
            HirStmt::While { condition, body } => {
                let (cond_inst, cond_alloc) = self.analyze_expr(condition);
                let mut body_inst = 0;
                let mut body_alloc = 0;

                for stmt in body {
                    let (inst, alloc, _) = self.analyze_stmt(stmt, loop_depth + 1);
                    body_inst += inst;
                    body_alloc += alloc;
                }

                // Assume loops run 10 times on average
                let loop_factor = 10.0_f64.powi(loop_depth as i32);
                (
                    cond_inst + (body_inst * 10),
                    cond_alloc + (body_alloc * 10),
                    loop_factor,
                )
            }
            HirStmt::For { iter, body, .. } => {
                let (iter_inst, iter_alloc) = self.analyze_expr(iter);
                let mut body_inst = 0;
                let mut body_alloc = 0;

                for stmt in body {
                    let (inst, alloc, _) = self.analyze_stmt(stmt, loop_depth + 1);
                    body_inst += inst;
                    body_alloc += alloc;
                }

                // Assume loops run 10 times on average
                let loop_factor = 10.0_f64.powi(loop_depth as i32);
                (
                    iter_inst + (body_inst * 10),
                    iter_alloc + (body_alloc * 10),
                    loop_factor,
                )
            }
            _ => (1, 0, 1.0),
        }
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

        output.push_str(&format!("\n{}\n", "Profiling Report".bold().blue()));
        output.push_str(&format!("{}\n\n", "═".repeat(50)));

        // Summary
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

        // Hot functions
        if !self.hot_paths.is_empty() {
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

        // Function metrics
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

        // Performance predictions
        if !self.predictions.is_empty() {
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

        // Overall prediction
        let total_speedup: f64 = self.predictions.iter().map(|p| p.speedup_factor).product();

        if total_speedup > 1.0 {
            output.push_str(&format!(
                "{} Estimated overall speedup: {}x\n",
                "🚀".green(),
                format!("{:.1}", total_speedup).bold().green()
            ));
        }

        output
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
        let mut annotations = Vec::new();

        for annotation in &self.annotations {
            match annotation.kind {
                AnnotationKind::TimingProbe => {
                    annotations.push(format!("# @probe {}: timing probe", annotation.target));
                }
                AnnotationKind::AllocationCounter => {
                    annotations.push(format!(
                        "# @probe {}: allocation counter = {}",
                        annotation.target, annotation.value
                    ));
                }
                AnnotationKind::HotPathMarker => {
                    annotations.push(format!("# @hot {}: hot path marker", annotation.target));
                }
                AnnotationKind::PerformanceHint => {
                    annotations.push(format!(
                        "# @hint {}: {}",
                        annotation.target, annotation.value
                    ));
                }
            }
        }

        annotations.join("\n")
    }
}

fn analyze_expr_inner(expr: &HirExpr) -> (usize, usize) {
    match expr {
        HirExpr::Literal(_) => (1, 0),
        HirExpr::Var(_) => (1, 0),
        HirExpr::Binary { left, right, .. } => {
            let (l_inst, l_alloc) = analyze_expr_inner(left);
            let (r_inst, r_alloc) = analyze_expr_inner(right);
            (l_inst + r_inst + 1, l_alloc + r_alloc)
        }
        HirExpr::Call { args, .. } => {
            let mut total_inst = 10; // Base cost for function call
            let mut total_alloc = 0;

            for arg in args {
                let (inst, alloc) = analyze_expr_inner(arg);
                total_inst += inst;
                total_alloc += alloc;
            }

            (total_inst, total_alloc)
        }
        HirExpr::List(items) => {
            let mut total_inst = 1;
            let total_alloc = 1; // List allocation

            for item in items {
                let (inst, _) = analyze_expr_inner(item);
                total_inst += inst;
            }

            (total_inst, total_alloc)
        }
        HirExpr::Dict(pairs) => {
            let mut total_inst = 1;
            let total_alloc = 1; // Dict allocation

            for (k, v) in pairs {
                let (k_inst, _) = analyze_expr_inner(k);
                let (v_inst, _) = analyze_expr_inner(v);
                total_inst += k_inst + v_inst + 2; // Insert cost
            }

            (total_inst, total_alloc)
        }
        _ => (1, 0),
    }
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

    #[test]
    fn test_profiler_creation() {
        let config = ProfileConfig::default();
        let profiler = Profiler::new(config);
        assert!(profiler.metrics.is_empty());
        assert!(profiler.hot_paths.is_empty());
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
                target: "i".to_string(),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(10))],
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
    fn test_hot_path_detection() {
        let mut profiler = Profiler::new(ProfileConfig {
            detect_hot_paths: true,
            ..Default::default()
        });

        // Create a function that will be "hot" (high percentage of time)
        let func = create_test_function(
            "hot_function",
            vec![HirStmt::For {
                target: "i".to_string(),
                iter: HirExpr::Call {
                    func: "range".to_string(),
                    args: vec![HirExpr::Literal(Literal::Int(1000))],
                },
                body: vec![HirStmt::For {
                    target: "j".to_string(),
                    iter: HirExpr::Call {
                        func: "range".to_string(),
                        args: vec![HirExpr::Literal(Literal::Int(1000))],
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
}
