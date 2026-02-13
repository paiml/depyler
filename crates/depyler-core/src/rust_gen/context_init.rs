//! Context initialization for code generation
//!
//! Extracts the ~265-line context initialization block from `generate_rust_file_internal`
//! in `rust_gen.rs` into composable functions:
//!
//! 1. [`analyze_module`] - Pre-analyzes HirModule to extract metadata (owned, no lifetimes)
//! 2. [`build_codegen_context`] - Constructs CodeGenContext from analysis + borrowed type_mapper
//! 3. [`populate_context_from_module`] - Populates function/class metadata into context
//!
//! # Integration
//!
//! In `generate_rust_file_internal`, replace lines 602-1013 with:
//!
//! ```rust,ignore
//! let analysis = context_init::analyze_module(module, type_mapper);
//! let type_mapper = analysis.resolve_type_mapper(type_mapper);
//! let type_mapper = &type_mapper;
//! let (mut ctx, unresolved_imports) =
//!     context_init::build_codegen_context(&analysis, type_mapper, initial_var_types);
//! context_init::populate_context_from_module(&mut ctx, module);
//! ```

use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::hir::{HirExpr, HirFunction, HirModule, HirStmt, Type};
use crate::string_optimization::StringOptimizer;
use std::collections::{HashMap, HashSet};

use super::argparse_transform;
use super::context::CodeGenContext;
use super::import_gen;
use super::validator_analysis;

// ============================================================================
// Phase 1: Module Analysis (pure data extraction, no lifetimes)
// ============================================================================

/// Pre-analyzed module metadata for context initialization
///
/// All fields are owned -- no lifetime constraints. This allows the caller
/// to create the owned type_mapper clone on their own stack before borrowing
/// it into `CodeGenContext<'a>`.
///
/// # Complexity
/// N/A (data structure)
pub(super) struct ModuleAnalysis {
    // Import processing results
    pub imported_modules: HashMap<String, crate::module_mapper::ModuleMapping>,
    pub imported_items: HashMap<String, String>,
    pub unresolved_imports: Vec<import_gen::UnresolvedImport>,
    pub module_aliases: HashMap<String, String>,
    pub all_imported_modules: HashSet<String>,

    // Dependency flags from imports
    pub needs_chrono: bool,
    pub needs_tempfile: bool,
    pub needs_itertools: bool,
    pub needs_statrs: bool,
    pub needs_url: bool,

    // Async detection
    pub has_async_code: bool,
    pub needs_tokio_from_async: bool,

    // Class metadata
    pub class_names: HashSet<String>,
    pub mutating_methods: HashMap<String, HashSet<String>>,
    pub property_methods: HashSet<String>,
    pub class_field_defaults: HashMap<String, Vec<Option<HirExpr>>>,
}

impl ModuleAnalysis {
    /// Resolve the type_mapper for async code
    ///
    /// If async code is detected and NASA mode is enabled, returns a cloned
    /// type_mapper with NASA mode disabled. Otherwise returns a plain clone.
    /// The caller should bind the result to a local variable and borrow it.
    ///
    /// # Complexity
    /// 2 (branch + clone)
    pub fn resolve_type_mapper(
        &self,
        type_mapper: &crate::type_mapper::TypeMapper,
    ) -> crate::type_mapper::TypeMapper {
        if self.has_async_code && type_mapper.nasa_mode {
            let mut async_mapper = type_mapper.clone();
            async_mapper.nasa_mode = false;
            async_mapper
        } else {
            type_mapper.clone()
        }
    }
}

