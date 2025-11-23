//! Call graph construction for interprocedural analysis
//!
//! This module builds a call graph representing function call relationships.
//! The call graph is used to propagate information (mutations, types, etc.)
//! through function boundaries.

use crate::hir::{HirExpr, HirFunction, HirModule, HirStmt};
use crate::interprocedural::signature_registry::FunctionSignatureRegistry;
use std::collections::{HashMap, HashSet, VecDeque};

/// Call graph representing function call relationships
#[derive(Debug, Clone)]
pub struct CallGraph {
    /// Map from function name to set of functions it calls (outgoing edges)
    callees: HashMap<String, HashSet<String>>,
    /// Map from function name to set of functions that call it (incoming edges)
    callers: HashMap<String, HashSet<String>>,
    /// All function names in the graph
    functions: HashSet<String>,
}

impl CallGraph {
    /// Create a new empty call graph
    pub fn new() -> Self {
        Self {
            callees: HashMap::new(),
            callers: HashMap::new(),
            functions: HashSet::new(),
        }
    }

    /// Add a function to the graph
    pub fn add_function(&mut self, func_name: String) {
        self.functions.insert(func_name);
    }

    /// Add a call edge from caller to callee
    pub fn add_call(&mut self, caller: String, callee: String) {
        // Add to callees (caller -> callee)
        self.callees
            .entry(caller.clone())
            .or_insert_with(HashSet::new)
            .insert(callee.clone());

        // Add to callers (callee -> caller)
        self.callers
            .entry(callee)
            .or_insert_with(HashSet::new)
            .insert(caller);
    }

    /// Get all functions called by a given function
    pub fn get_callees(&self, func_name: &str) -> Vec<&str> {
        self.callees
            .get(func_name)
            .map(|set| set.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get all functions that call a given function
    pub fn get_callers(&self, func_name: &str) -> Vec<&str> {
        self.callers
            .get(func_name)
            .map(|set| set.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get all function names in the graph
    pub fn functions(&self) -> impl Iterator<Item = &String> {
        self.functions.iter()
    }

    /// Compute topological order for bottom-up analysis
    /// Returns functions in reverse topological order (callees before callers)
    pub fn topological_order(&self) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();

        // Initialize in-degrees (count how many functions each function calls)
        for func in &self.functions {
            let degree = self.callees.get(func).map(|s| s.len()).unwrap_or(0);
            in_degree.insert(func.clone(), degree);
        }

        // Find functions that don't call anything (leaves in call graph)
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(func, _)| func.clone())
            .collect();

        while let Some(func) = queue.pop_front() {
            result.push(func.clone());

            // For each function that calls this function
            if let Some(callers) = self.callers.get(&func) {
                for caller in callers {
                    if let Some(degree) = in_degree.get_mut(caller) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(caller.clone());
                        }
                    }
                }
            }
        }

        // If not all functions processed, there's a cycle
        // For now, add remaining functions in arbitrary order
        for (func, &degree) in &in_degree {
            if degree > 0 {
                result.push(func.clone());
            }
        }

        result
    }

    /// Detect strongly connected components (cycles in call graph)
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        // Tarjan's algorithm for finding SCCs
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices: HashMap<String, usize> = HashMap::new();
        let mut lowlinks: HashMap<String, usize> = HashMap::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        let mut sccs = Vec::new();

        for func in &self.functions {
            if !indices.contains_key(func) {
                self.tarjan_scc(
                    func,
                    &mut index,
                    &mut stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut sccs,
                );
            }
        }

