//! Call site analysis for propagating borrowing and mutation requirements
//!
//! This module analyzes function call sites to determine how arguments are used
//! and propagates mutation/borrowing requirements back to the caller.

use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt};
use crate::interprocedural::signature_registry::{FunctionSignatureRegistry, ParamSignature};
use crate::interprocedural::BorrowKind;
use std::collections::{HashMap, HashSet};

/// Analyzes function calls to propagate borrowing requirements
pub struct CallSiteAnalyzer<'a> {
    /// Function signature registry
    registry: &'a FunctionSignatureRegistry,
    /// Current function being analyzed
    current_function: Option<String>,
    /// Tracking mutations propagated through calls
    propagated_mutations: HashMap<String, HashSet<String>>,
}

impl<'a> CallSiteAnalyzer<'a> {
    /// Create a new call site analyzer
    pub fn new(registry: &'a FunctionSignatureRegistry) -> Self {
        Self {
            registry,
            current_function: None,
            propagated_mutations: HashMap::new(),
        }
    }
    
    /// Analyze all function calls in a function
    pub fn analyze_function(&mut self, func: &HirFunction) -> CallAnalysisResult {
        self.current_function = Some(func.name.clone());
        let mut required_mutable_vars = HashSet::new();
        let mut borrow_insertions = HashMap::new();
        
        for stmt in &func.body {
            self.analyze_stmt_for_calls(
                stmt,
                &mut required_mutable_vars,
                &mut borrow_insertions,
            );
        }
        
        CallAnalysisResult {
            required_mutable_vars,
            borrow_insertions,
        }
    }
    
