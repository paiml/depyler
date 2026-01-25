//! DEPYLER-0950: Inter-Procedural Type Unification
//!
//! Implements call graph-based type inference to eliminate E0308 errors.
//!
//! # Architecture
//!
//! 1. Build call graph from HIR module
//! 2. Extract type constraints from each function
//! 3. Propagate constraints across function boundaries
//! 4. Resolve to concrete types with coercion lattice
//!
//! # Toyota Way Principles
//!
//! - **Jidoka**: Auto-detect type conflicts and insert casts
//! - **Genchi Genbutsu**: Build call graph from actual code
//! - **Poka-Yoke**: Constraints prevent impossible combinations

use crate::hir::{AssignTarget, HirExpr, HirFunction, HirModule, HirStmt, Literal, Type};
use std::collections::HashMap;

// =============================================================================
// Type Variables and Constraints
// =============================================================================

/// Type variable identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);

/// Concrete types that can be unified
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    StrRef,
    Unit,
    Vec(Box<ConcreteType>),
    Option(Box<ConcreteType>),
    HashMap(Box<ConcreteType>, Box<ConcreteType>),
    /// Unknown/unresolved type
    Unknown,
}

impl ConcreteType {
    /// Convert from HIR Type
    pub fn from_hir_type(ty: &Type) -> Self {
        match ty {
            Type::Int => ConcreteType::I64,
            Type::Float => ConcreteType::F64,
            Type::Bool => ConcreteType::Bool,
            Type::String => ConcreteType::String,
            Type::None => ConcreteType::Unit,
            Type::List(inner) => ConcreteType::Vec(Box::new(Self::from_hir_type(inner))),
            Type::Optional(inner) => ConcreteType::Option(Box::new(Self::from_hir_type(inner))),
            Type::Dict(k, v) => ConcreteType::HashMap(
                Box::new(Self::from_hir_type(k)),
                Box::new(Self::from_hir_type(v)),
            ),
            Type::Unknown => ConcreteType::Unknown,
            _ => ConcreteType::Unknown,
        }
    }

    /// Convert to HIR Type
    pub fn to_hir_type(&self) -> Type {
        match self {
            ConcreteType::I32 | ConcreteType::I64 => Type::Int,
            ConcreteType::F32 | ConcreteType::F64 => Type::Float,
            ConcreteType::Bool => Type::Bool,
            ConcreteType::String | ConcreteType::StrRef => Type::String,
            ConcreteType::Unit => Type::None,
            ConcreteType::Vec(inner) => Type::List(Box::new(inner.to_hir_type())),
            ConcreteType::Option(inner) => Type::Optional(Box::new(inner.to_hir_type())),
            ConcreteType::HashMap(k, v) => {
                Type::Dict(Box::new(k.to_hir_type()), Box::new(v.to_hir_type()))
            }
            ConcreteType::Unknown => Type::Unknown,
        }
    }
}

/// Type constraint from code analysis
#[derive(Debug, Clone)]
pub enum Constraint {
    /// α = β (type equality)
    Equal(TypeVar, TypeVar),
    /// α = T (type assignment)
    Assign(TypeVar, ConcreteType),
    /// Call f(α₁, α₂, ...) → β
    Call {
        callee: String,
        args: Vec<TypeVar>,
        ret: TypeVar,
    },
}

// =============================================================================
// Call Graph
// =============================================================================

/// Node ID in call graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// Call graph for inter-procedural analysis
#[derive(Debug, Default)]
pub struct CallGraph {
    /// Function name → node ID
    pub fn_to_node: HashMap<String, NodeId>,
    /// Node ID → function name
    pub node_to_fn: HashMap<NodeId, String>,
    /// Caller → callees
    pub edges: HashMap<NodeId, Vec<NodeId>>,
    /// Callee → callers (reverse edges)
    pub reverse_edges: HashMap<NodeId, Vec<NodeId>>,
    /// Next node ID
    next_id: u32,
}

impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a function node
    pub fn add_function(&mut self, name: &str) -> NodeId {
        if let Some(&id) = self.fn_to_node.get(name) {
            return id;
        }
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.fn_to_node.insert(name.to_string(), id);
        self.node_to_fn.insert(id, name.to_string());
        self.edges.insert(id, Vec::new());
        self.reverse_edges.insert(id, Vec::new());
        id
    }

    /// Add a call edge
    pub fn add_call(&mut self, caller: NodeId, callee: NodeId) {
        if let Some(callees) = self.edges.get_mut(&caller) {
            if !callees.contains(&callee) {
                callees.push(callee);
            }
        }
        if let Some(callers) = self.reverse_edges.get_mut(&callee) {
            if !callers.contains(&caller) {
                callers.push(caller);
            }
        }
    }

    /// Get callees of a function
    pub fn callees(&self, node: NodeId) -> &[NodeId] {
        self.edges.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get callers of a function
    pub fn callers(&self, node: NodeId) -> &[NodeId] {
        self.reverse_edges
            .get(&node)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Topological sort (callees before callers)
    pub fn topological_order(&self) -> Vec<NodeId> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        for &node in self.fn_to_node.values() {
            in_degree.insert(node, 0);
        }

        for callees in self.edges.values() {
            for &callee in callees {
                *in_degree.entry(callee).or_default() += 1;
            }
        }

        let mut queue: Vec<NodeId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop() {
            result.push(node);
            for &callee in self.callees(node) {
                if let Some(deg) = in_degree.get_mut(&callee) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push(callee);
                    }
                }
            }
        }

        result
    }
}

