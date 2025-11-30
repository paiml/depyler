//! Lifetime analysis and inference for safe Rust code generation
//!
//! This module implements sophisticated lifetime inference to generate
//! idiomatic Rust code with proper borrowing and ownership patterns.

use crate::borrowing_context::{BorrowingContext, BorrowingStrategy};
use crate::hir::{AssignTarget, HirExpr, HirFunction, HirStmt, Type};
use crate::rust_gen::func_gen::infer_param_type_from_body;
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

            // DEPYLER-0518: Try to infer type from body usage for Unknown parameters
            // This prevents defaulting to serde_json::Value for parameters like 'pattern' and 'text'
            // that are used with regex operations and should be strings
            let param_type = if matches!(param.ty, Type::Unknown) {
                // Try to infer from function body usage patterns
                if let Some(inferred) = infer_param_type_from_body(&param.name, &func.body) {
                    inferred
                } else {
                    param.ty.clone()
                }
            } else {
                param.ty.clone()
            };
            let rust_type = type_mapper.map_type(&param_type);

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
            // DEPYLER-0614: Recursively analyze Block statements
            HirStmt::Block(stmts) => {
                for s in stmts {
                    self.analyze_stmt_for_param(param, s, usage, in_loop);
                }
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
            // DEPYLER-0427: Nested function support
            // Analyze nested function body for parameter usage
            HirStmt::FunctionDef { body, .. } => {
                for stmt in body {
                    self.analyze_stmt_for_param(param, stmt, usage, in_loop);
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
            HirExpr::Call { func: _, args, .. } => {
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
                generators,
            } => {
                // DEPYLER-0504: Support multiple generators
                // List comprehensions create a new scope, so the target variable
                // shadows any outer variable with the same name
                for gen in generators {
                    // Check if any generator target shadows our parameter
                    let target_shadows = gen.target == param
                        || gen.target.contains(&format!("({})", param))
                        || gen.target.contains(&format!("{},", param))
                        || gen.target.contains(&format!(", {}", param));

                    if !target_shadows {
                        // Only analyze if the comprehension target doesn't shadow our parameter
                        self.analyze_expr_for_param(param, &gen.iter, usage, true, false);
                        for cond in &gen.conditions {
                            self.analyze_expr_for_param(param, cond, usage, true, false);
                        }
                    }
                }
                // Analyze element expression (after all generators)
                self.analyze_expr_for_param(param, element, usage, true, in_return);
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
                generators,
            } => {
                // DEPYLER-0504: Support multiple generators
                // Set comprehensions create a new scope, so the target variable
                // shadows any outer variable with the same name
                for gen in generators {
                    // Check if any generator target shadows our parameter
                    let target_shadows = gen.target == param
                        || gen.target.contains(&format!("({})", param))
                        || gen.target.contains(&format!("{},", param))
                        || gen.target.contains(&format!(", {}", param));

                    if !target_shadows {
                        // Only analyze if the comprehension target doesn't shadow our parameter
                        self.analyze_expr_for_param(param, &gen.iter, usage, true, false);
                        for cond in &gen.conditions {
                            self.analyze_expr_for_param(param, cond, usage, true, false);
                        }
                    }
                }
                // Analyze element expression (after all generators)
                self.analyze_expr_for_param(param, element, usage, true, in_return);
            }
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => {
                // DEPYLER-0504: Support multiple generators
                // Dict comprehensions create a new scope, so the target variable
                // shadows any outer variable with the same name
                for gen in generators {
                    // Check if any generator target shadows our parameter
                    let target_shadows = gen.target == param
                        || gen.target.contains(&format!("({})", param))
                        || gen.target.contains(&format!("{},", param))
                        || gen.target.contains(&format!(", {}", param));

                    if !target_shadows {
                        // Only analyze if the comprehension target doesn't shadow our parameter
                        self.analyze_expr_for_param(param, &gen.iter, usage, true, false);
                        for cond in &gen.conditions {
                            self.analyze_expr_for_param(param, cond, usage, true, false);
                        }
                    }
                }
                // Analyze key and value expressions (after all generators)
                self.analyze_expr_for_param(param, key, usage, true, in_return);
                self.analyze_expr_for_param(param, value, usage, true, in_return);
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
                iterable, key_body, ..
            } => {
                // Analyze the iterable and key lambda body
                self.analyze_expr_for_param(param, iterable, usage, in_loop, in_return);
                self.analyze_expr_for_param(param, key_body, usage, in_loop, in_return);
            }
            HirExpr::GeneratorExp {
                element,
                generators,
            } => {
                // Analyze element and all generator components
                self.analyze_expr_for_param(param, element, usage, in_loop, in_return);
                for gen in generators {
                    self.analyze_expr_for_param(param, &gen.iter, usage, in_loop, in_return);
                    for cond in &gen.conditions {
                        self.analyze_expr_for_param(param, cond, usage, in_loop, in_return);
                    }
                }
            }
            // DEPYLER-0188: Walrus operator - analyze the value expression
            HirExpr::NamedExpr { value, .. } => {
                self.analyze_expr_for_param(param, value, usage, in_loop, in_return);
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

            // DEPYLER-0518: Try to infer type from body usage for Unknown parameters
            let param_type = if matches!(param.ty, Type::Unknown) {
                if let Some(inferred) = infer_param_type_from_body(&param.name, &func.body) {
                    inferred
                } else {
                    param.ty.clone()
                }
            } else {
                param.ty.clone()
            };
            let rust_type = type_mapper.map_type(&param_type);

            // Determine if we should borrow or take ownership
            // If parameter escapes (returned) and it's the same type as return, it should be moved
            let escapes_as_self =
                usage.escapes && rust_type == type_mapper.map_return_type(&func.ret_type);

            // DEPYLER-0629: Don't borrow Function types (Callable) - they're already boxed as Box<dyn Fn()>
            // Also skip Optional<Function> since the inner type is already indirected
            let is_callable_type = matches!(&param_type, Type::Function { .. })
                || matches!(&param_type, Type::Optional(inner) if matches!(inner.as_ref(), Type::Function { .. }));

            let should_borrow =
                !is_callable_type && !usage.is_moved && !escapes_as_self && (usage.is_read_only || usage.is_mutated);
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
            // DEPYLER-0275: Elide lifetime even if return doesn't use it
            // Rust's elision rules allow omitting explicit lifetime in single-param functions
            return Some(LifetimeResult {
                param_lifetimes: full_result.param_lifetimes,
                return_lifetime: None,   // Elided
                lifetime_params: vec![], // No explicit lifetimes needed
                lifetime_bounds: vec![],
                borrowing_strategies: full_result.borrowing_strategies,
            });
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
    use crate::hir::{
        BinOp, FunctionProperties, HirComprehension, HirFunction, HirParam, Literal, Symbol,
        Type as PythonType, UnaryOp,
    };
    use crate::type_mapper::PrimitiveType;
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
                target: AssignTarget::Symbol(Symbol::from("s")),
                value: HirExpr::Binary {
                    op: BinOp::Add,
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

    #[test]
    fn test_lifetime_info_construction() {
        let info = LifetimeInfo {
            name: "'a".to_string(),
            is_static: false,
            outlives: HashSet::new(),
            source: LifetimeSource::Parameter("x".to_string()),
        };
        assert_eq!(info.name, "'a");
        assert!(!info.is_static);
        assert!(info.outlives.is_empty());
    }

    #[test]
    fn test_lifetime_info_static() {
        let info = LifetimeInfo {
            name: "'static".to_string(),
            is_static: true,
            outlives: HashSet::new(),
            source: LifetimeSource::StaticLiteral,
        };
        assert!(info.is_static);
    }

    #[test]
    fn test_lifetime_info_with_outlives() {
        let mut outlives = HashSet::new();
        outlives.insert("'b".to_string());
        outlives.insert("'c".to_string());
        let info = LifetimeInfo {
            name: "'a".to_string(),
            is_static: false,
            outlives,
            source: LifetimeSource::Return,
        };
        assert_eq!(info.outlives.len(), 2);
        assert!(info.outlives.contains("'b"));
    }

    #[test]
    fn test_lifetime_source_variants() {
        let param_src = LifetimeSource::Parameter("x".to_string());
        let static_src = LifetimeSource::StaticLiteral;
        let local_src = LifetimeSource::Local;
        let return_src = LifetimeSource::Return;
        let field_src = LifetimeSource::Field("name".to_string());

        assert_eq!(param_src, LifetimeSource::Parameter("x".to_string()));
        assert_eq!(static_src, LifetimeSource::StaticLiteral);
        assert_eq!(local_src, LifetimeSource::Local);
        assert_eq!(return_src, LifetimeSource::Return);
        assert_eq!(field_src, LifetimeSource::Field("name".to_string()));
    }

    #[test]
    fn test_lifetime_source_ne() {
        let src1 = LifetimeSource::Parameter("x".to_string());
        let src2 = LifetimeSource::Parameter("y".to_string());
        assert_ne!(src1, src2);
    }

    #[test]
    fn test_param_usage_default() {
        let usage = ParamUsage::default();
        assert!(!usage.is_mutated);
        assert!(!usage.is_moved);
        assert!(!usage.escapes);
        assert!(!usage.is_read_only);
        assert!(!usage.used_in_loop);
        assert!(!usage.has_nested_borrows);
    }

    #[test]
    fn test_param_usage_all_true() {
        let usage = ParamUsage {
            is_mutated: true,
            is_moved: true,
            escapes: true,
            is_read_only: true,
            used_in_loop: true,
            has_nested_borrows: true,
        };
        assert!(usage.is_mutated);
        assert!(usage.is_moved);
        assert!(usage.escapes);
    }

    #[test]
    fn test_lifetime_constraint_variants() {
        let outlives = LifetimeConstraint::Outlives;
        let equal = LifetimeConstraint::Equal;
        let at_least = LifetimeConstraint::AtLeast;

        // Just verify they can be constructed
        assert!(matches!(outlives, LifetimeConstraint::Outlives));
        assert!(matches!(equal, LifetimeConstraint::Equal));
        assert!(matches!(at_least, LifetimeConstraint::AtLeast));
    }

    #[test]
    fn test_lifetime_result_empty() {
        let result = LifetimeResult {
            param_lifetimes: IndexMap::new(),
            return_lifetime: None,
            lifetime_params: vec![],
            lifetime_bounds: vec![],
            borrowing_strategies: IndexMap::new(),
        };
        assert!(result.param_lifetimes.is_empty());
        assert!(result.return_lifetime.is_none());
        assert!(result.lifetime_params.is_empty());
    }

    #[test]
    fn test_lifetime_result_with_data() {
        let mut param_lifetimes = IndexMap::new();
        param_lifetimes.insert(
            "x".to_string(),
            InferredParam {
                should_borrow: true,
                needs_mut: false,
                lifetime: Some("'a".to_string()),
                rust_type: RustType::String,
            },
        );
        let result = LifetimeResult {
            param_lifetimes,
            return_lifetime: Some("'a".to_string()),
            lifetime_params: vec!["'a".to_string()],
            lifetime_bounds: vec![("'a".to_string(), "'b".to_string())],
            borrowing_strategies: IndexMap::new(),
        };
        assert_eq!(result.param_lifetimes.len(), 1);
        assert_eq!(result.return_lifetime, Some("'a".to_string()));
        assert_eq!(result.lifetime_bounds.len(), 1);
    }

    #[test]
    fn test_inferred_param_construction() {
        let param = InferredParam {
            should_borrow: true,
            needs_mut: true,
            lifetime: Some("'a".to_string()),
            rust_type: RustType::String,
        };
        assert!(param.should_borrow);
        assert!(param.needs_mut);
        assert_eq!(param.lifetime, Some("'a".to_string()));
    }

    #[test]
    fn test_inferred_param_owned() {
        let param = InferredParam {
            should_borrow: false,
            needs_mut: false,
            lifetime: None,
            rust_type: RustType::Primitive(PrimitiveType::I64),
        };
        assert!(!param.should_borrow);
        assert!(param.lifetime.is_none());
    }

    #[test]
    fn test_lifetime_inference_default() {
        let inference = LifetimeInference::default();
        assert_eq!(inference.lifetime_counter, 0);
        assert!(inference.variable_lifetimes.is_empty());
    }

    #[test]
    fn test_add_constraint() {
        let mut inference = LifetimeInference::new();
        inference.add_constraint("'a", "'b", LifetimeConstraint::Outlives);
        inference.add_constraint("'a", "'c", LifetimeConstraint::Equal);

        let constraints = inference.lifetime_constraints.get("'a").unwrap();
        assert!(constraints.contains("'b"));
        assert!(constraints.contains("'c"));
    }

    #[test]
    fn test_compute_lifetime_bounds() {
        let mut inference = LifetimeInference::new();
        inference.add_constraint("'a", "'b", LifetimeConstraint::Outlives);
        inference.add_constraint("'c", "'return", LifetimeConstraint::Outlives);

        let bounds = inference.compute_lifetime_bounds();
        // Should include "'a: 'b" but not "'c: 'return" (internal marker)
        assert!(bounds.iter().any(|(f, t)| f == "'a" && t == "'b"));
        assert!(!bounds.iter().any(|(_, t)| t == "'return"));
    }

    #[test]
    fn test_return_type_needs_lifetime_str() {
        let inference = LifetimeInference::new();
        let str_type = RustType::Str { lifetime: None };
        assert!(inference.return_type_needs_lifetime(&str_type));
    }

    #[test]
    fn test_return_type_needs_lifetime_reference() {
        let inference = LifetimeInference::new();
        let ref_type = RustType::Reference {
            lifetime: None,
            inner: Box::new(RustType::Primitive(PrimitiveType::I64)),
            mutable: false,
        };
        assert!(inference.return_type_needs_lifetime(&ref_type));
    }

    #[test]
    fn test_return_type_needs_lifetime_cow() {
        let inference = LifetimeInference::new();
        let cow_type = RustType::Cow {
            lifetime: "'a".to_string(),
        };
        assert!(inference.return_type_needs_lifetime(&cow_type));
    }

    #[test]
    fn test_return_type_needs_lifetime_vec_str() {
        let inference = LifetimeInference::new();
        let vec_str = RustType::Vec(Box::new(RustType::Str { lifetime: None }));
        assert!(inference.return_type_needs_lifetime(&vec_str));
    }

    #[test]
    fn test_return_type_needs_lifetime_option_i64() {
        let inference = LifetimeInference::new();
        let opt_i64 = RustType::Option(Box::new(RustType::Primitive(PrimitiveType::I64)));
        assert!(!inference.return_type_needs_lifetime(&opt_i64));
    }

    #[test]
    fn test_return_type_needs_lifetime_result() {
        let inference = LifetimeInference::new();
        let result_type = RustType::Result(
            Box::new(RustType::Str { lifetime: None }),
            Box::new(RustType::String),
        );
        assert!(inference.return_type_needs_lifetime(&result_type));
    }

    #[test]
    fn test_return_type_needs_lifetime_tuple() {
        let inference = LifetimeInference::new();
        let tuple = RustType::Tuple(vec![
            RustType::Primitive(PrimitiveType::I64),
            RustType::Str { lifetime: None },
        ]);
        assert!(inference.return_type_needs_lifetime(&tuple));
    }

    #[test]
    fn test_return_type_needs_lifetime_simple() {
        let inference = LifetimeInference::new();
        assert!(!inference.return_type_needs_lifetime(&RustType::Primitive(PrimitiveType::I64)));
        assert!(!inference.return_type_needs_lifetime(&RustType::Primitive(PrimitiveType::F64)));
        assert!(!inference.return_type_needs_lifetime(&RustType::Primitive(PrimitiveType::Bool)));
        assert!(!inference.return_type_needs_lifetime(&RustType::Unit));
    }

    #[test]
    fn test_is_reference_type() {
        let inference = LifetimeInference::new();

        let str_type = RustType::Str { lifetime: None };
        assert!(inference.is_reference_type(&str_type));

        let ref_type = RustType::Reference {
            lifetime: None,
            inner: Box::new(RustType::Primitive(PrimitiveType::I64)),
            mutable: false,
        };
        assert!(inference.is_reference_type(&ref_type));

        let cow_type = RustType::Cow {
            lifetime: "'a".to_string(),
        };
        assert!(inference.is_reference_type(&cow_type));

        assert!(!inference.is_reference_type(&RustType::String));
        assert!(!inference.is_reference_type(&RustType::Primitive(PrimitiveType::I64)));
    }

    #[test]
    fn test_analyze_stmt_for_param_assign() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::Assign {
            target: AssignTarget::Symbol(Symbol::from("x")),
            value: HirExpr::Literal(Literal::Int(42)),
            type_annotation: None,
        };
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.is_mutated);
    }

    #[test]
    fn test_analyze_stmt_for_param_if() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::If {
            condition: HirExpr::Var("x".to_string()),
            then_body: vec![HirStmt::Return(Some(HirExpr::Var("x".to_string())))],
            else_body: Some(vec![HirStmt::Pass]),
        };
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.is_read_only);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_stmt_for_param_while() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::While {
            condition: HirExpr::Var("x".to_string()),
            body: vec![HirStmt::Expr(HirExpr::Var("x".to_string()))],
        };
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.used_in_loop);
    }

    #[test]
    fn test_analyze_stmt_for_param_for() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::For {
            target: AssignTarget::Symbol(Symbol::from("i")),
            iter: HirExpr::Var("items".to_string()),
            body: vec![HirStmt::Pass],
        };
        inference.analyze_stmt_for_param("items", &stmt, &mut usage, false);
        assert!(usage.used_in_loop);
    }

    #[test]
    fn test_analyze_stmt_for_param_break_continue_pass() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        inference.analyze_stmt_for_param("x", &HirStmt::Break { label: None }, &mut usage, false);
        inference.analyze_stmt_for_param("x", &HirStmt::Continue { label: None }, &mut usage, false);
        inference.analyze_stmt_for_param("x", &HirStmt::Pass, &mut usage, false);

        // These statements don't affect parameter usage
        assert!(!usage.is_read_only);
        assert!(!usage.is_mutated);
    }

    #[test]
    fn test_analyze_stmt_for_param_block() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::Block(vec![
            HirStmt::Expr(HirExpr::Var("x".to_string())),
            HirStmt::Return(Some(HirExpr::Var("x".to_string()))),
        ]);
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.is_read_only);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_stmt_for_param_assert() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::Assert {
            test: HirExpr::Var("x".to_string()),
            msg: Some(HirExpr::Literal(Literal::String("error".to_string()))),
        };
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_stmt_for_param_raise() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let stmt = HirStmt::Raise {
            exception: Some(HirExpr::Var("x".to_string())),
            cause: None,
        };
        inference.analyze_stmt_for_param("x", &stmt, &mut usage, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_attribute() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        inference.analyze_expr_for_param("obj", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
        assert!(usage.has_nested_borrows);
    }

    #[test]
    fn test_analyze_expr_for_param_index() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        inference.analyze_expr_for_param("arr", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_call() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Call {
            func: Symbol::from("func"),
            args: vec![HirExpr::Var("x".to_string())],
            kwargs: vec![],
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, false);
        assert!(usage.is_moved);
    }

    #[test]
    fn test_analyze_expr_for_param_list() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Literal(Literal::Int(1)),
        ]);
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, true);
        assert!(usage.is_read_only);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_expr_for_param_dict() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Var("x".to_string()),
        )]);
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, true);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_expr_for_param_binary() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_unary() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_method_call() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![HirExpr::Var("arg".to_string())],
            kwargs: vec![],
        };
        inference.analyze_expr_for_param("obj", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_slice() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Slice {
            base: Box::new(HirExpr::Var("arr".to_string())),
            start: Some(Box::new(HirExpr::Var("start".to_string()))),
            stop: Some(Box::new(HirExpr::Var("stop".to_string()))),
            step: None,
        };
        inference.analyze_expr_for_param("arr", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_borrow() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Borrow {
            expr: Box::new(HirExpr::Var("x".to_string())),
            mutable: false,
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_list_comp() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::ListComp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![HirComprehension {
                target: Symbol::from("i"),
                iter: Box::new(HirExpr::Var("items".to_string())),
                conditions: vec![],
            }],
        };
        inference.analyze_expr_for_param("items", &expr, &mut usage, false, false);
        assert!(usage.used_in_loop);
    }

    #[test]
    fn test_analyze_expr_for_param_set() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Set(vec![HirExpr::Var("x".to_string())]);
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, true);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_expr_for_param_if_expr() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::IfExpr {
            test: Box::new(HirExpr::Var("cond".to_string())),
            body: Box::new(HirExpr::Var("x".to_string())),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, true);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_expr_for_param_lambda() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Lambda {
            params: vec![],
            body: Box::new(HirExpr::Var("x".to_string())),
        };
        inference.analyze_expr_for_param("x", &expr, &mut usage, false, false);
        assert!(usage.is_read_only);
    }

    #[test]
    fn test_analyze_expr_for_param_await() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Await {
            value: Box::new(HirExpr::Var("future".to_string())),
        };
        inference.analyze_expr_for_param("future", &expr, &mut usage, false, true);
        assert!(usage.escapes);
    }

    #[test]
    fn test_analyze_expr_for_param_yield() {
        let mut inference = LifetimeInference::new();
        let mut usage = ParamUsage::default();

        let expr = HirExpr::Yield {
            value: Some(Box::new(HirExpr::Var("val".to_string()))),
        };
        inference.analyze_expr_for_param("val", &expr, &mut usage, false, true);
        assert!(usage.escapes);
    }

    #[test]
    fn test_inferred_param_clone() {
        let param = InferredParam {
            should_borrow: true,
            needs_mut: false,
            lifetime: Some("'a".to_string()),
            rust_type: RustType::String,
        };
        let cloned = param.clone();
        assert_eq!(param.should_borrow, cloned.should_borrow);
        assert_eq!(param.lifetime, cloned.lifetime);
    }

    #[test]
    fn test_lifetime_result_clone() {
        let result = LifetimeResult {
            param_lifetimes: IndexMap::new(),
            return_lifetime: Some("'a".to_string()),
            lifetime_params: vec!["'a".to_string()],
            lifetime_bounds: vec![],
            borrowing_strategies: IndexMap::new(),
        };
        let cloned = result.clone();
        assert_eq!(result.return_lifetime, cloned.return_lifetime);
    }

    #[test]
    fn test_lifetime_info_clone() {
        let info = LifetimeInfo {
            name: "'a".to_string(),
            is_static: false,
            outlives: HashSet::new(),
            source: LifetimeSource::Local,
        };
        let cloned = info.clone();
        assert_eq!(info.name, cloned.name);
    }

    #[test]
    fn test_param_usage_clone() {
        let usage = ParamUsage {
            is_mutated: true,
            is_moved: false,
            escapes: true,
            is_read_only: false,
            used_in_loop: true,
            has_nested_borrows: false,
        };
        let cloned = usage.clone();
        assert_eq!(usage.is_mutated, cloned.is_mutated);
        assert_eq!(usage.escapes, cloned.escapes);
    }

    #[test]
    fn test_lifetime_constraint_clone() {
        let constraint = LifetimeConstraint::Outlives;
        let _cloned = constraint.clone();
    }

    #[test]
    fn test_lifetime_source_clone() {
        let src = LifetimeSource::Field("name".to_string());
        let cloned = src.clone();
        assert_eq!(src, cloned);
    }

    #[test]
    fn test_multiple_lifetime_generation() {
        let mut inference = LifetimeInference::new();
        let lifetimes: Vec<String> = (0..10).map(|_| inference.next_lifetime()).collect();

        assert_eq!(lifetimes[0], "'a");
        assert_eq!(lifetimes[1], "'b");
        assert_eq!(lifetimes[2], "'c");
        assert_eq!(lifetimes[3], "'l1");
        assert_eq!(lifetimes[9], "'l7");
    }

    #[test]
    fn test_analyze_function_with_no_params() {
        let mut inference = LifetimeInference::new();
        let type_mapper = crate::type_mapper::TypeMapper::new();

        let func = HirFunction {
            name: "no_params".to_string(),
            params: smallvec![],
            ret_type: PythonType::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::Int(42))))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = inference.analyze_function(&func, &type_mapper);
        assert!(result.param_lifetimes.is_empty());
    }

    #[test]
    fn test_elision_with_no_references() {
        let mut inference = LifetimeInference::new();
        let type_mapper = crate::type_mapper::TypeMapper::new();

        let func = HirFunction {
            name: "add".to_string(),
            params: smallvec![
                HirParam::new("a".to_string(), PythonType::Int),
                HirParam::new("b".to_string(), PythonType::Int),
            ],
            ret_type: PythonType::Int,
            body: vec![],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let result = inference.apply_elision_rules(&func, &type_mapper).unwrap();
        assert!(result.lifetime_params.is_empty());
        assert!(result.return_lifetime.is_none());
    }
}
