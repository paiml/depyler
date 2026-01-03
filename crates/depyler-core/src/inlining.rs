/// Function inlining heuristics and implementation for the optimizer
use crate::hir::{HirExpr, HirFunction, HirProgram, HirStmt};
use std::collections::{HashMap, HashSet};

/// Inlining analyzer that determines which functions should be inlined
pub struct InliningAnalyzer {
    /// Configuration for inlining decisions
    config: InliningConfig,
    /// Function call graph for dependency analysis
    call_graph: CallGraph,
    /// Metrics for each function
    function_metrics: HashMap<String, FunctionMetrics>,
}

#[derive(Debug, Clone)]
pub struct InliningConfig {
    /// Maximum size (in HIR nodes) for a function to be inlined
    pub max_inline_size: usize,
    /// Maximum depth of inlining (to prevent infinite recursion)
    pub max_inline_depth: usize,
    /// Whether to inline functions called only once
    pub inline_single_use: bool,
    /// Whether to inline trivial functions (single expression)
    pub inline_trivial: bool,
    /// Cost threshold for inlining decision
    pub cost_threshold: f64,
    /// Whether to inline functions with loops
    pub inline_loops: bool,
}

impl Default for InliningConfig {
    fn default() -> Self {
        Self {
            max_inline_size: 20,
            max_inline_depth: 3,
            inline_single_use: true,
            inline_trivial: true,
            cost_threshold: 1.5,
            inline_loops: false,
        }
    }
}

#[derive(Debug, Default)]
struct CallGraph {
    /// Map from function name to functions it calls
    calls: HashMap<String, HashSet<String>>,
    /// Map from function name to functions that call it
    called_by: HashMap<String, HashSet<String>>,
    /// Recursive functions
    recursive: HashSet<String>,
}

#[derive(Debug, Clone)]
struct FunctionMetrics {
    /// Number of HIR nodes in the function
    size: usize,
    /// Number of parameters
    _param_count: usize,
    /// Number of return statements
    _return_count: usize,
    /// Contains loops
    has_loops: bool,
    /// Contains I/O or other side effects
    has_side_effects: bool,
    /// Is a trivial function (single return expression)
    is_trivial: bool,
    /// Number of times this function is called
    call_count: usize,
    /// Estimated execution cost
    cost: f64,
}

/// Inlining decision for a specific call site
#[derive(Debug, Clone)]
pub struct InliningDecision {
    pub should_inline: bool,
    pub reason: InliningReason,
    pub cost_benefit: f64,
}

#[derive(Debug, Clone)]
pub enum InliningReason {
    /// Function is trivial (single expression)
    Trivial,
    /// Function is called only once
    SingleUse,
    /// Function is small and frequently called
    SmallHotFunction,
    /// Inlining would enable further optimizations
    EnablesOptimization,
    /// Not inlined due to size
    TooLarge,
    /// Not inlined due to recursion
    Recursive,
    /// Not inlined due to side effects
    HasSideEffects,
    /// Not inlined due to loops
    ContainsLoops,
    /// Not inlined due to cost
    CostTooHigh,
}

impl InliningAnalyzer {
    pub fn new(config: InliningConfig) -> Self {
        Self {
            config,
            call_graph: CallGraph::default(),
            function_metrics: HashMap::new(),
        }
    }

    /// Analyze a program and determine which functions should be inlined
    pub fn analyze_program(&mut self, program: &HirProgram) -> HashMap<String, InliningDecision> {
        // Step 1: Build call graph
        self.build_call_graph(program);

        // Step 2: Detect recursive functions
        self.detect_recursion();

        // Step 3: Calculate function metrics
        self.calculate_metrics(program);

        // Step 4: Make inlining decisions
        self.make_decisions()
    }

    /// Apply inlining decisions to transform the program
    pub fn apply_inlining(
        &self,
        mut program: HirProgram,
        decisions: &HashMap<String, InliningDecision>,
    ) -> HirProgram {
        // Create a map of functions for quick lookup
        let function_map: HashMap<String, HirFunction> = program
            .functions
            .iter()
            .map(|f| (f.name.clone(), f.clone()))
            .collect();

        // Track inlined functions to remove later
        let mut inlined_functions = HashSet::new();

        // Process each function
        for func_idx in 0..program.functions.len() {
            let func = &mut program.functions[func_idx];
            let mut modified_body = Vec::new();

            for stmt in &func.body {
                match self.try_inline_stmt(stmt, &function_map, decisions, 0) {
                    Some(inlined_stmts) => {
                        modified_body.extend(inlined_stmts);
                        // Track which functions were inlined
                        if let HirStmt::Expr(HirExpr::Call { func: callee, .. }) = stmt {
                            if decisions
                                .get(callee)
                                .map(|d| d.should_inline)
                                .unwrap_or(false)
                            {
                                inlined_functions.insert(callee.clone());
                            }
                        }
                    }
                    None => modified_body.push(stmt.clone()),
                }
            }

            func.body = modified_body;
        }

        // Remove functions that were fully inlined (called only once)
        if self.config.inline_single_use {
            program.functions.retain(|f| {
                !inlined_functions.contains(&f.name)
                    || self
                        .function_metrics
                        .get(&f.name)
                        .map(|m| m.call_count > 1)
                        .unwrap_or(true)
            });
        }

        program
    }

