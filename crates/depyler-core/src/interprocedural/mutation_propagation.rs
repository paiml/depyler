//! Mutation propagation engine for interprocedural analysis
//!
//! This module implements a fixpoint iteration algorithm to propagate
//! mutation information across function boundaries.
//!
//! # Overview
//!
//! When Python code like `update_dict(state.data, "key", 100)` is transpiled,
//! we need to determine that passing `state.data` to a function that mutates
//! its parameter means `state` itself must be mutable.
//!
//! # Algorithm
//!
//! The analysis proceeds in two phases:
//!
//! ## Phase 1: Local Mutation Collection (Intraprocedural)
//!
//! For each function, analyze its body to detect:
//! - Direct parameter mutations: `param.field = value`
//! - Method calls that mutate: `param.list.append(item)`
//! - Index assignments: `param[0] = value`
//!
//! ## Phase 2: Interprocedural Propagation (Fixpoint Iteration)
//!
//! Propagate mutations through function calls until convergence:
//! 1. Process functions in topological order (callees before callers)
//! 2. For each call site, check if arguments are passed to mutating parameters
//! 3. If `callee` mutates parameter `p` and `caller` passes `arg` to `p`:
//!    - Extract root variable from `arg` (e.g., `state.data` → `state`)
//!    - Mark root variable as mutated in `caller`
//! 4. Repeat until no changes (fixpoint)
//!
//! # Example
//!
//! ```python
//! def update_dict(data: dict[str, int], key: str, value: int) -> None:
//!     data[key] = value  # Phase 1: 'data' is mutated
//!
//! def use_helper(state: State) -> None:
//!     update_dict(state.data, "key1", 100)  # Phase 2: 'state' is mutated
//! ```
//!
//! Result after fixpoint:
//! - `update_dict`: mutated_params = {"data"}
//! - `use_helper`: mutated_params = {"state"}  ← propagated!
//!
//! # Integration
//!
//! Results are consumed by:
//! - `BorrowingContext::analyze_function_with_interprocedural()` - marks parameters as mutated
//! - `LifetimeInference` - generates `&mut` instead of `&` for mutated parameters

use crate::expr_utils::extract_root_var;
use crate::hir::{AssignTarget, HirExpr, HirFunction, HirModule, HirStmt};
use crate::interprocedural::call_graph::CallGraph;
use crate::interprocedural::signature_registry::FunctionSignatureRegistry;
use std::collections::{HashMap, HashSet};

/// Information about mutations in a function
#[derive(Debug, Clone)]
pub struct MutationInfo {
    /// Parameters that are mutated in this function
    pub mutated_params: HashSet<String>,
    /// Parameters that are borrowed (immutably)
    pub borrowed_params: HashSet<String>,
    /// Local variables that are mutated
    pub mutated_locals: HashSet<String>,
}

impl MutationInfo {
    /// Create empty mutation info
    pub fn new() -> Self {
        Self {
            mutated_params: HashSet::new(),
            borrowed_params: HashSet::new(),
            mutated_locals: HashSet::new(),
        }
    }

    /// Merge another mutation info into this one
    pub fn merge(&mut self, other: &MutationInfo) {
        self.mutated_params
            .extend(other.mutated_params.iter().cloned());
        self.borrowed_params
            .extend(other.borrowed_params.iter().cloned());
        self.mutated_locals
            .extend(other.mutated_locals.iter().cloned());
    }
}

impl Default for MutationInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of mutation propagation
#[derive(Debug, Clone)]
pub struct PropagationResult {
    /// Mutation information for each function
    pub mutations: HashMap<String, MutationInfo>,
    /// Number of iterations to convergence
    pub iterations: usize,
    /// Whether the analysis converged
    pub converged: bool,
}

/// Propagates mutation information across function boundaries
pub struct MutationPropagator<'a> {
    /// Function signature registry
    registry: &'a FunctionSignatureRegistry,
    /// Call graph
    call_graph: &'a CallGraph,
    /// Mutation info for each function
    mutations: HashMap<String, MutationInfo>,
    /// HIR functions for analysis
    functions: HashMap<String, &'a HirFunction>,
}

