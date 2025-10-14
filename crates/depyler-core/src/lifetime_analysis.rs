//! Lifetime analysis and inference for safe Rust code generation
//!
//! This module implements sophisticated lifetime inference to generate
//! idiomatic Rust code with proper borrowing and ownership patterns.

use crate::borrowing_context::{BorrowingContext, BorrowingStrategy};
use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt};
use crate::type_mapper::RustType;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

/// Lifetime inference engine for function parameters and returns
#[derive(Debug)]
pub struct LifetimeInference {
    /// Counter for generating unique lifetime names
    lifetime_counter: usize,
    /// Map from variable names to their lifetime information
    variable_lifetimes: HashMap<String, LifetimeInfo>,
    /// Lifetime relationships (from -> set of lifetimes it outlives)
    lifetime_constraints: HashMap<String, HashSet<String>>,
    /// Function parameter analysis results
    param_analysis: HashMap<String, ParamUsage>,
}

/// Information about a variable's lifetime
#[derive(Debug, Clone)]
pub struct LifetimeInfo {
    /// The inferred lifetime name (e.g., "'a", "'b")
    pub name: String,
    /// Whether this lifetime is static
    pub is_static: bool,
    /// Variables that this lifetime must outlive
    pub outlives: HashSet<String>,
    /// Source of this lifetime (parameter, local, etc.)
    pub source: LifetimeSource,
}

/// Source of a lifetime
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifetimeSource {
    /// Function parameter
    Parameter(String),
    /// String literal
    StaticLiteral,
    /// Local variable
    Local,
    /// Return value
    Return,
    /// Struct field
    Field(String),
}

/// How a parameter is used in a function
#[derive(Debug, Clone, Default)]
pub struct ParamUsage {
    /// Parameter is mutated
    pub is_mutated: bool,
    /// Parameter is moved/consumed
    pub is_moved: bool,
    /// Parameter escapes (returned or stored)
    pub escapes: bool,
    /// Parameter is only read
    pub is_read_only: bool,
    /// Parameter is used in loops
    pub used_in_loop: bool,
    /// Nested borrows exist
    pub has_nested_borrows: bool,
}

/// Constraint between two lifetimes
#[derive(Debug, Clone)]
pub enum LifetimeConstraint {
    /// 'a: 'b (a outlives b)
    Outlives,
    /// 'a = 'b (same lifetime)
    Equal,
    /// 'a is at least as long as 'b
    AtLeast,
}

/// Result of lifetime inference for a function
#[derive(Debug, Clone)]
pub struct LifetimeResult {
    /// Inferred parameter lifetimes
    pub param_lifetimes: IndexMap<String, InferredParam>,
    /// Return type lifetime
    pub return_lifetime: Option<String>,
    /// Additional lifetime parameters needed
    pub lifetime_params: Vec<String>,
    /// Lifetime bounds (e.g., "'a: 'b")
    pub lifetime_bounds: Vec<(String, String)>,
    /// Borrowing strategies for parameters
    pub borrowing_strategies: IndexMap<String, BorrowingStrategy>,
}

/// Inferred parameter information
#[derive(Debug, Clone)]
pub struct InferredParam {
    /// Should be borrowed instead of owned
    pub should_borrow: bool,
    /// Needs mutable borrow
    pub needs_mut: bool,
    /// Lifetime name if borrowed
    pub lifetime: Option<String>,
    /// Original Rust type
    pub rust_type: RustType,
}

impl LifetimeInference {
    pub fn new() -> Self {
        Self {
            lifetime_counter: 0,
            variable_lifetimes: HashMap::new(),
            lifetime_constraints: HashMap::new(),
            param_analysis: HashMap::new(),
        }
    }

    /// Generate a new unique lifetime name
    fn next_lifetime(&mut self) -> String {
        let name = match self.lifetime_counter {
            0 => "'a".to_string(),
            1 => "'b".to_string(),
            2 => "'c".to_string(),
            n => format!("'l{}", n - 2),
        };
        self.lifetime_counter += 1;
        name
    }

    /// Add a lifetime constraint (from outlives to)
    #[allow(dead_code)]
    fn add_constraint(&mut self, from: &str, to: &str, _constraint: LifetimeConstraint) {
        self.lifetime_constraints
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
    }

