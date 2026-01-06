//! Advanced function codegen: nested functions, type inference helpers
//!
//! DEPYLER-COVERAGE-95: Extracted from func_gen.rs to reduce file size
//! and improve testability. Contains return type inference and nested function detection.

use crate::hir::*;
use crate::lifetime_analysis::LifetimeInference;
use crate::rust_gen::context::{CodeGenContext, RustCodeGen};
use crate::rust_gen::control_flow_analysis::stmt_always_returns;
use crate::rust_gen::func_gen::{
    build_var_type_env, codegen_function_body, codegen_function_params,
    collect_return_types_with_env, function_returns_owned_string,
    function_returns_string_concatenation, infer_expr_type_with_env, infer_param_type_from_body,
    infer_return_type_from_body_with_params, rewrite_adt_child_type,
};
use crate::rust_gen::func_gen_helpers::{
    codegen_function_attrs, codegen_generic_params, codegen_where_clause,
};
use crate::rust_gen::generator_gen::codegen_generator_function;
use crate::rust_gen::keywords::is_rust_keyword;
use crate::rust_gen::rust_type_to_syn;
use crate::rust_gen::type_gen::update_import_needs;
use anyhow::Result;
use quote::quote;
use syn::parse_quote;


/// GH-70: Detect if function returns a nested function/closure
/// Returns Some((nested_fn_name, params, ret_type)) if detected
/// Stores inferred params in ctx.nested_function_params for use during code generation
pub(crate) fn detect_returns_nested_function(
    func: &HirFunction,
    ctx: &mut CodeGenContext,
) -> Option<(String, Vec<HirParam>, Type)> {
    // Look for pattern: function contains nested FunctionDef and ends with returning that name
    let mut nested_functions: std::collections::HashMap<String, (Vec<HirParam>, Type)> =
        std::collections::HashMap::new();

    // Collect nested function definitions with type inference
    for stmt in &func.body {
        if let HirStmt::FunctionDef {
            name,
            params,
            ret_type,
            body,
            ..
        } = stmt
        {
            // GH-70: Apply type inference to parameters
            // DEPYLER-0737: Also handle Optional(Unknown) for params with default=None
            let mut inferred_params = params.to_vec();
            for param in &mut inferred_params {
                if matches!(param.ty, Type::Unknown) {
                    // Try to infer from body usage
                    if let Some(inferred_ty) = infer_param_type_from_body(&param.name, body) {
                        param.ty = inferred_ty;
                    }
                } else if let Type::Optional(inner) = &param.ty {
                    // DEPYLER-0737: If param is Optional(Unknown), infer inner type and wrap
                    if matches!(inner.as_ref(), Type::Unknown) {
                        if let Some(inferred_ty) = infer_param_type_from_body(&param.name, body) {
                            param.ty = Type::Optional(Box::new(inferred_ty));
                        }
                    }
                }
            }

            // GH-70: Apply type inference to return type
            // Include inferred param types in the environment so that
            // expressions like `return item[0]` can infer the element type
            let inferred_ret_type = if matches!(ret_type, Type::Unknown) {
                // Build type env with inferred params
                let mut var_types: std::collections::HashMap<String, Type> =
                    std::collections::HashMap::new();
                for p in &inferred_params {
                    var_types.insert(p.name.clone(), p.ty.clone());
                }
                // Build from body assignments
                build_var_type_env(body, &mut var_types);

                // Collect return types using the enhanced environment
                let mut return_types = Vec::new();
                collect_return_types_with_env(body, &mut return_types, &var_types);

                // Check for trailing expression
                if let Some(HirStmt::Expr(expr)) = body.last() {
                    let trailing_type = infer_expr_type_with_env(expr, &var_types);
                    if !matches!(trailing_type, Type::Unknown) {
                        return_types.push(trailing_type);
                    }
                }

                // Get first known type
                return_types
                    .iter()
                    .find(|t| !matches!(t, Type::Unknown))
                    .cloned()
                    .unwrap_or_else(|| ret_type.clone())
            } else {
                ret_type.clone()
            };

            // Store inferred params in context for use during code generation
            ctx.nested_function_params
                .insert(name.clone(), inferred_params.clone());

            nested_functions.insert(name.clone(), (inferred_params, inferred_ret_type));
        }
    }

    // Check if last statement returns one of the nested functions
    if let Some(last_stmt) = func.body.last() {
        // Pattern 1: explicit return statement
        if let HirStmt::Return(Some(HirExpr::Var(var_name))) = last_stmt {
            if let Some((params, ret_type)) = nested_functions.get(var_name) {
                return Some((var_name.clone(), params.clone(), ret_type.clone()));
            }
        }
        // Pattern 2: implicit return (expression statement at end)
        if let HirStmt::Expr(HirExpr::Var(var_name)) = last_stmt {
            if let Some((params, ret_type)) = nested_functions.get(var_name) {
                return Some((var_name.clone(), params.clone(), ret_type.clone()));
            }
        }
    }

    None
}

/// DEPYLER-0626: Check if function returns heterogeneous IO types (File vs Stdout)
/// Returns true if function has return statements that return both file and stdio types
pub(crate) fn function_returns_heterogeneous_io(func: &HirFunction) -> bool {
    let mut has_file_return = false;
    let mut has_stdio_return = false;

    collect_io_return_types(&func.body, &mut has_file_return, &mut has_stdio_return);

    has_file_return && has_stdio_return
}

