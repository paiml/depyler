//! Expression code generation
//!
//! This module handles converting HIR expressions to Rust syn::Expr nodes.
//! It includes the ExpressionConverter for complex expression transformations
//! and the ToRustExpr trait implementation for HirExpr.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
#[cfg(test)]
use crate::rust_gen::expr_analysis;
use crate::rust_gen::keywords;
use crate::rust_gen::numpy_gen; // Phase 3: NumPy→Trueno codegen
#[cfg(test)]
use crate::rust_gen::precedence;
use crate::string_optimization::{StringContext, StringOptimizer};
use anyhow::{bail, Result};
use syn::{self, parse_quote};
mod binary_ops;
mod call_dispatch;
mod call_generic;
mod call_methods;
mod convert_unary_and_call;
mod stdlib_crypto;
mod stdlib_data;
mod stdlib_datetime;
mod stdlib_misc;
mod stdlib_numpy;
mod stdlib_os;
mod stdlib_pathlib;
mod stdlib_subprocess;
mod type_analysis;

pub(crate) struct ExpressionConverter<'a, 'b> {
    pub(crate) ctx: &'a mut CodeGenContext<'b>,
}

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn new(ctx: &'a mut CodeGenContext<'b>) -> Self {
        Self { ctx }
    }

    // DEPYLER-COVERAGE-95: is_rust_keyword and is_non_raw_keyword moved to crate::rust_gen::keywords module
    // DEPYLER-COVERAGE-95: Walrus analysis functions moved to crate::rust_gen::walrus_helpers module
    // Use walrus_helpers::collect_walrus_vars_from_conditions, walrus_helpers::expr_uses_any_var instead

    /// DEPYLER-0792: Generate let bindings for walrus expressions in a condition
    /// Extracts `(length := len(w))` as `let length = w.len() as i32;`
    pub(crate) fn generate_walrus_bindings(
        cond: &HirExpr,
        ctx: &mut CodeGenContext,
    ) -> Result<proc_macro2::TokenStream> {
        let mut bindings = proc_macro2::TokenStream::new();
        Self::collect_walrus_bindings_from_expr(cond, ctx, &mut bindings)?;
        Ok(bindings)
    }

    /// DEPYLER-0792: Helper to recursively extract walrus bindings from expression
    pub(crate) fn collect_walrus_bindings_from_expr(
        expr: &HirExpr,
        ctx: &mut CodeGenContext,
        bindings: &mut proc_macro2::TokenStream,
    ) -> Result<()> {
        match expr {
            HirExpr::NamedExpr { target, value } => {
                let var_ident = syn::Ident::new(target, proc_macro2::Span::call_site());
                let value_expr = value.to_rust_expr(ctx)?;
                bindings.extend(quote::quote! { let #var_ident = #value_expr; });
                // Recurse into value in case of nested walrus
                Self::collect_walrus_bindings_from_expr(value, ctx, bindings)?;
            }
            HirExpr::Binary { left, right, .. } => {
                Self::collect_walrus_bindings_from_expr(left, ctx, bindings)?;
                Self::collect_walrus_bindings_from_expr(right, ctx, bindings)?;
            }
            HirExpr::Unary { operand, .. } => {
                Self::collect_walrus_bindings_from_expr(operand, ctx, bindings)?;
            }
            HirExpr::Call { args, kwargs, .. } => {
                for arg in args {
                    Self::collect_walrus_bindings_from_expr(arg, ctx, bindings)?;
                }
                for (_, v) in kwargs {
                    Self::collect_walrus_bindings_from_expr(v, ctx, bindings)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    // DEPYLER-COVERAGE-95: looks_like_option_expr moved to crate::rust_gen::expr_analysis module
    // Use expr_analysis::looks_like_option_expr instead

    /// DEPYLER-0758: Check if HirExpr is a variable that's a borrowed parameter
    /// If so, return the dereferenced version of the syn::Expr
    /// Used to fix E0369 errors when doing arithmetic with reference types (e.g., date subtraction)
    pub(crate) fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // DEPYLER-0934: Handle Python builtin types used as function references
        // When int, float, str, bool are used as arguments (e.g., result.map(int)),
        // convert them to closures that perform the type conversion
        // int → |x| x as i32
        // float → |x| x as f64
        // str → |x| x.to_string()
        // bool → |x| x != 0
        match name {
            "int" => return Ok(parse_quote! { |x| x as i32 }),
            "float" => return Ok(parse_quote! { |x| x as f64 }),
            "str" => return Ok(parse_quote! { |x: &_| x.to_string() }),
            "bool" => return Ok(parse_quote! { |x| x != 0 }),
            _ => {}
        }

        // DEPYLER-0627: Check if variable is an unwrapped Option (inside if-let body)
        // When we're inside `if let Some(ref x_val) = x { ... }`, references to `x`
        // should use `x_val` (the unwrapped inner value) instead
        if let Some(unwrapped_name) = self.ctx.option_unwrap_map.get(name) {
            let ident = if keywords::is_rust_keyword(unwrapped_name) {
                syn::Ident::new_raw(unwrapped_name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(unwrapped_name, proc_macro2::Span::call_site())
            };
            // DEPYLER-0666: Return the unwrapped variable name directly
            // The variable is &T from `if let Some(ref x_val) = x { ... }`
            // Rust will auto-deref &String to &str when needed
            // Don't add .clone() - let the caller handle ownership if needed
            return Ok(parse_quote! { #ident });
        }

        // DEPYLER-1151: Check if variable has been narrowed after a None check
        // Pattern: `if x.is_none() { return }` narrows x to the inner type
        // So we can safely unwrap it in subsequent code
        if self.ctx.narrowed_option_vars.contains(name) {
            // Check if variable is actually an Option type
            if let Some(var_type) = self.ctx.var_types.get(name) {
                if matches!(var_type, Type::Optional(_)) {
                    let ident = if keywords::is_rust_keyword(name) {
                        syn::Ident::new_raw(name, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(name, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #ident.expect("value is None") });
                }
            }
        }

        // DEPYLER-0624: Handle Python's magic dunder variables
        // __file__ gives the path to the current file → file!() macro
        // __name__ gives the module name → "__main__" for main module
        if name == "__file__" {
            return Ok(parse_quote! { file!() });
        }
        if name == "__name__" {
            // In Rust binaries, this is always "__main__"
            // For library code, this would need more sophisticated handling
            return Ok(parse_quote! { "__main__" });
        }

        // Check for special keywords that cannot be raw identifiers
        if keywords::is_non_raw_keyword(name) {
            bail!(
                "Python variable '{}' conflicts with a special Rust keyword that cannot be escaped. \
                 Please rename this variable (e.g., '{}_var' or 'py_{}')",
                name, name, name
            );
        }

        // Inside generators, check if variable is a state variable
        if self.ctx.in_generator && self.ctx.generator_state_vars.contains(name) {
            // Generate self.field for state variables
            let ident = if keywords::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
            Ok(parse_quote! { self.#ident })
        } else {
            // Regular variable - use raw identifier if it's a Rust keyword
            let ident = if keywords::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
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

                // DEPYLER-0713 Part 2: When in JSON context, wrap NUMERIC/BOOL literals with json!()
                // This fixes "expected Value, found i32" errors when Value is legitimately needed
                // NOTE: String literals are NOT wrapped because:
                // 1. They may be arguments to functions expecting &str (like json.loads())
                // 2. String→Value conversion happens via serde_json::Value::from() or .into()
                // DEPYLER-1015: Skip in NASA mode - use std-only types
                if ctx.in_json_context && !ctx.type_mapper.nasa_mode {
                    // Only wrap numeric and boolean literals, not strings
                    let should_wrap =
                        matches!(lit, Literal::Int(_) | Literal::Float(_) | Literal::Bool(_));
                    if should_wrap {
                        ctx.needs_serde_json = true;
                        return Ok(parse_quote! { serde_json::json!(#expr) });
                    }
                }

                Ok(expr)
            }
            HirExpr::Var(name) => converter.convert_variable(name),
            HirExpr::Binary { op, left, right } => converter.convert_binary(*op, left, right),
            HirExpr::Unary { op, operand } => converter.convert_unary(op, operand),
            HirExpr::Call { func, args, kwargs } => converter.convert_call(func, args, kwargs),
            HirExpr::MethodCall {
                object,
                method,
                args,
                kwargs,
            } => {
                // DEPYLER-0391: Handle subprocess.run() with keyword arguments
                // subprocess.run(cmd, capture_output=True, cwd=cwd, check=check)
                // Must handle kwargs here before they're lost
                if let HirExpr::Var(module_name) = &**object {
                    if module_name == "subprocess" && method == "run" {
                        return converter.convert_subprocess_run(args, kwargs);
                    }

                    // DEPYLER-0931: Handle subprocess.Popen() for process management
                    // subprocess.Popen(cmd, shell=True) → Command::new(cmd).spawn()
                    if module_name == "subprocess" && method == "Popen" {
                        return converter.convert_subprocess_popen(args, kwargs);
                    }

                    // Phase 3: NumPy→Trueno codegen
                    // Handle numpy module calls: np.array(), np.dot(), np.sum(), etc.
                    if numpy_gen::is_numpy_module(module_name) {
                        if let Some(result) = converter.try_convert_numpy_call(method, args)? {
                            return Ok(result);
                        }
                    }

                    // DEPYLER-0756: Handle shlex module calls directly in MethodCall dispatch
                    // shlex.split(cmd) → inline shell lexer implementation
                    // This must be handled before falling through to convert_method_call
                    if module_name == "shlex" {
                        if let Some(result) = converter.try_convert_shlex_method(method, args)? {
                            return Ok(result);
                        }
                    }
                }

                // DEPYLER-0583: Handle np.linalg.norm() and other submodule calls
                // Pattern: np.linalg.norm(a) where object is Attribute { value: np, attr: linalg }
                if let HirExpr::Attribute { value, attr } = &**object {
                    if let HirExpr::Var(module_name) = &**value {
                        if numpy_gen::is_numpy_module(module_name) && attr == "linalg" {
                            // Map linalg.norm to norm
                            if let Some(result) = converter.try_convert_numpy_call(method, args)? {
                                return Ok(result);
                            }
                        }
                        // DEPYLER-0593: Handle os.path.join(), os.path.exists(), etc.
                        // Pattern: os.path.join(a, b) where object is Attribute { value: os, attr: path }
                        if module_name == "os" && attr == "path" {
                            if let Some(result) =
                                converter.try_convert_os_path_method(method, args)?
                            {
                                return Ok(result);
                            }
                        }
                    }
                }

                // DEPYLER-1113: Query Sovereign Type Database for external module method calls
                // When we encounter a call like requests.get(url), look up the return type
                // from the TypeDB to enable downstream type propagation.
                if let HirExpr::Var(module_name) = &**object {
                    if let Some(return_type) = converter
                        .ctx
                        .lookup_external_return_type(module_name, method)
                    {
                        // Store the return type for assignment handling in stmt_gen
                        // This enables: resp = requests.get(url) → resp: Response
                        converter.ctx.last_external_call_return_type = Some(return_type);
                    }

                    // DEPYLER-1136: Handle module alias calls (e.g., ET.fromstring() → ET::fromstring())
                    // When the object is a module alias, generate path notation instead of method notation
                    if converter.ctx.module_aliases.contains_key(module_name) {
                        let module_ident =
                            syn::Ident::new(module_name, proc_macro2::Span::call_site());
                        let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                        let arg_exprs: Vec<syn::Expr> = args
                            .iter()
                            .map(|arg| arg.to_rust_expr(converter.ctx))
                            .collect::<Result<Vec<_>>>()?;
                        return Ok(parse_quote! { #module_ident::#method_ident(#(#arg_exprs),*) });
                    }
                }

                // DEPYLER-0426: Pass kwargs to convert_method_call
                converter.convert_method_call(object, method, args, kwargs)
            }
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
                generators,
            } => converter.convert_list_comp(element, generators),
            HirExpr::Lambda { params, body } => converter.convert_lambda(params, body),
            HirExpr::SetComp {
                element,
                generators,
            } => converter.convert_set_comp(element, generators),
            HirExpr::DictComp {
                key,
                value,
                generators,
            } => converter.convert_dict_comp(key, value, generators),
            HirExpr::Await { value } => converter.convert_await(value),
            HirExpr::Yield { value } => converter.convert_yield(value),
            HirExpr::FString { parts } => converter.convert_fstring(parts),
            HirExpr::IfExpr { test, body, orelse } => converter.convert_ifexpr(test, body, orelse),
            HirExpr::SortByKey {
                iterable,
                key_params,
                key_body,
                reverse_expr,
            } => converter.convert_sort_by_key(iterable, key_params, key_body, reverse_expr),
            HirExpr::GeneratorExp {
                element,
                generators,
            } => converter.convert_generator_expression(element, generators),
            // DEPYLER-0188: Walrus operator (assignment expression)
            // Python: (x := expr) evaluates to expr and assigns to x
            // Rust: { let x = expr; x } or { let x = expr; x.clone() }
            HirExpr::NamedExpr { target, value } => converter.convert_named_expr(target, value),
            // DEPYLER-0188: Dynamic call: handlers[name](args) → (handlers[name])(args)
            HirExpr::DynamicCall { callee, args, .. } => {
                converter.convert_dynamic_call(callee, args)
            }
        }
    }
}

fn int_literal_to_rust_expr(val: i64) -> syn::Expr {
    if val > i64::from(i32::MAX) && val <= i64::from(u32::MAX) {
        let wrapped = val as i32;
        if wrapped == i32::MIN {
            parse_quote! { i32::MIN }
        } else {
            let abs_val = wrapped.unsigned_abs();
            let lit_tok = syn::LitInt::new(&abs_val.to_string(), proc_macro2::Span::call_site());
            parse_quote! { -#lit_tok }
        }
    } else if val > i64::from(u32::MAX) || val < i64::from(i32::MIN) {
        let lit = syn::LitInt::new(&format!("{}i64", val), proc_macro2::Span::call_site());
        parse_quote! { #lit }
    } else {
        let lit = syn::LitInt::new(&val.to_string(), proc_macro2::Span::call_site());
        parse_quote! { #lit }
    }
}

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
    ctx: &CodeGenContext,
) -> syn::Expr {
    let _ = ctx;
    match lit {
        Literal::Int(n) => int_literal_to_rust_expr(*n),
        Literal::Float(f) => {
            // Ensure float literals always have a decimal point
            // f64::to_string() outputs "0" for 0.0, which parses as integer
            let s = f.to_string();
            let float_str = if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            };
            let lit = syn::LitFloat::new(&float_str, proc_macro2::Span::call_site());
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
        Literal::Bytes(b) => {
            // Generate Rust byte array: &[u8] slice from byte values
            // Python: b"hello" → Rust: &[104_u8, 101, 108, 108, 111]
            let byte_str = syn::LitByteStr::new(b, proc_macro2::Span::call_site());
            parse_quote! { #byte_str }
        }
        Literal::Bool(b) => {
            let lit = syn::LitBool::new(*b, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        Literal::None => {
            // DEPYLER-0357: Python None maps to Rust None (for Option types)
            // When Python code uses None explicitly (e.g., in ternary expressions),
            // it should become Rust's None, not ()
            parse_quote! { None }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
