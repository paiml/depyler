//! Code generation context and core traits
//!
//! This module provides the central CodeGenContext struct that maintains
//! state during Rust code generation, along with core traits used across
//! the code generation pipeline.

use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::borrowing_context::BorrowingStrategy;
use crate::hir::{ExceptionScope, Type};
use crate::string_optimization::StringOptimizer;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Error type classification for Result<T, E> return types
///
/// DEPYLER-0310: Tracks whether function uses Box<dyn Error> (mixed types)
/// or a concrete error type (single type). This determines if raise statements
/// need Box::new() wrapper.
///
/// # Complexity
/// N/A (enum definition)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    /// Concrete error type (e.g., ValueError, ZeroDivisionError)
    /// No wrapping needed: `return Err(ValueError::new(...))`
    Concrete(String),
    /// Box<dyn Error> - mixed or generic error types
    /// Needs wrapping: `return Err(Box::new(ValueError::new(...)))`
    DynBox,
}

/// Code generation context
///
/// Maintains all state needed during Rust code generation including:
/// - Type mapping and string optimization
/// - Import tracking (needs_hashmap, needs_cow, etc.)
/// - Variable scoping and mutability analysis
/// - Generator state management
///
/// # Complexity
/// N/A (data structure)
#[derive(Debug, Clone)]
pub struct ParamBorrowInfo {
    pub should_borrow: bool,
    pub needs_mut: bool,
    pub takes_ownership: bool,
}

pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub annotation_aware_mapper: AnnotationAwareTypeMapper,
    pub string_optimizer: StringOptimizer,
    pub union_enum_generator: crate::union_enum_gen::UnionEnumGenerator,
    pub generated_enums: Vec<proc_macro2::TokenStream>,
    pub needs_hashmap: bool,
    pub needs_hashset: bool,
    pub needs_vecdeque: bool,
    pub needs_fnv_hashmap: bool,
    pub needs_ahash_hashmap: bool,
    pub needs_arc: bool,
    pub needs_rc: bool,
    pub needs_cow: bool,
    pub needs_rand: bool,
    pub needs_serde_json: bool,
    pub needs_regex: bool,
    pub needs_chrono: bool,
    pub needs_clap: bool, // DEPYLER-0384: Track clap dependency for ArgumentParser
    pub needs_csv: bool,
    pub needs_rust_decimal: bool,
    pub needs_num_rational: bool,
    pub needs_base64: bool,
    pub needs_md5: bool,
    pub needs_sha2: bool,
    pub needs_sha3: bool,
    pub needs_blake2: bool,
    pub needs_hex: bool,
    pub needs_uuid: bool,
    pub needs_hmac: bool,
    pub needs_crc32: bool,
    pub needs_url_encoding: bool,
    pub declared_vars: Vec<HashSet<String>>,
    pub current_function_can_fail: bool,
    pub current_function_name: Option<String>,
    pub current_return_type: Option<Type>,
    pub module_mapper: crate::module_mapper::ModuleMapper,
    pub imported_modules: std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    pub imported_items: std::collections::HashMap<String, String>,
    pub mutable_vars: HashSet<String>,
    pub needs_zerodivisionerror: bool,
    pub needs_indexerror: bool,
    pub needs_valueerror: bool,
    pub needs_argumenttypeerror: bool,
    pub is_classmethod: bool,
    pub in_generator: bool,
    pub generator_state_vars: HashSet<String>,
    pub var_types: HashMap<String, Type>,
    pub class_names: HashSet<String>,
    pub mutating_methods: HashMap<String, HashSet<String>>,
    /// DEPYLER-0269: Track function return types for Display trait selection
    /// Maps function name -> return type, populated during function generation
    /// Used to track types of variables assigned from function calls
    pub function_return_types: HashMap<String, Type>,
    /// DEPYLER-0270: Track function parameter borrowing for auto-borrow decisions
    /// Maps function name -> Vec of parameter borrowing metadata
    /// Used to determine whether to add &, &mut when passing List/Dict/Set arguments
    pub function_param_borrows: HashMap<String, Vec<ParamBorrowInfo>>,
    /// Borrowing strategies inferred for each function parameter (called function)
    /// Used by subsequent lifetime analyses to understand callee ownership semantics
    pub function_param_strategies: HashMap<String, Vec<BorrowingStrategy>>,
    /// Maps parameter names in the CURRENT function to their ownership information
    /// Populated when entering a function, cleared when exiting
    /// Used to determine if .into_iter() or .iter() should be used for parameters
    pub current_function_param_ownership: HashMap<String, bool>, // param_name -> takes_ownership
    /// Parameters in the CURRENT function that require cloning before being passed by value
    /// Populated per-function from lifetime analysis results
    pub param_clone_requirements: HashSet<String>,
    /// DEPYLER-0307 Fix #9: Track variables that iterate over tuples (from zip())
    /// Used to generate tuple field access syntax (tuple.0, tuple.1) instead of vector indexing
    pub tuple_iter_vars: HashSet<String>,
    /// DEPYLER-0271: Tracks if current statement is the final statement in its block
    /// Used to generate idiomatic expression-based returns (no `return` keyword)
    pub is_final_statement: bool,
    /// DEPYLER-0308: Track functions that return Result<bool, E>
    /// Used to auto-unwrap in boolean contexts (if/while conditions)
    pub result_bool_functions: HashSet<String>,
    /// DEPYLER-0270: Track ALL functions that return Result<T, E>
    /// Used to auto-unwrap at call sites in non-Result functions
    pub result_returning_functions: HashSet<String>,
    /// DEPYLER-0310: Current function's error type (for raise statement wrapping)
    /// None if function doesn't return Result, Some(ErrorType) if it does
    pub current_error_type: Option<ErrorType>,
    /// DEPYLER-0333: Stack of exception scopes for try/except tracking
    /// Tracks whether code is inside try/except blocks to determine error handling strategy
    /// Empty stack = Unhandled scope (exceptions propagate to caller)
    pub exception_scopes: Vec<ExceptionScope>,
    /// DEPYLER-0363: Track ArgumentParser patterns for clap transformation
    /// Accumulates ArgumentParser instances and add_argument calls
    /// to generate #[derive(Parser)] struct definitions
    pub argparser_tracker: crate::rust_gen::argparse_transform::ArgParserTracker,
    /// DEPYLER-0424: Generated Args struct for ArgumentParser (emitted at module level)
    /// Stored here so it can be hoisted outside main() function
    pub generated_args_struct: Option<proc_macro2::TokenStream>,
    /// DEPYLER-0424: Generated Commands enum for subcommands (emitted at module level)
    /// Stored here so it can be hoisted outside main() function
    pub generated_commands_enum: Option<proc_macro2::TokenStream>,
    /// DEPYLER-0425: Current function's subcommand fields (for expression rewriting)
    /// If current function accesses subcommand fields, this maps field names to variant name
    /// Used by expr_gen to rewrite args.field → field (extracted via pattern matching)
    pub current_subcommand_fields: Option<std::collections::HashSet<String>>,
    /// These functions should have &str parameter type regardless of type inference
    /// Populated when processing add_argument(type=validator_func) calls
    pub validator_functions: std::collections::HashSet<String>,
    /// DEPYLER-0452: Stdlib API mapping system for Python→Rust API translations
    /// Maps Python stdlib patterns (module, class, attribute) to Rust code patterns
    pub stdlib_mappings: crate::stdlib_mappings::StdlibMappings,
    /// Interprocedural analysis results for cross-function mutation detection
    /// Used by lifetime analysis to determine parameter mutability across function boundaries
    pub interprocedural_analysis: Option<&'a crate::interprocedural::InterproceduralAnalysis<'a>>,
}