// =============================================================================
// Union-Find for Type Unification
// =============================================================================

/// Union-Find data structure with path compression
#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    /// Resolved concrete type for each root
    pub resolved: HashMap<usize, ConcreteType>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            resolved: HashMap::new(),
        }
    }

    /// Find root with path compression
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Union two sets, returns Ok(()) or Err if types conflict
    pub fn union(&mut self, x: usize, y: usize) -> Result<(), UnifyError> {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return Ok(());
        }

        // Check for type conflicts
        match (
            self.resolved.get(&rx).cloned(),
            self.resolved.get(&ry).cloned(),
        ) {
            (Some(tx), Some(ty)) if tx != ty => {
                // Try coercion
                if let Some(common) = coerce_types(&tx, &ty) {
                    // Update both to common type
                    self.resolved.insert(rx, common.clone());
                    self.resolved.insert(ry, common);
                } else {
                    return Err(UnifyError::TypeConflict(tx, ty));
                }
            }
            (Some(t), None) => {
                self.resolved.insert(ry, t);
            }
            (None, Some(t)) => {
                self.resolved.insert(rx, t);
            }
            _ => {}
        }

        // Union by rank
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        Ok(())
    }

    /// Assign a concrete type to a type variable
    pub fn assign(&mut self, var: usize, ty: ConcreteType) -> Result<(), UnifyError> {
        let root = self.find(var);
        if let Some(existing) = self.resolved.get(&root).cloned() {
            if existing != ty {
                if let Some(common) = coerce_types(&existing, &ty) {
                    self.resolved.insert(root, common);
                } else {
                    // Type conflict - cannot coerce
                    // NOTE: String→numeric override removed (caused DEPYLER-0302 regression)
                    return Err(UnifyError::TypeConflict(existing, ty));
                }
            }
        } else {
            self.resolved.insert(root, ty);
        }
        Ok(())
    }

    /// Get resolved type for a variable
    pub fn get_type(&mut self, var: usize) -> Option<ConcreteType> {
        let root = self.find(var);
        self.resolved.get(&root).cloned()
    }
}

/// Error during type unification
#[derive(Debug, Clone)]
pub enum UnifyError {
    TypeConflict(ConcreteType, ConcreteType),
}

// =============================================================================
// Numeric Coercion Lattice
// =============================================================================

/// Find common type for two numeric types (widening)
pub fn coerce_types(a: &ConcreteType, b: &ConcreteType) -> Option<ConcreteType> {
    use ConcreteType::*;

    // Same type - no coercion needed
    if a == b {
        return Some(a.clone());
    }

    // Numeric coercion lattice
    match (a, b) {
        // Int hierarchy
        (I32, I64) | (I64, I32) => Some(I64),

        // Float hierarchy
        (F32, F64) | (F64, F32) => Some(F64),

        // Int to float promotion
        (I32, F32) | (F32, I32) => Some(F32),
        (I32, F64) | (F64, I32) => Some(F64),
        (I64, F32) | (F32, I64) => Some(F64), // i64 + f32 → f64 (precision)
        (I64, F64) | (F64, I64) => Some(F64),

        // String coercion
        (String, StrRef) | (StrRef, String) => Some(String),

        // Unknown accepts anything
        (Unknown, other) | (other, Unknown) => Some(other.clone()),

        // No coercion possible
        _ => None,
    }
}

// =============================================================================
// Type Unifier - Main Entry Point
// =============================================================================

/// Function signature for type analysis
#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub param_vars: Vec<TypeVar>,
    pub ret_var: TypeVar,
}

/// Type unifier for a module
pub struct TypeUnifier {
    /// Call graph
    pub call_graph: CallGraph,
    /// Function signatures
    pub signatures: HashMap<String, FunctionSig>,
    /// All constraints collected
    pub constraints: Vec<Constraint>,
    /// Union-Find for unification
    pub uf: UnionFind,
    /// Next type variable ID
    next_var: u32,
    /// Variable name to type var mapping per function
    pub var_map: HashMap<String, HashMap<String, TypeVar>>,
}

