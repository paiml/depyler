//! Generator support and code generation
//!
//! This module handles Python generator functions, converting them to
//! Rust Iterator implementations with state structs.

use crate::generator_state::GeneratorStateInfo;
use crate::generator_yield_analysis::YieldAnalysis;
use crate::hir::{HirExpr, HirFunction, HirStmt, Literal, Type};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::keywords::safe_ident; // DEPYLER-0562: Escape keywords like 'match'
use crate::rust_gen::type_gen::rust_type_to_syn;
use anyhow::Result;
use quote::quote;

/// Generate struct fields for generator state variables
///
/// Creates field declarations for variables that need to persist across yields.
///
/// # Complexity
/// 3 (iter + map + collect)
fn generate_state_fields(
    state_info: &GeneratorStateInfo,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    state_info
        .state_variables
        .iter()
        .map(|var| {
            // DEPYLER-0562: Use safe_ident to escape keywords like 'match'
            let field_name = safe_ident(&var.name);
            let rust_type = ctx.type_mapper.map_type(&var.ty);
            // DEPYLER-0188: Box impl Trait types for struct fields
            let field_rust_type = box_impl_trait_for_field(&rust_type);
            // DEPYLER-0772: Use concrete fallback for TypeParam in struct fields
            let concrete_type = concretize_type_param_for_struct(&field_rust_type);
            let field_type = rust_type_to_syn(&concrete_type)?;
            Ok(quote! { #field_name: #field_type })
        })
        .collect()
}

/// DEPYLER-0772: Convert TypeParam to concrete type for struct fields
///
/// Generator state structs cannot use bare type parameters like `T` without
/// declaring them. When a state variable has unknown type that maps to `T`,
/// we use `i32` as a safe default since most generator variables are counters
/// or indices.
///
/// # Complexity: 2 (match + clone)
#[inline]
fn concretize_type_param_for_struct(
    rust_type: &crate::type_mapper::RustType,
) -> crate::type_mapper::RustType {
    use crate::type_mapper::{PrimitiveType, RustType};

    match rust_type {
        // DEPYLER-0772: Replace bare `T` with concrete `i32`
        // Most generator state variables are loop counters or accumulator values
        RustType::TypeParam(_) => RustType::Primitive(PrimitiveType::I32),
        other => other.clone(),
    }
}

/// Convert impl Trait types to boxed trait objects for struct fields
///
/// DEPYLER-0188: Rust doesn't allow `impl Trait` in struct field positions,
/// so we convert them to `Box<dyn Trait>` for dynamic dispatch.
///
/// # Complexity
/// 2 (string ops)
fn box_impl_trait_for_field(
    rust_type: &crate::type_mapper::RustType,
) -> crate::type_mapper::RustType {
    use crate::type_mapper::RustType;

    match rust_type {
        RustType::Custom(s) if s.starts_with("impl Iterator") => {
            // impl Iterator<Item=T> -> Box<dyn Iterator<Item=T>>
            let boxed = s.replace("impl Iterator", "Box<dyn Iterator");
            RustType::Custom(format!("{}>", boxed))
        }
        RustType::Custom(s) if s.starts_with("impl IntoIterator") => {
            // impl IntoIterator<Item=T> -> Box<dyn IntoIterator<Item=T>>
            let boxed = s.replace("impl IntoIterator", "Box<dyn IntoIterator");
            RustType::Custom(format!("{}>", boxed))
        }
        other => other.clone(),
    }
}

/// Generate struct fields for captured parameters
///
/// Creates field declarations for function parameters that are captured
/// in the generator state.
///
/// # Complexity
/// 4 (iter + filter + map + collect)
fn generate_param_fields(
    func: &HirFunction,
    state_info: &GeneratorStateInfo,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    func.params
        .iter()
        .filter(|p| state_info.captured_params.contains(&p.name))
        .map(|param| {
            // DEPYLER-0562: Use safe_ident to escape keywords like 'match'
            let field_name = safe_ident(&param.name);
            let rust_type = ctx.type_mapper.map_type(&param.ty);
            // DEPYLER-0188: Box impl Trait types for struct fields
            let field_rust_type = box_impl_trait_for_field(&rust_type);
            // DEPYLER-0772: Use concrete fallback for TypeParam in struct fields
            let concrete_type = concretize_type_param_for_struct(&field_rust_type);
            let field_type = rust_type_to_syn(&concrete_type)?;
            Ok(quote! { #field_name: #field_type })
        })
        .collect()
}

/// Extract the Item type for the Iterator from generator return type
///
/// DEPYLER-0263 FIX: Infer yield type from yield analysis, not func.ret_type.
/// func.ret_type is often Type::Unknown for generators, which maps to DynamicType.
/// Instead, we analyze the actual yield expressions to infer the concrete type.
///
/// # Complexity
/// 3 (analyze + map + convert)
#[inline]
fn extract_generator_item_type(
    func: &HirFunction,
    yield_analysis: &YieldAnalysis,
    ctx: &CodeGenContext,
) -> Result<syn::Type> {
    // DEPYLER-0495: Extract element type from Iterator[T] return type
    // If return type is Iterator[int], we need item type = int (not Iterator<int>)
    let element_type = match &func.ret_type {
        // Iterator[T] -> extract T
        Type::Generic { base, params } if base == "Iterator" && params.len() == 1 => {
            params[0].clone()
        }
        // Generator[YieldType, SendType, ReturnType] -> extract YieldType
        Type::Generic { base, params } if base == "Generator" && !params.is_empty() => {
            params[0].clone()
        }
        // Unknown -> infer from yield expression (DEPYLER-0263)
        Type::Unknown if !yield_analysis.yield_points.is_empty() => {
            infer_yield_type(&yield_analysis.yield_points[0].yield_expr)
        }
        // Other types -> use as-is (backwards compatibility)
        other => other.clone(),
    };

    let rust_element_type = ctx.type_mapper.map_type(&element_type);
    rust_type_to_syn(&rust_element_type)
}

/// Infer type from a yield expression
///
/// DEPYLER-0769: Handle tuple yields and other composite expressions
/// to prevent undefined type `T` in generated code.
///
/// # Complexity: 4 (nested match + recursion for tuples)
#[inline]
fn infer_yield_type(expr: &HirExpr) -> Type {
    match expr {
        HirExpr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bytes(_) => Type::Custom("bytes".to_string()),
            Literal::Bool(_) => Type::Bool,
            Literal::None => Type::None,
        },
        // DEPYLER-0769: Handle tuple yields like `yield a, b`
        // Recursively infer types for each element
        HirExpr::Tuple(elems) => {
            let elem_types: Vec<Type> = elems.iter().map(infer_yield_type).collect();
            Type::Tuple(elem_types)
        }
        // DEPYLER-0769: Default to String for variables without type info
        // String is safer than Int as it can represent most yield patterns
        HirExpr::Var(_) => Type::String,
        // For other expressions (calls, binary ops, etc.), default to String
        // which is a safer fallback than Unknown (which maps to undefined T)
        _ => Type::String,
    }
}

/// Generate field initializers for state variables (with default values)
///
/// Creates initialization expressions like `field_name: 0` or `field_name: false`.
///
/// # Complexity
/// 3 (iter + map + collect)
fn generate_state_initializers(state_info: &GeneratorStateInfo) -> Vec<proc_macro2::TokenStream> {
    state_info
        .state_variables
        .iter()
        .map(|var| {
            // DEPYLER-0562: Use safe_ident to escape keywords like 'match'
            let field_name = safe_ident(&var.name);
            // Initialize with type-appropriate default (0 for int, false for bool, etc.)
            let default_value = get_default_value_for_type(&var.ty);
            quote! { #field_name: #default_value }
        })
        .collect()
}

/// Generate field initializers for captured parameters
///
/// Creates initialization expressions like `field_name: field_name` to capture
/// the parameter value.
///
/// # Complexity
/// 4 (iter + filter + map + collect)
fn generate_param_initializers(
    func: &HirFunction,
    state_info: &GeneratorStateInfo,
) -> Vec<proc_macro2::TokenStream> {
    func.params
        .iter()
        .filter(|p| state_info.captured_params.contains(&p.name))
        .map(|param| {
            // DEPYLER-0562: Use safe_ident to escape keywords like 'match'
            let field_name = safe_ident(&param.name);

            // DEPYLER-0498: Dereference Option parameters (&Option<T> -> Option<T>)
            // DEPYLER-1078: Clone List/Dict/Set parameters (&Vec<T> -> Vec<T>)
            // DEPYLER-1079: Box Iterator/Generator parameters for struct fields
            // Five-Whys Root Cause: Parameters are borrowed but struct fields are owned
            // Solution: Dereference, clone, or box based on type
            let init_value = match &param.ty {
                // Option<T> implements Copy for Copy inner types - dereference
                Type::Optional(_) => quote! { *#field_name },
                // List/Vec - clone the reference to get owned value
                Type::List(_) => quote! { #field_name.clone() },
                // Dict/HashMap - clone the reference
                Type::Dict(_, _) => quote! { #field_name.clone() },
                // Set - clone the reference
                Type::Set(_) => quote! { #field_name.clone() },
                // String - clone the reference
                Type::String => quote! { #field_name.clone() },
                // DEPYLER-1082: Box impl Iterator/Generator parameters for struct fields
                // Struct fields are typed as Box<dyn Iterator> (via box_impl_trait_for_field)
                // so we must wrap the impl Trait parameter with Box::new() for type erasure
                Type::Generic { base, .. }
                    if base == "Iterator" || base == "Generator" || base == "Iterable" =>
                {
                    quote! { Box::new(#field_name) as _ }
                }
                // Other types - direct assignment (primitives implement Copy)
                _ => quote! { #field_name },
            };

            quote! { #field_name: #init_value }
        })
        .collect()
}

/// Get default value expression for Int type
///
/// # Complexity: 1
#[inline]
fn default_int() -> proc_macro2::TokenStream {
    quote! { 0 }
}

/// Get default value expression for Float type
///
/// # Complexity: 1
#[inline]
fn default_float() -> proc_macro2::TokenStream {
    quote! { 0.0 }
}

/// Get default value expression for Bool type
///
/// # Complexity: 1
#[inline]
fn default_bool() -> proc_macro2::TokenStream {
    quote! { false }
}

/// Get default value expression for String type
///
/// # Complexity: 1
#[inline]
fn default_string() -> proc_macro2::TokenStream {
    quote! { String::new() }
}

/// Get default value expression for other types
///
/// # Complexity: 1
#[inline]
fn default_generic() -> proc_macro2::TokenStream {
    quote! { Default::default() }
}

/// Get default value expression for a type
///
/// Returns appropriate default value based on HIR type.
///
/// # Complexity
/// 6 (match with 5 arms)
#[inline]
fn get_default_value_for_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Int => default_int(),
        Type::Float => default_float(),
        Type::Bool => default_bool(),
        Type::String => default_string(),
        _ => default_generic(),
    }
}

/// Generate state struct name by converting snake_case to PascalCase
///
/// DEPYLER-0259: Converts snake_case to PascalCase properly
/// Examples: count_up → CountUpState, counter → CounterState
///
/// # Complexity: 6 (within ≤10 target)
#[inline]
fn generate_state_struct_name(name: &syn::Ident) -> syn::Ident {
    let name_str = name.to_string();

    // DEPYLER-0259 FIX: Convert snake_case to PascalCase properly
    let pascal_case = name_str
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>();

    let state_struct_name = format!("{}State", pascal_case);
    syn::Ident::new(&state_struct_name, name.span())
}

/// Populate generator state variables in context
///
/// # Complexity: 4 (clear + 2 loops + iterator check)
#[inline]
fn populate_generator_state_vars(
    ctx: &mut CodeGenContext,
    state_info: &GeneratorStateInfo,
    func: &HirFunction,
) {
    ctx.generator_state_vars.clear();
    ctx.generator_iterator_state_vars.clear();
    for var in &state_info.state_variables {
        ctx.generator_state_vars.insert(var.name.clone());
    }
    for param in &state_info.captured_params {
        ctx.generator_state_vars.insert(param.clone());
    }
    // DEPYLER-1082: Track which state vars have Iterator/Generator type
    // These need while-let iteration because Box<dyn Iterator> doesn't impl IntoIterator
    for param in &func.params {
        if matches!(&param.ty, Type::Generic { base, .. }
            if base == "Iterator" || base == "Generator" || base == "Iterable")
        {
            ctx.generator_iterator_state_vars.insert(param.name.clone());
        }
    }
}

/// Generate generator body statements with proper context flags
///
/// # Complexity: 4 (set flag + collect + clear flag + clear vars)
#[inline]
fn generate_generator_body(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    use crate::rust_gen::RustCodeGen;

    ctx.in_generator = true;
    let generator_body_stmts: Vec<_> = func
        .body
        .iter()
        .map(|stmt| stmt.to_rust_tokens(ctx))
        .collect::<Result<Vec<_>>>()?;
    ctx.in_generator = false;
    ctx.generator_state_vars.clear();
    ctx.generator_iterator_state_vars.clear();

    Ok(generator_body_stmts)
}

/// Convert HirExpr to syn::Expr for code generation
///
/// # Complexity: 1 (direct conversion)
#[inline]
fn hir_expr_to_syn(expr: &HirExpr, ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    // Use the ToRustExpr trait to convert HIR expression to syn::Expr
    expr.to_rust_expr(ctx)
}

/// Generate multi-state match arms for sequential yields
///
/// DEPYLER-0262 Phase 3A: Transforms sequential yield points into proper state machine.
/// Each yield becomes a separate state with resumption at the next statement.
///
/// # Complexity: 5 (iterate yields + generate arms)
#[inline]
fn generate_simple_multi_state_match(
    yield_analysis: &YieldAnalysis,
    _func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0263: Set generator context flag for proper variable scoping
    ctx.in_generator = true;

    let mut match_arms = Vec::new();

    // State 0: Initial state - execute up to first yield
    if let Some(first_yield) = yield_analysis.yield_points.first() {
        let yield_value = hir_expr_to_syn(&first_yield.yield_expr, ctx)?;
        let next_state = first_yield.state_id;

        match_arms.push(quote! {
            0 => {
                self.state = #next_state;
                return Some(#yield_value);
            }
        });
    }

    // Generate state for each yield point (states 1..N)
    for (idx, yield_point) in yield_analysis.yield_points.iter().enumerate() {
        let current_state = yield_point.state_id;

        // If there's a next yield, transition to it; otherwise go to terminal state
        if let Some(next_yield) = yield_analysis.yield_points.get(idx + 1) {
            let yield_value = hir_expr_to_syn(&next_yield.yield_expr, ctx)?;
            let next_state = next_yield.state_id;

            match_arms.push(quote! {
                #current_state => {
                    self.state = #next_state;
                    return Some(#yield_value);
                }
            });
        } else {
            // Last yield - transition to terminal state
            match_arms.push(quote! {
                #current_state => {
                    self.state = #current_state + 1;
                    None
                }
            });
        }
    }

    // Terminal state: generator exhausted
    match_arms.push(quote! {
        _ => None
    });

    // Clear generator context flag
    ctx.in_generator = false;

    Ok(quote! {
        match self.state {
            #(#match_arms)*
        }
    })
}

/// Generate loop with yield transformation
///
/// DEPYLER-0262 Phase 3B: Transforms simple loops with single yield into state machines.
/// Handles pattern: `while condition: yield value; increment`
///
/// Strategy:
/// - Extract loop from function body
/// - Generate initialization code (statements before loop)
/// - Generate loop state that checks condition and yields properly
///
/// # Complexity: 8 (within ≤10 target)
#[inline]
fn generate_simple_loop_with_yield(
    func: &HirFunction,
    yield_analysis: &YieldAnalysis,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-0263: Set generator context flag for proper variable scoping
    ctx.in_generator = true;

    // Find the loop statement in the function body
    let loop_info = extract_loop_info(func)?;

    // Generate initialization statements (before the loop)
    let init_stmts = generate_loop_init_stmts(&loop_info.pre_loop_stmts, ctx)?;

    // Get the yield expression
    let yield_point = &yield_analysis.yield_points[0];
    let yield_value = hir_expr_to_syn(&yield_point.yield_expr, ctx)?;

    // Extract loop condition
    let loop_condition = hir_expr_to_syn(&loop_info.condition, ctx)?;

    // Generate loop body statements (updates, increments)
    let loop_body_stmts = generate_loop_body_stmts(&loop_info.body_stmts, ctx)?;

    // Clear generator context flag
    ctx.in_generator = false;

    // Generate the state machine
    Ok(quote! {
        match self.state {
            0 => {
                // Initialize loop variables
                #(#init_stmts)*
                // Transition to loop state
                self.state = 1;
                // Check condition immediately
                self.next()
            }
            1 => {
                // Check loop condition
                if #loop_condition {
                    // Yield the value
                    let result = #yield_value;
                    // Execute loop body (increments, updates)
                    #(#loop_body_stmts)*
                    // Stay in state 1 (continue looping)
                    return Some(result);
                } else {
                    // Condition false - exit loop
                    self.state = 2;
                    None
                }
            }
            _ => None
        }
    })
}

/// Extract loop information from function body
///
/// # Complexity: 5
#[inline]
fn extract_loop_info(func: &HirFunction) -> Result<LoopInfo> {
    // Find the While statement in the body
    let mut pre_loop_stmts = Vec::new();
    let mut loop_stmt = None;

    for stmt in &func.body {
        match stmt {
            HirStmt::While { condition, body } => {
                loop_stmt = Some((condition.clone(), body.clone()));
                break;
            }
            _ => {
                // Statements before the loop (initialization)
                pre_loop_stmts.push(stmt.clone());
            }
        }
    }

    let (condition, body) =
        loop_stmt.ok_or_else(|| anyhow::anyhow!("No while loop found in generator function"))?;

    // Separate yield statement from other body statements
    let mut body_stmts = Vec::new();
    for stmt in &body {
        // Skip the yield statement itself - we'll handle it separately
        if !matches!(stmt, HirStmt::Expr(HirExpr::Yield { .. })) {
            body_stmts.push(stmt.clone());
        }
    }

    Ok(LoopInfo {
        pre_loop_stmts,
        condition,
        body_stmts,
    })
}

/// Loop information structure
struct LoopInfo {
    pre_loop_stmts: Vec<HirStmt>,
    condition: HirExpr,
    body_stmts: Vec<HirStmt>,
}

/// Generate initialization statements before loop
///
/// # Complexity: 2
#[inline]
fn generate_loop_init_stmts(
    stmts: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    use crate::rust_gen::RustCodeGen;
    stmts.iter().map(|stmt| stmt.to_rust_tokens(ctx)).collect()
}

/// Generate loop body statements (after yield)
///
/// # Complexity: 2
#[inline]
fn generate_loop_body_stmts(
    stmts: &[HirStmt],
    ctx: &mut CodeGenContext,
) -> Result<Vec<proc_macro2::TokenStream>> {
    use crate::rust_gen::RustCodeGen;
    stmts.iter().map(|stmt| stmt.to_rust_tokens(ctx)).collect()
}

/// Generate complete generator function with state struct and Iterator impl
///
/// This is the main entry point for generator code generation. It:
/// 1. Analyzes generator state requirements
/// 2. Creates a state struct with captured variables
/// 3. Generates a constructor function
/// 4. Implements Iterator with state machine logic
///
/// # Arguments
/// * `func` - The HIR function to generate
/// * `name` - Function name identifier
/// * `generic_params` - Generic parameters token stream
/// * `where_clause` - Where clause token stream
/// * `params` - Parameter declarations
/// * `attrs` - Function attributes
/// * `rust_ret_type` - Return type
/// * `ctx` - Code generation context
///
/// # Returns
/// Complete generator implementation including state struct and Iterator impl
///
/// # Complexity
/// 5 (delegated to helper functions)
#[inline]
#[allow(clippy::too_many_arguments)] // Generator needs all metadata for complex transformation
pub fn codegen_generator_function(
    func: &HirFunction,
    name: &syn::Ident,
    generic_params: &proc_macro2::TokenStream,
    where_clause: &proc_macro2::TokenStream,
    params: &[proc_macro2::TokenStream],
    attrs: &[proc_macro2::TokenStream],
    _rust_ret_type: &crate::type_mapper::RustType,
    ctx: &mut CodeGenContext,
) -> Result<proc_macro2::TokenStream> {
    // DEPYLER-1082: Add 'static bound to impl Iterator params that will be boxed
    // When impl Iterator params are stored in struct fields as Box<dyn Iterator>,
    // they require 'static lifetime for the unsized coercion
    let params: Vec<proc_macro2::TokenStream> = params
        .iter()
        .map(|p| {
            let p_str = p.to_string();
            // Check if this param is impl Iterator without 'static
            if p_str.contains("impl Iterator") && !p_str.contains("'static") {
                // DEPYLER-1082: Add 'static bound for boxed iterator params
                // impl Iterator<Item=T> -> impl Iterator<Item=T> + 'static
                let modified = if p_str.contains(">") {
                    p_str.replace(">", "> + 'static")
                } else {
                    p_str.clone()
                };
                syn::parse_str(&modified).unwrap_or_else(|_| p.clone())
            } else {
                p.clone()
            }
        })
        .collect();

    // Analyze generator state requirements
    let state_info = GeneratorStateInfo::analyze(func);

    // DEPYLER-0262 Phase 2: Analyze yield points for state machine transformation
    let yield_analysis = YieldAnalysis::analyze(func);

    // DEPYLER-0262 Phase 3A: Check if we can use simple multi-state transformation
    let use_simple_multi_state =
        yield_analysis.has_yields() && yield_analysis.yield_points.iter().all(|yp| yp.depth == 0);

    // Generate state struct name
    let state_ident = generate_state_struct_name(name);

    // Build state struct fields from analysis
    let state_fields = generate_state_fields(&state_info, ctx)?;
    let param_fields = generate_param_fields(func, &state_info, ctx)?;
    let all_fields = [state_fields, param_fields].concat();

    // Build field initializers
    let state_inits = generate_state_initializers(&state_info);
    let param_inits = generate_param_initializers(func, &state_info);
    let all_inits = [state_inits, param_inits].concat();

    // Generate state machine field
    let state_machine_field = quote! {
        state: usize
    };

    // DEPYLER-0263: Extract yield value type, inferring from yield expressions if needed
    let item_type = extract_generator_item_type(func, &yield_analysis, ctx)?;

    // DEPYLER-0494 FIX: Populate generator state variables BEFORE generating state machine
    // This ensures ctx.generator_state_vars is available when codegen_assign_tuple() is called
    populate_generator_state_vars(ctx, &state_info, func);

    // DEPYLER-0262 Phase 3B: Check if we have simple loop with yield pattern
    let has_while_loop = func
        .body
        .iter()
        .any(|stmt| matches!(stmt, HirStmt::While { .. }));
    let has_loop_yields =
        yield_analysis.has_yields() && yield_analysis.yield_points.iter().any(|yp| yp.depth > 0);

    // Generate state machine implementation based on yield analysis
    let state_machine_impl = if use_simple_multi_state {
        // DEPYLER-0262 Phase 3A: Multi-state transformation for sequential yields
        generate_simple_multi_state_match(&yield_analysis, func, ctx)?
    } else if has_while_loop && has_loop_yields && yield_analysis.yield_points.len() == 1 {
        // DEPYLER-0262 Phase 3B: Simple loop with single yield pattern
        generate_simple_loop_with_yield(func, &yield_analysis, ctx)?
    } else {
        // Fallback: Single-state implementation (for complex cases or no yields)
        let generator_body_stmts = generate_generator_body(func, ctx)?;
        quote! {
            match self.state {
                0 => {
                    self.state = 1;
                    // Execute generator body with early-exit semantics
                    #(#generator_body_stmts)*
                    None
                }
                _ => None
            }
        }
    };

    // DEPYLER-1082: Check if we need manual Debug impl (Box<dyn Iterator> doesn't impl Debug)
    let has_iterator_fields = func.params.iter().any(|p| {
        matches!(&p.ty, Type::Generic { base, .. }
            if base == "Iterator" || base == "Generator" || base == "Iterable")
    });

    // Generate the complete generator implementation
    let debug_impl = if has_iterator_fields {
        // DEPYLER-1082: Manual Debug impl for structs with Box<dyn Iterator> fields
        // Box<dyn Iterator> doesn't implement Debug, so we print a placeholder
        quote! {
            impl std::fmt::Debug for #state_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#state_ident))
                        .field("state", &self.state)
                        .finish_non_exhaustive()
                }
            }
        }
    } else {
        // No iterator fields - can derive Debug normally
        quote! {}
    };

    let derive_debug = if has_iterator_fields {
        quote! {} // No derive, use manual impl
    } else {
        quote! { #[derive(Debug)] }
    };

    Ok(quote! {
        #(#attrs)*
        #[doc = " Generator state struct"]
        #derive_debug
        struct #state_ident {
            #state_machine_field,
            #(#all_fields),*
        }

        #debug_impl

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
                #state_machine_impl
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_mapper::{PrimitiveType, RustType};

    // ============================================================
    // generate_state_struct_name tests
    // ============================================================

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0259_snake_case_to_pascal_case_naming() {
        let input_name = syn::Ident::new("count_up", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "CountUpState");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0259_single_word_naming() {
        let input_name = syn::Ident::new("counter", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "CounterState");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_depyler_0259_multiple_words_naming() {
        let input_name = syn::Ident::new(
            "fibonacci_generator_with_memo",
            proc_macro2::Span::call_site(),
        );
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "FibonacciGeneratorWithMemoState");
    }

    #[test]
    fn test_generate_state_struct_name_empty_parts() {
        // Test with double underscore (edge case)
        let input_name = syn::Ident::new("a__b", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        // Empty parts become empty strings in PascalCase
        assert_eq!(result.to_string(), "ABState");
    }

    #[test]
    fn test_generate_state_struct_name_leading_underscore() {
        let input_name = syn::Ident::new("_private_gen", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "PrivateGenState");
    }

    #[test]
    fn test_generate_state_struct_name_all_caps() {
        let input_name = syn::Ident::new("HTTP_GEN", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        // DEPYLER capitalizes first letter, keeps rest
        assert_eq!(result.to_string(), "HTTPGENState");
    }

    // ============================================================
    // concretize_type_param_for_struct tests
    // ============================================================

    #[test]
    fn test_concretize_type_param_converts_to_i32() {
        let type_param = RustType::TypeParam("T".to_string());
        let result = concretize_type_param_for_struct(&type_param);
        assert!(matches!(result, RustType::Primitive(PrimitiveType::I32)));
    }

    #[test]
    fn test_concretize_type_param_preserves_primitives() {
        let int_type = RustType::Primitive(PrimitiveType::I64);
        let result = concretize_type_param_for_struct(&int_type);
        assert!(matches!(result, RustType::Primitive(PrimitiveType::I64)));
    }

    #[test]
    fn test_concretize_type_param_preserves_string() {
        let string_type = RustType::String;
        let result = concretize_type_param_for_struct(&string_type);
        assert!(matches!(result, RustType::String));
    }

    #[test]
    fn test_concretize_type_param_preserves_vec() {
        let vec_type = RustType::Vec(Box::new(RustType::Primitive(PrimitiveType::I32)));
        let result = concretize_type_param_for_struct(&vec_type);
        assert!(matches!(result, RustType::Vec(_)));
    }

    #[test]
    fn test_concretize_type_param_preserves_option() {
        let opt_type = RustType::Option(Box::new(RustType::String));
        let result = concretize_type_param_for_struct(&opt_type);
        assert!(matches!(result, RustType::Option(_)));
    }

    #[test]
    fn test_concretize_type_param_preserves_custom() {
        let custom_type = RustType::Custom("MyType".to_string());
        let result = concretize_type_param_for_struct(&custom_type);
        assert!(matches!(result, RustType::Custom(s) if s == "MyType"));
    }

    // ============================================================
    // box_impl_trait_for_field tests
    // ============================================================

    #[test]
    fn test_box_impl_iterator_trait() {
        let impl_iter = RustType::Custom("impl Iterator<Item=i32>".to_string());
        let result = box_impl_trait_for_field(&impl_iter);
        match result {
            RustType::Custom(s) => assert_eq!(s, "Box<dyn Iterator<Item=i32>>"),
            _ => panic!("Expected Custom type"),
        }
    }

    #[test]
    fn test_box_impl_into_iterator_trait() {
        let impl_into_iter = RustType::Custom("impl IntoIterator<Item=String>".to_string());
        let result = box_impl_trait_for_field(&impl_into_iter);
        match result {
            RustType::Custom(s) => assert_eq!(s, "Box<dyn IntoIterator<Item=String>>"),
            _ => panic!("Expected Custom type"),
        }
    }

    #[test]
    fn test_box_impl_trait_preserves_non_impl() {
        let regular_type = RustType::Custom("Vec<i32>".to_string());
        let result = box_impl_trait_for_field(&regular_type);
        match result {
            RustType::Custom(s) => assert_eq!(s, "Vec<i32>"),
            _ => panic!("Expected Custom type"),
        }
    }

    #[test]
    fn test_box_impl_trait_preserves_primitives() {
        let int_type = RustType::Primitive(PrimitiveType::I32);
        let result = box_impl_trait_for_field(&int_type);
        assert!(matches!(result, RustType::Primitive(PrimitiveType::I32)));
    }

    #[test]
    fn test_box_impl_trait_preserves_string() {
        let string_type = RustType::String;
        let result = box_impl_trait_for_field(&string_type);
        assert!(matches!(result, RustType::String));
    }

    // ============================================================
    // default value helper tests
    // ============================================================

    #[test]
    fn test_default_int() {
        let result = default_int();
        assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_default_float() {
        let result = default_float();
        assert_eq!(result.to_string(), "0.0");
    }

    #[test]
    fn test_default_bool() {
        let result = default_bool();
        assert_eq!(result.to_string(), "false");
    }

    #[test]
    fn test_default_string() {
        let result = default_string();
        assert_eq!(result.to_string(), "String :: new ()");
    }

    #[test]
    fn test_default_generic() {
        let result = default_generic();
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    // ============================================================
    // get_default_value_for_type tests
    // ============================================================

    #[test]
    fn test_get_default_value_int() {
        let result = get_default_value_for_type(&Type::Int);
        assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_get_default_value_float() {
        let result = get_default_value_for_type(&Type::Float);
        assert_eq!(result.to_string(), "0.0");
    }

    #[test]
    fn test_get_default_value_bool() {
        let result = get_default_value_for_type(&Type::Bool);
        assert_eq!(result.to_string(), "false");
    }

    #[test]
    fn test_get_default_value_string() {
        let result = get_default_value_for_type(&Type::String);
        assert_eq!(result.to_string(), "String :: new ()");
    }

    #[test]
    fn test_get_default_value_unknown() {
        let result = get_default_value_for_type(&Type::Unknown);
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_list() {
        let result = get_default_value_for_type(&Type::List(Box::new(Type::Int)));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_optional() {
        let result = get_default_value_for_type(&Type::Optional(Box::new(Type::Int)));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_tuple() {
        let result = get_default_value_for_type(&Type::Tuple(vec![Type::Int, Type::Bool]));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_none() {
        let result = get_default_value_for_type(&Type::None);
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    // ============================================================
    // infer_yield_type tests
    // ============================================================

    #[test]
    fn test_infer_yield_type_int_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::Int));
    }

    #[test]
    fn test_infer_yield_type_float_literal() {
        let expr = HirExpr::Literal(Literal::Float(3.15));
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::Float));
    }

    #[test]
    fn test_infer_yield_type_string_literal() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_bool_literal() {
        let expr = HirExpr::Literal(Literal::Bool(true));
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::Bool));
    }

    #[test]
    fn test_infer_yield_type_none_literal() {
        let expr = HirExpr::Literal(Literal::None);
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::None));
    }

    #[test]
    fn test_infer_yield_type_bytes_literal() {
        let expr = HirExpr::Literal(Literal::Bytes(vec![0u8, 1, 2]));
        let result = infer_yield_type(&expr);
        assert!(matches!(result, Type::Custom(s) if s == "bytes"));
    }

    #[test]
    fn test_infer_yield_type_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::String("hello".to_string())),
        ]);
        let result = infer_yield_type(&expr);
        match result {
            Type::Tuple(types) => {
                assert_eq!(types.len(), 2);
                assert!(matches!(types[0], Type::Int));
                assert!(matches!(types[1], Type::String));
            }
            _ => panic!("Expected Tuple type"),
        }
    }

    #[test]
    fn test_infer_yield_type_var() {
        let expr = HirExpr::Var("x".to_string());
        let result = infer_yield_type(&expr);
        // Variables default to String (safer than Unknown)
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_binary_op() {
        let expr = HirExpr::Binary {
            op: crate::hir::BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        let result = infer_yield_type(&expr);
        // Binary ops default to String (catch-all)
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_call() {
        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let result = infer_yield_type(&expr);
        // Calls default to String
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_nested_tuple() {
        let expr = HirExpr::Tuple(vec![
            HirExpr::Tuple(vec![
                HirExpr::Literal(Literal::Int(1)),
                HirExpr::Literal(Literal::Int(2)),
            ]),
            HirExpr::Literal(Literal::Bool(true)),
        ]);
        let result = infer_yield_type(&expr);
        match result {
            Type::Tuple(types) => {
                assert_eq!(types.len(), 2);
                match &types[0] {
                    Type::Tuple(inner) => {
                        assert_eq!(inner.len(), 2);
                        assert!(matches!(inner[0], Type::Int));
                        assert!(matches!(inner[1], Type::Int));
                    }
                    _ => panic!("Expected nested Tuple"),
                }
                assert!(matches!(types[1], Type::Bool));
            }
            _ => panic!("Expected Tuple type"),
        }
    }

    // ============================================================
    // LoopInfo tests (via extract_loop_info)
    // ============================================================

    #[test]
    fn test_loop_info_struct_fields() {
        // LoopInfo struct has pre_loop_stmts, condition, body_stmts
        let loop_info = LoopInfo {
            pre_loop_stmts: vec![],
            condition: HirExpr::Literal(Literal::Bool(true)),
            body_stmts: vec![],
        };
        assert!(loop_info.pre_loop_stmts.is_empty());
        assert!(loop_info.body_stmts.is_empty());
    }

    // ============================================================
    // populate_generator_state_vars tests
    // ============================================================

    // Helper function to create empty HirFunction for tests
    fn empty_hir_function() -> HirFunction {
        use crate::hir::FunctionProperties;
        use depyler_annotations::TranspilationAnnotations;
        use smallvec::smallvec;
        HirFunction {
            name: "test".to_string(),
            params: smallvec![],
            body: vec![],
            ret_type: Type::Unknown,
            properties: FunctionProperties::default(),
            annotations: TranspilationAnnotations::default(),
            docstring: None,
        }
    }

    #[test]
    fn test_populate_generator_state_vars_empty() {
        let mut ctx = CodeGenContext::default();
        let state_info = GeneratorStateInfo {
            state_variables: vec![],
            captured_params: vec![],
            yield_count: 0,
            has_loops: false,
        };
        populate_generator_state_vars(&mut ctx, &state_info, &empty_hir_function());
        assert!(ctx.generator_state_vars.is_empty());
    }

    #[test]
    fn test_populate_generator_state_vars_with_state() {
        use crate::generator_state::StateVariable;
        let mut ctx = CodeGenContext::default();
        let state_info = GeneratorStateInfo {
            state_variables: vec![StateVariable {
                name: "counter".to_string(),
                ty: Type::Int,
            }],
            captured_params: vec![],
            yield_count: 1,
            has_loops: false,
        };
        populate_generator_state_vars(&mut ctx, &state_info, &empty_hir_function());
        assert!(ctx.generator_state_vars.contains("counter"));
        assert_eq!(ctx.generator_state_vars.len(), 1);
    }

    #[test]
    fn test_populate_generator_state_vars_with_params() {
        let mut ctx = CodeGenContext::default();
        let state_info = GeneratorStateInfo {
            state_variables: vec![],
            captured_params: vec!["n".to_string(), "limit".to_string()],
            yield_count: 1,
            has_loops: true,
        };
        populate_generator_state_vars(&mut ctx, &state_info, &empty_hir_function());
        assert!(ctx.generator_state_vars.contains("n"));
        assert!(ctx.generator_state_vars.contains("limit"));
        assert_eq!(ctx.generator_state_vars.len(), 2);
    }

    #[test]
    fn test_populate_generator_state_vars_mixed() {
        use crate::generator_state::StateVariable;
        let mut ctx = CodeGenContext::default();
        let state_info = GeneratorStateInfo {
            state_variables: vec![
                StateVariable {
                    name: "i".to_string(),
                    ty: Type::Int,
                },
                StateVariable {
                    name: "acc".to_string(),
                    ty: Type::Float,
                },
            ],
            captured_params: vec!["start".to_string()],
            yield_count: 2,
            has_loops: true,
        };
        populate_generator_state_vars(&mut ctx, &state_info, &empty_hir_function());
        assert!(ctx.generator_state_vars.contains("i"));
        assert!(ctx.generator_state_vars.contains("acc"));
        assert!(ctx.generator_state_vars.contains("start"));
        assert_eq!(ctx.generator_state_vars.len(), 3);
    }

    #[test]
    fn test_populate_generator_state_vars_clears_previous() {
        use crate::generator_state::StateVariable;
        let mut ctx = CodeGenContext::default();
        ctx.generator_state_vars.insert("old_var".to_string());

        let state_info = GeneratorStateInfo {
            state_variables: vec![StateVariable {
                name: "new_var".to_string(),
                ty: Type::Int,
            }],
            captured_params: vec![],
            yield_count: 1,
            has_loops: false,
        };
        populate_generator_state_vars(&mut ctx, &state_info, &empty_hir_function());
        assert!(!ctx.generator_state_vars.contains("old_var"));
        assert!(ctx.generator_state_vars.contains("new_var"));
        assert_eq!(ctx.generator_state_vars.len(), 1);
    }

    // ============================================================
    // Edge case tests
    // ============================================================

    #[test]
    fn test_box_impl_trait_iterator_without_item() {
        let impl_iter = RustType::Custom("impl Iterator".to_string());
        let result = box_impl_trait_for_field(&impl_iter);
        match result {
            RustType::Custom(s) => assert_eq!(s, "Box<dyn Iterator>"),
            _ => panic!("Expected Custom type"),
        }
    }

    #[test]
    fn test_infer_yield_type_empty_tuple() {
        let expr = HirExpr::Tuple(vec![]);
        let result = infer_yield_type(&expr);
        match result {
            Type::Tuple(types) => assert!(types.is_empty()),
            _ => panic!("Expected empty Tuple type"),
        }
    }

    #[test]
    fn test_infer_yield_type_attribute_access() {
        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        let result = infer_yield_type(&expr);
        // Attribute access defaults to String (catch-all)
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_subscript() {
        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        let result = infer_yield_type(&expr);
        // Index access defaults to String (catch-all)
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_generate_state_struct_name_trailing_underscore() {
        let input_name = syn::Ident::new("gen_", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "GenState");
    }

    #[test]
    fn test_concretize_preserves_result() {
        let result_type = RustType::Result(
            Box::new(RustType::String),
            Box::new(RustType::Custom("Error".to_string())),
        );
        let result = concretize_type_param_for_struct(&result_type);
        assert!(matches!(result, RustType::Result(_, _)));
    }

    #[test]
    fn test_concretize_preserves_hashmap() {
        let map_type = RustType::HashMap(
            Box::new(RustType::String),
            Box::new(RustType::Primitive(PrimitiveType::I32)),
        );
        let result = concretize_type_param_for_struct(&map_type);
        assert!(matches!(result, RustType::HashMap(_, _)));
    }

    #[test]
    fn test_concretize_type_param_u() {
        // TypeParam with different name
        let type_param = RustType::TypeParam("U".to_string());
        let result = concretize_type_param_for_struct(&type_param);
        assert!(matches!(result, RustType::Primitive(PrimitiveType::I32)));
    }

    #[test]
    fn test_get_default_value_dict() {
        let result =
            get_default_value_for_type(&Type::Dict(Box::new(Type::String), Box::new(Type::Int)));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_set() {
        let result = get_default_value_for_type(&Type::Set(Box::new(Type::Int)));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_custom() {
        let result = get_default_value_for_type(&Type::Custom("MyType".to_string()));
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_get_default_value_generic() {
        let result = get_default_value_for_type(&Type::Generic {
            base: "Iterator".to_string(),
            params: vec![Type::Int],
        });
        assert_eq!(result.to_string(), "Default :: default ()");
    }

    #[test]
    fn test_infer_yield_type_unary_op() {
        let expr = HirExpr::Unary {
            op: crate::hir::UnaryOp::Neg,
            operand: Box::new(HirExpr::Literal(Literal::Int(5))),
        };
        let result = infer_yield_type(&expr);
        // UnaryOp defaults to String
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_list() {
        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(2)),
        ]);
        let result = infer_yield_type(&expr);
        // List defaults to String (catch-all for complex expressions)
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_infer_yield_type_dict() {
        let expr = HirExpr::Dict(vec![(
            HirExpr::Literal(Literal::String("key".to_string())),
            HirExpr::Literal(Literal::Int(1)),
        )]);
        let result = infer_yield_type(&expr);
        // Dict defaults to String
        assert!(matches!(result, Type::String));
    }

    #[test]
    fn test_box_impl_trait_impl_clone() {
        // Other impl Trait types are not boxed
        let impl_clone = RustType::Custom("impl Clone".to_string());
        let result = box_impl_trait_for_field(&impl_clone);
        match result {
            RustType::Custom(s) => assert_eq!(s, "impl Clone"),
            _ => panic!("Expected Custom type"),
        }
    }

    #[test]
    fn test_generate_state_struct_name_numeric_suffix() {
        let input_name = syn::Ident::new("gen_v2", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "GenV2State");
    }

    #[test]
    fn test_generate_state_struct_name_single_char() {
        let input_name = syn::Ident::new("g", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);
        assert_eq!(result.to_string(), "GState");
    }
}