    fn build_call_graph(&mut self, program: &HirProgram) {
        for func in &program.functions {
            let calls = self.extract_calls_from_function(func);

            self.call_graph
                .calls
                .insert(func.name.clone(), calls.clone());

            for callee in calls {
                self.call_graph
                    .called_by
                    .entry(callee)
                    .or_default()
                    .insert(func.name.clone());
            }
        }
    }

    fn extract_calls_from_function(&self, func: &HirFunction) -> HashSet<String> {
        let mut calls = HashSet::new();

        for stmt in &func.body {
            self.extract_calls_from_stmt(stmt, &mut calls);
        }

        calls
    }

    fn extract_calls_from_stmt(&self, stmt: &HirStmt, calls: &mut HashSet<String>) {
        match stmt {
            HirStmt::Expr(expr) => self.extract_calls_from_expr(expr, calls),
            HirStmt::Assign { value, .. } => self.extract_calls_from_expr(value, calls),
            HirStmt::Return(Some(expr)) => self.extract_calls_from_expr(expr, calls),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.extract_calls_from_if(condition, then_body, else_body, calls),
            HirStmt::While { condition, body }
            | HirStmt::For {
                iter: condition,
                body,
                ..
            } => self.extract_calls_from_loop(condition, body, calls),
            _ => {}
        }
    }

    fn extract_calls_from_if(
        &self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
        calls: &mut HashSet<String>,
    ) {
        self.extract_calls_from_expr(condition, calls);
        self.extract_calls_from_body(then_body, calls);
        if let Some(else_stmts) = else_body {
            self.extract_calls_from_body(else_stmts, calls);
        }
    }

    fn extract_calls_from_loop(
        &self,
        condition: &HirExpr,
        body: &[HirStmt],
        calls: &mut HashSet<String>,
    ) {
        self.extract_calls_from_expr(condition, calls);
        self.extract_calls_from_body(body, calls);
    }

    fn extract_calls_from_body(&self, body: &[HirStmt], calls: &mut HashSet<String>) {
        for s in body {
            self.extract_calls_from_stmt(s, calls);
        }
    }

    fn extract_calls_from_expr(&self, expr: &HirExpr, calls: &mut HashSet<String>) {
        extract_calls_from_expr_inner(expr, calls);
    }
}

fn extract_calls_from_expr_inner(expr: &HirExpr, calls: &mut HashSet<String>) {
    match expr {
        HirExpr::Call { func, args, .. } => extract_from_call(func, args, calls),
        HirExpr::Binary { left, right, .. } => extract_from_binary(left, right, calls),
        HirExpr::Unary { operand, .. } => extract_calls_from_expr_inner(operand, calls),
        HirExpr::List(items) | HirExpr::Tuple(items) => extract_from_items(items, calls),
        HirExpr::Dict(pairs) => extract_from_dict(pairs, calls),
        HirExpr::MethodCall { object, args, .. } => extract_from_method_call(object, args, calls),
        HirExpr::Lambda { body, .. } => extract_calls_from_expr_inner(body, calls),
        _ => {}
    }
}

fn extract_from_call(func: &str, args: &[HirExpr], calls: &mut HashSet<String>) {
    calls.insert(func.to_string());
    for arg in args {
        extract_calls_from_expr_inner(arg, calls);
    }
}

fn extract_from_binary(left: &HirExpr, right: &HirExpr, calls: &mut HashSet<String>) {
    extract_calls_from_expr_inner(left, calls);
    extract_calls_from_expr_inner(right, calls);
}

fn extract_from_items(items: &[HirExpr], calls: &mut HashSet<String>) {
    for item in items {
        extract_calls_from_expr_inner(item, calls);
    }
}

fn extract_from_dict(pairs: &[(HirExpr, HirExpr)], calls: &mut HashSet<String>) {
    for (k, v) in pairs {
        extract_calls_from_expr_inner(k, calls);
        extract_calls_from_expr_inner(v, calls);
    }
}

fn extract_from_method_call(object: &HirExpr, args: &[HirExpr], calls: &mut HashSet<String>) {
    extract_calls_from_expr_inner(object, calls);
    for arg in args {
        extract_calls_from_expr_inner(arg, calls);
    }
}

impl InliningAnalyzer {
    fn detect_recursion(&mut self) {
        // Use DFS to detect cycles in the call graph
        for func_name in self.call_graph.calls.keys() {
            let mut visited = HashSet::new();
            let mut stack = HashSet::new();

            if self.is_recursive_dfs(func_name, &mut visited, &mut stack) {
                self.call_graph.recursive.insert(func_name.clone());
            }
        }
    }

    fn is_recursive_dfs(
        &self,
        func: &str,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(func.to_string());
        stack.insert(func.to_string());

        if let Some(callees) = self.call_graph.calls.get(func) {
            for callee in callees {
                if stack.contains(callee) {
                    return true; // Found cycle
                }

                if !visited.contains(callee) && self.is_recursive_dfs(callee, visited, stack) {
                    return true;
                }
            }
        }

        stack.remove(func);
        false
    }