impl TypeUnifier {
    /// Create a new type unifier for a module
    pub fn new() -> Self {
        Self {
            call_graph: CallGraph::new(),
            signatures: HashMap::new(),
            constraints: Vec::new(),
            uf: UnionFind::new(10000), // Pre-allocate
            next_var: 0,
            var_map: HashMap::new(),
        }
    }

    /// Allocate a new type variable
    fn fresh_var(&mut self) -> TypeVar {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        var
    }

    /// Build call graph and extract constraints from module
    pub fn analyze_module(&mut self, module: &HirModule) {
        // Phase 1: Register all functions
        for func in &module.functions {
            self.call_graph.add_function(&func.name);
            self.register_function(func);
        }

        // Phase 2: Extract call edges and constraints
        for func in &module.functions {
            self.analyze_function(func);
        }
    }

    /// Register function signature
    fn register_function(&mut self, func: &HirFunction) {
        let mut param_vars = Vec::new();
        let mut local_vars: HashMap<String, TypeVar> = HashMap::new();

        for param in &func.params {
            let var = self.fresh_var();
            param_vars.push(var);
            local_vars.insert(param.name.clone(), var);

            // If parameter has type annotation, add constraint
            if param.ty != Type::Unknown {
                let concrete = ConcreteType::from_hir_type(&param.ty);
                self.constraints.push(Constraint::Assign(var, concrete));
            }
        }

        let ret_var = self.fresh_var();

        // If return type is annotated, add constraint
        if func.ret_type != Type::Unknown {
            let concrete = ConcreteType::from_hir_type(&func.ret_type);
            self.constraints.push(Constraint::Assign(ret_var, concrete));
        }

        let sig = FunctionSig {
            name: func.name.clone(),
            param_vars,
            ret_var,
        };

        self.signatures.insert(func.name.clone(), sig);
        self.var_map.insert(func.name.clone(), local_vars);
    }

    /// Analyze function body for calls and constraints
    fn analyze_function(&mut self, func: &HirFunction) {
        let caller_id = self.call_graph.fn_to_node[&func.name];

        for stmt in &func.body {
            self.analyze_stmt(stmt, &func.name, caller_id);
        }
    }

