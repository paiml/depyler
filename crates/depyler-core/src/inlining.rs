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
    param_count: usize,
    /// Number of return statements
    return_count: usize,
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
                            if decisions.get(callee).map(|d| d.should_inline).unwrap_or(false) {
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
                !inlined_functions.contains(&f.name) ||
                self.function_metrics.get(&f.name).map(|m| m.call_count > 1).unwrap_or(true)
            });
        }

        program
    }

    fn build_call_graph(&mut self, program: &HirProgram) {
        for func in &program.functions {
            let calls = self.extract_calls_from_function(func);
            
            self.call_graph.calls.insert(func.name.clone(), calls.clone());
            
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
            HirStmt::If { condition, then_body, else_body } => {
                self.extract_calls_from_expr(condition, calls);
                for s in then_body {
                    self.extract_calls_from_stmt(s, calls);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.extract_calls_from_stmt(s, calls);
                    }
                }
            }
            HirStmt::While { condition, body } | HirStmt::For { iter: condition, body, .. } => {
                self.extract_calls_from_expr(condition, calls);
                for s in body {
                    self.extract_calls_from_stmt(s, calls);
                }
            }
            _ => {}
        }
    }

    fn extract_calls_from_expr(&self, expr: &HirExpr, calls: &mut HashSet<String>) {
        match expr {
            HirExpr::Call { func, args } => {
                calls.insert(func.clone());
                for arg in args {
                    self.extract_calls_from_expr(arg, calls);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.extract_calls_from_expr(left, calls);
                self.extract_calls_from_expr(right, calls);
            }
            HirExpr::Unary { operand, .. } => {
                self.extract_calls_from_expr(operand, calls);
            }
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                for item in items {
                    self.extract_calls_from_expr(item, calls);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.extract_calls_from_expr(k, calls);
                    self.extract_calls_from_expr(v, calls);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.extract_calls_from_expr(object, calls);
                for arg in args {
                    self.extract_calls_from_expr(arg, calls);
                }
            }
            HirExpr::Lambda { body, .. } => {
                self.extract_calls_from_expr(body, calls);
            }
            _ => {}
        }
    }

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
            let call_count = self.call_graph
                .called_by
                .get(&func.name)
                .map(|callers| callers.len())
                .unwrap_or(0);
            
            // Estimate execution cost
            let cost = self.estimate_cost(func, size, has_loops, has_side_effects);
            
            let metrics = FunctionMetrics {
                size,
                param_count: func.params.len(),
                return_count,
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
            HirStmt::If { condition, then_body, else_body } => {
                let mut size = 1 + self.calculate_expr_size(condition);
                for s in then_body {
                    size += self.calculate_stmt_size(s);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        size += self.calculate_stmt_size(s);
                    }
                }
                size
            }
            HirStmt::While { condition, body } | HirStmt::For { iter: condition, body, .. } => {
                let mut size = 1 + self.calculate_expr_size(condition);
                for s in body {
                    size += self.calculate_stmt_size(s);
                }
                size
            }
            _ => 1,
        }
    }

    fn calculate_expr_size(&self, expr: &HirExpr) -> usize {
        match expr {
            HirExpr::Literal(_) | HirExpr::Var(_) => 1,
            HirExpr::Binary { left, right, .. } => {
                1 + self.calculate_expr_size(left) + self.calculate_expr_size(right)
            }
            HirExpr::Unary { operand, .. } => 1 + self.calculate_expr_size(operand),
            HirExpr::Call { args, .. } => {
                1 + args.iter().map(|a| self.calculate_expr_size(a)).sum::<usize>()
            }
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                1 + items.iter().map(|i| self.calculate_expr_size(i)).sum::<usize>()
            }
            HirExpr::Dict(pairs) => {
                1 + pairs.iter().map(|(k, v)| {
                    self.calculate_expr_size(k) + self.calculate_expr_size(v)
                }).sum::<usize>()
            }
            _ => 1,
        }
    }

    fn contains_loops(&self, body: &[HirStmt]) -> bool {
        for stmt in body {
            match stmt {
                HirStmt::While { .. } | HirStmt::For { .. } => return true,
                HirStmt::If { then_body, else_body, .. } => {
                    if self.contains_loops(then_body) {
                        return true;
                    }
                    if let Some(else_stmts) = else_body {
                        if self.contains_loops(else_stmts) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
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
            HirStmt::If { condition, then_body, else_body } => {
                self.expr_has_side_effects(condition) ||
                then_body.iter().any(|s| self.stmt_has_side_effects(s)) ||
                else_body.as_ref().map(|stmts| stmts.iter().any(|s| self.stmt_has_side_effects(s))).unwrap_or(false)
            }
            HirStmt::While { condition, body } | HirStmt::For { iter: condition, body, .. } => {
                self.expr_has_side_effects(condition) ||
                body.iter().any(|s| self.stmt_has_side_effects(s))
            }
            HirStmt::Raise { .. } => true, // Exceptions are side effects
            _ => false,
        }
    }

    fn expr_has_side_effects(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Call { func, args } => {
                // Known side-effect-free functions
                let pure_functions = ["len", "abs", "min", "max", "sum", "str", "int", "float"];
                !pure_functions.contains(&func.as_str()) ||
                args.iter().any(|a| self.expr_has_side_effects(a))
            }
            HirExpr::MethodCall { method, .. } => {
                // Methods that mutate are side effects
                let mutating_methods = ["append", "extend", "remove", "pop", "clear", "sort", "reverse"];
                mutating_methods.contains(&method.as_str())
            }
            HirExpr::Binary { left, right, .. } => {
                self.expr_has_side_effects(left) || self.expr_has_side_effects(right)
            }
            HirExpr::Unary { operand, .. } => self.expr_has_side_effects(operand),
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                items.iter().any(|i| self.expr_has_side_effects(i))
            }
            HirExpr::Dict(pairs) => {
                pairs.iter().any(|(k, v)| {
                    self.expr_has_side_effects(k) || self.expr_has_side_effects(v)
                })
            }
            _ => false,
        }
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
        let mut count = 0;
        for stmt in body {
            match stmt {
                HirStmt::Return(_) => count += 1,
                HirStmt::If { then_body, else_body, .. } => {
                    count += self.count_returns(then_body);
                    if let Some(else_stmts) = else_body {
                        count += self.count_returns(else_stmts);
                    }
                }
                HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                    count += self.count_returns(body);
                }
                _ => {}
            }
        }
        count
    }

    fn estimate_cost(&self, func: &HirFunction, size: usize, has_loops: bool, has_side_effects: bool) -> f64 {
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
        let return_count = self.count_returns(&func.body);
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
        // Check for recursion
        if self.call_graph.recursive.contains(func_name) {
            return InliningDecision {
                should_inline: false,
                reason: InliningReason::Recursive,
                cost_benefit: 0.0,
            };
        }
        
        // Check for trivial functions
        if self.config.inline_trivial && metrics.is_trivial {
            return InliningDecision {
                should_inline: true,
                reason: InliningReason::Trivial,
                cost_benefit: 10.0, // High benefit for trivial functions
            };
        }
        
        // Check for single-use functions
        if self.config.inline_single_use && metrics.call_count == 1 && !metrics.has_side_effects {
            return InliningDecision {
                should_inline: true,
                reason: InliningReason::SingleUse,
                cost_benefit: 5.0,
            };
        }
        
        // Check size constraints
        if metrics.size > self.config.max_inline_size {
            return InliningDecision {
                should_inline: false,
                reason: InliningReason::TooLarge,
                cost_benefit: 0.0,
            };
        }
        
        // Check for loops
        if metrics.has_loops && !self.config.inline_loops {
            return InliningDecision {
                should_inline: false,
                reason: InliningReason::ContainsLoops,
                cost_benefit: 0.0,
            };
        }
        
        // Check for side effects
        if metrics.has_side_effects {
            return InliningDecision {
                should_inline: false,
                reason: InliningReason::HasSideEffects,
                cost_benefit: 0.0,
            };
        }
        
        // Calculate cost-benefit ratio
        let call_overhead = 1.0; // Base cost of a function call
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
            HirStmt::Expr(HirExpr::Call { func, args }) => {
                if let Some(decision) = decisions.get(func) {
                    if decision.should_inline {
                        if let Some(target_func) = function_map.get(func) {
                            return Some(self.inline_function_call(target_func, args, function_map, decisions, depth));
                        }
                    }
                }
                None
            }
            HirStmt::Assign { target, value } => {
                if let HirExpr::Call { func, args } = value {
                    if let Some(decision) = decisions.get(func) {
                        if decision.should_inline {
                            if let Some(target_func) = function_map.get(func) {
                                let inlined = self.inline_function_call(target_func, args, function_map, decisions, depth);
                                // Modify the last statement to assign to the target
                                if !inlined.is_empty() {
                                    let mut result = inlined;
                                    if let Some(last) = result.last_mut() {
                                        if let HirStmt::Return(Some(expr)) = last {
                                            *last = HirStmt::Assign {
                                                target: target.clone(),
                                                value: expr.clone(),
                                            };
                                        }
                                    }
                                    return Some(result);
                                }
                            }
                        }
                    }
                }
                None
            }
            _ => None,
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
        for (i, (param_name, _)) in func.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                inlined_body.push(HirStmt::Assign {
                    target: crate::hir::AssignTarget::Symbol(format!("_inline_{}", param_name)),
                    value: arg.clone(),
                });
            }
        }
        
        // Copy and transform the function body
        for stmt in &func.body {
            let transformed = self.transform_stmt_for_inlining(stmt, &func.params, depth + 1);
            
            // Recursively try to inline nested calls
            if let Some(inlined) = self.try_inline_stmt(&transformed, function_map, decisions, depth + 1) {
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
        params: &[(String, crate::hir::Type)],
        _depth: usize,
    ) -> HirStmt {
        match stmt {
            HirStmt::Expr(expr) => HirStmt::Expr(self.transform_expr_for_inlining(expr, params)),
            HirStmt::Assign { target, value } => HirStmt::Assign {
                target: self.transform_assign_target_for_inlining(target, params),
                value: self.transform_expr_for_inlining(value, params),
            },
            HirStmt::Return(Some(expr)) => HirStmt::Return(Some(self.transform_expr_for_inlining(expr, params))),
            HirStmt::If { condition, then_body, else_body } => HirStmt::If {
                condition: self.transform_expr_for_inlining(condition, params),
                then_body: then_body.iter()
                    .map(|s| self.transform_stmt_for_inlining(s, params, _depth))
                    .collect(),
                else_body: else_body.as_ref().map(|stmts| {
                    stmts.iter()
                        .map(|s| self.transform_stmt_for_inlining(s, params, _depth))
                        .collect()
                }),
            },
            _ => stmt.clone(),
        }
    }

    fn transform_expr_for_inlining(
        &self,
        expr: &HirExpr,
        params: &[(String, crate::hir::Type)],
    ) -> HirExpr {
        match expr {
            HirExpr::Var(name) => {
                // Replace parameter references with inlined versions
                if params.iter().any(|(p, _)| p == name) {
                    HirExpr::Var(format!("_inline_{}", name))
                } else {
                    expr.clone()
                }
            }
            HirExpr::Binary { left, right, op } => HirExpr::Binary {
                left: Box::new(self.transform_expr_for_inlining(left, params)),
                right: Box::new(self.transform_expr_for_inlining(right, params)),
                op: *op,
            },
            HirExpr::Unary { operand, op } => HirExpr::Unary {
                operand: Box::new(self.transform_expr_for_inlining(operand, params)),
                op: *op,
            },
            HirExpr::Call { func, args } => HirExpr::Call {
                func: func.clone(),
                args: args.iter()
                    .map(|a| self.transform_expr_for_inlining(a, params))
                    .collect(),
            },
            _ => expr.clone(),
        }
    }

    fn transform_assign_target_for_inlining(
        &self,
        target: &crate::hir::AssignTarget,
        params: &[(String, crate::hir::Type)],
    ) -> crate::hir::AssignTarget {
        match target {
            crate::hir::AssignTarget::Symbol(name) => {
                if params.iter().any(|(p, _)| p == name) {
                    crate::hir::AssignTarget::Symbol(format!("_inline_{}", name))
                } else {
                    target.clone()
                }
            }
            _ => target.clone(),
        }
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
                target: AssignTarget::Symbol(format!("x{}", i)),
                value: HirExpr::Literal(Literal::Int(i as i64)),
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
            params: smallvec![("x".to_string(), Type::Int)],
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
        assert_eq!(size, 6); // 5 assignments + 1 return
    }

    #[test]
    fn test_loop_detection() {
        let body = vec![
            HirStmt::While {
                condition: HirExpr::Literal(Literal::Bool(true)),
                body: vec![HirStmt::Break { label: None }],
            },
        ];

        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(analyzer.contains_loops(&body));
    }

    #[test]
    fn test_side_effect_detection() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(42))],
        };

        let analyzer = InliningAnalyzer::new(InliningConfig::default());
        assert!(analyzer.expr_has_side_effects(&expr));
    }

    #[test]
    fn test_inlining_config_default() {
        let config = InliningConfig::default();
        assert_eq!(config.max_inline_size, 20);
        assert_eq!(config.max_inline_depth, 3);
        assert!(config.inline_single_use);
        assert!(config.inline_trivial);
    }
}