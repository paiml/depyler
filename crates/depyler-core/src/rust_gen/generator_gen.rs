//! Generator support and code generation
//!
//! This module handles Python generator functions, converting them to
//! Rust Iterator implementations with state structs.

use crate::generator_state::GeneratorStateInfo;
use crate::generator_yield_analysis::YieldAnalysis;
use crate::hir::{HirFunction, Type};
use crate::rust_gen::type_gen::rust_type_to_syn;
use crate::rust_gen::CodeGenContext;
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
/// DEPYLER-0260 FIX: Use func.ret_type directly as yield type, then map to Rust type.
/// This fixes the DynamicType bug where generators used undefined DynamicType instead of
/// concrete types like i32.
///
/// For generators, func.ret_type contains the yield type directly (e.g., Type::Int),
/// not wrapped in a Generator variant. The is_generator flag marks it as a generator.
///
/// # Complexity
/// 2 (map + convert)
#[inline]
fn extract_generator_item_type(
    func: &HirFunction,
    ctx: &CodeGenContext,
) -> Result<syn::Type> {
    // DEPYLER-0260 FIX: func.ret_type already contains the yield type
    // (e.g., Type::Int for a generator that yields integers)
    let rust_yield_type = ctx.type_mapper.map_type(&func.ret_type);
    rust_type_to_syn(&rust_yield_type)
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
    let _yield_analysis = YieldAnalysis::analyze(func);
    // TODO(DEPYLER-0262 Phase 3): Use _yield_analysis to generate multi-state machine
    // Currently: Single-state implementation (state 0 → state 1 → done)
    // Future: Multi-state with proper resume points for each yield
    // yield_analysis contains:
    //   - yield_points: Vec<YieldPoint> with state_id, depth, live_vars
    //   - resume_points: HashMap<state_id, stmt_idx> for resumption
    //   - num_states(): Total states needed (0 + num yields)

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

    // DEPYLER-0260 FIX: Extract yield value type from HIR Type::Generator, not mapped RustType
    let item_type = extract_generator_item_type(func, ctx)?;

    // Populate generator state variables for scoping
    populate_generator_state_vars(ctx, &state_info);

    // Generate body statements with proper context
    let generator_body_stmts = generate_generator_body(func, ctx)?;

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
                // State machine implementation: Simplified single-state execution
                // for basic generator support (v3.12.0). Multi-state transformation
                // with resumable yield points is future work.
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
