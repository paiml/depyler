use crate::annotation_aware_type_mapper::AnnotationAwareTypeMapper;
use crate::generator_state::GeneratorStateInfo;
use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{self, parse_quote};

/// Context for code generation including type mapping and configuration
pub struct CodeGenContext<'a> {
    pub type_mapper: &'a crate::type_mapper::TypeMapper,
    pub annotation_aware_mapper: AnnotationAwareTypeMapper,
    pub string_optimizer: StringOptimizer,
    pub union_enum_generator: crate::union_enum_gen::UnionEnumGenerator,
    pub generated_enums: Vec<proc_macro2::TokenStream>,
    pub needs_hashmap: bool,
    pub needs_hashset: bool,
    pub needs_fnv_hashmap: bool,
    pub needs_ahash_hashmap: bool,
    pub needs_arc: bool,
    pub needs_rc: bool,
    pub needs_cow: bool,
    pub declared_vars: Vec<HashSet<String>>,
    pub current_function_can_fail: bool,
    pub current_return_type: Option<Type>,
    pub module_mapper: crate::module_mapper::ModuleMapper,
    pub imported_modules: std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    pub imported_items: std::collections::HashMap<String, String>,
    pub mutable_vars: HashSet<String>,
    pub needs_zerodivisionerror: bool,
    pub needs_indexerror: bool,
    pub is_classmethod: bool,
    pub in_generator: bool,
    pub generator_state_vars: HashSet<String>,
}

impl<'a> CodeGenContext<'a> {
    fn enter_scope(&mut self) {
        self.declared_vars.push(HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.declared_vars.pop();
    }

    fn is_declared(&self, var_name: &str) -> bool {
        self.declared_vars
            .iter()
            .any(|scope| scope.contains(var_name))
    }

    fn declare_var(&mut self, var_name: &str) {
        if let Some(current_scope) = self.declared_vars.last_mut() {
            current_scope.insert(var_name.to_string());
        }
    }

    /// Process a Union type and generate an enum if needed
    pub fn process_union_type(&mut self, types: &[crate::hir::Type]) -> String {
        let (enum_name, enum_def) = self.union_enum_generator.generate_union_enum(types);
        if !enum_def.is_empty() {
            self.generated_enums.push(enum_def);
        }
        enum_name
    }
}

/// Trait for converting HIR elements to Rust tokens
pub trait RustCodeGen {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream>;
}

/// Process a whole module import (e.g., `import math`)
///
/// Adds the module mapping to imported_modules if found.
/// Complexity: 2 (single if let)
fn process_whole_module_import(
    import: &Import,
    module_mapper: &crate::module_mapper::ModuleMapper,
    imported_modules: &mut std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        imported_modules.insert(import.module.clone(), mapping.clone());
    }
}

/// Process a single import item and add to imported_items
///
/// Handles special case for typing module (no full path).
/// Complexity: 4 (if let + 2 if checks for typing/empty path)
fn process_import_item(
    import_module: &str,
    item_name: &str,
    import_key: &str, // Either name or alias
    mapping: &crate::module_mapper::ModuleMapping,
    imported_items: &mut std::collections::HashMap<String, String>,
) {
    if let Some(rust_name) = mapping.item_map.get(item_name) {
        // Special handling for typing module
        if import_module == "typing" && !rust_name.is_empty() {
            // Types from typing module don't need full paths
            imported_items.insert(import_key.to_string(), rust_name.clone());
        } else if !mapping.rust_path.is_empty() {
            imported_items.insert(
                import_key.to_string(),
                format!("{}::{}", mapping.rust_path, rust_name),
            );
        }
    }
}

/// Process specific items import (e.g., `from typing import List, Dict`)
///
/// Handles both Named and Aliased import items.
/// Complexity: 5 (if let + loop + match with 2 arms)
fn process_specific_items_import(
    import: &Import,
    module_mapper: &crate::module_mapper::ModuleMapper,
    imported_items: &mut std::collections::HashMap<String, String>,
) {
    if let Some(mapping) = module_mapper.get_mapping(&import.module) {
        for item in &import.items {
            match item {
                ImportItem::Named(name) => {
                    process_import_item(&import.module, name, name, mapping, imported_items);
                }
                ImportItem::Aliased { name, alias } => {
                    process_import_item(&import.module, name, alias, mapping, imported_items);
                }
            }
        }
    }
}

/// Process module imports and populate import mappings
///
/// Extracts import processing logic from generate_rust_file.
/// Complexity: 3 (loop + if/else) - Down from 15!
fn process_module_imports(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
) -> (
    std::collections::HashMap<String, crate::module_mapper::ModuleMapping>,
    std::collections::HashMap<String, String>,
) {
    let mut imported_modules = std::collections::HashMap::new();
    let mut imported_items = std::collections::HashMap::new();

    for import in imports {
        if import.items.is_empty() {
            process_whole_module_import(import, module_mapper, &mut imported_modules);
        } else {
            process_specific_items_import(import, module_mapper, &mut imported_items);
        }
    }

    (imported_modules, imported_items)
}

/// Analyze functions for string optimization
///
/// Performs string optimization analysis on all functions.
/// Complexity: 2 (well within ≤10 target)
fn analyze_string_optimization(ctx: &mut CodeGenContext, functions: &[HirFunction]) {
    for func in functions {
        ctx.string_optimizer.analyze_function(func);
    }
}

/// Analyze which variables are reassigned (mutated) in a list of statements
///
/// Populates ctx.mutable_vars with variables that are reassigned after declaration.
/// Complexity: 4 (loop + match + if)
fn analyze_mutable_vars(stmts: &[HirStmt], ctx: &mut CodeGenContext) {
    let mut declared = HashSet::new();

    fn analyze_stmt(stmt: &HirStmt, declared: &mut HashSet<String>, mutable: &mut HashSet<String>) {
        match stmt {
            HirStmt::Assign { target, .. } => {
                match target {
                    AssignTarget::Symbol(name) => {
                        if declared.contains(name) {
                            // Variable is being reassigned - mark as mutable
                            mutable.insert(name.clone());
                        } else {
                            // First declaration
                            declared.insert(name.clone());
                        }
                    }
                    AssignTarget::Tuple(targets) => {
                        // Tuple assignment - analyze each element
                        for t in targets {
                            if let AssignTarget::Symbol(name) = t {
                                if declared.contains(name) {
                                    // Variable is being reassigned - mark as mutable
                                    mutable.insert(name.clone());
                                } else {
                                    // First declaration
                                    declared.insert(name.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            HirStmt::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    analyze_stmt(stmt, declared, mutable);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        analyze_stmt(stmt, declared, mutable);
                    }
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                for stmt in body {
                    analyze_stmt(stmt, declared, mutable);
                }
            }
            _ => {}
        }
    }

    for stmt in stmts {
        analyze_stmt(stmt, &mut declared, &mut ctx.mutable_vars);
    }
}

/// Convert Python classes to Rust structs
///
/// Processes all classes and generates token streams.
/// Complexity: 3 (well within ≤10 target)
fn convert_classes_to_rust(
    classes: &[HirClass],
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut class_items = Vec::new();
    for class in classes {
        let items = crate::direct_rules::convert_class_to_struct(class, type_mapper)?;
        for item in items {
            let tokens = item.to_token_stream();
            class_items.push(tokens);
        }
    }
    Ok(class_items)
}

/// Convert HIR functions to Rust token streams
///
/// Processes all functions using the code generation context.
/// Complexity: 2 (well within ≤10 target)
fn convert_functions_to_rust(
    functions: &[HirFunction],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    functions
        .iter()
        .map(|f| f.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()
}

/// Generate conditional imports based on code generation context
///
/// Adds imports for collections and smart pointers as needed.
/// Complexity: 1 (data-driven approach, well within ≤10 target)
fn generate_conditional_imports(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut imports = Vec::new();

    // Define all possible conditional imports
    let conditional_imports = [
        (ctx.needs_hashmap, quote! { use std::collections::HashMap; }),
        (ctx.needs_hashset, quote! { use std::collections::HashSet; }),
        (ctx.needs_fnv_hashmap, quote! { use fnv::FnvHashMap; }),
        (ctx.needs_ahash_hashmap, quote! { use ahash::AHashMap; }),
        (ctx.needs_arc, quote! { use std::sync::Arc; }),
        (ctx.needs_rc, quote! { use std::rc::Rc; }),
        (ctx.needs_cow, quote! { use std::borrow::Cow; }),
    ];

    // Add imports where needed
    for (needed, import_tokens) in conditional_imports {
        if needed {
            imports.push(import_tokens);
        }
    }

    imports
}

/// Generate error type definitions if needed
///
/// Generates struct definitions for Python error types like ZeroDivisionError and IndexError.
/// Complexity: 2 (simple conditionals)
fn generate_error_type_definitions(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let mut definitions = Vec::new();

    if ctx.needs_zerodivisionerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct ZeroDivisionError {
                message: String,
            }

            impl std::fmt::Display for ZeroDivisionError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "division by zero: {}", self.message)
                }
            }

            impl std::error::Error for ZeroDivisionError {}

            impl ZeroDivisionError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    if ctx.needs_indexerror {
        definitions.push(quote! {
            #[derive(Debug, Clone)]
            pub struct IndexError {
                message: String,
            }

            impl std::fmt::Display for IndexError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "index out of range: {}", self.message)
                }
            }

            impl std::error::Error for IndexError {}

            impl IndexError {
                pub fn new(message: impl Into<String>) -> Self {
                    Self { message: message.into() }
                }
            }
        });
    }

    definitions
}

