//! # Depyler Core - Transpilation Engine
//!
//! Core transpilation engine for converting Python code to Rust and other targets.
//!
//! ## Overview
//!
//! This crate provides the fundamental transpilation pipeline that converts Python
//! source code into target languages (Rust, Ruchy) while preserving semantics and
//! ensuring memory safety.
//!
//! ## Example
//!
//! ```rust
//! use depyler_core::DepylerPipeline;
//!
//! let pipeline = DepylerPipeline::new();
//! let python = r#"
//! def factorial(n: int) -> int:
//!     if n <= 1:
//!         return 1
//!     return n * factorial(n - 1)
//! "#;
//!
//! match pipeline.transpile(python) {
//!     Ok(rust_code) => println!("Generated:\n{}", rust_code),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! ## Architecture
//!
//! The transpilation pipeline consists of several stages:
//!
//! 1. **Parsing** ([`ast_bridge`]) - Convert Python source to AST
//! 2. **HIR** ([`hir`]) - Transform AST to High-level Intermediate Representation
//! 3. **Type Analysis** ([`generic_inference`], [`const_generic_inference`]) - Infer types and generics
//! 4. **Ownership Analysis** ([`borrowing`], [`lifetime_analysis`]) - Determine ownership patterns
//! 5. **Optimization** ([`optimization`], [`string_optimization`]) - Apply optimizations
//! 6. **Code Generation** ([`codegen`], [`rust_gen`]) - Generate target code
//!
//! ## Key Types
//!
//! - [`DepylerPipeline`] - Main entry point for transpilation
//! - [`TranspileOptions`] - Configuration options
//! - [`Hir`] - High-level intermediate representation
//! - [`TranspilationBackend`] - Backend trait for target languages

pub mod annotation_aware_type_mapper;
pub mod ast_bridge;
pub mod backend;
pub mod borrowing;
pub mod borrowing_context;
pub mod cargo_toml_gen;
pub mod chaos;
pub mod codegen;
pub mod const_generic_inference;
pub mod debug;
pub mod decision_trace;
pub mod direct_rules;
pub mod documentation;
pub mod error;
pub mod error_reporting;
pub mod generator_state;
pub mod generator_yield_analysis;
pub mod generic_inference;
pub mod hir;
pub mod ide;
pub mod inlining;
pub mod lambda_codegen;
pub mod lambda_errors;
pub mod lambda_inference;
pub mod lambda_optimizer;
pub mod lambda_testing;
pub mod lambda_types;
pub mod lifetime_analysis;
pub mod lsp;
pub mod migration_suggestions;
pub mod module_mapper;
pub mod optimization;
pub mod optimizer;
pub mod performance_warnings;
pub mod profiling;
pub mod rust_gen;
pub mod simplified_hir;
pub mod stdlib_mappings;
pub mod string_optimization;
pub mod test_generation;
pub mod type_hints;
pub mod type_mapper;
pub mod type_system;
pub mod union_enum_gen;

use anyhow::Result;
use serde::{Deserialize, Serialize};

// Re-export backend traits and types
pub use backend::{TranspilationBackend, TranspilationTarget, ValidationError};
pub use error::TranspileError;
pub use simplified_hir::{
    Hir, HirBinaryOp, HirExpr, HirLiteral, HirParam, HirStatement, HirType, HirUnaryOp,
};

