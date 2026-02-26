//! Mutable variable analysis — extracted from `analyze_mutable_vars` in `rust_gen.rs`
//!
//! This module contains the two large nested functions that were previously defined
//! inside `analyze_mutable_vars`:
//!
//! - [`analyze_expr_for_mutations`]: Recursive expression visitor that detects mutating
//!   method calls, attribute accesses, and transitive mutations through function calls.
//! - [`analyze_stmt`]: Statement-level mutation analysis that identifies variable
//!   reassignments, index/attribute assignments, and delegates to
//!   `analyze_expr_for_mutations` for expression-level detection.
//!
//! By moving these to module-level, the measured cyclomatic complexity of
//! `analyze_mutable_vars` is reduced while preserving identical behavior.

use crate::hir::*;
use std::collections::{HashMap, HashSet};

use super::mutation_helpers;

/// Recursive expression visitor detecting mutating method calls.
///
/// Walks the HIR expression tree and inserts variable names into `mutable`
/// when a mutating method or attribute is found on that variable.
pub(super) fn analyze_expr_for_mutations(
    expr: &HirExpr,
    mutable: &mut HashSet<String>,
    var_types: &HashMap<String, String>,
    mutating_methods: &HashMap<String, HashSet<String>>,
    function_param_muts: &HashMap<String, Vec<bool>>,
) {
    match expr {
        HirExpr::MethodCall { object, method, args, .. } => {
            // Check if this is a mutating method call
            let is_mut = if mutation_helpers::is_mutating_method(method) {
                // Built-in mutating method
                true
            } else if let HirExpr::Var(var_name) = &**object {
                // Check if this is a user-defined mutating method
                if let Some(class_name) = var_types.get(var_name) {
                    if let Some(mut_methods) = mutating_methods.get(class_name) {
                        mut_methods.contains(method)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if is_mut {
                if let HirExpr::Var(var_name) = &**object {
                    mutable.insert(var_name.clone());
                }
            }
            // Recursively check nested expressions
            analyze_expr_for_mutations(
                object,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            for arg in args {
                analyze_expr_for_mutations(
                    arg,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        HirExpr::Binary { left, right, .. } => {
            analyze_expr_for_mutations(
                left,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            analyze_expr_for_mutations(
                right,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        HirExpr::Unary { operand, .. } => {
            analyze_expr_for_mutations(
                operand,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        // DEPYLER-1217: Detect transitive mutation through function calls
        // If a variable is passed to a function that expects &mut at that position,
        // the variable must be mutable in the caller's scope
        HirExpr::Call { func, args, .. } => {
            // Check if called function has param_muts info
            if let Some(param_muts) = function_param_muts.get(func) {
                for (idx, arg) in args.iter().enumerate() {
                    // If this param needs &mut, mark the variable as mutable
                    if param_muts.get(idx).copied().unwrap_or(false) {
                        if let HirExpr::Var(var_name) = arg {
                            mutable.insert(var_name.clone());
                        }
                    }
                    analyze_expr_for_mutations(
                        arg,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            } else {
                // No param_muts info - just recurse into args
                for arg in args {
                    analyze_expr_for_mutations(
                        arg,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            }
        }
        HirExpr::IfExpr { test, body, orelse } => {
            analyze_expr_for_mutations(
                test,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            analyze_expr_for_mutations(
                body,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            analyze_expr_for_mutations(
                orelse,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        HirExpr::List(items)
        | HirExpr::Tuple(items)
        | HirExpr::Set(items)
        | HirExpr::FrozenSet(items) => {
            for item in items {
                analyze_expr_for_mutations(
                    item,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        HirExpr::Dict(pairs) => {
            for (key, value) in pairs {
                analyze_expr_for_mutations(
                    key,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
                analyze_expr_for_mutations(
                    value,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        HirExpr::Index { base, index } => {
            analyze_expr_for_mutations(
                base,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            analyze_expr_for_mutations(
                index,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        HirExpr::Attribute { value, attr } => {
            // DEPYLER-0835: Some Python attributes translate to mutating method calls in Rust
            // e.g., csv.DictReader.fieldnames -> reader.headers() (requires &mut self)
            if mutation_helpers::is_mutating_attribute(attr) {
                if let HirExpr::Var(name) = value.as_ref() {
                    mutable.insert(name.clone());
                }
            }
            analyze_expr_for_mutations(
                value,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        _ => {}
    }
}

/// Statement-level mutation analysis.
///
/// Walks the HIR statement tree and identifies variables that need `mut`
/// due to reassignment, index assignment, attribute assignment, or
/// mutating method/function calls in expressions.
pub(super) fn analyze_stmt(
    stmt: &HirStmt,
    declared: &mut HashSet<String>,
    mutable: &mut HashSet<String>,
    var_types: &mut HashMap<String, String>,
    mutating_methods: &HashMap<String, HashSet<String>>,
    function_param_muts: &HashMap<String, Vec<bool>>,
) {
    match stmt {
        HirStmt::Assign { target, value, .. } => {
            // Check if the value expression contains method calls that mutate variables
            analyze_expr_for_mutations(
                value,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );

            match target {
                AssignTarget::Symbol(name) => {
                    // Track variable type if assigned from class constructor
                    if let HirExpr::Call { func, .. } = value {
                        // Store the type (class name) for this variable
                        var_types.insert(name.clone(), func.clone());
                    }

                    // DEPYLER-0549: Mark csv readers/writers as mutable
                    // In Rust, csv::Reader and csv::Writer require &mut self for most operations
                    // Detection: variable names or call patterns involving csv/reader/writer
                    // DEPYLER-0835: Name heuristic should ALWAYS apply, not just as fallback
                    let name_heuristic = name == "reader"
                        || name == "writer"
                        || name.contains("reader")
                        || name.contains("writer");
                    let pattern_match = if let HirExpr::MethodCall { object, method, .. } = value {
                        // csv.DictReader() or csv.reader()
                        if let HirExpr::Var(module) = object.as_ref() {
                            module == "csv"
                                && (method.contains("Reader")
                                    || method.contains("reader")
                                    || method.contains("Writer")
                                    || method.contains("writer"))
                        } else {
                            false
                        }
                    } else if let HirExpr::Call { func, .. } = value {
                        // DictReader(f) or csv.ReaderBuilder...
                        func.contains("Reader")
                            || func.contains("Writer")
                            || func.contains("reader")
                            || func.contains("writer")
                    } else {
                        false
                    };
                    let needs_csv_mut = name_heuristic || pattern_match;

                    if needs_csv_mut {
                        mutable.insert(name.clone());
                    }

                    if declared.contains(name) {
                        // Variable is being reassigned - mark as mutable
                        mutable.insert(name.clone());
                    } else {
                        // First declaration
                        declared.insert(name.clone());
                    }
                }
                AssignTarget::Tuple(targets) => {
                    // DEPYLER-1217: Tuple assignment - recursively handle all target types
                    // including Index targets (e.g., arr[i], arr[j] = arr[j], arr[i])
                    fn handle_tuple_target(
                        t: &AssignTarget,
                        declared: &mut HashSet<String>,
                        mutable: &mut HashSet<String>,
                    ) {
                        match t {
                            AssignTarget::Symbol(name) => {
                                if declared.contains(name) {
                                    // Variable is being reassigned - mark as mutable
                                    mutable.insert(name.clone());
                                } else {
                                    // First declaration
                                    declared.insert(name.clone());
                                }
                            }
                            AssignTarget::Index { base, .. } => {
                                // Index assignment mutates the base
                                // DEPYLER-0596-FIX: Handle nested index in tuple assignments
                                fn find_base_var(expr: &HirExpr) -> Option<String> {
                                    match expr {
                                        HirExpr::Var(name) => Some(name.clone()),
                                        HirExpr::Index { base, .. } => find_base_var(base),
                                        _ => None,
                                    }
                                }
                                if let Some(var_name) = find_base_var(base.as_ref()) {
                                    mutable.insert(var_name);
                                }
                            }
                            AssignTarget::Attribute { value, .. } => {
                                // Attribute assignment mutates the base
                                if let HirExpr::Var(var_name) = value.as_ref() {
                                    mutable.insert(var_name.clone());
                                }
                            }
                            AssignTarget::Tuple(nested_targets) => {
                                // Recursively handle nested tuples
                                for nested in nested_targets {
                                    handle_tuple_target(nested, declared, mutable);
                                }
                            }
                        }
                    }
                    for t in targets {
                        handle_tuple_target(t, declared, mutable);
                    }
                }
                AssignTarget::Attribute { value, .. } => {
                    // DEPYLER-0235 FIX: Property writes require the base object to be mutable
                    // e.g., `b.size = 20` requires `let mut b = ...`
                    if let HirExpr::Var(var_name) = value.as_ref() {
                        mutable.insert(var_name.clone());
                    }
                }
                AssignTarget::Index { base, .. } => {
                    // DEPYLER-0235 FIX: Index assignments also require mutability
                    // e.g., `arr[i] = value` requires `let mut arr = ...`
                    // DEPYLER-0596-FIX: Handle nested index (e.g., `d["a"]["b"] = v`)
                    // by recursively finding the innermost variable
                    fn find_base_var(expr: &HirExpr) -> Option<String> {
                        match expr {
                            HirExpr::Var(name) => Some(name.clone()),
                            HirExpr::Index { base, .. } => find_base_var(base),
                            _ => None,
                        }
                    }
                    if let Some(var_name) = find_base_var(base.as_ref()) {
                        mutable.insert(var_name);
                    }
                }
            }
        }
        HirStmt::Expr(expr) => {
            // Check standalone expressions for method calls (e.g., numbers.push(4))
            analyze_expr_for_mutations(
                expr,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        HirStmt::Return(Some(expr)) => {
            analyze_expr_for_mutations(
                expr,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
        }
        HirStmt::If { condition, then_body, else_body, .. } => {
            analyze_expr_for_mutations(
                condition,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            for stmt in then_body {
                analyze_stmt(
                    stmt,
                    declared,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
            if let Some(else_stmts) = else_body {
                for stmt in else_stmts {
                    analyze_stmt(
                        stmt,
                        declared,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            }
        }
        HirStmt::While { condition, body, .. } => {
            analyze_expr_for_mutations(
                condition,
                mutable,
                var_types,
                mutating_methods,
                function_param_muts,
            );
            for stmt in body {
                analyze_stmt(
                    stmt,
                    declared,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        HirStmt::For { body, .. } => {
            for stmt in body {
                analyze_stmt(
                    stmt,
                    declared,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        // DEPYLER-0549: Handle WITH statements - analyze body for mutations
        HirStmt::With { body, .. } => {
            for stmt in body {
                analyze_stmt(
                    stmt,
                    declared,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
        }
        // DEPYLER-0549: Handle Try - analyze all branches
        HirStmt::Try { body, handlers, orelse, finalbody, .. } => {
            for stmt in body {
                analyze_stmt(
                    stmt,
                    declared,
                    mutable,
                    var_types,
                    mutating_methods,
                    function_param_muts,
                );
            }
            for handler in handlers {
                for stmt in &handler.body {
                    analyze_stmt(
                        stmt,
                        declared,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            }
            if let Some(else_stmts) = orelse {
                for stmt in else_stmts {
                    analyze_stmt(
                        stmt,
                        declared,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            }
            if let Some(final_stmts) = finalbody {
                for stmt in final_stmts {
                    analyze_stmt(
                        stmt,
                        declared,
                        mutable,
                        var_types,
                        mutating_methods,
                        function_param_muts,
                    );
                }
            }
        }
        _ => {}
    }
}
