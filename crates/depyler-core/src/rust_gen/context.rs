//! Code generation context and core traits
//!
//! This module provides the central CodeGenContext struct that maintains
//! state during Rust code generation, along with core traits used across
//! the code generation pipeline.

#[cfg(test)]
#[path = "context_tests.rs"]
mod tests;

use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
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
    pub needs_slice_random: bool, // GH-207: Track rand::seq::SliceRandom trait for choose/shuffle
    pub needs_rand_distr: bool,   // GH-207: Track rand_distr crate for distributions (gauss, triangular)
    pub needs_serde_json: bool,
    pub needs_regex: bool,
    pub needs_chrono: bool,
    pub needs_tempfile: bool, // DEPYLER-0490: Track tempfile crate for temporary file operations
    pub needs_itertools: bool, // DEPYLER-0490: Track itertools crate for advanced iteration
    pub needs_clap: bool,     // DEPYLER-0384: Track clap dependency for ArgumentParser
    pub needs_csv: bool,
    pub needs_rust_decimal: bool,
    pub needs_num_rational: bool,
    pub needs_base64: bool,
    pub needs_md5: bool,
    pub needs_sha1: bool,  // DEPYLER-1001: sha1 crate for hashlib.sha1()
    pub needs_sha2: bool,
    pub needs_sha3: bool,
    pub needs_digest: bool, // DEPYLER-0558: For Box<dyn DynDigest> type-erased hashers
    pub needs_blake2: bool,
    pub needs_hex: bool,
    pub needs_uuid: bool,
    pub needs_hmac: bool,
    pub needs_crc32: bool,
    pub needs_url_encoding: bool,
    pub needs_io_read: bool, // DEPYLER-0458: Track std::io::Read trait for file I/O
    pub needs_io_write: bool, // DEPYLER-0458: Track std::io::Write trait for file I/O
    pub needs_bufread: bool, // DEPYLER-0522: Track std::io::BufRead trait for .lines() method
    pub needs_once_cell: bool, // DEPYLER-REARCH-001: Track once_cell for lazy static initialization
    pub needs_lazy_lock: bool, // DEPYLER-1016: Track std::sync::LazyLock for NASA mode (std-only)
    pub needs_trueno: bool,    // Phase 3: NumPy→Trueno codegen (SIMD-accelerated tensor lib)
    pub numpy_vars: HashSet<String>, // DEPYLER-0932: Track variables assigned from numpy operations
    pub needs_tokio: bool,     // DEPYLER-0747: asyncio→tokio async runtime mapping
    pub needs_glob: bool,      // DEPYLER-0829: glob crate for Path.glob()/rglob()
    pub needs_statrs: bool,    // DEPYLER-1001: statrs crate for statistics module
    pub needs_url: bool,       // DEPYLER-1001: url crate for urllib.parse module
    pub declared_vars: Vec<HashSet<String>>,
    pub current_function_can_fail: bool,
    pub current_return_type: Option<Type>,
    pub module_mapper: crate::module_mapper::ModuleMapper,
    pub imported_modules: std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    pub imported_items: std::collections::HashMap<String, String>,
    pub mutable_vars: HashSet<String>,
    pub needs_zerodivisionerror: bool,
    pub needs_indexerror: bool,
    pub needs_valueerror: bool,
    pub needs_argumenttypeerror: bool,
    pub needs_runtimeerror: bool, // DEPYLER-0551: Python RuntimeError
    pub needs_filenotfounderror: bool, // DEPYLER-0551: Python FileNotFoundError
    pub needs_syntaxerror: bool,    // GH-204: Python SyntaxError
    pub needs_typeerror: bool,      // GH-204: Python TypeError
    pub needs_keyerror: bool,       // GH-204: Python KeyError
    pub needs_ioerror: bool,        // GH-204: Python IOError
    pub needs_attributeerror: bool, // GH-204: Python AttributeError
    pub needs_stopiteration: bool,  // GH-204: Python StopIteration
    pub is_classmethod: bool,
    pub in_generator: bool,
    pub generator_state_vars: HashSet<String>,
    /// DEPYLER-1082: Track generator state vars with Iterator/Generator type
    /// These need `while let Some(x) = self.g.next()` instead of `for x in self.g`
    /// because Box<dyn Iterator> doesn't implement IntoIterator
    pub generator_iterator_state_vars: HashSet<String>,
    /// DEPYLER-1076: Track when function returns impl Iterator/IntoIterator
    /// When true, closures in iterator chains that capture local variables need `move`
    pub returns_impl_iterator: bool,
    pub var_types: HashMap<String, Type>,
    pub class_names: HashSet<String>,
    pub mutating_methods: HashMap<String, HashSet<String>>,
    /// DEPYLER-0737: Track property method names across all classes
    /// In Python, @property allows method access without (), but Rust requires ()
    /// Used in convert_attribute to emit method call syntax for property access
    pub property_methods: HashSet<String>,
    /// DEPYLER-0269: Track function return types for Display trait selection
    /// Maps function name -> return type, populated during function generation
    /// Used to track types of variables assigned from function calls
    pub function_return_types: HashMap<String, Type>,
    /// DEPYLER-0270: Track function parameter borrowing for auto-borrow decisions
    /// Maps function name -> Vec of booleans (true if param is borrowed, false if owned)
    /// Used to determine whether to add & when passing List/Dict/Set arguments
    pub function_param_borrows: HashMap<String, Vec<bool>>,
    /// DEPYLER-0574: Track function parameters that need &mut (mutable borrow)
    /// Maps function name -> Vec of booleans (true if param is &mut, false if &)
    /// Used to determine whether to add &mut when passing arguments
    pub function_param_muts: HashMap<String, Vec<bool>>,
    /// DEPYLER-0621: Track function parameter default values
    /// Maps function name -> Vec of Option<HirExpr> (None if no default, Some if has default)
    /// Used to supply default arguments at call sites when fewer args are passed
    pub function_param_defaults: HashMap<String, Vec<Option<crate::hir::HirExpr>>>,
    /// DEPYLER-0932: Track dataclass field default values
    /// Maps class name -> Vec of Option<HirExpr> (None if no default, Some if has default)
    /// Used to supply default arguments at constructor call sites when fewer args are passed
    pub class_field_defaults: HashMap<String, Vec<Option<crate::hir::HirExpr>>>,
    /// DEPYLER-0720: Track class field types for self.field attribute access
    /// Maps field name -> Type, used to determine if self.X is float for int-to-float coercion
    pub class_field_types: HashMap<String, Type>,
    /// DEPYLER-1007: Track class method return types for return type inference
    /// Maps (ClassName, MethodName) -> Type, used to infer types from method calls
    /// Example: ("Point", "distance_squared") -> Type::Int
    pub class_method_return_types: HashMap<(String, String), Type>,
    /// DEPYLER-0307 Fix #9: Track variables that iterate over tuples (from zip())
    /// Used to generate tuple field access syntax (tuple.0, tuple.1) instead of vector indexing
    pub tuple_iter_vars: HashSet<String>,
    /// DEPYLER-0520: Track variables assigned from iterator-producing expressions
    /// Used to avoid adding .iter().cloned() to variables that are already iterators
    pub iterator_vars: HashSet<String>,
    /// DEPYLER-0758: Track parameters passed by reference in current function
    /// Used to dereference reference params in arithmetic operations (e.g., date subtraction)
    pub ref_params: HashSet<String>,
    /// DEPYLER-0271: Tracks if current statement is the final statement in its block
    /// Used to generate idiomatic expression-based returns (no `return` keyword)
    pub is_final_statement: bool,
    /// DEPYLER-0308: Track functions that return Result<bool, E>
    /// Used to auto-unwrap in boolean contexts (if/while conditions)
    pub result_bool_functions: HashSet<String>,
    /// DEPYLER-0270: Track ALL functions that return Result<T, E>
    /// Used to auto-unwrap at call sites in non-Result functions
    pub result_returning_functions: HashSet<String>,
    /// DEPYLER-0497: Track functions that return Option<T>
    /// Used to unwrap Options in format! macro and other contexts where Display is required
    pub option_returning_functions: HashSet<String>,
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

    /// DEPYLER-0447: Track argparse validator functions (type= parameter in add_argument)
    /// These functions should have &str parameter type regardless of type inference
    /// Populated when processing add_argument(type=validator_func) calls
    pub validator_functions: std::collections::HashSet<String>,

    /// DEPYLER-0461: Track whether we're currently generating code inside a serde_json::json!() macro
    /// When true, nested dicts must also use json!() instead of HashMap (json!() doesn't accept code blocks)
    pub in_json_context: bool,

    /// DEPYLER-0452: Stdlib API mapping system for Python→Rust API translations
    /// Maps Python stdlib patterns (module, class, attribute) to Rust code patterns
    pub stdlib_mappings: crate::stdlib_mappings::StdlibMappings,

    /// DEPYLER-0455 Bug 2: Track hoisted variables without type annotations
    /// These variables need String literal normalization (.to_string()) to ensure
    /// consistent type inference across if/else branches
    /// Example: let mut format; if x { format = "json"; } else { format = s.to_lowercase(); }
    /// Without normalization: &str vs String type mismatch
    /// With normalization: String vs String (consistent)
    pub hoisted_inference_vars: HashSet<String>,

    /// DEPYLER-0823: Track variables assigned None placeholder that need hoisting
    /// When `var = None; if cond: var = value; use(var)`, the None assignment is skipped
    /// but the variable must be hoisted before the if block to be accessible after it.
    /// Without hoisting: let var = value declared INSIDE if → E0425 outside
    /// With hoisting: let mut var; if cond { var = value; } → var accessible outside
    pub none_placeholder_vars: HashSet<String>,

    /// DEPYLER-0108: Track precomputed Option field checks for argparse
    /// Contains field names (e.g., "hash") where we've precomputed `let has_<field> = args.<field>.is_some();`
    /// Used by convert_method_call to substitute `args.<field>.is_some()` with `has_<field>`
    pub precomputed_option_fields: HashSet<String>,

    /// DEPYLER-0456 Bug #2: Track CSE temp variables for subcommand checks
    /// Maps CSE temp variable names (e.g., "_cse_temp_0") to command names (e.g., "init")
    /// This allows is_subcommand_check() to recognize CSE-optimized subcommand comparisons
    pub cse_subcommand_temps: std::collections::HashMap<String, String>,

    /// GH-70: Track inferred parameter types for nested functions/closures
    /// Maps nested function name → Vec<HirParam> with inferred types from usage patterns
    /// Used by stmt_gen.rs when generating closures to apply inferred parameter types
    /// instead of defaulting all parameters to () for Unknown types
    pub nested_function_params: std::collections::HashMap<String, Vec<crate::hir::HirParam>>,

    /// DEPYLER-0543: Track function parameters with `str` type annotation
    /// These become `&str` in Rust and should NOT have `&` added when used as dict keys
    /// Populated when generating function signatures
    pub fn_str_params: HashSet<String>,

    /// DEPYLER-0608: Track if currently generating a cmd_* handler function
    /// When true, expr_gen should transform args.X → X (field is now a direct parameter)
    pub in_cmd_handler: bool,

    /// DEPYLER-0608: Track which fields were extracted from args.X accesses
    /// Used in cmd_* handlers to know which field names are now parameters
    pub cmd_handler_args_fields: Vec<String>,

    /// DEPYLER-0608: Track if in a subcommand match arm that calls a handler
    /// When true and calling cmd_*/handle_* with args, pass extracted fields instead
    pub in_subcommand_match_arm: bool,

    /// DEPYLER-0625: Track variables that need Box<dyn Write> type
    /// When a variable is assigned File in one if-branch and Stdout in another,
    /// we need to use `Box<dyn std::io::Write>` as the type and wrap values with Box::new()
    pub boxed_dyn_write_vars: HashSet<String>,

    /// DEPYLER-0608: Track extracted subcommand fields for match arm context
    /// These fields are bound via `ref field` patterns in the match arm
    pub subcommand_match_fields: Vec<String>,

    /// DEPYLER-0613: Track names of functions that have been hoisted to top of body
    /// These should be skipped when processing nested blocks (if/while/for/try)
    /// to avoid duplicate emission
    pub hoisted_function_names: Vec<String>,

    /// DEPYLER-0617: Track if currently generating code inside main() function
    /// When true, integer returns should become process::exit() or Ok(())
    /// because Rust main can only return () or Result<(), E>
    pub is_main_function: bool,

    /// DEPYLER-0626: Track if current function returns Box<dyn Write>
    /// When a function returns both File and Stdout, we need trait object return
    /// Return statements should wrap values with Box::new()
    pub function_returns_boxed_write: bool,

    /// DEPYLER-0627: Track Option variables unwrapped via if-let pattern
    /// Maps original variable name to unwrapped name (e.g., "override" -> "override_val")
    /// Used inside if-let blocks to substitute variable references
    pub option_unwrap_map: HashMap<String, String>,

    /// DEPYLER-0627: Track if subprocess.run is used (needs CompletedProcess struct)
    /// When True, generate a CompletedProcess struct in the output
    pub needs_completed_process: bool,

    /// DEPYLER-0648: Track functions that have vararg parameters (*args in Python)
    /// These functions expect &[T] slice arguments, so call sites need to wrap args in &[...]
    pub vararg_functions: HashSet<String>,

    /// DEPYLER-0716: Type substitutions for current function's generic inference
    /// When a type parameter T can be inferred to a concrete type (e.g., String),
    /// this maps "T" -> String. Used to substitute in return types.
    pub type_substitutions: HashMap<String, Type>,

    /// DEPYLER-0727: Track assignment target type for dict literal Value wrapping
    /// When assigning to a variable with type annotation (e.g., `d: Dict[str, Any] = {...}`),
    /// this stores the annotation so dict codegen can wrap values in serde_json::json!()
    pub current_assign_type: Option<Type>,

    /// DEPYLER-0741: Force dict values to be wrapped in Some() when in list context
    /// When generating a list of dicts where ANY dict has None values, all dicts
    /// must use Option<V> for consistency. This flag is set by convert_list.
    pub force_dict_value_option_wrap: bool,

    /// DEPYLER-0737: Track which function parameters are Optional (from =None default)
    /// Maps function name → Vec<bool> where true means param at that index is Option<T>
    /// Used by call sites to wrap non-None values in Some()
    pub function_param_optionals: HashMap<String, Vec<bool>>,

    /// DEPYLER-0795: Track loop variables that iterate over string.chars()
    /// These variables are `char` type in Rust (not `str`), so ord(var) should be `var as u32`
    /// not `var.chars().next().unwrap() as i32`
    pub char_iter_vars: HashSet<String>,

    /// DEPYLER-0821: Track Counter variables created from strings (HashMap<char, i32>)
    /// Used to mark key variables as char when iterating with .items()
    pub char_counter_vars: HashSet<String>,

    /// DEPYLER-0936: Track ADT child→parent mappings for type rewriting
    /// When a Python ABC hierarchy is converted to a Rust enum (e.g., Iter with children
    /// ListIter, RangeIter), return types mentioning children must be rewritten to parent.
    /// Maps child name (e.g., "ListIter") → parent name (e.g., "Iter")
    pub adt_child_to_parent: HashMap<String, String>,

    /// DEPYLER-0950: Track function parameter types for literal coercion at call sites
    /// Maps function name → Vec<Type> for each parameter position
    /// Used to coerce integer literals to float literals when callee expects f64
    pub function_param_types: HashMap<String, Vec<Type>>,

    /// DEPYLER-0964: Track function parameters that are `&mut Option<Dict>` type
    /// When a parameter has type Dict[K,V] with default None, it becomes &mut Option<HashMap>
    /// Inside the function body, assignments and method calls need special handling:
    /// - Assignment: `memo = {}` → `*memo = Some(HashMap::new())`
    /// - Method calls: `memo.get(k)` → `memo.as_ref().unwrap().get(&k)`
    /// - Subscript: `memo[k] = v` → `memo.as_mut().unwrap().insert(k, v)`
    pub mut_option_dict_params: HashSet<String>, // DEPYLER-0964: Track &mut Option<Dict> params
    pub needs_depyler_value_enum: bool,          // DEPYLER-FIX-RC2: Track need for DepylerValue enum
    pub needs_depyler_date: bool,                // DEPYLER-1066: Track need for DepylerDate struct
    pub needs_depyler_datetime: bool,            // DEPYLER-1067: Track need for DepylerDateTime struct
    pub needs_depyler_timedelta: bool,           // DEPYLER-1068: Track need for DepylerTimeDelta struct
    pub needs_depyler_regex_match: bool,         // DEPYLER-1070: Track need for DepylerRegexMatch struct
    /// DEPYLER-1060: Track module-level constant types (dict, list, set)
    /// These persist across function boundaries and aren't cleared when processing functions.
    /// Used by is_dict_expr() to recognize module-level statics accessed from within functions.
    pub module_constant_types: HashMap<String, Type>,

    /// DEPYLER-1112: Sovereign Type Database for external library type resolution
    /// When enabled, provides O(1) lookup of external library function return types
    /// to reduce E0308 type mismatch errors.
    #[cfg(feature = "sovereign-types")]
    pub type_query: Option<std::sync::Arc<std::sync::Mutex<depyler_knowledge::TypeQuery>>>,

    /// DEPYLER-1113: Last external call return type for assignment propagation
    /// When a MethodCall on an external module is encountered (e.g., requests.get),
    /// stores the return type string for use when processing the assignment.
    /// Reset to None after the assignment is processed.
    pub last_external_call_return_type: Option<String>,
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

    /// Returns the current exception scope nesting depth
    ///
    /// DEPYLER-0931: Used to detect nested try/except blocks for proper return wrapping.
    /// When inside nested try/except, returns from inner match arms must be wrapped
    /// in Ok() to match the outer closure's Result<T, E> return type.
    ///
    /// # Complexity
    /// 1 (len() call)
    pub fn exception_nesting_depth(&self) -> usize {
        self.exception_scopes.len()
    }

    /// DEPYLER-1022: Return the fallback type annotation based on NASA mode
    ///
    /// In NASA mode (default), returns `: String` to avoid external crate dependencies.
    /// In non-NASA mode, returns `: serde_json::Value` and sets needs_serde_json flag.
    ///
    /// # Complexity
    /// 2 (if + quote)
    pub fn fallback_type_annotation(&mut self) -> proc_macro2::TokenStream {
        if self.type_mapper.nasa_mode {
            quote::quote! { : String }
        } else {
            self.needs_serde_json = true;
            quote::quote! { : serde_json::Value }
        }
    }

    /// DEPYLER-1022: Return the fallback type (without colon) based on NASA mode
    ///
    /// In NASA mode, returns `String`. In non-NASA mode, returns `serde_json::Value`.
    ///
    /// # Complexity
    /// 2 (if + quote)
    pub fn fallback_type(&mut self) -> proc_macro2::TokenStream {
        if self.type_mapper.nasa_mode {
            quote::quote! { String }
        } else {
            self.needs_serde_json = true;
            quote::quote! { serde_json::Value }
        }
    }

    /// DEPYLER-1112: Look up the return type for an external library function
    ///
    /// Uses the Sovereign Type Database (when available) to resolve return types
    /// for external library calls like `requests.get()` -> `Response`.
    ///
    /// Returns None if:
    /// - The `sovereign-types` feature is not enabled
    /// - No TypeQuery is configured
    /// - The symbol is not found in the database
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(ret_type) = ctx.lookup_external_return_type("requests", "get") {
    ///     // ret_type = "Response"
    /// }
    /// ```
    ///
    /// # Complexity
    /// 3 (feature check + option check + mutex lock + query)
    #[cfg(feature = "sovereign-types")]
    pub fn lookup_external_return_type(&self, module: &str, symbol: &str) -> Option<String> {
        self.type_query.as_ref().and_then(|tq| {
            tq.lock()
                .ok()
                .and_then(|mut query| query.find_return_type(module, symbol).ok())
        })
    }

    /// DEPYLER-1112: Stub for when sovereign-types feature is disabled
    ///
    /// Always returns None since the type database is not available.
    #[cfg(not(feature = "sovereign-types"))]
    pub fn lookup_external_return_type(&self, _module: &str, _symbol: &str) -> Option<String> {
        None
    }

    /// DEPYLER-1112: Check if a symbol exists in the external type database
    ///
    /// Returns true if the symbol is known (feature enabled and found in DB).
    ///
    /// # Complexity
    /// 3 (feature check + option check + mutex lock + query)
    #[cfg(feature = "sovereign-types")]
    pub fn has_external_symbol(&self, module: &str, symbol: &str) -> bool {
        self.type_query.as_ref().map_or(false, |tq| {
            tq.lock()
                .ok()
                .map_or(false, |mut query| query.has_symbol(module, symbol))
        })
    }

    /// DEPYLER-1112: Stub for when sovereign-types feature is disabled
    #[cfg(not(feature = "sovereign-types"))]
    pub fn has_external_symbol(&self, _module: &str, _symbol: &str) -> bool {
        false
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

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use once_cell::sync::Lazy;

    /// Static TypeMapper for test contexts
    static TEST_TYPE_MAPPER: Lazy<crate::type_mapper::TypeMapper> =
        Lazy::new(crate::type_mapper::TypeMapper::new);

    /// Create a test CodeGenContext with default values
    ///
    /// This is used by unit tests that need a CodeGenContext but don't
    /// need custom type mapper configuration.
    pub fn test_context() -> CodeGenContext<'static> {
        CodeGenContext {
            type_mapper: &*TEST_TYPE_MAPPER,
            annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(
                TEST_TYPE_MAPPER.clone(),
            ),
            string_optimizer: StringOptimizer::new(),
            union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
            generated_enums: Vec::new(),
            needs_hashmap: false,
            needs_hashset: false,
            needs_vecdeque: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            needs_rand: false,
            needs_slice_random: false,
            needs_rand_distr: false,
            needs_serde_json: false,
            needs_regex: false,
            needs_chrono: false,
            needs_tempfile: false,
            needs_itertools: false,
            needs_clap: false,
            needs_csv: false,
            needs_rust_decimal: false,
            needs_num_rational: false,
            needs_base64: false,
            needs_md5: false,
            needs_sha1: false,
            needs_sha2: false,
            needs_sha3: false,
            needs_digest: false,
            needs_blake2: false,
            needs_hex: false,
            needs_uuid: false,
            needs_hmac: false,
            needs_crc32: false,
            needs_url_encoding: false,
            needs_io_read: false,
            needs_io_write: false,
            needs_bufread: false,
            needs_once_cell: false,
            needs_lazy_lock: false, // DEPYLER-1016
            needs_trueno: false,
            numpy_vars: HashSet::new(),
            needs_tokio: false,
            needs_glob: false,
            needs_statrs: false,
            needs_url: false,
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: HashMap::new(),
            imported_items: HashMap::new(),
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            needs_valueerror: false,
            needs_argumenttypeerror: false,
            needs_runtimeerror: false,
            needs_filenotfounderror: false,
            needs_syntaxerror: false,
            needs_typeerror: false,
            needs_keyerror: false,
            needs_ioerror: false,
            needs_attributeerror: false,
            needs_stopiteration: false,
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
            generator_iterator_state_vars: HashSet::new(),
            returns_impl_iterator: false,
            var_types: HashMap::new(),
            class_names: HashSet::new(),
            mutating_methods: HashMap::new(),
            property_methods: HashSet::new(),
            function_return_types: HashMap::new(),
            function_param_borrows: HashMap::new(),
            function_param_muts: HashMap::new(),
            function_param_defaults: HashMap::new(),
            class_field_defaults: HashMap::new(),
            class_field_types: HashMap::new(),
            class_method_return_types: HashMap::new(),
            tuple_iter_vars: HashSet::new(),
            iterator_vars: HashSet::new(),
            ref_params: HashSet::new(),
            is_final_statement: false,
            result_bool_functions: HashSet::new(),
            result_returning_functions: HashSet::new(),
            option_returning_functions: HashSet::new(),
            current_error_type: None,
            exception_scopes: Vec::new(),
            argparser_tracker: crate::rust_gen::argparse_transform::ArgParserTracker::new(),
            generated_args_struct: None,
            generated_commands_enum: None,
            current_subcommand_fields: None,
            validator_functions: HashSet::new(),
            in_json_context: false,
            stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(),
            hoisted_inference_vars: HashSet::new(),
            none_placeholder_vars: HashSet::new(),
            precomputed_option_fields: HashSet::new(),
            cse_subcommand_temps: HashMap::new(),
            nested_function_params: HashMap::new(),
            fn_str_params: HashSet::new(),
            in_cmd_handler: false,
            cmd_handler_args_fields: Vec::new(),
            in_subcommand_match_arm: false,
            boxed_dyn_write_vars: HashSet::new(),
            subcommand_match_fields: Vec::new(),
            hoisted_function_names: Vec::new(),
            is_main_function: false,
            function_returns_boxed_write: false,
            option_unwrap_map: HashMap::new(),
            needs_completed_process: false,
            vararg_functions: HashSet::new(),
            type_substitutions: HashMap::new(),
            current_assign_type: None,
            force_dict_value_option_wrap: false,
            function_param_optionals: HashMap::new(),
            char_iter_vars: HashSet::new(),
            char_counter_vars: HashSet::new(),
            adt_child_to_parent: HashMap::new(),
            function_param_types: HashMap::new(),
            mut_option_dict_params: HashSet::new(),
            needs_depyler_value_enum: false,
            needs_depyler_date: false,      // DEPYLER-1066
            needs_depyler_datetime: false,  // DEPYLER-1067
            needs_depyler_timedelta: false, // DEPYLER-1068
            needs_depyler_regex_match: false, // DEPYLER-1070
            module_constant_types: HashMap::new(), // DEPYLER-1060
            #[cfg(feature = "sovereign-types")]
            type_query: None, // DEPYLER-1112
            last_external_call_return_type: None, // DEPYLER-1113
        }
    }
}

/// Default implementation for CodeGenContext (test-only)
///
/// Uses a static TypeMapper to allow Default trait implementation.
/// This is primarily useful for unit testing.
#[cfg(test)]
impl Default for CodeGenContext<'static> {
    fn default() -> Self {
        test_helpers::test_context()
    }
}