impl<'a> MutationPropagator<'a> {
    /// Create a new mutation propagator
    pub fn new(registry: &'a FunctionSignatureRegistry, call_graph: &'a CallGraph) -> Self {
        Self {
            registry,
            call_graph,
            mutations: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Set the HIR functions for analysis
    pub fn with_module(mut self, module: &'a HirModule) -> Self {
        for func in &module.functions {
            self.functions.insert(func.name.clone(), func);
        }
        self
    }

    /// Run fixpoint iteration to propagate mutations
    /// Also updates the registry with mutation information
    pub fn propagate(&mut self) -> PropagationResult {
        // Phase 1: Collect local mutations (intraprocedural)
        self.collect_local_mutations();

        // Phase 2: Propagate through call graph (interprocedural)
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            // Process functions in topological order (callees before callers)
            for func_name in self.call_graph.topological_order() {
                if self.propagate_for_function(&func_name) {
                    changed = true;
                }
            }
        }

        PropagationResult {
            mutations: self.mutations.clone(),
            iterations,
            converged: !changed,
        }
    }

    /// Collect local mutations in each function
    fn collect_local_mutations(&mut self) {
        for (func_name, func) in &self.functions {
            let mut mutation_info = MutationInfo::new();

            // Collect parameters (all are potentially borrowed)
            for param in &func.params {
                mutation_info.borrowed_params.insert(param.name.clone());
            }

            // Analyze function body for mutations
            for stmt in &func.body {
                self.analyze_stmt_for_mutations(
                    stmt,
                    &mut mutation_info,
                    &func.params.iter().map(|p| p.name.clone()).collect(),
                );
            }

            self.mutations.insert(func_name.to_string(), mutation_info);
        }
    }

    /// Analyze a statement for mutations
    fn analyze_stmt_for_mutations(
        &self,
        stmt: &HirStmt,
        mutation_info: &mut MutationInfo,
        param_names: &HashSet<String>,
    ) {
        match stmt {
            HirStmt::Assign { target, value, .. } => {
                // Check what's being mutated
                match target {
                    AssignTarget::Symbol(name) => {
                        if param_names.contains(name) {
                            mutation_info.mutated_params.insert(name.clone());
                        } else {
                            mutation_info.mutated_locals.insert(name.clone());
                        }
                    }
                    AssignTarget::Index { base, .. } => {
                        // Indexing mutates the base
                        if let Some(root_var) = extract_root_var(base) {
                            if param_names.contains(&root_var) {
                                mutation_info.mutated_params.insert(root_var);
                            } else {
                                mutation_info.mutated_locals.insert(root_var);
                            }
                        }
                    }
                    AssignTarget::Attribute { value, .. } => {
                        // Attribute assignment mutates the object
                        if let Some(root_var) = extract_root_var(value) {
                            if param_names.contains(&root_var) {
                                mutation_info.mutated_params.insert(root_var);
                            } else {
                                mutation_info.mutated_locals.insert(root_var);
                            }
                        }
                    }
                    AssignTarget::Tuple(_) => {
                        // Tuple unpacking - for now, skip detailed analysis
                    }
                }

                // Also analyze the value expression for method calls
                self.analyze_expr_for_mutations(value, mutation_info, param_names);
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr_for_mutations(expr, mutation_info, param_names);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr_for_mutations(condition, mutation_info, param_names);
                for stmt in then_body {
                    self.analyze_stmt_for_mutations(stmt, mutation_info, param_names);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        self.analyze_stmt_for_mutations(stmt, mutation_info, param_names);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr_for_mutations(condition, mutation_info, param_names);
                for stmt in body {
                    self.analyze_stmt_for_mutations(stmt, mutation_info, param_names);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr_for_mutations(iter, mutation_info, param_names);
                for stmt in body {
                    self.analyze_stmt_for_mutations(stmt, mutation_info, param_names);
                }
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr_for_mutations(expr, mutation_info, param_names);
            }
            _ => {}
        }
    }

    /// Analyze an expression for mutations (mainly method calls)
    fn analyze_expr_for_mutations(
        &self,
        expr: &HirExpr,
        mutation_info: &mut MutationInfo,
        param_names: &HashSet<String>,
    ) {
        match expr {
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                // Check if method is mutating
                if is_mutating_method(method) {
                    if let Some(root_var) = extract_root_var(object) {
                        if param_names.contains(&root_var) {
                            mutation_info.mutated_params.insert(root_var);
                        } else {
                            mutation_info.mutated_locals.insert(root_var);
                        }
                    }
                }

                // Recurse into object and args
                self.analyze_expr_for_mutations(object, mutation_info, param_names);
                for arg in args {
                    self.analyze_expr_for_mutations(arg, mutation_info, param_names);
                }
            }
            HirExpr::Call { args, .. } => {
                for arg in args {
                    self.analyze_expr_for_mutations(arg, mutation_info, param_names);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr_for_mutations(left, mutation_info, param_names);
                self.analyze_expr_for_mutations(right, mutation_info, param_names);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expr_for_mutations(operand, mutation_info, param_names);
            }
            HirExpr::Attribute { value, .. } => {
                self.analyze_expr_for_mutations(value, mutation_info, param_names);
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr_for_mutations(base, mutation_info, param_names);
                self.analyze_expr_for_mutations(index, mutation_info, param_names);
            }
            _ => {}
        }
    }

    /// Propagate mutations for a specific function by analyzing its calls
    fn propagate_for_function(&mut self, func_name: &str) -> bool {
        let mut changed = false;

        // Get the function
        let Some(func) = self.functions.get(func_name) else {
            return false;
        };

        // Get current mutation info
        let current_mutations = self.mutations.get(func_name).cloned().unwrap_or_default();

        let mut new_mutations = current_mutations.clone();
        let param_names: HashSet<String> = func.params.iter().map(|p| p.name.clone()).collect();

        // Analyze all calls in this function
        for stmt in &func.body {
            if self.propagate_calls_in_stmt(stmt, &mut new_mutations, &param_names) {
                changed = true;
            }
        }

        // Update mutations if changed
        if new_mutations.mutated_params != current_mutations.mutated_params
            || new_mutations.borrowed_params != current_mutations.borrowed_params
        {
            self.mutations.insert(func_name.to_string(), new_mutations);
            changed = true;
        }

        changed
    }

    /// Propagate mutations through calls in a statement
    fn propagate_calls_in_stmt(
        &self,
        stmt: &HirStmt,
        new_mutations: &mut MutationInfo,
        param_names: &HashSet<String>,
    ) -> bool {
        let mut changed = false;

        match stmt {
            HirStmt::Expr(HirExpr::Call { func, args, .. }) => {
                if let Some(callee_sig) = self.registry.get(func) {
                    if let Some(callee_mutations) = self.mutations.get(func) {
                        // For each argument, check if corresponding parameter is mutated in callee
                        for (arg, param) in args.iter().zip(&callee_sig.params) {
                            if callee_mutations.mutated_params.contains(&param.name) {
                                // The argument is passed to a mutating parameter
                                // So we need to mark the root variable as mutated
                                if let Some(root_var) = extract_root_var(arg) {
                                    if param_names.contains(&root_var) {
                                        if !new_mutations.mutated_params.contains(&root_var) {
                                            new_mutations.mutated_params.insert(root_var);
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            HirStmt::Assign { value, .. } => {
                if let HirExpr::Call { func, args, .. } = value {
                    if let Some(callee_sig) = self.registry.get(func) {
                        if let Some(callee_mutations) = self.mutations.get(func) {
                            for (arg, param) in args.iter().zip(&callee_sig.params) {
                                if callee_mutations.mutated_params.contains(&param.name) {
                                    if let Some(root_var) = extract_root_var(arg) {
                                        if param_names.contains(&root_var) {
                                            if !new_mutations.mutated_params.contains(&root_var) {
                                                new_mutations.mutated_params.insert(root_var);
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    changed |= self.propagate_calls_in_stmt(stmt, new_mutations, param_names);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        changed |= self.propagate_calls_in_stmt(stmt, new_mutations, param_names);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for stmt in body {
                    changed |= self.propagate_calls_in_stmt(stmt, new_mutations, param_names);
                }
            }
            _ => {}
        }

        changed
    }
}

/// Check if a method name represents a mutating operation
fn is_mutating_method(method: &str) -> bool {
    matches!(
        method,
        // List methods
        "append" | "extend" | "insert" | "remove" | "pop" | "clear" | "reverse" | "sort" |
        // Dict methods
        "update" | "setdefault" | "popitem" |
        // Set methods
        "add" | "discard" | "difference_update" | "intersection_update" | 
        "symmetric_difference_update" | "union_update" |
        // Other mutating methods
        "push" | "pop_front" | "push_front" | "pop_back" | "push_back"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_info_merge() {
        let mut info1 = MutationInfo::new();
        info1.mutated_params.insert("x".to_string());

        let mut info2 = MutationInfo::new();
        info2.mutated_params.insert("y".to_string());

        info1.merge(&info2);

        assert!(info1.mutated_params.contains("x"));
        assert!(info1.mutated_params.contains("y"));
    }
}