/// Generate import token streams from Python imports
///
/// Maps Python imports to Rust use statements.
/// Complexity: ~7-8 (within ≤10 target)
fn generate_import_tokens(
    imports: &[Import],
    module_mapper: &crate::module_mapper::ModuleMapper,
) -> Vec<proc_macro2::TokenStream> {
    let mut items = Vec::new();
    let mut external_imports = Vec::new();
    let mut std_imports = Vec::new();

    // Categorize imports
    for import in imports {
        let rust_imports = module_mapper.map_import(import);
        for rust_import in rust_imports {
            if rust_import.path.starts_with("//") {
                // Comment for unmapped imports
                let comment = &rust_import.path;
                items.push(quote! { #[doc = #comment] });
            } else if rust_import.is_external {
                external_imports.push(rust_import);
            } else {
                std_imports.push(rust_import);
            }
        }
    }

    // Add external imports
    for import in external_imports {
        let path: syn::Path =
            syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { unknown });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    // Add standard library imports
    for import in std_imports {
        // Skip typing imports as they're handled by the type system
        if import.path.starts_with("::") || import.path.is_empty() {
            continue;
        }
        let path: syn::Path = syn::parse_str(&import.path).unwrap_or_else(|_| parse_quote! { std });
        if let Some(alias) = import.alias {
            let alias_ident = syn::Ident::new(&alias, proc_macro2::Span::call_site());
            items.push(quote! { use #path as #alias_ident; });
        } else {
            items.push(quote! { use #path; });
        }
    }

    items
}

/// Generate interned string constant tokens
///
/// Generates constant definitions for interned strings.
/// Complexity: 2 (well within ≤10 target)
fn generate_interned_string_tokens(optimizer: &StringOptimizer) -> Vec<proc_macro2::TokenStream> {
    let interned_constants = optimizer.generate_interned_constants();
    interned_constants
        .into_iter()
        .filter_map(|constant| constant.parse().ok())
        .collect()
}

/// Generate a complete Rust file from HIR module
pub fn generate_rust_file(
    module: &HirModule,
    type_mapper: &crate::type_mapper::TypeMapper,
) -> Result<String> {
    let module_mapper = crate::module_mapper::ModuleMapper::new();

    // Process imports to populate the context
    let (imported_modules, imported_items) =
        process_module_imports(&module.imports, &module_mapper);

    let mut ctx = CodeGenContext {
        type_mapper,
        annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(type_mapper.clone()),
        string_optimizer: StringOptimizer::new(),
        union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
        generated_enums: Vec::new(),
        needs_hashmap: false,
        needs_hashset: false,
        needs_fnv_hashmap: false,
        needs_ahash_hashmap: false,
        needs_arc: false,
        needs_rc: false,
        needs_cow: false,
        declared_vars: vec![HashSet::new()],
        current_function_can_fail: false,
        current_return_type: None,
        module_mapper,
        imported_modules,
        imported_items,
        mutable_vars: HashSet::new(),
        needs_zerodivisionerror: false,
        in_generator: false,
        needs_indexerror: false,
        is_classmethod: false,
        generator_state_vars: HashSet::new(),
    };

    // Analyze all functions first for string optimization
    analyze_string_optimization(&mut ctx, &module.functions);

    // Convert classes first (they might be used by functions)
    let classes = convert_classes_to_rust(&module.classes, ctx.type_mapper)?;

    // Convert all functions to detect what imports we need
    let functions = convert_functions_to_rust(&module.functions, &mut ctx)?;

    // Build items list with all generated code
    let mut items = Vec::new();

    // Add module imports (create new mapper for token generation)
    let import_mapper = crate::module_mapper::ModuleMapper::new();
    items.extend(generate_import_tokens(&module.imports, &import_mapper));

    // Add interned string constants
    items.extend(generate_interned_string_tokens(&ctx.string_optimizer));

    // Add collection imports if needed
    items.extend(generate_conditional_imports(&ctx));

    // Add error type definitions if needed
    items.extend(generate_error_type_definitions(&ctx));

    // Add generated union enums
    items.extend(ctx.generated_enums.clone());

    // Add classes
    items.extend(classes);

    // Add all functions
    items.extend(functions);

    // Generate tests for functions if applicable
    let test_gen = crate::test_generation::TestGenerator::new(Default::default());
    let mut test_modules = Vec::new();

    for func in &module.functions {
        if let Some(test_module) = test_gen.generate_tests(func)? {
            test_modules.push(test_module);
        }
    }

    // Add test modules
    items.extend(test_modules);

    let file = quote! {
        #(#items)*
    };

    Ok(format_rust_code(file.to_string()))
}

/// Generate struct fields for generator state variables
fn generate_state_fields(
    state_info: &crate::generator_state::GeneratorStateInfo,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    state_info
        .state_variables
        .iter()
        .map(|var| {
            let field_name = syn::Ident::new(&var.name, proc_macro2::Span::call_site());
            let rust_type = ctx.type_mapper.map_type(&var.ty);
            let field_type = rust_type_to_syn(&rust_type)?;
            Ok(quote! { #field_name: #field_type })
        })
        .collect()
}

/// Generate struct fields for captured parameters
fn generate_param_fields(
    func: &HirFunction,
    state_info: &crate::generator_state::GeneratorStateInfo,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    func.params
        .iter()
        .filter(|p| state_info.captured_params.contains(&p.name))
        .map(|param| {
            let field_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            let rust_type = ctx.type_mapper.map_type(&param.ty);
            let field_type = rust_type_to_syn(&rust_type)?;
            Ok(quote! { #field_name: #field_type })
        })
        .collect()
}

/// Extract the Item type for the Iterator from generator return type
fn extract_generator_item_type(
    rust_ret_type: &crate::type_mapper::RustType,
) -> Result<syn::Type> {
    // For now, use the return type directly as Item type
    // TODO: Handle more complex cases (Generator<Yield=T>)
    rust_type_to_syn(rust_ret_type)
}

/// Generate field initializers for state variables (with default values)
fn generate_state_initializers(
    state_info: &crate::generator_state::GeneratorStateInfo,
) -> Vec<proc_macro2::TokenStream> {
    state_info
        .state_variables
        .iter()
        .map(|var| {
            let field_name = syn::Ident::new(&var.name, proc_macro2::Span::call_site());
            // Initialize with type-appropriate default (0 for int, false for bool, etc.)
            let default_value = get_default_value_for_type(&var.ty);
            quote! { #field_name: #default_value }
        })
        .collect()
}

/// Generate field initializers for captured parameters
fn generate_param_initializers(
    func: &HirFunction,
    state_info: &crate::generator_state::GeneratorStateInfo,
) -> Vec<proc_macro2::TokenStream> {
    func.params
        .iter()
        .filter(|p| state_info.captured_params.contains(&p.name))
        .map(|param| {
            let field_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            // Initialize with parameter value (n: n)
            quote! { #field_name: #field_name }
        })
        .collect()
}

/// Get default value expression for a type
fn get_default_value_for_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => quote! { 0 },
        Type::Float => quote! { 0.0 },
        Type::Bool => quote! { false },
        Type::String => quote! { String::new() },
        _ => quote! { Default::default() },
    }
}

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();
        let type_params = generic_registry.infer_function_generics(self)?;

        // Perform lifetime analysis
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference.analyze_function(self, ctx.type_mapper);

        // Generate combined generic parameters (lifetimes + type params)
        let generic_params = if type_params.is_empty() && lifetime_result.lifetime_params.is_empty()
        {
            quote! {}
        } else {
            let mut all_params = Vec::new();

            // Add lifetime parameters first
            for lt in &lifetime_result.lifetime_params {
                let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
                all_params.push(quote! { #lt_ident });
            }

            // Add type parameters with their bounds
            for type_param in &type_params {
                let param_name = syn::Ident::new(&type_param.name, proc_macro2::Span::call_site());
                if type_param.bounds.is_empty() {
                    all_params.push(quote! { #param_name });
                } else {
                    let bounds: Vec<_> = type_param
                        .bounds
                        .iter()
                        .map(|b| {
                            let bound: syn::Path =
                                syn::parse_str(b).unwrap_or_else(|_| parse_quote! { Clone });
                            quote! { #bound }
                        })
                        .collect();
                    all_params.push(quote! { #param_name: #(#bounds)+* });
                }
            }

            quote! { <#(#all_params),*> }
        };

        // Generate lifetime bounds
        let where_clause = if lifetime_result.lifetime_bounds.is_empty() {
            quote! {}
        } else {
            let bounds: Vec<_> = lifetime_result
                .lifetime_bounds
                .iter()
                .map(|(from, to)| {
                    let from_lt = syn::Lifetime::new(from, proc_macro2::Span::call_site());
                    let to_lt = syn::Lifetime::new(to, proc_macro2::Span::call_site());
                    quote! { #from_lt: #to_lt }
                })
                .collect();
            quote! { where #(#bounds),* }
        };

        // Convert parameters using lifetime analysis results
        let params: Vec<_> = self
            .params
            .iter()
            .map(|param| {
                let param_ident = syn::Ident::new(&param.name, proc_macro2::Span::call_site());

                // Check if parameter is mutated
                let is_param_mutated = matches!(
                    lifetime_result.borrowing_strategies.get(&param.name),
                    Some(crate::borrowing_context::BorrowingStrategy::TakeOwnership)
                ) && self.body.iter().any(
                    |stmt| matches!(stmt, HirStmt::Assign { target: AssignTarget::Symbol(s), .. } if s == &param.name),
                );

                // Get the inferred parameter info
                if let Some(inferred) = lifetime_result.param_lifetimes.get(&param.name) {
                    let rust_type = &inferred.rust_type;

                    // Check if this is a placeholder Union enum that needs proper generation
                    let actual_rust_type =
                        if let crate::type_mapper::RustType::Enum { name, variants: _ } = rust_type
                        {
                            if name == "UnionType" {
                                // Generate a proper enum name and definition from the original Union type
                                if let Type::Union(types) = &param.ty {
                                    let enum_name = ctx.process_union_type(types);
                                    crate::type_mapper::RustType::Custom(enum_name)
                                } else {
                                    rust_type.clone()
                                }
                            } else {
                                rust_type.clone()
                            }
                        } else {
                            rust_type.clone()
                        };

                    update_import_needs(ctx, &actual_rust_type);
                    let mut ty = rust_type_to_syn(&actual_rust_type)?;

                    // Check if we're dealing with a string that should use Cow
                    if let Some(strategy) = lifetime_result.borrowing_strategies.get(&param.name) {
                        match strategy {
                            crate::borrowing_context::BorrowingStrategy::UseCow { lifetime } => {
                                ctx.needs_cow = true;
                                let lt =
                                    syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
                                ty = parse_quote! { Cow<#lt, str> };
                            }
                            _ => {
                                // Apply normal borrowing if needed
                                if inferred.should_borrow {
                                    // Special case for strings: use &str instead of &String
                                    if matches!(rust_type, crate::type_mapper::RustType::String) {
                                        if let Some(ref lifetime) = inferred.lifetime {
                                            let lt = syn::Lifetime::new(
                                                lifetime,
                                                proc_macro2::Span::call_site(),
                                            );
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &#lt mut str }
                                            } else {
                                                parse_quote! { &#lt str }
                                            };
                                        } else {
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &mut str }
                                            } else {
                                                parse_quote! { &str }
                                            };
                                        }
                                    } else {
                                        // Non-string types
                                        if let Some(ref lifetime) = inferred.lifetime {
                                            let lt = syn::Lifetime::new(
                                                lifetime,
                                                proc_macro2::Span::call_site(),
                                            );
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &#lt mut #ty }
                                            } else {
                                                parse_quote! { &#lt #ty }
                                            };
                                        } else {
                                            ty = if inferred.needs_mut {
                                                parse_quote! { &mut #ty }
                                            } else {
                                                parse_quote! { &#ty }
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Fallback to normal borrowing
                        if inferred.should_borrow {
                            // Special case for strings: use &str instead of &String
                            if matches!(rust_type, crate::type_mapper::RustType::String) {
                                if let Some(ref lifetime) = inferred.lifetime {
                                    let lt = syn::Lifetime::new(
                                        lifetime,
                                        proc_macro2::Span::call_site(),
                                    );
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &#lt mut str }
                                    } else {
                                        parse_quote! { &#lt str }
                                    };
                                } else {
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &mut str }
                                    } else {
                                        parse_quote! { &str }
                                    };
                                }
                            } else {
                                // Non-string types
                                if let Some(ref lifetime) = inferred.lifetime {
                                    let lt = syn::Lifetime::new(
                                        lifetime,
                                        proc_macro2::Span::call_site(),
                                    );
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &#lt mut #ty }
                                    } else {
                                        parse_quote! { &#lt #ty }
                                    };
                                } else {
                                    ty = if inferred.needs_mut {
                                        parse_quote! { &mut #ty }
                                    } else {
                                        parse_quote! { &#ty }
                                    };
                                }
                            }
                        }
                    }

                    if is_param_mutated {
                        Ok(quote! { mut #param_ident: #ty })
                    } else {
                        Ok(quote! { #param_ident: #ty })
                    }
                } else {
                    // Fallback to original mapping
                    let rust_type = ctx
                        .annotation_aware_mapper
                        .map_type_with_annotations(&param.ty, &self.annotations);
                    update_import_needs(ctx, &rust_type);
                    let ty = rust_type_to_syn(&rust_type)?;
                    if is_param_mutated {
                        Ok(quote! { mut #param_ident: #ty })
                    } else {
                        Ok(quote! { #param_ident: #ty })
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Convert return type using annotation-aware mapping
        let mapped_ret_type = ctx
            .annotation_aware_mapper
            .map_return_type_with_annotations(&self.ret_type, &self.annotations);

        // Check if this is a placeholder Union enum that needs proper generation
        let rust_ret_type =
            if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
                if name == "UnionType" {
                    // Generate a proper enum name and definition from the original Union type
                    if let Type::Union(types) = &self.ret_type {
                        let enum_name = ctx.process_union_type(types);
                        crate::type_mapper::RustType::Custom(enum_name)
                    } else {
                        mapped_ret_type
                    }
                } else {
                    mapped_ret_type
                }
            } else {
                mapped_ret_type
            };

        // Update import needs based on return type
        update_import_needs(ctx, &rust_ret_type);

        // Clone rust_ret_type for generator use (before any partial moves)
        let rust_ret_type_for_generator = rust_ret_type.clone();

        // Check if function can fail and needs Result wrapper
        let can_fail = self.properties.can_fail;
        let error_type_str = if can_fail && !self.properties.error_types.is_empty() {
            // Use first error type or generic for mixed types
            if self.properties.error_types.len() == 1 {
                self.properties.error_types[0].clone()
            } else {
                "Box<dyn std::error::Error>".to_string()
            }
        } else {
            "Box<dyn std::error::Error>".to_string()
        };

        // Mark error types as needed for type generation
        if error_type_str.contains("ZeroDivisionError") {
            ctx.needs_zerodivisionerror = true;
        }
        if error_type_str.contains("IndexError") {
            ctx.needs_indexerror = true;
        }

        let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            if can_fail {
                let error_type: syn::Type = syn::parse_str(&error_type_str)
                    .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
                quote! { -> Result<(), #error_type> }
            } else {
                quote! {}
            }
        } else {
            let mut ty = rust_type_to_syn(&rust_ret_type)?;

            // Check if any parameter escapes through return and uses Cow
            let mut uses_cow_return = false;
            for param in &self.params {
                if let Some(strategy) = lifetime_result.borrowing_strategies.get(&param.name) {
                    if matches!(
                        strategy,
                        crate::borrowing_context::BorrowingStrategy::UseCow { .. }
                    ) {
                        if let Some(_usage) = lifetime_result.param_lifetimes.get(&param.name) {
                            // If a Cow parameter escapes, return type should also be Cow
                            if matches!(self.ret_type, crate::hir::Type::String) {
                                uses_cow_return = true;
                                break;
                            }
                        }
                    }
                }
            }

            if uses_cow_return {
                // Use the same Cow type for return
                ctx.needs_cow = true;
                if let Some(ref return_lt) = lifetime_result.return_lifetime {
                    let lt = syn::Lifetime::new(return_lt, proc_macro2::Span::call_site());
                    ty = parse_quote! { Cow<#lt, str> };
                } else {
                    ty = parse_quote! { Cow<'static, str> };
                }
            } else {
                // Apply return lifetime if needed
                if let Some(ref return_lt) = lifetime_result.return_lifetime {
                    // Check if the return type needs lifetime substitution
                    if matches!(
                        rust_ret_type,
                        crate::type_mapper::RustType::Str { .. }
                            | crate::type_mapper::RustType::Reference { .. }
                    ) {
                        let lt = syn::Lifetime::new(return_lt, proc_macro2::Span::call_site());
                        match rust_ret_type {
                            crate::type_mapper::RustType::Str { .. } => {
                                ty = parse_quote! { &#lt str };
                            }
                            crate::type_mapper::RustType::Reference { mutable, inner, .. } => {
                                let inner_ty = rust_type_to_syn(&inner)?;
                                ty = if mutable {
                                    parse_quote! { &#lt mut #inner_ty }
                                } else {
                                    parse_quote! { &#lt #inner_ty }
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }

            if can_fail {
                let error_type: syn::Type = syn::parse_str(&error_type_str)
                    .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });
                quote! { -> Result<#ty, #error_type> }
            } else {
                quote! { -> #ty }
            }
        };

        // Enter function scope and declare parameters
        ctx.enter_scope();
        ctx.current_function_can_fail = can_fail;
        ctx.current_return_type = Some(self.ret_type.clone());
        for param in &self.params {
            ctx.declare_var(&param.name);
        }

        // Analyze which variables are mutated in the function body
        analyze_mutable_vars(&self.body, ctx);

        // Convert body
        let body_stmts: Vec<_> = self
            .body
            .iter()
            .map(|stmt| stmt.to_rust_tokens(ctx))
            .collect::<Result<Vec<_>>>()?;

        ctx.exit_scope();
        ctx.current_function_can_fail = false;
        ctx.current_return_type = None;

        // Add documentation
        let mut attrs = vec![];

        // Add docstring as documentation if present
        if let Some(docstring) = &self.docstring {
            attrs.push(quote! {
                #[doc = #docstring]
            });
        }

        if self.properties.panic_free {
            attrs.push(quote! {
                #[doc = " Depyler: verified panic-free"]
            });
        }
        if self.properties.always_terminates {
            attrs.push(quote! {
                #[doc = " Depyler: proven to terminate"]
            });
        }

        // Check if function is a generator (contains yield)
        let func_tokens = if self.properties.is_generator {
            // Analyze generator state requirements
            let state_info = GeneratorStateInfo::analyze(self);

            // Generate state struct name (capitalize function name)
            let state_struct_name = format!(
                "{}State",
                name.to_string()
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().to_string())
                    .unwrap_or_default()
                    + &name.to_string()[1..]
            );
            let state_ident = syn::Ident::new(&state_struct_name, name.span());

            // Build state struct fields from analysis (for struct definition)
            let state_fields = generate_state_fields(&state_info, ctx)?;
            let param_fields = generate_param_fields(self, &state_info, ctx)?;
            let all_fields = [state_fields, param_fields].concat();

            // Build field initializers (for struct construction)
            let state_inits = generate_state_initializers(&state_info);
            let param_inits = generate_param_initializers(self, &state_info);
            let all_inits = [state_inits, param_inits].concat();

            // Generate state machine field (tracks which yield point we're at)
            let state_machine_field = quote! {
                state: usize
            };

            // Extract yield value type from return type
            let item_type = extract_generator_item_type(&rust_ret_type_for_generator)?;

            // Populate generator state variables for scoping
            ctx.generator_state_vars.clear();
            for var in &state_info.state_variables {
                ctx.generator_state_vars.insert(var.name.clone());
            }
            for param in &state_info.captured_params {
                ctx.generator_state_vars.insert(param.clone());
            }

            // Generate body statements with in_generator flag set
            ctx.in_generator = true;
            let generator_body_stmts: Vec<_> = self
                .body
                .iter()
                .map(|stmt| stmt.to_rust_tokens(ctx))
                .collect::<Result<Vec<_>>>()?;
            ctx.in_generator = false;
            ctx.generator_state_vars.clear();

            // Generate the complete generator implementation
            quote! {
                #(#attrs)*
                #[doc = " Generator state struct"]
                #[derive(Debug)]
                struct #state_ident {
                    #state_machine_field,
                    #(#all_fields),*
                }

                #[doc = " Generator function - returns Iterator"]
                pub fn #name #generic_params(#(#params),*) -> impl Iterator<Item = #item_type> #where_clause {
                    #state_ident {
                        state: 0,
                        #(#all_inits),*
                    }
                }

                impl Iterator for #state_ident {
                    type Item = #item_type;

                    fn next(&mut self) -> Option<Self::Item> {
                        // NOTE: State machine transformation not yet implemented
                        // Current limitation: yield statements become immediate returns,
                        // causing loops to exit early. Proper implementation requires
                        // transforming control flow into resumable state machine.
                        //
                        // See DEPYLER-0115 Phase 3 for full state machine transformation
                        match self.state {
                            0 => {
                                self.state = 1;
                                // KNOWN ISSUE: Direct body execution causes unreachable code
                                // after yield/return statements. Needs state splitting.
                                #(#generator_body_stmts)*
                                None
                            }
                            _ => None
                        }
                    }
                }
            }
        } else if self.properties.is_async {
            quote! {
                #(#attrs)*
                pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                    #(#body_stmts)*
                }
            }
        } else {
            quote! {
                #(#attrs)*
                pub fn #name #generic_params(#(#params),*) #return_type #where_clause {
                    #(#body_stmts)*
                }
            }
        };

        Ok(func_tokens)
    }
}

/// Helper to build nested dictionary access for assignment
/// Returns (base_expr, access_chain) where access_chain is a vec of index expressions
fn extract_nested_indices_tokens(
    expr: &HirExpr,
    ctx: &mut CodeGenContext,
) -> Result<(syn::Expr, Vec<syn::Expr>)> {
    let mut indices = Vec::new();
    let mut current = expr;

    // Walk up the chain collecting indices
    loop {
        match current {
            HirExpr::Index { base, index } => {
                let index_expr = index.to_rust_expr(ctx)?;
                indices.push(index_expr);
                current = base;
            }
            _ => {
                // We've reached the base
                let base_expr = current.to_rust_expr(ctx)?;
                indices.reverse(); // We collected from inner to outer, need outer to inner
                return Ok((base_expr, indices));
            }
        }
    }
}

/// Check if a type annotation requires explicit conversion
///
/// For numeric types like Int, we always apply conversion to ensure correctness
/// even after optimizations like CSE transform the expression.
/// Complexity: 2 (simple match)
fn needs_type_conversion(target_type: &Type) -> bool {
    // For Int annotations, always apply conversion to handle cases where
    // the value might be usize (from len(), range, etc.)
    matches!(target_type, Type::Int)
}

/// Apply type conversion to value expression
///
/// Wraps the expression with appropriate conversion (e.g., `as i32`)
/// Complexity: 2 (simple match)
fn apply_type_conversion(value_expr: syn::Expr, target_type: &Type) -> syn::Expr {
    match target_type {
        Type::Int => {
            // Convert to i32 using 'as' cast
            // This handles usize->i32 conversions and is a no-op if already i32
            parse_quote! { (#value_expr) as i32 }
        }
        _ => value_expr,
    }
}

impl RustCodeGen for HirStmt {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        match self {
            HirStmt::Assign {
                target,
                value,
                type_annotation,
            } => {
                let mut value_expr = value.to_rust_expr(ctx)?;

                // If there's a type annotation, handle type conversions
                let type_annotation_tokens = if let Some(target_type) = type_annotation {
                    let target_rust_type = ctx.type_mapper.map_type(target_type);
                    let target_syn_type = rust_type_to_syn(&target_rust_type)?;

                    // Check if we need type conversion (e.g., usize to i32)
                    if needs_type_conversion(target_type) {
                        value_expr = apply_type_conversion(value_expr, target_type);
                    }

                    Some(quote! { : #target_syn_type })
                } else {
                    None
                };

                match target {
                    AssignTarget::Symbol(symbol) => {
                        let target_ident = syn::Ident::new(symbol, proc_macro2::Span::call_site());

                        // Inside generators, check if variable is a state variable
                        if ctx.in_generator && ctx.generator_state_vars.contains(symbol) {
                            // State variable assignment: self.field = value
                            Ok(quote! { self.#target_ident = #value_expr; })
                        } else if ctx.is_declared(symbol) {
                            // Variable already exists, just assign
                            Ok(quote! { #target_ident = #value_expr; })
                        } else {
                            // First declaration - check if variable needs mut
                            ctx.declare_var(symbol);
                            if ctx.mutable_vars.contains(symbol) {
                                if let Some(type_ann) = type_annotation_tokens {
                                    Ok(quote! { let mut #target_ident #type_ann = #value_expr; })
                                } else {
                                    Ok(quote! { let mut #target_ident = #value_expr; })
                                }
                            } else if let Some(type_ann) = type_annotation_tokens {
                                Ok(quote! { let #target_ident #type_ann = #value_expr; })
                            } else {
                                Ok(quote! { let #target_ident = #value_expr; })
                            }
                        }
                    }
                    AssignTarget::Index { base, index } => {
                        // Dictionary/list subscript assignment
                        let final_index = index.to_rust_expr(ctx)?;

                        // Extract the base and all intermediate indices
                        let (base_expr, indices) = extract_nested_indices_tokens(base, ctx)?;

                        if indices.is_empty() {
                            // Simple assignment: d[k] = v
                            Ok(quote! { #base_expr.insert(#final_index, #value_expr); })
                        } else {
                            // Nested assignment: build chain of get_mut calls
                            let mut chain = quote! { #base_expr };
                            for idx in &indices {
                                chain = quote! {
                                    #chain.get_mut(&#idx).unwrap()
                                };
                            }

                            Ok(quote! { #chain.insert(#final_index, #value_expr); })
                        }
                    }
                    AssignTarget::Attribute { value, attr } => {
                        // Struct field assignment: obj.field = value
                        let base_expr = value.to_rust_expr(ctx)?;
                        let attr_ident =
                            syn::Ident::new(attr.as_str(), proc_macro2::Span::call_site());
                        Ok(quote! { #base_expr.#attr_ident = #value_expr; })
                    }
                    AssignTarget::Tuple(targets) => {
                        // Tuple unpacking: a, b = value
                        // Check if all targets are simple symbols
                        let all_symbols: Option<Vec<&str>> = targets
                            .iter()
                            .map(|t| match t {
                                AssignTarget::Symbol(s) => Some(s.as_str()),
                                _ => None,
                            })
                            .collect();

                        match all_symbols {
                            Some(symbols) => {
                                let all_declared = symbols.iter().all(|s| ctx.is_declared(s));

                                if all_declared {
                                    // All variables exist, do reassignment
                                    let idents: Vec<_> = symbols
                                        .iter()
                                        .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                                        .collect();
                                    Ok(quote! { (#(#idents),*) = #value_expr; })
                                } else {
                                    // First declaration - mark each variable individually
                                    symbols.iter().for_each(|s| ctx.declare_var(s));
                                    let idents_with_mut: Vec<_> = symbols
                                        .iter()
                                        .map(|s| {
                                            let ident =
                                                syn::Ident::new(s, proc_macro2::Span::call_site());
                                            if ctx.mutable_vars.contains(*s) {
                                                quote! { mut #ident }
                                            } else {
                                                quote! { #ident }
                                            }
                                        })
                                        .collect();
                                    Ok(quote! { let (#(#idents_with_mut),*) = #value_expr; })
                                }
                            }
                            None => {
                                bail!("Complex tuple unpacking not yet supported")
                            }
                        }
                    }
                }
            }
            HirStmt::Return(expr) => {
                if let Some(e) = expr {
                    let expr_tokens = e.to_rust_expr(ctx)?;

                    // Check if return type is Optional and wrap value in Some()
                    let is_optional_return =
                        matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));

                    // Check if the expression is None literal
                    let is_none_literal = matches!(e, HirExpr::Literal(Literal::None));

                    if ctx.current_function_can_fail {
                        if is_optional_return && !is_none_literal {
                            // Wrap value in Some() for Optional return types
                            Ok(quote! { return Ok(Some(#expr_tokens)); })
                        } else {
                            Ok(quote! { return Ok(#expr_tokens); })
                        }
                    } else if is_optional_return && !is_none_literal {
                        // Wrap value in Some() for Optional return types
                        Ok(quote! { return Some(#expr_tokens); })
                    } else {
                        Ok(quote! { return #expr_tokens; })
                    }
                } else if ctx.current_function_can_fail {
                    // No expression - check if return type is Optional
                    let is_optional_return =
                        matches!(ctx.current_return_type.as_ref(), Some(Type::Optional(_)));
                    if is_optional_return {
                        Ok(quote! { return Ok(None); })
                    } else {
                        Ok(quote! { return Ok(()); })
                    }
                } else {
                    Ok(quote! { return; })
                }
            }
            HirStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond = condition.to_rust_expr(ctx)?;
                ctx.enter_scope();
                let then_stmts: Vec<_> = then_body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();

                if let Some(else_stmts) = else_body {
                    ctx.enter_scope();
                    let else_tokens: Vec<_> = else_stmts
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
                    ctx.exit_scope();
                    Ok(quote! {
                        if #cond {
                            #(#then_stmts)*
                        } else {
                            #(#else_tokens)*
                        }
                    })
                } else {
                    Ok(quote! {
                        if #cond {
                            #(#then_stmts)*
                        }
                    })
                }
            }
            HirStmt::While { condition, body } => {
                let cond = condition.to_rust_expr(ctx)?;
                ctx.enter_scope();
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                Ok(quote! {
                    while #cond {
                        #(#body_stmts)*
                    }
                })
            }
            HirStmt::For { target, iter, body } => {
                let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
                let mut iter_expr = iter.to_rust_expr(ctx)?;

                // Check if we're iterating over a borrowed collection
                // If iter is a simple variable that refers to a borrowed collection (e.g., &Vec<T>),
                // we need to add .iter() to properly iterate over it
                if let HirExpr::Var(_var_name) = iter {
                    // This is a simple heuristic: if the expression is just a variable name,
                    // it's likely a parameter or local var that might be borrowed
                    // The generated code already has the variable as borrowed (e.g., data: &Vec<T>)
                    // so we need to call .iter() on it
                    iter_expr = parse_quote! { #iter_expr.iter() };
                }

                ctx.enter_scope();
                ctx.declare_var(target); // for loop variable is declared in the loop scope
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();
                Ok(quote! {
                    for #target_ident in #iter_expr {
                        #(#body_stmts)*
                    }
                })
            }
            HirStmt::Expr(expr) => {
                let expr_tokens = expr.to_rust_expr(ctx)?;
                Ok(quote! { #expr_tokens; })
            }
            HirStmt::Raise {
                exception,
                cause: _,
            } => {
                // For V1, we'll implement basic error handling
                if let Some(exc) = exception {
                    let exc_expr = exc.to_rust_expr(ctx)?;
                    Ok(quote! { return Err(#exc_expr); })
                } else {
                    // Re-raise or bare raise - use generic error
                    Ok(quote! { return Err("Exception raised".into()); })
                }
            }
            HirStmt::Break { label } => {
                if let Some(label_name) = label {
                    let label_ident = syn::Lifetime::new(
                        &format!("'{}", label_name),
                        proc_macro2::Span::call_site(),
                    );
                    Ok(quote! { break #label_ident; })
                } else {
                    Ok(quote! { break; })
                }
            }
            HirStmt::Continue { label } => {
                if let Some(label_name) = label {
                    let label_ident = syn::Lifetime::new(
                        &format!("'{}", label_name),
                        proc_macro2::Span::call_site(),
                    );
                    Ok(quote! { continue #label_ident; })
                } else {
                    Ok(quote! { continue; })
                }
            }
            HirStmt::With {
                context,
                target,
                body,
            } => {
                // Convert context expression
                let context_expr = context.to_rust_expr(ctx)?;

                // Convert body statements
                let body_stmts: Vec<_> = body
                    .iter()
                    .map(|stmt| stmt.to_rust_tokens(ctx))
                    .collect::<Result<_>>()?;

                // Note: Currently generates a simple scope block for context managers.
                // Proper RAII pattern with Drop trait implementation is not yet supported.
                // This is a known limitation - __enter__/__exit__ methods are not translated.
                if let Some(var_name) = target {
                    let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                    ctx.declare_var(var_name);
                    Ok(quote! {
                        {
                            let mut #var_ident = #context_expr;
                            #(#body_stmts)*
                        }
                    })
                } else {
                    Ok(quote! {
                        {
                            let _context = #context_expr;
                            #(#body_stmts)*
                        }
                    })
                }
            }
            HirStmt::Try {
                body,
                handlers,
                orelse: _,
                finalbody,
            } => {
                // Convert try body to statements
                ctx.enter_scope();
                let try_stmts: Vec<_> = body
                    .iter()
                    .map(|s| s.to_rust_tokens(ctx))
                    .collect::<Result<Vec<_>>>()?;
                ctx.exit_scope();

                // Generate except handler code
                let mut handler_tokens = Vec::new();
                for handler in handlers {
                    ctx.enter_scope();

                    // If there's a name binding, declare it in scope
                    if let Some(var_name) = &handler.name {
                        ctx.declare_var(var_name);
                    }

                    let handler_stmts: Vec<_> = handler
                        .body
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
                    ctx.exit_scope();

                    handler_tokens.push(quote! { #(#handler_stmts)* });
                }

                // Generate finally clause if present
                let finally_stmts = if let Some(finally_body) = finalbody {
                    let stmts: Vec<_> = finally_body
                        .iter()
                        .map(|s| s.to_rust_tokens(ctx))
                        .collect::<Result<Vec<_>>>()?;
                    Some(quote! { #(#stmts)* })
                } else {
                    None
                };

                // Generate try/except/finally pattern
                if handlers.is_empty() {
                    // Try/finally without except
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                #(#try_stmts)*
                                #finally_code
                            }
                        })
                    } else {
                        // Just try block
                        Ok(quote! { #(#try_stmts)* })
                    }
                } else if handlers.len() == 1 {
                    let handler_code = &handler_tokens[0];
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(())
                                })();
                                if let Err(_e) = _result {
                                    #handler_code
                                }
                                #finally_code
                            }
                        })
                    } else {
                        Ok(quote! {
                            {
                                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(())
                                })();
                                if let Err(_e) = _result {
                                    #handler_code
                                }
                            }
                        })
                    }
                } else {
                    // Multiple handlers
                    if let Some(finally_code) = finally_stmts {
                        Ok(quote! {
                            {
                                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(())
                                })();
                                if let Err(_e) = _result {
                                    #(#handler_tokens)*
                                }
                                #finally_code
                            }
                        })
                    } else {
                        Ok(quote! {
                            {
                                let _result = (|| -> Result<(), Box<dyn std::error::Error>> {
                                    #(#try_stmts)*
                                    Ok(())
                                })();
                                if let Err(_e) = _result {
                                    #(#handler_tokens)*
                                }
                            }
                        })
                    }
                }
            }
            HirStmt::Pass => {
                // Pass statement generates no code - it's a no-op
                Ok(quote! {})
            }
        }
    }
}

/// Extension trait for converting expressions to Rust
trait ToRustExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr>;
}

/// Expression converter to reduce complexity
struct ExpressionConverter<'a, 'b> {
    ctx: &'a mut CodeGenContext<'b>,
}

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    fn new(ctx: &'a mut CodeGenContext<'b>) -> Self {
        Self { ctx }
    }

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // Inside generators, check if variable is a state variable
        if self.ctx.in_generator && self.ctx.generator_state_vars.contains(name) {
            // Generate self.field for state variables
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Ok(parse_quote! { self.#ident })
        } else {
            // Regular variable
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            Ok(parse_quote! { #ident })
        }
    }

    fn convert_binary(&mut self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = left.to_rust_expr(self.ctx)?;
        let right_expr = right.to_rust_expr(self.ctx)?;

        match op {
            BinOp::In => {
                // Convert "x in dict" to "dict.contains_key(x)" or "dict.contains_key(&x)"
                // String literals are already &str, so don't add extra &
                if matches!(left, HirExpr::Literal(Literal::String(_))) {
                    Ok(parse_quote! { #right_expr.contains_key(#left_expr) })
                } else {
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                }
            }
            BinOp::NotIn => {
                // Convert "x not in dict" to "!dict.contains_key(x)" or "!dict.contains_key(&x)"
                // String literals are already &str, so don't add extra &
                if matches!(left, HirExpr::Literal(Literal::String(_))) {
                    Ok(parse_quote! { !#right_expr.contains_key(#left_expr) })
                } else {
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                }
            }
            BinOp::Add => {
                // Special handling for string concatenation
                // Only use format! if we're certain at least one operand is a string
                let is_definitely_string = matches!(left, HirExpr::Literal(Literal::String(_)))
                    || matches!(right, HirExpr::Literal(Literal::String(_)));

                if is_definitely_string {
                    // This is string concatenation - use format! to handle references properly
                    Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
                } else {
                    // Regular arithmetic addition or unknown types
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // For now, we generate code that works for integers with proper floor semantics
                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        // Avoid != in boolean expression due to formatting issues
                        let r_negative = r < 0;
                        let b_negative = b < 0;
                        let r_nonzero = r != 0;
                        let signs_differ = r_negative != b_negative;
                        let needs_adjustment = r_nonzero && signs_differ;
                        if needs_adjustment { q - 1 } else { q }
                    }
                })
            }
            // Set operators - check if both operands are sets
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                if self.is_set_expr(left) && self.is_set_expr(right) =>
            {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub if self.is_set_expr(left) && self.is_set_expr(right) => {
                // Set difference operation
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if self.is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    Ok(parse_quote! { #left_expr.saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_expr #rust_op #right_expr })
                }
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = elts[0].to_rust_expr(self.ctx)?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        Ok(parse_quote! { #left_expr #rust_op #right_expr })
                    }
                }
            }
            BinOp::Pow => {
                // Python power operator ** needs type-specific handling in Rust
                // For integers: use .pow() with u32 exponent
                // For floats: use .powf() with f64 exponent
                // For negative integer exponents: convert to float

                // Check if we have literals to determine types
                match (left, right) {
                    // Integer literal base with integer literal exponent
                    (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                        if *exp < 0 {
                            // Negative exponent: convert to float operation
                            Ok(parse_quote! {
                                (#left_expr as f64).powf(#right_expr as f64)
                            })
                        } else {
                            // Positive integer exponent: use .pow() with u32
                            // Add checked_pow for overflow safety
                            Ok(parse_quote! {
                                #left_expr.checked_pow(#right_expr as u32)
                                    .expect("Power operation overflowed")
                            })
                        }
                    }
                    // Float literal base: always use .powf()
                    (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                        #left_expr.powf(#right_expr as f64)
                    }),
                    // Any base with float exponent: use .powf()
                    (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                        (#left_expr as f64).powf(#right_expr)
                    }),
                    // Variables or complex expressions: generate type-safe code
                    _ => {
                        // For non-literal expressions, we need runtime type checking
                        // This is a conservative approach that works for common cases
                        Ok(parse_quote! {
                            {
                                // Try integer power first if exponent can be u32
                                if #right_expr >= 0 && #right_expr <= u32::MAX as i64 {
                                    #left_expr.checked_pow(#right_expr as u32)
                                        .expect("Power operation overflowed")
                                } else {
                                    // Fall back to float power for negative or large exponents
                                    (#left_expr as f64).powf(#right_expr as f64) as i64
                                }
                            }
                        })
                    }
                }
            }
            _ => {
                let rust_op = convert_binop(op)?;
                Ok(parse_quote! { #left_expr #rust_op #right_expr })
            }
        }
    }

    fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = operand.to_rust_expr(self.ctx)?;
        match op {
            UnaryOp::Not => Ok(parse_quote! { !#operand_expr }),
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    fn convert_call(&mut self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Handle classmethod cls(args) → Self::new(args)
        if func == "cls" && self.ctx.is_classmethod {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return Ok(parse_quote! { Self::new(#(#arg_exprs),*) });
        }

        // Handle map() with lambda → convert to Rust iterator pattern
        if func == "map" && args.len() >= 2 {
            if let Some(result) = self.try_convert_map_with_zip(args)? {
                return Ok(result);
            }
        }

        // Handle sum(generator_exp) → generator_exp.sum()
        if func == "sum" && args.len() == 1
            && matches!(args[0], HirExpr::GeneratorExp { .. }) {
                let gen_expr = args[0].to_rust_expr(self.ctx)?;
                return Ok(parse_quote! { #gen_expr.sum() });
            }

        // Handle max(generator_exp) → generator_exp.max()
        if func == "max" && args.len() == 1
            && matches!(args[0], HirExpr::GeneratorExp { .. }) {
                let gen_expr = args[0].to_rust_expr(self.ctx)?;
                return Ok(parse_quote! { #gen_expr.max() });
            }

        // Handle enumerate(items) → items.into_iter().enumerate()
        if func == "enumerate" && args.len() == 1 {
            let items_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #items_expr.into_iter().enumerate() });
        }

        // Handle zip(a, b, ...) → a.iter().zip(b.iter()).zip(c.iter())...
        if func == "zip" && args.len() >= 2 {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;

            // Start with first.iter()
            let first = &arg_exprs[0];
            let mut chain: syn::Expr = parse_quote! { #first.iter() };

            // Chain .zip() for each subsequent argument
            for arg in &arg_exprs[1..] {
                chain = parse_quote! { #chain.zip(#arg.iter()) };
            }

            return Ok(chain);
        }

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            _ => self.convert_generic_call(func, &arg_exprs),
        }
    }

    fn try_convert_map_with_zip(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
        // Check if first argument is a lambda
        if let HirExpr::Lambda { params, body } = &args[0] {
            let num_iterables = args.len() - 1;

            // Check if lambda has matching number of parameters
            if params.len() != num_iterables {
                bail!(
                    "Lambda has {} parameters but map() called with {} iterables",
                    params.len(),
                    num_iterables
                );
            }

            // Convert the iterables
            let mut iterable_exprs: Vec<syn::Expr> = Vec::new();
            for iterable in &args[1..] {
                iterable_exprs.push(iterable.to_rust_expr(self.ctx)?);
            }

            // Create lambda parameter pattern
            let param_idents: Vec<syn::Ident> = params
                .iter()
                .map(|p| syn::Ident::new(p, proc_macro2::Span::call_site()))
                .collect();

            // Convert lambda body
            let body_expr = body.to_rust_expr(self.ctx)?;

            // Handle based on number of iterables
            if num_iterables == 1 {
                // Single iterable: iterable.iter().map(|x| ...).collect()
                let iter_expr = &iterable_exprs[0];
                let param = &param_idents[0];
                Ok(Some(parse_quote! {
                    #iter_expr.iter().map(|#param| #body_expr).collect::<Vec<_>>()
                }))
            } else {
                // Multiple iterables: use zip pattern
                // Build the zip chain
                let first_iter = &iterable_exprs[0];
                let mut zip_expr: syn::Expr = parse_quote! { #first_iter.iter() };

                for iter_expr in &iterable_exprs[1..] {
                    zip_expr = parse_quote! { #zip_expr.zip(#iter_expr.iter()) };
                }

                // Build the tuple pattern based on number of parameters
                let tuple_pat: syn::Pat = if param_idents.len() == 2 {
                    let p0 = &param_idents[0];
                    let p1 = &param_idents[1];
                    parse_quote! { (#p0, #p1) }
                } else if param_idents.len() == 3 {
                    // For 3 parameters, zip creates ((a, b), c)
                    let p0 = &param_idents[0];
                    let p1 = &param_idents[1];
                    let p2 = &param_idents[2];
                    parse_quote! { ((#p0, #p1), #p2) }
                } else {
                    // For 4+ parameters, continue the nested pattern
                    bail!("map() with more than 3 iterables is not yet supported");
                };

                // Generate the final expression
                Ok(Some(parse_quote! {
                    #zip_expr.map(|#tuple_pat| #body_expr).collect::<Vec<_>>()
                }))
            }
        } else {
            // Not a lambda, fall through to normal handling
            Ok(None)
        }
    }

    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        Ok(parse_quote! { #arg.len() })
    }

    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => {
                let start = &args[0];
                let end = &args[1];
                let step = &args[2];

                // Check if step is negative by looking at the expression
                let is_negative_step = if let syn::Expr::Unary(unary) = step {
                    matches!(unary.op, syn::UnOp::Neg(_))
                } else {
                    false
                };

                if is_negative_step {
                    // For negative steps, we need to reverse the range
                    // Python: range(10, 0, -1) → Rust: (0..10).rev()
                    // But we also need to handle step sizes > 1
                    Ok(parse_quote! {
                        {
                            let step = (#step).abs() as usize;
                            if step == 0 {
                                panic!("range() arg 3 must not be zero");
                            }
                            if step == 1 {
                                (#end..#start).rev()
                            } else {
                                (#end..#start).rev().step_by(step)
                            }
                        }
                    })
                } else {
                    // Positive step - check for zero
                    Ok(parse_quote! {
                        {
                            let step = #step as usize;
                            if step == 0 {
                                panic!("range() arg 3 must not be zero");
                            }
                            (#start..#end).step_by(step)
                        }
                    })
                }
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    fn convert_array_init_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        _arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }

        // Extract size from first argument if it's a literal
        if let HirExpr::Literal(Literal::Int(size)) = &args[0] {
            if *size > 0 && *size <= 32 {
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                match func {
                    "zeros" => Ok(parse_quote! { [0; #size_lit] }),
                    "ones" => Ok(parse_quote! { [1; #size_lit] }),
                    "full" => {
                        if args.len() >= 2 {
                            let value = args[1].to_rust_expr(self.ctx)?;
                            Ok(parse_quote! { [#value; #size_lit] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                // For large arrays or dynamic sizes, fall back to vec!
                match func {
                    "zeros" => {
                        let size_expr = args[0].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![0; #size_expr as usize] })
                    }
                    "ones" => {
                        let size_expr = args[0].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![1; #size_expr as usize] })
                    }
                    "full" => {
                        if args.len() >= 2 {
                            let size_expr = args[0].to_rust_expr(self.ctx)?;
                            let value = args[1].to_rust_expr(self.ctx)?;
                            Ok(parse_quote! { vec![#value; #size_expr as usize] })
                        } else {
                            bail!("full() requires a value argument");
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            // Dynamic size - use vec!
            let size_expr = args[0].to_rust_expr(self.ctx)?;
            match func {
                "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
                "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
                "full" => {
                    if args.len() >= 2 {
                        let value = args[1].to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { vec![#value; #size_expr as usize] })
                    } else {
                        bail!("full() requires a value argument");
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn convert_set_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        if args.is_empty() {
            // Empty set: set()
            Ok(parse_quote! { HashSet::new() })
        } else if args.len() == 1 {
            // Set from iterable: set([1, 2, 3])
            let arg = &args[0];
            Ok(parse_quote! {
                #arg.into_iter().collect::<HashSet<_>>()
            })
        } else {
            bail!("set() takes at most 1 argument ({} given)", args.len())
        }
    }

    fn convert_frozenset_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        if args.is_empty() {
            // Empty frozenset: frozenset()
            // In Rust, we can use Arc<HashSet> to make it immutable
            Ok(parse_quote! { std::sync::Arc::new(HashSet::new()) })
        } else if args.len() == 1 {
            // Frozenset from iterable: frozenset([1, 2, 3])
            let arg = &args[0];
            Ok(parse_quote! {
                std::sync::Arc::new(#arg.into_iter().collect::<HashSet<_>>())
            })
        } else {
            bail!(
                "frozenset() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

    fn convert_generic_call(&self, func: &str, args: &[syn::Expr]) -> Result<syn::Expr> {
        // Check if this is an imported function
        if let Some(rust_path) = self.ctx.imported_items.get(func) {
            // Parse the rust path and generate the call
            let path_parts: Vec<&str> = rust_path.split("::").collect();
            let mut path = quote! {};
            for (i, part) in path_parts.iter().enumerate() {
                let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                if i == 0 {
                    path = quote! { #part_ident };
                } else {
                    path = quote! { #path::#part_ident };
                }
            }
            if args.is_empty() {
                return Ok(parse_quote! { #path() });
            } else {
                return Ok(parse_quote! { #path(#(#args),*) });
            }
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // Treat as constructor call - ClassName::new(args)
            let class_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            if args.is_empty() {
                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                match func {
                    "Counter" => Ok(parse_quote! { #class_ident::new(0) }),
                    _ => Ok(parse_quote! { #class_ident::new() }),
                }
            } else {
                Ok(parse_quote! { #class_ident::new(#(#args),*) })
            }
        } else {
            // Regular function call
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            Ok(parse_quote! { #func_ident(#(#args),*) })
        }
    }

    fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // Handle classmethod cls.method() → Self::method()
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.ctx.is_classmethod {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { Self::#method_ident(#(#arg_exprs),*) });
            }
        }

        // Check if this is a module method call (e.g., os.getcwd())
        if let HirExpr::Var(module_name) = object {
            let rust_name_opt = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| mapping.item_map.get(method).cloned());

            if let Some(rust_name) = rust_name_opt {
                // Convert args
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                // Build the Rust function path
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                let mut path = quote! { std };
                for part in path_parts {
                    let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                    path = quote! { #path::#part_ident };
                }

                // Special handling for certain functions
                match rust_name.as_str() {
                    "env::current_dir" => {
                        // current_dir returns Result<PathBuf>, we need to convert to String
                        return Ok(parse_quote! {
                            #path().unwrap().to_string_lossy().to_string()
                        });
                    }
                    "Regex::new" => {
                        // re.compile(pattern) -> Regex::new(pattern)
                        if arg_exprs.is_empty() {
                            bail!("re.compile() requires a pattern argument");
                        }
                        let pattern = &arg_exprs[0];
                        return Ok(parse_quote! {
                            regex::Regex::new(#pattern).unwrap()
                        });
                    }
                    _ => {
                        if arg_exprs.is_empty() {
                            return Ok(parse_quote! { #path() });
                        } else {
                            return Ok(parse_quote! { #path(#(#arg_exprs),*) });
                        }
                    }
                }
            }
        }

        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Map Python collection methods to Rust equivalents
        match method {
            // List methods
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push(#arg) })
            }
            "extend" => {
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.extend(#arg) })
            }
            "pop" => {
                if self.is_set_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments for sets");
                    }
                    // HashSet doesn't have pop(), simulate with iter().next() and remove
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else {
                    // List pop
                    if arg_exprs.is_empty() {
                        Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                    } else {
                        bail!("pop() with index not supported in V1");
                    }
                }
            }
            "insert" => {
                if arg_exprs.len() != 2 {
                    bail!("insert() requires exactly two arguments");
                }
                let index = &arg_exprs[0];
                let value = &arg_exprs[1];
                Ok(parse_quote! { #object_expr.insert(#index as usize, #value) })
            }
            "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                // Check if it's a set or list based on the object expression
                if self.is_set_expr(object) {
                    // HashSet's remove returns bool, Python's raises KeyError if not found
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#value) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    // List remove behavior
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#value) {
                            #object_expr.remove(pos)
                        } else {
                            panic!("ValueError: list.remove(x): x not in list")
                        }
                    })
                }
            }

            // Dict methods
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.get(&#key).cloned() })
                } else if arg_exprs.len() == 2 {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    Ok(parse_quote! { #object_expr.get(&#key).cloned().unwrap_or(#default) })
                } else {
                    bail!("get() requires 1 or 2 arguments");
                }
            }
            "keys" => {
                if !arg_exprs.is_empty() {
                    bail!("keys() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
            }
            "values" => {
                if !arg_exprs.is_empty() {
                    bail!("values() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
            }
            "items" => {
                if !arg_exprs.is_empty() {
                    bail!("items() takes no arguments");
                }
                Ok(
                    parse_quote! { #object_expr.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
                )
            }
            "update" => {
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    for (k, v) in #arg {
                        #object_expr.insert(k, v);
                    }
                })
            }

            // String methods
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }
            "strip" => {
                if !arg_exprs.is_empty() {
                    bail!("strip() with arguments not supported in V1");
                }
                Ok(parse_quote! { #object_expr.trim().to_string() })
            }
            "startswith" => {
                if arg_exprs.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                let prefix = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.starts_with(#prefix) })
            }
            "endswith" => {
                if arg_exprs.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                let suffix = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.ends_with(#suffix) })
            }
            "split" => {
                if arg_exprs.is_empty() {
                    Ok(
                        parse_quote! { #object_expr.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    let sep = &arg_exprs[0];
                    Ok(
                        parse_quote! { #object_expr.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() with maxsplit not supported in V1");
                }
            }
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                Ok(parse_quote! { #iterable.join(#object_expr) })
            }

            // Set methods
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // discard() is like remove() but doesn't raise error
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }

            // Regex methods
            "findall" => {
                // regex.findall(text) -> regex.find_iter(text).map(|m| m.as_str().to_string()).collect()
                if arg_exprs.is_empty() {
                    bail!("findall() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.find_iter(#text)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<String>>()
                })
            }

            // Generic method call fallback
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
            }
        }
    }

    fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // Discriminate between HashMap and Vec access based on base type or index type
        let is_string_key = self.is_string_index(base, index)?;

        if is_string_key {
            // HashMap/Dict access with string keys
            match index {
                HirExpr::Literal(Literal::String(s)) => {
                    // String literal - use it directly without .to_string()
                    Ok(parse_quote! {
                        #base_expr.get(#s).cloned().unwrap_or_default()
                    })
                }
                _ => {
                    // String variable - needs proper referencing
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    Ok(parse_quote! {
                        #base_expr.get(#index_expr).cloned().unwrap_or_default()
                    })
                }
            }
        } else {
            // Vec/List access with numeric index
            let index_expr = index.to_rust_expr(self.ctx)?;
            Ok(parse_quote! {
                #base_expr.get(#index_expr as usize).copied().unwrap_or_default()
            })
        }
    }

    /// Check if the index expression is a string key (for HashMap access)
    /// Returns true if: index is string literal, OR base is Dict/HashMap type
    fn is_string_index(&self, base: &HirExpr, index: &HirExpr) -> Result<bool> {
        // Check 1: Is index a string literal?
        if matches!(index, HirExpr::Literal(Literal::String(_))) {
            return Ok(true);
        }

        // Check 2: Is base expression a Dict/HashMap type?
        // We need to look at the base's inferred type
        if let HirExpr::Var(sym) = base {
            // Try to find the variable's type in the current function context
            // For parameters, we can check the function signature
            // For local variables, this is harder without full type inference
            //
            // Heuristic: If the symbol name contains "dict" or "data" or "map"
            // and index doesn't look numeric, assume HashMap
            let name = sym.as_str();
            if (name.contains("dict") || name.contains("data") || name.contains("map"))
                && !self.is_numeric_index(index)
            {
                return Ok(true);
            }
        }

        // Check 3: Does the index expression look like a string variable?
        if self.is_string_variable(index) {
            return Ok(true);
        }

        // Default: assume numeric index (Vec/List access)
        Ok(false)
    }

    /// Check if expression is likely a string variable (heuristic)
    fn is_string_variable(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Heuristic: variable names like "key", "name", "id", "word", etc.
                name == "key"
                    || name == "name"
                    || name == "id"
                    || name == "word"
                    || name == "text"
                    || name.ends_with("_key")
                    || name.ends_with("_name")
            }
            _ => false,
        }
    }

    /// Check if expression is likely numeric (heuristic)
    fn is_numeric_index(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::Int(_)) => true,
            HirExpr::Var(sym) => {
                let name = sym.as_str();
                // Common numeric index names
                name == "i"
                    || name == "j"
                    || name == "k"
                    || name == "idx"
                    || name == "index"
                    || name.starts_with("idx_")
                    || name.ends_with("_idx")
                    || name.ends_with("_index")
            }
            HirExpr::Binary { .. } => true, // Arithmetic expressions are numeric
            HirExpr::Call { .. } => false,  // Could be anything
            _ => false,
        }
    }

    fn convert_slice(
        &mut self,
        base: &HirExpr,
        start: &Option<Box<HirExpr>>,
        stop: &Option<Box<HirExpr>>,
        step: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let base_expr = base.to_rust_expr(self.ctx)?;

        // Convert slice parameters
        let start_expr = if let Some(s) = start {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let stop_expr = if let Some(s) = stop {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        let step_expr = if let Some(s) = step {
            Some(s.to_rust_expr(self.ctx)?)
        } else {
            None
        };

        // Generate slice code based on the parameters
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let step = #step;
                        if step == 1 {
                            #base_expr.clone()
                        } else if step > 0 {
                            #base_expr.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            #base_expr.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            #base_expr.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    let stop = (#stop).max(0) as usize;
                    if start < #base_expr.len() {
                        #base_expr[start..stop.min(#base_expr.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    if start < #base_expr.len() {
                        #base_expr[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let stop = (#stop).max(0) as usize;
                    #base_expr[..stop.min(#base_expr.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let start = (#start).max(0) as usize;
                        let stop = (#stop).max(0) as usize;
                        let step = #step;

                        if step == 1 {
                            if start < #base_expr.len() {
                                #base_expr[start..stop.min(#base_expr.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            #base_expr[start..stop.min(#base_expr.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < #base_expr.len() {
                                #base_expr[start..stop.min(#base_expr.len())]
                                    .iter()
                                    .rev()
                                    .step_by(abs_step)
                                    .cloned()
                                    .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            }
                        }
                    }
                })
            }

            // Start and step: base[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let start = (#start).max(0) as usize;
                    let step = #step;

                    if start < #base_expr.len() {
                        if step == 1 {
                            #base_expr[start..].to_vec()
                        } else if step > 0 {
                            #base_expr[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            #base_expr[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            #base_expr[start..]
                                .iter()
                                .rev()
                                .step_by(abs_step)
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop and step: base[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let stop = (#stop).max(0) as usize;
                    let step = #step;

                    if step == 1 {
                        #base_expr[..stop.min(#base_expr.len())].to_vec()
                    } else if step > 0 {
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        #base_expr[..stop.min(#base_expr.len())]
                            .iter()
                            .rev()
                            .step_by(abs_step)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }),
        }
    }

    fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Always use vec! for now to ensure mutability works
        // In the future, we should analyze if the list is mutated before deciding
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        self.ctx.needs_hashmap = true;
        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let key_expr = key.to_rust_expr(self.ctx)?;
            let val_expr = value.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
        }
        Ok(parse_quote! {
            {
                let mut map = HashMap::new();
                #(#insert_stmts)*
                map
            }
        })
    }

    fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| e.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_set(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        let mut insert_stmts = Vec::new();
        for elem in elts {
            let elem_expr = elem.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { set.insert(#elem_expr); });
        }
        Ok(parse_quote! {
            {
                let mut set = HashSet::new();
                #(#insert_stmts)*
                set
            }
        })
    }

    fn convert_frozenset(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        self.ctx.needs_arc = true;
        let mut insert_stmts = Vec::new();
        for elem in elts {
            let elem_expr = elem.to_rust_expr(self.ctx)?;
            insert_stmts.push(quote! { set.insert(#elem_expr); });
        }
        Ok(parse_quote! {
            {
                let mut set = HashSet::new();
                #(#insert_stmts)*
                std::sync::Arc::new(set)
            }
        })
    }

    fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { Self::#attr_ident });
            }
        }

        // Check if this is a module attribute access
        if let HirExpr::Var(module_name) = value {
            let rust_name_opt = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| mapping.item_map.get(attr).cloned());

            if let Some(rust_name) = rust_name_opt {
                // Map to the Rust equivalent
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                if path_parts.len() > 1 {
                    // It's a path like "env::current_dir"
                    let mut path = quote! { std };
                    for part in path_parts {
                        let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                        path = quote! { #path::#part_ident };
                    }
                    return Ok(parse_quote! { #path });
                } else {
                    // Simple identifier
                    let ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #ident });
                }
            }
        }

        // Default behavior for non-module attributes
        let value_expr = value.to_rust_expr(self.ctx)?;
        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
        Ok(parse_quote! { #value_expr.#attr_ident })
    }

    fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        let expr_tokens = expr.to_rust_expr(self.ctx)?;
        if mutable {
            Ok(parse_quote! { &mut #expr_tokens })
        } else {
            Ok(parse_quote! { &#expr_tokens })
        }
    }

    fn convert_list_comp(
        &mut self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let element_expr = element.to_rust_expr(self.ctx)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_ident| #cond_expr)
                    .map(|#target_ident| #element_expr)
                    .collect::<Vec<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_ident| #element_expr)
                    .collect::<Vec<_>>()
            })
        }
    }

    fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_name) => {
                // For rust_gen, we're more conservative since we don't have type info
                // Only treat explicit set literals and calls as sets
                false
            }
            _ => false,
        }
    }

    fn convert_set_operation(
        &self,
        op: BinOp,
        left: syn::Expr,
        right: syn::Expr,
    ) -> Result<syn::Expr> {
        match op {
            BinOp::BitAnd => Ok(parse_quote! {
                #left.intersection(&#right).cloned().collect()
            }),
            BinOp::BitOr => Ok(parse_quote! {
                #left.union(&#right).cloned().collect()
            }),
            BinOp::Sub => Ok(parse_quote! {
                #left.difference(&#right).cloned().collect()
            }),
            BinOp::BitXor => Ok(parse_quote! {
                #left.symmetric_difference(&#right).cloned().collect()
            }),
            _ => bail!("Invalid set operator"),
        }
    }

    fn convert_set_comp(
        &mut self,
        element: &HirExpr,
        target: &str,
        iter: &HirExpr,
        condition: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        let target_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let iter_expr = iter.to_rust_expr(self.ctx)?;
        let element_expr = element.to_rust_expr(self.ctx)?;

        if let Some(cond) = condition {
            // With condition: iter().filter().map().collect()
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .filter(|#target_ident| #cond_expr)
                    .map(|#target_ident| #element_expr)
                    .collect::<HashSet<_>>()
            })
        } else {
            // Without condition: iter().map().collect()
            Ok(parse_quote! {
                #iter_expr
                    .into_iter()
                    .map(|#target_ident| #element_expr)
                    .collect::<HashSet<_>>()
            })
        }
    }

    fn convert_lambda(&mut self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // Convert parameters to pattern identifiers
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = syn::Ident::new(p, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = body.to_rust_expr(self.ctx)?;

        // Generate closure
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { |#(#param_pats),*| #body_expr })
        }
    }

    /// Check if an expression is a len() call
    fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args } if func == "len" && args.len() == 1)
    }

    fn convert_await(&mut self, value: &HirExpr) -> Result<syn::Expr> {
        let value_expr = value.to_rust_expr(self.ctx)?;
        Ok(parse_quote! { #value_expr.await })
    }

    fn convert_yield(&mut self, value: &Option<Box<HirExpr>>) -> Result<syn::Expr> {
        if self.ctx.in_generator {
            // Inside Iterator::next() - convert to return Some(value)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { return Some(#value_expr) })
            } else {
                Ok(parse_quote! { return None })
            }
        } else {
            // Outside generator context - keep as yield (placeholder for future)
            if let Some(v) = value {
                let value_expr = v.to_rust_expr(self.ctx)?;
                Ok(parse_quote! { yield #value_expr })
            } else {
                Ok(parse_quote! { yield })
            }
        }
    }

    fn convert_fstring(&mut self, parts: &[FStringPart]) -> Result<syn::Expr> {
        // Handle empty f-strings
        if parts.is_empty() {
            return Ok(parse_quote! { "".to_string() });
        }

        // Check if it's just a plain string (no expressions)
        let has_expressions = parts.iter().any(|p| matches!(p, FStringPart::Expr(_)));

        if !has_expressions {
            // Just literal parts - concatenate them
            let mut result = String::new();
            for part in parts {
                if let FStringPart::Literal(s) = part {
                    result.push_str(s);
                }
            }
            return Ok(parse_quote! { #result.to_string() });
        }

        // Build format string template and collect arguments
        let mut template = String::new();
        let mut args = Vec::new();

        for part in parts {
            match part {
                FStringPart::Literal(s) => {
                    template.push_str(s);
                }
                FStringPart::Expr(expr) => {
                    template.push_str("{}");
                    let arg_expr = expr.to_rust_expr(self.ctx)?;
                    args.push(arg_expr);
                }
            }
        }

        // Generate format!() macro call
        if args.is_empty() {
            // No arguments (shouldn't happen but be safe)
            Ok(parse_quote! { #template.to_string() })
        } else {
            // Build the format! call with template and arguments
            Ok(parse_quote! { format!(#template, #(#args),*) })
        }
    }

    fn convert_ifexpr(&mut self, test: &HirExpr, body: &HirExpr, orelse: &HirExpr) -> Result<syn::Expr> {
        let test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }

    fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse: bool,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // Create the closure parameter pattern
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = syn::Ident::new(&key_params[0], proc_macro2::Span::call_site());
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // Generate: { let mut result = iterable.clone(); result.sort_by_key(|param| body); [result.reverse();] result }
        if reverse {
            Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort_by_key(|#param_pat| #body_expr);
                    __sorted_result.reverse();
                    __sorted_result
                }
            })
        } else {
            Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort_by_key(|#param_pat| #body_expr);
                    __sorted_result
                }
            })
        }
    }

    fn convert_generator_expression(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Strategy: Simple cases use iterator chains, nested use flat_map

        if generators.is_empty() {
            bail!("Generator expression must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

            // Add filters for each condition
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            return Ok(chain);
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: (x + y for x in range(3) for y in range(3))
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y))

        self.convert_nested_generators(element, generators)
    }

    fn convert_nested_generators(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build the nested expression recursively
        let inner_expr = self.build_nested_chain(element, generators, 1)?;

        // Start the chain with the first generator
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // Add filters for first generator's conditions
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
    }

    fn build_nested_chain(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return the element expression
            let element_expr = element.to_rust_expr(self.ctx)?;
            return Ok(element_expr);
        }

        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build the inner expression (recursive)
        let inner_expr = self.build_nested_chain(element, generators, depth + 1)?;

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // Add filters for this generator's conditions
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
        }

        // Use flat_map for intermediate generators, map for the last
        if depth < generators.len() - 1 {
            // Intermediate generator: use flat_map
            chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_expr) };
        } else {
            // Last generator: use map
            chain = parse_quote! { #chain.map(move |#target_pat| #inner_expr) };
        }

        Ok(chain)
    }

    fn parse_target_pattern(&self, target: &str) -> Result<syn::Pat> {
        // Handle simple variable: x
        // Handle tuple: (x, y)
        if target.starts_with('(') && target.ends_with(')') {
            // Tuple pattern
            let inner = &target[1..target.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            let idents: Vec<syn::Ident> = parts
                .iter()
                .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()))
                .collect();
            Ok(parse_quote! { ( #(#idents),* ) })
        } else {
            // Simple variable
            let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
            Ok(parse_quote! { #ident })
        }
    }
}

impl ToRustExpr for HirExpr {
    fn to_rust_expr(&self, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
        let mut converter = ExpressionConverter::new(ctx);

        match self {
            HirExpr::Literal(lit) => {
                let expr = literal_to_rust_expr(lit, &ctx.string_optimizer, &ctx.needs_cow, ctx);
                if let Literal::String(s) = lit {
                    let context = StringContext::Literal(s.clone());
                    if matches!(
                        ctx.string_optimizer.get_optimal_type(&context),
                        crate::string_optimization::OptimalStringType::CowStr
                    ) {
                        ctx.needs_cow = true;
                    }
                }
                Ok(expr)
            }
            HirExpr::Var(name) => converter.convert_variable(name),
            HirExpr::Binary { op, left, right } => converter.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => converter.convert_unary(op, operand),
            HirExpr::Call { func, args } => converter.convert_call(func, args),
            HirExpr::MethodCall {
                object,
                method,
                args,
            } => converter.convert_method_call(object, method, args),
            HirExpr::Index { base, index } => converter.convert_index(base, index),
            HirExpr::Slice {
                base,
                start,
                stop,
                step,
            } => converter.convert_slice(base, start, stop, step),
            HirExpr::List(elts) => converter.convert_list(elts),
            HirExpr::Dict(items) => converter.convert_dict(items),
            HirExpr::Tuple(elts) => converter.convert_tuple(elts),
            HirExpr::Set(elts) => converter.convert_set(elts),
            HirExpr::FrozenSet(elts) => converter.convert_frozenset(elts),
            HirExpr::Attribute { value, attr } => converter.convert_attribute(value, attr),
            HirExpr::Borrow { expr, mutable } => converter.convert_borrow(expr, *mutable),
            HirExpr::ListComp {
                element,
                target,
                iter,
                condition,
            } => converter.convert_list_comp(element, target, iter, condition),
            HirExpr::Lambda { params, body } => converter.convert_lambda(params, body),
            HirExpr::SetComp {
                element,
                target,
                iter,
                condition,
            } => converter.convert_set_comp(element, target, iter, condition),
            HirExpr::Await { value } => converter.convert_await(value),
            HirExpr::Yield { value } => converter.convert_yield(value),
            HirExpr::FString { parts } => converter.convert_fstring(parts),
            HirExpr::IfExpr { test, body, orelse } => converter.convert_ifexpr(test, body, orelse),
            HirExpr::SortByKey {
                iterable,
                key_params,
                key_body,
                reverse,
            } => converter.convert_sort_by_key(iterable, key_params, key_body, *reverse),
            HirExpr::GeneratorExp { element, generators } => {
                converter.convert_generator_expression(element, generators)
            }
        }
    }
}

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
    ctx: &CodeGenContext,
) -> syn::Expr {
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::Float(f) => {
            let lit = syn::LitFloat::new(&f.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::String(s) => {
            // Check if this string should be interned
            if let Some(interned_name) = string_optimizer.get_interned_name(s) {
                let ident = syn::Ident::new(&interned_name, proc_macro2::Span::call_site());
                parse_quote! { #ident }
            } else {
                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());

                // Use string optimizer to determine if we need .to_string()
                let context = StringContext::Literal(s.clone());
                match string_optimizer.get_optimal_type(&context) {
                    crate::string_optimization::OptimalStringType::StaticStr => {
                        // For read-only strings, just use the literal
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::BorrowedStr { .. } => {
                        // Use &'static str for literals that can be borrowed
                        parse_quote! { #lit }
                    }
                    crate::string_optimization::OptimalStringType::CowStr => {
                        // Check if we're in a context where String is required
                        if let Some(Type::String) = &ctx.current_return_type {
                            // Function returns String, so convert to owned
                            parse_quote! { #lit.to_string() }
                        } else {
                            // Use Cow for flexible ownership
                            parse_quote! { std::borrow::Cow::Borrowed(#lit) }
                        }
                    }
                    crate::string_optimization::OptimalStringType::OwnedString => {
                        // Only use .to_string() when absolutely necessary
                        parse_quote! { #lit.to_string() }
                    }
                }
            }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::None => parse_quote! { None },
    }
}

fn convert_binop(op: BinOp) -> Result<syn::BinOp> {
    use BinOp::*;

    match op {
        // Arithmetic operators
        Add => Ok(parse_quote! { + }),
        Sub => Ok(parse_quote! { - }),
        Mul => Ok(parse_quote! { * }),
        Div => Ok(parse_quote! { / }),
        Mod => Ok(parse_quote! { % }),

        // Special arithmetic cases handled by convert_binary
        FloorDiv => {
            bail!("Floor division handled by convert_binary with Python semantics")
        }
        Pow => bail!("Power operator handled by convert_binary with type-specific logic"),

        // Comparison operators
        Eq => Ok(parse_quote! { == }),
        NotEq => Ok(parse_quote! { != }),
        Lt => Ok(parse_quote! { < }),
        LtEq => Ok(parse_quote! { <= }),
        Gt => Ok(parse_quote! { > }),
        GtEq => Ok(parse_quote! { >= }),

        // Logical operators
        And => Ok(parse_quote! { && }),
        Or => Ok(parse_quote! { || }),

        // Bitwise operators
        BitAnd => Ok(parse_quote! { & }),
        BitOr => Ok(parse_quote! { | }),
        BitXor => Ok(parse_quote! { ^ }),
        LShift => Ok(parse_quote! { << }),
        RShift => Ok(parse_quote! { >> }),

        // Special membership operators handled in convert_binary
        In | NotIn => bail!("in/not in operators should be handled by convert_binary"),
    }
}

/// Convert Str type with optional lifetime to syn::Type
///
/// Handles both `&str` and `&'a str` variants.
/// Complexity: 2 (single if/else branch)
fn str_type_to_syn(lifetime: &Option<String>) -> syn::Type {
    if let Some(lt) = lifetime {
        let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
        parse_quote! { &#lt_ident str }
    } else {
        parse_quote! { &str }
    }
}

/// Convert Reference type with mutable and lifetime to syn::Type
///
/// Handles all 4 combinations of mutable × lifetime:
/// - `&T`, `&mut T`, `&'a T`, `&'a mut T`
///
/// Complexity: 5 (nested if/else for mutable and lifetime)
fn reference_type_to_syn(
    lifetime: &Option<String>,
    mutable: bool,
    inner: &crate::type_mapper::RustType,
) -> Result<syn::Type> {
    let inner_ty = rust_type_to_syn(inner)?;

    Ok(if mutable {
        if let Some(lt) = lifetime {
            let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
            parse_quote! { &#lt_ident mut #inner_ty }
        } else {
            parse_quote! { &mut #inner_ty }
        }
    } else if let Some(lt) = lifetime {
        let lt_ident = syn::Lifetime::new(lt, proc_macro2::Span::call_site());
        parse_quote! { &#lt_ident #inner_ty }
    } else {
        parse_quote! { &#inner_ty }
    })
}

/// Convert Array type with const generic size to syn::Type
///
/// Handles 3 const generic size variants:
/// - Literal: `[T; 10]`
/// - Parameter: `[T; N]`
/// - Expression: `[T; SIZE * 2]`
///
/// Complexity: 4 (match with 3 arms)
fn array_type_to_syn(
    element_type: &crate::type_mapper::RustType,
    size: &crate::type_mapper::RustConstGeneric,
) -> Result<syn::Type> {
    let element = rust_type_to_syn(element_type)?;

    Ok(match size {
        crate::type_mapper::RustConstGeneric::Literal(n) => {
            let size_lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { [#element; #size_lit] }
        }
        crate::type_mapper::RustConstGeneric::Parameter(name) => {
            let param_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { [#element; #param_ident] }
        }
        crate::type_mapper::RustConstGeneric::Expression(expr) => {
            let expr_tokens: proc_macro2::TokenStream = expr.parse().unwrap_or_else(|_| {
                quote! { /* invalid const expression */ }
            });
            parse_quote! { [#element; #expr_tokens] }
        }
    })
}

pub fn rust_type_to_syn(rust_type: &crate::type_mapper::RustType) -> Result<syn::Type> {
    use crate::type_mapper::RustType;

    Ok(match rust_type {
        RustType::Primitive(p) => {
            let ident = syn::Ident::new(p.to_rust_string(), proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::String => parse_quote! { String },
        RustType::Str { lifetime } => str_type_to_syn(lifetime),
        RustType::Cow { lifetime } => {
            let lt_ident = syn::Lifetime::new(lifetime, proc_macro2::Span::call_site());
            parse_quote! { Cow<#lt_ident, str> }
        }
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Vec<#inner_ty> }
        }
        RustType::HashMap(k, v) => {
            let key_ty = rust_type_to_syn(k)?;
            let val_ty = rust_type_to_syn(v)?;
            parse_quote! { HashMap<#key_ty, #val_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { Option<#inner_ty> }
        }
        RustType::Result(ok, err) => {
            let ok_ty = rust_type_to_syn(ok)?;
            let err_ty = rust_type_to_syn(err)?;
            parse_quote! { Result<#ok_ty, #err_ty> }
        }
        RustType::Reference {
            lifetime,
            mutable,
            inner,
        } => reference_type_to_syn(lifetime, *mutable, inner)?,
        RustType::Tuple(types) => {
            let tys: Vec<_> = types
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { (#(#tys),*) }
        }
        RustType::Unit => parse_quote! { () },
        RustType::Custom(name) => {
            let ty: syn::Type = syn::parse_str(name)?;
            ty
        }
        RustType::Unsupported(reason) => bail!("Unsupported Rust type: {}", reason),
        RustType::TypeParam(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Generic { base, params } => {
            let base_ident = syn::Ident::new(base, proc_macro2::Span::call_site());
            let param_types: Vec<_> = params
                .iter()
                .map(rust_type_to_syn)
                .collect::<Result<Vec<_>>>()?;
            parse_quote! { #base_ident<#(#param_types),*> }
        }
        RustType::Enum { name, .. } => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            parse_quote! { #ident }
        }
        RustType::Array { element_type, size } => array_type_to_syn(element_type, size)?,
        RustType::HashSet(inner) => {
            let inner_ty = rust_type_to_syn(inner)?;
            parse_quote! { HashSet<#inner_ty> }
        }
    })
}

/// Format Rust code using basic prettification
/// Note: This is a simple formatter for V1. rustfmt integration planned for V2.
fn format_rust_code(code: String) -> String {
    code.replace(" ; ", ";\n    ")
        .replace(" { ", " {\n    ")
        .replace(" } ", "\n}\n")
        .replace("} ;", "};")
        .replace(
            "use std :: collections :: HashMap ;",
            "use std::collections::HashMap;",
        )
        // Fix method call spacing
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace(" )", ")")
        // Fix specific common patterns
        .replace(".len ()", ".len()")
        .replace(".push (", ".push(")
        .replace(".insert (", ".insert(")
        .replace(".get (", ".get(")
        .replace(".contains_key (", ".contains_key(")
        .replace(".to_string ()", ".to_string()")
        // Fix spacing around operators in some contexts
        .replace(" ::", "::")
        .replace(":: ", "::")
        // Fix attribute spacing
        .replace("# [", "#[")
        // Fix type annotations
        .replace(" : ", ": ")
        // Fix parameter spacing
        .replace(" , ", ", ")
        // Fix assignment operator spacing issues
        .replace("=(", " = (")
        .replace("= (", " = (")
        .replace("  =", " =") // Fix multiple spaces before =
        .replace("   =", " =") // Fix even more spaces
        // Fix generic type spacing
        .replace("Vec < ", "Vec<")
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace("> ", ">")
        .replace("< ", "<")
        .replace(" >", ">") // Fix trailing space before closing bracket
        // Fix return type spacing
        .replace("->", " -> ")
        .replace(" ->  ", " -> ")
        .replace(" ->   ", " -> ")
        // Fix range spacing
        .replace(" .. ", "..")
        .replace(" ..", "..")
        .replace(".. ", "..")
        // Fix 'in' keyword spacing
        .replace("in(", "in (")
        .replace(";\n    }", "\n}")
}

/// Updates the import needs based on the rust type being used
fn update_import_needs(ctx: &mut CodeGenContext, rust_type: &crate::type_mapper::RustType) {
    match rust_type {
        crate::type_mapper::RustType::HashMap(_, _) => ctx.needs_hashmap = true,
        crate::type_mapper::RustType::Cow { .. } => ctx.needs_cow = true,
        crate::type_mapper::RustType::Custom(name) => {
            if name.contains("FnvHashMap") {
                ctx.needs_fnv_hashmap = true;
            } else if name.contains("AHashMap") {
                ctx.needs_ahash_hashmap = true;
            } else if name.contains("Arc<") {
                ctx.needs_arc = true;
            } else if name.contains("Rc<") {
                ctx.needs_rc = true;
            } else if name.contains("HashMap<")
                && !name.contains("FnvHashMap")
                && !name.contains("AHashMap")
            {
                ctx.needs_hashmap = true;
            }
        }
        crate::type_mapper::RustType::Reference { inner, .. } => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Vec(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Option(inner) => {
            update_import_needs(ctx, inner);
        }
        crate::type_mapper::RustType::Result(ok, err) => {
            update_import_needs(ctx, ok);
            update_import_needs(ctx, err);
        }
        crate::type_mapper::RustType::Tuple(types) => {
            for t in types {
                update_import_needs(ctx, t);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::TypeMapper;
    use depyler_annotations::TranspilationAnnotations;

    fn create_test_context() -> CodeGenContext<'static> {
        // This is a bit of a hack for testing - in real use, the TypeMapper would have a longer lifetime
        let type_mapper: &'static TypeMapper = Box::leak(Box::new(TypeMapper::default()));
        CodeGenContext {
            type_mapper,
            annotation_aware_mapper: AnnotationAwareTypeMapper::with_base_mapper(
                type_mapper.clone(),
            ),
            string_optimizer: StringOptimizer::new(),
            union_enum_generator: crate::union_enum_gen::UnionEnumGenerator::new(),
            generated_enums: Vec::new(),
            needs_hashmap: false,
            needs_hashset: false,
            needs_fnv_hashmap: false,
            needs_ahash_hashmap: false,
            needs_arc: false,
            needs_rc: false,
            needs_cow: false,
            declared_vars: vec![HashSet::new()],
            current_function_can_fail: false,
            current_return_type: None,
            module_mapper: crate::module_mapper::ModuleMapper::new(),
            imported_modules: std::collections::HashMap::new(),
            imported_items: std::collections::HashMap::new(),
            mutable_vars: HashSet::new(),
            needs_zerodivisionerror: false,
            needs_indexerror: false,
            is_classmethod: false,
            in_generator: false,
            generator_state_vars: HashSet::new(),
        }
    }

    #[test]
    fn test_simple_function_generation() {
        let func = HirFunction {
            name: "add".to_string(),
            params: vec![
                HirParam::new("a".to_string(), Type::Int),
                HirParam::new("b".to_string(), Type::Int),
            ]
            .into(),
            ret_type: Type::Int,
            body: vec![HirStmt::Return(Some(HirExpr::Binary {
                op: BinOp::Add,
                left: Box::new(HirExpr::Var("a".to_string())),
                right: Box::new(HirExpr::Var("b".to_string())),
            }))],
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        };

        let mut ctx = create_test_context();
        let tokens = func.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("pub fn add"));
        assert!(code.contains("i32"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_control_flow_generation() {
        let if_stmt = HirStmt::If {
            condition: HirExpr::Binary {
                op: BinOp::Gt,
                left: Box::new(HirExpr::Var("x".to_string())),
                right: Box::new(HirExpr::Literal(Literal::Int(0))),
            },
            then_body: vec![HirStmt::Return(Some(HirExpr::Literal(Literal::String(
                "positive".to_string(),
            ))))],
            else_body: Some(vec![HirStmt::Return(Some(HirExpr::Literal(
                Literal::String("negative".to_string()),
            )))]),
        };

        let mut ctx = create_test_context();
        let tokens = if_stmt.to_rust_tokens(&mut ctx).unwrap();
        let code = tokens.to_string();

        assert!(code.contains("if"));
        assert!(code.contains("else"));
        assert!(code.contains("return"));
    }

    #[test]
    fn test_list_generation() {
        // Test literal array generation
        let list_expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(3)),
        ]);

        let mut ctx = create_test_context();
        let expr = list_expr.to_rust_expr(&mut ctx).unwrap();
        let code = quote! { #expr }.to_string();

        // Small literal lists should generate arrays
        assert!(code.contains("[") && code.contains("]"));
        assert!(code.contains("1"));
        assert!(code.contains("2"));
        assert!(code.contains("3"));

        // Test non-literal list still uses vec!
        let var_list = HirExpr::List(vec![
            HirExpr::Var("x".to_string()),
            HirExpr::Var("y".to_string()),
        ]);

        let expr2 = var_list.to_rust_expr(&mut ctx).unwrap();
        let code2 = quote! { #expr2 }.to_string();
        assert!(code2.contains("vec !"));
    }

    #[test]
    fn test_dict_generation_sets_needs_hashmap() {
        let dict_expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(42)),
        )]);

        let mut ctx = create_test_context();
        assert!(!ctx.needs_hashmap);

        let _ = dict_expr.to_rust_expr(&mut ctx).unwrap();

        assert!(ctx.needs_hashmap);
    }

    #[test]
    fn test_binary_operations() {
        let ops = vec![
            (BinOp::Add, "+"),
            (BinOp::Sub, "-"),
            (BinOp::Mul, "*"),
            (BinOp::Eq, "=="),
            (BinOp::Lt, "<"),
        ];

        for (op, expected) in ops {
            let result = convert_binop(op).unwrap();
            assert_eq!(quote! { #result }.to_string(), expected);
        }
    }

    #[test]
    fn test_unsupported_operators() {
        assert!(convert_binop(BinOp::Pow).is_err());
        assert!(convert_binop(BinOp::In).is_err());
        assert!(convert_binop(BinOp::NotIn).is_err());
    }
}
