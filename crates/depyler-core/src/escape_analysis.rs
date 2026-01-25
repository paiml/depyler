//! # Escape Analysis for Ownership Inference
//!
//! DEPYLER-PHASE2: Ownership Inference Engine - 80% Single-Shot Compile
//!
//! This module implements interprocedural escape analysis to detect:
//! - Use-after-move violations (~60% of ownership errors)
//! - Aliasing requiring strategic cloning (~15% of ownership errors)
//! - Mutability requirements (~5% of ownership errors)
//!
//! ## Architecture
//!
//! ```text
//! Python Code
//!     │
//!     ▼
//! ┌─────────────────┐
//! │ UseAfterMove    │ ← Detects: print(x); x.log()
//! │ Analysis        │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Strategic Clone │ ← Detects: b = a; use(a); use(b)
//! │ Analysis        │
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Ownership Fix   │ ← Emits: &x, x.clone(), &mut x
//! │ Generation      │
//! └─────────────────┘
//! ```

use std::collections::{HashMap, HashSet};

use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};

/// Location information for error reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
    /// Optional statement index for ordering
    pub stmt_index: usize,
}

impl Default for SourceSpan {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            stmt_index: 0,
        }
    }
}

/// A detected use-after-move violation
#[derive(Debug, Clone)]
pub struct UseAfterMoveError {
    /// Variable name that was used after move
    pub var: String,
    /// Where the variable was moved (consumed)
    pub move_site: SourceSpan,
    /// Where the variable was used after move
    pub use_site: SourceSpan,
    /// The function that consumed the variable
    pub moved_by: String,
    /// Suggested fix
    pub fix: OwnershipFix,
}

/// Suggested fix for an ownership violation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipFix {
    /// Change to borrow: `f(x)` → `f(&x)`
    Borrow,
    /// Change to mutable borrow: `f(x)` → `f(&mut x)`
    MutableBorrow,
    /// Add clone: `f(x)` → `f(x.clone())`
    Clone,
    /// Add clone at assignment: `let b = a` → `let b = a.clone()`
    CloneAtAssignment { var: String },
    /// No fix available (Poka-Yoke rejection needed)
    Reject { reason: String },
}

/// Result of analyzing a variable's aliasing pattern
#[derive(Debug, Clone)]
pub struct AliasingPattern {
    /// Original variable
    pub source: String,
    /// Alias variable (from assignment)
    pub alias: String,
    /// Whether source is used after aliasing
    pub source_used_after: bool,
    /// Whether alias is used after aliasing
    pub alias_used_after: bool,
    /// Type of the variable
    pub var_type: Type,
}

/// Tracks the movement state of a variable
#[derive(Debug, Clone, PartialEq, Eq)]
enum MoveState {
    /// Variable is available
    Available,
    /// Variable was moved at this location
    Moved(SourceSpan),
    /// Variable might be moved (conditional)
    ConditionallyMoved(SourceSpan),
}

/// Use-After-Move Analysis Engine
///
/// Walks function bodies in statement order, tracking when variables
/// are consumed by ownership-taking functions and flagging subsequent uses.
#[derive(Debug)]
pub struct UseAfterMoveAnalysis {
    /// Current move state of each variable
    move_states: HashMap<String, MoveState>,
    /// Detected use-after-move violations
    errors: Vec<UseAfterMoveError>,
    /// Current statement index for ordering
    current_stmt_index: usize,
    /// Functions known to take ownership
    ownership_functions: HashSet<String>,
    /// Functions known to borrow
    borrowing_functions: HashSet<String>,
}

impl Default for UseAfterMoveAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl UseAfterMoveAnalysis {
    /// Create a new analysis instance
    pub fn new() -> Self {
        Self {
            move_states: HashMap::new(),
            errors: Vec::new(),
            current_stmt_index: 0,
            ownership_functions: Self::default_ownership_functions(),
            borrowing_functions: Self::default_borrowing_functions(),
        }
    }