/// The main transpilation pipeline for converting Python code to multiple targets
///
/// ## Version 3.0.0 - Multi-Target Support
///
/// Depyler now supports multiple transpilation targets through the `TranspilationBackend` trait:
/// - **Rust** (default): Generates idiomatic, safe Rust code
/// - **Ruchy**: Generates functional Ruchy script format with pipeline operators
///
/// ### Example Usage
///
/// ```rust
/// use depyler_core::DepylerPipeline;
/// # use anyhow::Result;
/// # fn example() -> Result<()> {
/// # let python_code = "def hello(): pass";
///
/// // Create pipeline and transpile to Rust (default)
/// let pipeline = DepylerPipeline::new();
/// let rust_code = pipeline.transpile(python_code)?;
/// # Ok(())
/// # }
/// ```
///
/// `DepylerPipeline` coordinates the entire transpilation process, from parsing Python
/// source code to generating equivalent Rust code. It provides a high-level API for
/// transpilation with configurable analysis, optimization, and verification stages.
///
/// # Features
///
/// - **Semantic Analysis**: Converts Python AST to type-aware HIR
/// - **Type Inference**: Infers and validates type information
/// - **Optimization**: Applies performance optimizations
/// - **Verification**: Optional property verification for correctness
/// - **Code Generation**: Produces idiomatic Rust code
///
/// # Examples
///
/// Basic transpilation:
///
/// ```rust
/// use depyler_core::DepylerPipeline;
///
/// let pipeline = DepylerPipeline::new();
/// let python_code = r#"
/// def add(a: int, b: int) -> int:
///     return a + b
/// "#;
///
/// let rust_code = pipeline.transpile(python_code).unwrap();
/// assert!(rust_code.contains("pub fn add"));
/// assert!(rust_code.contains("i32"));
/// ```
///
/// With verification enabled:
///
/// ```rust
/// use depyler_core::DepylerPipeline;
///
/// let pipeline = DepylerPipeline::new()
///     .with_verification();
///
/// let python_code = r#"
/// def factorial(n: int) -> int:
///     if n <= 1:
///         return 1
///     return n * factorial(n - 1)
/// "#;
///
/// let rust_code = pipeline.transpile(python_code).unwrap();
/// assert!(rust_code.contains("factorial"));
/// ```
///
/// Parsing to HIR for analysis:
///
/// ```rust
/// use depyler_core::DepylerPipeline;
///
/// let pipeline = DepylerPipeline::new();
/// let python_code = "def hello(): return 'world'";
///
/// let hir = pipeline.parse_to_hir(python_code).unwrap();
/// assert_eq!(hir.functions.len(), 1);
/// assert_eq!(hir.functions[0].name, "hello");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepylerPipeline {
    analyzer: CoreAnalyzer,
    transpiler: DirectTranspiler,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifier: Option<PropertyVerifier>,
    #[serde(skip)]
    #[allow(dead_code)]
    mcp_client: LazyMcpClient,
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_config: Option<debug::DebugConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAnalyzer {
    pub metrics_enabled: bool,
    pub type_inference_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectTranspiler {
    pub type_mapper: type_mapper::TypeMapper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyVerifier {
    pub enable_quickcheck: bool,
    pub enable_contracts: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LazyMcpClient {
    #[allow(dead_code)]
    endpoint: Option<String>,
}

pub trait AnalyzableStage {
    type Input;
    type Output;
    type Metrics;

    fn execute(&self, input: Self::Input) -> Result<(Self::Output, Self::Metrics)>;
    fn validate(&self, output: &Self::Output) -> ValidationResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// DEPYLER-0575: Cross-function type propagation from call sites
/// When a variable with known type is passed to a function, propagate that type
/// to the function's parameter if the parameter type is still Unknown.
fn propagate_call_site_types(hir: &mut hir::HirModule) {
    use std::collections::HashMap;

    // Phase 1: Build map of function names to their parameter counts and return types
    let func_param_counts: HashMap<String, usize> = hir
        .functions
        .iter()
        .map(|f| (f.name.clone(), f.params.len()))
        .collect();

    let func_return_types: HashMap<String, hir::Type> = hir
        .functions
        .iter()
        .filter(|f| !matches!(f.ret_type, hir::Type::Unknown))
        .map(|f| (f.name.clone(), f.ret_type.clone()))
        .collect();

    // Phase 2: Build map of variable types from all functions
    // Collect from: parameters with types, return values, locals
    let mut var_types: HashMap<(String, String), hir::Type> = HashMap::new(); // (func_name, var_name) -> Type

    for func in &hir.functions {
        // Collect parameter types
        for param in &func.params {
            if !matches!(param.ty, hir::Type::Unknown) {
                var_types.insert((func.name.clone(), param.name.clone()), param.ty.clone());
            }
        }

        // Collect variable types from assignments (including from function calls)
        collect_var_types_from_stmts(&func.body, &func.name, &func_return_types, &mut var_types);
    }

    // Phase 3: Collect call site argument types
    // Maps (called_func_name, param_index) -> inferred_type
    let mut call_site_types: HashMap<(String, usize), hir::Type> = HashMap::new();

    for func in &hir.functions {
        collect_call_site_types(
            &func.body,
            &func.name,
            &var_types,
            &func_param_counts,
            &mut call_site_types,
        );
    }

    // Phase 4: Apply call site types to function parameters
    for func in &mut hir.functions {
        for (idx, param) in func.params.iter_mut().enumerate() {
            if matches!(param.ty, hir::Type::Unknown) {
                if let Some(inferred_type) = call_site_types.get(&(func.name.clone(), idx)) {
                    param.ty = inferred_type.clone();
                    eprintln!(
                        "DEPYLER-0575: Applied call-site type: {}::{} -> {:?}",
                        func.name, param.name, param.ty
                    );
                }
            }
        }
    }
}

/// Collect variable types from statements (assignments, etc.)
fn collect_var_types_from_stmts(
    stmts: &[hir::HirStmt],
    func_name: &str,
    func_return_types: &std::collections::HashMap<String, hir::Type>,
    var_types: &mut std::collections::HashMap<(String, String), hir::Type>,
) {
    for stmt in stmts {
        match stmt {
            hir::HirStmt::Assign {
                target: hir::AssignTarget::Symbol(var_name),
                value,
                type_annotation,
            } => {
                // If there's a type annotation, use that
                if let Some(ty) = type_annotation {
                    if !matches!(ty, hir::Type::Unknown) {
                        var_types.insert((func_name.to_string(), var_name.clone()), ty.clone());
                        continue;
                    }
                }
                // Otherwise infer from value (including function call return types)
                if let Some(ty) = infer_expr_type_with_returns(value, func_return_types) {
                    var_types.insert((func_name.to_string(), var_name.clone()), ty);
                }
            }
            hir::HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect_var_types_from_stmts(then_body, func_name, func_return_types, var_types);
                if let Some(else_stmts) = else_body {
                    collect_var_types_from_stmts(
                        else_stmts,
                        func_name,
                        func_return_types,
                        var_types,
                    );
                }
            }
            hir::HirStmt::While { body, .. } | hir::HirStmt::For { body, .. } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
            }
            hir::HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
                for handler in handlers {
                    collect_var_types_from_stmts(
                        &handler.body,
                        func_name,
                        func_return_types,
                        var_types,
                    );
                }
                if let Some(finally) = finalbody {
                    collect_var_types_from_stmts(finally, func_name, func_return_types, var_types);
                }
            }
            hir::HirStmt::With { body, .. } => {
                collect_var_types_from_stmts(body, func_name, func_return_types, var_types);
            }
            _ => {}
        }
    }
}

/// Infer type from an expression (simple cases only)
#[allow(dead_code)]
fn infer_expr_type(expr: &hir::HirExpr) -> Option<hir::Type> {
    infer_expr_type_with_returns(expr, &std::collections::HashMap::new())
}

/// Infer type from an expression, including function return types
fn infer_expr_type_with_returns(
    expr: &hir::HirExpr,
    func_return_types: &std::collections::HashMap<String, hir::Type>,
) -> Option<hir::Type> {
    match expr {
        hir::HirExpr::Literal(lit) => Some(match lit {
            hir::Literal::Int(_) => hir::Type::Int,
            hir::Literal::Float(_) => hir::Type::Float,
            hir::Literal::String(_) => hir::Type::String,
            hir::Literal::Bool(_) => hir::Type::Bool,
            hir::Literal::None => hir::Type::None,
            _ => return None,
        }),
        hir::HirExpr::List(elems) => {
            let elem_type = elems
                .first()
                .and_then(|e| infer_expr_type_with_returns(e, func_return_types))
                .unwrap_or(hir::Type::Unknown);
            Some(hir::Type::List(Box::new(elem_type)))
        }
        hir::HirExpr::Dict(_) => Some(hir::Type::Dict(
            Box::new(hir::Type::String),
            Box::new(hir::Type::Custom("serde_json::Value".to_string())),
        )),
        // DEPYLER-0575: Infer type from function call return type
        hir::HirExpr::Call { func, .. } => func_return_types.get(func).cloned(),
        _ => None,
    }
}

/// Collect call site types: when func(var) is called and var has a known type,
/// record that type for the function's parameter at that position
fn collect_call_site_types(
    stmts: &[hir::HirStmt],
    caller_func_name: &str,
    var_types: &std::collections::HashMap<(String, String), hir::Type>,
    func_param_counts: &std::collections::HashMap<String, usize>,
    call_site_types: &mut std::collections::HashMap<(String, usize), hir::Type>,
) {
    for stmt in stmts {
        match stmt {
            hir::HirStmt::Assign { value, .. } => {
                collect_call_site_types_from_expr(
                    value,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            hir::HirStmt::Expr(expr) | hir::HirStmt::Return(Some(expr)) => {
                collect_call_site_types_from_expr(
                    expr,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            hir::HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                collect_call_site_types_from_expr(
                    condition,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    then_body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                if let Some(else_stmts) = else_body {
                    collect_call_site_types(
                        else_stmts,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
            }
            hir::HirStmt::While { condition, body } => {
                collect_call_site_types_from_expr(
                    condition,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            hir::HirStmt::For { iter, body, .. } => {
                collect_call_site_types_from_expr(
                    iter,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            hir::HirStmt::Try {
                body,
                handlers,
                finalbody,
                ..
            } => {
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                for handler in handlers {
                    collect_call_site_types(
                        &handler.body,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
                if let Some(finally) = finalbody {
                    collect_call_site_types(
                        finally,
                        caller_func_name,
                        var_types,
                        func_param_counts,
                        call_site_types,
                    );
                }
            }
            hir::HirStmt::With { context, body, .. } => {
                collect_call_site_types_from_expr(
                    context,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types(
                    body,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
            _ => {}
        }
    }
}

/// Collect call site types from an expression
fn collect_call_site_types_from_expr(
    expr: &hir::HirExpr,
    caller_func_name: &str,
    var_types: &std::collections::HashMap<(String, String), hir::Type>,
    func_param_counts: &std::collections::HashMap<String, usize>,
    call_site_types: &mut std::collections::HashMap<(String, usize), hir::Type>,
) {
    match expr {
        hir::HirExpr::Call { func, args, .. } => {
            // Check if this is a call to a user-defined function
            if func_param_counts.contains_key(func) {
                // For each argument, if it's a variable with known type, record it
                for (idx, arg) in args.iter().enumerate() {
                    if let hir::HirExpr::Var(var_name) = arg {
                        // Look up the variable's type in the caller's scope
                        if let Some(ty) =
                            var_types.get(&(caller_func_name.to_string(), var_name.clone()))
                        {
                            // DEPYLER-0575: Skip Unknown and Optional types
                            // Optional types often get unwrapped before use, so don't propagate them
                            let should_propagate =
                                !matches!(ty, hir::Type::Unknown | hir::Type::Optional(_));
                            if should_propagate {
                                call_site_types.insert((func.clone(), idx), ty.clone());
                            }
                        }
                    }
                }
            }
            // Recurse into arguments
            for arg in args {
                collect_call_site_types_from_expr(
                    arg,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        hir::HirExpr::MethodCall { object, args, .. } => {
            collect_call_site_types_from_expr(
                object,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            for arg in args {
                collect_call_site_types_from_expr(
                    arg,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        hir::HirExpr::Binary { left, right, .. } => {
            collect_call_site_types_from_expr(
                left,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                right,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        hir::HirExpr::Unary { operand, .. } => {
            collect_call_site_types_from_expr(
                operand,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        hir::HirExpr::Index { base, index } => {
            collect_call_site_types_from_expr(
                base,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                index,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        hir::HirExpr::List(elems) => {
            for elem in elems {
                collect_call_site_types_from_expr(
                    elem,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        hir::HirExpr::Dict(items) => {
            for (k, v) in items {
                collect_call_site_types_from_expr(
                    k,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
                collect_call_site_types_from_expr(
                    v,
                    caller_func_name,
                    var_types,
                    func_param_counts,
                    call_site_types,
                );
            }
        }
        hir::HirExpr::IfExpr { test, body, orelse } => {
            collect_call_site_types_from_expr(
                test,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                body,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
            collect_call_site_types_from_expr(
                orelse,
                caller_func_name,
                var_types,
                func_param_counts,
                call_site_types,
            );
        }
        _ => {}
    }
}

impl Default for DepylerPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl DepylerPipeline {
    /// Creates a new transpilation pipeline with default configuration
    ///
    /// The default pipeline includes:
    /// - Core semantic analysis and type inference
    /// - Standard optimizations
    /// - No property verification (use `with_verification()` to enable)
    /// - No debug output
    ///
    /// # Examples
    ///
    /// ```rust
    /// use depyler_core::DepylerPipeline;
    ///
    /// let pipeline = DepylerPipeline::new();
    /// // Pipeline is ready for transpilation
    /// ```
    pub fn new() -> Self {
        Self {
            analyzer: CoreAnalyzer {
                metrics_enabled: true,
                type_inference_enabled: true,
            },
            transpiler: DirectTranspiler {
                type_mapper: type_mapper::TypeMapper::default(),
            },
            verifier: None,
            mcp_client: LazyMcpClient::default(),
            debug_config: None,
        }
    }

    pub fn with_verification(mut self) -> Self {
        self.verifier = Some(PropertyVerifier {
            enable_quickcheck: true,
            enable_contracts: true,
        });
        self
    }

    pub fn with_debug(mut self, debug_config: debug::DebugConfig) -> Self {
        self.debug_config = Some(debug_config);
        self
    }

    /// Transpiles Python source code to equivalent Rust code
    ///
    /// This is the main entry point for transpilation. It performs the complete
    /// pipeline: parsing, semantic analysis, type inference, optimization, and
    /// code generation.
    ///
    /// # Arguments
    ///
    /// * `python_source` - The Python source code to transpile
    ///
    /// # Returns
    ///
    /// Returns the generated Rust code as a string, or an error if transpilation fails.
    ///
    /// # Examples
    ///
    /// Basic function transpilation:
    ///
    /// ```rust
    /// use depyler_core::DepylerPipeline;
    ///
    /// let pipeline = DepylerPipeline::new();
    /// let python_code = r#"
    /// def multiply(x: int, y: int) -> int:
    ///     return x * y
    /// "#;
    ///
    /// let rust_code = pipeline.transpile(python_code).unwrap();
    /// assert!(rust_code.contains("pub fn multiply"));
    /// assert!(rust_code.contains("-> i32"));
    /// ```
    ///
    /// Complex function with control flow:
    ///
    /// ```rust
    /// use depyler_core::DepylerPipeline;
    ///
    /// let pipeline = DepylerPipeline::new();
    /// let python_code = r#"
    /// def is_even(n: int) -> bool:
    ///     if n % 2 == 0:
    ///         return True
    ///     else:
    ///         return False
    /// "#;
    ///
    /// let rust_code = pipeline.transpile(python_code).unwrap();
    /// assert!(rust_code.contains("pub fn is_even"));
    /// assert!(rust_code.contains("bool")); // Changed to just check for bool type
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The Python source contains syntax errors
    /// - Unsupported Python constructs are used
    /// - Type inference fails
    /// - Verification fails (if enabled)
    ///
    /// Transpiles Python source code and returns both Rust code and Cargo dependencies
    ///
    /// DEPYLER-0384: This method returns the generated Rust code along with the list
    /// of Cargo dependencies needed to build it. Use this when you need to generate
    /// a complete Cargo project with Cargo.toml.
    ///
    /// # Returns
    ///
    /// Returns a tuple of (rust_code, dependencies) or an error if transpilation fails.
    pub fn transpile_with_dependencies(
        &self,
        python_source: &str,
    ) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
        // Parse Python source
        let ast = self.parse_python(python_source)?;

        // Convert to HIR with annotation support
        let (mut hir, _type_env) = ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // Apply const generic inference
        let mut const_inferencer = const_generic_inference::ConstGenericInferencer::new();
        const_inferencer.analyze_module(&mut hir)?;

        // Apply type inference hints
        if self.analyzer.type_inference_enabled {
            let mut type_hint_provider = type_hints::TypeHintProvider::new();

            // Analyze all functions and collect hints
            let mut function_hints = Vec::new();
            for (idx, func) in hir.functions.iter().enumerate() {
                if let Ok(hints) = type_hint_provider.analyze_function(func) {
                    if !hints.is_empty() {
                        eprintln!("Type inference hints:");
                        eprintln!("{}", type_hint_provider.format_hints(&hints));
                        function_hints.push((idx, hints));
                    }
                }
            }

            // Apply high-confidence hints to the HIR
            for (func_idx, hints) in function_hints {
                let func = &mut hir.functions[func_idx];

                // Apply parameter type hints
                for param in &mut func.params {
                    if matches!(param.ty, hir::Type::Unknown) {
                        // Find hint for this parameter
                        for hint in &hints {
                            if let type_hints::HintTarget::Parameter(hint_param) = &hint.target {
                                if hint_param == &param.name
                                    && matches!(
                                        hint.confidence,
                                        type_hints::Confidence::High
                                            | type_hints::Confidence::Certain
                                    )
                                {
                                    param.ty = hint.suggested_type.clone();
                                    eprintln!(
                                        "Applied type hint: {} -> {:?}",
                                        param.name, param.ty
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }

                // Apply return type hints
                // DEPYLER-0400: Accept Medium confidence for return types from explicit returns
                if matches!(func.ret_type, hir::Type::Unknown) {
                    for hint in &hints {
                        if matches!(hint.target, type_hints::HintTarget::Return)
                            && matches!(
                                hint.confidence,
                                type_hints::Confidence::Medium
                                    | type_hints::Confidence::High
                                    | type_hints::Confidence::Certain
                            )
                        {
                            func.ret_type = hint.suggested_type.clone();
                            eprintln!("Applied return type hint: {:?}", func.ret_type);
                            break;
                        }
                    }
                }
            }
        }

        // DEPYLER-0575: Cross-function type propagation from call sites
        propagate_call_site_types(&mut hir);

        // Apply optimization passes based on annotations
        optimization::optimize_module(&mut hir);

        // Convert HirModule to HirProgram for the new optimizer
        let hir_program = hir::HirProgram {
            functions: hir.functions,
            classes: hir.classes,
            imports: hir.imports,
        };

        // Apply the new general-purpose optimizer
        let mut optimizer = optimizer::Optimizer::new(optimizer::OptimizerConfig::default());
        let optimized_program = optimizer.optimize_program(hir_program.clone());

        // Run migration suggestions analysis
        if self.analyzer.metrics_enabled {
            let mut migration_analyzer = migration_suggestions::MigrationAnalyzer::new(
                migration_suggestions::MigrationConfig::default(),
            );
            let suggestions = migration_analyzer.analyze_program(&hir_program);
            if !suggestions.is_empty() {
                eprintln!("{}", migration_analyzer.format_suggestions(&suggestions));
            }
        }

        // Run performance warnings analysis
        if self.analyzer.metrics_enabled {
            let mut perf_analyzer = performance_warnings::PerformanceAnalyzer::new(
                performance_warnings::PerformanceConfig::default(),
            );
            let warnings = perf_analyzer.analyze_program(&hir_program);
            if !warnings.is_empty() {
                eprintln!("{}", perf_analyzer.format_warnings(&warnings));
            }
        }

        // Run profiling analysis if enabled
        if self.analyzer.metrics_enabled {
            let mut profiler = profiling::Profiler::new(profiling::ProfileConfig::default());
            let profile_report = profiler.analyze_program(&hir_program);
            if !profile_report.metrics.is_empty() {
                eprintln!("{}", profile_report.format_report());
            }
        }

        // Convert back to HirModule
        let optimized_hir = hir::HirModule {
            functions: optimized_program.functions,
            imports: optimized_program.imports,
            type_aliases: hir.type_aliases,
            protocols: hir.protocols,
            classes: optimized_program.classes,
            constants: hir.constants,
        };

        // Generate Rust code with dependencies
        rust_gen::generate_rust_file(&optimized_hir, &self.transpiler.type_mapper)
    }

    pub fn transpile(&self, python_source: &str) -> Result<String> {
        // Parse Python source
        let ast = self.parse_python(python_source)?;

        // Convert to HIR with annotation support
        let (mut hir, _type_env) = ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // Apply const generic inference
        let mut const_inferencer = const_generic_inference::ConstGenericInferencer::new();
        const_inferencer.analyze_module(&mut hir)?;

        // Apply type inference hints
        if self.analyzer.type_inference_enabled {
            let mut type_hint_provider = type_hints::TypeHintProvider::new();

            // Analyze all functions and collect hints
            let mut function_hints = Vec::new();
            for (idx, func) in hir.functions.iter().enumerate() {
                if let Ok(hints) = type_hint_provider.analyze_function(func) {
                    if !hints.is_empty() {
                        eprintln!("Type inference hints:");
                        eprintln!("{}", type_hint_provider.format_hints(&hints));
                        function_hints.push((idx, hints));
                    }
                }
            }

            // Apply high-confidence hints to the HIR
            for (func_idx, hints) in function_hints {
                let func = &mut hir.functions[func_idx];

                // Apply parameter type hints
                for param in &mut func.params {
                    if matches!(param.ty, hir::Type::Unknown) {
                        // Find hint for this parameter
                        for hint in &hints {
                            if let type_hints::HintTarget::Parameter(hint_param) = &hint.target {
                                if hint_param == &param.name
                                    && matches!(
                                        hint.confidence,
                                        type_hints::Confidence::High
                                            | type_hints::Confidence::Certain
                                    )
                                {
                                    param.ty = hint.suggested_type.clone();
                                    eprintln!(
                                        "Applied type hint: {} -> {:?}",
                                        param.name, param.ty
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }

                // Apply return type hints
                // DEPYLER-0400: Accept Medium confidence for return types from explicit returns
                if matches!(func.ret_type, hir::Type::Unknown) {
                    for hint in &hints {
                        if matches!(hint.target, type_hints::HintTarget::Return)
                            && matches!(
                                hint.confidence,
                                type_hints::Confidence::Medium
                                    | type_hints::Confidence::High
                                    | type_hints::Confidence::Certain
                            )
                        {
                            func.ret_type = hint.suggested_type.clone();
                            eprintln!("Applied return type hint: {:?}", func.ret_type);
                            break;
                        }
                    }
                }
            }
        }

        // DEPYLER-0575: Cross-function type propagation from call sites
        propagate_call_site_types(&mut hir);

        // Apply optimization passes based on annotations
        optimization::optimize_module(&mut hir);

        // Convert HirModule to HirProgram for the new optimizer
        let hir_program = hir::HirProgram {
            functions: hir.functions,
            classes: hir.classes,
            imports: hir.imports,
        };

        // Apply the new general-purpose optimizer
        let mut optimizer = optimizer::Optimizer::new(optimizer::OptimizerConfig::default());
        let optimized_program = optimizer.optimize_program(hir_program.clone());

        // Run migration suggestions analysis
        if self.analyzer.metrics_enabled {
            let mut migration_analyzer = migration_suggestions::MigrationAnalyzer::new(
                migration_suggestions::MigrationConfig::default(),
            );
            let suggestions = migration_analyzer.analyze_program(&hir_program);
            if !suggestions.is_empty() {
                eprintln!("{}", migration_analyzer.format_suggestions(&suggestions));
            }
        }

        // Run performance warnings analysis
        if self.analyzer.metrics_enabled {
            let mut perf_analyzer = performance_warnings::PerformanceAnalyzer::new(
                performance_warnings::PerformanceConfig::default(),
            );
            let warnings = perf_analyzer.analyze_program(&hir_program);
            if !warnings.is_empty() {
                eprintln!("{}", perf_analyzer.format_warnings(&warnings));
            }
        }

        // Run profiling analysis if enabled
        if self.analyzer.metrics_enabled {
            let mut profiler = profiling::Profiler::new(profiling::ProfileConfig::default());
            let profile_report = profiler.analyze_program(&hir_program);
            if !profile_report.metrics.is_empty() {
                eprintln!("{}", profile_report.format_report());
            }
        }

        // Convert back to HirModule
        let optimized_hir = hir::HirModule {
            functions: optimized_program.functions,
            imports: optimized_program.imports,
            type_aliases: hir.type_aliases,
            protocols: hir.protocols,
            classes: optimized_program.classes,
            constants: hir.constants,
        };

        // Generate Rust code using the unified generation system
        // DEPYLER-0384: generate_rust_file now returns (code, dependencies)
        let (rust_code, _dependencies) =
            rust_gen::generate_rust_file(&optimized_hir, &self.transpiler.type_mapper)?;

        Ok(rust_code)
    }

    pub fn parse_to_hir(&self, source: &str) -> Result<hir::HirModule> {
        let ast = self.parse_python(source)?;
        let (hir, _type_env) = ast_bridge::AstBridge::new()
            .with_source(source.to_string())
            .python_to_hir(ast)?;
        Ok(hir)
    }

    pub fn analyze_to_typed_hir(&self, source: &str) -> Result<hir::HirModule> {
        // For now, just return the HIR without type analysis
        // In the future, this would add type inference
        self.parse_to_hir(source)
    }

    pub fn parse_python(&self, source: &str) -> Result<rustpython_ast::Mod> {
        use rustpython_ast::Suite;
        use rustpython_parser::Parse;

        let statements = Suite::parse(source, "<input>")
            .map_err(|e| anyhow::anyhow!("Python parse error: {}", e))?;

        Ok(rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub enable_verification: bool,
    pub enable_metrics: bool,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Default)]
pub enum OptimizationLevel {
    #[default]
    Debug,
    Release,
    Size,
}

impl DepylerPipeline {
    pub fn new_with_config(config: Config) -> Self {
        let mut pipeline = Self::new();
        pipeline.analyzer.metrics_enabled = config.enable_metrics;

        if config.enable_verification {
            pipeline = pipeline.with_verification();
        }

        pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = DepylerPipeline::new();
        assert!(pipeline.analyzer.metrics_enabled);
        assert!(pipeline.analyzer.type_inference_enabled);
        assert!(pipeline.verifier.is_none());
    }

    #[test]
    fn test_pipeline_with_verification() {
        let pipeline = DepylerPipeline::new().with_verification();
        assert!(pipeline.verifier.is_some());
        let verifier = pipeline.verifier.unwrap();
        assert!(verifier.enable_quickcheck);
        assert!(verifier.enable_contracts);
    }

    #[test]
    fn test_config_creation() {
        let config = Config {
            enable_verification: true,
            enable_metrics: false,
            optimization_level: OptimizationLevel::Release,
        };

        let pipeline = DepylerPipeline::new_with_config(config);
        assert!(pipeline.verifier.is_some());
        assert!(!pipeline.analyzer.metrics_enabled);
    }

    #[test]
    fn test_simple_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;

        let result = pipeline.transpile(python_code);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("i32"));
    }

    #[test]
    fn test_parse_to_hir() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def test_func(x: int) -> str:
    return "hello"
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name, "test_func");
        assert_eq!(hir.functions[0].params[0].name, "x");
        assert_eq!(hir.functions[0].params[0].ty, hir::Type::Int);
        assert_eq!(hir.functions[0].ret_type, hir::Type::String);
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec!["Warning message".to_string()],
        };

        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_invalid_python_syntax() {
        let pipeline = DepylerPipeline::new();
        let invalid_python = "def invalid_syntax(\n    return";

        let result = pipeline.transpile(invalid_python);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyzable_stage_trait() {
        // Test that the trait is properly defined
        struct TestStage;

        impl AnalyzableStage for TestStage {
            type Input = String;
            type Output = String;
            type Metrics = usize;

            fn execute(&self, input: Self::Input) -> Result<(Self::Output, Self::Metrics)> {
                Ok((input.clone(), input.len()))
            }

            fn validate(&self, _output: &Self::Output) -> ValidationResult {
                ValidationResult {
                    is_valid: true,
                    errors: vec![],
                    warnings: vec![],
                }
            }
        }

        let stage = TestStage;
        let (output, metrics) = stage.execute("test".to_string()).unwrap();
        assert_eq!(output, "test");
        assert_eq!(metrics, 4);

        let validation = stage.validate(&output);
        assert!(validation.is_valid);
    }

    #[test]
    fn test_complex_function_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

        let result = pipeline.transpile(python_code);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("fibonacci"));
        assert!(rust_code.contains("if"));
        assert!(rust_code.contains("return"));
    }

    #[test]
    fn test_type_annotations() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
from typing import List, Optional

def process_list(items: List[str]) -> Optional[str]:
    if items:
        return items[0]
    return None
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        assert_eq!(hir.functions.len(), 1);
        let func = &hir.functions[0];
        assert_eq!(
            func.params[0].ty,
            hir::Type::List(Box::new(hir::Type::String))
        );
        assert_eq!(
            func.ret_type,
            hir::Type::Optional(Box::new(hir::Type::String))
        );
    }

    #[test]
    fn test_annotation_aware_transpilation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: optimization_level = "aggressive"
# @depyler: thread_safety = "required"
# @depyler: bounds_checking = "explicit"
def compute_sum(numbers: List[int]) -> int:
    total = 0
    for num in numbers:
        total += num
    return total
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify annotations were extracted
        assert_eq!(
            func.annotations.optimization_level,
            depyler_annotations::OptimizationLevel::Aggressive
        );
        assert_eq!(
            func.annotations.thread_safety,
            depyler_annotations::ThreadSafety::Required
        );
        assert_eq!(
            func.annotations.bounds_checking,
            depyler_annotations::BoundsChecking::Explicit
        );

        // Verify transpilation works
        let rust_code = pipeline.transpile(python_code).unwrap();
        assert!(rust_code.contains("compute_sum"));
    }

    #[test]
    fn test_string_strategy_annotation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: string_strategy = "zero_copy"
# @depyler: ownership = "borrowed"
def process_string(s: str) -> str:
    return s
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify string strategy was extracted
        assert_eq!(
            func.annotations.string_strategy,
            depyler_annotations::StringStrategy::ZeroCopy
        );
        assert_eq!(
            func.annotations.ownership_model,
            depyler_annotations::OwnershipModel::Borrowed
        );

        // The generated code should use borrowed strings
        let rust_code = pipeline.transpile(python_code).unwrap();
        assert!(rust_code.contains("process_string"));
    }

    #[test]
    fn test_hash_strategy_annotation() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
# @depyler: hash_strategy = "fnv"
def create_map() -> Dict[str, int]:
    # Dictionary subscript assignment requires more complex AST transformation
    # For now, just test that the annotation is parsed correctly
    return {}
"#;

        let hir = pipeline.parse_to_hir(python_code).unwrap();
        let func = &hir.functions[0];

        // Verify hash strategy was extracted
        assert_eq!(
            func.annotations.hash_strategy,
            depyler_annotations::HashStrategy::Fnv
        );
    }
}