    fn calculate_metrics(&mut self, program: &HirProgram) {
        for func in &program.functions {
            let size = self.calculate_function_size(func);
            let has_loops = self.contains_loops(&func.body);
            let has_side_effects = self.has_side_effects(func);
            let is_trivial = self.is_trivial_function(func);
            let return_count = self.count_returns(&func.body);

            // Calculate call count
            let call_count = self
                .call_graph
                .called_by
                .get(&func.name)
                .map(|callers| callers.len())
                .unwrap_or(0);

            // Estimate execution cost
            let cost = self.estimate_cost(func, size, has_loops, has_side_effects);

            let metrics = FunctionMetrics {
                size,
                _param_count: func.params.len(),
                _return_count: return_count,
                has_loops,
                has_side_effects,
                is_trivial,
                call_count,
                cost,
            };

            self.function_metrics.insert(func.name.clone(), metrics);
        }
    }

    fn calculate_function_size(&self, func: &HirFunction) -> usize {
        let mut size = 0;
        for stmt in &func.body {
            size += self.calculate_stmt_size(stmt);
        }
        size
    }

    fn calculate_stmt_size(&self, stmt: &HirStmt) -> usize {
        match stmt {
            HirStmt::Expr(expr) => self.calculate_expr_size(expr),
            HirStmt::Assign { value, .. } => 1 + self.calculate_expr_size(value),
            HirStmt::Return(Some(expr)) => 1 + self.calculate_expr_size(expr),
            HirStmt::Return(None) => 1,
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.calculate_if_size(condition, then_body, else_body),
            HirStmt::While { condition, body }
            | HirStmt::For {
                iter: condition,
                body,
                ..
            } => self.calculate_loop_size(condition, body),
            _ => 1,
        }
    }

    fn calculate_if_size(
        &self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) -> usize {
        let mut size = 1 + self.calculate_expr_size(condition);
        size += self.calculate_body_size(then_body);
        if let Some(else_stmts) = else_body {
            size += self.calculate_body_size(else_stmts);
        }
        size
    }

    fn calculate_loop_size(&self, condition: &HirExpr, body: &[HirStmt]) -> usize {
        let mut size = 1 + self.calculate_expr_size(condition);
        size += self.calculate_body_size(body);
        size
    }

    fn calculate_body_size(&self, body: &[HirStmt]) -> usize {
        body.iter().map(|s| self.calculate_stmt_size(s)).sum()
    }

    fn calculate_expr_size(&self, expr: &HirExpr) -> usize {
        calculate_expr_size_inner(expr)
    }

    fn contains_loops(&self, body: &[HirStmt]) -> bool {
        contains_loops_inner(body)
    }

    fn has_side_effects(&self, func: &HirFunction) -> bool {
        // Check function properties
        // A function has side effects if it's not pure
        if !func.properties.is_pure {
            return true;
        }

        // Check for I/O operations, mutations, etc.
        for stmt in &func.body {
            if self.stmt_has_side_effects(stmt) {
                return true;
            }
        }

        false
    }

    fn stmt_has_side_effects(&self, stmt: &HirStmt) -> bool {
        match stmt {
            HirStmt::Expr(expr) => self.expr_has_side_effects(expr),
            HirStmt::Assign { value, .. } => self.expr_has_side_effects(value),
            HirStmt::Return(Some(expr)) => self.expr_has_side_effects(expr),
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => self.if_has_side_effects(condition, then_body, else_body),
            HirStmt::While { condition, body }
            | HirStmt::For {
                iter: condition,
                body,
                ..
            } => self.loop_has_side_effects(condition, body),
            HirStmt::Raise { .. } => true,
            _ => false,
        }
    }

    fn if_has_side_effects(
        &self,
        condition: &HirExpr,
        then_body: &[HirStmt],
        else_body: &Option<Vec<HirStmt>>,
    ) -> bool {
        self.expr_has_side_effects(condition)
            || self.body_has_side_effects(then_body)
            || else_body
                .as_ref()
                .map(|stmts| self.body_has_side_effects(stmts))
                .unwrap_or(false)
    }

    fn loop_has_side_effects(&self, condition: &HirExpr, body: &[HirStmt]) -> bool {
        self.expr_has_side_effects(condition) || self.body_has_side_effects(body)
    }

    fn body_has_side_effects(&self, body: &[HirStmt]) -> bool {
        body.iter().any(|s| self.stmt_has_side_effects(s))
    }

    fn expr_has_side_effects(&self, expr: &HirExpr) -> bool {
        expr_has_side_effects_inner(expr)
    }

    fn is_trivial_function(&self, func: &HirFunction) -> bool {
        // A trivial function has a single return statement
        if func.body.len() == 1 {
            matches!(func.body[0], HirStmt::Return(_))
        } else {
            false
        }
    }

    fn count_returns(&self, body: &[HirStmt]) -> usize {
        count_returns_inner(body)
    }

    fn estimate_cost(
        &self,
        func: &HirFunction,
        size: usize,
        has_loops: bool,
        has_side_effects: bool,
    ) -> f64 {
        let mut cost = size as f64;

        // Loops significantly increase cost
        if has_loops {
            cost *= 10.0;
        }

        // Side effects add cost
        if has_side_effects {
            cost *= 2.0;
        }

        // Multiple returns add complexity
        let return_count = count_returns_inner(&func.body);
        if return_count > 1 {
            cost *= 1.0 + (return_count as f64 * 0.2);
        }

        // Parameters add overhead
        cost += func.params.len() as f64 * 0.5;

        cost
    }

    fn make_decisions(&self) -> HashMap<String, InliningDecision> {
        let mut decisions = HashMap::new();

        for (func_name, metrics) in &self.function_metrics {
            let decision = self.decide_inlining(func_name, metrics);
            decisions.insert(func_name.clone(), decision);
        }

        decisions
    }

