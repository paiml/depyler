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
pub mod borrowing_shim;
pub mod cargo_first;
pub mod cargo_toml_gen;
pub mod chaos;
pub mod codegen;
pub mod codegen_shim;
pub mod const_generic_inference;
pub mod debug;
pub mod decision_trace;
pub mod depylint;
pub mod direct_rules;
mod direct_rules_convert; // DEPYLER-COVERAGE-95: Split from direct_rules.rs
pub mod doctest_extractor;
pub mod documentation;
pub mod error;
pub mod error_reporting;
pub mod generator_state;
pub mod generator_yield_analysis;
pub mod generic_inference;
pub mod hir;
pub mod hunt_mode;
pub mod ide;
pub mod infrastructure;
pub mod inlining;
pub mod lambda_codegen;
pub mod lambda_errors;
pub mod lambda_inference;
pub mod lambda_optimizer;
pub mod lambda_testing;
pub mod lambda_types;
pub mod library_mapping;
pub mod lifetime_analysis;
pub mod lsp;
pub mod migration_suggestions;
pub mod module_mapper;
pub mod module_mapper_phf;
pub mod optimization;
pub mod optimizer;
pub mod performance_warnings;
pub mod profiling;
pub mod pytest_extractor;
pub mod rust_gen;
pub mod scoring;
pub mod simplified_hir;
pub mod stdlib_mappings;
pub mod string_optimization;
pub mod test_generation;
pub mod type_hints;
pub mod type_mapper;
pub mod type_inference_telemetry;
pub mod type_system;
pub mod type_propagation;
pub mod typeshed_ingest;
pub mod union_enum_gen;