    /// Functions that take ownership of their arguments
    fn default_ownership_functions() -> HashSet<String> {
        [
            "push",
            "append",
            "extend",
            "insert",
            "add",
            "put",
            "set",
            "store",
            "consume",
            "take",
            "into_iter",
            "drain",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Functions that borrow their arguments
    fn default_borrowing_functions() -> HashSet<String> {
        [
            "len",
            "str",
            "repr",
            "format",
            "print",
            "isinstance",
            "hasattr",
            "getattr",
            "contains",
            "startswith",
            "endswith",
            "find",
            "index",
            "count",
            "int",
            "float",
            "bool",
            "sum",
            "min",
            "max",
            "any",
            "all",
            "reversed",
            "enumerate",
            "zip",
            "map",
            "filter",
            "iter",
            "get",
            "keys",
            "values",
            "items",
            "copy",
            "deepcopy",
            "sorted",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Analyze a function for use-after-move violations
    pub fn analyze_function(&mut self, func: &HirFunction) -> Vec<UseAfterMoveError> {
        // Reset state
        self.move_states.clear();
        self.errors.clear();
        self.current_stmt_index = 0;

        // Initialize parameters as available
        for param in &func.params {
            self.move_states
                .insert(param.name.clone(), MoveState::Available);
        }

        // Analyze each statement in order
        for stmt in &func.body {
            self.analyze_statement(stmt);
            self.current_stmt_index += 1;
        }

        self.errors.clone()
    }

    /// Analyze a statement for move semantics
    fn analyze_statement(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // First, analyze the value for uses
                self.analyze_expression_for_use(value);

                // Handle assignment as potential move source
                if let HirExpr::Var(source_name) = value {
                    // This is a potential aliasing pattern: `b = a`
                    if let AssignTarget::Symbol(target_name) = target {
                        self.record_potential_alias(target_name, source_name);
                    }
                }

                // Make target available
                if let AssignTarget::Symbol(target_name) = target {
                    self.move_states
                        .insert(target_name.clone(), MoveState::Available);
                }
            }

            HirStmt::Return(Some(expr)) => {
                self.analyze_expression_for_use(expr);
            }

            HirStmt::Return(None) => {}

            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expression_for_use(condition);

                // Save state before branches
                let state_before = self.move_states.clone();

                // Analyze then branch
                for stmt in then_body {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }
                let state_after_then = self.move_states.clone();

                // Restore and analyze else branch
                self.move_states = state_before.clone();
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                        self.current_stmt_index += 1;
                    }
                }
                let state_after_else = self.move_states.clone();

                // Merge states
                self.merge_branch_states(&state_before, &state_after_then, &state_after_else);
            }

            HirStmt::While { condition, body } => {
                self.analyze_expression_for_use(condition);

                let state_before = self.move_states.clone();
                for stmt in body {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }

                self.merge_loop_state(&state_before);
            }

            HirStmt::For { target, iter, body } => {
                // Iterating consumes the iterator
                self.analyze_expression_for_move(iter, "for");

                // Target is fresh each iteration
                if let AssignTarget::Symbol(target_name) = target {
                    self.move_states
                        .insert(target_name.clone(), MoveState::Available);
                }

                let state_before = self.move_states.clone();
                for stmt in body {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }

                self.merge_loop_state(&state_before);
            }

            HirStmt::Block(stmts) => {
                for stmt in stmts {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }
            }

            HirStmt::Expr(expr) => {
                self.analyze_expression_for_use(expr);
            }

            HirStmt::Try {
                body,
                handlers: _,
                orelse,
                finalbody,
            } => {
                for stmt in body {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }
                if let Some(else_stmts) = orelse {
                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                        self.current_stmt_index += 1;
                    }
                }
                if let Some(finally_stmts) = finalbody {
                    for stmt in finally_stmts {
                        self.analyze_statement(stmt);
                        self.current_stmt_index += 1;
                    }
                }
            }

            HirStmt::With { body, .. } => {
                for stmt in body {
                    self.analyze_statement(stmt);
                    self.current_stmt_index += 1;
                }
            }

            HirStmt::Assert { test, msg } => {
                self.analyze_expression_for_use(test);
                if let Some(m) = msg {
                    self.analyze_expression_for_use(m);
                }
            }