        // Filter out single-node SCCs (not cycles)
        sccs.into_iter().filter(|scc| scc.len() > 1).collect()
    }

    /// Get all strongly connected components (including single-node SCCs)
    pub fn strongly_connected_components(&self) -> Vec<Vec<String>> {
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices: HashMap<String, usize> = HashMap::new();
        let mut lowlinks: HashMap<String, usize> = HashMap::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        let mut sccs = Vec::new();

        for func in &self.functions {
            if !indices.contains_key(func) {
                self.tarjan_scc(
                    func,
                    &mut index,
                    &mut stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut sccs,
                );
            }
        }

        sccs
    }

    /// Check if a function calls itself (directly or indirectly)
    pub fn calls_itself(&self, func_name: &str) -> bool {
        if let Some(callees) = self.callees.get(func_name) {
            if callees.contains(func_name) {
                // Direct self-recursion
                return true;
            }

            // Check for indirect recursion
            let mut visited = HashSet::new();
            self.has_path_to(func_name, func_name, &mut visited)
        } else {
            false
        }
    }

    /// Check if there's a path from start to end
    fn has_path_to(&self, start: &str, end: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(start) {
            return false;
        }
        visited.insert(start.to_string());

        if let Some(callees) = self.callees.get(start) {
            for callee in callees {
                if callee == end {
                    return true;
                }
                if self.has_path_to(callee, end, visited) {
                    return true;
                }
            }
        }

        false
    }

    /// Get reverse topological order (for bottom-up analysis)
    pub fn reverse_topological_order(&self) -> Vec<String> {
        self.topological_order()
    }

    fn tarjan_scc(
        &self,
        func: &str,
        index: &mut usize,
        stack: &mut Vec<String>,
        indices: &mut HashMap<String, usize>,
        lowlinks: &mut HashMap<String, usize>,
        on_stack: &mut HashSet<String>,
        sccs: &mut Vec<Vec<String>>,
    ) {
        indices.insert(func.to_string(), *index);
        lowlinks.insert(func.to_string(), *index);
        *index += 1;
        stack.push(func.to_string());
        on_stack.insert(func.to_string());

        // Consider successors
        if let Some(callees) = self.callees.get(func) {
            for callee in callees {
                if !indices.contains_key(callee) {
                    // Successor not yet visited, recurse
                    self.tarjan_scc(callee, index, stack, indices, lowlinks, on_stack, sccs);
                    let callee_lowlink = *lowlinks.get(callee).unwrap();
                    let func_lowlink = lowlinks.get_mut(func).unwrap();
                    *func_lowlink = (*func_lowlink).min(callee_lowlink);
                } else if on_stack.contains(callee) {
                    // Successor is on stack, hence in current SCC
                    let callee_index = *indices.get(callee).unwrap();
                    let func_lowlink = lowlinks.get_mut(func).unwrap();
                    *func_lowlink = (*func_lowlink).min(callee_index);
                }
            }
        }

        // If func is a root node, pop the stack to get SCC
        if lowlinks.get(func) == indices.get(func) {
            let mut scc = Vec::new();
            loop {
                let node = stack.pop().unwrap();
                on_stack.remove(&node);
                scc.push(node.clone());
                if node == func {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing call graphs from HIR modules
pub struct CallGraphBuilder<'a> {
    registry: &'a FunctionSignatureRegistry,
    graph: CallGraph,
}

impl<'a> CallGraphBuilder<'a> {
    /// Create a new call graph builder
    pub fn new(registry: &'a FunctionSignatureRegistry) -> Self {
        Self {
            registry,
            graph: CallGraph::new(),
        }
    }

    /// Build the call graph from a HIR module
    pub fn build(mut self, module: &HirModule) -> CallGraph {
        // Add all functions to the graph
        for func in &module.functions {
            self.graph.add_function(func.name.clone());
        }

        // Analyze each function to find calls
        for func in &module.functions {
            self.analyze_function(func);
        }

        self.graph
    }

    /// Analyze a function to find all calls it makes
    fn analyze_function(&mut self, func: &HirFunction) {
        let caller = &func.name;

        for stmt in &func.body {
            self.analyze_stmt(caller, stmt);
        }
    }

    /// Analyze a statement to find calls
    fn analyze_stmt(&mut self, caller: &str, stmt: &HirStmt) {
        match stmt {
            HirStmt::Assign { value, .. } => {
                self.analyze_expr(caller, value);
            }
            HirStmt::Return(Some(expr)) => {
                self.analyze_expr(caller, expr);
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(caller, condition);
                for stmt in then_body {
                    self.analyze_stmt(caller, stmt);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        self.analyze_stmt(caller, stmt);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr(caller, condition);
                for stmt in body {
                    self.analyze_stmt(caller, stmt);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr(caller, iter);
                for stmt in body {
                    self.analyze_stmt(caller, stmt);
                }
            }
            HirStmt::Expr(expr) => {
                self.analyze_expr(caller, expr);
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for stmt in body {
                    self.analyze_stmt(caller, stmt);
                }
                for handler in handlers {
                    for stmt in &handler.body {
                        self.analyze_stmt(caller, stmt);
                    }
                }
                if let Some(orelse) = orelse {
                    for stmt in orelse {
                        self.analyze_stmt(caller, stmt);
                    }
                }
                if let Some(finalbody) = finalbody {
                    for stmt in finalbody {
                        self.analyze_stmt(caller, stmt);
                    }
                }
            }
            _ => {}
        }
    }

    /// Analyze an expression to find calls
    fn analyze_expr(&mut self, caller: &str, expr: &HirExpr) {
        match expr {
            HirExpr::Call { func, args, .. } => {
                // Record the call edge if callee is in registry
                if self.registry.contains(func) {
                    self.graph.add_call(caller.to_string(), func.clone());
                }
                // Analyze arguments recursively
                for arg in args {
                    self.analyze_expr(caller, arg);
                }
            }
            HirExpr::MethodCall { object, args, .. } => {
                self.analyze_expr(caller, object);
                for arg in args {
                    self.analyze_expr(caller, arg);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr(caller, left);
                self.analyze_expr(caller, right);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expr(caller, operand);
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr(caller, base);
                self.analyze_expr(caller, index);
            }
            HirExpr::Attribute { value, .. } => {
                self.analyze_expr(caller, value);
            }
            HirExpr::List(exprs) | HirExpr::Tuple(exprs) | HirExpr::Set(exprs) => {
                for expr in exprs {
                    self.analyze_expr(caller, expr);
                }
            }
            HirExpr::Dict(pairs) => {
                for (key, value) in pairs {
                    self.analyze_expr(caller, key);
                    self.analyze_expr(caller, value);
                }
            }
            HirExpr::Borrow { expr, .. } => {
                self.analyze_expr(caller, expr);
            }
            HirExpr::ListComp { element, iter, .. } => {
                self.analyze_expr(caller, element);
                self.analyze_expr(caller, iter);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_graph_basic() {
        let mut graph = CallGraph::new();

        graph.add_function("main".to_string());
        graph.add_function("helper".to_string());
        graph.add_call("main".to_string(), "helper".to_string());

        let callees = graph.get_callees("main");
        assert_eq!(callees, vec!["helper"]);

        let callers = graph.get_callers("helper");
        assert_eq!(callers, vec!["main"]);
    }

    #[test]
    fn test_topological_order() {
        let mut graph = CallGraph::new();

        // Create call chain: main -> helper1 -> helper2
        graph.add_function("main".to_string());
        graph.add_function("helper1".to_string());
        graph.add_function("helper2".to_string());
        graph.add_call("main".to_string(), "helper1".to_string());
        graph.add_call("helper1".to_string(), "helper2".to_string());

        let order = graph.topological_order();

        // helper2 should come before helper1, helper1 before main
        let helper2_pos = order.iter().position(|f| f == "helper2").unwrap();
        let helper1_pos = order.iter().position(|f| f == "helper1").unwrap();
        let main_pos = order.iter().position(|f| f == "main").unwrap();

        assert!(helper2_pos < helper1_pos);
        assert!(helper1_pos < main_pos);
    }
}