    fn decide_inlining(&self, func_name: &str, metrics: &FunctionMetrics) -> InliningDecision {
        // Check disqualifying conditions first
        if let Some(rejection) = self.check_inlining_rejections(func_name, metrics) {
            return rejection;
        }

        // Check fast-path approvals
        if let Some(approval) = self.check_inlining_approvals(metrics) {
            return approval;
        }

        // Fall back to cost-benefit analysis
        self.decide_by_cost_benefit(metrics)
    }

    fn check_inlining_rejections(
        &self,
        func_name: &str,
        metrics: &FunctionMetrics,
    ) -> Option<InliningDecision> {
        if self.call_graph.recursive.contains(func_name) {
            return Some(InliningDecision {
                should_inline: false,
                reason: InliningReason::Recursive,
                cost_benefit: 0.0,
            });
        }

        if metrics.size > self.config.max_inline_size {
            return Some(InliningDecision {
                should_inline: false,
                reason: InliningReason::TooLarge,
                cost_benefit: 0.0,
            });
        }

        if metrics.has_loops && !self.config.inline_loops {
            return Some(InliningDecision {
                should_inline: false,
                reason: InliningReason::ContainsLoops,
                cost_benefit: 0.0,
            });
        }

        if metrics.has_side_effects {
            return Some(InliningDecision {
                should_inline: false,
                reason: InliningReason::HasSideEffects,
                cost_benefit: 0.0,
            });
        }

        None
    }

    fn check_inlining_approvals(&self, metrics: &FunctionMetrics) -> Option<InliningDecision> {
        if self.config.inline_trivial && metrics.is_trivial {
            return Some(InliningDecision {
                should_inline: true,
                reason: InliningReason::Trivial,
                cost_benefit: 10.0,
            });
        }

        if self.config.inline_single_use && metrics.call_count == 1 && !metrics.has_side_effects {
            return Some(InliningDecision {
                should_inline: true,
                reason: InliningReason::SingleUse,
                cost_benefit: 5.0,
            });
        }

        None
    }

    fn decide_by_cost_benefit(&self, metrics: &FunctionMetrics) -> InliningDecision {
        let call_overhead = 1.0;
        let benefit = (call_overhead * metrics.call_count as f64) - metrics.cost;
        let cost_benefit = benefit / metrics.cost;

        if cost_benefit >= self.config.cost_threshold {
            InliningDecision {
                should_inline: true,
                reason: InliningReason::SmallHotFunction,
                cost_benefit,
            }
        } else {
            InliningDecision {
                should_inline: false,
                reason: InliningReason::CostTooHigh,
                cost_benefit,
            }
        }
    }

    fn try_inline_stmt(
        &self,
        stmt: &HirStmt,
        function_map: &HashMap<String, HirFunction>,
        decisions: &HashMap<String, InliningDecision>,
        depth: usize,
    ) -> Option<Vec<HirStmt>> {
        // Prevent excessive inlining depth
        if depth >= self.config.max_inline_depth {
            return None;
        }

        match stmt {
            HirStmt::Expr(HirExpr::Call { func, args, .. }) => {
                self.try_inline_expr_call(func, args, function_map, decisions, depth)
            }
            HirStmt::Assign { target, value, .. } => {
                self.try_inline_assign_call(target, value, function_map, decisions, depth)
            }
            _ => None,
        }
    }

    fn try_inline_expr_call(
        &self,
        func: &str,
        args: &[HirExpr],
        function_map: &HashMap<String, HirFunction>,
        decisions: &HashMap<String, InliningDecision>,
        depth: usize,
    ) -> Option<Vec<HirStmt>> {
        let decision = decisions.get(func)?;
        if !decision.should_inline {
            return None;
        }
        let target_func = function_map.get(func)?;
        Some(self.inline_function_call(target_func, args, function_map, decisions, depth))
    }

    fn try_inline_assign_call(
        &self,
        target: &crate::hir::AssignTarget,
        value: &HirExpr,
        function_map: &HashMap<String, HirFunction>,
        decisions: &HashMap<String, InliningDecision>,
        depth: usize,
    ) -> Option<Vec<HirStmt>> {
        if let HirExpr::Call { func, args, .. } = value {
            let decision = decisions.get(func)?;
            if !decision.should_inline {
                return None;
            }
            let target_func = function_map.get(func)?;
            let inlined =
                self.inline_function_call(target_func, args, function_map, decisions, depth);

            if inlined.is_empty() {
                return None;
            }

            let mut result = inlined;
            self.replace_return_with_assign(&mut result, target);
            Some(result)
        } else {
            None
        }
    }

    fn replace_return_with_assign(
        &self,
        statements: &mut [HirStmt],
        target: &crate::hir::AssignTarget,
    ) {
        if let Some(last) = statements.last_mut() {
            if let HirStmt::Return(Some(expr)) = last {
                *last = HirStmt::Assign {
                    target: target.clone(),
                    value: expr.clone(),
                    type_annotation: None,
                };
            }
        }
    }