/// DEPYLER-0626: Helper to collect IO return types from statements
pub(crate) fn collect_io_return_types(stmts: &[HirStmt], has_file: &mut bool, has_stdio: &mut bool) {
    for stmt in stmts {
        match stmt {
            HirStmt::Return(Some(expr)) => {
                if is_file_creating_return_expr(expr) {
                    *has_file = true;
                }
                if is_stdio_return_expr(expr) {
                    *has_stdio = true;
                }
            }
            HirStmt::If { then_body, else_body, .. } => {
                collect_io_return_types(then_body, has_file, has_stdio);
                if let Some(else_stmts) = else_body {
                    collect_io_return_types(else_stmts, has_file, has_stdio);
                }
            }
            HirStmt::While { body, .. } | HirStmt::For { body, .. } => {
                collect_io_return_types(body, has_file, has_stdio);
            }
            _ => {}
        }
    }
}

/// DEPYLER-0626: Check if expression creates a File (open() or File::create())
pub(crate) fn is_file_creating_return_expr(expr: &HirExpr) -> bool {
    match expr {
        HirExpr::Call { func, .. } => func == "open",
        HirExpr::MethodCall { object, method, .. } => {
            if method == "create" || method == "open" {
                if let HirExpr::Var(name) = object.as_ref() {
                    return name == "File";
                }
                if let HirExpr::Attribute { attr, .. } = object.as_ref() {
                    return attr == "File";
                }
            }
            false
        }
        _ => false,
    }
}

/// DEPYLER-0626: Check if expression is sys.stdout or sys.stderr
pub(crate) fn is_stdio_return_expr(expr: &HirExpr) -> bool {
    if let HirExpr::Attribute { value, attr } = expr {
        if attr == "stdout" || attr == "stderr" {
            if let HirExpr::Var(name) = value.as_ref() {
                return name == "sys";
            }
        }
    }
    false
}

