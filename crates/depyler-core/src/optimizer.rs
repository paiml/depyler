//! Optimization passes for generated Rust code

use crate::hir::{
    AssignTarget, BinOp, HirExpr, HirFunction, HirProgram, HirStmt, Literal, UnaryOp,
};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// Main optimizer that runs various optimization passes
pub struct Optimizer {
    /// Configuration for optimization passes
    config: OptimizerConfig,
}

#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Enable inlining of small functions
    pub inline_functions: bool,
    /// Enable dead code elimination
    pub eliminate_dead_code: bool,
    /// Enable constant propagation
    pub propagate_constants: bool,
    /// Enable common subexpression elimination
    pub eliminate_common_subexpressions: bool,
    /// Maximum function size for inlining (in HIR nodes)
    pub inline_threshold: usize,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            // DEPYLER-0161: Disabled broken inlining optimization
            // BUG: Inlining pass marks functions as "Trivial" but doesn't inline them,
            // then dead code elimination removes assignments, leaving undefined variables.
            // TODO: Fix inlining logic in v3.19.0, then re-enable this optimization.
            inline_functions: false,
            eliminate_dead_code: true,
            propagate_constants: true,
            eliminate_common_subexpressions: true,
            inline_threshold: 20,
        }
    }
}

impl Optimizer {
    pub fn new(config: OptimizerConfig) -> Self {
        Self { config }
    }

    /// Run all optimization passes on a HIR program
    pub fn optimize_program(&mut self, mut program: HirProgram) -> HirProgram {
        // Pass 1: Constant propagation
        if self.config.propagate_constants {
            program = self.propagate_constants_program(program);
        }

        // Pass 2: Dead code elimination
        if self.config.eliminate_dead_code {
            program = self.eliminate_dead_code_program(program);
        }

        // Pass 3: Function inlining
        if self.config.inline_functions {
            program = self.inline_functions_program(program);
        }

        // Pass 4: Common subexpression elimination
        if self.config.eliminate_common_subexpressions {
            program = self.eliminate_common_subexpressions_program(program);
        }

        program
    }

    /// Propagate constant values through the program
    fn propagate_constants_program(&self, mut program: HirProgram) -> HirProgram {
        let mut constants = HashMap::new();

        // First pass: find which variables are mutated (assigned more than once)
        let mut mutated_vars = HashSet::new();
        for func in &program.functions {
            self.collect_mutated_vars_function(func, &mut mutated_vars);
        }

        // Second pass: collect constants (but skip mutated variables)
        for func in &program.functions {
            self.collect_constants_function(func, &mut constants, &mutated_vars);
        }

        // Third pass: propagate constants
        for func in &mut program.functions {
            self.propagate_constants_function(func, &constants);
        }

        program
    }

    fn collect_mutated_vars_function(&self, func: &HirFunction, mutated_vars: &mut HashSet<String>) {
        let mut assignments = HashMap::new();
        self.count_assignments_stmt(&func.body, &mut assignments);

        // Any variable assigned more than once is mutated
        for (var, count) in assignments {
            if count > 1 {
                mutated_vars.insert(var);
            }
        }
    }

    fn count_assignments_stmt(&self, stmts: &[HirStmt], assignments: &mut HashMap<String, usize>) {
        for stmt in stmts {
            self.count_assignments_in_single_stmt(stmt, assignments);
        }
    }