            HirStmt::FunctionDef { body, params, .. } => {
                // Nested function captures - analyze for use
                let outer_vars: HashSet<_> = self.move_states.keys().cloned().collect();
                let params_set: HashSet<String> = params.iter().map(|p| p.name.clone()).collect();
                self.analyze_nested_captures(body, &outer_vars, &params_set);
            }

            HirStmt::Raise { exception, cause } => {
                if let Some(e) = exception {
                    self.analyze_expression_for_use(e);
                }
                if let Some(c) = cause {
                    self.analyze_expression_for_use(c);
                }
            }

            HirStmt::Pass | HirStmt::Break { .. } | HirStmt::Continue { .. } => {}
        }
    }

    /// Analyze an expression to check if it uses a moved variable
    fn analyze_expression_for_use(&mut self, expr: &HirExpr) {
        match expr {
            HirExpr::Var(name) => {
                self.check_use(name);
            }

            HirExpr::Call { func, args, kwargs } => {
                // func is a Symbol (String), not an expression
                let takes_ownership = self.function_takes_ownership(func);

                for arg in args {
                    if takes_ownership {
                        self.analyze_expression_for_move(arg, func);
                    } else {
                        self.analyze_expression_for_use(arg);
                    }
                }

                for (_, value) in kwargs {
                    if takes_ownership {
                        self.analyze_expression_for_move(value, func);
                    } else {
                        self.analyze_expression_for_use(value);
                    }
                }
            }

            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                let takes_ownership = self.ownership_functions.contains(method);
                self.analyze_expression_for_use(object);

                for arg in args {
                    if takes_ownership {
                        self.analyze_expression_for_move(arg, method);
                    } else {
                        self.analyze_expression_for_use(arg);
                    }
                }
            }

            HirExpr::Binary { left, right, .. } => {
                self.analyze_expression_for_use(left);
                self.analyze_expression_for_use(right);
            }

            HirExpr::Unary { operand, .. } => {
                self.analyze_expression_for_use(operand);
            }

            HirExpr::Index { base, index } => {
                self.analyze_expression_for_use(base);
                self.analyze_expression_for_use(index);
            }

            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                self.analyze_expression_for_use(base);
                if let Some(s) = start {
                    self.analyze_expression_for_use(s);
                }
                if let Some(e) = stop {
                    self.analyze_expression_for_use(e);
                }
                if let Some(st) = step {
                    self.analyze_expression_for_use(st);
                }
            }

            HirExpr::Attribute { value, .. } => {
                self.analyze_expression_for_use(value);
            }

            HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
                for elem in elements {
                    self.analyze_expression_for_use(elem);
                }
            }

            HirExpr::Dict(pairs) => {
                for (key, value) in pairs {
                    self.analyze_expression_for_use(key);
                    self.analyze_expression_for_use(value);
                }
            }

            HirExpr::ListComp {
                element,
                generators,
            }
            | HirExpr::SetComp {
                element,
                generators,
            } => {
                for gen in generators {
                    self.analyze_expression_for_use(&gen.iter);
                    for cond in &gen.conditions {
                        self.analyze_expression_for_use(cond);
                    }
                }
                self.analyze_expression_for_use(element);
            }

            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                for gen in generators {
                    self.analyze_expression_for_use(&gen.iter);
                    for cond in &gen.conditions {
                        self.analyze_expression_for_use(cond);
                    }
                }
                self.analyze_expression_for_use(key);
                self.analyze_expression_for_use(value);
            }

            HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                for gen in generators {
                    self.analyze_expression_for_use(&gen.iter);
                    for cond in &gen.conditions {
                        self.analyze_expression_for_use(cond);
                    }
                }
                self.analyze_expression_for_use(element);
            }

            HirExpr::Lambda { body, .. } => {
                self.analyze_expression_for_use(body);
            }

            HirExpr::IfExpr { test, body, orelse } => {
                self.analyze_expression_for_use(test);
                self.analyze_expression_for_use(body);
                self.analyze_expression_for_use(orelse);
            }

            HirExpr::Await { value } => {
                self.analyze_expression_for_use(value);
            }

            HirExpr::Yield { value: Some(v), .. } => {
                self.analyze_expression_for_use(v);
            }

            HirExpr::Yield { value: None, .. } => {}

            HirExpr::NamedExpr { value, .. } => {
                self.analyze_expression_for_use(value);
            }

            HirExpr::FString { parts } => {
                for part in parts {
                    if let crate::hir::FStringPart::Expr(expr) = part {
                        self.analyze_expression_for_use(expr);
                    }
                }
            }

            // Literals don't use variables
            HirExpr::Literal(_) => {}

            // Handle remaining cases
            _ => {}
        }
    }

    /// Analyze an expression that will be moved (consumed)
    fn analyze_expression_for_move(&mut self, expr: &HirExpr, moved_by: &str) {
        self.analyze_expression_for_use(expr);

        if let HirExpr::Var(name) = expr {
            self.record_move(name, moved_by);
        }
    }

    /// Check if using a variable that was moved
    fn check_use(&mut self, name: &str) {
        if let Some(state) = self.move_states.get(name) {
            match state {
                MoveState::Moved(move_site) => {
                    self.errors.push(UseAfterMoveError {
                        var: name.to_string(),
                        move_site: move_site.clone(),
                        use_site: SourceSpan {
                            stmt_index: self.current_stmt_index,
                            ..Default::default()
                        },
                        moved_by: String::new(),
                        fix: OwnershipFix::Borrow,
                    });
                }
                MoveState::ConditionallyMoved(move_site) => {
                    self.errors.push(UseAfterMoveError {
                        var: name.to_string(),
                        move_site: move_site.clone(),
                        use_site: SourceSpan {
                            stmt_index: self.current_stmt_index,
                            ..Default::default()
                        },
                        moved_by: String::new(),
                        fix: OwnershipFix::Clone,
                    });
                }
                MoveState::Available => {}
            }
        }
    }

    /// Record that a variable was moved
    fn record_move(&mut self, name: &str, moved_by: &str) {
        let move_site = SourceSpan {
            stmt_index: self.current_stmt_index,
            ..Default::default()
        };

        for error in &mut self.errors {
            if error.var == name && error.moved_by.is_empty() {
                error.moved_by = moved_by.to_string();
            }
        }

        self.move_states
            .insert(name.to_string(), MoveState::Moved(move_site));
    }

    /// Record potential alias pattern
    fn record_potential_alias(&mut self, _target: &str, _source: &str) {
        // Track aliasing for later analysis
    }

    /// Determine if a function takes ownership of arguments
    fn function_takes_ownership(&self, func_name: &str) -> bool {
        if self.borrowing_functions.contains(func_name) {
            return false;
        }
        // Conservative: unknown functions are assumed to borrow (Python semantics)
        false
    }

    /// Merge states after if/else branches
    fn merge_branch_states(
        &mut self,
        before: &HashMap<String, MoveState>,
        after_then: &HashMap<String, MoveState>,
        after_else: &HashMap<String, MoveState>,
    ) {
        let all_vars: HashSet<_> = before
            .keys()
            .chain(after_then.keys())
            .chain(after_else.keys())
            .cloned()
            .collect();

        for var in all_vars {
            let then_state = after_then.get(&var);
            let else_state = after_else.get(&var);

            let new_state = match (then_state, else_state) {
                (Some(MoveState::Moved(s)), _) | (_, Some(MoveState::Moved(s))) => {
                    MoveState::ConditionallyMoved(s.clone())
                }
                (Some(MoveState::ConditionallyMoved(s)), _)
                | (_, Some(MoveState::ConditionallyMoved(s))) => {
                    MoveState::ConditionallyMoved(s.clone())
                }
                _ => before.get(&var).cloned().unwrap_or(MoveState::Available),
            };

            self.move_states.insert(var, new_state);
        }
    }

    /// Merge state after loop body
    fn merge_loop_state(&mut self, before: &HashMap<String, MoveState>) {
        let current = self.move_states.clone();

        for (var, state) in &current {
            if let MoveState::Moved(span) = state {
                if before
                    .get(var)
                    .map(|s| s == &MoveState::Available)
                    .unwrap_or(false)
                {
                    self.move_states
                        .insert(var.clone(), MoveState::ConditionallyMoved(span.clone()));
                }
            }
        }
    }

    /// Analyze captures in nested function
    fn analyze_nested_captures(
        &mut self,
        body: &[HirStmt],
        outer_vars: &HashSet<String>,
        params: &HashSet<String>,
    ) {
        for stmt in body {
            self.find_captured_vars_in_stmt(stmt, outer_vars, params);
        }
    }

    /// Find captured variables in a statement
    fn find_captured_vars_in_stmt(
        &mut self,
        stmt: &HirStmt,
        outer_vars: &HashSet<String>,
        params: &HashSet<String>,
    ) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.find_captured_vars_in_expr(value, outer_vars, params);
            }
            HirStmt::Return(Some(expr)) => {
                self.find_captured_vars_in_expr(expr, outer_vars, params);
            }
            HirStmt::Expr(expr) => {
                self.find_captured_vars_in_expr(expr, outer_vars, params);
            }
            _ => {}
        }
    }

    /// Find captured variables in an expression
    fn find_captured_vars_in_expr(
        &mut self,
        expr: &HirExpr,
        outer_vars: &HashSet<String>,
        params: &HashSet<String>,
    ) {
        if let HirExpr::Var(name) = expr {
            if outer_vars.contains(name) && !params.contains(name) {
                self.check_use(name);
            }
        }

        match expr {
            HirExpr::Call { args, kwargs, .. } => {
                // func is a Symbol, not an expression - no need to check it
                for arg in args {
                    self.find_captured_vars_in_expr(arg, outer_vars, params);
                }
                for (_, v) in kwargs {
                    self.find_captured_vars_in_expr(v, outer_vars, params);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.find_captured_vars_in_expr(left, outer_vars, params);
                self.find_captured_vars_in_expr(right, outer_vars, params);
            }
            HirExpr::Unary { operand, .. } => {
                self.find_captured_vars_in_expr(operand, outer_vars, params);
            }
            _ => {}
        }
    }

    /// Get the detected errors
    pub fn errors(&self) -> &[UseAfterMoveError] {
        &self.errors
    }
}