    fn inline_function_call(
        &self,
        func: &HirFunction,
        args: &[HirExpr],
        function_map: &HashMap<String, HirFunction>,
        decisions: &HashMap<String, InliningDecision>,
        depth: usize,
    ) -> Vec<HirStmt> {
        let mut inlined_body = Vec::new();

        // Create parameter bindings
        for (i, param) in func.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                inlined_body.push(HirStmt::Assign {
                    target: crate::hir::AssignTarget::Symbol(format!("_inline_{}", param.name)),
                    value: arg.clone(),
                    type_annotation: None,
                });
            }
        }

        // Copy and transform the function body
        for stmt in &func.body {
            let transformed = self.transform_stmt_for_inlining(stmt, &func.params, depth + 1);

            // Recursively try to inline nested calls
            if let Some(inlined) =
                self.try_inline_stmt(&transformed, function_map, decisions, depth + 1)
            {
                inlined_body.extend(inlined);
            } else {
                inlined_body.push(transformed);
            }
        }

        inlined_body
    }

    fn transform_stmt_for_inlining(
        &self,
        stmt: &HirStmt,
        params: &[crate::hir::HirParam],
        _depth: usize,
    ) -> HirStmt {
        match stmt {
            HirStmt::Expr(expr) => HirStmt::Expr(transform_expr_for_inlining_inner(expr, params)),
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => HirStmt::Assign {
                target: self.transform_assign_target_for_inlining(target, params),
                value: transform_expr_for_inlining_inner(value, params),
                type_annotation: type_annotation.clone(),
            },
            HirStmt::Return(Some(expr)) => {
                HirStmt::Return(Some(transform_expr_for_inlining_inner(expr, params)))
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => HirStmt::If {
                condition: transform_expr_for_inlining_inner(condition, params),
                then_body: then_body
                    .iter()
                    .map(|s| self.transform_stmt_for_inlining(s, params, _depth))
                    .collect(),
                else_body: else_body.as_ref().map(|stmts| {
                    stmts
                        .iter()
                        .map(|s| self.transform_stmt_for_inlining(s, params, _depth))
                        .collect()
                }),
            },
            _ => stmt.clone(),
        }
    }

    #[allow(dead_code)]
    fn transform_expr_for_inlining(
        &self,
        expr: &HirExpr,
        params: &[crate::hir::HirParam],
    ) -> HirExpr {
        transform_expr_for_inlining_inner(expr, params)
    }

    fn transform_assign_target_for_inlining(
        &self,
        target: &crate::hir::AssignTarget,
        params: &[crate::hir::HirParam],
    ) -> crate::hir::AssignTarget {
        match target {
            crate::hir::AssignTarget::Symbol(name) => {
                if params.iter().any(|p| &p.name == name) {
                    crate::hir::AssignTarget::Symbol(format!("_inline_{}", name))
                } else {
                    target.clone()
                }
            }
            _ => target.clone(),
        }
    }
}

fn calculate_expr_size_inner(expr: &HirExpr) -> usize {
    match expr {
        HirExpr::Literal(_) | HirExpr::Var(_) => 1,
        HirExpr::Binary { left, right, .. } => {
            1 + calculate_expr_size_inner(left) + calculate_expr_size_inner(right)
        }
        HirExpr::Unary { operand, .. } => 1 + calculate_expr_size_inner(operand),
        HirExpr::Call { args, .. } => 1 + args.iter().map(calculate_expr_size_inner).sum::<usize>(),
        HirExpr::List(items) | HirExpr::Tuple(items) => {
            1 + items.iter().map(calculate_expr_size_inner).sum::<usize>()
        }
        HirExpr::Dict(pairs) => {
            1 + pairs
                .iter()
                .map(|(k, v)| calculate_expr_size_inner(k) + calculate_expr_size_inner(v))
                .sum::<usize>()
        }
        _ => 1,
    }
}