impl<'a> CodeGenContext<'a> {
    /// Enter a new lexical scope
    ///
    /// # Complexity
    /// 1 (simple push)
    pub fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    /// Exit the current lexical scope
    ///
    /// # Complexity
    /// 1 (simple pop)
    pub fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    /// Check if a variable is declared in any scope
    ///
    /// # Complexity
    /// 2 (iterator + any)
    pub fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    /// Declare a variable in the current scope
    ///
    /// # Complexity
    /// 2 (if let + insert)
    pub fn declare_var(&mut self, var_name: &str) {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string());
        }
    }

    /// Process a Union type and generate an enum if needed
    ///
    /// Returns the enum name and optionally generates an enum definition
    /// that is added to generated_enums.
    ///
    /// # Complexity
    /// 2 (if + push)
    pub fn process_union_type(&mut self, types: &[crate::hir::Type]) -> String {
        let (enum_name, enum_def) = self.union_enum_generator.generate_union_enum(types);
        if !enum_def.is_empty() {
            self.generated_enums.push(enum_def);
        }
        enum_name
    }

    // ========================================================================
    // DEPYLER-0333: Exception Scope Tracking
    // ========================================================================

    /// Get the current exception scope
    ///
    /// Returns the most recent scope from the stack, or Unhandled if stack is empty.
    ///
    /// # Complexity
    /// 2 (last + unwrap_or)
    pub fn current_exception_scope(&self) -> &ExceptionScope {
        self.exception_scopes
            .last()
            .unwrap_or(&ExceptionScope::Unhandled)
    }

    /// Check if currently inside a try block
    ///
    /// # Complexity
    /// 2 (current_exception_scope + matches)
    pub fn is_in_try_block(&self) -> bool {
        matches!(
            self.current_exception_scope(),
            ExceptionScope::TryCaught { .. }
        )
    }

    /// Check if a specific exception type is handled by current try block
    ///
    /// Returns true if:
    /// - Inside a try block with bare except (empty handled_types)
    /// - Inside a try block that explicitly handles this exception type
    ///
    /// # Complexity
    /// 4 (match + is_empty + contains + comparison)
    pub fn is_exception_handled(&self, exception_type: &str) -> bool {
        if let ExceptionScope::TryCaught { handled_types } = self.current_exception_scope() {
            // Empty list = bare except (catches all)
            handled_types.is_empty() || handled_types.contains(&exception_type.to_string())
        } else {
            false
        }
    }

    /// Enter a try block scope with specified exception handlers
    ///
    /// # Complexity
    /// 1 (simple push)
    pub fn enter_try_scope(&mut self, handled_types: Vec<String>) {
        self.exception_scopes
            .push(ExceptionScope::TryCaught { handled_types });
    }

    /// Enter an exception handler scope
    ///
    /// # Complexity
    /// 1 (simple push)
    pub fn enter_handler_scope(&mut self) {
        self.exception_scopes.push(ExceptionScope::Handler);
    }

    /// Exit the current exception scope
    ///
    /// # Complexity
    /// 1 (simple pop)
    pub fn exit_exception_scope(&mut self) {
        self.exception_scopes.pop();
    }
}

/// Trait for converting HIR elements to Rust tokens
///
/// This is the main trait for code generation. All HIR types that can
/// be converted to Rust code implement this trait.
pub trait RustCodeGen {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream>;
}

/// Extension trait for converting expressions to Rust syn::Expr
///
/// Used internally for expression-to-expression conversions where
/// we need syn::Expr specifically rather than TokenStream.
pub trait ToRustExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr>;
}
