//! Generator support and code generation
//!
//! This module handles Python generator functions, converting them to
//! Rust Iterator implementations with state structs.

use crate::generator_state::GeneratorStateInfo;
use crate::generator_yield_analysis::YieldAnalysis;
use crate::hir::{HirExpr, HirFunction, HirStmt, Literal, Type};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
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
            let field_name = syn::Ident::new(&var.name, proc_macro2::Span::call_site());
            let rust_type = ctx.type_mapper.map_type(&var.ty);
            let field_type = rust_type_to_syn(&rust_type)?;
            Ok(quote! { #field_name: #field_type })
        })
        .collect()
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
            let field_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            let rust_type = ctx.type_mapper.map_type(&param.ty);
            let field_type = rust_type_to_syn(&rust_type)?;
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
    // DEPYLER-0263: Infer type from first yield expression if func.ret_type is Unknown
    let yield_type = if matches!(func.ret_type, Type::Unknown) && !yield_analysis.yield_points.is_empty() {
        // Infer from first yield expression
        infer_yield_type(&yield_analysis.yield_points[0].yield_expr)
    } else {
        // Use func.ret_type if available
        func.ret_type.clone()
    };

    let rust_yield_type = ctx.type_mapper.map_type(&yield_type);
    rust_type_to_syn(&rust_yield_type)
}

/// Infer type from a yield expression
///
/// # Complexity: 2 (match + return)
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
        HirExpr::Var(_) => Type::Int, // Default to Int for variables without type info
        _ => Type::Unknown,
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
            let field_name = syn::Ident::new(&var.name, proc_macro2::Span::call_site());
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
            let field_name = syn::Ident::new(&param.name, proc_macro2::Span::call_site());
            // Initialize with parameter value (n: n)
            quote! { #field_name: #field_name }
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
/// # Complexity: 3 (clear + 2 loops)
#[inline]
fn populate_generator_state_vars(ctx: &mut CodeGenContext, state_info: &GeneratorStateInfo) {
    ctx.generator_state_vars.clear();
    for var in &state_info.state_variables {
        ctx.generator_state_vars.insert(var.name.clone());
    }
    for param in &state_info.captured_params {
        ctx.generator_state_vars.insert(param.clone());
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

    let (condition, body) = loop_stmt
        .ok_or_else(|| anyhow::anyhow!("No while loop found in generator function"))?;

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
    // Analyze generator state requirements
    let state_info = GeneratorStateInfo::analyze(func);

    // DEPYLER-0262 Phase 2: Analyze yield points for state machine transformation
    let yield_analysis = YieldAnalysis::analyze(func);

    // DEPYLER-0262 Phase 3A: Check if we can use simple multi-state transformation
    let use_simple_multi_state = yield_analysis.has_yields()
        && yield_analysis.yield_points.iter().all(|yp| yp.depth == 0);

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

    // Populate generator state variables for scoping
    populate_generator_state_vars(ctx, &state_info);

    // DEPYLER-0262 Phase 3B: Check if we have simple loop with yield pattern
    let has_while_loop = func.body.iter().any(|stmt| matches!(stmt, HirStmt::While { .. }));
    let has_loop_yields = yield_analysis.has_yields()
        && yield_analysis.yield_points.iter().any(|yp| yp.depth > 0);

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

    // Generate the complete generator implementation
    Ok(quote! {
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
                #state_machine_impl
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_DEPYLER_0259_snake_case_to_pascal_case_naming() {
        // BUG #2: generate_state_struct_name only capitalizes first character
        // Input: "count_up" → Current: "Count_upState" (WRONG)
        // Input: "count_up" → Expected: "CountUpState" (CORRECT)

        let input_name = syn::Ident::new("count_up", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);

        // This WILL FAIL (RED phase) because current code produces "Count_upState"
        assert_eq!(
            result.to_string(),
            "CountUpState",
            "DEPYLER-0259: Should convert snake_case to PascalCase, not just capitalize first char"
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DEPYLER_0259_single_word_naming() {
        // Edge case: single word (no underscores)
        let input_name = syn::Ident::new("counter", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);

        // Should just capitalize and add "State"
        assert_eq!(result.to_string(), "CounterState");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DEPYLER_0259_multiple_words_naming() {
        // Test with multiple underscores
        let input_name = syn::Ident::new("fibonacci_generator_with_memo", proc_macro2::Span::call_site());
        let result = generate_state_struct_name(&input_name);

        // Should capitalize each word
        assert_eq!(result.to_string(), "FibonacciGeneratorWithMemoState");
    }
}