/// Generate return type with Result wrapper and lifetime handling
///
/// DEPYLER-0310: Now returns ErrorType (4th tuple element) for raise statement wrapping
/// GH-70: Now detects when function returns nested function and uses Box<dyn Fn>
#[inline]
pub(crate) fn codegen_return_type(
    func: &HirFunction,
    lifetime_result: &crate::lifetime_analysis::LifetimeResult,
    ctx: &mut CodeGenContext,
) -> Result<(
    proc_macro2::TokenStream,
    crate::type_mapper::RustType,
    bool,
    Option<crate::rust_gen::context::ErrorType>,
)> {
    // GH-70: Check if function returns a nested function/closure
    if let Some((_nested_name, params, nested_ret_type)) = detect_returns_nested_function(func, ctx)
    {
        use quote::quote;

        // Build Box<dyn Fn(params) -> ret> type
        let param_types: Vec<proc_macro2::TokenStream> = params
            .iter()
            .map(|p| crate::rust_gen::type_tokens::hir_type_to_tokens(&p.ty))
            .collect();

        let ret_ty_tokens = crate::rust_gen::type_tokens::hir_type_to_tokens(&nested_ret_type);

        let fn_type = if params.is_empty() {
            quote! { -> Box<dyn Fn() -> #ret_ty_tokens> }
        } else {
            quote! { -> Box<dyn Fn(#(#param_types),*) -> #ret_ty_tokens> }
        };

        return Ok((
            fn_type.clone(),
            crate::type_mapper::RustType::Custom("BoxedFn".to_string()),
            false, // can_fail
            None,  // error_type
        ));
    }

    // DEPYLER-0626: Check if function returns heterogeneous IO types (File vs Stdout)
    // If so, return type should be Box<dyn std::io::Write>
    if function_returns_heterogeneous_io(func) {
        use quote::quote;
        ctx.function_returns_boxed_write = true;
        ctx.needs_io_write = true;

        // Check if function can fail (uses open() which can fail)
        let can_fail = func.properties.can_fail;
        let error_type = if can_fail {
            Some(crate::rust_gen::context::ErrorType::Concrete("std::io::Error".to_string()))
        } else {
            None
        };

        let return_type = if can_fail {
            quote! { -> Result<Box<dyn std::io::Write>, std::io::Error> }
        } else {
            quote! { -> Box<dyn std::io::Write> }
        };

        return Ok((
            return_type,
            crate::type_mapper::RustType::Custom("BoxedWrite".to_string()),
            can_fail,
            error_type,
        ));
    }

    // DEPYLER-0410: Infer return type from body when annotation is Unknown
    // DEPYLER-0420: Also infer when tuple/list contains Unknown elements
    // DEPYLER-0460: Use _with_params version for Optional pattern detection
    // DEPYLER-0460: Also infer when ret_type is None, because that could be:
    // 1. A function returning None in all paths → () in Rust
    // 2. A function returning None|T (Optional pattern) → Option<T> in Rust
    // DEPYLER-0662: Also infer when ret_type is empty tuple (from `-> tuple` annotation)
    // Python `-> tuple` without type params should be inferred from return statements
    // DEPYLER-0662: Python `-> tuple` parses to Type::Custom("tuple"), not Type::Tuple
    let should_infer = matches!(func.ret_type, Type::Unknown | Type::None)
        || matches!(&func.ret_type, Type::Tuple(elems) if elems.is_empty() || elems.iter().any(|t| matches!(t, Type::Unknown)))
        || matches!(&func.ret_type, Type::List(elem) if matches!(**elem, Type::Unknown))
        || matches!(&func.ret_type, Type::Custom(name) if name == "tuple");

    let effective_ret_type = if should_infer {
        // Try to infer from return statements in body (with parameter type tracking for Optional detection)
        infer_return_type_from_body_with_params(func, ctx).unwrap_or_else(|| func.ret_type.clone())
    } else {
        func.ret_type.clone()
    };

    // DEPYLER-0719: Update function_return_types with inferred type
    // When a function's return type is inferred (e.g., `-> tuple` → `(f64, f64)`),
    // update the map so callers like `point: tuple = get_point()` can use the inferred type
    if should_infer && effective_ret_type != func.ret_type {
        ctx.function_return_types
            .insert(func.name.clone(), effective_ret_type.clone());
    }

    // DEPYLER-0716: Apply type substitutions to return type
    // When generic parameters are substituted (e.g., T -> String), apply to return type too
    let effective_ret_type = if !ctx.type_substitutions.is_empty() {
        crate::generic_inference::TypeVarRegistry::apply_substitutions(
            &effective_ret_type,
            &ctx.type_substitutions,
        )
    } else {
        effective_ret_type
    };

    // DEPYLER-0936: Rewrite ADT child types to parent enum types
    // When a Python ABC hierarchy is converted to a Rust enum, return types mentioning
    // child classes (e.g., ListIter[T]) must be rewritten to parent (e.g., Iter[T])
    let effective_ret_type = rewrite_adt_child_type(&effective_ret_type, &ctx.adt_child_to_parent);

    // Convert return type using annotation-aware mapping
    let mapped_ret_type = ctx
        .annotation_aware_mapper
        .map_return_type_with_annotations(&effective_ret_type, &func.annotations);

    // Check if this is a placeholder Union enum that needs proper generation
    let rust_ret_type = if let crate::type_mapper::RustType::Enum { name, .. } = &mapped_ret_type {
        if name == "UnionType" {
            // Generate a proper enum name and definition from the original Union type
            if let Type::Union(types) = &func.ret_type {
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

    // v3.16.0 Phase 1: Override return type to String if function returns owned via string methods
    // This prevents lifetime analysis from incorrectly converting to borrowed &str
    let rust_ret_type =
        if matches!(func.ret_type, Type::String) && function_returns_owned_string(func) {
            // Force owned String return, don't use lifetime borrowing
            crate::type_mapper::RustType::String
        } else {
            rust_ret_type
        };

    // Update import needs based on return type
    update_import_needs(ctx, &rust_ret_type);

    // Check if function can fail and needs Result wrapper
    let can_fail = func.properties.can_fail;
    let mut error_type_str = if can_fail && !func.properties.error_types.is_empty() {
        // Use first error type or generic for mixed types
        if func.properties.error_types.len() == 1 {
            func.properties.error_types[0].clone()
        } else {
            "Box<dyn std::error::Error>".to_string()
        }
    } else {
        "Box<dyn std::error::Error>".to_string()
    };

    // DEPYLER-0597: Map Python exception types to Rust error types
    // This ensures function signatures like `-> Result<T, OSError>` compile
    // Using Box<dyn std::error::Error> for most exceptions since it doesn't require external crates
    error_type_str = match error_type_str.as_str() {
        // File/IO related exceptions map to std::io::Error for idiomatic Rust
        "OSError" | "IOError" | "FileNotFoundError" | "PermissionError" => {
            "std::io::Error".to_string()
        }
        // General exceptions map to Box<dyn std::error::Error> (no external crate needed)
        "Exception" | "BaseException" | "ValueError" | "TypeError" | "KeyError"
        | "IndexError" | "RuntimeError" | "AttributeError" | "NotImplementedError"
        | "AssertionError" | "StopIteration" | "ZeroDivisionError" | "OverflowError"
        | "ArithmeticError" => "Box<dyn std::error::Error>".to_string(),
        // Keep other types as-is (might be custom error types)
        _ => error_type_str,
    };

    // DEPYLER-0447: Validators always use Box<dyn Error> for compatibility with clap
    if ctx.validator_functions.contains(&func.name) {
        error_type_str = "Box<dyn std::error::Error>".to_string();
    }

    // DEPYLER-0310: Determine ErrorType for raise statement wrapping
    // If Box<dyn Error>, we need to wrap exceptions with Box::new()
    // If concrete type, no wrapping needed
    let error_type = if can_fail {
        Some(if error_type_str.contains("Box<dyn") {
            crate::rust_gen::context::ErrorType::DynBox
        } else {
            crate::rust_gen::context::ErrorType::Concrete(error_type_str.clone())
        })
    } else {
        None
    };

    // DEPYLER-0327 Fix #5: Mark error types as needed for type generation
    // Check BOTH error_type_str (for functions that return Result) AND
    // func.properties.error_types (for types used in try/except blocks)
    // DEPYLER-0551: Added RuntimeError and FileNotFoundError
    if error_type_str.contains("ZeroDivisionError") {
        ctx.needs_zerodivisionerror = true;
    }
    if error_type_str.contains("IndexError") {
        ctx.needs_indexerror = true;
    }
    if error_type_str.contains("ValueError") {
        ctx.needs_valueerror = true;
    }
    if error_type_str.contains("RuntimeError") {
        ctx.needs_runtimeerror = true;
    }
    if error_type_str.contains("FileNotFoundError") {
        ctx.needs_filenotfounderror = true;
    }

    // Also check all error_types from properties (even if can_fail=false)
    // This ensures types used in try/except blocks are generated
    for err_type in &func.properties.error_types {
        if err_type.contains("ZeroDivisionError") {
            ctx.needs_zerodivisionerror = true;
        }
        if err_type.contains("IndexError") {
            ctx.needs_indexerror = true;
        }
        if err_type.contains("ValueError") {
            ctx.needs_valueerror = true;
        }
        if err_type.contains("RuntimeError") {
            ctx.needs_runtimeerror = true;
        }
        if err_type.contains("FileNotFoundError") {
            ctx.needs_filenotfounderror = true;
        }
    }

    let return_type = if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
        if can_fail {
            let error_type: syn::Type = syn::parse_str(&error_type_str)
                .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });

            // DEPYLER-0455 Bug 7: Infer return type from function body
            // Functions without type annotations but that return values (e.g., argparse validators)
            // should infer their return type from actual return statements
            //
            // Example: def email_address(value):
            //              return value  # <- Returns string, not None
            //
            // Before fix: Result<(), Box<dyn Error>>  [WRONG - type mismatch with returned value]
            // After fix:  Result<String, Box<dyn Error>>  [CORRECT - matches return value]
            if let Some(inferred_type) = infer_return_type_from_body_with_params(func, ctx) {
                // We found a return statement with a value!
                // Map the inferred HIR type to Rust type
                let inferred_rust_type = ctx
                    .annotation_aware_mapper
                    .map_return_type_with_annotations(&inferred_type, &func.annotations);

                // Convert to syn type
                if let Ok(ty) = rust_type_to_syn(&inferred_rust_type) {
                    // DEPYLER-0612: main() can only return () or Result<(), E>
                    if func.name == "main" {
                        quote! { -> Result<(), #error_type> }
                    } else {
                        // Use inferred type instead of ()
                        quote! { -> Result<#ty, #error_type> }
                    }
                } else {
                    // Fallback to () if conversion fails
                    quote! { -> Result<(), #error_type> }
                }
            } else {
                // No return value found, use ()
                quote! { -> Result<(), #error_type> }
            }
        } else {
            quote! {}
        }
    } else {
        let mut ty = rust_type_to_syn(&rust_ret_type)?;

        // DEPYLER-0270: Check if function returns string concatenation
        // String concatenation (format!(), a + b) always returns owned String
        // Never use Cow for concatenation results
        let returns_concatenation = matches!(func.ret_type, crate::hir::Type::String)
            && function_returns_string_concatenation(func);

        // Check if any parameter escapes through return and uses Cow
        let mut uses_cow_return = false;
        if !returns_concatenation {
            // Only consider Cow if NOT doing string concatenation
            for param in &func.params {
                if let Some(strategy) = lifetime_result.borrowing_strategies.get(&param.name) {
                    if matches!(
                        strategy,
                        crate::borrowing_context::BorrowingStrategy::UseCow { .. }
                    ) {
                        if let Some(_usage) = lifetime_result.param_lifetimes.get(&param.name) {
                            // If a Cow parameter escapes, return type should also be Cow
                            if matches!(func.ret_type, crate::hir::Type::String) {
                                uses_cow_return = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if uses_cow_return && !returns_concatenation {
            // Use the same Cow type for return
            ctx.needs_cow = true;
            if let Some(ref return_lt) = lifetime_result.return_lifetime {
                let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
                ty = parse_quote! { Cow<#lt, str> };
            } else {
                ty = parse_quote! { Cow<'static, str> };
            }
        } else {
            // v3.16.0 Phase 1: Check if function returns owned String via transformation methods
            // If so, don't convert to borrowed &str even if lifetime analysis suggests it
            let returns_owned_string =
                matches!(func.ret_type, Type::String) && function_returns_owned_string(func);

            // Apply return lifetime if needed (unless returning owned String)
            if let Some(ref return_lt) = lifetime_result.return_lifetime {
                // Check if the return type needs lifetime substitution
                if matches!(
                    rust_ret_type,
                    crate::type_mapper::RustType::Str { .. }
                        | crate::type_mapper::RustType::Reference { .. }
                ) && !returns_owned_string
                {
                    // Only apply lifetime if NOT returning owned String
                    let lt = syn::Lifetime::new(return_lt.as_str(), proc_macro2::Span::call_site());
                    match &rust_ret_type {
                        crate::type_mapper::RustType::Str { .. } => {
                            ty = parse_quote! { &#lt str };
                        }
                        crate::type_mapper::RustType::Reference { mutable, inner, .. } => {
                            let inner_ty = rust_type_to_syn(inner)?;
                            ty = if *mutable {
                                parse_quote! { &#lt mut #inner_ty }
                            } else {
                                parse_quote! { &#lt #inner_ty }
                            };
                        }
                        _ => {}
                    }
                }
            }
            // If returns_owned_string is true, keep ty as String (already set from rust_type_to_syn)
        }

        if can_fail {
            let error_type: syn::Type = syn::parse_str(&error_type_str)
                .unwrap_or_else(|_| parse_quote! { Box<dyn std::error::Error> });

            // DEPYLER-0612: main() can only return () or Result<(), E>
            // Convert Result<i32, E> to Result<(), E> for main
            if func.name == "main" {
                quote! { -> Result<(), #error_type> }
            } else {
                quote! { -> Result<#ty, #error_type> }
            }
        } else if func.name == "main" && matches!(func.ret_type, Type::Int) {
            // DEPYLER-0617: main() can only return () or Result<(), E>
            // Convert i32 return to () for non-fallible main
            quote! {}  // No return type annotation (defaults to ())
        } else {
            quote! { -> #ty }
        }
    };

    Ok((return_type, rust_ret_type, can_fail, error_type))
}

// ========== Phase 3c: Generator Implementation ==========
// (Moved to generator_gen.rs in v3.18.0 Phase 4)

impl RustCodeGen for HirFunction {
    fn to_rust_tokens(&self, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        // DEPYLER-0717: Clear var_types at the start of each function to prevent type leaking
        // Without this, parameter types from one function can leak to the next function
        // when they share the same parameter name (e.g., both have `items` parameter)
        ctx.var_types.clear();
        ctx.type_substitutions.clear();

        // DEPYLER-0306 FIX: Use raw identifiers for function names that are Rust keywords
        let name = if is_rust_keyword(&self.name) {
            syn::Ident::new_raw(&self.name, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(&self.name, proc_macro2::Span::call_site())
        };

        // DEPYLER-0269: Track function return type for Display trait selection
        // Store function return type in ctx for later lookup when processing assignments
        // This enables tracking `result = merge(&a, &b)` where merge returns list[int]
        ctx.function_return_types
            .insert(self.name.clone(), self.ret_type.clone());

        // DEPYLER-0621: Track parameter defaults for call-site argument completion
        // When a function like `def f(x=None)` is called as `f()`, we need to supply `None`
        let param_defaults: Vec<Option<crate::hir::HirExpr>> = self
            .params
            .iter()
            .map(|p| p.default.clone())
            .collect();
        ctx.function_param_defaults
            .insert(self.name.clone(), param_defaults);

        // Perform generic type inference
        let mut generic_registry = crate::generic_inference::TypeVarRegistry::new();

        // DEPYLER-0716: Infer type substitutions (e.g., T -> String when comparing to strings)
        let type_substitutions = generic_registry.infer_type_substitutions(self)?;

        // DEPYLER-0716: Apply substitutions to parameter types in var_types
        // This ensures List(Unknown) becomes List(String) when elements are compared to strings
        if !type_substitutions.is_empty() {
            for param in &self.params {
                let substituted_ty = crate::generic_inference::TypeVarRegistry::apply_substitutions(
                    &param.ty,
                    &type_substitutions,
                );
                if substituted_ty != param.ty {
                    ctx.var_types.insert(param.name.clone(), substituted_ty);
                }
            }
            // DEPYLER-0716: Store substitutions in context for return type processing
            ctx.type_substitutions = type_substitutions;
        }

        // DEPYLER-0524: Infer parameter types from usage in function body
        // This updates var_types so parameters with Unknown type can be inferred from usage
        // DEPYLER-0737: Also handle Optional(Unknown) for params with default=None
        // IMPORTANT: Must run BEFORE generic inference so that inferred concrete types
        // prevent unnecessary generic parameters from being generated
        let mut inferred_params = self.params.clone();
        for param in &mut inferred_params {
            if matches!(param.ty, Type::Unknown) {
                if let Some(inferred_ty) = infer_param_type_from_body(&param.name, &self.body) {
                    param.ty = inferred_ty.clone();
                    ctx.var_types.insert(param.name.clone(), inferred_ty);
                }
            } else if let Type::Optional(inner) = &param.ty {
                // DEPYLER-0737: If param is Optional(Unknown), infer inner type and wrap in Optional
                if matches!(inner.as_ref(), Type::Unknown) {
                    if let Some(inferred_ty) = infer_param_type_from_body(&param.name, &self.body) {
                        let new_ty = Type::Optional(Box::new(inferred_ty));
                        param.ty = new_ty.clone();
                        ctx.var_types.insert(param.name.clone(), new_ty);
                    }
                }
            }
        }

        // Create a modified version of self with inferred params for generic inference
        let inferred_self = HirFunction {
            params: inferred_params,
            ..self.clone()
        };
        let type_params = generic_registry.infer_function_generics(&inferred_self)?;

        // Perform lifetime analysis with automatic elision (DEPYLER-0275)
        let mut lifetime_inference = LifetimeInference::new();
        let lifetime_result = lifetime_inference
            .apply_elision_rules(self, ctx.type_mapper)
            .unwrap_or_else(|| lifetime_inference.analyze_function(self, ctx.type_mapper));

        // Generate combined generic parameters (lifetimes + type params)
        let generic_params = codegen_generic_params(&type_params, &lifetime_result.lifetime_params);

        // Generate lifetime bounds
        let where_clause = codegen_where_clause(&lifetime_result.lifetime_bounds);

        // DEPYLER-0738: Analyze variable mutability BEFORE parameter generation
        // This detects reassignments (x = 1; x = 2) and method mutations (.insert(), .push())
        // Must run before codegen_function_params so param_muts can access ctx.mutable_vars
        crate::rust_gen::analyze_mutable_vars(&self.body, ctx, &self.params);

        // Convert parameters using lifetime analysis results
        let params = codegen_function_params(self, &lifetime_result, ctx)?;

        // DEPYLER-0270: Extract parameter borrowing information for auto-borrow decisions
        // Check which parameters are references (borrowed) vs owned
        let param_borrows: Vec<bool> = self
            .params
            .iter()
            .map(|p| {
                lifetime_result
                    .param_lifetimes
                    .get(&p.name)
                    .map(|inf| inf.should_borrow)
                    .unwrap_or(false)
            })
            .collect();

        // DEPYLER-0758: Populate ref_params with borrowed parameter names for current function
        // Used in convert_binary to dereference reference params in arithmetic operations
        ctx.ref_params.clear();
        for (p, &is_borrowed) in self.params.iter().zip(param_borrows.iter()) {
            if is_borrowed {
                ctx.ref_params.insert(p.name.clone());
            }
        }

        ctx.function_param_borrows
            .insert(self.name.clone(), param_borrows);

        // DEPYLER-0574: Extract parameter mutability information for &mut decisions
        // Check which borrowed parameters need &mut (mutable borrow)
        let param_muts: Vec<bool> = self
            .params
            .iter()
            .map(|p| {
                let is_mutated = ctx.mutable_vars.contains(&p.name);
                let should_borrow = lifetime_result
                    .param_lifetimes
                    .get(&p.name)
                    .map(|inf| inf.should_borrow)
                    .unwrap_or(false);
                // needs_mut = mutated in body AND borrowed (not owned)
                is_mutated && should_borrow
            })
            .collect();
        ctx.function_param_muts
            .insert(self.name.clone(), param_muts);

        // DEPYLER-0779: Extract parameter optionality for Some() wrapping at call sites
        // A parameter is optional if: (a) type is Optional(T), OR (b) default is None
        let param_optionals: Vec<bool> = self
            .params
            .iter()
            .map(|p| {
                // Check if type is Optional(T)
                let type_is_optional = matches!(p.ty, Type::Optional(_));
                // Check if default value is None
                let default_is_none = matches!(
                    p.default,
                    Some(HirExpr::Literal(Literal::None))
                );
                type_is_optional || default_is_none
            })
            .collect();
        ctx.function_param_optionals
            .insert(self.name.clone(), param_optionals);

        // DEPYLER-0648: Track if function has vararg parameter (*args in Python)
        // These become &[T] in Rust, so call sites need to wrap args in &[...]
        if self.params.iter().any(|p| p.is_vararg) {
            ctx.vararg_functions.insert(self.name.clone());
        }

        // DEPYLER-0964: Track parameters that are &mut Option<HashMap<K, V>>
        // These occur when param type is Dict[K,V] with default None
        // Inside the function body, we need special handling:
        // - Assignment: `memo = {}` → `*memo = Some(HashMap::new())`
        // - Method calls: `memo.get(k)` → `memo.as_ref().unwrap().get(&k)`
        // - Subscript: `memo[k] = v` → `memo.as_mut().unwrap().insert(k, v)`
        for param in &self.params {
            let is_dict = matches!(&param.ty, Type::Dict { .. })
                || matches!(&param.ty, Type::Custom(name) if name == "dict");
            let has_none_default = matches!(
                &param.default,
                Some(HirExpr::Literal(Literal::None))
            );
            // Also check for Optional(Dict) type
            let is_optional_dict = matches!(
                &param.ty,
                Type::Optional(inner) if matches!(inner.as_ref(), Type::Dict { .. })
            );
            if (is_dict && has_none_default) || is_optional_dict {
                ctx.mut_option_dict_params.insert(param.name.clone());
            }
        }

        // Generate return type with Result wrapper and lifetime handling
        let (return_type, rust_ret_type, can_fail, error_type) =
            codegen_return_type(self, &lifetime_result, ctx)?;

        // DEPYLER-0839: Fix E0700 "hidden type captures lifetime" for impl Fn returns
        // When a function returns `impl Fn(...)` and captures reference parameters,
        // the return type must include a lifetime bound: `impl Fn(...) + 'a`
        // and the function must have the lifetime parameter: `fn foo<'a>(p: &'a str) -> impl Fn(...) + 'a`
        // Additionally, reference parameters must have the 'a lifetime: `&str` -> `&'a str`
        let (generic_params, return_type, params) = if let crate::type_mapper::RustType::Custom(ref type_str) = rust_ret_type {
            if type_str.contains("impl Fn") {
                // Check if any parameter is a reference (borrowed)
                // Access from ctx since param_borrows was moved into function_param_borrows earlier
                let has_ref_params = ctx.function_param_borrows
                    .get(&self.name)
                    .map(|borrows| borrows.iter().any(|&b| b))
                    .unwrap_or(false);
                if has_ref_params {
                    // Add 'a lifetime to generic params if not already present
                    let mut lifetime_params_with_a = lifetime_result.lifetime_params.clone();
                    if !lifetime_params_with_a.contains(&"'a".to_string()) {
                        lifetime_params_with_a.push("'a".to_string());
                    }
                    let new_generic_params = codegen_generic_params(&type_params, &lifetime_params_with_a);

                    // Modify return type to add + 'a bound
                    // The return type looks like "-> impl Fn(...) -> R" and we need "-> impl Fn(...) -> R + 'a"
                    let return_str = return_type.to_string();
                    let modified_return = if return_str.contains("impl Fn") {
                        // Find the impl Fn type and add + 'a at the end
                        // Handle both simple `impl Fn(T) -> R` and complex cases
                        let modified = format!("{} + 'a", return_str.trim());
                        syn::parse_str::<proc_macro2::TokenStream>(&modified)
                            .unwrap_or(return_type.clone())
                    } else {
                        return_type.clone()
                    };

                    // DEPYLER-0839: Also add 'a to reference parameter types
                    // `&str` -> `&'a str`, `& mut T` -> `&'a mut T`
                    let modified_params: Vec<proc_macro2::TokenStream> = params
                        .into_iter()
                        .map(|p| {
                            let param_str = p.to_string();
                            // Add 'a lifetime to references that don't already have a lifetime
                            // Pattern: `& ` (reference without lifetime) -> `& 'a `
                            // Pattern: `& mut ` (mutable reference without lifetime) -> `& 'a mut `
                            let modified_param = param_str
                                .replace("& str", "& 'a str")
                                .replace("& mut ", "& 'a mut ");
                            syn::parse_str::<proc_macro2::TokenStream>(&modified_param)
                                .unwrap_or(p)
                        })
                        .collect();

                    (new_generic_params, modified_return, modified_params)
                } else {
                    (generic_params.clone(), return_type, params)
                }
            } else {
                (generic_params.clone(), return_type, params)
            }
        } else {
            (generic_params.clone(), return_type, params)
        };

        // DEPYLER-0425: Analyze subcommand field access BEFORE generating body
        // This sets ctx.current_subcommand_fields so expression generation can rewrite args.field → field
        let subcommand_info = if ctx.argparser_tracker.has_subcommands() {
            crate::rust_gen::argparse_transform::analyze_subcommand_field_access(
                self,
                &ctx.argparser_tracker,
            )
        } else {
            None
        };

        // Set context for expression generation
        if let Some((_, ref fields)) = subcommand_info {
            ctx.current_subcommand_fields = Some(fields.iter().cloned().collect());
        }

        // DEPYLER-0456 Bug #1: Pre-register all add_parser() calls before body codegen
        // This ensures expression statement subcommands (no variable assignment) are included
        // in Commands enum generation. Must run BEFORE codegen_function_body() below.
        crate::rust_gen::argparse_transform::preregister_subcommands_from_hir(
            self,
            &mut ctx.argparser_tracker,
        );

        // DEPYLER-0108: Pre-populate Option fields for substitution BEFORE body codegen
        // This must happen before codegen_function_body() so that convert_method_call
        // can substitute args.<field>.is_some() with has_<field>
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                for arg in &parser_info.arguments {
                    if arg.rust_type().starts_with("Option<") {
                        ctx.precomputed_option_fields
                            .insert(arg.rust_field_name().to_string());
                    }
                }
            }
        }

        // DEPYLER-0617: Set flag if we're generating main() function
        // This affects return statement handling (integer returns → process::exit)
        let was_main = ctx.is_main_function;
        ctx.is_main_function = self.name == "main";

        // Process function body with proper scoping (expressions will now be rewritten if needed)
        let mut body_stmts = codegen_function_body(self, can_fail, error_type, ctx)?;

        // DEPYLER-0838: If function body is effectively empty (only pass statements) AND
        // return type is not unit, add unimplemented!() to satisfy the return type.
        // This handles Python's @abstractmethod pattern where body is just `pass`.
        {
            use quote::quote;
            let body_is_empty = body_stmts.iter().all(|stmt| stmt.is_empty());
            let is_non_unit_return = !matches!(rust_ret_type, crate::type_mapper::RustType::Unit);
            if body_is_empty && is_non_unit_return {
                body_stmts.push(quote! { unimplemented!() });
            }
        }

        // DEPYLER-0694: If function returns unit type (no return annotation in Python),
        // ensure trailing expressions don't accidentally return a value.
        // Add semicolon to discard the expression's value when not returning.
        // DEPYLER-0702: Use `let _ = expr;` instead of `expr;` to avoid unused-must-use warnings
        if matches!(rust_ret_type, crate::type_mapper::RustType::Unit) {
            if let Some(last) = body_stmts.last_mut() {
                let last_str = last.to_string();
                // If statement doesn't end with semicolon or closing brace, it's an expression
                // that would return a value - we need to discard it for Unit return types
                // DEPYLER-0711: Skip empty tokens (e.g., from `pass` statement)
                if !last_str.is_empty()
                    && !last_str.trim_end().ends_with(';')
                    && !last_str.trim_end().ends_with('}')
                {
                    use quote::quote;
                    let tokens = std::mem::take(last);
                    // Use `let _ = expr;` to discard the value without triggering
                    // "unused arithmetic operation" or similar warnings
                    *last = quote! { let _ = #tokens; };
                }
            }
        }

        // DEPYLER-0617: Restore flag after body generation
        ctx.is_main_function = was_main;

        // GH-70: Wrap returned closure in Box::new() if function returns Box<dyn Fn>
        if let Some((nested_name, _, _)) = detect_returns_nested_function(self, ctx) {
            // Find last statement and wrap if it's returning the nested function
            if let Some(last_stmt) = body_stmts.last_mut() {
                use quote::quote;
                let nested_ident = syn::Ident::new(&nested_name, proc_macro2::Span::call_site());
                // Check if last statement is just the variable name (implicit return)
                let last_stmt_str = last_stmt.to_string();
                if last_stmt_str.trim() == nested_name {
                    // Replace with Box::new(name)
                    *last_stmt = quote! { Box::new(#nested_ident) };
                }
            }
        }

        // Clear the subcommand fields context after body generation
        ctx.current_subcommand_fields = None;

        // DEPYLER-0363: Check if ArgumentParser was detected and generate Args struct
        // DEPYLER-0424: Store Args struct and Commands enum in context for module-level emission
        // (hoisted outside function to make Args accessible to handler functions)
        if ctx.argparser_tracker.has_parsers() {
            if let Some(parser_info) = ctx.argparser_tracker.get_first_parser() {
                // DEPYLER-0384: Set flag to include clap dependency in Cargo.toml
                ctx.needs_clap = true;

                // DEPYLER-0399: Generate Commands enum if subcommands exist
                let commands_enum = crate::rust_gen::argparse_transform::generate_commands_enum(
                    &ctx.argparser_tracker,
                );
                if !commands_enum.is_empty() {
                    ctx.generated_commands_enum = Some(commands_enum);
                }

                // Generate the Args struct definition
                let args_struct = crate::rust_gen::argparse_transform::generate_args_struct(
                    parser_info,
                    &ctx.argparser_tracker,
                );
                ctx.generated_args_struct = Some(args_struct);

                // DEPYLER-0108: Inject precompute statements for Option fields
                // This prevents borrow-after-move when Option is passed then checked with is_some()
                let precompute_stmts =
                    crate::rust_gen::argparse_transform::generate_option_precompute(parser_info);
                if !precompute_stmts.is_empty() {
                    // DEPYLER-0108: FIRST post-process body to replace args.<field>.is_some() with has_<field>
                    // This must happen BEFORE injecting precompute statements to avoid replacing them too
                    let option_fields: Vec<String> = parser_info
                        .arguments
                        .iter()
                        .filter(|arg| arg.rust_type().starts_with("Option<"))
                        .map(|arg| arg.rust_field_name().to_string())
                        .collect();

                    if !option_fields.is_empty() {
                        body_stmts = body_stmts
                            .into_iter()
                            .map(|stmt| {
                                let mut stmt_str = stmt.to_string();
                                for field in &option_fields {
                                    // Replace "args . <field> . is_some ()" with "has_<field>"
                                    let pattern = format!("args . {} . is_some ()", field);
                                    let replacement = format!("has_{}", field);
                                    stmt_str = stmt_str.replace(&pattern, &replacement);
                                    // Also handle is_none
                                    let pattern_none = format!("args . {} . is_none ()", field);
                                    let replacement_none = format!("! has_{}", field);
                                    stmt_str = stmt_str.replace(&pattern_none, &replacement_none);
                                }
                                syn::parse_str(&stmt_str).unwrap_or(stmt)
                            })
                            .collect();
                    }

                    // THEN inject precompute statements after replacement
                    // Find the Args::parse() statement index and insert after it
                    // The parse() call is typically the first statement in main()
                    let insert_idx = body_stmts
                        .iter()
                        .position(|s| s.to_string().contains("Args :: parse"))
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    for (offset, stmt) in precompute_stmts.into_iter().enumerate() {
                        body_stmts.insert(insert_idx + offset, stmt);
                    }
                }

                // Note: ArgumentParser-related statements are filtered in stmt_gen.rs
                // parse_args() calls are transformed in stmt_gen.rs::codegen_assign_stmt
            }

            // DO NOT clear tracker yet - we need it for parameter type resolution
            // It will be cleared after all functions are generated
        }

        // DEPYLER-0425: Wrap handler functions with subcommand pattern matching
        // If this function accesses subcommand-specific fields, wrap body in pattern matching
        // DEPYLER-0914: Skip wrapping when in_cmd_handler is true - fields are already parameters
        // In cmd_* handlers, expr_gen transforms args.field → field, so we don't need
        // the if let pattern to extract fields from args.command
        if let Some((variant_name, fields)) = subcommand_info {
            if !ctx.in_cmd_handler {
                // Get args parameter name (first parameter)
                if let Some(args_param) = self.params.first() {
                    let args_param_name = args_param.name.as_ref();
                    // Wrap body statements in pattern matching to extract fields from enum variant
                    body_stmts = crate::rust_gen::argparse_transform::wrap_body_with_subcommand_pattern(
                        body_stmts,
                        &variant_name,
                        &fields,
                        args_param_name,
                    );
                }
            }
        }

        // DEPYLER-0270: Add Ok(()) for functions with Result<(), E> return type
        // When Python function has `-> None` but uses fallible operations (e.g., indexing),
        // the Rust return type becomes `Result<(), IndexError>` and needs Ok(()) at the end
        // Only add Ok(()) if the function doesn't already end with a return statement
        //
        // DEPYLER-0450: Extended to handle all Result return types, not just Type::None
        // This fixes functions with side effects that use error handling (raise/try/except)
        // Also handles Type::Unknown (functions without type annotations that don't explicitly return)
        //
        // DEPYLER-0455 Bug 6: Check if last statement always returns (including try-except)
        // Validator functions with try-except that return in all branches should not get Ok(())
        // Use stmt_always_returns() instead of simple Return check to handle exhaustive returns
        if can_fail {
            let needs_ok = self
                .body
                .last()
                .is_none_or(|stmt| !stmt_always_returns(stmt));
            if needs_ok {
                // For functions returning unit type (or Unknown which defaults to unit), add Ok(())
                // For functions returning values with explicit returns, they already have Ok() wrapping
                if matches!(self.ret_type, Type::None | Type::Unknown) {
                    body_stmts.push(parse_quote! { Ok(()) });
                }
            }
        }

        // Add documentation and custom attributes
        let attrs = codegen_function_attrs(
            &self.docstring,
            &self.properties,
            &self.annotations.custom_attributes,
        );

        // Check if function is a generator (contains yield)
        let func_tokens = if self.properties.is_generator {
            codegen_generator_function(
                self,
                &name,
                &generic_params,
                &where_clause,
                &params,
                &attrs,
                &rust_ret_type,
                ctx,
            )?
        } else if self.properties.is_async {
            // DEPYLER-0748: If this is async main(), add #[tokio::main] attribute
            if self.name == "main" {
                ctx.needs_tokio = true;
                quote! {
                    #(#attrs)*
                    #[tokio::main]
                    pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                        #(#body_stmts)*
                    }
                }
            } else {
                quote! {
                    #(#attrs)*
                    pub async fn #name #generic_params(#(#params),*) #return_type #where_clause {
                        #(#body_stmts)*
                    }
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