    /// Analyze a function to infer parameter lifetimes
    pub fn analyze_function(
        &mut self,
        func: &HirFunction,
        type_mapper: &crate::type_mapper::TypeMapper,
    ) -> LifetimeResult {
        // Use enhanced borrowing context for comprehensive analysis
        let mut borrowing_ctx = BorrowingContext::new(Some(func.ret_type.clone()));
        let borrowing_result = borrowing_ctx.analyze_function(func, type_mapper);

        // Convert borrowing strategies to lifetime information
        let mut param_lifetimes = IndexMap::new();
        let mut lifetime_params = HashSet::new();

        for param in &func.params {
            let strategy = borrowing_result
                .param_strategies
                .get(&param.name)
                .cloned()
                .unwrap_or(BorrowingStrategy::TakeOwnership);

            let rust_type = type_mapper.map_type(&param.ty);

            let (should_borrow, needs_mut, lifetime) = match strategy {
                BorrowingStrategy::BorrowImmutable { lifetime } => {
                    let lt = lifetime.unwrap_or_else(|| self.next_lifetime());
                    lifetime_params.insert(lt.clone());
                    (true, false, Some(lt))
                }
                BorrowingStrategy::BorrowMutable { lifetime } => {
                    let lt = lifetime.unwrap_or_else(|| self.next_lifetime());
                    lifetime_params.insert(lt.clone());
                    (true, true, Some(lt))
                }
                BorrowingStrategy::UseCow { lifetime } => {
                    lifetime_params.insert(lifetime.clone());
                    // Cow is not a borrow, it's a flexible ownership type
                    (false, false, Some(lifetime))
                }
                _ => (false, false, None),
            };

            if let Some(ref lt) = lifetime {
                self.variable_lifetimes.insert(
                    param.name.clone(),
                    LifetimeInfo {
                        name: lt.clone(),
                        is_static: lt == "'static",
                        outlives: HashSet::new(),
                        source: LifetimeSource::Parameter(param.name.clone()),
                    },
                );
            }

            param_lifetimes.insert(
                param.name.clone(),
                InferredParam {
                    should_borrow,
                    needs_mut,
                    lifetime,
                    rust_type,
                },
            );
        }

        // Analyze return type lifetime requirements
        let return_lifetime = self.analyze_return_lifetime(func, type_mapper);
        if let Some(ref lt) = return_lifetime {
            lifetime_params.insert(lt.clone());
        }

        // Compute lifetime bounds from the constraint graph
        let lifetime_bounds = self.compute_lifetime_bounds();

        LifetimeResult {
            param_lifetimes,
            return_lifetime,
            lifetime_params: lifetime_params.into_iter().collect(),
            lifetime_bounds,
            borrowing_strategies: borrowing_result.param_strategies,
        }
    }

    /// Analyze how parameters are used in the function body
    #[allow(dead_code)]
    fn analyze_parameter_usage(&mut self, func: &HirFunction) {
        for param in &func.params {
            let mut usage = ParamUsage::default();
            for stmt in &func.body {
                self.analyze_stmt_for_param(&param.name, stmt, &mut usage, false);
            }
            self.param_analysis.insert(param.name.clone(), usage);
        }
    }