    /// Analyze statement for calls
    fn analyze_stmt(&mut self, stmt: &HirStmt, current_fn: &str, caller_id: NodeId) {
        match stmt {
            HirStmt::Expr(expr) => {
                self.analyze_expr(expr, current_fn, caller_id);
            }
            HirStmt::Return(Some(expr)) => {
                // DEPYLER-0950: Link return expression type to function return type
                let expr_var = self.analyze_expr(expr, current_fn, caller_id);
                if let Some(ev) = expr_var {
                    if let Some(sig) = self.signatures.get(current_fn) {
                        self.constraints.push(Constraint::Equal(sig.ret_var, ev));
                    }
                }
            }
            HirStmt::Assign { target, value, .. } => {
                if let Some(name) = self.extract_assign_target_name(target) {
                    let var = self.get_or_create_var(current_fn, &name);
                    let expr_var = self.analyze_expr(value, current_fn, caller_id);
                    if let Some(ev) = expr_var {
                        self.constraints.push(Constraint::Equal(var, ev));
                    }
                } else {
                    // Complex target - just analyze the value
                    self.analyze_expr(value, current_fn, caller_id);
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr(condition, current_fn, caller_id);
                for s in then_body {
                    self.analyze_stmt(s, current_fn, caller_id);
                }
                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.analyze_stmt(s, current_fn, caller_id);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr(condition, current_fn, caller_id);
                for s in body {
                    self.analyze_stmt(s, current_fn, caller_id);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr(iter, current_fn, caller_id);
                for s in body {
                    self.analyze_stmt(s, current_fn, caller_id);
                }
            }
            _ => {}
        }
    }

    /// Extract simple variable name from AssignTarget
    fn extract_assign_target_name(&self, target: &AssignTarget) -> Option<String> {
        match target {
            AssignTarget::Symbol(sym) => Some(sym.clone()),
            _ => None, // Complex targets (index, attribute, tuple) not tracked
        }
    }

    /// Analyze expression, return type variable if applicable
    fn analyze_expr(
        &mut self,
        expr: &HirExpr,
        current_fn: &str,
        caller_id: NodeId,
    ) -> Option<TypeVar> {
        match expr {
            HirExpr::Literal(lit) => {
                let var = self.fresh_var();
                let ty = match lit {
                    Literal::Int(_) => ConcreteType::I64,
                    Literal::Float(_) => ConcreteType::F64,
                    Literal::Bool(_) => ConcreteType::Bool,
                    Literal::String(_) => ConcreteType::String,
                    Literal::None => ConcreteType::Unit,
                    _ => ConcreteType::Unknown,
                };
                self.constraints.push(Constraint::Assign(var, ty));
                Some(var)
            }
            HirExpr::Var(name) => Some(self.get_or_create_var(current_fn, name)),
            HirExpr::Call { func, args, .. } => {
                // Check if calling a known function
                if let Some(callee_id) = self.call_graph.fn_to_node.get(func).copied() {
                    self.call_graph.add_call(caller_id, callee_id);

                    // Create call constraint
                    let arg_vars: Vec<TypeVar> = args
                        .iter()
                        .filter_map(|a| self.analyze_expr(a, current_fn, caller_id))
                        .collect();

                    let ret_var = self.fresh_var();

                    self.constraints.push(Constraint::Call {
                        callee: func.clone(),
                        args: arg_vars,
                        ret: ret_var,
                    });

                    Some(ret_var)
                } else {
                    // External function - analyze args but don't track
                    for arg in args {
                        self.analyze_expr(arg, current_fn, caller_id);
                    }
                    None
                }
            }
            HirExpr::Binary { left, right, .. } => {
                let left_var = self.analyze_expr(left, current_fn, caller_id);
                let right_var = self.analyze_expr(right, current_fn, caller_id);

                // Binary op result unifies left and right
                let result_var = self.fresh_var();
                if let Some(lv) = left_var {
                    self.constraints.push(Constraint::Equal(result_var, lv));
                }
                if let Some(rv) = right_var {
                    self.constraints.push(Constraint::Equal(result_var, rv));
                }
                Some(result_var)
            }
            HirExpr::Unary { operand, .. } => self.analyze_expr(operand, current_fn, caller_id),
            HirExpr::MethodCall { object, args, .. } => {
                self.analyze_expr(object, current_fn, caller_id);
                for arg in args {
                    self.analyze_expr(arg, current_fn, caller_id);
                }
                None // Method return types handled elsewhere
            }
            HirExpr::List(items) => {
                for item in items {
                    self.analyze_expr(item, current_fn, caller_id);
                }
                None
            }
            _ => None,
        }
    }

    /// Get or create a type variable for a local variable
    fn get_or_create_var(&mut self, func_name: &str, var_name: &str) -> TypeVar {
        // Check if already exists
        if let Some(local_vars) = self.var_map.get(func_name) {
            if let Some(&var) = local_vars.get(var_name) {
                return var;
            }
        }
        // Create new variable
        let var = self.fresh_var();
        self.var_map
            .entry(func_name.to_string())
            .or_default()
            .insert(var_name.to_string(), var);
        var
    }

    /// Solve all constraints
    pub fn solve(&mut self) -> Result<(), UnifyError> {
        // Iterate until fixpoint
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            for constraint in self.constraints.clone() {
                match constraint {
                    Constraint::Equal(a, b) => {
                        let ra = self.uf.find(a.0 as usize);
                        let rb = self.uf.find(b.0 as usize);
                        if ra != rb {
                            self.uf.union(a.0 as usize, b.0 as usize)?;
                            changed = true;
                        }
                    }
                    Constraint::Assign(v, ty) => {
                        let root = self.uf.find(v.0 as usize);
                        if !self.uf.resolved.contains_key(&root) {
                            self.uf.assign(v.0 as usize, ty)?;
                            changed = true;
                        }
                    }
                    Constraint::Call { callee, args, ret } => {
                        if let Some(sig) = self.signatures.get(&callee).cloned() {
                            // Unify arguments with parameters
                            for (arg_var, param_var) in args.iter().zip(&sig.param_vars) {
                                let ra = self.uf.find(arg_var.0 as usize);
                                let rp = self.uf.find(param_var.0 as usize);
                                if ra != rp {
                                    self.uf.union(arg_var.0 as usize, param_var.0 as usize)?;
                                    changed = true;
                                }
                            }
                            // Unify return type
                            let rr = self.uf.find(ret.0 as usize);
                            let rs = self.uf.find(sig.ret_var.0 as usize);
                            if rr != rs {
                                self.uf.union(ret.0 as usize, sig.ret_var.0 as usize)?;
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get resolved parameter types for a function
    pub fn get_param_types(&mut self, func_name: &str) -> Vec<Type> {
        if let Some(sig) = self.signatures.get(func_name) {
            sig.param_vars
                .iter()
                .map(|var| {
                    self.uf
                        .get_type(var.0 as usize)
                        .map(|t| t.to_hir_type())
                        .unwrap_or(Type::Unknown)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get resolved return type for a function
    pub fn get_return_type(&mut self, func_name: &str) -> Type {
        if let Some(sig) = self.signatures.get(func_name) {
            self.uf
                .get_type(sig.ret_var.0 as usize)
                .map(|t| t.to_hir_type())
                .unwrap_or(Type::Unknown)
        } else {
            Type::Unknown
        }
    }
}

impl Default for TypeUnifier {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Module Integration
// =============================================================================

/// Determine if call-site type should override existing type
///
/// Call-site evidence wins when:
/// 1. Going from Int to Float (widening based on actual usage)
/// 2. Going from Unknown to anything
///
/// NOTE: String → Int/Float override was removed (caused DEPYLER-0302 regression).
/// Call-site override for heuristic strings is handled in lib.rs propagate_call_site_types.
fn should_override_type(existing: &Type, new: &Type) -> bool {
    match (existing, new) {
        // Unknown should always be updated to concrete type
        (Type::Unknown, _) => true,
        // Int → Float is valid widening from call-site evidence
        (Type::Int, Type::Float) => true,
        // Same type - no override needed
        (a, b) if a == b => false,
        _ => false,
    }
}

/// Apply type unification to a module, updating function signatures
pub fn unify_module_types(module: &mut HirModule) -> Result<(), UnifyError> {
    let mut unifier = TypeUnifier::new();

    // Analyze and solve
    unifier.analyze_module(module);
    unifier.solve()?;

    // Update function signatures with resolved types
    // DEPYLER-0950: Call-site evidence can override local inference
    for func in &mut module.functions {
        // Update parameter types
        if let Some(sig) = unifier.signatures.get(&func.name) {
            for (param, var) in func.params.iter_mut().zip(&sig.param_vars) {
                if let Some(resolved) = unifier.uf.get_type(var.0 as usize) {
                    let resolved_hir = resolved.to_hir_type();
                    // Update if currently Unknown, or if call-site gives more specific type
                    // (e.g., Float from call site is more specific than Int from local `* 2`)
                    if param.ty == Type::Unknown || should_override_type(&param.ty, &resolved_hir) {
                        param.ty = resolved_hir;
                    }
                }
            }

            // Update return type
            if let Some(resolved) = unifier.uf.get_type(sig.ret_var.0 as usize) {
                let resolved_hir = resolved.to_hir_type();
                if func.ret_type == Type::Unknown
                    || should_override_type(&func.ret_type, &resolved_hir)
                {
                    func.ret_type = resolved_hir;
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // TypeVar tests
    // =========================================================================

    #[test]
    fn test_type_var_creation() {
        let tv = TypeVar(0);
        assert_eq!(tv.0, 0);
    }

    #[test]
    fn test_type_var_equality() {
        let tv1 = TypeVar(5);
        let tv2 = TypeVar(5);
        let tv3 = TypeVar(10);
        assert_eq!(tv1, tv2);
        assert_ne!(tv1, tv3);
    }

    #[test]
    fn test_type_var_clone() {
        let tv = TypeVar(42);
        let cloned = tv;
        assert_eq!(tv, cloned);
    }

    #[test]
    fn test_type_var_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TypeVar(1));
        set.insert(TypeVar(2));
        set.insert(TypeVar(1)); // duplicate
        assert_eq!(set.len(), 2);
    }

    // =========================================================================
    // NodeId tests
    // =========================================================================

    #[test]
    fn test_node_id_creation() {
        let n = NodeId(0);
        assert_eq!(n.0, 0);
    }

    #[test]
    fn test_node_id_equality() {
        let n1 = NodeId(5);
        let n2 = NodeId(5);
        let n3 = NodeId(10);
        assert_eq!(n1, n2);
        assert_ne!(n1, n3);
    }

    #[test]
    fn test_node_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NodeId(1));
        set.insert(NodeId(2));
        assert_eq!(set.len(), 2);
    }

    // =========================================================================
    // ConcreteType tests
    // =========================================================================

    #[test]
    fn test_concrete_type_i32() {
        let t = ConcreteType::I32;
        assert_eq!(t, ConcreteType::I32);
    }

    #[test]
    fn test_concrete_type_i64() {
        let t = ConcreteType::I64;
        assert_eq!(t, ConcreteType::I64);
    }

    #[test]
    fn test_concrete_type_f32() {
        let t = ConcreteType::F32;
        assert_eq!(t, ConcreteType::F32);
    }

    #[test]
    fn test_concrete_type_f64() {
        let t = ConcreteType::F64;
        assert_eq!(t, ConcreteType::F64);
    }

    #[test]
    fn test_concrete_type_bool() {
        let t = ConcreteType::Bool;
        assert_eq!(t, ConcreteType::Bool);
    }

    #[test]
    fn test_concrete_type_string() {
        let t = ConcreteType::String;
        assert_eq!(t, ConcreteType::String);
    }

    #[test]
    fn test_concrete_type_str_ref() {
        let t = ConcreteType::StrRef;
        assert_eq!(t, ConcreteType::StrRef);
    }

    #[test]
    fn test_concrete_type_unit() {
        let t = ConcreteType::Unit;
        assert_eq!(t, ConcreteType::Unit);
    }

    #[test]
    fn test_concrete_type_vec() {
        let t = ConcreteType::Vec(Box::new(ConcreteType::I64));
        assert!(matches!(t, ConcreteType::Vec(_)));
    }

    #[test]
    fn test_concrete_type_option() {
        let t = ConcreteType::Option(Box::new(ConcreteType::String));
        assert!(matches!(t, ConcreteType::Option(_)));
    }

    #[test]
    fn test_concrete_type_hashmap() {
        let t = ConcreteType::HashMap(Box::new(ConcreteType::String), Box::new(ConcreteType::I64));
        assert!(matches!(t, ConcreteType::HashMap(_, _)));
    }

    #[test]
    fn test_concrete_type_unknown() {
        let t = ConcreteType::Unknown;
        assert_eq!(t, ConcreteType::Unknown);
    }

    #[test]
    fn test_concrete_type_clone() {
        let t = ConcreteType::Vec(Box::new(ConcreteType::Bool));
        let cloned = t.clone();
        assert_eq!(t, cloned);
    }

    // =========================================================================
    // ConcreteType::from_hir_type tests
    // =========================================================================

    #[test]
    fn test_from_hir_type_int() {
        let ct = ConcreteType::from_hir_type(&Type::Int);
        assert_eq!(ct, ConcreteType::I64);
    }

    #[test]
    fn test_from_hir_type_float() {
        let ct = ConcreteType::from_hir_type(&Type::Float);
        assert_eq!(ct, ConcreteType::F64);
    }

    #[test]
    fn test_from_hir_type_bool() {
        let ct = ConcreteType::from_hir_type(&Type::Bool);
        assert_eq!(ct, ConcreteType::Bool);
    }

    #[test]
    fn test_from_hir_type_string() {
        let ct = ConcreteType::from_hir_type(&Type::String);
        assert_eq!(ct, ConcreteType::String);
    }

    #[test]
    fn test_from_hir_type_none() {
        let ct = ConcreteType::from_hir_type(&Type::None);
        assert_eq!(ct, ConcreteType::Unit);
    }

    #[test]
    fn test_from_hir_type_list() {
        let ct = ConcreteType::from_hir_type(&Type::List(Box::new(Type::Int)));
        assert!(matches!(ct, ConcreteType::Vec(_)));
    }

    #[test]
    fn test_from_hir_type_optional() {
        let ct = ConcreteType::from_hir_type(&Type::Optional(Box::new(Type::String)));
        assert!(matches!(ct, ConcreteType::Option(_)));
    }

    #[test]
    fn test_from_hir_type_dict() {
        let ct =
            ConcreteType::from_hir_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        assert!(matches!(ct, ConcreteType::HashMap(_, _)));
    }

    #[test]
    fn test_from_hir_type_unknown() {
        let ct = ConcreteType::from_hir_type(&Type::Unknown);
        assert_eq!(ct, ConcreteType::Unknown);
    }

    // =========================================================================
    // ConcreteType::to_hir_type tests
    // =========================================================================

    #[test]
    fn test_to_hir_type_i32() {
        let ht = ConcreteType::I32.to_hir_type();
        assert_eq!(ht, Type::Int);
    }

    #[test]
    fn test_to_hir_type_i64() {
        let ht = ConcreteType::I64.to_hir_type();
        assert_eq!(ht, Type::Int);
    }

    #[test]
    fn test_to_hir_type_f32() {
        let ht = ConcreteType::F32.to_hir_type();
        assert_eq!(ht, Type::Float);
    }

    #[test]
    fn test_to_hir_type_f64() {
        let ht = ConcreteType::F64.to_hir_type();
        assert_eq!(ht, Type::Float);
    }

    #[test]
    fn test_to_hir_type_bool() {
        let ht = ConcreteType::Bool.to_hir_type();
        assert_eq!(ht, Type::Bool);
    }

    #[test]
    fn test_to_hir_type_string() {
        let ht = ConcreteType::String.to_hir_type();
        assert_eq!(ht, Type::String);
    }

    #[test]
    fn test_to_hir_type_str_ref() {
        let ht = ConcreteType::StrRef.to_hir_type();
        assert_eq!(ht, Type::String);
    }

    #[test]
    fn test_to_hir_type_unit() {
        let ht = ConcreteType::Unit.to_hir_type();
        assert_eq!(ht, Type::None);
    }

    #[test]
    fn test_to_hir_type_vec() {
        let ht = ConcreteType::Vec(Box::new(ConcreteType::I64)).to_hir_type();
        assert!(matches!(ht, Type::List(_)));
    }

    #[test]
    fn test_to_hir_type_option() {
        let ht = ConcreteType::Option(Box::new(ConcreteType::String)).to_hir_type();
        assert!(matches!(ht, Type::Optional(_)));
    }

    #[test]
    fn test_to_hir_type_hashmap() {
        let ht = ConcreteType::HashMap(Box::new(ConcreteType::String), Box::new(ConcreteType::I64))
            .to_hir_type();
        assert!(matches!(ht, Type::Dict(_, _)));
    }

    #[test]
    fn test_to_hir_type_unknown() {
        let ht = ConcreteType::Unknown.to_hir_type();
        assert_eq!(ht, Type::Unknown);
    }

    // =========================================================================
    // Constraint tests
    // =========================================================================

    #[test]
    fn test_constraint_equal() {
        let c = Constraint::Equal(TypeVar(0), TypeVar(1));
        assert!(matches!(c, Constraint::Equal(_, _)));
    }

    #[test]
    fn test_constraint_assign() {
        let c = Constraint::Assign(TypeVar(0), ConcreteType::I64);
        assert!(matches!(c, Constraint::Assign(_, _)));
    }

    #[test]
    fn test_constraint_call() {
        let c = Constraint::Call {
            callee: "foo".to_string(),
            args: vec![TypeVar(0), TypeVar(1)],
            ret: TypeVar(2),
        };
        assert!(matches!(c, Constraint::Call { .. }));
    }

    #[test]
    fn test_constraint_clone() {
        let c = Constraint::Assign(TypeVar(5), ConcreteType::Bool);
        let cloned = c.clone();
        assert!(matches!(cloned, Constraint::Assign(_, _)));
    }

    // =========================================================================
    // CallGraph tests
    // =========================================================================

    #[test]
    fn test_call_graph_basic() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        let b = cg.add_function("b");
        cg.add_call(a, b);

        assert_eq!(cg.callees(a), &[b]);
        assert_eq!(cg.callers(b), &[a]);
    }

    #[test]
    fn test_call_graph_new() {
        let cg = CallGraph::new();
        assert!(cg.fn_to_node.is_empty());
        assert!(cg.node_to_fn.is_empty());
    }

    #[test]
    fn test_call_graph_add_function() {
        let mut cg = CallGraph::new();
        let id = cg.add_function("test");
        assert_eq!(id.0, 0);
        assert!(cg.fn_to_node.contains_key("test"));
    }

    #[test]
    fn test_call_graph_add_same_function_twice() {
        let mut cg = CallGraph::new();
        let id1 = cg.add_function("test");
        let id2 = cg.add_function("test");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_call_graph_add_multiple_functions() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        let b = cg.add_function("b");
        let c = cg.add_function("c");
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_eq!(cg.fn_to_node.len(), 3);
    }

    #[test]
    fn test_call_graph_add_call_no_duplicates() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        let b = cg.add_function("b");
        cg.add_call(a, b);
        cg.add_call(a, b); // duplicate
        assert_eq!(cg.callees(a).len(), 1);
    }

    #[test]
    fn test_call_graph_callees_empty() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        assert!(cg.callees(a).is_empty());
    }

    #[test]
    fn test_call_graph_callers_empty() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        assert!(cg.callers(a).is_empty());
    }

    #[test]
    fn test_call_graph_topological_order_empty() {
        let cg = CallGraph::new();
        let order = cg.topological_order();
        assert!(order.is_empty());
    }

    #[test]
    fn test_call_graph_topological_order_single() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        let order = cg.topological_order();
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], a);
    }

    #[test]
    fn test_call_graph_topological_order_chain() {
        let mut cg = CallGraph::new();
        let a = cg.add_function("a");
        let b = cg.add_function("b");
        let c = cg.add_function("c");
        cg.add_call(a, b);
        cg.add_call(b, c);
        let order = cg.topological_order();
        assert_eq!(order.len(), 3);
    }

    // =========================================================================
    // UnionFind tests
    // =========================================================================

    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(10);
        uf.assign(0, ConcreteType::I64).unwrap();
        uf.union(0, 1).unwrap();

        assert_eq!(uf.get_type(1), Some(ConcreteType::I64));
    }

    #[test]
    fn test_union_find_new() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.parent.len(), 5);
        assert_eq!(uf.rank.len(), 5);
    }

    #[test]
    fn test_union_find_find_self() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(4), 4);
    }

    #[test]
    fn test_union_find_union_same() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 0).is_ok());
    }

    #[test]
    fn test_union_find_union_different() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1).is_ok());
        assert_eq!(uf.find(0), uf.find(1));
    }

    #[test]
    fn test_union_find_assign() {
        let mut uf = UnionFind::new(5);
        assert!(uf.assign(0, ConcreteType::Bool).is_ok());
        assert_eq!(uf.get_type(0), Some(ConcreteType::Bool));
    }

    #[test]
    fn test_union_find_assign_same_type() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::I64).unwrap();
        assert!(uf.assign(0, ConcreteType::I64).is_ok());
    }

    #[test]
    fn test_union_find_assign_coercible() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::I32).unwrap();
        assert!(uf.assign(0, ConcreteType::I64).is_ok());
        assert_eq!(uf.get_type(0), Some(ConcreteType::I64));
    }

    #[test]
    fn test_union_find_assign_conflict() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::Bool).unwrap();
        assert!(uf.assign(0, ConcreteType::String).is_err());
    }

    #[test]
    fn test_union_find_get_type_none() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.get_type(0), None);
    }

    #[test]
    fn test_union_find_union_with_types() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::I64).unwrap();
        uf.assign(1, ConcreteType::I64).unwrap();
        assert!(uf.union(0, 1).is_ok());
    }

    #[test]
    fn test_union_find_union_type_conflict() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::Bool).unwrap();
        uf.assign(1, ConcreteType::String).unwrap();
        assert!(uf.union(0, 1).is_err());
    }

    #[test]
    fn test_union_find_union_type_coercion() {
        let mut uf = UnionFind::new(5);
        uf.assign(0, ConcreteType::I32).unwrap();
        uf.assign(1, ConcreteType::F64).unwrap();
        assert!(uf.union(0, 1).is_ok());
    }

    // =========================================================================
    // UnifyError tests
    // =========================================================================

    #[test]
    fn test_unify_error_type_conflict() {
        let err = UnifyError::TypeConflict(ConcreteType::Bool, ConcreteType::String);
        assert!(matches!(err, UnifyError::TypeConflict(_, _)));
    }

    #[test]
    fn test_unify_error_clone() {
        let err = UnifyError::TypeConflict(ConcreteType::I32, ConcreteType::Bool);
        let cloned = err.clone();
        assert!(matches!(cloned, UnifyError::TypeConflict(_, _)));
    }

    // =========================================================================
    // coerce_types tests
    // =========================================================================

    #[test]
    fn test_coercion_lattice() {
        assert_eq!(
            coerce_types(&ConcreteType::I32, &ConcreteType::F64),
            Some(ConcreteType::F64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::I64, &ConcreteType::I64),
            Some(ConcreteType::I64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::Bool, &ConcreteType::String),
            None
        );
    }

    #[test]
    fn test_coerce_same_type() {
        assert_eq!(
            coerce_types(&ConcreteType::Bool, &ConcreteType::Bool),
            Some(ConcreteType::Bool)
        );
        assert_eq!(
            coerce_types(&ConcreteType::Unit, &ConcreteType::Unit),
            Some(ConcreteType::Unit)
        );
    }

    #[test]
    fn test_coerce_i32_i64() {
        assert_eq!(
            coerce_types(&ConcreteType::I32, &ConcreteType::I64),
            Some(ConcreteType::I64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::I64, &ConcreteType::I32),
            Some(ConcreteType::I64)
        );
    }

    #[test]
    fn test_coerce_f32_f64() {
        assert_eq!(
            coerce_types(&ConcreteType::F32, &ConcreteType::F64),
            Some(ConcreteType::F64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::F64, &ConcreteType::F32),
            Some(ConcreteType::F64)
        );
    }

    #[test]
    fn test_coerce_i32_f32() {
        assert_eq!(
            coerce_types(&ConcreteType::I32, &ConcreteType::F32),
            Some(ConcreteType::F32)
        );
        assert_eq!(
            coerce_types(&ConcreteType::F32, &ConcreteType::I32),
            Some(ConcreteType::F32)
        );
    }

    #[test]
    fn test_coerce_i32_f64() {
        assert_eq!(
            coerce_types(&ConcreteType::I32, &ConcreteType::F64),
            Some(ConcreteType::F64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::F64, &ConcreteType::I32),
            Some(ConcreteType::F64)
        );
    }

    #[test]
    fn test_coerce_i64_f32() {
        assert_eq!(
            coerce_types(&ConcreteType::I64, &ConcreteType::F32),
            Some(ConcreteType::F64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::F32, &ConcreteType::I64),
            Some(ConcreteType::F64)
        );
    }

    #[test]
    fn test_coerce_i64_f64() {
        assert_eq!(
            coerce_types(&ConcreteType::I64, &ConcreteType::F64),
            Some(ConcreteType::F64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::F64, &ConcreteType::I64),
            Some(ConcreteType::F64)
        );
    }

    #[test]
    fn test_coerce_string_strref() {
        assert_eq!(
            coerce_types(&ConcreteType::String, &ConcreteType::StrRef),
            Some(ConcreteType::String)
        );
        assert_eq!(
            coerce_types(&ConcreteType::StrRef, &ConcreteType::String),
            Some(ConcreteType::String)
        );
    }

    #[test]
    fn test_coerce_unknown() {
        assert_eq!(
            coerce_types(&ConcreteType::Unknown, &ConcreteType::I64),
            Some(ConcreteType::I64)
        );
        assert_eq!(
            coerce_types(&ConcreteType::Bool, &ConcreteType::Unknown),
            Some(ConcreteType::Bool)
        );
    }

    #[test]
    fn test_coerce_incompatible() {
        assert_eq!(coerce_types(&ConcreteType::Bool, &ConcreteType::I64), None);
        assert_eq!(
            coerce_types(&ConcreteType::String, &ConcreteType::F64),
            None
        );
        assert_eq!(coerce_types(&ConcreteType::Unit, &ConcreteType::Bool), None);
    }
}