/// Strategic Clone Analysis
#[derive(Debug)]
pub struct StrategicCloneAnalysis {
    aliases: Vec<AliasingPattern>,
    clone_assignments: Vec<String>,
}

impl Default for StrategicCloneAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl StrategicCloneAnalysis {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
            clone_assignments: Vec::new(),
        }
    }

    pub fn analyze_function(&mut self, func: &HirFunction) -> Vec<AliasingPattern> {
        self.aliases.clear();
        self.clone_assignments.clear();

        let mut var_assignments: HashMap<String, Vec<usize>> = HashMap::new();
        let mut var_uses: HashMap<String, Vec<usize>> = HashMap::new();
        let mut aliases: Vec<(String, String, usize, Type)> = Vec::new();

        for (stmt_idx, stmt) in func.body.iter().enumerate() {
            self.collect_var_info(
                stmt,
                stmt_idx,
                &mut var_assignments,
                &mut var_uses,
                &mut aliases,
            );
        }

        for (target, source, alias_idx, var_type) in aliases {
            let source_used_after = var_uses
                .get(&source)
                .map(|uses| uses.iter().any(|&u| u > alias_idx))
                .unwrap_or(false);

            let alias_used_after = var_uses
                .get(&target)
                .map(|uses| uses.iter().any(|&u| u > alias_idx))
                .unwrap_or(false);

            if source_used_after && alias_used_after {
                self.aliases.push(AliasingPattern {
                    source: source.clone(),
                    alias: target.clone(),
                    source_used_after,
                    alias_used_after,
                    var_type,
                });
                self.clone_assignments.push(target);
            }
        }

        self.aliases.clone()
    }

    fn collect_var_info(
        &self,
        stmt: &HirStmt,
        stmt_idx: usize,
        assignments: &mut HashMap<String, Vec<usize>>,
        uses: &mut HashMap<String, Vec<usize>>,
        aliases: &mut Vec<(String, String, usize, Type)>,
    ) {
        match stmt {
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => {
                if let AssignTarget::Symbol(target_name) = target {
                    assignments
                        .entry(target_name.clone())
                        .or_default()
                        .push(stmt_idx);

                    if let HirExpr::Var(source_name) = value {
                        let ty = type_annotation.clone().unwrap_or(Type::Unknown);
                        aliases.push((target_name.clone(), source_name.clone(), stmt_idx, ty));
                    }
                }
                self.collect_uses_in_expr(value, stmt_idx, uses);
            }

            HirStmt::Return(Some(expr)) => {
                self.collect_uses_in_expr(expr, stmt_idx, uses);
            }

            HirStmt::Expr(expr) => {
                self.collect_uses_in_expr(expr, stmt_idx, uses);
            }

            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.collect_uses_in_expr(condition, stmt_idx, uses);
                for s in then_body {
                    self.collect_var_info(s, stmt_idx, assignments, uses, aliases);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.collect_var_info(s, stmt_idx, assignments, uses, aliases);
                    }
                }
            }

            HirStmt::While { condition, body } => {
                self.collect_uses_in_expr(condition, stmt_idx, uses);
                for s in body {
                    self.collect_var_info(s, stmt_idx, assignments, uses, aliases);
                }
            }

            HirStmt::For { iter, body, .. } => {
                self.collect_uses_in_expr(iter, stmt_idx, uses);
                for s in body {
                    self.collect_var_info(s, stmt_idx, assignments, uses, aliases);
                }
            }

            HirStmt::Block(stmts) => {
                for s in stmts {
                    self.collect_var_info(s, stmt_idx, assignments, uses, aliases);
                }
            }

            _ => {}
        }
    }

    fn collect_uses_in_expr(
        &self,
        expr: &HirExpr,
        stmt_idx: usize,
        uses: &mut HashMap<String, Vec<usize>>,
    ) {
        match expr {
            HirExpr::Var(name) => {
                uses.entry(name.clone()).or_default().push(stmt_idx);
            }
            HirExpr::Call { args, kwargs, .. } => {
                // func is a Symbol, not an expression - no need to check it
                for arg in args {
                    self.collect_uses_in_expr(arg, stmt_idx, uses);
                }
                for (_, v) in kwargs {
                    self.collect_uses_in_expr(v, stmt_idx, uses);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.collect_uses_in_expr(object, stmt_idx, uses);
                for arg in args {
                    self.collect_uses_in_expr(arg, stmt_idx, uses);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.collect_uses_in_expr(left, stmt_idx, uses);
                self.collect_uses_in_expr(right, stmt_idx, uses);
            }
            HirExpr::Unary { operand, .. } => {
                self.collect_uses_in_expr(operand, stmt_idx, uses);
            }
            HirExpr::List(elements) | HirExpr::Tuple(elements) | HirExpr::Set(elements) => {
                for e in elements {
                    self.collect_uses_in_expr(e, stmt_idx, uses);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.collect_uses_in_expr(k, stmt_idx, uses);
                    self.collect_uses_in_expr(v, stmt_idx, uses);
                }
            }
            HirExpr::Index { base, index } => {
                self.collect_uses_in_expr(base, stmt_idx, uses);
                self.collect_uses_in_expr(index, stmt_idx, uses);
            }
            HirExpr::Attribute { value, .. } => {
                self.collect_uses_in_expr(value, stmt_idx, uses);
            }
            HirExpr::IfExpr { test, body, orelse } => {
                self.collect_uses_in_expr(test, stmt_idx, uses);
                self.collect_uses_in_expr(body, stmt_idx, uses);
                self.collect_uses_in_expr(orelse, stmt_idx, uses);
            }
            _ => {}
        }
    }

    pub fn needs_clone(&self) -> &[String] {
        &self.clone_assignments
    }
}

/// Combined ownership analysis result
#[derive(Debug, Clone)]
pub struct OwnershipAnalysisResult {
    pub use_after_move_errors: Vec<UseAfterMoveError>,
    pub aliasing_patterns: Vec<AliasingPattern>,
    pub borrow_sites: HashMap<String, Vec<usize>>,
    pub clone_sites: HashMap<String, Vec<usize>>,
    pub mut_borrow_sites: HashMap<String, Vec<usize>>,
}

/// Run complete ownership analysis on a function
pub fn analyze_ownership(func: &HirFunction) -> OwnershipAnalysisResult {
    let mut uam_analysis = UseAfterMoveAnalysis::new();
    let use_after_move_errors = uam_analysis.analyze_function(func);

    let mut clone_analysis = StrategicCloneAnalysis::new();
    let aliasing_patterns = clone_analysis.analyze_function(func);

    let mut borrow_sites: HashMap<String, Vec<usize>> = HashMap::new();
    let mut clone_sites: HashMap<String, Vec<usize>> = HashMap::new();
    let mut mut_borrow_sites: HashMap<String, Vec<usize>> = HashMap::new();

    for error in &use_after_move_errors {
        match &error.fix {
            OwnershipFix::Borrow => {
                borrow_sites
                    .entry(error.var.clone())
                    .or_default()
                    .push(error.move_site.stmt_index);
            }
            OwnershipFix::MutableBorrow => {
                mut_borrow_sites
                    .entry(error.var.clone())
                    .or_default()
                    .push(error.move_site.stmt_index);
            }
            OwnershipFix::Clone | OwnershipFix::CloneAtAssignment { .. } => {
                clone_sites
                    .entry(error.var.clone())
                    .or_default()
                    .push(error.move_site.stmt_index);
            }
            OwnershipFix::Reject { .. } => {}
        }
    }

    OwnershipAnalysisResult {
        use_after_move_errors,
        aliasing_patterns,
        borrow_sites,
        clone_sites,
        mut_borrow_sites,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, Literal};
    use smallvec::smallvec;

    fn make_var(name: &str) -> HirExpr {
        HirExpr::Var(name.to_string())
    }

    fn make_literal_int(n: i64) -> HirExpr {
        HirExpr::Literal(Literal::Int(n))
    }

    fn make_function(name: &str, body: Vec<HirStmt>) -> HirFunction {
        HirFunction {
            name: name.to_string(),
            params: smallvec![],
            ret_type: Type::Unknown,
            body,
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_source_span_default() {
        let span = SourceSpan::default();
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 1);
        assert_eq!(span.stmt_index, 0);
    }

    #[test]
    fn test_use_after_move_analysis_new() {
        let analysis = UseAfterMoveAnalysis::new();
        assert!(analysis.errors.is_empty());
        assert!(analysis.move_states.is_empty());
    }

    #[test]
    fn test_no_use_after_move_simple() {
        let func = HirFunction {
            name: "foo".to_string(),
            params: smallvec![crate::hir::HirParam {
                name: "x".to_string(),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(make_var("x")))],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let mut analysis = UseAfterMoveAnalysis::new();
        let errors = analysis.analyze_function(&func);
        assert!(errors.is_empty(), "No use-after-move expected");
    }

    #[test]
    fn test_strategic_clone_aliasing_pattern() {
        let func = make_function(
            "foo",
            vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("a".to_string()),
                    value: make_literal_int(1),
                    type_annotation: Some(Type::Int),
                },
                HirStmt::Assign {
                    target: AssignTarget::Symbol("b".to_string()),
                    value: make_var("a"),
                    type_annotation: Some(Type::Int),
                },
                HirStmt::Expr(HirExpr::Call {
                    func: "use".to_string(),
                    args: vec![make_var("a")],
                    kwargs: vec![],
                }),
                HirStmt::Expr(HirExpr::Call {
                    func: "use".to_string(),
                    args: vec![make_var("b")],
                    kwargs: vec![],
                }),
            ],
        );

        let mut analysis = StrategicCloneAnalysis::new();
        let patterns = analysis.analyze_function(&func);

        assert!(!patterns.is_empty(), "Should detect aliasing pattern");
        assert_eq!(patterns[0].source, "a");
        assert_eq!(patterns[0].alias, "b");
    }

    #[test]
    fn test_ownership_fix_variants() {
        let borrow = OwnershipFix::Borrow;
        let mut_borrow = OwnershipFix::MutableBorrow;
        let clone = OwnershipFix::Clone;
        let clone_assign = OwnershipFix::CloneAtAssignment {
            var: "x".to_string(),
        };
        let reject = OwnershipFix::Reject {
            reason: "test".to_string(),
        };

        assert_eq!(borrow, OwnershipFix::Borrow);
        assert_eq!(mut_borrow, OwnershipFix::MutableBorrow);
        assert_ne!(clone, clone_assign);
        assert_ne!(borrow, reject);
    }

    #[test]
    fn test_analyze_ownership_comprehensive() {
        let func = make_function(
            "test",
            vec![
                HirStmt::Assign {
                    target: AssignTarget::Symbol("x".to_string()),
                    value: make_literal_int(42),
                    type_annotation: Some(Type::Int),
                },
                HirStmt::Return(Some(make_var("x"))),
            ],
        );

        let result = analyze_ownership(&func);
        assert!(result.use_after_move_errors.is_empty());
    }

    #[test]
    fn test_default_impls() {
        let uam = UseAfterMoveAnalysis::default();
        assert!(uam.errors.is_empty());

        let clone = StrategicCloneAnalysis::default();
        assert!(clone.aliases.is_empty());
    }

    #[test]
    fn test_ownership_analysis_with_if() {
        let func = HirFunction {
            name: "foo".to_string(),
            params: smallvec![crate::hir::HirParam {
                name: "x".to_string(),
                ty: Type::Int,
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::Int,
            body: vec![
                HirStmt::If {
                    condition: HirExpr::Binary {
                        op: BinOp::Gt,
                        left: Box::new(make_var("x")),
                        right: Box::new(make_literal_int(0)),
                    },
                    then_body: vec![HirStmt::Assign {
                        target: AssignTarget::Symbol("y".to_string()),
                        value: make_var("x"),
                        type_annotation: Some(Type::Int),
                    }],
                    else_body: Some(vec![HirStmt::Assign {
                        target: AssignTarget::Symbol("y".to_string()),
                        value: HirExpr::Unary {
                            op: crate::hir::UnaryOp::Neg,
                            operand: Box::new(make_var("x")),
                        },
                        type_annotation: Some(Type::Int),
                    }]),
                },
                HirStmt::Return(Some(make_var("y"))),
            ],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let mut analysis = UseAfterMoveAnalysis::new();
        let errors = analysis.analyze_function(&func);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_ownership_analysis_with_loop() {
        let func = HirFunction {
            name: "foo".to_string(),
            params: smallvec![crate::hir::HirParam {
                name: "items".to_string(),
                ty: Type::List(Box::new(Type::Int)),
                default: None,
                is_vararg: false,
            }],
            ret_type: Type::None,
            body: vec![HirStmt::For {
                target: AssignTarget::Symbol("item".to_string()),
                iter: make_var("items"),
                body: vec![HirStmt::Expr(HirExpr::Call {
                    func: "print".to_string(),
                    args: vec![make_var("item")],
                    kwargs: vec![],
                })],
            }],
            properties: Default::default(),
            annotations: Default::default(),
            docstring: None,
        };

        let result = analyze_ownership(&func);
        assert!(
            result.use_after_move_errors.is_empty(),
            "print should borrow"
        );
    }

    #[test]
    fn test_move_state_variants() {
        let available = MoveState::Available;
        let moved = MoveState::Moved(SourceSpan::default());
        let conditional = MoveState::ConditionallyMoved(SourceSpan::default());

        assert_eq!(available, MoveState::Available);
        assert_ne!(available, moved);
        assert_ne!(moved, conditional);
    }

    #[test]
    fn test_aliasing_pattern_struct() {
        let pattern = AliasingPattern {
            source: "a".to_string(),
            alias: "b".to_string(),
            source_used_after: true,
            alias_used_after: true,
            var_type: Type::String,
        };

        assert_eq!(pattern.source, "a");
        assert_eq!(pattern.alias, "b");
        assert!(pattern.source_used_after);
        assert!(pattern.alias_used_after);
    }
}