    /// Recursively analyze statements for parameter usage
    #[allow(dead_code)]
    fn analyze_stmt_for_param(
        &self,
        param: &str,
        stmt: &HirStmt,
        usage: &mut ParamUsage,
        in_loop: bool,
    ) {
        match stmt {
            HirStmt::Expr(expr) => self.analyze_expr_for_param(param, expr, usage, in_loop, false),
            HirStmt::Assign { target, value, .. } => {
                // Check if we're assigning to the parameter
                if let AssignTarget::Symbol(symbol) = target {
                    if symbol == param {
                        usage.is_mutated = true;
                    }
                }
                self.analyze_expr_for_param(param, value, usage, in_loop, false);
            }
            HirStmt::Return(value) => {
                if let Some(expr) = value {
                    self.analyze_expr_for_param(param, expr, usage, in_loop, true);
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.analyze_expr_for_param(param, condition, usage, in_loop, false);
                for stmt in then_body {
                    self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                    }
                }
            }
            HirStmt::While { condition, body } => {
                self.analyze_expr_for_param(param, condition, usage, true, false);
                for stmt in body {
                    self.analyze_stmt_for_param(param, stmt, usage, true);
                }
            }
            HirStmt::For { iter, body, .. } => {
                self.analyze_expr_for_param(param, iter, usage, true, false);
                for stmt in body {
                    self.analyze_stmt_for_param(param, stmt, usage, true);
                }
            }
            HirStmt::Raise { exception, cause } => {
                if let Some(exc) = exception {
                    self.analyze_expr_for_param(param, exc, usage, in_loop, false);
                }
                if let Some(c) = cause {
                    self.analyze_expr_for_param(param, c, usage, in_loop, false);
                }
            }
            HirStmt::Break { .. } | HirStmt::Continue { .. } | HirStmt::Pass => {
                // Break, continue, and pass don't contain expressions to analyze
            }
            HirStmt::Assert { test, msg } => {
                // Analyze the test expression and optional message
                self.analyze_expr_for_param(param, test, usage, in_loop, false);
                if let Some(message) = msg {
                    self.analyze_expr_for_param(param, message, usage, in_loop, false);
                }
            }
            HirStmt::With {
                context,
                target: _,
                body,
            } => {
                // Analyze context expression
                self.analyze_expr_for_param(param, context, usage, in_loop, false);

                // Analyze body statements
                for stmt in body {
                    self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                // Analyze try body
                for stmt in body {
                    self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                }

                // Analyze except handlers
                for handler in handlers {
                    for stmt in &handler.body {
                        self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                    }
                }

                // Analyze else clause
                if let Some(else_stmts) = orelse {
                    for stmt in else_stmts {
                        self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                    }
                }

                // Analyze finally clause
                if let Some(finally_stmts) = finalbody {
                    for stmt in finally_stmts {
                        self.analyze_stmt_for_param(param, stmt, usage, in_loop);
                    }
                }
            }
        }
    }

    /// Analyze expressions for parameter usage
    #[allow(dead_code, clippy::only_used_in_recursion)]
    fn analyze_expr_for_param(
        &self,
        param: &str,
        expr: &HirExpr,
        usage: &mut ParamUsage,
        in_loop: bool,
        in_return: bool,
    ) {
        match expr {
            HirExpr::Var(id) => {
                if id == param {
                    usage.is_read_only = true;
                    if in_return {
                        usage.escapes = true;
                    }
                    if in_loop {
                        usage.used_in_loop = true;
                    }
                }
            }
            HirExpr::Attribute { value, .. } => {
                if let HirExpr::Var(id) = &**value {
                    if id == param {
                        usage.is_read_only = true;
                        usage.has_nested_borrows = true;
                    }
                }
                self.analyze_expr_for_param(param, value, usage, in_loop, in_return);
            }
            HirExpr::Index { base, index } => {
                self.analyze_expr_for_param(param, base, usage, in_loop, false);
                self.analyze_expr_for_param(param, index, usage, in_loop, false);
            }
            HirExpr::Call { func: _, args } => {
                // Check if parameter is passed to a function (potential move)
                for arg in args {
                    if let HirExpr::Var(id) = arg {
                        if id == param {
                            // Conservative: assume moved unless we know the function borrows
                            usage.is_moved = true;
                        }
                    }
                }
                // Note: func is a Symbol, not an expression in HirExpr
                for arg in args {
                    self.analyze_expr_for_param(param, arg, usage, in_loop, false);
                }
            }
            HirExpr::List(elements) | HirExpr::Tuple(elements) => {
                for elem in elements {
                    self.analyze_expr_for_param(param, elem, usage, in_loop, in_return);
                }
            }
            HirExpr::Dict(pairs) => {
                for (k, v) in pairs {
                    self.analyze_expr_for_param(param, k, usage, in_loop, false);
                    self.analyze_expr_for_param(param, v, usage, in_loop, in_return);
                }
            }
            HirExpr::Binary { left, right, .. } => {
                self.analyze_expr_for_param(param, left, usage, in_loop, false);
                self.analyze_expr_for_param(param, right, usage, in_loop, false);
            }
            HirExpr::Unary { operand, .. } => {
                self.analyze_expr_for_param(param, operand, usage, in_loop, false);
            }
            HirExpr::Literal(_) => {}
            HirExpr::MethodCall { object, args, .. } => {
                self.analyze_expr_for_param(param, object, usage, in_loop, in_return);
                for arg in args {
                    self.analyze_expr_for_param(param, arg, usage, in_loop, in_return);
                }
            }
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => {
                self.analyze_expr_for_param(param, base, usage, in_loop, in_return);
                if let Some(s) = start {
                    self.analyze_expr_for_param(param, s, usage, in_loop, in_return);
                }
                if let Some(s) = stop {
                    self.analyze_expr_for_param(param, s, usage, in_loop, in_return);
                }
                if let Some(s) = step {
                    self.analyze_expr_for_param(param, s, usage, in_loop, in_return);
                }
            }
            HirExpr::Borrow { expr, .. } => {
                self.analyze_expr_for_param(param, expr, usage, in_loop, in_return);
            }
            HirExpr::ListComp {
                element,
                target,
                iter,
                condition,
            } => {
                // List comprehensions create a new scope, so the target variable
                // shadows any outer variable with the same name
                if target != param {
                    // Only analyze if the comprehension target doesn't shadow our parameter
                    self.analyze_expr_for_param(param, iter, usage, true, false);
                    self.analyze_expr_for_param(param, element, usage, true, in_return);
                    if let Some(cond) = condition {
                        self.analyze_expr_for_param(param, cond, usage, true, false);
                    }
                }
                // If target shadows param, we don't analyze element/condition
                // since they would refer to the comprehension variable, not the parameter
            }
            HirExpr::Lambda { params: _, body } => {
                // Lambda functions can capture parameters by reference
                // Analyze the body for parameter usage
                self.analyze_expr_for_param(param, body, usage, in_loop, in_return);
            }
            HirExpr::Set(elements) | HirExpr::FrozenSet(elements) => {
                for elem in elements {
                    self.analyze_expr_for_param(param, elem, usage, in_loop, in_return);
                }
            }
            HirExpr::SetComp {
                element,
                target,
                iter,
                condition,
            } => {
                // Set comprehensions create a new scope, so the target variable
                // shadows any outer variable with the same name
                if target != param {
                    // Only analyze if the comprehension target doesn't shadow our parameter
                    self.analyze_expr_for_param(param, iter, usage, true, false);
                    self.analyze_expr_for_param(param, element, usage, true, in_return);
                    if let Some(cond) = condition {
                        self.analyze_expr_for_param(param, cond, usage, true, false);
                    }
                }
                // If target shadows param, we don't analyze element/condition
                // since they would refer to the comprehension variable, not the parameter
            }
            HirExpr::Await { value } => {
                // Await expressions propagate parameter usage
                self.analyze_expr_for_param(param, value, usage, in_loop, in_return);
            }
            HirExpr::Yield { value } => {
                // Yield expressions pass values to iterator
                if let Some(v) = value {
                    self.analyze_expr_for_param(param, v, usage, in_loop, in_return);
                }
            }
            HirExpr::FString { .. } => {
                // FString support not yet implemented for lifetime analysis
            }
            HirExpr::IfExpr { test, body, orelse } => {
                // Analyze all branches of the ternary expression
                self.analyze_expr_for_param(param, test, usage, in_loop, in_return);
                self.analyze_expr_for_param(param, body, usage, in_loop, in_return);
                self.analyze_expr_for_param(param, orelse, usage, in_loop, in_return);
            }
            HirExpr::SortByKey {
                iterable,
                key_body,
                ..
            } => {
                // Analyze the iterable and key lambda body
                self.analyze_expr_for_param(param, iterable, usage, in_loop, in_return);
                self.analyze_expr_for_param(param, key_body, usage, in_loop, in_return);
            }
            HirExpr::GeneratorExp { element, generators } => {
                // Analyze element and all generator components
                self.analyze_expr_for_param(param, element, usage, in_loop, in_return);
                for gen in generators {
                    self.analyze_expr_for_param(param, &gen.iter, usage, in_loop, in_return);
                    for cond in &gen.conditions {
                        self.analyze_expr_for_param(param, cond, usage, in_loop, in_return);
                    }
                }
            }
        }
    }

    /// Infer parameter lifetimes based on usage analysis
    #[allow(dead_code)]
    fn infer_parameter_lifetimes(
        &mut self,
        func: &HirFunction,
        type_mapper: &crate::type_mapper::TypeMapper,
    ) -> IndexMap<String, InferredParam> {
        let mut result = IndexMap::new();

        for param in &func.params {
            let usage = self
                .param_analysis
                .get(&param.name)
                .cloned()
                .unwrap_or_default();
            let rust_type = type_mapper.map_type(&param.ty);

            // Determine if we should borrow or take ownership
            // If parameter escapes (returned) and it's the same type as return, it should be moved
            let escapes_as_self =
                usage.escapes && rust_type == type_mapper.map_return_type(&func.ret_type);
            let should_borrow =
                !usage.is_moved && !escapes_as_self && (usage.is_read_only || usage.is_mutated);
            let needs_mut = usage.is_mutated;

            let lifetime = if should_borrow {
                let lt = self.next_lifetime();

                // Add lifetime to our tracking
                self.variable_lifetimes.insert(
                    param.name.clone(),
                    LifetimeInfo {
                        name: lt.clone(),
                        is_static: false,
                        outlives: HashSet::new(),
                        source: LifetimeSource::Parameter(param.name.clone()),
                    },
                );

                // If parameter escapes, it needs to outlive the return
                if usage.escapes {
                    self.add_constraint(&lt, "'return", LifetimeConstraint::Outlives);
                }

                Some(lt)
            } else {
                None
            };

            result.insert(
                param.name.clone(),
                InferredParam {
                    should_borrow,
                    needs_mut,
                    lifetime,
                    rust_type,
                },
            );
        }

        result
    }

    /// Analyze return type lifetime requirements
    fn analyze_return_lifetime(
        &mut self,
        func: &HirFunction,
        type_mapper: &crate::type_mapper::TypeMapper,
    ) -> Option<String> {
        // Check if return type needs a lifetime
        let return_rust_type = type_mapper.map_return_type(&func.ret_type);
        if self.return_type_needs_lifetime(&return_rust_type) {
            // Look for parameters that escape through return
            for (param_name, usage) in &self.param_analysis {
                if usage.escapes {
                    if let Some(info) = self.variable_lifetimes.get(param_name) {
                        return Some(info.name.clone());
                    }
                }
            }

            // If no escaping parameters, might need a new lifetime
            Some(self.next_lifetime())
        } else {
            None
        }
    }

    /// Check if a type needs lifetime parameters
    #[allow(clippy::only_used_in_recursion)]
    fn return_type_needs_lifetime(&self, rust_type: &RustType) -> bool {
        match rust_type {
            RustType::Str { .. } => true,
            RustType::Reference { .. } => true,
            RustType::Cow { .. } => true,
            RustType::Vec(inner) | RustType::Option(inner) => {
                self.return_type_needs_lifetime(inner)
            }
            RustType::Result(ok, err) => {
                self.return_type_needs_lifetime(ok) || self.return_type_needs_lifetime(err)
            }
            RustType::Tuple(types) => types.iter().any(|t| self.return_type_needs_lifetime(t)),
            _ => false,
        }
    }

    /// Compute lifetime bounds from the constraints
    fn compute_lifetime_bounds(&self) -> Vec<(String, String)> {
        let mut bounds = Vec::new();

        for (from, tos) in &self.lifetime_constraints {
            for to in tos {
                if to != "'return" {
                    // Don't include internal markers
                    bounds.push((from.clone(), to.clone()));
                }
            }
        }

        bounds
    }

    /// Apply Rust's lifetime elision rules
    pub fn apply_elision_rules(
        &mut self,
        func: &HirFunction,
        type_mapper: &crate::type_mapper::TypeMapper,
    ) -> Option<LifetimeResult> {
        // First, do the full analysis
        let full_result = self.analyze_function(func, type_mapper);

        // Count reference parameters
        let ref_params: Vec<_> = full_result
            .param_lifetimes
            .iter()
            .filter(|(_, param)| param.should_borrow)
            .collect();

        let return_needs_lifetime = full_result.return_lifetime.is_some();

        // Apply elision rules
        if ref_params.is_empty() {
            // No references, no lifetimes needed
            return Some(LifetimeResult {
                param_lifetimes: full_result.param_lifetimes,
                return_lifetime: None,
                lifetime_params: vec![],
                lifetime_bounds: vec![],
                borrowing_strategies: full_result.borrowing_strategies,
            });
        }

        if ref_params.len() == 1 {
            // Rule 2: Single input lifetime can be elided
            if return_needs_lifetime {
                // The return type uses the same lifetime as the single input
                return Some(LifetimeResult {
                    param_lifetimes: full_result.param_lifetimes,
                    return_lifetime: None,   // Elided
                    lifetime_params: vec![], // No explicit lifetimes needed
                    lifetime_bounds: vec![],
                    borrowing_strategies: full_result.borrowing_strategies,
                });
            }
        }

        // Rule 3: If there's a &self or &mut self parameter, use its lifetime for outputs
        if let Some((_name, _)) = ref_params.iter().find(|(name, _)| *name == "self") {
            if return_needs_lifetime {
                return Some(LifetimeResult {
                    param_lifetimes: full_result.param_lifetimes,
                    return_lifetime: None, // Uses self's lifetime implicitly
                    lifetime_params: vec![],
                    lifetime_bounds: vec![],
                    borrowing_strategies: full_result.borrowing_strategies,
                });
            }
        }

        // Cannot elide - return the full analysis
        Some(full_result)
    }

    /// Check if a type is a reference type
    #[allow(dead_code)]
    fn is_reference_type(&self, rust_type: &RustType) -> bool {
        matches!(
            rust_type,
            RustType::Str { .. } | RustType::Reference { .. } | RustType::Cow { .. }
        )
    }
}

impl Default for LifetimeInference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{FunctionProperties, HirFunction, HirParam, Literal, Type as PythonType};
    use depyler_annotations::TranspilationAnnotations;
    use smallvec::smallvec;