    fn count_assignments_in_single_stmt(&self, stmt: &HirStmt, assignments: &mut HashMap<String, usize>) {
        match stmt {
            HirStmt::Assign { target: AssignTarget::Symbol(name), .. } => {
                *assignments.entry(name.clone()).or_insert(0) += 1;
            }
            HirStmt::If { then_body, else_body, .. } => {
                self.count_assignments_stmt(then_body, assignments);
                if let Some(else_stmts) = else_body {
                    self.count_assignments_stmt(else_stmts, assignments);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                self.count_assignments_stmt(body, assignments);
            }
            _ => {}
        }
    }

    fn collect_constants_function(
        &self,
        func: &HirFunction,
        constants: &mut HashMap<String, HirExpr>,
        mutated_vars: &HashSet<String>,
    ) {
        for stmt in &func.body {
            self.collect_constants_stmt(stmt, constants, mutated_vars);
        }
    }

    fn collect_constants_stmt(&self, stmt: &HirStmt, constants: &mut HashMap<String, HirExpr>, mutated_vars: &HashSet<String>) {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                value,
                ..
            } => {
                // Only treat as constant if variable is never mutated AND value is constant
                if !mutated_vars.contains(name) && self.is_constant_expr(value) {
                    constants.insert(name.clone(), value.clone());
                }
            }
            HirStmt::Assign { .. } => {}
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    self.collect_constants_stmt(s, constants, mutated_vars);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_constants_stmt(s, constants, mutated_vars);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for s in body {
                    self.collect_constants_stmt(s, constants, mutated_vars);
                }
            }
            _ => {}
        }
    }

    fn is_constant_expr(&self, expr: &HirExpr) -> bool {
        is_constant_expr_inner(expr)
    }

    fn propagate_constants_function(
        &self,
        func: &mut HirFunction,
        constants: &HashMap<String, HirExpr>,
    ) {
        for stmt in &mut func.body {
            self.propagate_constants_stmt(stmt, constants);
        }
    }

    fn propagate_constants_stmt(&self, stmt: &mut HirStmt, constants: &HashMap<String, HirExpr>) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.propagate_constants_expr(value, constants);
            }
            HirStmt::Return(Some(expr)) => {
                self.propagate_constants_expr(expr, constants);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.propagate_constants_expr(condition, constants);
                for s in then_body {
                    self.propagate_constants_stmt(s, constants);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.propagate_constants_stmt(s, constants);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.propagate_constants_expr(condition, constants);
                for s in body {
                    self.propagate_constants_stmt(s, constants);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.propagate_constants_expr(iter, constants);
                for s in body {
                    self.propagate_constants_stmt(s, constants);
                }
            }
            HirStmt::Expr(expr) => {
                self.propagate_constants_expr(expr, constants);
            }
            _ => {}
        }
    }

    fn propagate_constants_expr(&self, expr: &mut HirExpr, constants: &HashMap<String, HirExpr>) {
        match expr {
            HirExpr::Var(name) => {
                if let Some(const_expr) = constants.get(name) {
                    *expr = const_expr.clone();
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.propagate_constants_expr(left, constants);
                self.propagate_constants_expr(right, constants);

                // Try to evaluate constant expressions
                if let Some(result) = self.evaluate_constant_binop(expr) {
                    *expr = result;
                }
            }
            HirExpr::Unary { operand, .. } => {
                self.propagate_constants_expr(operand, constants);

                // Try to evaluate constant expressions
                if let Some(result) = self.evaluate_constant_unaryop(expr) {
                    *expr = result;
                }
            }
            HirExpr::List(items) => {
                for item in items {
                    self.propagate_constants_expr(item, constants);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.propagate_constants_expr(k, constants);
                    self.propagate_constants_expr(v, constants);
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.propagate_constants_expr(arg, constants);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.propagate_constants_expr(object, constants);
                for arg in args {
                    self.propagate_constants_expr(arg, constants);
                }
            }
            HirExpr::Lambda { body, .. } => {
                self.propagate_constants_expr(body, constants);
            }
            _ => {}
        }
    }

    fn evaluate_constant_binop(&self, expr: &HirExpr) -> Option<HirExpr> {
        if let HirExpr::Binary { left, right, op } = expr {
            match (left.as_ref(), right.as_ref(), op) {
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Add,
                ) => Some(HirExpr::Literal(Literal::Int(a + b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Sub,
                ) => Some(HirExpr::Literal(Literal::Int(a - b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Mul,
                ) => Some(HirExpr::Literal(Literal::Int(a * b))),
                (
                    HirExpr::Literal(Literal::Int(a)),
                    HirExpr::Literal(Literal::Int(b)),
                    BinOp::Div,
                ) if *b != 0 => Some(HirExpr::Literal(Literal::Int(a / b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Add,
                ) => Some(HirExpr::Literal(Literal::Float(a + b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Sub,
                ) => Some(HirExpr::Literal(Literal::Float(a - b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Mul,
                ) => Some(HirExpr::Literal(Literal::Float(a * b))),
                (
                    HirExpr::Literal(Literal::Float(a)),
                    HirExpr::Literal(Literal::Float(b)),
                    BinOp::Div,
                ) if *b != 0.0 => Some(HirExpr::Literal(Literal::Float(a / b))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn evaluate_constant_unaryop(&self, expr: &HirExpr) -> Option<HirExpr> {
        if let HirExpr::Unary { op, operand } = expr {
            match (operand.as_ref(), op) {
                (HirExpr::Literal(Literal::Int(n)), UnaryOp::Neg) => {
                    Some(HirExpr::Literal(Literal::Int(-n)))
                }
                (HirExpr::Literal(Literal::Float(f)), UnaryOp::Neg) => {
                    Some(HirExpr::Literal(Literal::Float(-f)))
                }
                (HirExpr::Literal(Literal::Bool(b)), UnaryOp::Not) => {
                    Some(HirExpr::Literal(Literal::Bool(!b)))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Eliminate dead code from the program
    fn eliminate_dead_code_program(&self, mut program: HirProgram) -> HirProgram {
        for func in &mut program.functions {
            self.eliminate_dead_code_function(func);
        }
        program
    }

    fn eliminate_dead_code_function(&self, func: &mut HirFunction) {
        // Collect used variables
        let mut used_vars = HashMap::new();
        for stmt in &func.body {
            self.collect_used_vars_stmt(stmt, &mut used_vars);
        }

        // Remove assignments to unused variables
        func.body.retain(|stmt| {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } = stmt
            {
                used_vars.contains_key(name)
            } else {
                true
            }
        });
    }

    fn collect_used_vars_stmt(&self, stmt: &HirStmt, used: &mut HashMap<String, bool>) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.collect_used_vars_expr(value, used);
            }
            HirStmt::Return(Some(expr)) => {
                self.collect_used_vars_expr(expr, used);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_used_vars_expr(condition, used);
                for s in then_body {
                    self.collect_used_vars_stmt(s, used);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_used_vars_stmt(s, used);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.collect_used_vars_expr(condition, used);
                for s in body {
                    self.collect_used_vars_stmt(s, used);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.collect_used_vars_expr(iter, used);
                for s in body {
                    self.collect_used_vars_stmt(s, used);
                }
            }
            HirStmt::Expr(expr) => {
                self.collect_used_vars_expr(expr, used);
            }
            _ => {}
        }
    }

    fn collect_used_vars_expr(&self, expr: &HirExpr, used: &mut HashMap<String, bool>) {
        collect_used_vars_expr_inner(expr, used);
    }

    /// Inline small functions using sophisticated heuristics
    fn inline_functions_program(&self, program: HirProgram) -> HirProgram {
        use crate::inlining::{InliningAnalyzer, InliningConfig};

        // Configure inlining based on optimizer settings
        let config = InliningConfig {
            max_inline_size: self.config.inline_threshold,
            max_inline_depth: 3,
            inline_single_use: true,
            inline_trivial: true,
            cost_threshold: 1.5,
            inline_loops: false,
        };

        // Analyze the program for inlining opportunities
        let mut analyzer = InliningAnalyzer::new(config);
        let decisions = analyzer.analyze_program(&program);

        // Report inlining decisions if verbose
        for (func_name, decision) in &decisions {
            if decision.should_inline {
                eprintln!(
                    "Inlining function '{}': {:?} (cost-benefit: {:.2})",
                    func_name, decision.reason, decision.cost_benefit
                );
            }
        }

        // Apply the inlining transformations
        analyzer.apply_inlining(program, &decisions)
    }

    /// Eliminate common subexpressions
    fn eliminate_common_subexpressions_program(&self, mut program: HirProgram) -> HirProgram {
        for func in &mut program.functions {
            let mut cse_map: HashMap<u64, (HirExpr, String)> = HashMap::new();
            let mut temp_counter = 0;

            func.body = self.eliminate_cse_in_body(&func.body, &mut cse_map, &mut temp_counter);
        }

        program
    }

    fn eliminate_cse_in_body(
        &self,
        body: &[HirStmt],
        cse_map: &mut HashMap<u64, (HirExpr, String)>,
        temp_counter: &mut usize,
    ) -> Vec<HirStmt> {
        let mut new_body = Vec::new();

        for stmt in body {
            match stmt {
                HirStmt::Assign { target, value, type_annotation } => {
                    let (new_value, extra_stmts) =
                        self.process_expr_for_cse(value, cse_map, temp_counter);
                    new_body.extend(extra_stmts);
                    new_body.push(HirStmt::Assign {
                        target: target.clone(),
                        value: new_value,
                        type_annotation: type_annotation.clone(),
                    });
                }
                HirStmt::Return(Some(expr)) => {
                    let (new_expr, extra_stmts) =
                        self.process_expr_for_cse(expr, cse_map, temp_counter);
                    new_body.extend(extra_stmts);
                    new_body.push(HirStmt::Return(Some(new_expr)));
                }
                HirStmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    let (new_condition, extra_stmts) =
                        self.process_expr_for_cse(condition, cse_map, temp_counter);
                    new_body.extend(extra_stmts);

                    // CSE within branches (with separate scopes)
                    let mut then_cse = cse_map.clone();
                    let new_then =
                        self.eliminate_cse_in_body(then_body, &mut then_cse, temp_counter);

                    let new_else = else_body.as_ref().map(|else_stmts| {
                        let mut else_cse = cse_map.clone();
                        self.eliminate_cse_in_body(else_stmts, &mut else_cse, temp_counter)
                    });

                    new_body.push(HirStmt::If {
                        condition: new_condition,
                        then_body: new_then,
                        else_body: new_else,
                    });
                }
                _ => new_body.push(stmt.clone()),
            }
        }

        new_body
    }

    fn process_expr_for_cse(
        &self,
        expr: &HirExpr,
        cse_map: &mut HashMap<u64, (HirExpr, String)>,
        temp_counter: &mut usize,
    ) -> (HirExpr, Vec<HirStmt>) {
        let mut extra_stmts = Vec::new();

        // Only process complex expressions
        match expr {
            HirExpr::Binary { left, right, op } => {
                // Recursively process operands
                let (new_left, left_stmts) = self.process_expr_for_cse(left, cse_map, temp_counter);
                let (new_right, right_stmts) =
                    self.process_expr_for_cse(right, cse_map, temp_counter);
                extra_stmts.extend(left_stmts);
                extra_stmts.extend(right_stmts);

                let new_expr = HirExpr::Binary {
                    op: *op,
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                };

                // Check if this expression is worth caching (not trivial)
                if self.is_complex_expr(&new_expr) {
                    let hash = self.hash_expr(&new_expr);

                    if let Some((_, var_name)) = cse_map.get(&hash) {
                        // Reuse existing computation
                        (HirExpr::Var(var_name.clone()), extra_stmts)
                    } else {
                        // Create new temporary
                        let temp_name = format!("_cse_temp_{}", temp_counter);
                        *temp_counter += 1;

                        extra_stmts.push(HirStmt::Assign {
                            target: AssignTarget::Symbol(temp_name.clone()),
                            value: new_expr.clone(),
                            type_annotation: None,
                        });

                        cse_map.insert(hash, (new_expr, temp_name.clone()));
                        (HirExpr::Var(temp_name), extra_stmts)
                    }
                } else {
                    (new_expr, extra_stmts)
                }
            }
            HirExpr::Call { func, args } if self.is_pure_function(func) => {
                // Process arguments
                let mut new_args = Vec::new();
                for arg in args {
                    let (new_arg, arg_stmts) =
                        self.process_expr_for_cse(arg, cse_map, temp_counter);
                    extra_stmts.extend(arg_stmts);
                    new_args.push(new_arg);
                }

                let new_expr = HirExpr::Call {
                    func: func.clone(),
                    args: new_args,
                };

                let hash = self.hash_expr(&new_expr);

                if let Some((_, var_name)) = cse_map.get(&hash) {
                    (HirExpr::Var(var_name.clone()), extra_stmts)
                } else {
                    let temp_name = format!("_cse_temp_{}", temp_counter);
                    *temp_counter += 1;

                    extra_stmts.push(HirStmt::Assign {
                        target: AssignTarget::Symbol(temp_name.clone()),
                        value: new_expr.clone(),
                        type_annotation: None,
                    });

                    cse_map.insert(hash, (new_expr, temp_name.clone()));
                    (HirExpr::Var(temp_name), extra_stmts)
                }
            }
            _ => (expr.clone(), extra_stmts),
        }
    }

    fn is_complex_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Binary { op, left, right } => {
                // Consider non-trivial operations or non-literal operands
                !matches!(op, BinOp::Add | BinOp::Sub)
                    || !matches!(left.as_ref(), HirExpr::Var(_) | HirExpr::Literal(_))
                    || !matches!(right.as_ref(), HirExpr::Var(_) | HirExpr::Literal(_))
            }
            HirExpr::Call { .. } => true,
            _ => false,
        }
    }

    fn is_pure_function(&self, func: &str) -> bool {
        // List of known pure functions
        let pure_functions = [
            "abs", "len", "min", "max", "sum", "str", "int", "float", "bool", "round", "pow",
            "sqrt",
        ];
        pure_functions.contains(&func)
    }

    fn hash_expr(&self, expr: &HirExpr) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        self.hash_expr_recursive(expr, &mut hasher);
        hasher.finish()
    }

    fn hash_expr_recursive<H: Hasher>(&self, expr: &HirExpr, hasher: &mut H) {
        hash_expr_recursive_inner(expr, hasher);
    }
}

fn hash_expr_recursive_inner<H: Hasher>(expr: &HirExpr, hasher: &mut H) {
    match expr {
        HirExpr::Literal(lit) => {
            "literal".hash(hasher);
            match lit {
                Literal::Int(n) => n.hash(hasher),
                Literal::Float(f) => f.to_bits().hash(hasher),
                Literal::String(s) => s.hash(hasher),
                Literal::Bool(b) => b.hash(hasher),
                Literal::None => "none".hash(hasher),
            }
        }
        HirExpr::Var(name) => {
            "var".hash(hasher);
            name.hash(hasher);
        }
        HirExpr::Binary { op, left, right } => {
            "binary".hash(hasher);
            format!("{:?}", op).hash(hasher);
            hash_expr_recursive_inner(left, hasher);
            hash_expr_recursive_inner(right, hasher);
        }
        HirExpr::Call { func, args } => {
            "call".hash(hasher);
            func.hash(hasher);
            for arg in args {
                hash_expr_recursive_inner(arg, hasher);
            }
        }
        _ => {
            // For other expressions, use a simple discriminant
            format!("{:?}", std::mem::discriminant(expr)).hash(hasher);
        }
    }
}

fn is_constant_expr_inner(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Literal(_) => true,
        HirExpr::Unary { operand, .. } => is_constant_expr_inner(operand),
        HirExpr::Binary { left, right, .. } => {
            is_constant_expr_inner(left) && is_constant_expr_inner(right)
        }
        _ => false,
    }
}

fn collect_used_vars_expr_inner(expr: &HirExpr, used: &mut HashMap<String, bool>) {
    match expr {
        HirExpr::Var(name) => {
            used.insert(name.clone(), true);
        }
        HirExpr::Binary { left, right, .. } => {
            collect_used_vars_expr_inner(left, used);
            collect_used_vars_expr_inner(right, used);
        }
        HirExpr::Unary { operand, .. } => {
            collect_used_vars_expr_inner(operand, used);
        }
        HirExpr::List(items) => {
            for item in items {
                collect_used_vars_expr_inner(item, used);
            }
        }
        HirExpr::Tuple(items) => {
            // DEPYLER-0161 FIX: Collect variables from tuple expressions
            // This was causing dead code elimination to remove assignments
            // for variables used in tuple returns like: return (a, b, c)
            for item in items {
                collect_used_vars_expr_inner(item, used);
            }
        }
        HirExpr::Dict(pairs) => {
            for (k, v) in pairs {
                collect_used_vars_expr_inner(k, used);
                collect_used_vars_expr_inner(v, used);
            }
        }
        HirExpr::Call { func, args } => {
            // Mark the function name as used (important for lambda variables)
            used.insert(func.clone(), true);
            for arg in args {
                collect_used_vars_expr_inner(arg, used);
            }
        }
        HirExpr::MethodCall { object, args, .. } => {
            collect_used_vars_expr_inner(object, used);
            for arg in args {
                collect_used_vars_expr_inner(arg, used);
            }
        }
        HirExpr::Lambda { body, .. } => {
            collect_used_vars_expr_inner(body, used);
        }
        HirExpr::ListComp {
            element,
            iter,
            condition,
            ..
        } => {
            collect_used_vars_expr_inner(element, used);
            collect_used_vars_expr_inner(iter, used);
            if let Some(cond) = condition {
                collect_used_vars_expr_inner(cond, used);
            }
        }
        HirExpr::SetComp {
            element,
            iter,
            condition,
            ..
        } => {
            collect_used_vars_expr_inner(element, used);
            collect_used_vars_expr_inner(iter, used);
            if let Some(cond) = condition {
                collect_used_vars_expr_inner(cond, used);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{
        AssignTarget, BinOp, FunctionProperties, HirExpr, HirFunction, HirProgram, HirStmt,
        Literal, Type,
    };
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_constant_propagation() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("x".to_string()),
                        value: HirExpr::Literal(Literal::Int(5)),
                        type_annotation: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("y".to_string()),
                        value: HirExpr::Binary {
                            left: Box::new(HirExpr::Var("x".to_string())),
                            op: BinOp::Add,
                            right: Box::new(HirExpr::Literal(Literal::Int(3))),
                        },
                        type_annotation: None,
                    },
                    HirStmt::Return(Some(HirExpr::Var("y".to_string()))),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // Check that constant propagation occurred
        let func = &optimized.functions[0];
        if let HirStmt::Assign { value, .. } = &func.body[1] {
            // The expression x + 3 should be optimized to 8
            assert!(matches!(value, HirExpr::Literal(Literal::Int(8))));
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("unused".to_string()),
                        value: HirExpr::Literal(Literal::Int(42)),
                        type_annotation: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("used".to_string()),
                        value: HirExpr::Literal(Literal::Int(10)),
                        type_annotation: None,
                    },
                    HirStmt::Return(Some(HirExpr::Var("used".to_string()))),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // Check optimization results
        let func = &optimized.functions[0];
        // After constant propagation and dead code elimination, we should only have the return statement
        // Both assignments should be eliminated since the value 10 is propagated directly to the return
        assert_eq!(
            func.body.len(),
            1,
            "Expected 1 statement after optimization, got: {:?}",
            func.body
        );

        // The return should directly contain the literal value
        assert!(matches!(
            &func.body[0],
            HirStmt::Return(Some(HirExpr::Literal(Literal::Int(10))))
        ));
    }
}