    /// Analyze a statement for function calls
    fn analyze_stmt_for_calls(
        &mut self,
        stmt: &HirStmt,
        required_mutable_vars: &mut HashSet<String>,
        borrow_insertions: &mut HashMap<usize, Vec<BorrowInsertion>>,
    ) {
        match stmt {
            HirStmt::Assign { value, target, .. } => {
                self.analyze_expr_for_calls(value, required_mutable_vars, borrow_insertions);
                
                // Check if assignment target involves mutation
                if let AssignTarget::Index { base, .. } = target {
                    if let Some(root_var) = extract_root_var(base) {
                        required_mutable_vars.insert(root_var);
                    }
                }
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr_for_calls(expr, required_mutable_vars, borrow_insertions);
            }
            HirStmt::If { condition, then_body, else_body } => {
                self.analyze_expr_for_calls(condition, required_mutable_vars, borrow_insertions);
                for stmt in then_body {
                    self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr_for_calls(condition, required_mutable_vars, borrow_insertions);
                for stmt in body {
                    self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr_for_calls(iter, required_mutable_vars, borrow_insertions);
                for stmt in body {
                    self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                }
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr_for_calls(expr, required_mutable_vars, borrow_insertions);
            }
            HirStmt::Try { body, handlers, .. } => {
                for stmt in body {
                    self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                }
                for handler in handlers {
                    for stmt in &handler.body {
                        self.analyze_stmt_for_calls(stmt, required_mutable_vars, borrow_insertions);
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Analyze an expression for function calls
    fn analyze_expr_for_calls(
        &mut self,
        expr: &HirExpr,
        required_mutable_vars: &mut HashSet<String>,
        borrow_insertions: &mut HashMap<usize, Vec<BorrowInsertion>>,
    ) {
        match expr {
            HirExpr::Call { func, args, .. } => {
                self.analyze_call(func, args, required_mutable_vars, borrow_insertions);
            }
            HirExpr::MethodCall { object, method, args, .. } => {
                // Check if method is mutating
                if is_mutating_method(method) {
                    if let Some(root_var) = extract_root_var(object) {
                        required_mutable_vars.insert(root_var);
                    }
                }
                
                self.analyze_expr_for_calls(object, required_mutable_vars, borrow_insertions);
                for arg in args {
                    self.analyze_expr_for_calls(arg, required_mutable_vars, borrow_insertions);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr_for_calls(left, required_mutable_vars, borrow_insertions);
                self.analyze_expr_for_calls(right, required_mutable_vars, borrow_insertions);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expr_for_calls(operand, required_mutable_vars, borrow_insertions);
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr_for_calls(base, required_mutable_vars, borrow_insertions);
                self.analyze_expr_for_calls(index, required_mutable_vars, borrow_insertions);
            }
            HirExpr::Attribute { value, .. } => {
                self.analyze_expr_for_calls(value, required_mutable_vars, borrow_insertions);
            }
            HirExpr::List(exprs) | HirExpr::Tuple(exprs) | HirExpr::Set(exprs) => {
                for expr in exprs {
                    self.analyze_expr_for_calls(expr, required_mutable_vars, borrow_insertions);
                }
            }
            HirExpr::Dict(pairs) => {
                for (key, value) in pairs {
                    self.analyze_expr_for_calls(key, required_mutable_vars, borrow_insertions);
                    self.analyze_expr_for_calls(value, required_mutable_vars, borrow_insertions);
                }
            }
            HirExpr::Borrow { expr, .. } => {
                self.analyze_expr_for_calls(expr, required_mutable_vars, borrow_insertions);
            }
            _ => {}
        }
    }
    
    /// Analyze a specific function call
    fn analyze_call(
        &mut self,
        func_name: &str,
        args: &[HirExpr],
        required_mutable_vars: &mut HashSet<String>,
        borrow_insertions: &mut HashMap<usize, Vec<BorrowInsertion>>,
    ) {
        // Look up the callee signature
        if let Some(callee_sig) = self.registry.get(func_name) {
            // Match arguments to parameters
            for (arg_index, (arg, param)) in args.iter().zip(&callee_sig.params).enumerate() {
                // If parameter is mutated in callee, propagate to caller
                if param.is_mutated {
                    self.propagate_mutation(arg, required_mutable_vars);
                    
                    // Record that we need to insert &mut at call site
                    borrow_insertions
                        .entry(arg_index)
                        .or_insert_with(Vec::new)
                        .push(BorrowInsertion {
                            arg_index,
                            kind: BorrowKind::MutableBorrow,
                        });
                }
            }
        }
    }
    
    /// Propagate mutation requirement through field access chain
    fn propagate_mutation(
        &mut self,
        arg: &HirExpr,
        required_mutable_vars: &mut HashSet<String>,
    ) {
        // Extract the root variable and field path
        // e.g., state.data -> root: "state", path: ["data"]
        if let Some(root_var) = extract_root_var(arg) {
            // If we're passing a field to a mutating function,
            // the root variable must be mutable
            required_mutable_vars.insert(root_var);
        }
    }
}

/// Extract the root variable name from an expression
fn extract_root_var(expr: &HirExpr) -> Option<String> {
    match expr {
        HirExpr::Var(name) => Some(name.clone()),
        HirExpr::Attribute { value, .. } => extract_root_var(value),
        HirExpr::Index { base, .. } => extract_root_var(base),
        _ => None,
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

/// Result of analyzing call sites in a function
#[derive(Debug, Clone)]
pub struct CallAnalysisResult {
    /// Variables that need to be mutable in caller
    pub required_mutable_vars: HashSet<String>,
    /// Borrow operators to insert at call site (indexed by statement/call position)
    pub borrow_insertions: HashMap<usize, Vec<BorrowInsertion>>,
}

/// Information about a borrow operator to insert
#[derive(Debug, Clone)]
pub struct BorrowInsertion {
    /// Index of the argument
    pub arg_index: usize,
    /// Kind of borrow to insert
    pub kind: BorrowKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{HirFunction, FunctionProperties, HirParam, Type as PythonType};
    use crate::interprocedural::signature_registry::FunctionSignatureRegistry;
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;
    
    #[test]
    fn test_extract_root_var() {
        let var_expr = HirExpr::Var("state".to_string());
        assert_eq!(extract_root_var(&var_expr), Some("state".to_string()));
        
        let attr_expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("state".to_string())),
            attr: "data".to_string(),
        };
        assert_eq!(extract_root_var(&attr_expr), Some("state".to_string()));
    }
    
    #[test]
    fn test_is_mutating_method() {
        assert!(is_mutating_method("append"));
        assert!(is_mutating_method("insert"));
        assert!(!is_mutating_method("get"));
        assert!(!is_mutating_method("keys"));
    }
}