    #[test]
    fn test_lifetime_generation() {
        let mut inference = LifetimeInference::new();
        assert_eq!(inference.next_lifetime(), "'a");
        assert_eq!(inference.next_lifetime(), "'b");
        assert_eq!(inference.next_lifetime(), "'c");
        assert_eq!(inference.next_lifetime(), "'l1");
    }

    #[test]
    fn test_parameter_usage_analysis() {
        let mut inference = LifetimeInference::new();
        let _type_mapper = crate::type_mapper::TypeMapper::new();

        // Create a simple function that reads a parameter
        let func = HirFunction {
            name: "test".to_string(),
            params: smallvec![HirParam::new("x".to_string(), PythonType::String)],
            ret_type: PythonType::String,
            body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        inference.analyze_parameter_usage(&func);
        let usage = inference.param_analysis.get("x").unwrap();
        assert!(usage.is_read_only);
        assert!(usage.escapes);
        assert!(!usage.is_mutated);
    }

    #[test]
    fn test_lifetime_inference() {
        let mut inference = LifetimeInference::new();
        let type_mapper = crate::type_mapper::TypeMapper::new();

        let func = HirFunction {
            name: "get_len".to_string(),
            params: smallvec![HirParam::new("s".to_string(), PythonType::String)],
            ret_type: PythonType::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Attribute {
                value: Box::new(HirExpr::Var("s".to_string())),
                attr: "len".to_string(),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = inference.analyze_function(&func, &type_mapper);

        // Should infer that 's' can be borrowed
        let s_param = result.param_lifetimes.get("s").unwrap();
        assert!(s_param.should_borrow);
        assert!(!s_param.needs_mut);
        assert!(s_param.lifetime.is_some());
    }

    #[test]
    fn test_elision_rules() {
        let mut inference = LifetimeInference::new();
        let type_mapper = crate::type_mapper::TypeMapper::new();

        // Function with single reference parameter
        let func = HirFunction {
            name: "identity".to_string(),
            params: smallvec![HirParam::new("x".to_string(), PythonType::String)],
            ret_type: PythonType::String,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        // Elision should work for a single parameter function
        let elision_result = inference.apply_elision_rules(&func, &type_mapper);
        // Elision rules are now implemented
        assert!(elision_result.is_some());

        // With elision, no explicit lifetime parameters should be needed
        if let Some(result) = elision_result {
            assert!(result.lifetime_params.is_empty());
        }
    }

    #[test]
    fn test_mutable_parameter_detection() {
        let mut inference = LifetimeInference::new();
        let type_mapper = crate::type_mapper::TypeMapper::new();

        // Function that mutates a parameter
        let func = HirFunction {
            name: "append_bang".to_string(),
            params: smallvec![HirParam::new("s".to_string(), PythonType::String)],
            ret_type: PythonType::None,
            body: vec![HirStmt::Assign {
                target: AssignTarget::Symbol("s".to_string()),
                value: HirExpr::Binary {
                    op: crate::hir::BinOp::Add,
                    left: Box::new(HirExpr::Var("s".to_string())),
                    right: Box::new(HirExpr::Literal(Literal::String("!".to_string()))),
                },
                type_annotation: None,
            }],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = inference.analyze_function(&func, &type_mapper);

        // When a string parameter is reassigned (mutated in Python terms),
        // Rust requires ownership since strings are not Copy
        let s_param = result.param_lifetimes.get("s").unwrap();
        assert!(!s_param.should_borrow); // Should take ownership
        assert!(!s_param.needs_mut); // Not a mutable borrow
    }
}