/// Analyze a HirModule to extract all metadata needed for context initialization
///
/// This performs import processing, async detection, and class metadata extraction.
/// All results are owned (no lifetimes), allowing flexible composition with
/// the lifetime-constrained `CodeGenContext`.
///
/// # Arguments
/// * `module` - The HIR module to analyze
/// * `type_mapper` - Used only to read the module_mapper (not borrowed long-term)
///
/// # Complexity
/// 8 (import processing + async scan + class loops)
pub(super) fn analyze_module(module: &HirModule) -> ModuleAnalysis {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports (DEPYLER-0615, DEPYLER-1136)
    let (imported_modules, imported_items, unresolved_imports, module_aliases) =
        import_gen::process_module_imports(&module.imports, &module_mapper);

    // DEPYLER-1115: Collect ALL imported module names (including external unmapped ones)
    let all_imported_modules: HashSet<String> = module
        .imports
        .iter()
        .filter(|imp| imp.items.is_empty())
        .map(|imp| imp.module.clone())
        .collect();

    // DEPYLER-0490/0491: Set needs_* flags based on imported modules AND items
    let needs_chrono = imported_modules.contains_key("datetime")
        || imported_items
            .values()
            .any(|path| path.starts_with("chrono::"));
    let needs_tempfile = imported_modules.contains_key("tempfile")
        || imported_items
            .values()
            .any(|path| path.starts_with("tempfile::"));
    let needs_itertools = imported_modules.contains_key("itertools")
        || imported_items
            .values()
            .any(|path| path.starts_with("itertools::"));
    let needs_statrs = imported_modules.contains_key("statistics")
        || imported_items
            .values()
            .any(|path| path.starts_with("statrs::"));
    let needs_url = imported_modules.contains_key("urllib.parse")
        || imported_modules.contains_key("urllib")
        || imported_items
            .values()
            .any(|path| path.starts_with("url::"));

    // DEPYLER-NASA-ASYNC: Detect async usage
    let has_async_functions = module.functions.iter().any(|f| f.properties.is_async);
    let has_async_methods = module
        .classes
        .iter()
        .any(|c| c.methods.iter().any(|m| m.is_async));
    let has_asyncio_import = imported_modules.contains_key("asyncio");
    let has_async_code = has_async_functions || has_async_methods || has_asyncio_import;
    let needs_tokio_from_async = has_async_code;

    // Extract class metadata (DEPYLER-0230, DEPYLER-0231, DEPYLER-0737, DEPYLER-0932)
    let class_names = extract_class_names(module);
    let mutating_methods = extract_mutating_methods(module);
    let property_methods = extract_property_methods(module);
    let class_field_defaults = extract_class_field_defaults(module);

    ModuleAnalysis {
        imported_modules,
        imported_items,
        unresolved_imports,
        module_aliases,
        all_imported_modules,
        needs_chrono,
        needs_tempfile,
        needs_itertools,
        needs_statrs,
        needs_url,
        has_async_code,
        needs_tokio_from_async,
        class_names,
        mutating_methods,
        property_methods,
        class_field_defaults,
    }
}

// ============================================================================
// Phase 1 helpers: Class metadata extraction
// ============================================================================

/// Extract class names from module (DEPYLER-0230)
///
/// # Complexity
/// 2 (iter + map + collect)
fn extract_class_names(module: &HirModule) -> HashSet<String> {
    module
        .classes
        .iter()
        .map(|class| class.name.clone())
        .collect()
}

/// Build map of mutating methods per class (DEPYLER-0231)
///
/// # Complexity
/// 4 (nested loop + mutation check)
fn extract_mutating_methods(module: &HirModule) -> HashMap<String, HashSet<String>> {
    let mut result = HashMap::new();
    for class in &module.classes {
        let mut mut_methods = HashSet::new();
        for method in &class.methods {
            if crate::direct_rules::method_mutates_self(method) {
                mut_methods.insert(method.name.clone());
            }
        }
        result.insert(class.name.clone(), mut_methods);
    }
    result
}

/// Collect @property method names from all classes (DEPYLER-0737)
///
/// # Complexity
/// 3 (nested loop + filter)
fn extract_property_methods(module: &HirModule) -> HashSet<String> {
    let mut result = HashSet::new();
    for class in &module.classes {
        for method in &class.methods {
            if method.is_property {
                result.insert(method.name.clone());
            }
        }
    }
    result
}

/// Collect dataclass field defaults for constructor call sites (DEPYLER-0932)
///
/// # Complexity
/// 3 (loop + map + collect)
fn extract_class_field_defaults(
    module: &HirModule,
) -> HashMap<String, Vec<Option<HirExpr>>> {
    let mut result = HashMap::new();
    for class in &module.classes {
        let defaults: Vec<Option<HirExpr>> = class
            .fields
            .iter()
            .map(|f| f.default_value.clone())
            .collect();
        result.insert(class.name.clone(), defaults);
    }
    result
}

// ============================================================================
// Phase 2: Build CodeGenContext from analysis + borrowed type_mapper
// ============================================================================