fn contains_loops_inner(body: &[HirStmt]) -> bool {
    for stmt in body {
        match stmt {
            HirStmt::While { .. } | HirStmt::For { .. } => return true,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                if contains_loops_inner(then_body) {
                    return true;
                }
                if let Some(else_stmts) = else_body {
                    if contains_loops_inner(else_stmts) {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn expr_has_side_effects_inner(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Call { func, args, .. } => call_has_side_effects(func, args),
        HirExpr::MethodCall { object, method, .. } => {
            method_has_side_effects(method) || expr_has_side_effects_inner(object)
        }
        HirExpr::Binary { left, right, .. } => binary_has_side_effects(left, right),
        HirExpr::Unary { operand, .. } => expr_has_side_effects_inner(operand),
        HirExpr::List(items) | HirExpr::Tuple(items) => collection_has_side_effects(items),
        HirExpr::Dict(pairs) => dict_has_side_effects(pairs),
        _ => false,
    }
}

fn call_has_side_effects(func: &str, args: &[HirExpr]) -> bool {
    let pure_functions = ["len", "abs", "min", "max", "sum", "str", "int", "float"];
    !pure_functions.contains(&func) || args.iter().any(expr_has_side_effects_inner)
}

fn method_has_side_effects(method: &str) -> bool {
    let mutating_methods = [
        "append", "extend", "remove", "pop", "clear", "sort", "reverse", "insert", "update",
        "write",
    ];
    mutating_methods.contains(&method)
}

fn binary_has_side_effects(left: &HirExpr, right: &HirExpr) -> bool {
    expr_has_side_effects_inner(left) || expr_has_side_effects_inner(right)
}

fn collection_has_side_effects(items: &[HirExpr]) -> bool {
    items.iter().any(expr_has_side_effects_inner)
}

fn dict_has_side_effects(pairs: &[(HirExpr, HirExpr)]) -> bool {
    pairs
        .iter()
        .any(|(k, v)| expr_has_side_effects_inner(k) || expr_has_side_effects_inner(v))
}

fn count_returns_inner(body: &[HirStmt]) -> usize {
    let mut count = 0;
    for stmt in body {
        match stmt {
            HirStmt::Return(_) => count += 1,
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                count += count_returns_inner(then_body);
                if let Some(else_stmts) = else_body {
                    count += count_returns_inner(else_stmts);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                count += count_returns_inner(body);
            }
            _ => {}
        }
    }
    count
}

fn transform_expr_for_inlining_inner(expr: &HirExpr, params: &[crate::hir::HirParam]) -> HirExpr {
    match expr {
        HirExpr::Var(name) => {
            // Replace parameter references with inlined versions
            if params.iter().any(|p| &p.name == name) {
                HirExpr::Var(format!("_inline_{}", name))
            } else {
                expr.clone()
            }
        }
        HirExpr::Binary { left, right, op } => HirExpr::Binary {
            left: Box::new(transform_expr_for_inlining_inner(left, params)),
            right: Box::new(transform_expr_for_inlining_inner(right, params)),
            op: *op,
        },
        HirExpr::Unary { operand, op } => HirExpr::Unary {
            operand: Box::new(transform_expr_for_inlining_inner(operand, params)),
            op: *op,
        },
        HirExpr::Call { func, args, .. } => HirExpr::Call {
            func: func.clone(),
            args: args
                .iter()
                .map(|a| transform_expr_for_inlining_inner(a, params))
                .collect(),
            kwargs: vec![],
        },
        _ => expr.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use smallvec::smallvec;

    fn create_simple_function(name: &str, size: usize) -> HirFunction {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(HirStmt::Assign {
                target: AssignTarget::Symbol(Symbol::from(format!("x{}", i).as_str())),
                value: HirExpr::Literal(Literal::Int(i as i64)),
                type_annotation: None,
            });
        }
        body.push(HirStmt::Return(Some(HirExpr::Var("x0".to_string()))));

        HirFunction {
            name: name.to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body,
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_trivial_function_detection() {
        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(analyzer.is_trivial_function(&func));
    }

    #[test]
    fn test_function_size_calculation() {
        let func = create_simple_function("test", 5);
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let size = analyzer.calculate_function_size(&func);
        assert_eq!(size, 12); // 5 assignments (2 each) + 1 return (2) = 12
    }

    #[test]
    fn test_loop_detection() {
        let body = vec![HirStmt::While {
            condition: HirExpr::Literal(Literal::Bool(true)),
            body: vec![HirStmt::Break { label: None }],
        }];

        let _analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(contains_loops_inner(&body));
    }

    #[test]
    fn test_side_effect_detection() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
            kwargs: vec![],
        };

        let _analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_inlining_config_default() {
        let config = InliningConfig::default();
        assert_eq!(config.max_inline_size, 20);
        assert_eq!(config.max_inline_depth, 3);
        assert!(config.inline_single_use);
        assert!(config.inline_trivial);
    }

    #[test]
    fn test_inlining_config_custom() {
        let config = InliningConfig {
            max_inline_size: 50,
            max_inline_depth: 5,
            inline_single_use: false,
            inline_trivial: false,
            cost_threshold: 2.0,
            inline_loops: true,
        };
        assert_eq!(config.max_inline_size, 50);
        assert_eq!(config.max_inline_depth, 5);
        assert!(!config.inline_single_use);
        assert!(!config.inline_trivial);
        assert_eq!(config.cost_threshold, 2.0);
        assert!(config.inline_loops);
    }

    #[test]
    fn test_inlining_reason_variants() {
        let reasons = [
            InliningReason::Trivial,
            InliningReason::SingleUse,
            InliningReason::SmallHotFunction,
            InliningReason::EnablesOptimization,
            InliningReason::TooLarge,
            InliningReason::Recursive,
            InliningReason::HasSideEffects,
            InliningReason::ContainsLoops,
        ];
        for reason in &reasons {
            let _ = format!("{:?}", reason);
        }
    }

    #[test]
    fn test_inlining_decision_creation() {
        let decision = InliningDecision {
            should_inline: true,
            reason: InliningReason::Trivial,
            cost_benefit: 1.5,
        };
        assert!(decision.should_inline);
        assert_eq!(decision.cost_benefit, 1.5);
    }

    #[test]
    fn test_analyzer_new() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(std::mem::size_of_val(&analyzer) > 0);
    }

    #[test]
    fn test_non_trivial_function() {
        let func = HirFunction {
            name: "complex".to_string(),
            params: smallvec![HirParam::new("x".to_string(), Type::Int)],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("y".to_string()),
                    value: HirExpr::Var("x".to_string()),
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Var("y".to_string()))),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(!analyzer.is_trivial_function(&func));
    }

    #[test]
    fn test_for_loop_detection() {
        let body = vec![HirStmt::For {
            target: AssignTarget::Symbol("i".to_string()),
            iter: HirExpr::Call {
                func: "range".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(10))],
                kwargs: vec![],
            },
            body: vec![],
        }];
        assert!(contains_loops_inner(&body));
    }

    #[test]
    fn test_if_no_loop() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(1))))],
            else_body: None,
        }];
        assert!(!contains_loops_inner(&body));
    }

    #[test]
    fn test_print_side_effect() {
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_append_side_effect() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_pure_function_no_side_effect() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_function_size_empty() {
        let func = HirFunction {
            name: "empty".to_string(),
            params: smallvec![],
            ret_type: Type::None,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert_eq!(analyzer.calculate_function_size(&func), 0);
    }

    #[test]
    fn test_call_graph_default() {
        let cg = CallGraph::default();
        assert!(cg.calls.is_empty());
        assert!(cg.called_by.is_empty());
        assert!(cg.recursive.is_empty());
    }

    #[test]
    fn test_expr_size_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert_eq!(calculate_expr_size_inner(&expr), 1);
    }

    #[test]
    fn test_expr_size_var() {
        let expr = HirExpr::Var("x".to_string());
        assert_eq!(calculate_expr_size_inner(&expr), 1);
    }

    #[test]
    fn test_expr_size_binary() {
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert_eq!(calculate_expr_size_inner(&expr), 3);
    }

    #[test]
    fn test_expr_size_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert_eq!(calculate_expr_size_inner(&expr), 3);
    }

    #[test]
    fn test_stmt_size_return_none() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Return(None);
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 1);
    }

    #[test]
    fn test_stmt_size_return_some() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))));
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 2);
    }

    #[test]
    fn test_stmt_size_expr() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Expr(HirExpr::Literal(Literal::Int(42)));
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 1);
    }

    #[test]
    fn test_stmt_size_pass() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Pass;
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 1);
    }

    #[test]
    fn test_stmt_size_break() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Break { label: None };
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 1);
    }

    #[test]
    fn test_stmt_size_continue() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let stmt = HirStmt::Continue { label: None };
        assert_eq!(analyzer.calculate_stmt_size(&stmt), 1);
    }

    // ============================================================================
    // ADDITIONAL COVERAGE TESTS - Call extraction and analysis
    // ============================================================================

    #[test]
    fn test_extract_calls_from_call_expr() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("foo"));
    }

    #[test]
    fn test_extract_calls_from_binary_expr() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Call {
                func: "left_fn".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Call {
                func: "right_fn".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("left_fn"));
        assert!(calls.contains("right_fn"));
    }

    #[test]
    fn test_extract_calls_from_unary_expr() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Call {
                func: "inner".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("inner"));
    }

    #[test]
    fn test_extract_calls_from_list() {
        let mut calls = HashSet::new();
        let expr = HirExpr::List(vec![
            HirExpr::Call {
                func: "fn1".to_string(),
                args: vec![],
                kwargs: vec![],
            },
            HirExpr::Call {
                func: "fn2".to_string(),
                args: vec![],
                kwargs: vec![],
            },
        ]);
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("fn1"));
        assert!(calls.contains("fn2"));
    }

    #[test]
    fn test_extract_calls_from_tuple() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Tuple(vec![HirExpr::Call {
            func: "tuple_fn".to_string(),
            args: vec![],
            kwargs: vec![],
        }]);
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("tuple_fn"));
    }

    #[test]
    fn test_extract_calls_from_dict() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Dict(vec![(
            HirExpr::Call {
                func: "key_fn".to_string(),
                args: vec![],
                kwargs: vec![],
            },
            HirExpr::Call {
                func: "val_fn".to_string(),
                args: vec![],
                kwargs: vec![],
            },
        )]);
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("key_fn"));
        assert!(calls.contains("val_fn"));
    }

    #[test]
    fn test_extract_calls_from_method_call() {
        let mut calls = HashSet::new();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Call {
                func: "get_obj".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "method".to_string(),
            args: vec![HirExpr::Call {
                func: "get_arg".to_string(),
                args: vec![],
                kwargs: vec![],
            }],
            kwargs: vec![],
        };
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("get_obj"));
        assert!(calls.contains("get_arg"));
    }

    #[test]
    fn test_extract_calls_from_lambda() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Call {
                func: "lambda_call".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
        };
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.contains("lambda_call"));
    }

    #[test]
    fn test_extract_calls_from_var() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Var("x".to_string());
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.is_empty());
    }

    #[test]
    fn test_extract_calls_from_literal() {
        let mut calls = HashSet::new();
        let expr = HirExpr::Literal(Literal::Int(42));
        extract_calls_from_expr_inner(&expr, &mut calls);
        assert!(calls.is_empty());
    }

    #[test]
    fn test_analyzer_extract_calls_from_function() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let func = HirFunction {
            name: "caller".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![
                HirStmt::Expr(HirExpr::Call {
                    func: "callee1".to_string(),
                    args: vec![],
                    kwargs: vec![],
                }),
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: HirExpr::Call {
                        func: "callee2".to_string(),
                        args: vec![],
                        kwargs: vec![],
                    },
                    type_annotation: None,
                },
                HirStmt::Return(Some(HirExpr::Call {
                    func: "callee3".to_string(),
                    args: vec![],
                    kwargs: vec![],
                })),
            ],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let calls = analyzer.extract_calls_from_function(&func);
        assert!(calls.contains("callee1"));
        assert!(calls.contains("callee2"));
        assert!(calls.contains("callee3"));
    }

    #[test]
    fn test_analyzer_extract_calls_from_if() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let func = HirFunction {
            name: "if_caller".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::If {
                condition: HirExpr::Call {
                    func: "cond_fn".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                then_body: vec![HirStmt::Expr(HirExpr::Call {
                    func: "then_fn".to_string(),
                    args: vec![],
                    kwargs: vec![],
                })],
                else_body: Some(vec![HirStmt::Expr(HirExpr::Call {
                    func: "else_fn".to_string(),
                    args: vec![],
                    kwargs: vec![],
                })]),
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let calls = analyzer.extract_calls_from_function(&func);
        assert!(calls.contains("cond_fn"));
        assert!(calls.contains("then_fn"));
        assert!(calls.contains("else_fn"));
    }

    #[test]
    fn test_analyzer_extract_calls_from_while() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let func = HirFunction {
            name: "while_caller".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::While {
                condition: HirExpr::Call {
                    func: "while_cond".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Expr(HirExpr::Call {
                    func: "while_body".to_string(),
                    args: vec![],
                    kwargs: vec![],
                })],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let calls = analyzer.extract_calls_from_function(&func);
        assert!(calls.contains("while_cond"));
        assert!(calls.contains("while_body"));
    }

    #[test]
    fn test_analyzer_extract_calls_from_for() {
        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        let func = HirFunction {
            name: "for_caller".to_string(),
            params: smallvec![],
            ret_type: Type::Int,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Call {
                    func: "for_iter".to_string(),
                    args: vec![],
                    kwargs: vec![],
                },
                body: vec![HirStmt::Expr(HirExpr::Call {
                    func: "for_body".to_string(),
                    args: vec![],
                    kwargs: vec![],
                })],
            }],
            properties: FunctionProperties::default(),
            annotations: Default::default(),
            docstring: None,
        };
        let calls = analyzer.extract_calls_from_function(&func);
        assert!(calls.contains("for_iter"));
        assert!(calls.contains("for_body"));
    }

    #[test]
    fn test_function_metrics_default() {
        let metrics = FunctionMetrics {
            size: 10,
            _param_count: 2,
            _return_count: 1,
            has_loops: false,
            has_side_effects: false,
            is_trivial: true,
            call_count: 3,
            cost: 1.5,
        };
        assert_eq!(metrics.size, 10);
        assert!(!metrics.has_loops);
        assert!(!metrics.has_side_effects);
        assert!(metrics.is_trivial);
        assert_eq!(metrics.call_count, 3);
    }

    #[test]
    fn test_expr_size_call() {
        let expr = HirExpr::Call {
            func: "fn".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ],
            kwargs: vec![],
        };
        let size = calculate_expr_size_inner(&expr);
        assert_eq!(size, 3); // 1 for call + 1 for each arg (2 args)
    }

    #[test]
    fn test_expr_size_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        let size = calculate_expr_size_inner(&expr);
        assert_eq!(size, 1); // MethodCall falls into catchall case
    }

    #[test]
    fn test_expr_size_dict() {
        let expr = HirExpr::Dict(vec![
            (
                HirExpr::Literal(Literal::String("a".to_string())),
                HirExpr::Literal(Literal::Int(1)),
            ),
            (
                HirExpr::Literal(Literal::String("b".to_string())),
                HirExpr::Literal(Literal::Int(2)),
            ),
        ]);
        let size = calculate_expr_size_inner(&expr);
        assert!(size >= 5); // 2 pairs * 2 elements + 1 base
    }

    #[test]
    fn test_expr_size_lambda() {
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Var("x".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
        };
        let size = calculate_expr_size_inner(&expr);
        assert_eq!(size, 1); // Lambda falls into catchall case
    }

    #[test]
    fn test_expr_size_unary() {
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let size = calculate_expr_size_inner(&expr);
        assert_eq!(size, 2); // unary + operand
    }

    #[test]
    fn test_contains_loops_with_nested_while() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![],
            }],
            else_body: None,
        }];
        assert!(contains_loops_inner(&body));
    }

    #[test]
    fn test_contains_loops_with_nested_for() {
        let body = vec![HirStmt::If {
            condition: HirExpr::Literal(Literal::Bool(true)),
            then_body: vec![HirStmt::For {
                target: AssignTarget::Symbol("i".to_string()),
                iter: HirExpr::Var("items".to_string()),
                body: vec![],
            }],
            else_body: None,
        }];
        assert!(contains_loops_inner(&body));
    }

    #[test]
    fn test_side_effect_write() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("file".to_string())),
            method: "write".to_string(),
            args: vec![HirExpr::Literal(Literal::String("data".to_string()))],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_extend() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "extend".to_string(),
            args: vec![HirExpr::List(vec![])],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_insert() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "insert".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(0)),
                HirExpr::Literal(Literal::Int(42)),
            ],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_pop() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "pop".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_remove() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "remove".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_clear() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "clear".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_update() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "update".to_string(),
            args: vec![HirExpr::Dict(vec![])],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_pure_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "copy".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_in_args() {
        let expr = HirExpr::Call {
            func: "pure_fn".to_string(),
            args: vec![HirExpr::Call {
                func: "print".to_string(),
                args: vec![],
                kwargs: vec![],
            }],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }

    #[test]
    fn test_side_effect_in_method_object() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("list".to_string())),
                method: "append".to_string(),
                args: vec![HirExpr::Literal(Literal::Int(1))],
                kwargs: vec![],
            }),
            method: "copy".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_has_side_effects_inner(&expr));
    }
}
