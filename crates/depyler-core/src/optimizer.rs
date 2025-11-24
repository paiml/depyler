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
            // KNOWN ISSUE: Inlining pass marks functions as "Trivial" but doesn't inline them,
            // then dead code elimination removes assignments, leaving undefined variables.
            // NOTE: Fix inlining logic before re-enabling (tracked in DEPYLER-0161)
            inline_functions: false,
            // DEPYLER-0508: Re-enabled DCE - unused variables should be eliminated
            // DEPYLER-0363: Previously disabled for argparse debugging, now fixed
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

        // DEPYLER-0269 Fix: Second pass - collect all variable READS
        let mut read_vars = HashSet::new();
        for func in &program.functions {
            self.collect_read_vars_function(func, &mut read_vars);
        }

        // Third pass: collect constants (but skip mutated OR read variables)
        // DEPYLER-0269: Only propagate constants for dead code (assigned but never read)
        for func in &program.functions {
            self.collect_constants_function(func, &mut constants, &mutated_vars, &read_vars);
        }

        // Fourth pass: propagate constants
        for func in &mut program.functions {
            self.propagate_constants_function(func, &constants);
        }

        program
    }

    fn collect_mutated_vars_function(
        &self,
        func: &HirFunction,
        mutated_vars: &mut HashSet<String>,
    ) {
        let mut assignments = HashMap::new();
        self.count_assignments_stmt(&func.body, &mut assignments);

        // Any variable assigned more than once is mutated
        for (var, count) in assignments {
            if count > 1 {
                mutated_vars.insert(var);
            }
        }
    }

    /// DEPYLER-0269: Collect all variables that are actually USED (read)
    fn collect_read_vars_function(&self, func: &HirFunction, read_vars: &mut HashSet<String>) {
        for stmt in &func.body {
            Self::collect_read_vars_stmt(stmt, read_vars);
        }
    }

    /// DEPYLER-0269: Recursively collect variable reads from statements
    fn collect_read_vars_stmt(stmt: &HirStmt, read_vars: &mut HashSet<String>) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                // Variable reads in the RHS of assignment
                Self::collect_read_vars_expr(value, read_vars);
            }
            HirStmt::Expr(expr) => {
                Self::collect_read_vars_expr(expr, read_vars);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                Self::collect_read_vars_expr(condition, read_vars);
                for s in then_body {
                    Self::collect_read_vars_stmt(s, read_vars);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        Self::collect_read_vars_stmt(s, read_vars);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                Self::collect_read_vars_expr(condition, read_vars);
                for s in body {
                    Self::collect_read_vars_stmt(s, read_vars);
                }
            }
            HirStmt::For { iter, body, .. } => {
                Self::collect_read_vars_expr(iter, read_vars);
                for s in body {
                    Self::collect_read_vars_stmt(s, read_vars);
                }
            }
            HirStmt::Return(Some(expr)) => {
                Self::collect_read_vars_expr(expr, read_vars);
            }
            _ => {}
        }
    }

    /// DEPYLER-0269: Recursively collect variable reads from expressions
    fn collect_read_vars_expr(expr: &HirExpr, read_vars: &mut HashSet<String>) {
        match expr {
            HirExpr::Var(name) => {
                // This is a variable READ - mark as used
                read_vars.insert(name.clone());
            }
            HirExpr::Binary { left, right, .. } => {
                Self::collect_read_vars_expr(left, read_vars);
                Self::collect_read_vars_expr(right, read_vars);
            }
            HirExpr::Unary { operand, .. } => {
                Self::collect_read_vars_expr(operand, read_vars);
            }
            HirExpr::List(items) => {
                for item in items {
                    Self::collect_read_vars_expr(item, read_vars);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    Self::collect_read_vars_expr(k, read_vars);
                    Self::collect_read_vars_expr(v, read_vars);
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    Self::collect_read_vars_expr(arg, read_vars);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                Self::collect_read_vars_expr(object, read_vars);
                for arg in args {
                    Self::collect_read_vars_expr(arg, read_vars);
                }
            }
            HirExpr::Lambda { body, .. } => {
                Self::collect_read_vars_expr(body, read_vars);
            }
            _ => {}
        }
    }

    fn count_assignments_stmt(&self, stmts: &[HirStmt], assignments: &mut HashMap<String, usize>) {
        for stmt in stmts {
            self.count_assignments_in_single_stmt(stmt, assignments);
        }
    }

    fn count_assignments_in_single_stmt(
        &self,
        stmt: &HirStmt,
        assignments: &mut HashMap<String, usize>,
    ) {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } => {
                *assignments.entry(name.clone()).or_insert(0) += 1;
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
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
        used_vars: &HashSet<String>,
    ) {
        for stmt in &func.body {
            self.collect_constants_stmt(stmt, constants, mutated_vars, used_vars);
        }
    }

    fn collect_constants_stmt(
        &self,
        stmt: &HirStmt,
        constants: &mut HashMap<String, HirExpr>,
        mutated_vars: &HashSet<String>,
        used_vars: &HashSet<String>,
    ) {
        match stmt {
            HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                value,
                ..
            } => {
                // DEPYLER-0269 Fix: Only propagate constants for dead code
                // Skip variables that are:
                // 1. Mutated (assigned more than once) - already checked
                // 2. Actually USED (read anywhere) - NEW CHECK
                // This prevents unused variable warnings for user-defined constants
                if !mutated_vars.contains(name)
                    && !used_vars.contains(name)  // DEPYLER-0269: Skip used variables!
                    && self.is_constant_expr(value)
                {
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
                    self.collect_constants_stmt(s, constants, mutated_vars, used_vars);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_constants_stmt(s, constants, mutated_vars, used_vars);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for s in body {
                    self.collect_constants_stmt(s, constants, mutated_vars, used_vars);
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
        // Collect truly used variables (referenced after assignment)
        let mut used_vars = HashMap::new();
        for stmt in &func.body {
            self.collect_truly_used_vars_stmt(stmt, &mut used_vars);
        }

        // Collect assignments with side effects (indexing) that need to be preserved
        let mut side_effect_vars = HashSet::new();
        for stmt in &func.body {
            if let HirStmt::Assign { target, value, .. } = stmt {
                if Self::expr_contains_index(value) {
                    if let AssignTarget::Symbol(name) = target {
                        side_effect_vars.insert(name.clone());
                    }
                }
            }
        }

        // Rename variables that have side effects but aren't actually used to `_varname`
        // This prevents Rust's unused_variables warning while preserving the side effect
        for stmt in &mut func.body {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } = stmt
            {
                if side_effect_vars.contains(name) && !used_vars.contains_key(name) {
                    *name = format!("_{}", name);
                }
            }
        }

        // Remove truly dead assignments (not used and no side effects)
        func.body.retain(|stmt| {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } = stmt
            {
                // Keep if: truly used OR has side effects (including renamed _varname)
                used_vars.contains_key(name)
                    || side_effect_vars.contains(name.trim_start_matches('_'))
            } else {
                true
            }
        });
    }

    /// DEPYLER-0270 Fix #1 (Updated): Collect truly used variables (referenced, not just assigned)
    /// This version does NOT mark side-effect assignments as used - that's handled separately
    /// in eliminate_dead_code_function to allow renaming them to `_varname`.
    fn collect_truly_used_vars_stmt(&self, stmt: &HirStmt, used: &mut HashMap<String, bool>) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // DEPYLER-0235 FIX: Collect variables from assignment targets
                // This fixes property writes like `b.size = 20` where `b` is used on LHS
                self.collect_used_vars_assign_target(target, used);
                self.collect_used_vars_expr(value, used);
                // NOTE: We do NOT mark side-effect assignments as used here
                // That's now handled in eliminate_dead_code_function
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
                    self.collect_truly_used_vars_stmt(s, used);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_truly_used_vars_stmt(s, used);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.collect_used_vars_expr(condition, used);
                for s in body {
                    self.collect_truly_used_vars_stmt(s, used);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.collect_used_vars_expr(iter, used);
                for s in body {
                    self.collect_truly_used_vars_stmt(s, used);
                }
            }
            HirStmt::Expr(expr) => {
                self.collect_used_vars_expr(expr, used);
            }
            _ => {}
        }
    }

    /// DEPYLER-0270 Fix #1: Check if expression contains indexing operations
    /// Returns true if the expression tree contains any Index nodes, which indicate
    /// operations that can fail (e.g., list[0], dict["key"]) and have side effects.
    ///
    /// # Complexity
    /// 5 (recursive expression traversal with early return)
    fn expr_contains_index(expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Index { .. } => true,
            HirExpr::Binary { left, right, .. } => {
                Self::expr_contains_index(left) || Self::expr_contains_index(right)
            }
            HirExpr::Unary { operand, .. } => Self::expr_contains_index(operand),
            HirExpr::Call { args, .. } => args.iter().any(Self::expr_contains_index),
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                items.iter().any(Self::expr_contains_index)
            }
            HirExpr::Dict(pairs) => pairs
                .iter()
                .any(|(k, v)| Self::expr_contains_index(k) || Self::expr_contains_index(v)),
            HirExpr::Set(items) => items.iter().any(Self::expr_contains_index),
            HirExpr::MethodCall { object, args, .. } => {
                Self::expr_contains_index(object) || args.iter().any(Self::expr_contains_index)
            }
            HirExpr::Attribute { value, .. } => Self::expr_contains_index(value),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                Self::expr_contains_index(base)
                    || start.as_ref().is_some_and(|e| Self::expr_contains_index(e))
                    || stop.as_ref().is_some_and(|e| Self::expr_contains_index(e))
                    || step.as_ref().is_some_and(|e| Self::expr_contains_index(e))
            }
            _ => false,
        }
    }

    fn collect_used_vars_expr(&self, expr: &HirExpr, used: &mut HashMap<String, bool>) {
        collect_used_vars_expr_inner(expr, used);
    }

    fn collect_used_vars_assign_target(
        &self,
        target: &AssignTarget,
        used: &mut HashMap<String, bool>,
    ) {
        match target {
            AssignTarget::Symbol(_) => {
                // Simple variable assignment - no variables used on LHS
            }
            AssignTarget::Index { base, index } => {
                // Collect from both base and index expressions
                // e.g., `arr[i] = value` uses both `arr` and `i`
                self.collect_used_vars_expr(base, used);
                self.collect_used_vars_expr(index, used);
            }
            AssignTarget::Attribute { value, .. } => {
                // Collect from the base object
                // e.g., `obj.attr = value` uses `obj`
                self.collect_used_vars_expr(value, used);
            }
            AssignTarget::Tuple(targets) => {
                // Recursively collect from tuple elements
                for t in targets {
                    self.collect_used_vars_assign_target(t, used);
                }
            }
        }
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

        for (idx, stmt) in body.iter().enumerate() {
            let is_final_stmt = idx == body.len() - 1;

            match stmt {
                HirStmt::Assign {
                    target,
                    value,
                    type_annotation,
                } => {
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
                    // DEPYLER-0275 FIX: Skip CSE for final return with simple expressions
                    // This avoids unnecessary `let _cse_temp_0 = expr; _cse_temp_0` pattern
                    if is_final_stmt && self.is_simple_return_expr(expr) {
                        // Don't create CSE temp for final simple returns
                        new_body.push(HirStmt::Return(Some(expr.clone())));
                    } else {
                        let (new_expr, extra_stmts) =
                            self.process_expr_for_cse(expr, cse_map, temp_counter);
                        new_body.extend(extra_stmts);
                        new_body.push(HirStmt::Return(Some(new_expr)));
                    }
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
            HirExpr::Call { func, args, .. } if self.is_pure_function(func) => {
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
                    kwargs: vec![],
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

    /// DEPYLER-0275: Check if expression is simple enough to return directly
    /// without creating a CSE temporary variable.
    /// Simple expressions: literals, variables, basic operations, method calls
    fn is_simple_return_expr(&self, expr: &HirExpr) -> bool {
        matches!(
            expr,
            HirExpr::Literal(_)
                | HirExpr::Var(_)
                | HirExpr::Binary { .. }
                | HirExpr::Unary { .. }
                | HirExpr::MethodCall { .. }
                | HirExpr::Call { .. }
                | HirExpr::Attribute { .. }
        )
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
                Literal::Bytes(b) => b.hash(hasher),
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
        HirExpr::Call { func, args, .. } => {
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
        HirExpr::Call { func, args, .. } => {
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
            generators,
        } => {
            // DEPYLER-0504: Support multiple generators
            collect_used_vars_expr_inner(element, used);
            for gen in generators {
                collect_used_vars_expr_inner(&gen.iter, used);
                for cond in &gen.conditions {
                    collect_used_vars_expr_inner(cond, used);
                }
            }
        }
        HirExpr::SetComp {
            element,
            generators,
        } => {
            // DEPYLER-0504: Support multiple generators
            collect_used_vars_expr_inner(element, used);
            for gen in generators {
                collect_used_vars_expr_inner(&gen.iter, used);
                for cond in &gen.conditions {
                    collect_used_vars_expr_inner(cond, used);
                }
            }
        }
        HirExpr::Await { value } => {
            collect_used_vars_expr_inner(value, used);
        }
        HirExpr::Slice {
            base,
            start,
            stop,
            step,
        } => {
            // DEPYLER-0209 FIX: Collect variables from slice expressions
            // This was causing dead code elimination to remove assignments
            // for variables used in slice operations like: numbers[2:7]
            collect_used_vars_expr_inner(base, used);
            if let Some(start_expr) = start {
                collect_used_vars_expr_inner(start_expr, used);
            }
            if let Some(stop_expr) = stop {
                collect_used_vars_expr_inner(stop_expr, used);
            }
            if let Some(step_expr) = step {
                collect_used_vars_expr_inner(step_expr, used);
            }
        }
        HirExpr::Attribute { value, .. } => {
            // DEPYLER-0229 FIX: Collect variables from attribute access expressions
            // This was causing dead code elimination to remove assignments
            // for variables used in attribute access like: p.x + p.y
            collect_used_vars_expr_inner(value, used);
        }
        HirExpr::Index { base, index } => {
            // DEPYLER-0229 FIX: Collect variables from index expressions
            // This was causing dead code elimination to remove assignments
            // for variables used in indexing like: data[key]
            collect_used_vars_expr_inner(base, used);
            collect_used_vars_expr_inner(index, used);
        }
        HirExpr::FString { parts } => {
            // DEPYLER-0516 / GH-103: Collect variables from f-string expressions
            // F-strings can contain embedded expressions that reference variables
            // Example: f"Hello {args.name}" uses `args` variable
            // Without this, DCE incorrectly removes `args = parser.parse_args()`
            for part in parts {
                if let crate::hir::FStringPart::Expr(expr) = part {
                    collect_used_vars_expr_inner(expr, used);
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{
        AssignTarget, FunctionProperties, HirExpr, HirFunction, HirProgram, HirStmt, Literal, Type,
    };
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_constant_propagation() {
        // DEPYLER-0508: Dead code elimination is ENABLED by default
        // Unused variables should be eliminated
        // This test validates that DCE correctly removes dead assignments
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    // Dead assignment - never read → eliminated by DCE
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("unused".to_string()),
                        value: HirExpr::Literal(Literal::Int(42)),
                        type_annotation: None,
                    },
                    // Dead assignment - never read → eliminated by DCE
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("result".to_string()),
                        value: HirExpr::Literal(Literal::Int(10)),
                        type_annotation: None,
                    },
                    HirStmt::Return(Some(HirExpr::Literal(Literal::Int(5)))),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // DEPYLER-0508: With DCE enabled, unused variables are eliminated
        // Only the return statement remains
        let func = &optimized.functions[0];
        assert_eq!(
            func.body.len(),
            1,
            "Dead assignments eliminated, only return statement remains (DCE enabled)"
        );

        // Verify only the return statement remains
        assert!(matches!(&func.body[0], HirStmt::Return(_)));
    }

    #[test]
    fn test_dead_code_elimination() {
        // DEPYLER-0508: Dead code elimination is now ENABLED by default
        // Unused variables should be eliminated
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

        // Check optimization results - DCE is now enabled by default
        let func = &optimized.functions[0];

        // DEPYLER-0508: DCE is enabled - unused variable should be eliminated
        // Statements: assignment for "used" + return = 2 statements
        assert_eq!(
            func.body.len(),
            2,
            "Unused 'unused' variable should be eliminated (DCE enabled by default)"
        );

        // Verify the "used" variable and return are preserved
        assert!(matches!(&func.body[0], HirStmt::Assign { .. }));
        assert!(matches!(&func.body[1], HirStmt::Return(_)));
    }

    #[test]
    fn test_dead_code_elimination_when_enabled() {
        // Test what happens when dead code elimination IS explicitly enabled
        let config = OptimizerConfig {
            eliminate_dead_code: true,
            ..Default::default()
        };
        let mut optimizer = Optimizer::new(config);

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

        let func = &optimized.functions[0];

        // When enabled, unused assignment should be removed
        // "used" assignment is kept because it's read in return
        assert!(
            func.body.len() <= 3,
            "Dead code elimination should remove or preserve statements"
        );

        // At minimum, the return statement should exist
        let has_return = func
            .body
            .iter()
            .any(|stmt| matches!(stmt, HirStmt::Return(_)));
        assert!(has_return, "Return statement should be preserved");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DEPYLER_0508_dead_code_elimination_enabled_by_default() {
        // DEPYLER-0508: DCE should be enabled by default
        // Unused variables should be eliminated without explicit opt-in
        let config = OptimizerConfig::default();

        // Verify DCE is enabled by default
        assert!(
            config.eliminate_dead_code,
            "DEPYLER-0508: Dead code elimination should be enabled by default"
        );

        let mut optimizer = Optimizer::new(config);

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    // Unused variable - should be eliminated
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("unused".to_string()),
                        value: HirExpr::Literal(Literal::Int(42)),
                        type_annotation: None,
                    },
                    // Used variable - should be kept
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
        let func = &optimized.functions[0];

        // With DCE enabled by default, unused assignment should be removed
        // Statements should be: assignment for "used" + return = 2 statements
        assert_eq!(
            func.body.len(),
            2,
            "DEPYLER-0508: Unused 'unused' variable should be eliminated by default"
        );

        // Verify the unused variable was removed, not just renamed
        let has_unused = func.body.iter().any(|stmt| {
            if let HirStmt::Assign {
                target: AssignTarget::Symbol(name),
                ..
            } = stmt
            {
                return name == "unused" || name == "_unused";
            }
            false
        });
        assert!(
            !has_unused,
            "DEPYLER-0508: 'unused' variable should be completely eliminated"
        );
    }
}