/// Construct a CodeGenContext from pre-analyzed module data
///
/// The caller must own the `type_mapper` on their stack (possibly modified
/// by [`ModuleAnalysis::resolve_type_mapper`]) and pass a borrow here.
/// Returns both the context and the unresolved imports (needed later for
/// stub generation).
///
/// # Arguments
/// * `analysis` - Pre-analyzed module metadata from [`analyze_module`]
/// * `type_mapper` - Borrowed type mapper (caller owns the clone)
/// * `initial_var_types` - Pre-seeded var_types from Oracle (DEPYLER-1133)
///
/// # Complexity
/// 3 (struct init + conditional + alias check)
pub(super) fn build_codegen_context<'a>(
    analysis: ModuleAnalysis,
    type_mapper: &'a crate::type_mapper::TypeMapper,
    initial_var_types: HashMap<String, Type>,
) -> (CodeGenContext<'a>, Vec<import_gen::UnresolvedImport>) {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
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
        needs_chrono: analysis.needs_chrono,
        needs_tempfile: analysis.needs_tempfile,
        needs_itertools: analysis.needs_itertools,
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
        needs_lazy_lock: false,
        needs_depyler_value_enum: false,
        needs_python_string_ops: false,
        needs_python_int_ops: false,
        needs_depyler_date: false,
        needs_depyler_datetime: false,
        needs_depyler_timedelta: false,
        needs_depyler_regex_match: false,
        needs_trueno: false,
        numpy_vars: HashSet::new(),
        needs_glob: false,
        needs_statrs: analysis.needs_statrs,
        needs_url: analysis.needs_url,
        needs_tokio: analysis.needs_tokio_from_async,
        needs_completed_process: false,
        vararg_functions: HashSet::new(),
        slice_params: HashSet::new(),
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
        module_mapper,
        imported_modules: analysis.imported_modules,
        imported_items: analysis.imported_items,
        all_imported_modules: analysis.all_imported_modules,
        module_aliases: analysis.module_aliases,
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
        in_generator: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
        generator_iterator_state_vars: HashSet::new(),
        returns_impl_iterator: false,
        // DEPYLER-1133: Pre-seed var_types with Oracle-learned types
        var_types: initial_var_types,
        class_names: analysis.class_names,
        mutating_methods: analysis.mutating_methods,
        property_methods: analysis.property_methods,
        function_return_types: HashMap::new(),
        class_method_return_types: HashMap::new(),
        function_param_borrows: HashMap::new(),
        function_param_muts: HashMap::new(),
        function_param_defaults: HashMap::new(),
        class_field_defaults: analysis.class_field_defaults,
        function_param_optionals: HashMap::new(),
        class_field_types: HashMap::new(),
        tuple_iter_vars: HashSet::new(),
        iterator_vars: HashSet::new(),
        ref_params: HashSet::new(),
        mut_ref_params: HashSet::new(),
        is_final_statement: false,
        result_bool_functions: HashSet::new(),
        result_returning_functions: HashSet::new(),
        option_returning_functions: HashSet::new(),
        current_error_type: None,
        exception_scopes: Vec::new(),
        argparser_tracker: argparse_transform::ArgParserTracker::new(),
        generated_args_struct: None,
        generated_commands_enum: None,
        current_subcommand_fields: None,
        validator_functions: HashSet::new(),
        in_json_context: false,
        stdlib_mappings: crate::stdlib_mappings::StdlibMappings::new(),
        hoisted_inference_vars: HashSet::new(),
        none_placeholder_vars: HashSet::new(),
        cse_subcommand_temps: HashMap::new(),
        precomputed_option_fields: HashSet::new(),
        nested_function_params: HashMap::new(),
        fn_str_params: HashSet::new(),
        in_cmd_handler: false,
        cmd_handler_args_fields: Vec::new(),
        in_subcommand_match_arm: false,
        subcommand_match_fields: Vec::new(),
        hoisted_function_names: Vec::new(),
        is_main_function: false,
        boxed_dyn_write_vars: HashSet::new(),
        function_returns_boxed_write: false,
        option_unwrap_map: HashMap::new(),
        narrowed_option_vars: HashSet::new(),
        type_substitutions: HashMap::new(),
        current_assign_type: None,
        force_dict_value_option_wrap: false,
        char_iter_vars: HashSet::new(),
        char_counter_vars: HashSet::new(),
        adt_child_to_parent: HashMap::new(),
        function_param_types: HashMap::new(),
        mut_option_dict_params: HashSet::new(),
        mut_option_params: HashSet::new(),
        module_constant_types: HashMap::new(),
        #[cfg(feature = "sovereign-types")]
        type_query: super::load_type_database(),
        last_external_call_return_type: None,
        type_overrides: HashMap::new(),
        vars_used_later: HashSet::new(),
    };

    // DEPYLER-1137: Enable DepylerValue enum when module aliases are present
    if !ctx.module_aliases.is_empty() {
        ctx.needs_depyler_value_enum = true;
    }

    (ctx, analysis.unresolved_imports)
}

// ============================================================================
// Phase 3: Post-init population of function/class metadata
// ============================================================================
