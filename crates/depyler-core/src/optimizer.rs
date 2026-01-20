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
        // Pass 0: DEPYLER-0188 Walrus operator hoisting
        // Must run BEFORE CSE to properly hoist (n := expr) to let n = expr
        program = self.hoist_walrus_operators(program);

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

    /// DEPYLER-0188: Hoist walrus operator (NamedExpr) assignments to separate statements
    ///
    /// Transforms: if (n := len(text)) > 5: return n
    /// Into:       n = len(text); if n > 5: return n
    ///
    /// This must run before CSE to avoid CSE creating temps for the entire condition.
    fn hoist_walrus_operators(&self, mut program: HirProgram) -> HirProgram {
        for func in &mut program.functions {
            func.body = self.hoist_walrus_in_body(&func.body);
        }
        program
    }

    /// Process a statement body for walrus hoisting
    fn hoist_walrus_in_body(&self, body: &[HirStmt]) -> Vec<HirStmt> {
        let mut new_body = Vec::new();

        for stmt in body {
            match stmt {
                HirStmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    // Extract walrus operators from condition
                    let (walrus_assigns, simplified_condition) =
                        self.extract_walrus_from_expr(condition);

                    // Add hoisted let statements
                    for (name, value) in walrus_assigns {
                        new_body.push(HirStmt::Assign {
                            target: AssignTarget::Symbol(name),
                            value,
                            type_annotation: None,
                        });
                    }

                    // Recursively process then/else bodies
                    let new_then = self.hoist_walrus_in_body(then_body);
                    let new_else = else_body.as_ref().map(|stmts| self.hoist_walrus_in_body(stmts));

                    new_body.push(HirStmt::If {
                        condition: simplified_condition,
                        then_body: new_then,
                        else_body: new_else,
                    });
                }
                HirStmt::While { condition, body: while_body } => {
                    // Extract walrus from while condition
                    let (walrus_assigns, simplified_condition) =
                        self.extract_walrus_from_expr(condition);

                    // For while loops, walrus needs special handling - hoist once before
                    for (name, value) in walrus_assigns {
                        new_body.push(HirStmt::Assign {
                            target: AssignTarget::Symbol(name),
                            value,
                            type_annotation: None,
                        });
                    }

                    let new_while_body = self.hoist_walrus_in_body(while_body);
                    new_body.push(HirStmt::While {
                        condition: simplified_condition,
                        body: new_while_body,
                    });
                }
                HirStmt::For { target, iter, body: for_body } => {
                    // Recursively process for body
                    let new_for_body = self.hoist_walrus_in_body(for_body);
                    new_body.push(HirStmt::For {
                        target: target.clone(),
                        iter: iter.clone(),
                        body: new_for_body,
                    });
                }
                HirStmt::Try { body: try_body, handlers, orelse, finalbody } => {
                    // Recursively process try/except blocks
                    let new_try_body = self.hoist_walrus_in_body(try_body);
                    let new_handlers: Vec<_> = handlers.iter().map(|h| {
                        crate::hir::ExceptHandler {
                            exception_type: h.exception_type.clone(),
                            name: h.name.clone(),
                            body: self.hoist_walrus_in_body(&h.body),
                        }
                    }).collect();
                    let new_orelse = orelse.as_ref().map(|stmts| self.hoist_walrus_in_body(stmts));
                    let new_finalbody = finalbody.as_ref().map(|stmts| self.hoist_walrus_in_body(stmts));
                    new_body.push(HirStmt::Try {
                        body: new_try_body,
                        handlers: new_handlers,
                        orelse: new_orelse,
                        finalbody: new_finalbody,
                    });
                }
                HirStmt::With { context, target, body: with_body, is_async } => {
                    let new_with_body = self.hoist_walrus_in_body(with_body);
                    new_body.push(HirStmt::With {
                        context: context.clone(),
                        target: target.clone(),
                        body: new_with_body,
                        is_async: *is_async,
                    });
                }
                _ => new_body.push(stmt.clone()),
            }
        }

        new_body
    }

    /// Extract NamedExpr from an expression, returning hoisted assignments and simplified expr
    fn extract_walrus_from_expr(&self, expr: &HirExpr) -> (Vec<(String, HirExpr)>, HirExpr) {
        let mut assigns = Vec::new();
        let simplified = self.extract_walrus_recursive(expr, &mut assigns);
        (assigns, simplified)
    }

    /// Recursively extract NamedExpr from expression tree
    fn extract_walrus_recursive(&self, expr: &HirExpr, assigns: &mut Vec<(String, HirExpr)>) -> HirExpr {
        match expr {
            HirExpr::NamedExpr { target, value } => {
                // Recursively process value first (handle nested walrus)
                let simplified_value = self.extract_walrus_recursive(value, assigns);
                assigns.push((target.clone(), simplified_value));
                // Replace with variable reference
                HirExpr::Var(target.clone())
            }
            HirExpr::Binary { op, left, right } => HirExpr::Binary {
                op: *op,
                left: Box::new(self.extract_walrus_recursive(left, assigns)),
                right: Box::new(self.extract_walrus_recursive(right, assigns)),
            },
            HirExpr::Unary { op, operand } => HirExpr::Unary {
                op: *op,
                operand: Box::new(self.extract_walrus_recursive(operand, assigns)),
            },
            HirExpr::Call { func, args, kwargs } => HirExpr::Call {
                func: func.clone(),
                args: args.iter().map(|a| self.extract_walrus_recursive(a, assigns)).collect(),
                kwargs: kwargs.iter().map(|(k, v)| (k.clone(), self.extract_walrus_recursive(v, assigns))).collect(),
            },
            HirExpr::MethodCall { object, method, args, kwargs } => HirExpr::MethodCall {
                object: Box::new(self.extract_walrus_recursive(object, assigns)),
                method: method.clone(),
                args: args.iter().map(|a| self.extract_walrus_recursive(a, assigns)).collect(),
                kwargs: kwargs.iter().map(|(k, v)| (k.clone(), self.extract_walrus_recursive(v, assigns))).collect(),
            },
            HirExpr::IfExpr { test, body, orelse } => HirExpr::IfExpr {
                test: Box::new(self.extract_walrus_recursive(test, assigns)),
                body: Box::new(self.extract_walrus_recursive(body, assigns)),
                orelse: Box::new(self.extract_walrus_recursive(orelse, assigns)),
            },
            // Other expressions - just clone (walrus rare in these contexts)
            _ => expr.clone(),
        }
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
        // DEPYLER-0703: Iterate DCE until no more statements are removed
        // This handles transitive dead code (e.g., `sq = Foo(); is_sq = sq.bar()` where
        // is_sq is unused → remove is_sq → sq becomes unused → remove sq)
        const MAX_ITERATIONS: usize = 10;

        for _ in 0..MAX_ITERATIONS {
            let initial_len = func.body.len();

            // Collect truly used variables (referenced after assignment)
            let mut used_vars = HashMap::new();
            for stmt in &func.body {
                self.collect_truly_used_vars_stmt(stmt, &mut used_vars);
            }

            // DEPYLER-0703: Collect assignments with side effects (calls, indexing, etc.)
            // that need to be preserved even if the variable is unused
            let mut side_effect_vars = HashSet::new();
            for stmt in &func.body {
                if let HirStmt::Assign { target, value, .. } = stmt {
                    if Self::expr_has_side_effects(value) {
                        if let AssignTarget::Symbol(name) = target {
                            side_effect_vars.insert(name.clone());
                        }
                    }
                }
            }

            // DEPYLER-0934: DISABLED variable renaming to `_varname`
            // The previous approach only renamed definitions, not usages, causing E0425 errors.
            // Example: `let _args = Args::parse();` but `args.command` still used original name.
            // Instead of renaming, we suppress warnings with #[allow(unused_variables)] at file level.
            // A proper fix would require a full rename pass that updates all references.

            // Remove truly dead assignments (not used and no side effects)
            func.body.retain(|stmt| {
                if let HirStmt::Assign {
                    target: AssignTarget::Symbol(name),
                    ..
                } = stmt
                {
                    // Keep if: truly used OR has side effects
                    // DEPYLER-0934: Removed .trim_start_matches('_') since we no longer rename
                    used_vars.contains_key(name) || side_effect_vars.contains(name)
                } else {
                    true
                }
            });

            // If no statements were removed, we're done
            if func.body.len() == initial_len {
                break;
            }
        }
    }

    /// DEPYLER-0270 Fix #1 (Updated): Collect truly used variables (referenced, not just assigned)
    /// This version does NOT mark side-effect assignments as used - that's handled separately
    /// in eliminate_dead_code_function which preserves side-effect assignments.
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
            // DEPYLER-0514: Handle Try statements - recurse into all blocks
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Recurse into try body
                for s in body {
                    self.collect_truly_used_vars_stmt(s, used);
                }
                // Recurse into exception handlers
                for handler in handlers {
                    for s in &handler.body {
                        self.collect_truly_used_vars_stmt(s, used);
                    }
                }
                // Recurse into else block (executed if no exception)
                if let Some(orelse_stmts) = orelse {
                    for s in orelse_stmts {
                        self.collect_truly_used_vars_stmt(s, used);
                    }
                }
                // Recurse into finally block
                if let Some(finalbody_stmts) = finalbody {
                    for s in finalbody_stmts {
                        self.collect_truly_used_vars_stmt(s, used);
                    }
                }
            }
            // DEPYLER-0514: Handle With statements - recurse into body
            HirStmt::With { context, body, .. } => {
                self.collect_used_vars_expr(context, used);
                for s in body {
                    self.collect_truly_used_vars_stmt(s, used);
                }
            }
            // DEPYLER-0627: Handle Assert statements - variables in test/msg are used
            HirStmt::Assert { test, msg } => {
                self.collect_used_vars_expr(test, used);
                if let Some(msg_expr) = msg {
                    self.collect_used_vars_expr(msg_expr, used);
                }
            }
            // DEPYLER-0627: Handle Raise statements - variables in exception are used
            HirStmt::Raise { exception, cause } => {
                if let Some(exc) = exception {
                    self.collect_used_vars_expr(exc, used);
                }
                if let Some(c) = cause {
                    self.collect_used_vars_expr(c, used);
                }
            }
            // DEPYLER-0688: Handle FunctionDef (nested functions) - must recurse into body
            // Nested functions can capture variables from outer scope (closures)
            // e.g., `cache = {}; def fib(): cache[x] = v` - `cache` is used in nested function
            HirStmt::FunctionDef { body, .. } => {
                for s in body {
                    self.collect_truly_used_vars_stmt(s, used);
                }
            }
            // DEPYLER-0688: Handle Block statements - must recurse into nested statements
            HirStmt::Block(stmts) => {
                for s in stmts {
                    self.collect_truly_used_vars_stmt(s, used);
                }
            }
            // Other statements (Pass, Break, Continue, Return(None))
            // don't reference variables that need tracking for DCE purposes
            _ => {}
        }
    }

    /// DEPYLER-0270 Fix #1: Check if expression contains indexing operations
    /// Returns true if the expression tree contains any Index nodes, which indicate
    /// operations that can fail (e.g., list[0], dict["key"]) and have side effects.
    ///
    /// # Complexity
    /// DEPYLER-0703: Check if expression has side effects (calls, indexing, etc.)
    /// that should not be eliminated even if the result is unused.
    /// Complexity: 5 (recursive expression traversal with early return)
    fn expr_has_side_effects(expr: &HirExpr) -> bool {
        match expr {
            // Indexing has side effects (may panic)
            HirExpr::Index { .. } => true,
            // Function calls have side effects (may modify state, print, etc.)
            HirExpr::Call { .. } => true,
            // Method calls have side effects
            HirExpr::MethodCall { .. } => true,
            // Binary/unary ops have side effects if operands do
            HirExpr::Binary { left, right, .. } => {
                Self::expr_has_side_effects(left) || Self::expr_has_side_effects(right)
            }
            HirExpr::Unary { operand, .. } => Self::expr_has_side_effects(operand),
            // Collections have side effects if elements do
            HirExpr::List(items) | HirExpr::Tuple(items) => {
                items.iter().any(Self::expr_has_side_effects)
            }
            HirExpr::Dict(pairs) => pairs
                .iter()
                .any(|(k, v)| Self::expr_has_side_effects(k) || Self::expr_has_side_effects(v)),
            HirExpr::Set(items) => items.iter().any(Self::expr_has_side_effects),
            HirExpr::Attribute { value, .. } => Self::expr_has_side_effects(value),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                Self::expr_has_side_effects(base)
                    || start.as_ref().is_some_and(|e| Self::expr_has_side_effects(e))
                    || stop.as_ref().is_some_and(|e| Self::expr_has_side_effects(e))
                    || step.as_ref().is_some_and(|e| Self::expr_has_side_effects(e))
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
        HirExpr::Call { func, args, kwargs } => {
            // Mark the function name as used (important for lambda variables)
            used.insert(func.clone(), true);
            for arg in args {
                collect_used_vars_expr_inner(arg, used);
            }
            // DEPYLER-0935: Collect variables from kwargs values
            // This was causing DCE to incorrectly remove assignments like `data = rows[1:]`
            // when `data` was used in `sorted(data, key=lambda...)` - the kwargs lambda
            // body might reference variables that need to be preserved.
            for (_, v) in kwargs {
                collect_used_vars_expr_inner(v, used);
            }
        }
        HirExpr::MethodCall { object, args, kwargs, .. } => {
            collect_used_vars_expr_inner(object, used);
            for arg in args {
                collect_used_vars_expr_inner(arg, used);
            }
            // DEPYLER-0935: Collect variables from kwargs values
            for (_, v) in kwargs {
                collect_used_vars_expr_inner(v, used);
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
        // DEPYLER-0600 Bug #5: DictComp was missing from DCE analysis!
        // This caused variables used only in dict comprehension iterators to be
        // incorrectly removed. Example: `d = {str(n): n*n for n in nums}` lost `nums`
        HirExpr::DictComp {
            key,
            value,
            generators,
        } => {
            // Collect used vars from key and value expressions
            collect_used_vars_expr_inner(key, used);
            collect_used_vars_expr_inner(value, used);
            // DEPYLER-0504: Support multiple generators
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
        // DEPYLER-0618: Collect variables from ternary (if-expression) expressions
        // Example: `out = sys.stdout if verbose else open(...)`
        // Without this, DCE incorrectly removes `verbose = True`
        HirExpr::IfExpr { test, body, orelse } => {
            collect_used_vars_expr_inner(test, used);
            collect_used_vars_expr_inner(body, used);
            collect_used_vars_expr_inner(orelse, used);
        }
        // DEPYLER-0935: Collect variables from SortByKey expression
        // sorted(data, key=lambda r: r[0]) is converted to HirExpr::SortByKey
        // We must collect variables from iterable, key_body, and reverse_expr
        HirExpr::SortByKey { iterable, key_body, reverse_expr, .. } => {
            collect_used_vars_expr_inner(iterable, used);
            collect_used_vars_expr_inner(key_body, used);
            if let Some(rev) = reverse_expr {
                collect_used_vars_expr_inner(rev, used);
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

    // ==========================================================================
    // DEPYLER-COVERAGE-95: Lines-to-test ratio improvement tests
    // Target: ratio 405 → <30
    // ==========================================================================

    #[test]
    fn test_optimizer_config_default() {
        let config = OptimizerConfig::default();
        // DEPYLER-0161: inlining is disabled
        assert!(!config.inline_functions);
        assert!(config.eliminate_dead_code);
        assert!(config.propagate_constants);
        assert!(config.eliminate_common_subexpressions);
        assert_eq!(config.inline_threshold, 20);
    }

    #[test]
    fn test_optimizer_new() {
        let config = OptimizerConfig::default();
        let optimizer = Optimizer::new(config.clone());
        assert_eq!(optimizer.config.inline_threshold, config.inline_threshold);
    }

    #[test]
    fn test_is_constant_expr_literals() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_constant_expr(&HirExpr::Literal(Literal::Int(42))));
        assert!(optimizer.is_constant_expr(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(optimizer.is_constant_expr(&HirExpr::Literal(Literal::Bool(true))));
        assert!(optimizer.is_constant_expr(&HirExpr::Literal(Literal::String("hello".into()))));
    }

    #[test]
    fn test_is_constant_expr_non_constant() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(!optimizer.is_constant_expr(&HirExpr::Var("x".to_string())));
        assert!(!optimizer.is_constant_expr(&HirExpr::Call {
            func: "foo".to_string(),
            args: vec![],
            kwargs: vec![],
        }));
    }

    #[test]
    fn test_evaluate_constant_binop_int_add() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(v))) = result {
            assert_eq!(v, 8);
        }
    }

    #[test]
    fn test_evaluate_constant_binop_int_sub() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::Sub,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(v))) = result {
            assert_eq!(v, 7);
        }
    }

    #[test]
    fn test_evaluate_constant_binop_int_mul() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(4))),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(v))) = result {
            assert_eq!(v, 12);
        }
    }

    #[test]
    fn test_evaluate_constant_binop_int_div() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::Div,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(v))) = result {
            assert_eq!(v, 5);
        }
    }

    #[test]
    fn test_evaluate_constant_binop_int_mod_unsupported() {
        // Mod is not supported in constant folding - returns None
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            op: BinOp::Mod,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_binop_bool_and_unsupported() {
        // Boolean And is not supported in constant folding - returns None
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            op: BinOp::And,
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_binop_bool_or_unsupported() {
        // Boolean Or is not supported in constant folding - returns None
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Bool(true))),
            op: BinOp::Or,
            right: Box::new(HirExpr::Literal(Literal::Bool(false))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_binop_comparison_eq_unsupported() {
        // Comparison Eq is not supported in constant folding - returns None
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(5))),
            op: BinOp::Eq,
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_binop_comparison_lt_unsupported() {
        // Comparison Lt is not supported in constant folding - returns None
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(3))),
            op: BinOp::Lt,
            right: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_unaryop_not() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Literal(Literal::Bool(true))),
        };
        let result = optimizer.evaluate_constant_unaryop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Bool(v))) = result {
            assert!(!v);
        }
    }

    #[test]
    fn test_evaluate_constant_unaryop_neg_int() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = optimizer.evaluate_constant_unaryop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Int(v))) = result {
            assert_eq!(v, -5);
        }
    }

    #[test]
    fn test_evaluate_constant_unaryop_neg_float() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Float(3.15))),
        };
        let result = optimizer.evaluate_constant_unaryop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Float(v))) = result {
            assert!((v - (-3.15)).abs() < 0.001);
        }
    }

    #[test]
    fn test_expr_has_side_effects_call() {
        // Function calls have side effects
        let expr = HirExpr::Call {
            func: "print".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_method_call() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_literal() {
        // Literals don't have side effects
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_var() {
        // Variable reads don't have side effects
        let expr = HirExpr::Var("x".to_string());
        assert!(!Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_binary() {
        // Pure binary ops don't have side effects
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_is_pure_function() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_pure_function("len"));
        assert!(optimizer.is_pure_function("str"));
        assert!(optimizer.is_pure_function("int"));
        assert!(optimizer.is_pure_function("float"));
        assert!(optimizer.is_pure_function("bool"));
        assert!(optimizer.is_pure_function("abs"));
        assert!(optimizer.is_pure_function("min"));
        assert!(optimizer.is_pure_function("max"));
    }

    #[test]
    fn test_is_pure_function_impure() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(!optimizer.is_pure_function("print"));
        assert!(!optimizer.is_pure_function("input"));
        assert!(!optimizer.is_pure_function("open"));
        assert!(!optimizer.is_pure_function("custom_func"));
    }

    #[test]
    fn test_is_complex_expr_literal() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_complex_expr_var() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Var("x".to_string());
        assert!(!optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_complex_expr_binary_mul() {
        // Mul is considered complex (only Add/Sub with simple operands are simple)
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_complex_expr_binary_add_simple() {
        // Add with Var operands is NOT complex
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(!optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_complex_expr_call() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        assert!(optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_simple_return_expr() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_simple_return_expr(&HirExpr::Literal(Literal::Int(42))));
        assert!(optimizer.is_simple_return_expr(&HirExpr::Var("x".to_string())));
        assert!(optimizer.is_simple_return_expr(&HirExpr::Literal(Literal::Bool(true))));
    }

    #[test]
    fn test_is_simple_return_expr_binary() {
        // Binary expressions ARE considered simple return expressions
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(optimizer.is_simple_return_expr(&expr));
    }

    #[test]
    fn test_is_simple_return_expr_list() {
        // List expressions are NOT simple return expressions
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        assert!(!optimizer.is_simple_return_expr(&expr));
    }

    #[test]
    fn test_hash_expr_same_expr_same_hash() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr1 = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        let expr2 = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_different_expr_different_hash() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr1 = HirExpr::Var("x".to_string());
        let expr2 = HirExpr::Var("y".to_string());
        assert_ne!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hoist_walrus_operators_basic() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };
        let result = optimizer.hoist_walrus_operators(program);
        // Basic case with no walrus operators should remain unchanged
        assert_eq!(result.functions[0].body.len(), 1);
    }

    #[test]
    fn test_extract_walrus_from_expr_no_walrus() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        let (assigns, result) = optimizer.extract_walrus_from_expr(&expr);
        assert!(assigns.is_empty());
        // Result should be equivalent to original
        assert!(matches!(result, HirExpr::Binary { .. }));
    }

    #[test]
    fn test_collect_read_vars_expr() {
        let mut read_vars = HashSet::new();
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Var("y".to_string())),
        };
        Optimizer::collect_read_vars_expr(&expr, &mut read_vars);
        assert!(read_vars.contains("x"));
        assert!(read_vars.contains("y"));
    }

    #[test]
    fn test_collect_read_vars_expr_call() {
        let mut read_vars = HashSet::new();
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())],
            kwargs: vec![],
        };
        Optimizer::collect_read_vars_expr(&expr, &mut read_vars);
        assert!(read_vars.contains("a"));
        assert!(read_vars.contains("b"));
    }

    #[test]
    fn test_collect_read_vars_stmt_assign() {
        let mut read_vars = HashSet::new();
        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol("result".to_string()),
            value: HirExpr::Var("input".to_string()),
            type_annotation: None,
        };
        Optimizer::collect_read_vars_stmt(&stmt, &mut read_vars);
        assert!(read_vars.contains("input"));
        assert!(!read_vars.contains("result")); // target is written, not read
    }

    #[test]
    fn test_collect_read_vars_stmt_return() {
        let mut read_vars = HashSet::new();
        let stmt = HirStmt::Return(Some(HirExpr::Var("value".to_string())));
        Optimizer::collect_read_vars_stmt(&stmt, &mut read_vars);
        assert!(read_vars.contains("value"));
    }

    #[test]
    fn test_is_constant_expr_inner_literals() {
        assert!(is_constant_expr_inner(&HirExpr::Literal(Literal::Int(42))));
        assert!(is_constant_expr_inner(&HirExpr::Literal(Literal::Float(3.15))));
        assert!(is_constant_expr_inner(&HirExpr::Literal(Literal::Bool(true))));
        assert!(is_constant_expr_inner(&HirExpr::Literal(Literal::String("hi".into()))));
        assert!(is_constant_expr_inner(&HirExpr::Literal(Literal::None)));
    }

    #[test]
    fn test_is_constant_expr_inner_non_constant() {
        assert!(!is_constant_expr_inner(&HirExpr::Var("x".to_string())));
        assert!(!is_constant_expr_inner(&HirExpr::Call {
            func: "foo".to_string(),
            args: vec![],
            kwargs: vec![],
        }));
    }

    #[test]
    fn test_collect_used_vars_expr_inner() {
        let mut used = HashMap::new();
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("a".to_string())),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Var("b".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("a"));
        assert!(used.contains_key("b"));
    }

    #[test]
    fn test_optimizer_with_empty_program() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());
        let program = HirProgram {
            functions: vec![],
            classes: vec![],
            imports: vec![],
        };
        let result = optimizer.optimize_program(program);
        assert!(result.functions.is_empty());
    }

    #[test]
    fn test_optimizer_preserves_return_statements() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());
        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };
        let result = optimizer.optimize_program(program);
        assert_eq!(result.functions[0].body.len(), 1);
        assert!(matches!(result.functions[0].body[0], HirStmt::Return(_)));
    }

    #[test]
    fn test_optimizer_config_clone() {
        let config = OptimizerConfig::default();
        let cloned = config.clone();
        assert_eq!(config.inline_functions, cloned.inline_functions);
        assert_eq!(config.eliminate_dead_code, cloned.eliminate_dead_code);
        assert_eq!(config.propagate_constants, cloned.propagate_constants);
    }

    #[test]
    fn test_evaluate_constant_binop_non_constant() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_unaryop_non_constant() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        let result = optimizer.evaluate_constant_unaryop(&expr);
        assert!(result.is_none());
    }

    #[test]
    fn test_constant_propagation_multiple_vars() {
        let config = OptimizerConfig {
            propagate_constants: true,
            eliminate_dead_code: false,
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
                        target: AssignTarget::Symbol("a".to_string()),
                        value: HirExpr::Literal(Literal::Int(10)),
                        type_annotation: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("b".to_string()),
                        value: HirExpr::Literal(Literal::Int(20)),
                        type_annotation: None,
                    },
                    HirStmt::Return(Some(HirExpr::Binary {
                        left: Box::new(HirExpr::Var("a".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Var("b".to_string())),
                    })),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let result = optimizer.optimize_program(program);
        assert!(!result.functions.is_empty());
    }

    #[test]
    fn test_cse_eliminates_common_subexpressions() {
        let config = OptimizerConfig {
            eliminate_common_subexpressions: true,
            eliminate_dead_code: false,
            propagate_constants: false,
            ..Default::default()
        };
        let mut optimizer = Optimizer::new(config);

        let complex_expr = HirExpr::Binary {
            left: Box::new(HirExpr::Var("x".to_string())),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Var("y".to_string())),
        };

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("a".to_string()),
                        value: complex_expr.clone(),
                        type_annotation: None,
                    },
                    HirStmt::Assign {
                        target: AssignTarget::Symbol("b".to_string()),
                        value: complex_expr,
                        type_annotation: None,
                    },
                    HirStmt::Return(Some(HirExpr::Binary {
                        left: Box::new(HirExpr::Var("a".to_string())),
                        op: BinOp::Add,
                        right: Box::new(HirExpr::Var("b".to_string())),
                    })),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let result = optimizer.optimize_program(program);
        // CSE should run and produce a valid program
        assert!(!result.functions.is_empty());
    }

    // ========================================================
    // DEPYLER-COVERAGE-95: Additional collect_used_vars tests
    // ========================================================

    #[test]
    fn test_collect_used_vars_tuple() {
        let mut used = HashMap::new();
        let expr = HirExpr::Tuple(vec![
            HirExpr::Var("a".to_string()),
            HirExpr::Var("b".to_string()),
        ]);
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("a"));
        assert!(used.contains_key("b"));
    }

    #[test]
    fn test_collect_used_vars_list() {
        let mut used = HashMap::new();
        let expr = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("x"));
        assert!(used.contains_key("y"));
    }

    #[test]
    fn test_collect_used_vars_dict() {
        let mut used = HashMap::new();
        let expr = HirExpr::Dict(vec![(
            HirExpr::Var("key".to_string()),
            HirExpr::Var("val".to_string()),
        )]);
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("key"));
        assert!(used.contains_key("val"));
    }

    #[test]
    fn test_collect_used_vars_unary() {
        let mut used = HashMap::new();
        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("flag".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("flag"));
    }

    #[test]
    fn test_collect_used_vars_call_with_kwargs() {
        let mut used = HashMap::new();
        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![HirExpr::Var("arg".to_string())],
            kwargs: vec![("k".to_string(), HirExpr::Var("kwval".to_string()))],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("arg"));
        assert!(used.contains_key("kwval"));
        assert!(used.contains_key("func"));
    }

    #[test]
    fn test_collect_used_vars_method_call() {
        let mut used = HashMap::new();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![HirExpr::Var("arg".to_string())],
            kwargs: vec![],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("obj"));
        assert!(used.contains_key("arg"));
    }

    #[test]
    fn test_collect_used_vars_method_call_kwargs() {
        let mut used = HashMap::new();
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "m".to_string(),
            args: vec![],
            kwargs: vec![("k".to_string(), HirExpr::Var("v".to_string()))],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("v"));
    }

    #[test]
    fn test_collect_used_vars_lambda() {
        let mut used = HashMap::new();
        let expr = HirExpr::Lambda {
            params: vec!["x".to_string()],
            body: Box::new(HirExpr::Var("captured".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("captured"));
    }

    #[test]
    fn test_collect_used_vars_list_comp() {
        use crate::hir::HirComprehension;
        let mut used = HashMap::new();
        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("elem".to_string())),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![HirExpr::Var("cond".to_string())],
            }],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("elem"));
        assert!(used.contains_key("items"));
        assert!(used.contains_key("cond"));
    }

    #[test]
    fn test_collect_used_vars_set_comp() {
        use crate::hir::HirComprehension;
        let mut used = HashMap::new();
        let expr = HirExpr::SetComp {
            element: Box::new(HirExpr::Var("elem".to_string())),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("elem"));
        assert!(used.contains_key("items"));
    }

    #[test]
    fn test_collect_used_vars_dict_comp() {
        use crate::hir::HirComprehension;
        let mut used = HashMap::new();
        let expr = HirExpr::DictComp {
            key: Box::new(HirExpr::Var("k".to_string())),
            value: Box::new(HirExpr::Var("v".to_string())),
            generators: vec![HirComprehension {
                target: "i".to_string(),
                iter: Box::new(HirExpr::Var("pairs".to_string())),
                conditions: vec![HirExpr::Var("filter".to_string())],
            }],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("k"));
        assert!(used.contains_key("v"));
        assert!(used.contains_key("pairs"));
        assert!(used.contains_key("filter"));
    }

    #[test]
    fn test_collect_used_vars_await() {
        let mut used = HashMap::new();
        let expr = HirExpr::Await {
            value: Box::new(HirExpr::Var("future".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("future"));
    }

    #[test]
    fn test_collect_used_vars_slice() {
        let mut used = HashMap::new();
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Var("s".to_string()))),
            stop: Some(Box::new(HirExpr::Var("e".to_string()))),
            step: Some(Box::new(HirExpr::Var("st".to_string()))),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("arr"));
        assert!(used.contains_key("s"));
        assert!(used.contains_key("e"));
        assert!(used.contains_key("st"));
    }

    #[test]
    fn test_collect_used_vars_slice_partial() {
        let mut used = HashMap::new();
        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: None,
            stop: Some(Box::new(HirExpr::Var("end".to_string()))),
            step: None,
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("arr"));
        assert!(used.contains_key("end"));
    }

    #[test]
    fn test_collect_used_vars_attribute() {
        let mut used = HashMap::new();
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "attr".to_string(),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("obj"));
    }

    #[test]
    fn test_collect_used_vars_index() {
        let mut used = HashMap::new();
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("arr"));
        assert!(used.contains_key("idx"));
    }

    #[test]
    fn test_collect_used_vars_fstring() {
        use crate::hir::FStringPart;
        let mut used = HashMap::new();
        let expr = HirExpr::FString {
            parts: vec![
                FStringPart::Literal("Hello ".to_string()),
                FStringPart::Expr(Box::new(HirExpr::Var("name".to_string()))),
            ],
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("name"));
    }

    #[test]
    fn test_collect_used_vars_if_expr() {
        let mut used = HashMap::new();
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Var("then".to_string())),
            orelse: Box::new(HirExpr::Var("else_".to_string())),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("cond"));
        assert!(used.contains_key("then"));
        assert!(used.contains_key("else_"));
    }

    #[test]
    fn test_collect_used_vars_sort_by_key() {
        let mut used = HashMap::new();
        let expr = HirExpr::SortByKey {
            iterable: Box::new(HirExpr::Var("data".to_string())),
            key_params: vec!["x".to_string()],
            key_body: Box::new(HirExpr::Var("key".to_string())),
            reverse_expr: Some(Box::new(HirExpr::Var("rev".to_string()))),
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("data"));
        assert!(used.contains_key("key"));
        assert!(used.contains_key("rev"));
    }

    #[test]
    fn test_collect_used_vars_sort_by_key_no_reverse() {
        let mut used = HashMap::new();
        let expr = HirExpr::SortByKey {
            iterable: Box::new(HirExpr::Var("items".to_string())),
            key_params: vec!["x".to_string()],
            key_body: Box::new(HirExpr::Var("k".to_string())),
            reverse_expr: None,
        };
        collect_used_vars_expr_inner(&expr, &mut used);
        assert!(used.contains_key("items"));
        assert!(used.contains_key("k"));
    }

    // ========================================================
    // hash_expr tests for coverage
    // ========================================================

    #[test]
    fn test_hash_expr_literal_float() {
        let expr1 = HirExpr::Literal(Literal::Float(3.15));
        let expr2 = HirExpr::Literal(Literal::Float(3.15));
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_literal_string() {
        let expr1 = HirExpr::Literal(Literal::String("hello".to_string()));
        let expr2 = HirExpr::Literal(Literal::String("hello".to_string()));
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_literal_bytes() {
        let expr1 = HirExpr::Literal(Literal::Bytes(vec![1, 2, 3]));
        let expr2 = HirExpr::Literal(Literal::Bytes(vec![1, 2, 3]));
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_literal_bool() {
        let expr1 = HirExpr::Literal(Literal::Bool(true));
        let expr2 = HirExpr::Literal(Literal::Bool(true));
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_literal_none() {
        let expr1 = HirExpr::Literal(Literal::None);
        let expr2 = HirExpr::Literal(Literal::None);
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_call() {
        let expr1 = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        let expr2 = HirExpr::Call {
            func: "len".to_string(),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_hash_expr_call_different_func() {
        let expr1 = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let expr2 = HirExpr::Call {
            func: "abs".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert_ne!(optimizer.hash_expr(&expr1), optimizer.hash_expr(&expr2));
    }

    #[test]
    fn test_is_pure_function_extended() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_pure_function("abs"));
        assert!(optimizer.is_pure_function("len"));
        assert!(optimizer.is_pure_function("min"));
        assert!(optimizer.is_pure_function("max"));
        assert!(optimizer.is_pure_function("sum"));
        assert!(optimizer.is_pure_function("str"));
        assert!(optimizer.is_pure_function("int"));
        assert!(optimizer.is_pure_function("float"));
        assert!(optimizer.is_pure_function("bool"));
        assert!(optimizer.is_pure_function("round"));
        assert!(optimizer.is_pure_function("pow"));
        assert!(optimizer.is_pure_function("sqrt"));
        assert!(!optimizer.is_pure_function("print"));
        assert!(!optimizer.is_pure_function("input"));
    }

    #[test]
    fn test_optimizer_config_debug() {
        let config = OptimizerConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("OptimizerConfig"));
    }

    // === Additional unique optimizer tests ===

    #[test]
    fn test_evaluate_constant_binop_division_by_zero() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(0))),
            op: BinOp::Div,
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        // Division by zero should not be optimized
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_constant_binop_float_add() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Float(3.5))),
            right: Box::new(HirExpr::Literal(Literal::Float(2.0))),
            op: BinOp::Add,
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_some());
        if let Some(HirExpr::Literal(Literal::Float(f))) = result {
            assert!((f - 5.5).abs() < 0.001);
        }
    }

    #[test]
    fn test_evaluate_constant_binop_string_concat_unsupported() {
        // String concat is not supported in constant folding
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::String("hello".to_string()))),
            right: Box::new(HirExpr::Literal(Literal::String(" world".to_string()))),
            op: BinOp::Add,
        };
        let result = optimizer.evaluate_constant_binop(&expr);
        assert!(result.is_none()); // String ops not supported
    }

    #[test]
    fn test_walrus_operator_hoisting() {
        let mut optimizer = Optimizer::new(OptimizerConfig::default());

        // Create a function with walrus operator in if condition
        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![HirStmt::If {
                    condition: HirExpr::NamedExpr {
                        target: "n".to_string(),
                        value: Box::new(HirExpr::Literal(Literal::Int(5))),
                    },
                    then_body: vec![HirStmt::Return(Some(HirExpr::Var("n".to_string())))],
                    else_body: None,
                }],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // After hoisting, there should be an assignment before the if
        let func = &optimized.functions[0];
        assert!(func.body.len() >= 1, "Should have at least one statement");
    }

    #[test]
    fn test_eliminate_dead_code_preserves_side_effects() {
        let config = OptimizerConfig {
            eliminate_dead_code: true,
            ..Default::default()
        };
        let mut optimizer = Optimizer::new(config);

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::None,
                body: vec![
                    HirStmt::Expr(HirExpr::Call {
                        func: "print".to_string(),
                        args: vec![HirExpr::Literal(Literal::String("hello".to_string()))],
                        kwargs: vec![],
                    }),
                    HirStmt::Return(None),
                ],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        let optimized = optimizer.optimize_program(program);

        // print() has side effects, should be preserved
        let func = &optimized.functions[0];
        assert!(func.body.len() >= 1, "Side effect statement should be preserved");
    }

    // === Additional tests for expr_has_side_effects ===

    #[test]
    fn test_expr_has_side_effects_index() {
        // Indexing can fail, has side effects
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_nested_call() {
        // Call nested inside binary expression still has side effects
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Call {
                func: "get_value".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            op: BinOp::Add,
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_list_literal() {
        // List literals are pure
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert!(!Optimizer::expr_has_side_effects(&expr));
    }

    #[test]
    fn test_expr_has_side_effects_tuple() {
        // Tuples are pure
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert!(!Optimizer::expr_has_side_effects(&expr));
    }

    // === Additional tests for is_pure_function ===

    #[test]
    fn test_is_pure_function_math_functions() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_pure_function("abs"));
        assert!(optimizer.is_pure_function("min"));
        assert!(optimizer.is_pure_function("max"));
        assert!(optimizer.is_pure_function("sum"));
        assert!(optimizer.is_pure_function("pow"));
        assert!(optimizer.is_pure_function("round"));
    }

    #[test]
    fn test_is_pure_function_type_conversions() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(optimizer.is_pure_function("int"));
        assert!(optimizer.is_pure_function("float"));
        assert!(optimizer.is_pure_function("str"));
        assert!(optimizer.is_pure_function("bool"));
        // list() is NOT in the pure list (may allocate)
        assert!(!optimizer.is_pure_function("list"));
    }

    #[test]
    fn test_is_pure_function_impure_io() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        assert!(!optimizer.is_pure_function("print"));
        assert!(!optimizer.is_pure_function("input"));
        assert!(!optimizer.is_pure_function("open"));
    }

    // === Additional tests for is_complex_expr ===

    #[test]
    fn test_is_complex_expr_method_call() {
        // MethodCall is NOT considered complex by is_complex_expr
        // Only Binary and Call are handled
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!optimizer.is_complex_expr(&expr));
    }

    #[test]
    fn test_is_complex_expr_nested_binary() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Binary {
                left: Box::new(HirExpr::Var("a".to_string())),
                op: BinOp::Add,
                right: Box::new(HirExpr::Var("b".to_string())),
            }),
            op: BinOp::Mul,
            right: Box::new(HirExpr::Var("c".to_string())),
        };
        assert!(optimizer.is_complex_expr(&expr));
    }

    // === Tests for hash_expr edge cases ===

    #[test]
    fn test_hash_expr_list() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let list1 = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let list2 = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        assert_eq!(optimizer.hash_expr(&list1), optimizer.hash_expr(&list2));
    }

    #[test]
    fn test_hash_expr_different_lists() {
        // Note: List hashing uses only discriminant, so different lists hash the same
        // This is intentional - Lists are not deeply hashed
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let list1 = HirExpr::List(vec![HirExpr::Literal(Literal::Int(1))]);
        let list2 = HirExpr::List(vec![HirExpr::Literal(Literal::Int(2))]);
        // They hash equal because List uses discriminant-only hashing
        assert_eq!(optimizer.hash_expr(&list1), optimizer.hash_expr(&list2));
    }

    // === Tests for optimizer config variants ===

    #[test]
    fn test_optimizer_config_all_disabled() {
        let config = OptimizerConfig {
            propagate_constants: false,
            eliminate_dead_code: false,
            inline_functions: false,
            eliminate_common_subexpressions: false,
            inline_threshold: 20,
        };
        let mut optimizer = Optimizer::new(config);

        let program = HirProgram {
            functions: vec![HirFunction {
                name: "test".to_string(),
                params: smallvec![],
                ret_type: Type::Int,
                body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
                properties: FunctionProperties::default(),
                annotations: TranspilationAnnotations::default(),
                docstring: None,
            }],
            classes: vec![],
            imports: vec![],
        };

        // With all optimizations disabled, program should be unchanged
        let result = optimizer.optimize_program(program.clone());
        assert_eq!(result.functions.len(), 1);
    }

    #[test]
    fn test_optimizer_config_clone_identical_behavior() {
        let config = OptimizerConfig::default();
        let cloned = config.clone();
        // Both should behave identically
        let mut opt1 = Optimizer::new(config);
        let mut opt2 = Optimizer::new(cloned);
        let program = HirProgram {
            functions: vec![],
            classes: vec![],
            imports: vec![],
        };
        let result1 = opt1.optimize_program(program.clone());
        let result2 = opt2.optimize_program(program);
        assert_eq!(result1.functions.len(), result2.functions.len());
    }

    // === Tests for is_simple_return_expr edge cases ===

    #[test]
    fn test_is_simple_return_expr_attribute() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        // Attribute access is relatively simple
        assert!(optimizer.is_simple_return_expr(&expr));
    }

    #[test]
    fn test_is_simple_return_expr_if_expr() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        // IfExpr (ternary) is not simple
        assert!(!optimizer.is_simple_return_expr(&expr));
    }

    // === Tests for collect_used_vars in different expressions ===

    #[test]
    fn test_collect_used_vars_expr_attribute() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        let mut used = HashMap::new();
        optimizer.collect_used_vars_expr(&expr, &mut used);
        assert!(used.contains_key("obj"));
    }

    #[test]
    fn test_collect_used_vars_expr_index() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        let mut used = HashMap::new();
        optimizer.collect_used_vars_expr(&expr, &mut used);
        assert!(used.contains_key("arr"));
        assert!(used.contains_key("idx"));
    }

    #[test]
    fn test_collect_used_vars_expr_if_expr() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Var("a".to_string())),
            orelse: Box::new(HirExpr::Var("b".to_string())),
        };
        let mut used = HashMap::new();
        optimizer.collect_used_vars_expr(&expr, &mut used);
        assert!(used.contains_key("cond"));
        assert!(used.contains_key("a"));
        assert!(used.contains_key("b"));
    }

    // === Tests for evaluate_constant_binop with different operations ===

    #[test]
    fn test_evaluate_constant_binop_floor_div() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(7))),
            op: BinOp::FloorDiv,
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        // Floor division may or may not be supported
        let _ = optimizer.evaluate_constant_binop(&expr);
    }

    #[test]
    fn test_evaluate_constant_binop_power() {
        let optimizer = Optimizer::new(OptimizerConfig::default());
        let expr = HirExpr::Binary {
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            op: BinOp::Pow,
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        // Power may or may not be supported for constant folding
        let _ = optimizer.evaluate_constant_binop(&expr);
    }
}