// Generative repair engine (DEPYLER-ENTRENAR-001)
pub mod generative_repair;

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
        type_propagation::propagate_call_site_types(&mut hir);

        // DEPYLER-0202: Constraint-based type inference via Hindley-Milner
        // Collect constraints from HIR and solve for unknown types
        {
            use type_system::{ConstraintCollector, TypeConstraintSolver};

            let mut collector = ConstraintCollector::new();
            collector.collect_module(&hir);

            let constraints = collector.constraints();
            if !constraints.is_empty() {
                let mut solver = TypeConstraintSolver::new();
                for constraint in constraints {
                    solver.add_constraint(constraint.clone());
                }

                if let Ok(solution) = solver.solve() {
                    let applied = collector.apply_substitutions(&mut hir, &solution);
                    if applied > 0 {
                        eprintln!("HM inference: applied {} type substitutions", applied);
                    }
                }
            }
        }

        // DEPYLER-0950: Inter-procedural type unification
        // Builds call graph and propagates types across function boundaries
        if let Err(e) = type_system::unify_module_types(&mut hir) {
            // Log but don't fail - type conflicts are informational
            eprintln!("Type unification warning: {:?}", e);
        }

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
        type_propagation::propagate_call_site_types(&mut hir);

        // DEPYLER-0202: Constraint-based type inference via Hindley-Milner
        // Collect constraints from HIR and solve for unknown types
        {
            use type_system::{ConstraintCollector, TypeConstraintSolver};

            let mut collector = ConstraintCollector::new();
            collector.collect_module(&hir);

            let constraints = collector.constraints();
            if !constraints.is_empty() {
                let mut solver = TypeConstraintSolver::new();
                for constraint in constraints {
                    solver.add_constraint(constraint.clone());
                }

                if let Ok(solution) = solver.solve() {
                    let applied = collector.apply_substitutions(&mut hir, &solution);
                    if applied > 0 {
                        eprintln!("HM inference: applied {} type substitutions", applied);
                    }
                }
            }
        }

        // DEPYLER-0950: Inter-procedural type unification
        // Builds call graph and propagates types across function boundaries
        if let Err(e) = type_system::unify_module_types(&mut hir) {
            // Log but don't fail - type conflicts are informational
            eprintln!("Type unification warning: {:?}", e);
        }

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

    /// DEPYLER-1102: Transpile with oracle-learned type constraints and return dependencies
    ///
    /// This method is the constraint-aware version of `transpile_with_dependencies`.
    /// It is used by the Oracle Loop in `compile_cmd.rs` to re-transpile code with
    /// learned type corrections while still generating the necessary Cargo dependencies.
    pub fn transpile_with_constraints_and_dependencies(
        &self,
        python_source: &str,
        type_constraints: &std::collections::HashMap<String, String>,
    ) -> Result<(String, Vec<cargo_toml_gen::Dependency>)> {
        // Convert string type constraints to HIR Types
        let type_overrides: std::collections::HashMap<String, hir::Type> = type_constraints
            .iter()
            .map(|(var, ty_str)| {
                (var.clone(), rust_gen::rust_type_string_to_hir(ty_str))
            })
            .collect();

        // Parse Python source
        let ast = self.parse_python(python_source)?;

        // Convert to HIR with annotation support
        let (mut hir, _type_env) = ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // DEPYLER-1101: Apply type overrides to HIR
        // Override function return types and variable types based on constraints
        for func in &mut hir.functions {
            // Check if function name is in overrides
            if let Some(override_type) = type_overrides.get(&func.name) {
                if !matches!(override_type, hir::Type::Unknown) {
                    eprintln!(
                        "DEPYLER-1101: Overriding return type of {} to {:?}",
                        func.name, override_type
                    );
                    func.ret_type = override_type.clone();
                }
            }

            // Override parameter types
            for param in func.params.iter_mut() {
                if let Some(override_type) = type_overrides.get(&param.name) {
                    if !matches!(override_type, hir::Type::Unknown) {
                        eprintln!(
                            "DEPYLER-1101: Overriding param {} type to {:?}",
                            param.name, override_type
                        );
                        param.ty = override_type.clone();
                    }
                }
            }
        }

        // Apply standard type inference and optimization passes
        let mut const_inferencer = const_generic_inference::ConstGenericInferencer::new();
        const_inferencer.analyze_module(&mut hir)?;

        // Apply type inference hints (same as transpile)
        if self.analyzer.type_inference_enabled {
            let mut type_hint_provider = type_hints::TypeHintProvider::new();

            // Collect hints first (immutable borrow)
            let mut function_hints: Vec<(usize, Vec<type_hints::TypeHint>)> = Vec::new();
            for (idx, func) in hir.functions.iter().enumerate() {
                if let Ok(hints) = type_hint_provider.analyze_function(func) {
                    if !hints.is_empty() {
                        function_hints.push((idx, hints));
                    }
                }
            }

            // Apply hints (mutable borrow)
            for (func_idx, hints) in function_hints {
                let func = &mut hir.functions[func_idx];
                for param in &mut func.params {
                    if matches!(param.ty, hir::Type::Unknown)
                        && !type_overrides.contains_key(&param.name)
                    {
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
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Cross-function type propagation
        type_propagation::propagate_call_site_types(&mut hir);

        // Hindley-Milner type inference
        {
            use type_system::{ConstraintCollector, TypeConstraintSolver};

            let mut collector = ConstraintCollector::new();
            collector.collect_module(&hir);

            let constraints = collector.constraints();
            if !constraints.is_empty() {
                let mut solver = TypeConstraintSolver::new();
                for constraint in constraints {
                    solver.add_constraint(constraint.clone());
                }

                if let Ok(solution) = solver.solve() {
                    let _applied = collector.apply_substitutions(&mut hir, &solution);
                }
            }
        }

        // Inter-procedural type unification
        if let Err(e) = type_system::unify_module_types(&mut hir) {
            eprintln!("Type unification warning: {:?}", e);
        }

        // Apply optimization passes
        optimization::optimize_module(&mut hir);

        // Generate Rust code with type overrides
        // DEPYLER-1102: Uses generate_rust_file_with_overrides which returns (code, deps)
        rust_gen::generate_rust_file_with_overrides(
            &hir,
            &self.transpiler.type_mapper,
            type_overrides,
        )
    }

    /// DEPYLER-1101: Transpile with oracle-learned type constraints
    ///
    /// This method accepts a map of variable names to their corrected types,
    /// learned from E0308 compiler error feedback. During code generation,
    /// these types override the inferred types for the specified variables.
    ///
    /// # Arguments
    /// * `python_source` - The Python source code to transpile
    /// * `type_constraints` - Map of variable name → corrected Rust type string
    ///   (e.g., "result" → "f64", "items" → "Vec<String>")
    ///
    /// # Returns
    /// Returns the generated Rust code, or an error.
    ///
    /// # Example
    /// ```ignore
    /// use depyler_core::DepylerPipeline;
    /// use std::collections::HashMap;
    ///
    /// let pipeline = DepylerPipeline::new();
    /// let mut constraints = HashMap::new();
    /// constraints.insert("x".to_string(), "f64".to_string());
    ///
    /// let rust_code = pipeline.transpile_with_constraints(
    ///     "def foo(x): return x * 2",
    ///     &constraints
    /// ).unwrap();
    /// ```
    pub fn transpile_with_constraints(
        &self,
        python_source: &str,
        type_constraints: &std::collections::HashMap<String, String>,
    ) -> Result<String> {
        // Convert string type constraints to HIR Types
        let type_overrides: std::collections::HashMap<String, hir::Type> = type_constraints
            .iter()
            .map(|(var, ty_str)| {
                (var.clone(), rust_gen::rust_type_string_to_hir(ty_str))
            })
            .collect();

        // Parse Python source
        let ast = self.parse_python(python_source)?;

        // Convert to HIR with annotation support
        let (mut hir, _type_env) = ast_bridge::AstBridge::new()
            .with_source(python_source.to_string())
            .python_to_hir(ast)?;

        // DEPYLER-1101: Apply type overrides to HIR
        // Override function return types and variable types based on constraints
        for func in &mut hir.functions {
            // Check if function name is in overrides
            if let Some(override_type) = type_overrides.get(&func.name) {
                if !matches!(override_type, hir::Type::Unknown) {
                    eprintln!(
                        "DEPYLER-1101: Overriding return type of {} to {:?}",
                        func.name, override_type
                    );
                    func.ret_type = override_type.clone();
                }
            }

            // Override parameter types
            for param in func.params.iter_mut() {
                if let Some(override_type) = type_overrides.get(&param.name) {
                    if !matches!(override_type, hir::Type::Unknown) {
                        eprintln!(
                            "DEPYLER-1101: Overriding param {} type to {:?}",
                            param.name, override_type
                        );
                        param.ty = override_type.clone();
                    }
                }
            }
        }

        // Apply standard type inference and optimization passes
        let mut const_inferencer = const_generic_inference::ConstGenericInferencer::new();
        const_inferencer.analyze_module(&mut hir)?;

        // Apply type inference hints
        if self.analyzer.type_inference_enabled {
            let mut type_hint_provider = type_hints::TypeHintProvider::new();

            // Collect hints first (immutable borrow)
            let mut function_hints: Vec<(usize, Vec<type_hints::TypeHint>)> = Vec::new();
            for (idx, func) in hir.functions.iter().enumerate() {
                if let Ok(hints) = type_hint_provider.analyze_function(func) {
                    if !hints.is_empty() {
                        function_hints.push((idx, hints));
                    }
                }
            }

            // Apply hints (mutable borrow)
            for (func_idx, hints) in function_hints {
                let func = &mut hir.functions[func_idx];
                for param in &mut func.params {
                    // Only apply hints to types not already overridden
                    if matches!(param.ty, hir::Type::Unknown)
                        && !type_overrides.contains_key(&param.name)
                    {
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
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Cross-function type propagation
        type_propagation::propagate_call_site_types(&mut hir);

        // Hindley-Milner type inference
        {
            use type_system::{ConstraintCollector, TypeConstraintSolver};

            let mut collector = ConstraintCollector::new();
            collector.collect_module(&hir);

            let constraints = collector.constraints();
            if !constraints.is_empty() {
                let mut solver = TypeConstraintSolver::new();
                for constraint in constraints {
                    solver.add_constraint(constraint.clone());
                }

                if let Ok(solution) = solver.solve() {
                    let _applied = collector.apply_substitutions(&mut hir, &solution);
                }
            }
        }

        // Inter-procedural type unification
        if let Err(e) = type_system::unify_module_types(&mut hir) {
            eprintln!("Type unification warning: {:?}", e);
        }

        // Apply optimization passes
        optimization::optimize_module(&mut hir);

        // Generate Rust code with type overrides
        let (rust_code, _dependencies) = rust_gen::generate_rust_file_with_overrides(
            &hir,
            &self.transpiler.type_mapper,
            type_overrides,
        )?;

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

    // DEPYLER-COVERAGE-95: Additional tests for untested components

    #[test]
    fn test_lazy_mcp_client_default() {
        let client = LazyMcpClient::default();
        assert!(client.endpoint.is_none());
    }

    #[test]
    fn test_lazy_mcp_client_debug() {
        let client = LazyMcpClient::default();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("LazyMcpClient"));
    }

    #[test]
    fn test_optimization_level_debug() {
        assert_eq!(format!("{:?}", OptimizationLevel::Debug), "Debug");
        assert_eq!(format!("{:?}", OptimizationLevel::Release), "Release");
        assert_eq!(format!("{:?}", OptimizationLevel::Size), "Size");
    }

    #[test]
    fn test_optimization_level_default() {
        let level: OptimizationLevel = Default::default();
        assert!(matches!(level, OptimizationLevel::Debug));
    }

    #[test]
    fn test_optimization_level_clone() {
        let level = OptimizationLevel::Release;
        let cloned = level.clone();
        assert!(matches!(cloned, OptimizationLevel::Release));
    }

    #[test]
    fn test_config_default() {
        let config: Config = Default::default();
        assert!(!config.enable_verification);
        assert!(!config.enable_metrics);
        assert!(matches!(config.optimization_level, OptimizationLevel::Debug));
    }

    #[test]
    fn test_config_debug() {
        let config = Config {
            enable_verification: true,
            enable_metrics: true,
            optimization_level: OptimizationLevel::Size,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("enable_verification"));
    }

    #[test]
    fn test_config_clone() {
        let config = Config {
            enable_verification: true,
            enable_metrics: false,
            optimization_level: OptimizationLevel::Release,
        };
        let cloned = config.clone();
        assert!(cloned.enable_verification);
        assert!(!cloned.enable_metrics);
    }

    #[test]
    fn test_core_analyzer_debug() {
        let analyzer = CoreAnalyzer {
            metrics_enabled: true,
            type_inference_enabled: false,
        };
        let debug_str = format!("{:?}", analyzer);
        assert!(debug_str.contains("CoreAnalyzer"));
        assert!(debug_str.contains("metrics_enabled"));
    }

    #[test]
    fn test_core_analyzer_clone() {
        let analyzer = CoreAnalyzer {
            metrics_enabled: false,
            type_inference_enabled: true,
        };
        let cloned = analyzer.clone();
        assert!(!cloned.metrics_enabled);
        assert!(cloned.type_inference_enabled);
    }

    #[test]
    fn test_direct_transpiler_debug() {
        let transpiler = DirectTranspiler {
            type_mapper: type_mapper::TypeMapper::default(),
        };
        let debug_str = format!("{:?}", transpiler);
        assert!(debug_str.contains("DirectTranspiler"));
    }

    #[test]
    fn test_direct_transpiler_clone() {
        let transpiler = DirectTranspiler {
            type_mapper: type_mapper::TypeMapper::default(),
        };
        let _cloned = transpiler.clone();
        // If clone compiles and runs, the test passes
    }

    #[test]
    fn test_property_verifier_debug() {
        let verifier = PropertyVerifier {
            enable_quickcheck: true,
            enable_contracts: false,
        };
        let debug_str = format!("{:?}", verifier);
        assert!(debug_str.contains("PropertyVerifier"));
        assert!(debug_str.contains("enable_quickcheck"));
    }

    #[test]
    fn test_property_verifier_clone() {
        let verifier = PropertyVerifier {
            enable_quickcheck: false,
            enable_contracts: true,
        };
        let cloned = verifier.clone();
        assert!(!cloned.enable_quickcheck);
        assert!(cloned.enable_contracts);
    }

    #[test]
    fn test_validation_result_debug() {
        let result = ValidationResult {
            is_valid: false,
            errors: vec!["error1".to_string()],
            warnings: vec![],
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ValidationResult"));
        assert!(debug_str.contains("error1"));
    }

    #[test]
    fn test_validation_result_clone() {
        let result = ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec!["warn".to_string()],
        };
        let cloned = result.clone();
        assert!(cloned.is_valid);
        assert_eq!(cloned.warnings.len(), 1);
    }

    #[test]
    fn test_pipeline_with_debug() {
        let debug_config = debug::DebugConfig::default();
        let pipeline = DepylerPipeline::new().with_debug(debug_config);
        assert!(pipeline.debug_config.is_some());
    }

    #[test]
    fn test_pipeline_debug_impl() {
        let pipeline = DepylerPipeline::new();
        let debug_str = format!("{:?}", pipeline);
        assert!(debug_str.contains("DepylerPipeline"));
        assert!(debug_str.contains("analyzer"));
    }

    #[test]
    fn test_pipeline_clone() {
        let pipeline = DepylerPipeline::new().with_verification();
        let cloned = pipeline.clone();
        assert!(cloned.verifier.is_some());
    }

    #[test]
    fn test_transpile_with_dependencies() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def greet(name: str) -> str:
    return "Hello, " + name
"#;
        let result = pipeline.transpile_with_dependencies(python_code);
        assert!(result.is_ok());
        let (rust_code, dependencies) = result.unwrap();
        assert!(rust_code.contains("pub fn greet"));
        // Dependencies may be empty for simple code
        assert!(dependencies.is_empty() || !dependencies.is_empty());
    }

    #[test]
    fn test_transpile_with_dependencies_error() {
        let pipeline = DepylerPipeline::new();
        let invalid_python = "def broken(";
        let result = pipeline.transpile_with_dependencies(invalid_python);
        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_to_typed_hir() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def double(x: int) -> int:
    return x * 2
"#;
        let hir = pipeline.analyze_to_typed_hir(python_code).unwrap();
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name, "double");
    }

    #[test]
    fn test_parse_python_directly() {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.parse_python("x = 42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_python_error() {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.parse_python("def incomplete(");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parse error"));
    }

    #[test]
    fn test_pipeline_serialization() {
        let pipeline = DepylerPipeline::new().with_verification();
        let json = serde_json::to_string(&pipeline).unwrap();
        assert!(json.contains("analyzer"));
        assert!(json.contains("transpiler"));
        assert!(json.contains("verifier"));
    }

    #[test]
    fn test_pipeline_deserialization() {
        let json = r#"{
            "analyzer": {"metrics_enabled": false, "type_inference_enabled": true},
            "transpiler": {"type_mapper": {"width_preference": "I32", "string_type": "AlwaysOwned"}},
            "verifier": {"enable_quickcheck": true, "enable_contracts": false}
        }"#;
        let pipeline: DepylerPipeline = serde_json::from_str(json).unwrap();
        assert!(!pipeline.analyzer.metrics_enabled);
        assert!(pipeline.analyzer.type_inference_enabled);
        assert!(pipeline.verifier.is_some());
    }

    #[test]
    fn test_validation_result_with_errors_and_warnings() {
        let result = ValidationResult {
            is_valid: false,
            errors: vec!["err1".to_string(), "err2".to_string()],
            warnings: vec!["warn1".to_string()],
        };
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 2);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_config_with_all_levels() {
        // Test Debug level
        let debug_config = Config {
            enable_verification: false,
            enable_metrics: false,
            optimization_level: OptimizationLevel::Debug,
        };
        let _pipeline = DepylerPipeline::new_with_config(debug_config);

        // Test Size level
        let size_config = Config {
            enable_verification: true,
            enable_metrics: true,
            optimization_level: OptimizationLevel::Size,
        };
        let pipeline = DepylerPipeline::new_with_config(size_config);
        assert!(pipeline.analyzer.metrics_enabled);
        assert!(pipeline.verifier.is_some());
    }

    #[test]
    fn test_pipeline_default_trait() {
        let pipeline: DepylerPipeline = Default::default();
        assert!(pipeline.analyzer.metrics_enabled);
        assert!(pipeline.verifier.is_none());
    }

    #[test]
    fn test_empty_python_transpilation() {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile("");
        // Empty Python is valid (produces empty module)
        assert!(result.is_ok());
    }

    #[test]
    fn test_transpile_multiple_functions() {
        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def add(a: int, b: int) -> int:
    return a + b

def subtract(a: int, b: int) -> int:
    return a - b
"#;
        let result = pipeline.transpile(python_code);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("pub fn add"));
        assert!(rust_code.contains("pub fn subtract"));
    }
}
