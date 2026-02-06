//! Expression code generation
//!
//! This module handles converting HIR expressions to Rust syn::Expr nodes.
//! It includes the ExpressionConverter for complex expression transformations
//! and the ToRustExpr trait implementation for HirExpr.

#[cfg(feature = "decision-tracing")]
use crate::decision_trace::DecisionCategory;
use crate::hir::*;
use crate::rust_gen::array_initialization; // DEPYLER-REFACTOR-001: Extracted array/range
use crate::rust_gen::builtin_conversions; // DEPYLER-REFACTOR-001: Extracted conversions
use crate::rust_gen::collection_constructors; // DEPYLER-REFACTOR-001: Extracted constructors
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::expr_analysis::{self, get_wrapped_chained_pyops}; // DEPYLER-COVERAGE-95: Use extracted helpers
use crate::rust_gen::keywords; // DEPYLER-COVERAGE-95: Use centralized keywords module
use crate::rust_gen::numpy_gen; // Phase 3: NumPy→Trueno codegen
use crate::rust_gen::precedence; // DEPYLER-COVERAGE-95: Use extracted helpers
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::stdlib_method_gen; // DEPYLER-COVERAGE-95: Extracted stdlib handlers
use crate::rust_gen::type_gen::convert_binop;
use crate::string_optimization::{StringContext, StringOptimizer};
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};
mod stdlib_data;
mod stdlib_datetime;
mod stdlib_subprocess;
mod stdlib_misc;
mod stdlib_numpy;
mod stdlib_os;
mod stdlib_pathlib;

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
    pub(crate) fn deref_if_borrowed_param(
        &self,
        hir_expr: &HirExpr,
        rust_expr: syn::Expr,
    ) -> syn::Expr {
        if let HirExpr::Var(name) = hir_expr {
            if self.ctx.ref_params.contains(name.as_str()) {
                // Dereference the expression: `*target` instead of `target`
                return parse_quote! { *#rust_expr };
            }
        }
        rust_expr
    }

    // DEPYLER-COVERAGE-95: Precedence functions moved to crate::rust_gen::precedence module
    // Use precedence::parenthesize_if_lower_precedence, precedence::get_rust_op_precedence,
    // precedence::get_python_op_precedence instead

    /// DEPYLER-0582: Coerce integer literal to float if other operand is float-typed
    /// Python automatically promotes int to float in arithmetic with floats
    /// e.g., `1 - beta1` where beta1:float → `1.0 - beta1` in Rust
    pub(crate) fn coerce_int_to_float_if_needed(
        &self,
        expr: syn::Expr,
        hir_expr: &HirExpr,
        other_hir: &HirExpr,
    ) -> syn::Expr {
        // Check if other operand is float-typed
        let other_is_float = self.expr_returns_float(other_hir) || self.is_float_var(other_hir);

        // DEPYLER-1072: Coerce integer literals to float when other operand is KNOWN to be float
        // Pattern: `x <= 0` where x is a float variable
        // NOTE: Don't coerce for untyped variables - we can't assume their type
        if let HirExpr::Literal(Literal::Int(val)) = hir_expr {
            // Only coerce if other operand is KNOWN to be float (has explicit type info)
            if other_is_float {
                let float_val = *val as f64;
                return parse_quote! { #float_val };
            }
        }

        if !other_is_float {
            return expr;
        }

        // Coerce integer literals to float (this handles non-common values)
        if let HirExpr::Literal(Literal::Int(val)) = hir_expr {
            let float_val = *val as f64;
            return parse_quote! { #float_val };
        }

        // DEPYLER-0694: Coerce integer variables to float when other operand is float
        if self.is_int_var(hir_expr) {
            return parse_quote! { (#expr as f64) };
        }

        // DEPYLER-0805: Coerce binary expressions of integers to float
        // Example: (i + 1) * dx where i is Int and dx is Float
        // The result of (i + 1) is also Int and needs casting
        if self.is_int_expr(hir_expr) {
            return parse_quote! { ((#expr) as f64) };
        }

        expr
    }

    /// DEPYLER-0805: Check if expression evaluates to an integer type
    /// Handles variables, literals, and binary operations on integers
    pub(crate) fn is_int_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Int)
                } else {
                    false
                }
            }
            HirExpr::Literal(Literal::Int(_)) => true,
            // Binary operations on integers produce integers
            HirExpr::Binary { left, right, op } => {
                // Arithmetic operations between integers return integers
                // (Add, Sub, Mul produce Int if both operands are Int)
                // Division in Python returns Float, so we don't include Div
                if matches!(
                    op,
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv
                ) {
                    self.is_int_expr(left) && self.is_int_expr(right)
                } else {
                    false
                }
            }
            // Unary minus on integer is still integer
            HirExpr::Unary { operand, .. } => self.is_int_expr(operand),
            _ => false,
        }
    }

    /// Check if expression is a variable with integer type
    pub(crate) fn is_int_var(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Var(name) = expr {
            if let Some(var_type) = self.ctx.var_types.get(name) {
                if matches!(var_type, Type::Int) {
                    return true;
                }
                if let Type::Custom(s) = var_type {
                    if s == "i32" || s == "i64" || s == "usize" || s == "isize" {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if expression is a variable with float type
    pub(crate) fn is_float_var(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Var(name) = expr {
            if let Some(var_type) = self.ctx.var_types.get(name) {
                if matches!(var_type, Type::Float) {
                    return true;
                }
                if let Type::Custom(s) = var_type {
                    if s == "f64" || s == "f32" {
                        return true;
                    }
                }
            }
            // Heuristic: common float parameter names
            let name_lower = name.to_lowercase();
            if name_lower.contains("beta")
                || name_lower.contains("alpha")
                || name_lower.contains("lr")
                || name_lower.contains("eps")
                || name_lower.contains("rate")
                || name_lower.contains("momentum")
            {
                return true;
            }
            // DEPYLER-0950: Heuristic for colorsys color channel variables
            // DEPYLER-1044: REMOVED - single-letter heuristics are too aggressive
            // "c", "r", "g", etc. often appear as loop counters or general-purpose vars
            // Better to require explicit type annotation for float variables
            // Previously caused false positives: c in test_cse.py nested_expressions
        }
        false
    }

    /// DEPYLER-1053: Infer element type from an iterable expression
    /// Used to propagate element types to lambda parameters in filter/map builtins
    /// so that comparison type coercion works correctly.
    ///
    /// Example: `filter(lambda x: x != 0, data)` where `data: list[float]`
    /// We need to know `x` is `Float` to coerce `0` to `0.0`
    pub(crate) fn infer_iterable_element_type(&self, iterable: &HirExpr) -> Option<Type> {
        match iterable {
            // Variable: look up type and extract element type from List/Set
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    match var_type {
                        Type::List(elem_t) => return Some(*elem_t.clone()),
                        Type::Set(elem_t) => return Some(*elem_t.clone()),
                        // For custom types that look like vectors, assume Float element
                        Type::Custom(s) if s.contains("Vec<f64>") => return Some(Type::Float),
                        Type::Custom(s) if s.contains("Vec<f32>") => return Some(Type::Float),
                        _ => {}
                    }
                }
                None
            }
            // Attribute: look up in class_field_types for self.field
            HirExpr::Attribute { attr, value, .. } => {
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self") {
                    if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                        match field_type {
                            Type::List(elem_t) => return Some(*elem_t.clone()),
                            Type::Set(elem_t) => return Some(*elem_t.clone()),
                            _ => {}
                        }
                    }
                }
                None
            }
            // List literal: infer from first element
            HirExpr::List(elems) => {
                if let Some(first) = elems.first() {
                    match first {
                        HirExpr::Literal(Literal::Float(_)) => return Some(Type::Float),
                        HirExpr::Literal(Literal::Int(_)) => return Some(Type::Int),
                        HirExpr::Literal(Literal::String(_)) => return Some(Type::String),
                        HirExpr::Literal(Literal::Bool(_)) => return Some(Type::Bool),
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// DEPYLER-0465: Add & to borrow a path expression if it's a simple variable
    /// This prevents moving String parameters in PathBuf::from() and File::open()
    ///
    /// # Complexity
    /// ≤10 (simple match pattern)
    pub(crate) fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
        match expr {
            // If it's a simple path (variable), add &
            syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
                parse_quote! { &#expr }
            }
            // Otherwise, use as-is (literals, method calls, etc.)
            _ => expr.clone(),
        }
    }

    /// DEPYLER-0541: Handle borrowing for potentially Option-typed path variables
    /// When path variable is Option<String>, use .as_ref().unwrap() for file operations
    pub(crate) fn borrow_path_with_option_check(
        &self,
        path_expr: &syn::Expr,
        hir_arg: &HirExpr,
    ) -> syn::Expr {
        // Check if the HIR arg is a variable that might be Option-typed
        if let HirExpr::Var(var_name) = hir_arg {
            // DEPYLER-0644: Check if variable is already unwrapped (inside if-let body)
            // If so, the variable is already a concrete String, not Option<String>
            // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
            // e.g., hash_algorithm_val from `if let Some(ref hash_algorithm_val) = hash_algorithm`
            let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name)
                || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
            if is_unwrapped {
                // Variable was already unwrapped, just borrow it
                return Self::borrow_if_needed(path_expr);
            }
            // Check if variable is Option-typed
            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                // DEPYLER-0571: PathBuf/Path types are NOT Optional, just borrow them
                if matches!(var_type, Type::Custom(ref s) if s == "PathBuf" || s == "Path") {
                    return Self::borrow_if_needed(path_expr);
                }
                if matches!(var_type, Type::Optional(_)) {
                    // Option<String> → use .as_ref().expect() for path
                    return parse_quote! { #path_expr.as_ref().expect("value is None") };
                }
            }
            // DEPYLER-0541: Heuristic for common optional file path PARAMETER names
            // DEPYLER-0571: Only apply to parameters, not local variables created from unwrapped Options
            // Variables like output_path that are created from PathBuf::from() are NOT Option-typed
            // This heuristic should only apply to function parameters that might be optional
            // Removed output_path as it's commonly a local PathBuf variable, not an Optional parameter
            if matches!(
                var_name.as_str(),
                "output_file" | "out_file" | "outfile" | "out_path"
            ) && self.ctx.fn_str_params.contains(var_name.as_str())
            {
                return parse_quote! { #path_expr.as_ref().expect("value is None") };
            }
        }
        // Fall back to standard borrow
        Self::borrow_if_needed(path_expr)
    }

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

    pub(crate) fn convert_binary(
        &mut self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
    ) -> Result<syn::Expr> {
        // CITL: Trace binary operation type mapping decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "binop_conversion",
            chosen = &format!("{:?}", op),
            alternatives = ["arithmetic", "comparison", "logical", "bitwise"],
            confidence = 0.95
        );

        // DEPYLER-0496: Check if operands return Result types (need ? operator)
        let left_returns_result = self.expr_returns_result(left);
        let right_returns_result = self.expr_returns_result(right);

        // DEPYLER-0498: Check if operands are Option types (need unwrap for comparisons)
        let left_is_option = self.expr_is_option(left);
        let right_is_option = self.expr_is_option(right);

        let mut left_expr = left.to_rust_expr(self.ctx)?;
        let mut right_expr = right.to_rust_expr(self.ctx)?;

        // DEPYLER-0496: Add ? operator for Result-returning expressions in binary operations
        // Only add ? if we're in a Result-returning context (current function can fail)
        if self.ctx.current_function_can_fail {
            if left_returns_result {
                left_expr = parse_quote! { #left_expr? };
            }
            if right_returns_result {
                right_expr = parse_quote! { #right_expr? };
            }
        }

        // DEPYLER-0498: Unwrap Option types in comparison operations
        // Use unwrap_or with appropriate defaults for comparison
        let is_comparison = matches!(
            op,
            BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq
        );

        // DEPYLER-1109: Universal PyOps Dispatch (NASA Mode)
        // Delegate arithmetic/indexing to PyOps traits to handle type coercion (i32+f64, etc.)
        // Note: Wrap left_expr in parens to handle cast expressions (a as f64).method() is invalid
        if self.ctx.type_mapper.nasa_mode && !is_comparison {
            // DEPYLER-1163: Check if return type expects int for division
            // py_div always returns f64, so we need to cast when int is expected
            let return_expects_int = self
                .ctx
                .current_return_type
                .as_ref()
                .map(crate::rust_gen::func_gen::return_type_expects_int)
                .unwrap_or(false);

            // DEPYLER-E0282-FIX: Add i32 suffix to integer literals to resolve
            // type inference ambiguity with PyOps traits that have multiple impls
            let left_pyops = if let HirExpr::Literal(Literal::Int(n)) = left {
                // Only add suffix for small integers that fit in i32
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    let lit_str = format!("{}i32", n);
                    let lit = syn::LitInt::new(&lit_str, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                } else {
                    left_expr.clone()
                }
            } else {
                left_expr.clone()
            };
            let right_pyops = if let HirExpr::Literal(Literal::Int(n)) = right {
                if *n >= i32::MIN as i64 && *n <= i32::MAX as i64 {
                    let lit_str = format!("{}i32", n);
                    let lit = syn::LitInt::new(&lit_str, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                } else {
                    right_expr.clone()
                }
            } else {
                right_expr.clone()
            };

            // DEPYLER-E0282-FIX: When left operand is also a binary arithmetic expression,
            // add explicit type cast to help Rust infer intermediate types in chains.
            // Pattern: ((a).py_add(b)).py_add(c) fails type inference
            // Fix: ((a).py_add(b) as i32).py_add(c) provides type anchor for intermediate result
            //
            // DEPYLER-STRING-CONCAT-FIX: Skip cast for string concatenation
            // String + String = String, casting to i32 is wrong
            let left_is_string = self.expr_is_string_type(left);
            let right_is_string = self.expr_is_string_type(right);
            let is_string_concat = left_is_string || right_is_string;

            let left_is_chain = if let HirExpr::Binary { op: inner_op, .. } = left {
                matches!(
                    inner_op,
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod
                )
            } else {
                false
            };
            // Add type cast when we detect a chain to help Rust infer intermediate types
            // DEPYLER-STRING-CONCAT-FIX: Skip cast for string operations
            let left_typed: syn::Expr = if left_is_chain && !is_string_concat {
                let return_expects_float = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .map(crate::rust_gen::func_gen::return_type_expects_float)
                    .unwrap_or(false);
                if return_expects_float {
                    parse_quote! { (#left_pyops as f64) }
                } else {
                    parse_quote! { (#left_pyops as i32) }
                }
            } else {
                left_pyops
            };

            match op {
                BinOp::Add => return Ok(parse_quote! { (#left_typed).py_add(#right_pyops) }),
                BinOp::Sub => return Ok(parse_quote! { (#left_typed).py_sub(#right_pyops) }),
                BinOp::Mul => return Ok(parse_quote! { (#left_typed).py_mul(#right_pyops) }),
                BinOp::Div => {
                    // DEPYLER-1163: Cast py_div result to i32 when return type expects int
                    if return_expects_int {
                        return Ok(parse_quote! { ((#left_typed).py_div(#right_pyops) as i32) });
                    } else {
                        return Ok(parse_quote! { (#left_typed).py_div(#right_pyops) });
                    }
                }
                BinOp::Mod => return Ok(parse_quote! { (#left_typed).py_mod(#right_pyops) }),
                _ => {}
            }
        }

        if is_comparison {
            if left_is_option && !right_is_option {
                // Left is Option, right is plain - unwrap left
                left_expr = parse_quote! { #left_expr.unwrap_or_default() };
            }
            if right_is_option && !left_is_option {
                // Right is Option, left is plain - unwrap right for comparison
                // For less-than: unwrap_or(i32::MAX) so None is treated as "very large"
                // For greater-than: unwrap_or(i32::MIN) so None is treated as "very small"
                // For equality: unwrap_or_default()
                match op {
                    BinOp::Lt | BinOp::LtEq => {
                        right_expr = parse_quote! { #right_expr.unwrap_or(i32::MAX) };
                    }
                    BinOp::Gt | BinOp::GtEq => {
                        right_expr = parse_quote! { #right_expr.unwrap_or(i32::MIN) };
                    }
                    _ => {
                        right_expr = parse_quote! { #right_expr.unwrap_or_default() };
                    }
                }
            }

            // DEPYLER-1074: Auto-dereference variables that are references (e.g. from iterators or ref params)
            // This fixes E0308 errors where we compare &T with T (e.g. &i32 == 0)
            let left_is_ref = if let HirExpr::Var(name) = left {
                self.ctx.ref_params.contains(name)
            } else {
                false
            };

            let right_is_ref = if let HirExpr::Var(name) = right {
                self.ctx.ref_params.contains(name)
            } else {
                false
            };

            if left_is_ref && !right_is_ref {
                left_expr = parse_quote! { (*#left_expr) };
            } else if right_is_ref && !left_is_ref {
                right_expr = parse_quote! { (*#right_expr) };
            }

            // DEPYLER-0550: Handle serde_json::Value comparisons
            // When comparing Option<String> (from dict.get()) with serde_json::Value,
            // convert the Value to Option<String> for compatibility
            // Pattern: row.get(col).cloned() == val where val comes from JSON .items()
            let left_is_dict_get =
                matches!(left, HirExpr::MethodCall { method, .. } if method == "get");
            let right_is_json_value = self.is_serde_json_value_expr(right);

            if left_is_dict_get && right_is_json_value {
                // Convert serde_json::Value to Option<String> for comparison
                right_expr = parse_quote! { #right_expr.as_str().map(|s| s.to_string()) };
            }

            // Also handle the reverse case
            let right_is_dict_get =
                matches!(right, HirExpr::MethodCall { method, .. } if method == "get");
            let left_is_json_value = self.is_serde_json_value_expr(left);

            if right_is_dict_get && left_is_json_value {
                left_expr = parse_quote! { #left_expr.as_str().map(|s| s.to_string()) };
            }

            // DEPYLER-0575: Coerce integer literal to float when comparing with float expression
            // DEPYLER-0720: Extended to ALL integer literals, not just 0
            // DEPYLER-0828: Extended to ALL integer expressions (variables, not just literals)
            // DEPYLER-0920: Use f32 literals for trueno/numpy f32 results
            // Example: `self.balance > 0` -> `self.balance > 0.0` when balance is f64
            // Example: `std > 0` -> `std > 0f32` when std is trueno f32 result
            // Example: `x < y` where x:f64, y:i32 -> `x < (y as f64)`
            let left_is_float = self.expr_returns_float(left);
            let right_is_float = self.expr_returns_float(right);
            let left_is_f32 = self.expr_returns_f32(left);
            let right_is_f32 = self.expr_returns_f32(right);

            if left_is_float && !right_is_float {
                if let HirExpr::Literal(Literal::Int(n)) = right {
                    // Integer literal: convert at compile time
                    // DEPYLER-0920: Use f32 for trueno results
                    if left_is_f32 {
                        let float_val = *n as f32;
                        right_expr = parse_quote! { #float_val };
                    } else {
                        let float_val = *n as f64;
                        right_expr = parse_quote! { #float_val };
                    }
                } else {
                    // DEPYLER-0828: Integer variable/expression: cast at runtime
                    if left_is_f32 {
                        right_expr = parse_quote! { (#right_expr as f32) };
                    } else {
                        right_expr = parse_quote! { (#right_expr as f64) };
                    }
                }
            } else if right_is_float && !left_is_float {
                if let HirExpr::Literal(Literal::Int(n)) = left {
                    // Integer literal: convert at compile time
                    // DEPYLER-0920: Use f32 for trueno results
                    if right_is_f32 {
                        let float_val = *n as f32;
                        left_expr = parse_quote! { #float_val };
                    } else {
                        let float_val = *n as f64;
                        left_expr = parse_quote! { #float_val };
                    }
                } else {
                    // DEPYLER-0828: Integer variable/expression: cast at runtime
                    if right_is_f32 {
                        left_expr = parse_quote! { (#left_expr as f32) };
                    } else {
                        left_expr = parse_quote! { (#left_expr as f64) };
                    }
                }
            }

            // DEPYLER-STRING-COMPARE: Handle string comparison type mismatches
            // String >= &str doesn't work (PartialOrd not implemented)
            // String == &String doesn't work directly
            // Convert String operands to &str for comparison
            let is_ordering_compare =
                matches!(op, BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq);

            // Check if left is string index (produces String) and needs .as_str()
            let left_is_string_index =
                matches!(left, HirExpr::Index { base, .. } if self.is_string_base(base));
            // Check if right is string index (produces String) and needs .as_str()
            let right_is_string_index =
                matches!(right, HirExpr::Index { base, .. } if self.is_string_base(base));

            // Check if left is a String-typed variable
            // First check var_types, then fall back to heuristics
            // IMPORTANT: If var_types says it's NOT a string (e.g., Int), don't use heuristic
            let left_is_string_var = if let HirExpr::Var(name) = left {
                // Check explicit type info first
                if let Some(ty) = self.ctx.var_types.get(name) {
                    // If we have explicit type info, use it
                    matches!(ty, Type::String)
                } else {
                    // No type info - use heuristic for char/string variable names
                    // But be conservative: only match these names if right side is a string literal
                    false // Don't use name heuristic alone - too error-prone
                }
            } else {
                false
            };

            // Check if right side is a single-character string literal (like "a", "z")
            // This indicates we're comparing a character variable against a char literal
            let right_is_char_literal = matches!(
                right,
                HirExpr::Literal(Literal::String(s)) if s.len() == 1
            );

            // If comparing a variable with a single-char string literal in ordering comparison,
            // the variable is likely a String that needs .as_str() conversion
            let left_needs_as_str =
                is_ordering_compare && matches!(left, HirExpr::Var(_)) && right_is_char_literal;

            // For ordering comparisons with string expressions, convert to &str
            // because String doesn't implement PartialOrd<&str>
            if is_ordering_compare
                && (left_is_string_index || left_is_string_var || left_needs_as_str)
            {
                left_expr = parse_quote! { (#left_expr).as_str() };
            }
            if is_ordering_compare && right_is_string_index {
                right_expr = parse_quote! { (#right_expr).as_str() };
            }

            // For equality comparisons, handle String == &String case
            // by dereferencing the &String side to get String
            // Right side could be:
            // - A variable (HirExpr::Var)
            // - An attribute like args.target (HirExpr::Attribute)
            let right_is_ref_pattern =
                matches!(right, HirExpr::Var(_)) || matches!(right, HirExpr::Attribute { .. });
            if matches!(op, BinOp::Eq | BinOp::NotEq)
                && left_is_string_index
                && right_is_ref_pattern
            {
                // Right side might be &String from ref pattern - deref to String for comparison
                right_expr = parse_quote! { *#right_expr };
            }

            // DEPYLER-1045: Handle char vs &str comparison
            // When iterating over string.chars(), the loop variable is Rust `char` type.
            // Comparing char with &str doesn't work - need to convert char to String.
            if matches!(op, BinOp::Eq | BinOp::NotEq) {
                let left_is_char_iter = if let HirExpr::Var(name) = left {
                    self.ctx.char_iter_vars.contains(name)
                } else {
                    false
                };
                let right_is_char_iter = if let HirExpr::Var(name) = right {
                    self.ctx.char_iter_vars.contains(name)
                } else {
                    false
                };

                // Convert char to String for comparison
                if left_is_char_iter && !right_is_char_iter {
                    left_expr = parse_quote! { #left_expr.to_string() };
                }
                if right_is_char_iter && !left_is_char_iter {
                    right_expr = parse_quote! { #right_expr.to_string() };
                }
            }
        }

        match op {
            // DEPYLER-REFACTOR-001 Phase 2.7: Delegate to extracted helper
            BinOp::In => self.convert_containment_op(false, left, right, left_expr, right_expr),
            BinOp::NotIn => self.convert_containment_op(true, left, right, left_expr, right_expr),
            // DEPYLER-0926: Vector-Vector addition for trueno
            // trueno Vector doesn't implement Add trait, use method call instead
            BinOp::Add if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.add(&#right_expr).expect("arithmetic overflow") })
            }
            // DEPYLER-0928: Vector + scalar - element-wise addition
            BinOp::Add if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x + #right_expr as f32).collect())
                })
            }
            // DEPYLER-0928: scalar + Vector - element-wise addition (commutative)
            BinOp::Add if self.expr_returns_float(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#right_expr.as_slice().iter().map(|&x| x + #left_expr as f32).collect())
                })
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Add => self.convert_add_op(left, right, left_expr, right_expr, op),
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
            // DEPYLER-0303 Phase 3 Fix #7: Dict merge operator |
            // Python 3.9+ supports d1 | d2 for dictionary merge
            // Translate to: { let mut result = d1; result.extend(d2); result }
            BinOp::BitOr if self.is_dict_expr(left) || self.is_dict_expr(right) => {
                self.ctx.needs_hashmap = true;
                Ok(parse_quote! {
                    {
                        let mut __merge_result = #left_expr.clone();
                        __merge_result.extend(#right_expr.iter().map(|(k, v)| (k.clone(), *v)));
                        __merge_result
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
            // DEPYLER-0575: Vector-scalar subtraction for trueno
            // trueno Vector doesn't implement Sub<f32>, so use as_slice().iter().map()
            BinOp::Sub if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x - #right_expr).collect())
                })
            }
            // DEPYLER-0926: Vector-Vector subtraction for trueno
            // trueno Vector doesn't implement Sub trait, use method call instead
            BinOp::Sub if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.sub(&#right_expr).expect("arithmetic overflow") })
            }
            BinOp::Sub => {
                // Check if we're subtracting from a .len() call to prevent underflow
                if self.is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // Wrap left_expr in parens because it contains a cast: (arr.len() as i32).saturating_sub(x)
                    // Without parens, Rust parses "as i32.saturating_sub" incorrectly
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    // DEPYLER-0758: Dereference borrowed params in arithmetic operations
                    // Fixes E0369: cannot subtract NaiveDate from &NaiveDate
                    let left_deref = self.deref_if_borrowed_param(left, left_expr);
                    let right_deref = self.deref_if_borrowed_param(right, right_expr);

                    // DEPYLER-0582: Coerce int to float if operating with float
                    let left_coerced = self.coerce_int_to_float_if_needed(left_deref, left, right);
                    let right_coerced =
                        self.coerce_int_to_float_if_needed(right_deref, right, left);

                    // DEPYLER-1109: Universal PyOps Dispatch for subtraction
                    if self.ctx.type_mapper.nasa_mode {
                        return Ok(parse_quote! { #left_coerced.py_sub(#right_coerced) });
                    }

                    let rust_op = convert_binop(op)?;
                    Ok(parse_quote! { #left_coerced #rust_op #right_coerced })
                }
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Mul => self.convert_mul_op(left, right, left_expr, right_expr, op),
            // DEPYLER-0575: Vector-scalar division for trueno
            // trueno Vector doesn't implement Div<f32>, so use as_slice().iter().map()
            BinOp::Div if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! {
                    Vector::from_vec(#left_expr.as_slice().iter().map(|&x| x / #right_expr).collect())
                })
            }
            // DEPYLER-0926: Vector-Vector division for trueno
            // trueno Vector doesn't implement Div trait, use method call instead
            BinOp::Div if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.div(&#right_expr).expect("division failed") })
            }
            BinOp::Div => {
                // DEPYLER-0188: Check if this is pathlib Path division (path / "segment")
                // Python: Path(__file__).parent / "file.py"
                // Rust: PathBuf::from(file!()).parent().unwrap().join("file.py")
                if self.is_path_expr(left) {
                    // Convert division to .join() for path concatenation
                    return Ok(parse_quote! { #left_expr.join(#right_expr) });
                }

                // DEPYLER-1109: Universal PyOps Dispatch for division
                if self.ctx.type_mapper.nasa_mode {
                    return Ok(parse_quote! { #left_expr.py_div(#right_expr) });
                }

                // DEPYLER-0658: Check if either operand is a float
                // Rust can't divide i32 by f64 or vice versa - need to cast both to f64
                let left_is_float = self.expr_returns_float(left);
                let right_is_float = self.expr_returns_float(right);
                let has_float_operand = left_is_float || right_is_float;

                // v3.16.0 Phase 2: Python's `/` always returns float
                // Rust's `/` does integer division when both operands are integers
                // Check if we need to cast to float based on return type context
                let needs_float_division = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .map(return_type_expects_float)
                    .unwrap_or(false);

                if needs_float_division || has_float_operand {
                    // Cast both operands to f64 for Python float division semantics
                    // or for mixed int/float operations
                    // DEPYLER-0802: Double-wrap operands to ensure correct operator precedence
                    // Without inner parens: (n - 1 as f64) parses as (n - (1 as f64)) due to `as` precedence
                    // With inner parens: ((n - 1) as f64) correctly casts the entire expression
                    Ok(parse_quote! { ((#left_expr) as f64) / ((#right_expr) as f64) })
                } else {
                    // Regular division (int/int → int, float/float → float)
                    let rust_op = convert_binop(op)?;
                    // DEPYLER-0582: Wrap operands in parens if they have lower precedence
                    let left_wrapped = precedence::parenthesize_if_lower_precedence(left_expr, op);
                    let right_wrapped =
                        precedence::parenthesize_if_lower_precedence(right_expr, op);
                    Ok(syn::Expr::Binary(syn::ExprBinary {
                        attrs: vec![],
                        left: Box::new(left_wrapped),
                        op: rust_op,
                        right: Box::new(right_wrapped),
                    }))
                }
            }
            // DEPYLER-REFACTOR-001 Phase 2.8: Delegate to extracted helper
            BinOp::Pow => self.convert_pow_op(left, right, left_expr, right_expr),
            // DEPYLER-0422: Logical operators need Python truthiness conversion
            // Python: `if a and b:` where a, b are strings/lists/etc.
            // Rust: `if (!a.is_empty()) && (!b.is_empty())`
            BinOp::And | BinOp::Or => {
                // DEPYLER-0633: For Option or default pattern, use unwrap_or instead of ||
                // Python: path = env.get("KEY") or "default"
                // Rust: path = env.get("KEY").unwrap_or("default")
                if matches!(op, BinOp::Or) && expr_analysis::looks_like_option_expr(left) {
                    // The right side is the default value - convert to unwrap_or
                    return Ok(parse_quote! { #left_expr.unwrap_or(#right_expr.to_string()) });
                }

                // DEPYLER-0786: Python `or` returns first truthy value, not a boolean
                // For strings: `value or default` → `if value.is_empty() { default } else { value }`
                // This preserves the string type instead of returning bool
                if matches!(op, BinOp::Or) {
                    let left_is_string = self.expr_is_string_type(left);
                    let right_is_string = self.expr_is_string_type(right)
                        || matches!(right, HirExpr::Literal(Literal::String(_)));

                    // DEPYLER-0786: If right is a string literal, assume left is also string
                    // This handles cases where function parameters aren't tracked in var_types
                    // Example: `email or ""` where email: &str is a function parameter
                    let infer_left_from_right =
                        matches!(right, HirExpr::Literal(Literal::String(_)));

                    if (left_is_string || infer_left_from_right) && right_is_string {
                        // Generate: if left.is_empty() { right } else { left }
                        // Need to clone left_expr since we use it twice
                        return Ok(
                            parse_quote! { if #left_expr.is_empty() { #right_expr.to_string() } else { #left_expr.to_string() } },
                        );
                    }
                }

                // DEPYLER-1127: Python `or`/`and` are VALUE-RETURNING, not boolean operators
                // Python: x or y → returns x if truthy, else y (not True/False!)
                // Python: x and y → returns x if falsy, else y (not True/False!)
                // This is fundamentally different from Rust's || and && which return bool.
                //
                // Pattern: `wait = get_time() or 0.1` should return the time or 0.1, not bool
                // Pattern: `result = data and process(data)` should return data or process result
                //
                // We detect non-boolean operands and generate value-returning if-else:
                // x or y → { let _v = x; if _v.is_true() { _v } else { y } }
                // x and y → { let _v = x; if !_v.is_true() { _v } else { y } }
                let left_is_bool_expr = self.expr_is_boolean_expr(left);
                let right_is_bool_expr = self.expr_is_boolean_expr(right);

                // If BOTH operands are boolean expressions, use standard && / ||
                // Otherwise, generate value-returning pattern
                if !left_is_bool_expr || !right_is_bool_expr {
                    // Check if either operand is DepylerValue (needs special handling)
                    let left_is_depyler = self.expr_is_depyler_value(left);
                    let right_is_depyler = self.expr_is_depyler_value(right);
                    let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));
                    let right_is_float_literal =
                        matches!(right, HirExpr::Literal(Literal::Float(_)));

                    // When either side is DepylerValue, wrap the other side too
                    if left_is_depyler || right_is_depyler {
                        // Wrap right-hand side if it's a literal and left is DepylerValue
                        let right_wrapped: syn::Expr = if left_is_depyler && !right_is_depyler {
                            if right_is_int_literal {
                                parse_quote! { DepylerValue::Int(#right_expr as i64) }
                            } else if right_is_float_literal {
                                parse_quote! { DepylerValue::Float(#right_expr as f64) }
                            } else {
                                // Try .into() conversion for other types
                                parse_quote! { DepylerValue::from(#right_expr) }
                            }
                        } else {
                            right_expr.clone()
                        };

                        // Wrap left-hand side if right is DepylerValue and left is not
                        let left_wrapped: syn::Expr = if right_is_depyler && !left_is_depyler {
                            let left_is_int = matches!(left, HirExpr::Literal(Literal::Int(_)));
                            let left_is_float = matches!(left, HirExpr::Literal(Literal::Float(_)));
                            if left_is_int {
                                parse_quote! { DepylerValue::Int(#left_expr as i64) }
                            } else if left_is_float {
                                parse_quote! { DepylerValue::Float(#left_expr as f64) }
                            } else {
                                parse_quote! { DepylerValue::from(#left_expr) }
                            }
                        } else {
                            left_expr.clone()
                        };

                        // Generate value-returning pattern with PyTruthy
                        return match op {
                            BinOp::Or => Ok(parse_quote! {
                                {
                                    let _or_lhs = #left_wrapped;
                                    if _or_lhs.is_true() { _or_lhs } else { #right_wrapped }
                                }
                            }),
                            BinOp::And => Ok(parse_quote! {
                                {
                                    let _and_lhs = #left_wrapped;
                                    if !_and_lhs.is_true() { _and_lhs } else { #right_wrapped }
                                }
                            }),
                            _ => unreachable!(),
                        };
                    }

                    // For non-DepylerValue, non-boolean expressions:
                    // Generate value-returning pattern only for numeric defaults
                    // DEPYLER-1127: If left could be DepylerValue (e.g., from dict.get chain),
                    // we need to wrap the literal in DepylerValue to ensure type match.
                    // Detection of dict chains: .get(), .cloned(), .unwrap_or() patterns
                    let left_might_be_depyler = self.expr_might_be_depyler_value(left);

                    if right_is_int_literal || right_is_float_literal {
                        // If left might be DepylerValue, wrap the literal
                        let right_safe: syn::Expr = if left_might_be_depyler {
                            if right_is_int_literal {
                                parse_quote! { DepylerValue::Int(#right_expr as i64) }
                            } else {
                                parse_quote! { DepylerValue::Float(#right_expr as f64) }
                            }
                        } else {
                            right_expr.clone()
                        };

                        return match op {
                            BinOp::Or => Ok(parse_quote! {
                                {
                                    let _or_lhs = #left_expr;
                                    if _or_lhs.is_true() { _or_lhs } else { #right_safe }
                                }
                            }),
                            BinOp::And => Ok(parse_quote! {
                                {
                                    let _and_lhs = #left_expr;
                                    if !_and_lhs.is_true() { _and_lhs } else { #right_safe }
                                }
                            }),
                            _ => unreachable!(),
                        };
                    }
                }

                // Fall through: Apply truthiness conversion to both operands for bool context
                let left_converted = Self::apply_truthiness_conversion(left, left_expr, self.ctx);
                let right_converted =
                    Self::apply_truthiness_conversion(right, right_expr, self.ctx);

                // Generate the logical operator
                match op {
                    BinOp::And => Ok(parse_quote! { (#left_converted) && (#right_converted) }),
                    BinOp::Or => Ok(parse_quote! { (#left_converted) || (#right_converted) }),
                    _ => unreachable!(),
                }
            }
            _ => {
                // DEPYLER-1109: Universal PyOps Dispatch for modulo
                // In NASA mode, use PyOps trait methods for ALL arithmetic operations
                if matches!(op, BinOp::Mod) && self.ctx.type_mapper.nasa_mode {
                    return Ok(parse_quote! { #left_expr.py_mod(#right_expr) });
                }

                let rust_op = convert_binop(op)?;
                // DEPYLER-0339: Construct syn::ExprBinary directly instead of using parse_quote!
                // parse_quote! doesn't properly handle interpolated syn::BinOp values

                // DEPYLER-1051: Coerce int to float for comparison operators
                // Python allows comparing float with int: `x == 0` where x is float
                // Rust requires same types: `x == 0.0`
                let is_comparison = matches!(
                    op,
                    BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
                );
                let (left_coerced, right_coerced) = if is_comparison {
                    (
                        self.coerce_int_to_float_if_needed(left_expr, left, right),
                        self.coerce_int_to_float_if_needed(right_expr, right, left),
                    )
                } else {
                    (left_expr, right_expr)
                };

                // DEPYLER-0576: Parenthesize right side when it's a unary negation
                // Prevents "<-" tokenization issue: x < -20.0 becomes x<- 20.0 without parens
                let right_expr_final = if matches!(
                    right,
                    HirExpr::Unary {
                        op: UnaryOp::Neg,
                        ..
                    }
                ) {
                    parse_quote! { (#right_coerced) }
                } else {
                    right_coerced
                };

                Ok(syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: Box::new(left_coerced),
                    op: rust_op,
                    right: Box::new(right_expr_final),
                }))
            }
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted power operator helper
    ///
    /// Handles Python power operator with type-aware behavior:
    /// - Integer base with positive int exp: base.checked_pow(exp as u32)
    /// - Integer base with negative exp: (base as f64).powf(exp as f64)
    /// - Float base or exp: (base as f64).powf(exp as f64)
    /// - Variables: runtime type selection
    ///
    /// # Complexity: 7
    pub(crate) fn convert_pow_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        // CITL: Trace power operation type decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "pow_operation",
            chosen = "runtime_dispatch",
            alternatives = ["checked_pow", "powf_float", "powi_int"],
            confidence = 0.82
        );

        // DEPYLER-0699: Wrap expressions in explicit parentheses to ensure
        // correct operator precedence when casting (as binds tighter than * and +)
        let left_paren = Self::wrap_in_parens(left_expr.clone());
        let right_paren = Self::wrap_in_parens(right_expr.clone());

        match (left, right) {
            // Integer literal base with integer literal exponent
            (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                if *exp < 0 {
                    // Negative exponent: convert to float
                    Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    })
                } else {
                    // Positive integer exponent: use checked_pow
                    // DEPYLER-0405: Cast to i32 for concrete type
                    Ok(parse_quote! {
                        (#left_paren as i32).checked_pow(#right_paren as u32)
                            .expect("Power operation overflowed")
                    })
                }
            }
            // Float literal base: always use .powf()
            // DEPYLER-0408: Cast to f64 for concrete type
            (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // Any base with float exponent: use .powf()
            (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // DEPYLER-1072: Float-typed expression as exponent (e.g., 1.0 / n)
            // Division in Python always returns float, so 1.0/n is float
            _ if self.expr_returns_float(right) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // DEPYLER-1072: Float-typed expression as base
            _ if self.expr_returns_float(left) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            // Variables or complex expressions: runtime type selection
            _ => {
                let target_type = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .and_then(|t| match t {
                        Type::Int => Some(quote! { i32 }),
                        Type::Float => Some(quote! { f64 }),
                        _ => None,
                    })
                    .unwrap_or_else(|| quote! { i32 });

                // DEPYLER-0405: Runtime type selection
                Ok(parse_quote! {
                    {
                        if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                            (#left_paren as i32).checked_pow(#right_paren as u32)
                                .expect("Power operation overflowed")
                        } else {
                            (#left_paren as f64).powf(#right_paren as f64) as #target_type
                        }
                    }
                })
            }
        }
    }

    /// DEPYLER-0699: Wrap expression in explicit parentheses
    /// This ensures correct operator precedence when casting
    /// Uses a block expression { expr } which is guaranteed to not be optimized away
    pub(crate) fn wrap_in_parens(expr: syn::Expr) -> syn::Expr {
        // DEPYLER-0707: Construct block directly instead of using parse_quote!
        // parse_quote! re-parses tokens which can fail with complex expressions
        // that have unusual token spacing (e.g., "u32 :: MAX" instead of "u32::MAX")
        syn::Expr::Block(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: syn::Block {
                brace_token: syn::token::Brace::default(),
                stmts: vec![syn::Stmt::Expr(expr, None)],
            },
        })
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted multiplication operator helper
    ///
    /// Handles Python multiplication with type-aware behavior:
    /// - String repetition: "abc" * 3 → "abc".repeat(3)
    /// - Array creation: [0] * 5 → [0; 5]
    /// - Arithmetic multiplication: a * b
    ///
    /// # Complexity: 7
    pub(crate) fn convert_mul_op(
        &mut self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        // DEPYLER-0302: String repetition
        // DEPYLER-0908: Resolved false positive when variable could be either string or int
        // ONLY use .repeat() when one side is DEFINITELY a string LITERAL
        // Variables are NEVER treated as strings for multiplication because:
        // 1. var_types can have stale type info from different branches
        // 2. It's safer to generate `*` which will fail at compile time if wrong
        //    than to generate `.repeat()` which produces wrong semantics silently
        let left_is_string_literal = matches!(left, HirExpr::Literal(Literal::String(_)));
        let right_is_string_literal = matches!(right, HirExpr::Literal(Literal::String(_)));
        let left_is_int_literal = matches!(left, HirExpr::Literal(Literal::Int(_)));
        let right_is_int_literal = matches!(right, HirExpr::Literal(Literal::Int(_)));

        // DEPYLER-0908: Only trust literals, not variable type inference
        // This is conservative but correct - produces compile error rather than wrong behavior
        if left_is_string_literal && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_literal {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0950: String literal * int variable (e.g., "=" * width)
        // Safe because string literal is definite, and we verify int variable type
        let right_is_int_var_from_type = if let HirExpr::Var(sym) = right {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };
        let left_is_int_var_from_type = if let HirExpr::Var(sym) = left {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };

        if left_is_string_literal && right_is_int_var_from_type {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var_from_type && right_is_string_literal {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // For variable * literal patterns, check if variable is DEFINITELY not numeric
        // by looking for clear string method calls in its lineage
        let left_is_string_var = if let HirExpr::Var(sym) = left {
            // Only consider it string if we see string-specific patterns
            // NOT from var_types which can be stale across branches
            let name = sym.as_str();
            name == "text" || name == "s" || name == "line" || name.ends_with("_str")
        } else {
            false
        };
        let right_is_string_var = if let HirExpr::Var(sym) = right {
            let name = sym.as_str();
            name == "text" || name == "s" || name == "line" || name.ends_with("_str")
        } else {
            false
        };

        // Variable * int literal - only use repeat if variable name strongly suggests string
        if left_is_string_var && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_string_var {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0950: String var * int var - use explicit type annotations from context
        // This is safer than arbitrary inference because param types come from annotations
        let right_is_int_var = if let HirExpr::Var(sym) = right {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };
        let left_is_int_var = if let HirExpr::Var(sym) = left {
            matches!(self.ctx.var_types.get(sym), Some(crate::hir::Type::Int))
        } else {
            false
        };

        // String-named var * int-typed var → use .repeat()
        if left_is_string_var && right_is_int_var {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_var && right_is_string_var {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // DEPYLER-0817: Byte string repetition
        // Python: b"hello" * n → Rust: b"hello".repeat(n as usize)
        // Returns Vec<u8> which matches Python bytes behavior
        let left_is_bytes = matches!(left, HirExpr::Literal(Literal::Bytes(_)));
        let right_is_bytes = matches!(right, HirExpr::Literal(Literal::Bytes(_)));
        if left_is_bytes && right_is_int_literal {
            return Ok(parse_quote! { #left_expr.repeat(#right_expr as usize) });
        } else if left_is_int_literal && right_is_bytes {
            return Ok(parse_quote! { #right_expr.repeat(#left_expr as usize) });
        }

        // Array creation: [value] * n or n * [value]
        // DEPYLER-1129: Always use Vec for consistency with PyMul trait
        match (left, right) {
            // Pattern: [x] * n (any size → Vec)
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { vec![#elem; #size_lit] })
            }
            // Pattern: n * [x] (any size → Vec)
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 0 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { vec![#elem; #size_lit] })
            }
            // DEPYLER-0579: Pattern: [x] * var (variable size → Vec)
            // Example: [0.0] * n_params → vec![0.0; n_params as usize]
            (HirExpr::List(elts), HirExpr::Var(_)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; #right_expr as usize] })
            }
            // DEPYLER-0579: Pattern: var * [x] (variable size → Vec)
            (HirExpr::Var(_), HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; #left_expr as usize] })
            }
            // DEPYLER-0794: Pattern: [x] * expr (any expression for size → Vec)
            // Example: [True] * (limit + 1) → vec![true; (limit + 1) as usize]
            // Note: Parentheses needed because `as` has lower precedence than arithmetic
            (HirExpr::List(elts), _) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; (#right_expr) as usize] })
            }
            // DEPYLER-0794: Pattern: expr * [x] (any expression for size → Vec)
            (_, HirExpr::List(elts)) if elts.len() == 1 => {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                Ok(parse_quote! { vec![#elem; (#left_expr) as usize] })
            }
            // DEPYLER-0926: Vector-Vector multiplication for trueno
            // trueno Vector doesn't implement Mul trait, use method call instead
            _ if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #left_expr.mul(&#right_expr).expect("multiplication overflow") })
            }
            // DEPYLER-0926: Vector-scalar multiplication for trueno
            // trueno Vector has scale() method for scalar multiplication
            _ if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0926: scalar-Vector multiplication for trueno (commutative)
            _ if self.expr_returns_float(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0928: Vector * integer - convert integer to f32 for scale()
            _ if self.is_numpy_array_expr(left)
                && matches!(right, HirExpr::Literal(Literal::Int(_))) =>
            {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).expect("scale failed") })
            }
            // DEPYLER-0928: integer * Vector - convert integer to f32 for scale()
            _ if matches!(left, HirExpr::Literal(Literal::Int(_)))
                && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).expect("scale failed") })
            }
            // Default multiplication
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0582: Coerce int to float if operating with float
                let left_coerced = self.coerce_int_to_float_if_needed(left_expr, left, right);
                let right_coerced = self.coerce_int_to_float_if_needed(right_expr, right, left);

                // DEPYLER-1109: Universal PyOps Dispatch for multiplication
                // In NASA mode, use PyOps trait methods for ALL arithmetic operations
                // This delegates type coercion to Rust's trait system (robust, compiled)
                // instead of transpiler logic (complex, brittle)
                if self.ctx.type_mapper.nasa_mode {
                    // Use .py_mul() for all multiplication - traits handle i32*f64, etc.
                    return Ok(parse_quote! { #left_coerced.py_mul(#right_coerced) });
                }

                // DEPYLER-0582: Wrap operands in parens if they have lower precedence
                let left_wrapped = precedence::parenthesize_if_lower_precedence(left_coerced, op);
                let right_wrapped = precedence::parenthesize_if_lower_precedence(right_coerced, op);
                Ok(parse_quote! { #left_wrapped #rust_op #right_wrapped })
            }
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.8: Extracted addition operator helper
    ///
    /// Handles Python addition with type-aware behavior:
    /// - List concatenation: iter().chain().cloned().collect()
    /// - String concatenation: format!("{}{}", a, b)
    /// - Arithmetic addition: a + b
    ///
    /// # Complexity: 5
    pub(crate) fn convert_add_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
        op: BinOp,
    ) -> Result<syn::Expr> {
        // DEPYLER-0290/0299/0271: Type-aware list detection
        let is_definitely_list = self.is_list_expr(left) || self.is_list_expr(right);

        let is_list_var = match (left, right) {
            (HirExpr::Var(name), _) | (_, HirExpr::Var(name)) => self
                .ctx
                .var_types
                .get(name)
                .map(|t| matches!(t, Type::List(_)))
                .unwrap_or(false),
            _ => false,
        };

        // DEPYLER-0311: Slice concatenation
        let is_slice_concat =
            matches!(left, HirExpr::Slice { .. }) || matches!(right, HirExpr::Slice { .. });

        // DEPYLER-STRING-CONCAT: String variable detection for concatenation
        // Check if either operand is a String-typed variable
        let is_string_var = match (left, right) {
            (HirExpr::Var(name), _) => self
                .ctx
                .var_types
                .get(name)
                .map(|t| matches!(t, Type::String))
                .unwrap_or(false),
            (_, HirExpr::Var(name)) => self
                .ctx
                .var_types
                .get(name)
                .map(|t| matches!(t, Type::String))
                .unwrap_or(false),
            _ => false,
        };

        // DEPYLER-STRING-CONCAT: Check for str() calls, .to_string(), and string indexing
        let is_str_producing_expr = |expr: &HirExpr| -> bool {
            match expr {
                // str(x) call
                HirExpr::Call { func, .. } if func == "str" => true,
                // x.to_string() method call (not in HIR but detect common patterns)
                HirExpr::MethodCall { method, .. }
                    if method == "to_string" || method == "format" =>
                {
                    true
                }
                // String indexing: text[i] produces a character/String
                // Check if base is a string type variable
                HirExpr::Index { base, .. } => {
                    if let HirExpr::Var(var_name) = base.as_ref() {
                        // Check var_types for String type
                        self.ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::String))
                            .unwrap_or(false)
                            || self.is_string_base(base)
                    } else if let HirExpr::Attribute { attr: _, .. } = base.as_ref() {
                        // args.text, args.prefix etc.
                        self.is_string_base(base)
                    } else {
                        false
                    }
                }
                _ => false,
            }
        };

        // String detection - includes literals, variable types, str() calls, string indexing
        // NOTE: Do NOT use current_return_type here - just because a function returns String
        // doesn't mean all + operations are string concatenation (e.g., loop counter: i = i + 1)
        let is_definitely_string = matches!(left, HirExpr::Literal(Literal::String(_)))
            || matches!(right, HirExpr::Literal(Literal::String(_)))
            || is_string_var
            || is_str_producing_expr(left)
            || is_str_producing_expr(right);

        // DEPYLER-0672: Additional heuristic - check generated expressions for string patterns
        // Detect CSE temp vars from format!() and .unwrap_or_default() patterns
        let left_str = quote! { #left_expr }.to_string();
        let right_str = quote! { #right_expr }.to_string();
        // DEPYLER-0693: Be more precise about string detection
        // unwrap_or_default on array indexing (get()) returns the element default, not necessarily string
        // Only treat as string if we see string-producing patterns like to_string() or format!()
        let has_to_string = left_str.contains("to_string") || right_str.contains("to_string");
        let has_format = left_str.contains("format !") || right_str.contains("format !");
        let looks_like_string = has_format
            || (left_str.contains("_cse_temp_") && has_to_string)
            || (right_str.contains("_cse_temp_") && has_to_string);

        if (is_definitely_list || is_slice_concat || is_list_var) && !is_definitely_string {
            // List/slice concatenation
            Ok(parse_quote! {
                #left_expr.iter().chain(#right_expr.iter()).cloned().collect::<Vec<_>>()
            })
        } else if is_definitely_string || looks_like_string {
            // String concatenation
            Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
        } else {
            // Arithmetic addition
            let rust_op = convert_binop(op)?;
            // DEPYLER-0582: Coerce int to float if operating with float
            let left_coerced = self.coerce_int_to_float_if_needed(left_expr.clone(), left, right);
            let right_coerced = self.coerce_int_to_float_if_needed(right_expr.clone(), right, left);

            // DEPYLER-1064: Type annotate integer literals when used with DepylerValue
            // This prevents Rust's type inference ambiguity with multiple Add impls
            let is_left_depyler = if let HirExpr::Var(name) = left {
                self.ctx.type_mapper.nasa_mode
                    && self.ctx.var_types.get(name).is_some_and(|t| {
                        matches!(t, Type::Unknown)
                            || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                    })
            } else {
                false
            };
            let is_right_depyler = if let HirExpr::Var(name) = right {
                self.ctx.type_mapper.nasa_mode
                    && self.ctx.var_types.get(name).is_some_and(|t| {
                        matches!(t, Type::Unknown)
                            || matches!(t, Type::Custom(n) if n == "Any" || n == "object")
                    })
            } else {
                false
            };

            // DEPYLER-1109: Universal PyOps Dispatch
            // In NASA mode, use PyOps trait methods for ALL arithmetic operations
            // This delegates type coercion to Rust's trait system (robust, compiled)
            // instead of transpiler logic (complex, brittle)
            if self.ctx.type_mapper.nasa_mode {
                // Use .py_add() for all arithmetic - traits handle i32+f64, String+&str, etc.
                return Ok(parse_quote! { #left_coerced.py_add(#right_coerced) });
            }

            // If one side is DepylerValue and other is int literal, type annotate the literal
            let final_left =
                if is_right_depyler && matches!(left, HirExpr::Literal(Literal::Int(_))) {
                    // Annotate left as i64 for DepylerValue's Add<i64>
                    parse_quote! { #left_coerced as i64 }
                } else {
                    left_coerced
                };
            let final_right =
                if is_left_depyler && matches!(right, HirExpr::Literal(Literal::Int(_))) {
                    // Annotate right as i64 for DepylerValue's Add<i64>
                    parse_quote! { #right_coerced as i64 }
                } else {
                    right_coerced
                };

            Ok(parse_quote! { #final_left #rust_op #final_right })
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.7: Extracted containment operator helper
    ///
    /// Handles `In` and `NotIn` binary operators with type-aware method selection.
    /// - String: .contains(&value)
    /// - Set: .contains(&value)
    /// - List: .contains(&value)
    /// - Dict/HashMap: .get(&key).is_some()
    ///
    /// # Arguments
    /// * `negate` - true for NotIn operator, false for In operator
    /// * `left` - HIR expression for the left operand (for os.environ detection)
    /// * `right` - HIR expression for the right operand (container, for type detection)
    /// * `left_expr` - Generated Rust expression for left operand
    /// * `right_expr` - Generated Rust expression for right operand
    ///
    /// # Complexity: 6
    pub(crate) fn convert_containment_op(
        &self,
        negate: bool,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0380 #3: Handle `var in os.environ` / `var not in os.environ`
        if let HirExpr::Attribute { value, attr } = right {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    // os.environ maps to std::env::var().is_ok()
                    return if negate {
                        Ok(parse_quote! { !std::env::var(#left_expr).is_ok() })
                    } else {
                        Ok(parse_quote! { std::env::var(#left_expr).is_ok() })
                    };
                }
            }
        }

        // DEPYLER-0964: Handle containment check on &mut Option<HashMap<K, V>> parameters
        // When checking `key in memo` where memo is a mut_option_dict_param,
        // we need to unwrap the Option first: memo.as_ref().unwrap().get(&key).is_some()
        if let HirExpr::Var(var_name) = right {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let needs_borrow = match left {
                    HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
                    HirExpr::Literal(Literal::String(_)) => false,
                    _ => true,
                };
                if negate {
                    if needs_borrow {
                        return Ok(
                            parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_none() },
                        );
                    } else {
                        return Ok(
                            parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_none() },
                        );
                    }
                } else if needs_borrow {
                    return Ok(
                        parse_quote! { #right_expr.as_ref().expect("value is None").get(&#left_expr).is_some() },
                    );
                } else {
                    return Ok(
                        parse_quote! { #right_expr.as_ref().expect("value is None").get(#left_expr).is_some() },
                    );
                }
            }
        }

        // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
        // Dict check must come before string check because some names are ambiguous
        // Use .get().is_some() instead of .contains_key() for compatibility with both
        // HashMap and serde_json::Value types
        if self.is_dict_expr(right) {
            // DEPYLER-0559: Check if left side needs borrowing
            let needs_borrow = match left {
                HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
                HirExpr::Literal(Literal::String(_)) => false,
                _ => true,
            };
            // Dict/HashMap uses .get(&key).is_some() for compatibility
            if negate {
                if needs_borrow {
                    return Ok(parse_quote! { #right_expr.get(&#left_expr).is_none() });
                } else {
                    return Ok(parse_quote! { #right_expr.get(#left_expr).is_none() });
                }
            } else if needs_borrow {
                return Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() });
            } else {
                return Ok(parse_quote! { #right_expr.get(#left_expr).is_some() });
            }
        }

        // DEPYLER-0321: Type-aware container detection
        let is_string = self.is_string_type(right);
        let is_set = self.is_set_expr(right) || self.is_set_var(right);
        let is_list = self.is_list_expr(right);
        // DEPYLER-0618: Detect tuple expressions for containment check
        let is_tuple = matches!(right, HirExpr::Tuple(_));

        // DEPYLER-0559: Check if left side is already a borrowed &str
        // &str params and string literals don't need additional borrowing
        let needs_borrow = match left {
            HirExpr::Var(var_name) => !self.is_borrowed_str_param(var_name),
            HirExpr::Literal(Literal::String(_)) => false, // String literals are &str, no borrow needed
            _ => true, // Other expressions typically need borrowing
        };

        // DEPYLER-0618: Handle tuple containment check
        // Python: x in ("a", "b", "c") → Rust: [a, b, c].contains(&x)
        // Tuples don't have .contains() or .get(), so wrap in array slice and use .contains()
        // The right_expr is already converted, e.g., ("a".to_string(), "b".to_string())
        // We convert tuple (a, b, c) to [a, b, c] by string manipulation
        if is_tuple {
            // Convert tuple expression to array slice for .contains() support
            let right_str = right_expr.to_token_stream().to_string();
            // Replace outer parens with brackets: (a, b) → [a, b]
            let array_str = if right_str.starts_with('(') && right_str.ends_with(')') {
                format!("[{}]", &right_str[1..right_str.len() - 1])
            } else {
                format!("[{}]", right_str)
            };
            if let Ok(array_expr) = syn::parse_str::<syn::Expr>(&array_str) {
                if negate {
                    if needs_borrow {
                        return Ok(parse_quote! { !#array_expr.contains(&#left_expr) });
                    } else {
                        return Ok(parse_quote! { !#array_expr.contains(#left_expr) });
                    }
                } else if needs_borrow {
                    return Ok(parse_quote! { #array_expr.contains(&#left_expr) });
                } else {
                    return Ok(parse_quote! { #array_expr.contains(#left_expr) });
                }
            }
            // If parsing fails, fall through to default
        }

        if is_string || is_set || is_list {
            // DEPYLER-0555: For list contains with strings, use .iter().any(|s| s == value)
            // This handles both Vec<String>.contains(&str) and Vec<&str>.contains(&&str) correctly
            // because String implements PartialEq<str> and PartialEq<&str>
            //
            // Detect if right side is a list that likely contains strings:
            // - List literal with string elements
            // - Variable that could be Vec<String>
            let is_string_list = if let HirExpr::List(elems) = right {
                // Check if first element is a string literal (heuristic for list type)
                elems
                    .first()
                    .is_some_and(|e| matches!(e, HirExpr::Literal(Literal::String(_))))
            } else {
                false
            };

            // Use .iter().any() for string lists (handles &str vs String type mismatches)
            if is_list && is_string_list {
                // Use .iter().any() which works with mixed String/&str types
                if negate {
                    Ok(parse_quote! { !#right_expr.iter().any(|s| s == #left_expr) })
                } else {
                    Ok(parse_quote! { #right_expr.iter().any(|s| s == #left_expr) })
                }
            } else if is_string || is_set {
                // DEPYLER-0200: For string contains, use raw string literal or &* for Pattern trait
                // String literals should not have .to_string() added when used as patterns
                let pattern: syn::Expr = if is_string {
                    match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        HirExpr::Var(var_name) if self.ctx.char_iter_vars.contains(var_name) => {
                            // DEPYLER-1045: char can be passed directly to String.contains()
                            // No &* dereference needed - char implements Pattern trait
                            left_expr.clone()
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String (&*String -> &str)
                            // and &str (&*&str -> &str), avoiding unstable str_as_str feature
                            parse_quote! { &*#left_expr }
                        }
                    }
                } else if needs_borrow {
                    parse_quote! { &#left_expr }
                } else {
                    left_expr.clone()
                };

                // String and Set use .contains(&value)
                if negate {
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                }
            } else {
                // Regular list contains
                if negate {
                    if needs_borrow {
                        Ok(parse_quote! { !#right_expr.contains(&#left_expr) })
                    } else {
                        Ok(parse_quote! { !#right_expr.contains(#left_expr) })
                    }
                } else if needs_borrow {
                    Ok(parse_quote! { #right_expr.contains(&#left_expr) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#left_expr) })
                }
            }
        } else {
            // DEPYLER-0449: Check for serde_json::Value FIRST (dict-like key lookup)
            // Must check before left_is_string because dict keys are also strings
            let right_is_json_value = self.is_serde_json_value_expr(right);
            if right_is_json_value {
                // Dict/HashMap/serde_json::Value uses .get(key).is_some() for compatibility
                if negate {
                    if needs_borrow {
                        return Ok(parse_quote! { !#right_expr.get(&#left_expr).is_some() });
                    } else {
                        return Ok(parse_quote! { !#right_expr.get(#left_expr).is_some() });
                    }
                } else if needs_borrow {
                    return Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() });
                } else {
                    return Ok(parse_quote! { #right_expr.get(#left_expr).is_some() });
                }
            }

            // DEPYLER-0935: Check if left side is a string - if so, this is likely a substring check
            // Python: pattern in entry (where both are strings) → Rust: entry.contains(&*pattern)
            // This handles cases where type inference didn't detect the right side as a string
            let left_is_string = self.is_string_type(left);

            if left_is_string {
                // Substring containment check - use .contains()
                let pattern: syn::Expr = match left {
                    HirExpr::Literal(Literal::String(s)) => {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    }
                    _ => {
                        // Use &* to deref-reborrow for Pattern trait compatibility
                        parse_quote! { &*#left_expr }
                    }
                };
                if negate {
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                }
            } else {
                // DEPYLER-0449: Dict/HashMap uses .get(key).is_some() for compatibility
                // Works for both HashMap and serde_json::Value
                if negate {
                    if needs_borrow {
                        Ok(parse_quote! { !#right_expr.get(&#left_expr).is_some() })
                    } else {
                        Ok(parse_quote! { !#right_expr.get(#left_expr).is_some() })
                    }
                } else if needs_borrow {
                    Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() })
                } else {
                    Ok(parse_quote! { #right_expr.get(#left_expr).is_some() })
                }
            }
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.11: Extracted stdlib type constructors helper
    ///
    /// Handles stdlib type constructors: Path, datetime, date, time, timedelta
    /// Returns Some(result) if handled, None if not a stdlib type constructor.
    ///
    /// # Complexity: 8
    pub(crate) fn try_convert_stdlib_type_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-STDLIB-PATHLIB: Handle Path() constructor
            // DEPYLER-0559: Handle Optional args from argparse (Option<String>)
            "Path" if args.len() == 1 => {
                let path_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                // Check if this is an argparse Optional field (args.field where field is Option<T>)
                let is_optional_arg = if let HirExpr::Attribute { value, attr } = &args[0] {
                    if let HirExpr::Var(var_name) = &**value {
                        // Check if this is args.field pattern with Optional field
                        if var_name == "args" {
                            // Look through parsers for this argument
                            self.ctx
                                .argparser_tracker
                                .get_first_parser()
                                .map(|p| {
                                    p.arguments
                                        .iter()
                                        .find(|a| a.rust_field_name() == *attr)
                                        .map(|a| a.rust_type().starts_with("Option<"))
                                        .unwrap_or(false)
                                })
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_optional_arg {
                    // Unwrap the Option before PathBuf::from
                    Some(Ok(
                        parse_quote! { std::path::PathBuf::from(#path_expr.as_ref().expect("value is None")) },
                    ))
                } else {
                    let borrowed_path = Self::borrow_if_needed(&path_expr);
                    Some(Ok(
                        parse_quote! { std::path::PathBuf::from(#borrowed_path) },
                    ))
                }
            }

            // DEPYLER-STDLIB-DATETIME/1025: Handle datetime constructors
            "datetime" if args.len() >= 3 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let year = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let month = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let day = match args[2].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                if args.len() == 3 {
                    if nasa_mode {
                        // DEPYLER-1067: NASA mode - use DepylerDateTime::new()
                        self.ctx.needs_depyler_datetime = true;
                        Some(Ok(
                            parse_quote! { DepylerDateTime::new(#year as u32, #month as u32, #day as u32, 0, 0, 0, 0) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .expect("invalid date")
                                .and_hms_opt(0, 0, 0)
                                .expect("invalid time")
                        }))
                    }
                } else if args.len() >= 6 {
                    let hour = match args[3].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let minute = match args[4].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let second = match args[5].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    // DEPYLER-1067: Handle optional microsecond argument
                    let microsecond = if args.len() >= 7 {
                        match args[6].to_rust_expr(self.ctx) {
                            Ok(e) => e,
                            Err(e) => return Some(Err(e)),
                        }
                    } else {
                        parse_quote! { 0 }
                    };
                    if nasa_mode {
                        // DEPYLER-1067: NASA mode - use DepylerDateTime::new()
                        self.ctx.needs_depyler_datetime = true;
                        Some(Ok(
                            parse_quote! { DepylerDateTime::new(#year as u32, #month as u32, #day as u32, #hour as u32, #minute as u32, #second as u32, #microsecond as u32) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .expect("invalid date")
                                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                                .expect("invalid time")
                        }))
                    }
                } else {
                    Some(Err(anyhow::anyhow!(
                        "datetime() requires 3 or 6+ arguments"
                    )))
                }
            }
            "datetime" => Some(Err(anyhow::anyhow!(
                "datetime() requires at least 3 arguments (year, month, day)"
            ))),

            // DEPYLER-1025/1066: date(year, month, day) - NASA mode uses DepylerDate struct
            "date" if args.len() == 3 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                } else {
                    // DEPYLER-1066: Mark that we need the DepylerDate struct
                    self.ctx.needs_depyler_date = true;
                }
                let year = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let month = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let day = match args[2].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                if nasa_mode {
                    // DEPYLER-1066: Use DepylerDate::new() instead of raw tuple
                    Some(Ok(
                        parse_quote! { DepylerDate::new(#year as u32, #month as u32, #day as u32) },
                    ))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).expect("invalid date")
                    }))
                }
            }

            // DEPYLER-0938/1025: time() with no args - NASA mode uses tuple
            "time" if args.is_empty() => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                if nasa_mode {
                    Some(Ok(parse_quote! { (0u32, 0u32, 0u32) }))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).expect("invalid time")
                    }))
                }
            }

            // DEPYLER-0938/1025: time(hour) - NASA mode uses tuple
            "time" if args.len() == 1 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                if nasa_mode {
                    Some(Ok(parse_quote! { (#hour as u32, 0u32, 0u32) }))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(#hour as u32, 0, 0).expect("invalid time")
                    }))
                }
            }

            // DEPYLER-1025: time(hour, minute, second) - NASA mode uses tuple
            "time" if args.len() >= 2 => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let minute = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                if args.len() == 2 {
                    if nasa_mode {
                        Some(Ok(parse_quote! { (#hour as u32, #minute as u32, 0u32) }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, 0).expect("invalid time")
                        }))
                    }
                } else {
                    let second = match args[2].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(
                            parse_quote! { (#hour as u32, #minute as u32, #second as u32) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, #second as u32).expect("invalid time")
                        }))
                    }
                }
            }

            // DEPYLER-1025/1068: timedelta(days=..., seconds=...) - NASA mode uses DepylerTimeDelta
            "timedelta" => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                } else {
                    self.ctx.needs_depyler_timedelta = true;
                }
                if args.is_empty() {
                    if nasa_mode {
                        Some(Ok(parse_quote! { DepylerTimeDelta::new(0, 0, 0) }))
                    } else {
                        Some(Ok(parse_quote! { chrono::Duration::zero() }))
                    }
                } else if args.len() == 1 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(
                            parse_quote! { DepylerTimeDelta::new(#days as i64, 0, 0) },
                        ))
                    } else {
                        Some(Ok(parse_quote! { chrono::Duration::days(#days as i64) }))
                    }
                } else if args.len() == 2 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let seconds = match args[1].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(
                            parse_quote! { DepylerTimeDelta::new(#days as i64, #seconds as i64, 0) },
                        ))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::Duration::days(#days as i64) + chrono::Duration::seconds(#seconds as i64)
                        }))
                    }
                } else {
                    None // Let it fall through
                }
            }

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.12: Extracted numeric type constructors helper
    ///
    /// Handles Decimal and Fraction constructors.
    /// Returns Some(result) if handled, None if not a numeric type constructor.
    ///
    /// # Complexity: 7
    pub(crate) fn try_convert_numeric_type_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-STDLIB-DECIMAL: Handle Decimal() constructor
            "Decimal" if args.len() == 1 => {
                self.ctx.needs_rust_decimal = true;
                let arg = &args[0];

                let result = match arg {
                    HirExpr::Literal(Literal::String(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_str(&#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Int(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(parse_quote! { rust_decimal::Decimal::from(#arg_expr) }),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Float(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_f64_retain(#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_str(&(#arg_expr).to_string()).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                };
                Some(result)
            }

            // DEPYLER-STDLIB-FRACTIONS: Handle Fraction() constructor
            "Fraction" if args.len() == 1 => {
                self.ctx.needs_num_rational = true;
                let arg = &args[0];

                let result = match arg {
                    HirExpr::Literal(Literal::String(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(parse_quote! {
                            {
                                let s = #arg_expr;
                                let parts: Vec<&str> = s.split('/').collect();
                                if parts.len() == 2 {
                                    let num = parts[0].trim().parse::<i32>().expect("parse failed");
                                    let denom = parts[1].trim().parse::<i32>().expect("parse failed");
                                    num::rational::Ratio::new(num, denom)
                                } else {
                                    let num = s.parse::<i32>().expect("parse failed");
                                    num::rational::Ratio::from_integer(num)
                                }
                            }
                        }),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Int(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => {
                            Ok(parse_quote! { num::rational::Ratio::from_integer(#arg_expr) })
                        }
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Float(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr as f64).expect("parse failed") },
                        ),
                        Err(e) => Err(e),
                    },
                };
                Some(result)
            }

            "Fraction" if args.len() == 2 => {
                self.ctx.needs_num_rational = true;
                let num_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let denom_expr = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(
                    parse_quote! { num::rational::Ratio::new(#num_expr, #denom_expr) },
                ))
            }

            "Fraction" => Some(Err(anyhow::anyhow!("Fraction() requires 1 or 2 arguments"))),

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.13: Extracted iterator utility call helper
    ///
    /// Handles enumerate, zip, and isinstance calls.
    /// Returns Some(result) if handled, None if not an iterator utility call.
    ///
    /// # Complexity: 6
    pub(crate) fn try_convert_iterator_util_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        match func {
            // DEPYLER-0519: enumerate(items) → items.iter().cloned().enumerate()
            // Use iter().cloned() to preserve original collection (Python doesn't consume)
            // DEPYLER-0305: For file variables, use BufReader for line iteration
            "enumerate" if args.len() == 1 => {
                // Check if arg is a file variable (heuristic based on name)
                let is_file_var = if let HirExpr::Var(var_name) = &args[0] {
                    var_name == "f"
                        || var_name == "file"
                        || var_name == "input"
                        || var_name == "output"
                        || var_name.ends_with("_file")
                        || var_name.starts_with("file_")
                } else {
                    false
                };

                match args[0].to_rust_expr(self.ctx) {
                    Ok(items_expr) => {
                        if is_file_var {
                            // DEPYLER-0305: File iteration with enumerate
                            // DEPYLER-0692: Convert usize index to i32 for Python compatibility
                            self.ctx.needs_bufread = true;
                            Some(Ok(parse_quote! {
                                std::io::BufReader::new(#items_expr)
                                    .lines()
                                    .map(|l| l.unwrap_or_default())
                                    .enumerate()
                                    .map(|(i, x)| (i as i32, x))
                            }))
                        } else {
                            // DEPYLER-0692: Convert usize index to i32 for Python compatibility
                            Some(Ok(
                                parse_quote! { #items_expr.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) },
                            ))
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            }

            // zip(a, b, ...) → a.into_iter().zip(b.into_iter())...
            "zip" if args.len() >= 2 => {
                let arg_exprs: Vec<syn::Expr> = match args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()
                {
                    Ok(exprs) => exprs,
                    Err(e) => return Some(Err(e)),
                };

                // Determine if we should use .into_iter() or .iter()
                let use_into_iter = args.iter().all(|arg| self.is_owned_collection(arg));

                let first = &arg_exprs[0];
                let mut chain: syn::Expr = if use_into_iter {
                    parse_quote! { #first.into_iter() }
                } else {
                    parse_quote! { #first.iter() }
                };

                for arg in &arg_exprs[1..] {
                    chain = if use_into_iter {
                        parse_quote! { #chain.zip(#arg.into_iter()) }
                    } else {
                        parse_quote! { #chain.zip(#arg.iter()) }
                    };
                }

                Some(Ok(chain))
            }

            // isinstance(value, type) → true (Rust's type system guarantees correctness)
            "isinstance" if args.len() == 2 => Some(Ok(parse_quote! { true })),

            _ => None,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.15: Extracted debug format detection helper
    ///
    /// Determines if a HirExpr needs {:?} debug formatting instead of {} display formatting.
    /// Used by print() handler to select appropriate format specifiers.
    ///
    /// Returns true for:
    /// - Collection types (List, Dict, Set, Optional, Unknown)
    /// - Collection literals (list, dict, set, frozenset)
    /// - Function calls that return Result types
    /// - Variables named "value" (heuristic for Option<T>)
    ///
    /// # Complexity: 4
    pub(crate) fn needs_debug_format(&self, hir_arg: &HirExpr) -> bool {
        match hir_arg {
            HirExpr::Var(name) => {
                // DEPYLER-0468: Use debug formatter for collections and Optional types
                let type_based = self
                    .ctx
                    .var_types
                    .get(name)
                    .map(|t| {
                        matches!(
                            t,
                            Type::List(_)
                                | Type::Dict(_, _)
                                | Type::Set(_)
                                | Type::Optional(_)
                                | Type::Unknown
                        )
                    })
                    .unwrap_or(false);

                // Heuristic: "value" often comes from functions returning Option<T>
                let name_based = name == "value";

                type_based || name_based
            }
            // DEPYLER-0600 #6: Comprehension types also produce collections
            HirExpr::List(_)
            | HirExpr::Dict(_)
            | HirExpr::Set(_)
            | HirExpr::FrozenSet(_)
            | HirExpr::ListComp { .. }
            | HirExpr::DictComp { .. }
            | HirExpr::SetComp { .. } => true,
            // DEPYLER-1365: Result-returning calls should be unwrapped, not debug-formatted
            // See try_convert_print_call for unwrap handling
            HirExpr::Call { .. } => false,
            _ => false,
        }
    }

    /// DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
    ///
    /// PathBuf doesn't implement Display trait, so we need to detect it and wrap
    /// with .display() when used in print statements or format strings.
    ///
    /// # Complexity: 4
    pub(crate) fn is_pathbuf_expr(&self, hir_arg: &HirExpr) -> bool {
        match hir_arg {
            HirExpr::Var(name) => {
                // Check var_types for PathBuf/Path type
                self.ctx
                    .var_types
                    .get(name)
                    .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                    .unwrap_or(false)
            }
            HirExpr::MethodCall { object, method, .. } => {
                // Methods that return PathBuf - only match when receiver is PathBuf
                // DEPYLER-0930: `join` on String is different from `join` on PathBuf
                let is_pathbuf_method = matches!(
                    method.as_str(),
                    "parent" | "with_name" | "with_suffix" | "with_stem"
                );
                if is_pathbuf_method {
                    return true;
                }
                // For `join`, check if receiver is PathBuf type
                if method == "join" {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        return self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| {
                                matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                            })
                            .unwrap_or(false);
                    }
                }
                false
            }
            HirExpr::Attribute { value, attr } => {
                // path.parent returns PathBuf
                if matches!(attr.as_str(), "parent") {
                    if let HirExpr::Var(var_name) = value.as_ref() {
                        self.ctx
                            .var_types
                            .get(var_name)
                            .map(|t| {
                                matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path")
                            })
                            .unwrap_or(false)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-REFACTOR-001 Phase 2.16: Extracted numeric type token inference helper
    ///
    /// Infers the numeric type token for sum/aggregate operations based on
    /// the current function's return type context.
    ///
    /// Returns:
    /// - `quote! { i32 }` for Int return type
    /// - `quote! { f64 }` for Float return type
    /// - `quote! { i32 }` as default for other/unknown types
    ///
    /// # Complexity: 2
    pub(crate) fn infer_numeric_type_token(&self) -> proc_macro2::TokenStream {
        self.ctx
            .current_return_type
            .as_ref()
            .and_then(|t| match t {
                Type::Int => Some(quote! { i32 }),
                Type::Float => Some(quote! { f64 }),
                _ => None,
            })
            .unwrap_or_else(|| quote! { i32 })
    }

    /// DEPYLER-REFACTOR-001 Phase 2.17: Extracted print call handler
    ///
    /// Handles Python print() function conversion to Rust println!/eprintln!.
    ///
    /// Features:
    /// - print() with no args → println!()
    /// - print(single_arg) → println!("{}", arg) or println!("{:?}", arg) for debug types
    /// - print(multiple_args) → println!("{} {} ...", arg1, arg2, ...)
    /// - file=sys.stderr kwarg → eprintln! variants
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a print call.
    ///
    /// # Complexity: 5
    pub(crate) fn try_convert_print_call(
        &self,
        func: &str,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        kwargs: &[(String, HirExpr)],
    ) -> Option<Result<syn::Expr>> {
        if func != "print" {
            return None;
        }

        // DEPYLER-0462: Check if file=sys.stderr keyword is present
        let use_stderr = kwargs.iter().any(|(name, value)| {
            name == "file"
                && matches!(value, HirExpr::Attribute {
                    value: attr_value,
                    attr
                } if matches!(&**attr_value, HirExpr::Var(module) if module == "sys") && attr == "stderr")
        });

        // DEPYLER-0945: Process arguments to handle PathBuf.display() correctly
        // This handles both single and multiple arguments uniformly
        let processed_args: Vec<syn::Expr> = args
            .iter()
            .zip(arg_exprs.iter())
            .map(|(hir, syn)| {
                if self.is_pathbuf_expr(hir) {
                    parse_quote! { #syn.display() }
                } else {
                    syn.clone()
                }
            })
            .collect();

        let result = if args.is_empty() {
            // print() with no arguments
            if use_stderr {
                Ok(parse_quote! { eprintln!() })
            } else {
                Ok(parse_quote! { println!() })
            }
        } else if args.len() == 1 {
            // Single argument print
            let needs_debug = args
                .first()
                .map(|a| self.needs_debug_format(a))
                .unwrap_or(false);

            // DEPYLER-1365: Check if argument is a Result-returning call that needs unwrapping
            let is_result_call = matches!(&args[0], HirExpr::Call { func, .. }
                if self.ctx.result_returning_functions.contains(func));

            let arg = &processed_args[0];

            if is_result_call {
                // DEPYLER-1365: Unwrap Result types for semantic parity with Python
                // Python: print(divide(17, 5)) → "3"
                // Rust:   println!("{}", divide(17, 5).unwrap()) → "3"
                if use_stderr {
                    Ok(parse_quote! { eprintln!("{}", #arg.expect("operation failed")) })
                } else {
                    Ok(parse_quote! { println!("{}", #arg.expect("operation failed")) })
                }
            } else {
                let format_str = if needs_debug { "{:?}" } else { "{}" };

                if use_stderr {
                    Ok(parse_quote! { eprintln!(#format_str, #arg) })
                } else {
                    Ok(parse_quote! { println!(#format_str, #arg) })
                }
            }
        } else {
            // Multiple arguments - build format string with per-arg detection
            // DEPYLER-1365: Also handle Result-returning calls by unwrapping
            let format_specs: Vec<&str> = args
                .iter()
                .map(|hir_arg| {
                    if self.needs_debug_format(hir_arg) {
                        "{:?}"
                    } else {
                        "{}"
                    }
                })
                .collect();
            let format_str = format_specs.join(" ");

            // DEPYLER-1365: Process args to unwrap Result-returning calls
            let final_args: Vec<syn::Expr> = args
                .iter()
                .zip(processed_args.iter())
                .map(|(hir_arg, syn_arg)| {
                    if let HirExpr::Call { func, .. } = hir_arg {
                        if self.ctx.result_returning_functions.contains(func) {
                            return parse_quote! { #syn_arg.expect("operation failed") };
                        }
                    }
                    syn_arg.clone()
                })
                .collect();

            if use_stderr {
                Ok(parse_quote! { eprintln!(#format_str, #(#final_args),*) })
            } else {
                Ok(parse_quote! { println!(#format_str, #(#final_args),*) })
            }
        };

        Some(result)
    }

    /// DEPYLER-REFACTOR-001 Phase 2.18: Extracted sum call handler
    ///
    /// Handles Python sum() function conversion to Rust iterator patterns.
    ///
    /// Variants:
    /// - sum(generator_exp) → gen_expr.sum::<T>()
    /// - sum(range(...)) → (range_expr).sum::<T>()
    /// - sum(d.values()) / sum(d.keys()) → optimized iterator chain
    /// - sum(iterable) → iterable.iter().sum::<T>()
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a sum call.
    ///
    /// # Complexity: 6
    pub(crate) fn try_convert_sum_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if func != "sum" || args.len() != 1 {
            return None;
        }

        // DEPYLER-0247: Handle sum(generator_exp) → gen_expr.sum::<T>()
        if matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            let target_type = self.infer_numeric_type_token();
            return Some(Ok(parse_quote! { #gen_expr.sum::<#target_type>() }));
        }

        // DEPYLER-0307: Handle sum(range(...)) → (range_expr).sum::<T>()
        if let HirExpr::Call {
            func: range_func, ..
        } = &args[0]
        {
            if range_func == "range" {
                let range_expr = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let target_type = self.infer_numeric_type_token();
                return Some(Ok(parse_quote! { (#range_expr).sum::<#target_type>() }));
            }
        }

        // DEPYLER-0303: Handle sum(d.values()) and sum(d.keys()) - optimized path
        if let HirExpr::MethodCall {
            object,
            method,
            args: method_args,
            ..
        } = &args[0]
        {
            if (method == "values" || method == "keys") && method_args.is_empty() {
                let object_expr = match object.to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                // DEPYLER-0328: Infer sum type from collection element type
                let target_type = if method == "values" {
                    if let HirExpr::Var(var_name) = object.as_ref() {
                        self.ctx.var_types.get(var_name).and_then(|var_type| {
                            if let Type::Dict(_key_type, value_type) = var_type {
                                match value_type.as_ref() {
                                    Type::Int => Some(quote! { i32 }),
                                    Type::Float => Some(quote! { f64 }),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                } else {
                    None // .keys() typically returns strings
                }
                .unwrap_or_else(|| quote! { i32 });

                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                return Some(Ok(parse_quote! {
                    #object_expr.#method_ident().cloned().sum::<#target_type>()
                }));
            }
        }

        // Default: sum(iterable) → iterable.iter().sum::<T>()
        let iter_expr = match args[0].to_rust_expr(self.ctx) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };
        let target_type = self.infer_numeric_type_token();
        Some(Ok(parse_quote! { #iter_expr.iter().sum::<#target_type>() }))
    }

    /// DEPYLER-REFACTOR-001 Phase 2.19: Extracted min/max call handler
    ///
    /// Handles Python min()/max() function conversion to Rust.
    ///
    /// Variants:
    /// - max(a, b) / min(a, b) → std::cmp::max/min or f64.max/min for floats
    /// - max(iterable) / min(iterable) → iter.max/min().unwrap()
    ///
    /// Returns Some(Ok(expr)) if handled, None if not a min/max call.
    ///
    /// # Complexity: 5
    pub(crate) fn try_convert_minmax_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if func != "max" && func != "min" {
            return None;
        }

        let is_max = func == "max";

        // Handle max(a, b) / min(a, b) with mixed numeric types
        if args.len() == 2 {
            let arg1 = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            let arg2 = match args[1].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            // DEPYLER-0515: Check if either argument is a float literal
            let has_float = matches!(args[0], HirExpr::Literal(Literal::Float(_)))
                || matches!(args[1], HirExpr::Literal(Literal::Float(_)));

            if has_float {
                // Use f64 method call: (a as f64).max/min(b as f64)
                return if is_max {
                    Some(Ok(parse_quote! { (#arg1 as f64).max(#arg2 as f64) }))
                } else {
                    Some(Ok(parse_quote! { (#arg1 as f64).min(#arg2 as f64) }))
                };
            }

            // DEPYLER-1062: Use depyler_min/depyler_max helpers for safe comparison
            // These handle PartialOrd correctly and work with f64/DepylerValue
            // Parenthesize arguments to handle casts safely
            return if is_max {
                Some(Ok(
                    parse_quote! { depyler_max((#arg1).clone(), (#arg2).clone()) },
                ))
            } else {
                Some(Ok(
                    parse_quote! { depyler_min((#arg1).clone(), (#arg2).clone()) },
                ))
            };
        }

        // Handle max(iterable) / min(iterable)
        if args.len() == 1 {
            let iter_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            return if is_max {
                Some(Ok(
                    parse_quote! { *#iter_expr.iter().max().expect("empty collection") },
                ))
            } else {
                Some(Ok(
                    parse_quote! { *#iter_expr.iter().min().expect("empty collection") },
                ))
            };
        }

        None
    }

    /// DEPYLER-REFACTOR-001 Phase 2.20: Extracted any/all call handler
    ///
    /// Handles Python any()/all() function conversion to Rust.
    ///
    /// Variants:
    /// - any(generator_exp) / all(generator_exp) → gen.any/all(|x| x)
    /// - any(iterable) / all(iterable) → iter.any/all(|&x| x)
    ///
    /// Returns Some(Ok(expr)) if handled, None if not an any/all call.
    ///
    /// # Complexity: 4
    pub(crate) fn try_convert_any_all_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
    ) -> Option<Result<syn::Expr>> {
        if (func != "any" && func != "all") || args.len() != 1 {
            return None;
        }

        let is_any = func == "any";

        // Handle any/all with generator expressions - don't call .iter()
        if matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };
            return if is_any {
                Some(Ok(parse_quote! { #gen_expr.any(|x| x) }))
            } else {
                Some(Ok(parse_quote! { #gen_expr.all(|x| x) }))
            };
        }

        // Handle any/all with iterables - need .iter()
        let iter_expr = match args[0].to_rust_expr(self.ctx) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        if is_any {
            Some(Ok(parse_quote! { #iter_expr.iter().any(|&x| x) }))
        } else {
            Some(Ok(parse_quote! { #iter_expr.iter().all(|&x| x) }))
        }
    }

    pub(crate) fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace unary operation decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "unary_operation",
            chosen = &format!("{:?}", op),
            alternatives = ["not_bool", "is_empty", "is_none", "negate"],
            confidence = 0.88
        );

        let operand_expr = operand.to_rust_expr(self.ctx)?;
        match op {
            UnaryOp::Not => {
                // DEPYLER-0266: Check if operand is a collection type
                // For collections (list, dict, set, string), use .is_empty() instead of !
                // because Rust doesn't allow ! operator on non-bool types
                let is_collection = if let HirExpr::Var(var_name) = operand {
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(
                            var_type,
                            Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
                        )
                    } else {
                        false
                    }
                } else if let HirExpr::Attribute { value, attr } = operand {
                    // DEPYLER-0966: Check for self.field collection access (truthiness transformation)
                    // Python: `if not self.heap:` where self.heap is list[int]
                    // Rust: Must use `.is_empty()` instead of `!` for Vec types
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                            matches!(
                                field_type,
                                Type::List(_) | Type::Dict(_, _) | Type::Set(_) | Type::String
                            )
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                // DEPYLER-0767: Check if operand is an Optional type variable
                // Python: `if value:` where value is Optional[T] (e.g., from os.environ.get())
                // Rust: Cannot use ! on Option<T>, need .is_none()
                let is_optional_var = if let HirExpr::Var(var_name) = operand {
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(var_type, Type::Optional(_))
                    } else {
                        false
                    }
                } else if let HirExpr::Attribute { value, attr } = operand {
                    // DEPYLER-0966: Check for self.field Optional access
                    // Python: `if not self.cached_value:` where self.cached_value is Optional[T]
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.ctx.class_field_types.get(attr) {
                            matches!(field_type, Type::Optional(_))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                // DEPYLER-0443: Check if operand is a regex method call returning Option<Match>
                // Python: `if not re.match(...)` or `if not compiled.find(...)`
                // Rust: Cannot use ! on Option<Match>, need .is_none()
                let is_option_returning_call = if let HirExpr::MethodCall {
                    object: _,
                    method,
                    args: _,
                    kwargs: _,
                } = operand
                {
                    // Regex methods that return Option<Match>
                    matches!(method.as_str(), "find" | "search" | "match")
                } else if let HirExpr::Call {
                    func,
                    args: _,
                    kwargs: _,
                } = operand
                {
                    // Module-level regex functions (re.match, re.search, re.find)
                    matches!(func.as_str(), "match" | "search" | "find")
                } else {
                    false
                };

                if is_collection {
                    Ok(parse_quote! { #operand_expr.is_empty() })
                } else if is_optional_var || is_option_returning_call {
                    // DEPYLER-0767: For Optional type variables and Option-returning methods,
                    // use .is_none() instead of !
                    Ok(parse_quote! { #operand_expr.is_none() })
                } else {
                    Ok(parse_quote! { !#operand_expr })
                }
            }
            UnaryOp::Neg => Ok(parse_quote! { -#operand_expr }),
            UnaryOp::Pos => Ok(operand_expr), // No +x in Rust
            UnaryOp::BitNot => Ok(parse_quote! { !#operand_expr }),
        }
    }

    pub(crate) fn convert_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CITL: Trace function call dispatch decision
        trace_decision!(
            category = DecisionCategory::MethodDispatch,
            name = "function_call",
            chosen = func,
            alternatives = ["builtin", "stdlib", "user_defined", "constructor"],
            confidence = 0.90
        );

        // DEPYLER-E0282-FIX: Handle Ok(chained_pyops) and Some(chained_pyops) type inference
        // When generating Ok(expr) or Some(expr) where expr contains chained arithmetic operations
        // like ((a).py_add(b)).py_add(c), Rust can't infer the intermediate types.
        // Fix: Generate Ok({ let _r: T = expr; _r }) to provide explicit type annotation.
        if self.ctx.type_mapper.nasa_mode && (func == "Ok" || func == "Some") && args.len() == 1 {
            // Check if the argument has chained PyOps
            if let Some(inner_expr) = get_wrapped_chained_pyops(&HirExpr::Call {
                func: func.to_string(),
                args: args.to_vec(),
                kwargs: vec![],
            }) {
                // Determine the expected type from the return type context
                let inner_type = self
                    .ctx
                    .current_return_type
                    .as_ref()
                    .and_then(|rt| match rt {
                        Type::Optional(inner) => Some(inner.as_ref()),
                        _ => None,
                    });
                // Also check for explicit type hints on the inner expression
                let ty_tokens: Option<syn::Type> = match inner_type {
                    Some(Type::Int) => Some(parse_quote! { i32 }),
                    Some(Type::Float) => Some(parse_quote! { f64 }),
                    _ => {
                        // Fallback: check if inner expr is arithmetic, use i32 as default
                        if matches!(inner_expr, HirExpr::Binary { .. }) {
                            Some(parse_quote! { i32 })
                        } else {
                            None
                        }
                    }
                };
                if let Some(ty) = ty_tokens {
                    let inner_tokens = inner_expr.to_rust_expr(self.ctx)?;
                    let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
                    return Ok(parse_quote! { #func_ident({ let _r: #ty = #inner_tokens; _r }) });
                }
            }
        }

        // DEPYLER-0608: Transform calls to cmd_*/handle_* handlers in subcommand match arms
        // When calling a handler with `args`, pass the extracted subcommand fields instead
        // Pattern: cmd_list(args) → cmd_list(archive) (where archive is extracted in match pattern)
        if self.ctx.in_subcommand_match_arm
            && (func.starts_with("cmd_") || func.starts_with("handle_"))
            && args.len() == 1
            && matches!(&args[0], HirExpr::Var(v) if v == "args")
        {
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            let field_args: Vec<syn::Expr> = self
                .ctx
                .subcommand_match_fields
                .iter()
                .map(|f| {
                    let field_ident = syn::Ident::new(f, proc_macro2::Span::call_site());
                    parse_quote! { #field_ident }
                })
                .collect();
            return Ok(parse_quote! { #func_ident(#(#field_args),*) });
        }

        // DEPYLER-0382: Handle os.path.join(*parts) starred unpacking
        if func == "__os_path_join_starred" {
            if args.len() != 1 {
                bail!("__os_path_join_starred expects exactly 1 argument");
            }
            let parts = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                if #parts.is_empty() {
                    String::new()
                } else {
                    #parts.join(std::path::MAIN_SEPARATOR_STR)
                }
            });
        }

        // DEPYLER-0382: Handle print(*items) starred unpacking
        if func == "__print_starred" {
            if args.len() != 1 {
                bail!("__print_starred expects exactly 1 argument");
            }
            let items = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! {
                {
                    for item in #items {
                        print!("{} ", item);
                    }
                    println!();
                }
            });
        }

        // DEPYLER-REFACTOR-001 Phase 2.14: Removed redundant zeros/ones/full early handlers
        // These are now handled by the final match block via convert_array_init_call
        // which delegates to array_initialization module for consistent handling

        // DEPYLER-0363: Handle ArgumentParser() → Skip for now, will be replaced with struct generation
        // ArgumentParser pattern requires complex transformation:
        // - Accumulate add_argument() calls
        // - Generate #[derive(Parser)] struct
        // - Replace parse_args() with Args::parse()
        // For now, return unit to make code compile while transformation is implemented
        if func.contains("ArgumentParser") {
            // NOTE: Full argparse implementation requires generating Args struct with clap derives (tracked in DEPYLER-0363)
            // For now, just return unit to allow compilation
            return Ok(parse_quote! { () });
        }

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

        // DEPYLER-0178: Handle filter() with lambda → convert to Rust iterator pattern
        // DEPYLER-0754: Use .iter().cloned() instead of .into_iter() to produce Vec<T> not Vec<&T>
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned() properly clones elements to produce owned iterator.
        if func == "filter" && args.len() == 2 {
            if let HirExpr::Lambda { params, body } = &args[0] {
                if params.len() != 1 {
                    bail!("filter() lambda must have exactly one parameter");
                }
                let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
                let param_ident = crate::rust_gen::keywords::safe_ident(&params[0]);

                // DEPYLER-1053: Infer element type from iterable and add lambda param to var_types
                // This enables type coercion in comparisons like `x != 0` where x is f64
                let elem_type = self.infer_iterable_element_type(&args[1]);
                let param_name = params[0].clone();
                if let Some(ref elem_t) = elem_type {
                    self.ctx
                        .var_types
                        .insert(param_name.clone(), elem_t.clone());
                }

                let body_expr = body.to_rust_expr(self.ctx)?;

                // DEPYLER-1053: Remove lambda param from var_types to avoid polluting context
                if elem_type.is_some() {
                    self.ctx.var_types.remove(&param_name);
                }

                // DEPYLER-1053: Use |&x| pattern because filter() always receives &Item
                // Even with .cloned(), filter's closure parameter is a reference to the owned value
                return Ok(parse_quote! {
                    #iterable_expr.iter().cloned().filter(|&#param_ident| #body_expr)
                });
            }
        }

        // DEPYLER-REFACTOR-001 Phase 2.18: Delegate sum calls to helper
        if let Some(result) = self.try_convert_sum_call(func, args) {
            return result;
        }

        // DEPYLER-0950: Handle max(generator_exp) → generator_exp.max().unwrap_or_default()
        // Iterator::max() returns Option<T>, must unwrap for use in ranges/arithmetic
        if func == "max" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.max().unwrap_or_default() });
        }

        // DEPYLER-0950: Handle min(generator_exp) → generator_exp.min().unwrap_or_default()
        if func == "min" && args.len() == 1 && matches!(args[0], HirExpr::GeneratorExp { .. }) {
            let gen_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #gen_expr.min().unwrap_or_default() });
        }

        // DEPYLER-REFACTOR-001: sorted() and reversed() handlers consolidated
        // to final match block using convert_sorted_builtin/convert_reversed_builtin

        // DEPYLER-0022: Handle memoryview(data) → data (identity/no-op)
        // Rust byte slices (&[u8]) already provide memoryview functionality (zero-copy view)
        // Python's memoryview provides a buffer interface - Rust slices are already references
        if func == "memoryview" && args.len() == 1 {
            return args[0].to_rust_expr(self.ctx);
        }

        // DEPYLER-REFACTOR-001 Phase 2.18: sum handlers removed - now handled by try_convert_sum_call

        // DEPYLER-REFACTOR-001 Phase 2.19: Delegate min/max calls to helper
        if let Some(result) = self.try_convert_minmax_call(func, args) {
            return result;
        }

        // DEPYLER-0248: Handle abs(value) → (value).abs()
        // DEPYLER-0815: Parens required for correct precedence (abs(n - 10) → (n - 10).abs())
        if func == "abs" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { (#value_expr).abs() });
        }

        // DEPYLER-REFACTOR-001 Phase 2.20: Delegate any/all calls to helper
        if let Some(result) = self.try_convert_any_all_call(func, args) {
            return result;
        }

        // DEPYLER-0251: Handle round(value) → value.round() as i32
        // DEPYLER-0357: Add `as i32` cast because Python round() returns int
        // but Rust f64::round() returns f64
        if func == "round" && args.len() == 1 {
            let value_expr = args[0].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #value_expr.round() as i32 });
        }

        // DEPYLER-0252: Handle pow(base, exp) → base.pow(exp as u32)
        // Rust's pow() requires u32 exponent, so we cast
        if func == "pow" && args.len() == 2 {
            let base_expr = args[0].to_rust_expr(self.ctx)?;
            let exp_expr = args[1].to_rust_expr(self.ctx)?;
            return Ok(parse_quote! { #base_expr.pow(#exp_expr as u32) });
        }

        // DEPYLER-REFACTOR-001: chr() and ord() handlers consolidated
        // to final match block using convert_chr_builtin/convert_ord_builtin

        // DEPYLER-0255: Handle bool(value) → type-aware truthiness check
        // DEPYLER-REFACTOR-001: Handles different types correctly
        if func == "bool" && args.len() == 1 {
            let arg = &args[0];
            match arg {
                // String literals: non-empty → true, empty → false
                HirExpr::Literal(Literal::String(s)) => {
                    let is_true = !s.is_empty();
                    return Ok(parse_quote! { #is_true });
                }
                // Integer literals: non-zero → true, zero → false
                HirExpr::Literal(Literal::Int(n)) => {
                    let is_true = *n != 0;
                    return Ok(parse_quote! { #is_true });
                }
                // Float literals: non-zero → true, zero → false
                HirExpr::Literal(Literal::Float(f)) => {
                    let is_true = *f != 0.0;
                    return Ok(parse_quote! { #is_true });
                }
                // Bool literals: identity
                HirExpr::Literal(Literal::Bool(b)) => {
                    return Ok(parse_quote! { #b });
                }
                // Variables: check type
                HirExpr::Var(var_name) => {
                    let value_expr = arg.to_rust_expr(self.ctx)?;
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        return match var_type {
                            Type::String => Ok(parse_quote! { !#value_expr.is_empty() }),
                            Type::Float => Ok(parse_quote! { #value_expr != 0.0 }),
                            Type::List(_) | Type::Set(_) | Type::Dict(_, _) => {
                                Ok(parse_quote! { !#value_expr.is_empty() })
                            }
                            _ => Ok(parse_quote! { #value_expr != 0 }),
                        };
                    }
                    // Default for unknown variables: assume integer-like
                    return Ok(parse_quote! { #value_expr != 0 });
                }
                // Other expressions: default to != 0
                _ => {
                    let value_expr = arg.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { #value_expr != 0 });
                }
            }
        }

        // DEPYLER-REFACTOR-001 Phase 2.12: Delegate numeric type constructors to helper
        // Handles: Decimal, Fraction
        if let Some(result) = self.try_convert_numeric_type_call(func, args) {
            return result;
        }

        // DEPYLER-REFACTOR-001 Phase 2.11: Delegate stdlib type constructors to helper
        // Handles: Path, datetime, date, time, timedelta
        if let Some(result) = self.try_convert_stdlib_type_call(func, args) {
            return result;
        }

        // DEPYLER-REFACTOR-001 Phase 2.13: Delegate iterator utility calls to helper
        // Handles enumerate, zip, isinstance
        if let Some(result) = self.try_convert_iterator_util_call(func, args) {
            return result;
        }

        // DEPYLER-0230: Check if func is a user-defined class before treating as builtin
        let is_user_class = self.ctx.class_names.contains(func);

        // DEPYLER-0234: For user-defined class constructors, convert string literals to String
        // This fixes "expected String, found &str" errors when calling constructors
        // DEPYLER-1144: Also coerce list literals when class has Vec<f64> field
        // DEPYLER-1207: Pattern matching correction - use **elem to dereference Box
        let class_has_vec_f64_field = self
            .ctx
            .class_field_types
            .values()
            .any(|t| matches!(t, Type::List(elem) if matches!(**elem, Type::Float)));
        let arg_exprs: Vec<syn::Expr> = if is_user_class {
            args.iter()
                .map(|arg| {
                    // DEPYLER-1144: For list literals when class expects Vec<f64>, coerce integers
                    if class_has_vec_f64_field {
                        if let HirExpr::List(elems) = arg {
                            return self.convert_list_with_float_coercion(elems);
                        }
                    }
                    // DEPYLER-CLASS-STR-FIX: Add .to_string() for string literals in class constructors
                    // Python dataclass fields with `name: str` are owned Strings in Rust.
                    // String literals need .to_string() to convert &str to String.
                    if let HirExpr::Literal(Literal::String(s)) = arg {
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #lit.to_string() });
                    }
                    arg.to_rust_expr(self.ctx)
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            // DEPYLER-1215/DEPYLER-1218: Convert args with type context for dict and list literals
            // When a dict/list literal is passed to a function expecting Dict/List with Unknown/Any,
            // we need to set current_assign_type to trigger DepylerValue wrapping
            let mut converted_args = Vec::with_capacity(args.len());
            for (param_idx, arg) in args.iter().enumerate() {
                // DEPYLER-1215/DEPYLER-1218: Check if param expects Dict/List with Unknown/Any value type
                let prev_assign_type = if matches!(arg, HirExpr::Dict(_) | HirExpr::List(_)) {
                    if let Some(param_types) = self.ctx.function_param_types.get(func) {
                        if let Some(param_type) = param_types.get(param_idx) {
                            // Check if param is Dict[_, Any/Unknown] or List[Any/Unknown] or bare dict/list
                            let needs_depyler_value = match param_type {
                                Type::Dict(_, val_type) => {
                                    matches!(val_type.as_ref(), Type::Unknown)
                                        || matches!(val_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any")
                                }
                                // DEPYLER-1218: List with Unknown/DepylerValue element type
                                Type::List(elem_type) => {
                                    matches!(elem_type.as_ref(), Type::Unknown)
                                        || matches!(elem_type.as_ref(), Type::Custom(name) if name == "DepylerValue" || name == "Any")
                                }
                                Type::Custom(name) if name == "dict" || name == "Dict" => true,
                                Type::Custom(name) if name == "list" || name == "List" => true,
                                _ => false,
                            };
                            if needs_depyler_value {
                                let old = self.ctx.current_assign_type.clone();
                                self.ctx.current_assign_type = Some(param_type.clone());
                                Some(old)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let expr = arg.to_rust_expr(self.ctx)?;

                // Restore previous assign type if we changed it
                if let Some(old_type) = prev_assign_type {
                    self.ctx.current_assign_type = old_type;
                }

                // DEPYLER-0458: Add & prefix for Lazy const variables (e.g., DEFAULT_CONFIG)
                // When passing a const (all uppercase) to a function, it's likely a Lazy<T>
                // that needs to be borrowed (&) so Deref converts it to &T
                if let HirExpr::Var(var_name) = arg {
                    let is_const = var_name.chars().all(|c| c.is_uppercase() || c == '_');
                    if is_const {
                        converted_args.push(parse_quote! { &#expr });
                        continue;
                    }
                }
                converted_args.push(expr);
            }
            converted_args
        };

        // DEPYLER-0364: Convert kwargs to positional arguments
        // Python: greet(name="Alice", greeting="Hello") → Rust: greet("Alice", "Hello")
        // For now, we append kwargs as additional positional arguments. This works for
        // common cases where functions accept positional or keyword arguments in order.
        // DEPYLER-0477: Future work - look up function signatures to determine
        // the correct parameter order and merge positional + kwargs properly
        let kwarg_exprs: Vec<syn::Expr> = if is_user_class {
            // For user-defined classes, convert string literals to String
            // This prevents "expected String, found &str" errors in constructors
            // DEPYLER-1144: Also coerce list literals to match field types (e.g., [0] → vec![0.0] for Vec<f64>)
            kwargs
                .iter()
                .map(|(name, value)| {
                    // DEPYLER-1144: Check if field expects Vec<f64> and value is list of integers
                    if let Some(Type::List(elem_type)) = self.ctx.class_field_types.get(name) {
                        if matches!(elem_type.as_ref(), Type::Float) {
                            if let HirExpr::List(elems) = value {
                                // Convert list with integer coercion to f64
                                return self.convert_list_with_float_coercion(elems);
                            }
                        }
                    }
                    let expr = value.to_rust_expr(self.ctx)?;
                    if matches!(value, HirExpr::Literal(Literal::String(_))) {
                        Ok(parse_quote! { #expr.to_string() })
                    } else {
                        Ok(expr)
                    }
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            // For built-in functions and regular calls, use standard conversion
            kwargs
                .iter()
                .map(|(_name, value)| value.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?
        };

        // Merge positional args and kwargs (both HIR and converted Rust exprs)
        // This creates a single argument list that will be passed to the function
        let mut all_args = arg_exprs.clone();
        all_args.extend(kwarg_exprs);

        let mut all_hir_args: Vec<HirExpr> = args.to_vec();
        for (_name, value) in kwargs {
            all_hir_args.push(value.clone());
        }

        // DEPYLER-REFACTOR-001 Phase 2.17: Delegate print call to helper
        if let Some(result) = self.try_convert_print_call(func, args, &arg_exprs, kwargs) {
            return result;
        }

        match func {
            // Python built-in type conversions → Rust casting
            "int" => self.convert_int_cast(&all_hir_args, &arg_exprs),
            "float" => self.convert_float_cast(&all_hir_args, &arg_exprs),
            "str" => self.convert_str_conversion(&all_hir_args, &arg_exprs),
            "bool" => self.convert_bool_cast(&all_hir_args, &arg_exprs),
            // Other built-in functions
            // DEPYLER-0659: Handle len() on serde_json::Value
            "len" => self.convert_len_call_with_type(&all_hir_args, &arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            "zeros" | "ones" | "full" => {
                self.convert_array_init_call(func, &all_hir_args, &arg_exprs)
            }
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0171, 0172, 0173, 0174: Collection conversion builtins
            // DEPYLER-0230: Only treat as builtin if not a user-defined class
            // DEPYLER-0751: Pass HIR args to detect string type for .chars()
            "Counter" if !is_user_class => self.convert_counter_builtin(&all_hir_args, &arg_exprs),
            "defaultdict" if !is_user_class => self.convert_defaultdict_builtin(&arg_exprs),
            "dict" if !is_user_class => self.convert_dict_builtin(&arg_exprs),
            "deque" if !is_user_class => self.convert_deque_builtin(&arg_exprs),
            "list" if !is_user_class => self.convert_list_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-0935: bytes() builtin - convert to Vec<u8>
            "bytes" if !is_user_class => self.convert_bytes_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-0936: bytearray() builtin - convert to Vec<u8>
            "bytearray" if !is_user_class => {
                self.convert_bytearray_builtin(&all_hir_args, &arg_exprs)
            }
            // DEPYLER-0937: tuple() builtin - convert iterable to collected tuple-like Vec
            "tuple" if !is_user_class => self.convert_tuple_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Pure builtin functions delegated to extracted module
            // DEPYLER-COVERAGE-95: Extracted to stdlib_method_gen::builtin_functions for testability
            "all" => stdlib_method_gen::builtin_functions::convert_all_builtin(&arg_exprs),
            "any" => stdlib_method_gen::builtin_functions::convert_any_builtin(&arg_exprs),
            "divmod" => stdlib_method_gen::builtin_functions::convert_divmod_builtin(&arg_exprs),
            "enumerate" => {
                stdlib_method_gen::builtin_functions::convert_enumerate_builtin(&arg_exprs)
            }
            "zip" => stdlib_method_gen::builtin_functions::convert_zip_builtin(&arg_exprs),
            "reversed" => {
                stdlib_method_gen::builtin_functions::convert_reversed_builtin(&arg_exprs)
            }
            "sorted" => stdlib_method_gen::builtin_functions::convert_sorted_builtin(&arg_exprs),
            "filter" => self.convert_filter_builtin(&all_hir_args, &arg_exprs),
            "sum" => stdlib_method_gen::builtin_functions::convert_sum_builtin(&arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Final batch for 50% milestone
            "round" => stdlib_method_gen::builtin_functions::convert_round_builtin(&arg_exprs),
            "abs" => stdlib_method_gen::builtin_functions::convert_abs_builtin(&arg_exprs),
            "min" => stdlib_method_gen::builtin_functions::convert_min_builtin(&arg_exprs),
            "max" => stdlib_method_gen::builtin_functions::convert_max_builtin(&arg_exprs),
            "pow" => stdlib_method_gen::builtin_functions::convert_pow_builtin(&arg_exprs),
            "hex" => stdlib_method_gen::builtin_functions::convert_hex_builtin(&arg_exprs),
            "bin" => stdlib_method_gen::builtin_functions::convert_bin_builtin(&arg_exprs),
            "oct" => stdlib_method_gen::builtin_functions::convert_oct_builtin(&arg_exprs),
            // DEPYLER-0579: format(value, spec) builtin - needs HIR for literal extraction
            "format" => self.convert_format_builtin(&arg_exprs, &all_hir_args),
            "chr" => stdlib_method_gen::builtin_functions::convert_chr_builtin(&arg_exprs),
            // ord() needs context for char_iter_vars check
            "ord" => self.convert_ord_builtin(&arg_exprs, &all_hir_args),
            "hash" => stdlib_method_gen::builtin_functions::convert_hash_builtin(&arg_exprs),
            "repr" => stdlib_method_gen::builtin_functions::convert_repr_builtin(&arg_exprs),
            // DEPYLER-0387: File I/O builtin - needs context for needs_io_* flags
            "open" => self.convert_open_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-50: next(), getattr(), iter(), type()
            "next" => stdlib_method_gen::builtin_functions::convert_next_builtin(&arg_exprs),
            "getattr" => self.convert_getattr_builtin(&arg_exprs),
            "iter" => stdlib_method_gen::builtin_functions::convert_iter_builtin(&arg_exprs),
            "type" => stdlib_method_gen::builtin_functions::convert_type_builtin(&arg_exprs),
            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            "isinstance" => Ok(parse_quote! { true }),
            // DEPYLER-1205: E0425 Vocabulary Expansion - input(), hasattr()
            "input" => stdlib_method_gen::builtin_functions::convert_input_builtin(&arg_exprs),
            "hasattr" => stdlib_method_gen::builtin_functions::convert_hasattr_builtin(&arg_exprs),
            "setattr" => stdlib_method_gen::builtin_functions::convert_setattr_builtin(&arg_exprs),
            // GH-204: Additional E0425 Vocabulary Expansion
            "callable" => {
                stdlib_method_gen::builtin_functions::convert_callable_builtin(&arg_exprs)
            }
            "id" => stdlib_method_gen::builtin_functions::convert_id_builtin(&arg_exprs),
            "ascii" => stdlib_method_gen::builtin_functions::convert_ascii_builtin(&arg_exprs),
            "vars" => stdlib_method_gen::builtin_functions::convert_vars_builtin(&arg_exprs),
            "dir" => stdlib_method_gen::builtin_functions::convert_dir_builtin(&arg_exprs),
            "globals" => stdlib_method_gen::builtin_functions::convert_globals_builtin(&arg_exprs),
            "locals" => stdlib_method_gen::builtin_functions::convert_locals_builtin(&arg_exprs),
            "delattr" => stdlib_method_gen::builtin_functions::convert_delattr_builtin(&arg_exprs),
            "staticmethod" => {
                stdlib_method_gen::builtin_functions::convert_staticmethod_builtin(&arg_exprs)
            }
            "classmethod" => {
                stdlib_method_gen::builtin_functions::convert_classmethod_builtin(&arg_exprs)
            }
            "property" => {
                stdlib_method_gen::builtin_functions::convert_property_builtin(&arg_exprs)
            }
            "breakpoint" => {
                stdlib_method_gen::builtin_functions::convert_breakpoint_builtin(&arg_exprs)
            }
            "exit" => stdlib_method_gen::builtin_functions::convert_exit_builtin(&arg_exprs),
            "quit" => stdlib_method_gen::builtin_functions::convert_quit_builtin(&arg_exprs),
            _ => self.convert_generic_call(func, &all_hir_args, &all_args),
        }
    }

    pub(crate) fn try_convert_map_with_zip(
        &mut self,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // DEPYLER-0793: Handle map(str, iterable) → iterable.iter().map(|x| x.to_string())
        // Python's str builtin converts elements to strings, in Rust use .to_string()
        if args.len() == 2 {
            if let HirExpr::Var(func_name) = &args[0] {
                if func_name == "str" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().map(|x| x.to_string())
                    }));
                }
                // DEPYLER-0793: Handle map(int, iterable) → iterable.iter().map(|x| *x as i32)
                // For converting strings to int, this is a simplified version
                if func_name == "int" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().filter_map(|x| x.parse::<i32>().ok())
                    }));
                }
                // DEPYLER-0793: Handle map(float, iterable) → iterable.iter().filter_map(|x| x.parse::<f64>().ok())
                if func_name == "float" {
                    let iterable_expr = args[1].to_rust_expr(self.ctx)?;
                    return Ok(Some(parse_quote! {
                        #iterable_expr.iter().filter_map(|x| x.parse::<f64>().ok())
                    }));
                }
            }
        }

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

            // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
            let param_idents: Vec<syn::Ident> = params
                .iter()
                .map(|p| crate::rust_gen::keywords::safe_ident(p))
                .collect();

            // DEPYLER-1053: Infer element types from iterables and add lambda params to var_types
            // This enables type coercion in comparisons like `x > 0` where x is f64
            let mut added_params: Vec<String> = Vec::new();
            for (i, param) in params.iter().enumerate() {
                if let Some(iterable) = args.get(i + 1) {
                    if let Some(elem_type) = self.infer_iterable_element_type(iterable) {
                        self.ctx.var_types.insert(param.clone(), elem_type);
                        added_params.push(param.clone());
                    }
                }
            }

            // Convert lambda body
            let body_expr = body.to_rust_expr(self.ctx)?;

            // DEPYLER-1053: Remove lambda params from var_types to avoid polluting context
            for param in &added_params {
                self.ctx.var_types.remove(param);
            }

            // Handle based on number of iterables
            if num_iterables == 1 {
                // Single iterable: iterable.iter().map(|&x| ...).collect()
                // DEPYLER-1053: Use |&x| pattern because iter() yields references
                let iter_expr = &iterable_exprs[0];
                let param = &param_idents[0];
                Ok(Some(parse_quote! {
                    #iter_expr.iter().map(|&#param| #body_expr).collect::<Vec<_>>()
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

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    #[allow(dead_code)]
    pub(crate) fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        builtin_conversions::convert_len_call(args)
    }

    /// DEPYLER-0659: Handle len() with type awareness for serde_json::Value
    /// serde_json::Value doesn't have a direct .len() method
    /// - Arrays: use .as_array().map(|a| a.len()).unwrap_or(0)
    /// - Objects: use .as_object().map(|o| o.len()).unwrap_or(0)
    /// - Strings: use .as_str().map(|s| s.len()).unwrap_or(0)
    ///
    /// DEPYLER-DAY2-BUG-002: Handle len() on tuples
    /// Rust tuples don't have .len() method - size is known at compile time
    /// - Tuples: return compile-time constant (e.g., 4 for (i32, i32, i32, i32))
    pub(crate) fn convert_len_call_with_type(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if arg_exprs.len() != 1 || hir_args.is_empty() {
            return builtin_conversions::convert_len_call(arg_exprs);
        }

        let arg = &arg_exprs[0];
        let hir_arg = &hir_args[0];

        // DEPYLER-DAY2-BUG-002: Check if argument is a tuple type
        // Rust tuples don't have .len() - return compile-time constant
        if let HirExpr::Var(name) = hir_arg {
            if let Some(Type::Tuple(types)) = self.ctx.var_types.get(name) {
                let len = types.len() as i32;
                return Ok(parse_quote! { #len });
            }
        }

        // Check if the argument is a JSON Value (NOT a typed HashMap)
        // DEPYLER-0689: Only use as_array/as_object for serde_json::Value, not typed dicts
        // Typed dicts like dict[str, int] map to HashMap which has direct .len()
        if self.is_serde_json_value_expr(hir_arg) {
            // For JSON arrays: .as_array().map(|a| a.len()).unwrap_or(0)
            // This also works for objects and is the most common case
            Ok(parse_quote! {
                #arg.as_array().map(|a| a.len()).unwrap_or_else(||
                    #arg.as_object().map(|o| o.len()).unwrap_or(0)
                ) as i32
            })
        } else {
            // Default behavior for other types
            builtin_conversions::convert_len_call(arg_exprs)
        }
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_int_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_int_cast(
            self.ctx,
            hir_args,
            arg_exprs,
            |obj, method, args| {
                builtin_conversions::is_string_method_call(self.ctx, obj, method, args)
            },
            builtin_conversions::is_bool_expr,
        )
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_float_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_float_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    /// DEPYLER-0188: Pass HirExpr to detect PathBuf for .display().to_string()
    /// DEPYLER-0722: Handle Option<T> types - use .unwrap().to_string()
    /// GH-207: Don't add .unwrap() if expression already has .unwrap_or()
    pub(crate) fn convert_str_conversion(
        &self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0722: Check if argument is an Optional type
        if !hir_args.is_empty() && args.len() == 1 {
            let var_name = match &hir_args[0] {
                HirExpr::Var(name) => Some(name.as_str()),
                HirExpr::Attribute { attr, .. } => Some(attr.as_str()),
                _ => None,
            };
            if let Some(name) = var_name {
                if let Some(Type::Optional(_)) = self.ctx.var_types.get(name) {
                    let arg = &args[0];
                    // GH-207: Check if the arg already contains unwrap_or - if so, it's already
                    // unwrapped and we shouldn't add another .unwrap()
                    let arg_str = quote::quote!(#arg).to_string();
                    if arg_str.contains("unwrap_or") {
                        // Already unwrapped via unwrap_or - just call .to_string()
                        return Ok(parse_quote! { (#arg).to_string() });
                    }
                    return Ok(parse_quote! { (#arg).expect("value is None").to_string() });
                }
            }
        }
        builtin_conversions::convert_str_conversion(hir_args, args, |e| self.is_path_expr(e))
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    pub(crate) fn convert_bool_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_bool_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    pub(crate) fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        array_initialization::convert_range_call(args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    pub(crate) fn convert_array_init_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        array_initialization::convert_array_init_call(self.ctx, func, args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    pub(crate) fn convert_set_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_set_constructor(self.ctx, args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    pub(crate) fn convert_frozenset_constructor(
        &mut self,
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        collection_constructors::convert_frozenset_constructor(self.ctx, args)
    }

    // ========================================================================
    // DEPYLER-0171, 0172, 0173, 0174: Collection Conversion Builtins
    // DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    // ========================================================================

    /// DEPYLER-0751: Handle Counter(string) by using .chars() instead of .into_iter()
    pub(crate) fn convert_counter_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        self.ctx.needs_hashmap = true;
        // DEPYLER-0751: Handle Counter(string) → string.chars().fold(...)
        // String doesn't implement IntoIterator, need to use .chars()
        if hir_args.len() == 1 && args.len() == 1 {
            let hir_arg = &hir_args[0];
            let is_string = self.is_string_type(hir_arg)
                || matches!(
                    hir_arg,
                    HirExpr::Var(name) if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
                );
            if is_string {
                let arg = &args[0];
                return Ok(parse_quote! {
                    #arg.chars().fold(HashMap::new(), |mut acc, item| {
                        *acc.entry(item).or_insert(0) += 1;
                        acc
                    })
                });
            }
        }
        collection_constructors::convert_counter_builtin(self.ctx, args)
    }

    pub(crate) fn convert_defaultdict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_defaultdict_builtin(self.ctx, args)
    }

    pub(crate) fn convert_dict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_dict_builtin(self.ctx, args)
    }

    pub(crate) fn convert_deque_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_deque_builtin(self.ctx, args)
    }

    pub(crate) fn convert_list_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0651: Handle list(string) → string.chars().collect()
        // String doesn't implement IntoIterator, need to use .chars()
        if hir_args.len() == 1 && args.len() == 1 {
            let hir_arg = &hir_args[0];
            let is_string = self.is_string_type(hir_arg)
                || matches!(
                    hir_arg,
                    HirExpr::Var(name) if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::String))
                );
            if is_string {
                let arg = &args[0];
                return Ok(parse_quote! { #arg.chars().collect::<Vec<_>>() });
            }
        }
        collection_constructors::convert_list_builtin(self.ctx, args)
    }

    /// DEPYLER-0935: Convert Python bytes() constructor to Vec<u8>
    /// bytes() → Vec::<u8>::new()
    /// bytes(n) → vec![0u8; n]
    /// bytes([1, 2, 3]) → vec![1u8, 2u8, 3u8]
    /// bytes(string) → string.as_bytes().to_vec()
    pub(crate) fn convert_bytes_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytes() → Vec::<u8>::new()
            return Ok(parse_quote! { Vec::<u8>::new() });
        }

        if args.len() == 1 {
            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // bytes([1, 2, 3]) → list collected as Vec<u8>
            if matches!(hir_arg, HirExpr::List { .. }) {
                return Ok(parse_quote! { #arg.into_iter().map(|x| x as u8).collect::<Vec<u8>>() });
            }

            // bytes(string) → string.as_bytes().to_vec()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.as_bytes().to_vec() });
            }

            // bytes(bytearray_or_bytes) → just return the bytes/bytearray variable
            // Check if arg is a variable with list type (bytearray is Vec<u8> = List)
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::List(_)))
                {
                    return Ok(parse_quote! { #arg });
                }
            }

            // DEPYLER-0935: bytes(n) where n is numeric expression → vec![0u8; n as usize]
            // Check for int literal first
            if matches!(hir_arg, HirExpr::Literal(crate::hir::Literal::Int(_))) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Check for int variable
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::Int))
                {
                    return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
                }
            }

            // For method calls like .len(), assume they return size
            if matches!(hir_arg, HirExpr::MethodCall { .. }) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Default: assume it's a collection/bytes that should be returned as-is
            // This handles bytes(some_bytearray) → some_bytearray
            return Ok(parse_quote! { #arg });
        }

        // bytes with encoding args: bytes(source, encoding)
        if args.len() >= 2 {
            let arg = &args[0];
            return Ok(parse_quote! { #arg.as_bytes().to_vec() });
        }

        Ok(parse_quote! { Vec::<u8>::new() })
    }

    /// DEPYLER-0674: Convert Python bytearray() constructor to Vec<u8>
    /// bytearray() → Vec::new()
    /// bytearray(n) → vec![0u8; n]
    /// bytearray([1, 2, 3]) → vec![1u8, 2u8, 3u8]
    /// bytearray(b"hello") → b"hello".to_vec()
    pub(crate) fn convert_bytearray_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytearray() → Vec::<u8>::new()
            return Ok(parse_quote! { Vec::<u8>::new() });
        }

        if args.len() == 1 {
            // Safety check to prevent panic if hir_args is out of sync
            if hir_args.is_empty() {
                let arg = &args[0];
                return Ok(parse_quote! { #arg.to_vec() });
            }

            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // bytearray([1, 2, 3]) → list.into_iter() and collect as Vec<u8>
            if matches!(hir_arg, HirExpr::List { .. }) {
                return Ok(parse_quote! { #arg.into_iter().collect::<Vec<u8>>() });
            }

            // bytearray(string) → string.as_bytes().to_vec()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.as_bytes().to_vec() });
            }

            // bytearray(bytes) → copy the bytes into a new vec
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::List(_)))
                {
                    return Ok(parse_quote! { #arg.to_vec() });
                }
            }

            // DEPYLER-0936: bytearray(n) where n is numeric → vec![0u8; n as usize]
            // Check for int literal
            if matches!(hir_arg, HirExpr::Literal(crate::hir::Literal::Int(_))) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Check for int variable
            if let HirExpr::Var(name) = hir_arg {
                if self
                    .ctx
                    .var_types
                    .get(name)
                    .is_some_and(|t| matches!(t, Type::Int))
                {
                    return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
                }
            }

            // For method calls like .len(), assume they return size
            if matches!(hir_arg, HirExpr::MethodCall { .. }) {
                return Ok(parse_quote! { vec![0u8; (#arg) as usize] });
            }

            // Default: assume it's a collection that should be collected
            return Ok(parse_quote! { #arg.to_vec() });
        }

        // bytearray with multiple args (source, encoding, errors) - just get bytes
        if args.len() >= 2 {
            let arg = &args[0];
            return Ok(parse_quote! { #arg.as_bytes().to_vec() });
        }

        Ok(parse_quote! { Vec::<u8>::new() })
    }

    /// DEPYLER-0937: Convert Python tuple() constructor to Vec
    /// In Rust, we represent Python tuples as Vec since Rust tuples are fixed-size.
    /// tuple() → vec![]
    /// tuple([1, 2, 3]) → vec![1, 2, 3]
    /// tuple(iterable) → iterable.into_iter().collect::<Vec<_>>()
    pub(crate) fn convert_tuple_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            // tuple() → Vec::new()
            return Ok(parse_quote! { Vec::new() });
        }

        if args.len() == 1 {
            let hir_arg = &hir_args[0];
            let arg = &args[0];

            // tuple(string) → string.chars().collect()
            if self.is_string_type(hir_arg) {
                return Ok(parse_quote! { #arg.chars().collect::<Vec<_>>() });
            }

            // tuple(list) or tuple(iterable) → collect to Vec
            return Ok(parse_quote! { #arg.into_iter().collect::<Vec<_>>() });
        }

        // tuple doesn't take multiple args, but fallback to first arg
        let arg = &args[0];
        Ok(parse_quote! { #arg.into_iter().collect::<Vec<_>>() })
    }

    // DEPYLER-COVERAGE-95: all, any, divmod, enumerate, zip, reversed, sorted
    // moved to stdlib_method_gen::builtin_functions module for testability

    pub(crate) fn convert_filter_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.len() != 2 {
            bail!("filter() requires exactly 2 arguments");
        }
        // Check if first arg is lambda
        // DEPYLER-0754: Use .iter().cloned() instead of .into_iter() to produce owned values
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned() properly clones elements to produce owned values.
        if let HirExpr::Lambda { params, body } = &hir_args[0] {
            if params.len() != 1 {
                bail!("filter() lambda must have exactly 1 parameter");
            }
            // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
            let param_ident = crate::rust_gen::keywords::safe_ident(&params[0]);

            // DEPYLER-1053: Infer element type from iterable and add lambda param to var_types
            // This enables type coercion in comparisons like `x != 0` where x is f64
            let elem_type = self.infer_iterable_element_type(&hir_args[1]);
            let param_name = params[0].clone();
            if let Some(ref elem_t) = elem_type {
                self.ctx
                    .var_types
                    .insert(param_name.clone(), elem_t.clone());
            }

            let body_expr = body.to_rust_expr(self.ctx)?;

            // DEPYLER-1053: Remove lambda param from var_types to avoid polluting context
            if elem_type.is_some() {
                self.ctx.var_types.remove(&param_name);
            }

            let iterable = &args[1];
            // DEPYLER-1053: Use |&x| pattern because filter() always receives &Item
            // Even with .cloned(), filter's closure parameter is a reference to the owned value
            Ok(parse_quote! {
                #iterable.iter().cloned().filter(|&#param_ident| #body_expr)
            })
        } else {
            let predicate = &args[0];
            let iterable = &args[1];
            Ok(parse_quote! {
                #iterable.iter().cloned().filter(#predicate)
            })
        }
    }

    // DEPYLER-COVERAGE-95: sum, round, abs, min, max, pow, hex, bin, oct
    // moved to stdlib_method_gen::builtin_functions module for testability

    /// DEPYLER-0579: Python format(value, spec) builtin
    /// format(num, "b") → binary string
    /// format(num, "o") → octal string
    /// format(num, "x") → hex string
    /// format(num, "d") → decimal string
    pub(crate) fn convert_format_builtin(
        &self,
        args: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if args.len() != 2 {
            bail!("format() requires exactly 2 arguments (value, spec)");
        }
        let value = &args[0];
        // Extract format spec from HIR to get the actual string
        if let HirExpr::Literal(Literal::String(spec)) = &hir_args[1] {
            match spec.as_str() {
                "b" => Ok(parse_quote! { format!("{:b}", #value) }),
                "o" => Ok(parse_quote! { format!("{:o}", #value) }),
                "x" => Ok(parse_quote! { format!("{:x}", #value) }),
                "X" => Ok(parse_quote! { format!("{:X}", #value) }),
                "d" => Ok(parse_quote! { format!("{}", #value) }),
                "" => Ok(parse_quote! { format!("{}", #value) }),
                _ => {
                    // For unknown format specs, fall back to generic format
                    let spec_str = spec.as_str();
                    // Try to parse as f-string format spec
                    let format_str = format!("{{:{}}}", spec_str);
                    let format_lit: syn::LitStr = syn::parse_str(&format!("\"{}\"", format_str))?;
                    Ok(parse_quote! { format!(#format_lit, #value) })
                }
            }
        } else {
            // Dynamic format spec - can't handle at compile time
            bail!("format() requires a string literal format specifier");
        }
    }

    // DEPYLER-COVERAGE-95: chr moved to stdlib_method_gen::builtin_functions

    pub(crate) fn convert_ord_builtin(
        &self,
        args: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("ord() requires exactly 1 argument");
        }
        let char_str = &args[0];

        // DEPYLER-0795: Check if argument is a char iteration variable
        // When iterating over string.chars(), the loop variable is already a char,
        // so we should use `var as u32` instead of `var.chars().next().unwrap() as i32`
        if let Some(HirExpr::Var(var_name)) = hir_args.first() {
            if self.ctx.char_iter_vars.contains(var_name) {
                // Variable is a char from string iteration - just cast it
                return Ok(parse_quote! {
                    #char_str as u32 as i32
                });
            }
        }

        // Default: assume it's a string and get first char
        Ok(parse_quote! {
            #char_str.chars().next().expect("empty string") as i32
        })
    }

    /// Convert Python open() to Rust file I/O
    /// DEPYLER-0387: File I/O builtin for context managers
    ///
    /// Maps Python open() to Rust std::fs:
    /// - open(path) or open(path, 'r') → std::fs::File::open(path)?
    /// - open(path, 'w') → std::fs::File::create(path)?
    /// - open(path, 'a') → std::fs::OpenOptions::new().append(true).open(path)?
    ///
    /// # Complexity
    /// ≤10 (match with 3 branches)
    pub(crate) fn convert_open_builtin(
        &mut self,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("open() requires 1 or 2 arguments");
        }

        // DEPYLER-0458: File handles need Read/Write traits
        self.ctx.needs_io_read = true;
        self.ctx.needs_io_write = true;

        let path = &args[0];

        // Determine mode from second argument (default is 'r')
        let mode = if args.len() == 2 {
            // Try to extract string literal from HIR
            if let Some(HirExpr::Literal(Literal::String(mode_str))) = hir_args.get(1) {
                mode_str.as_str()
            } else {
                // If not a literal, default to read mode
                "r"
            }
        } else {
            "r" // Default mode
        };

        // DEPYLER-0541: Handle Option<String> paths with proper unwrapping
        // DEPYLER-0465: Borrow path to avoid moving String parameters
        let borrowed_path = if let Some(hir_arg) = hir_args.first() {
            self.borrow_path_with_option_check(path, hir_arg)
        } else {
            Self::borrow_if_needed(path)
        };

        // DEPYLER-0561: In generator context, use .ok()? since next() returns Option, not Result
        let in_generator = self.ctx.in_generator;

        match mode {
            "r" | "rb" => {
                // Read mode → std::fs::File::open(path)?
                if in_generator {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path)? })
                }
            }
            "w" | "wb" => {
                // Write mode → std::fs::File::create(path)?
                if in_generator {
                    Ok(parse_quote! { std::fs::File::create(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::create(#borrowed_path)? })
                }
            }
            "a" | "ab" => {
                // Append mode → OpenOptions with append
                if in_generator {
                    Ok(parse_quote! {
                        std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(#borrowed_path).ok()?
                    })
                } else {
                    Ok(parse_quote! {
                        std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(#borrowed_path)?
                    })
                }
            }
            _ => {
                // Unsupported mode, default to read
                if in_generator {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path).ok()? })
                } else {
                    Ok(parse_quote! { std::fs::File::open(#borrowed_path)? })
                }
            }
        }
    }

    // DEPYLER-COVERAGE-95: hash, repr, next, iter, type
    // moved to stdlib_method_gen::builtin_functions module for testability

    // DEPYLER-STDLIB-50: getattr() - get attribute by name (needs context-specific error)
    pub(crate) fn convert_getattr_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 || args.len() > 3 {
            bail!("getattr() requires 2 or 3 arguments (object, name, optional default)");
        }
        // Note: This is a simplified implementation
        // Full getattr() requires runtime attribute lookup which isn't possible in Rust
        // For now, we'll bail as it needs special handling
        bail!("getattr() requires dynamic attribute access not fully supported yet")
    }

    // DEPYLER-REFACTOR-001: Helper functions moved to collection_constructors module:
    // already_collected, is_range_expr, is_iterator_expr, is_csv_reader_var

    pub(crate) fn convert_generic_call(
        &mut self,
        func: &str,
        hir_args: &[HirExpr],
        args: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0462: print() is now handled in convert_call() to support file=stderr kwarg

        // Check if this is an imported function
        if let Some(rust_path) = self.ctx.imported_items.get(func) {
            // DEPYLER-0557: Special handling for itertools.groupby
            // It's a trait method, not a standalone function
            if rust_path == "itertools::Itertools" && func == "groupby" && args.len() >= 2 {
                let iterable = &args[0];
                let key_func = &args[1];
                // needs_itertools is already set from import processing
                return Ok(parse_quote! {
                    {
                        use itertools::Itertools;
                        #iterable.into_iter().group_by(#key_func)
                    }
                });
            }

            // DEPYLER-0702: Special handling for os.path method imports
            // `from os.path import join as path_join` → path_join(a, b) should generate
            // PathBuf::from(a).join(b).to_string_lossy().to_string()
            if rust_path == "std::path::Path::join" {
                if args.is_empty() {
                    bail!("path join requires at least 1 argument");
                }
                let first = &args[0];
                if args.len() == 1 {
                    return Ok(
                        parse_quote! { std::path::PathBuf::from(#first).to_string_lossy().to_string() },
                    );
                }
                // DEPYLER-0814: Check if any arg (after first) is a List/Vec type (varargs)
                // If so, generate iteration code instead of chaining .join()
                for (i, hir_arg) in hir_args[1..].iter().enumerate() {
                    if let HirExpr::Var(name) = hir_arg {
                        if let Some(Type::List(_)) = self.ctx.var_types.get(name) {
                            // This is a vararg parameter - generate iteration code
                            let parts_var = &args[i + 1];
                            return Ok(parse_quote! {
                                {
                                    let mut __path = std::path::PathBuf::from(#first);
                                    for __part in #parts_var {
                                        __path = __path.join(__part);
                                    }
                                    __path.to_string_lossy().to_string()
                                }
                            });
                        }
                    }
                }
                // Normal case: chain .join() calls
                let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                for part in &args[1..] {
                    result = parse_quote! { #result.join(#part) };
                }
                return Ok(parse_quote! { #result.to_string_lossy().to_string() });
            }

            // DEPYLER-0702: Handle other os.path method imports
            if rust_path == "std::path::Path::exists" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! { std::path::Path::new(&#path).exists() });
            }
            if rust_path == "std::path::Path::file_name" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                });
            }
            if rust_path == "std::path::Path::parent" && args.len() == 1 {
                let path = &args[0];
                return Ok(parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                });
            }

            // DEPYLER-0721: Handle os.path.splitext import
            // splitext(path) → (stem, extension) tuple
            if rust_path == "std::path::Path" && func == "splitext" && args.len() == 1 {
                let path_arg = &args[0];
                return Ok(parse_quote! {
                    {
                        let __path = std::path::Path::new(&#path_arg);
                        let __stem = __path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();
                        let __ext = __path.extension()
                            .and_then(|e| e.to_str())
                            .map(|e| format!(".{}", e))
                            .unwrap_or_default();
                        (__stem, __ext)
                    }
                });
            }

            // DEPYLER-0771: Special handling for math.isqrt import
            // isqrt(n) → (n as f64).sqrt().floor() as i32
            // This is needed because std::f64::isqrt doesn't exist in Rust
            // Check both exact match and ends_with for robustness
            if (rust_path == "std::f64::isqrt" || rust_path.ends_with("::isqrt"))
                && func == "isqrt"
                && args.len() == 1
            {
                let arg = &args[0];
                return Ok(parse_quote! { ((#arg) as f64).sqrt().floor() as i32 });
            }

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

            // DEPYLER-0493: Check if this is a struct type that needs constructor pattern
            // Look up constructor pattern from imported modules
            use crate::module_mapper::ConstructorPattern;
            let constructor_pattern = self.ctx.imported_modules.values().find_map(|module| {
                // Get the last part of the rust_path (e.g., "NamedTempFile" from "tempfile::NamedTempFile")
                let type_name = path_parts.last()?;
                module.constructor_patterns.get(*type_name)
            });

            // DEPYLER-1004: Special handling for serde_json::from_str to use proper type annotation
            // When called via `from json import loads`, we need to:
            // 1. Check return type context for HashMap vs Value
            // 2. Add type annotation and .unwrap()
            // DEPYLER-1022: NASA mode returns stub HashMap
            if rust_path == "serde_json::from_str" && args.len() == 1 {
                let arg = &args[0];
                if self.ctx.type_mapper.nasa_mode {
                    // NASA mode: return stub HashMap
                    // DEPYLER-1051: Use DepylerValue for Hybrid Fallback Strategy
                    self.ctx.needs_hashmap = true;
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! {
                        std::collections::HashMap::<String, DepylerValue>::new()
                    });
                }
                self.ctx.needs_serde_json = true;
                if stdlib_method_gen::json::return_type_needs_json_dict(self.ctx) {
                    self.ctx.needs_hashmap = true;
                    return Ok(parse_quote! {
                        serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&#arg).expect("parse failed")
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_str::<serde_json::Value>(&#arg).expect("parse failed")
                    });
                }
            }

            // DEPYLER-1004: Special handling for serde_json::from_reader
            // DEPYLER-1022: NASA mode returns stub HashMap
            if rust_path == "serde_json::from_reader" && args.len() == 1 {
                let arg = &args[0];
                if self.ctx.type_mapper.nasa_mode {
                    // NASA mode: return stub HashMap
                    // DEPYLER-1051: Use DepylerValue for Hybrid Fallback Strategy
                    self.ctx.needs_hashmap = true;
                    self.ctx.needs_depyler_value_enum = true;
                    return Ok(parse_quote! {
                        std::collections::HashMap::<String, DepylerValue>::new()
                    });
                }
                self.ctx.needs_serde_json = true;
                if stdlib_method_gen::json::return_type_needs_json_dict(self.ctx) {
                    self.ctx.needs_hashmap = true;
                    return Ok(parse_quote! {
                        serde_json::from_reader::<_, std::collections::HashMap<String, serde_json::Value>>(#arg).expect("parse failed")
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_reader::<_, serde_json::Value>(#arg).expect("parse failed")
                    });
                }
            }

            // DEPYLER-1004: Check if this function returns Result and needs .unwrap()
            // json to_string and to_writer still need .unwrap()
            let needs_unwrap = matches!(
                rust_path.as_str(),
                "serde_json::to_string" | "serde_json::to_writer"
            );

            // Generate call based on constructor pattern
            return match constructor_pattern {
                Some(ConstructorPattern::New) => {
                    // Struct type → use ::new() pattern
                    if args.is_empty() {
                        Ok(parse_quote! { #path::new() })
                    } else {
                        Ok(parse_quote! { #path::new(#(#args),*) })
                    }
                }
                Some(ConstructorPattern::Method(method)) => {
                    // Custom method (e.g., File::open())
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    if args.is_empty() {
                        Ok(parse_quote! { #path::#method_ident() })
                    } else {
                        Ok(parse_quote! { #path::#method_ident(#(#args),*) })
                    }
                }
                Some(ConstructorPattern::Function) | None => {
                    // Regular function call (default behavior)
                    // DEPYLER-1004: Add .unwrap() for Result-returning functions
                    if needs_unwrap {
                        if args.is_empty() {
                            Ok(parse_quote! { #path().expect("operation failed") })
                        } else {
                            Ok(parse_quote! { #path(#(#args),*).expect("operation failed") })
                        }
                    } else if args.is_empty() {
                        Ok(parse_quote! { #path() })
                    } else {
                        Ok(parse_quote! { #path(#(#args),*) })
                    }
                }
            };
        }

        // Check if this might be a constructor call (capitalized name)
        if func
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // DEPYLER-0900: Rename constructor if it shadows stdlib type (e.g., Box -> PyBox)
            // Treat as constructor call - ClassName::new(args)
            let safe_name = crate::direct_rules::safe_class_name(func);
            let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
            if args.is_empty() {
                // DEPYLER-0233: Only apply default argument heuristics for Python stdlib types
                // User-defined classes should always generate ClassName::new() with no args
                let is_user_class = self.ctx.class_names.contains(func);

                // Note: Constructor default parameter handling uses simple heuristics.
                // Ideally this would be context-aware and know the actual default values
                // for each class constructor, but currently uses hardcoded patterns.
                // This is a known limitation - constructors may require explicit arguments.
                if !is_user_class && func == "Counter" {
                    return Ok(parse_quote! { #class_ident::new(0) });
                }
                Ok(parse_quote! { #class_ident::new() })
            } else {
                // DEPYLER-0932: Complete missing constructor arguments with defaults
                // When Python calls Config("localhost") but Config has 3 fields with 2 defaults,
                // we need to generate Config::new("localhost".to_string(), 8080, false)
                let mut completed_args = args.to_vec();
                if let Some(defaults) = self.ctx.class_field_defaults.get(func) {
                    let num_provided = completed_args.len();
                    let num_fields = defaults.len();

                    if num_provided < num_fields {
                        // Fill in missing arguments from defaults
                        for i in num_provided..num_fields {
                            if let Some(Some(default_expr)) = defaults.get(i) {
                                use crate::hir::{HirExpr, Literal};
                                let default_syn: syn::Expr = match default_expr {
                                    HirExpr::Literal(Literal::None) => {
                                        parse_quote! { None }
                                    }
                                    HirExpr::Literal(Literal::Int(n)) => {
                                        let n_i32 = *n as i32;
                                        parse_quote! { #n_i32 }
                                    }
                                    HirExpr::Literal(Literal::Float(f)) => {
                                        let f = *f;
                                        parse_quote! { #f }
                                    }
                                    HirExpr::Literal(Literal::Bool(b)) => {
                                        let b = *b;
                                        parse_quote! { #b }
                                    }
                                    HirExpr::Literal(Literal::String(s)) => {
                                        parse_quote! { #s.to_string() }
                                    }
                                    // For complex defaults, skip
                                    _ => continue,
                                };
                                completed_args.push(default_syn);
                            }
                        }
                    }
                }
                Ok(parse_quote! { #class_ident::new(#(#completed_args),*) })
            }
        } else {
            // DEPYLER-0771: Fallback handling for isqrt if not found in imported_items
            // This handles cases where the import wasn't properly tracked
            if func == "isqrt" && args.len() == 1 {
                let arg = &args[0];
                return Ok(parse_quote! { ((#arg) as f64).sqrt().floor() as i32 });
            }

            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            // This is the fallback handler for isinstance calls that weren't caught earlier
            if func == "isinstance" {
                return Ok(parse_quote! { true });
            }

            // Regular function call
            // DEPYLER-0588: Use safe_ident to handle keywords and invalid characters
            let func_ident = crate::rust_gen::keywords::safe_ident(func);

            // DEPYLER-0301 Fix: Auto-borrow Vec/List arguments when calling functions
            // DEPYLER-0269 Fix: Auto-borrow Dict/HashMap/Set arguments when calling functions
            // DEPYLER-0270 Fix: Check function signature before auto-borrowing
            // When passing a Vec/HashMap/HashSet variable to a function expecting &Vec/&HashMap/&HashSet, automatically borrow it
            // This handles cases like: sum_list_recursive(rest) where rest is Vec but param is &Vec
            //
            // Strategy:
            // 1. Look up function signature to see which params are borrowed
            // 2. Only borrow if: (a) arg is List/Dict/Set AND (b) function expects borrow
            // 3. Otherwise pass as-is (either owned or primitive)
            let borrowed_args: Vec<syn::Expr> = hir_args
                .iter()
                .zip(args.iter())
                .enumerate()
                .map(|(param_idx, (hir_arg, arg_expr))| {
                    // DEPYLER-0950: Integer literal coercion at f64 call sites
                    // When calling add(1, 2.5) where add expects (f64, f64), coerce 1 to 1.0
                    if let HirExpr::Literal(Literal::Int(n)) = hir_arg {
                        if let Some(param_types) = self.ctx.function_param_types.get(func) {
                            if let Some(Type::Float) = param_types.get(param_idx) {
                                // Integer literal passed where f64 expected - coerce to float
                                let f_val = *n as f64;
                                return parse_quote! { #f_val };
                            }
                        }
                    }

                    // DEPYLER-1208: DepylerValue→concrete auto-coercion (Rule 2)
                    // When a DepylerValue variable is passed to a function expecting concrete type,
                    // generate appropriate extraction: x.as_i64() as i32, x.as_f64(), etc.
                    if let HirExpr::Var(var_name) = hir_arg {
                        let var_type = self.ctx.var_types.get(var_name);
                        let is_depyler_value = matches!(var_type, Some(Type::Unknown) | None)
                            || matches!(var_type, Some(Type::Custom(s)) if s == "DepylerValue");

                        if is_depyler_value {
                            if let Some(param_types) = self.ctx.function_param_types.get(func) {
                                if let Some(expected_type) = param_types.get(param_idx) {
                                    match expected_type {
                                        Type::Int => {
                                            return parse_quote! { #arg_expr.as_i64().unwrap_or_default() as i32 };
                                        }
                                        Type::Float => {
                                            return parse_quote! { #arg_expr.as_f64().unwrap_or_default() };
                                        }
                                        Type::String => {
                                            return parse_quote! { #arg_expr.as_str().unwrap_or_default().to_string() };
                                        }
                                        Type::Bool => {
                                            return parse_quote! { #arg_expr.as_bool().unwrap_or_default() };
                                        }
                                        _ => {} // Other types - let regular flow handle
                                    }
                                }
                            }
                        }
                    }

                    // DEPYLER-1045: Convert char to String when passing to functions expecting &str
                    // When a loop variable from string.chars() is passed to a function,
                    // it needs to be converted to String because char and &str are incompatible.
                    if let HirExpr::Var(var_name) = hir_arg {
                        if self.ctx.char_iter_vars.contains(var_name) {
                            // Check if function expects &str at this param position
                            let expects_str = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true); // Default to expecting str for char iter vars
                            if expects_str {
                                return parse_quote! { &#arg_expr.to_string() };
                            }
                        }
                    }

                    // DEPYLER-0471: Clone args.config when passing to functions taking owned String
                    // This avoids "use after move" errors when args.config is used multiple times
                    if matches!(hir_arg, HirExpr::Attribute { value, attr }
                        if attr == "config" && matches!(value.as_ref(), HirExpr::Var(v) if v == "args"))
                    {
                        // Check if function takes owned String (not &str)
                        // For save_config and load_config, clone args.config
                        if matches!(func, "save_config" | "load_config") {
                            return parse_quote! { #arg_expr.clone() };
                        }
                    }

                    // DEPYLER-0469/0488: Special case for known functions that need String borrowing
                    // get_nested_value(config, key) - key param (index 1) needs &str
                    // set_nested_value(config, key, value) - key (1) needs &str, value (2) needs &str (NOT &mut!)
                    // DEPYLER-0488: Removed incorrect &mut for value parameter - it's only READ in the function
                    // These work with both Var and Attribute expressions (before/after argparse transform)
                    if (func == "get_nested_value" || func == "set_nested_value") && param_idx == 1 {
                        // Immutable borrow for key parameter
                        return parse_quote! { &#arg_expr };
                    } else if func == "set_nested_value" && param_idx == 2 {
                        // DEPYLER-0488: value parameter is READ (RHS of assignment), not mutated
                        // Immutable borrow is sufficient
                        return parse_quote! { &#arg_expr };
                    }

                    // DEPYLER-0424: Check if argument is argparse args variable
                    // If so, always pass by reference (&args)
                    if let HirExpr::Var(var_name) = hir_arg {
                        let is_argparse_args =
                            self.ctx
                                .argparser_tracker
                                .parsers
                                .values()
                                .any(|parser_info| {
                                    parser_info
                                        .args_var
                                        .as_ref()
                                        .is_some_and(|args_var| args_var == var_name)
                                });

                        if is_argparse_args {
                            return parse_quote! { &#arg_expr };
                        }
                    }

                    // DEPYLER-0600: First check if function explicitly requires &mut at this position
                    // This enables borrowing for types like File that aren't in the standard borrow list
                    let func_requires_mut = self.ctx
                        .function_param_muts
                        .get(func)
                        .and_then(|muts| muts.get(param_idx))
                        .copied()
                        .unwrap_or(false);

                    // Check if this param should be borrowed by looking up function signature
                    let should_borrow = if func_requires_mut {
                        // If function explicitly needs &mut, we must borrow
                        true
                    } else {
                        match hir_arg {
                        HirExpr::Var(var_name) => {
                            // Check if variable has List, Dict, Set, String, or Custom type
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // DEPYLER-0467: Debug logging for key/value
                                if matches!(var_name.as_str(), "key" | "value") {
                                    eprintln!("[DEPYLER-0467] Variable '{}' has type: {:?}", var_name, var_type);
                                }

                                // DEPYLER-0467: Always borrow serde_json::Value types
                                // These are typically borrowed in idiomatic Rust
                                if matches!(var_type, Type::Custom(ref s) if s == "serde_json::Value") {
                                    true  // Always borrow Value types
                                } else if matches!(var_type, Type::Dict(_, _)) {
                                    // Also borrow Dict types (mapped to serde_json::Value)
                                    true
                                } else if matches!(var_type, Type::String) {
                                    // DEPYLER-0469: Borrow String types as &str
                                    // DEPYLER-0818: But DON'T borrow if the variable is already &str
                                    // (i.e., it's a function param that was mapped to &str).
                                    // Borrowing an &str would create &&str which is wrong.
                                    // DEPYLER-1092: Use ref_params instead of fn_str_params
                                    // ref_params contains ONLY params that are actually borrowed (&str)
                                    // fn_str_params incorrectly contains ALL Type::String params
                                    !self.ctx.ref_params.contains(var_name)
                                } else if matches!(var_type, Type::Unknown) {
                                    // DEPYLER-0467: Heuristic for Unknown types
                                    // If variable name suggests it's commonly borrowed, borrow it
                                    // This handles cases where type inference fails (e.g., Result unwrapping, pattern matching)
                                    matches!(var_name.as_str(),
                                        "config" | "data" | "json" | "obj" | "document" |
                                        "key" | "value" | "path" | "name" | "text" | "content"
                                    )
                                } else if matches!(var_type, Type::List(_) | Type::Set(_)) {
                                    // DEPYLER-0466: Also borrow collection types
                                    // Check if function param expects a borrow
                                    self.ctx
                                        .function_param_borrows
                                        .get(func)
                                        .and_then(|borrows| borrows.get(param_idx))
                                        .copied()
                                        .unwrap_or(true) // Default to borrow if unknown
                                } else if matches!(var_type, Type::Custom(_)) {
                                    // DEPYLER-0767: Check function_param_borrows for Custom types
                                    // datetime maps to Type::Custom("chrono::NaiveDateTime")
                                    // Check if function signature expects a reference parameter
                                    self.ctx
                                        .function_param_borrows
                                        .get(func)
                                        .and_then(|borrows| borrows.get(param_idx))
                                        .copied()
                                        .unwrap_or(false) // Default to no borrow for custom types
                                } else {
                                    false
                                }
                            } else {
                                // DEPYLER-0467/DEPYLER-0767: Variable not in var_types
                                // First check function_param_borrows (authoritative source)
                                // Fall back to name heuristic if not tracked
                                eprintln!("[DEPYLER-0467] Variable '{}' NOT in var_types, checking function_param_borrows", var_name);
                                self.ctx
                                    .function_param_borrows
                                    .get(func)
                                    .and_then(|borrows| borrows.get(param_idx))
                                    .copied()
                                    // Name-based heuristic as last resort
                                    .unwrap_or(matches!(var_name.as_str(),
                                        "config" | "data" | "json" | "obj" | "document" |
                                        "key" | "value" | "path" | "name" | "text" | "content"
                                    ))
                            }
                        }
                        // DEPYLER-0359: Auto-borrow list/dict/set literals when calling functions
                        // List literal [1, 2, 3] should be passed as &vec![1, 2, 3]
                        HirExpr::List(_) | HirExpr::Dict(_) | HirExpr::Set(_) => {
                            // Check if function param expects a borrow
                            self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true) // Default to borrow if unknown
                        }
                        // DEPYLER-1092: Handle string literals passed to functions expecting &str
                        // Python: parse_list(value, ",") → Rust: parse_list(&value, ",")
                        // String literals are already &str, no .to_string() needed for borrowed params
                        HirExpr::Literal(Literal::String(_)) => {
                            // Check if function param expects a borrow (&str)
                            self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(false) // Default: no borrow (owned String)
                        }
                        // DEPYLER-0550: Handle attribute access like args.column, args.value
                        // These are String fields from CLI args struct that need borrowing
                        // when passed to functions expecting &str
                        HirExpr::Attribute { value, attr } => {
                            // Check if accessing args struct field
                            let is_args_field = if let HirExpr::Var(v) = value.as_ref() {
                                v == "args"
                            } else {
                                false
                            };

                            // Check if function expects borrow at this position
                            if is_args_field {
                                // For args struct fields (typically String), check function signature
                                self.ctx
                                    .function_param_borrows
                                    .get(func)
                                    .and_then(|borrows| borrows.get(param_idx))
                                    .copied()
                                    .unwrap_or(
                                        // Heuristic: borrow common string-like field names
                                        matches!(attr.as_str(),
                                            "column" | "value" | "name" | "key" | "pattern" |
                                            "text" | "query" | "path" | "config" | "file"
                                        )
                                    )
                            } else {
                                false
                            }
                        }
                        _ => {
                            // Fallback: check if expression creates a Vec via .to_vec()
                            let expr_string = quote! { #arg_expr }.to_string();
                            expr_string.contains("to_vec")
                        }
                    }
                    }; // Close the if func_requires_mut else block

                    // DEPYLER-0515: Let Rust's type inference determine integer types
                    // from function signatures, rather than blindly casting to i64.

                    // DEPYLER-0568: Handle PathBuf → String conversion for function arguments
                    // When passing a PathBuf to a function that expects String
                    if let HirExpr::Var(var_name) = hir_arg {
                        // DEPYLER-0666: Check if variable was already unwrapped via if-let
                        // If so, don't add .as_ref().unwrap() - the value is already concrete
                        let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name);

                        if let Some(var_type) = self.ctx.var_types.get(var_name) {
                            // PathBuf → String conversion
                            if matches!(var_type, Type::Custom(ref s) if s == "PathBuf" || s == "Path") {
                                // Check if this is a String-expecting function (heuristic)
                                // Function params with names like file_path, path, etc. often want String
                                return parse_quote! { #arg_expr.display().to_string() };
                            }
                            // Option<String> → &str conversion when function expects &str
                            // DEPYLER-0666: Skip if already unwrapped
                            if !is_unwrapped && matches!(var_type, Type::Optional(ref inner) if matches!(inner.as_ref(), Type::String)) {
                                // Unwrap the Option and pass reference
                                return parse_quote! { #arg_expr.as_ref().expect("value is None") };
                            }
                        } else {
                            // DEPYLER-0568: Name-based heuristic for PathBuf when not in var_types
                            // Variables named "path" are typically PathBuf from pathlib.Path()
                            let name = var_name.as_str();
                            if name == "path" || name.ends_with("_path") {
                                return parse_quote! { #arg_expr.display().to_string() };
                            }
                        }
                    }

                    // DEPYLER-0818: Handle &str → String conversion
                    // When an &str param (fn_str_params) is passed to a function expecting String,
                    // we need to add .to_string() to convert the reference to owned.
                    if let HirExpr::Var(var_name) = hir_arg {
                        if self.ctx.fn_str_params.contains(var_name) && !should_borrow {
                            // Variable is &str param but callee doesn't expect borrow (wants String)
                            // Check if callee expects a borrow - if not, convert to String
                            let callee_expects_borrow = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(false);

                            if !callee_expects_borrow {
                                return parse_quote! { #arg_expr.to_string() };
                            }
                        }
                    }

                    if should_borrow {
                        // DEPYLER-0574: Check if function expects &mut for this param
                        let needs_mut = self.ctx
                            .function_param_muts
                            .get(func)
                            .and_then(|muts| muts.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0964/1217: Don't add &mut if variable is already &mut
                        // This includes:
                        // - mut_option_dict_params: &mut Option<HashMap>
                        // - mut_ref_params: parameters that are &mut T (detected via mutation analysis)
                        // Adding &mut would create &&mut which is invalid
                        let is_already_mut_ref = if let HirExpr::Var(var_name) = hir_arg {
                            self.ctx.mut_option_dict_params.contains(var_name)
                                || self.ctx.mut_ref_params.contains(var_name)
                        } else {
                            false
                        };

                        if is_already_mut_ref {
                            // Variable is already &mut, pass it directly
                            arg_expr.clone()
                        } else if needs_mut {
                            parse_quote! { &mut #arg_expr }
                        } else {
                            parse_quote! { &#arg_expr }
                        }
                    } else {
                        // DEPYLER-0737/0779: Check if function param is Optional FIRST
                        // This determines if we need to wrap the final result in Some()
                        let is_optional_param = self.ctx
                            .function_param_optionals
                            .get(func)
                            .and_then(|optionals| optionals.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0760: Don't double-wrap if arg is already Option<T>
                        let is_already_optional = if let HirExpr::Var(var_name) = hir_arg {
                            self.ctx
                                .var_types
                                .get(var_name)
                                .map(|ty| matches!(ty, Type::Optional(_)))
                                .unwrap_or(false)
                        } else if let HirExpr::Attribute { value: _, attr } = hir_arg {
                            // Handle attribute access like args.cwd
                            let check_optional = |arg: &crate::rust_gen::argparse_transform::ArgParserArgument| {
                                let field_name = arg.rust_field_name();
                                if field_name != *attr {
                                    return false;
                                }
                                if matches!(arg.action.as_deref(), Some("store_true") | Some("store_false")) {
                                    return false;
                                }
                                !arg.is_positional
                                    && !arg.required.unwrap_or(false)
                                    && arg.default.is_none()
                                    && !matches!(arg.nargs.as_deref(), Some("+") | Some("*"))
                            };

                            let is_optional_in_parser = self.ctx.argparser_tracker.parsers.values()
                                .any(|parser_info| parser_info.arguments.iter().any(&check_optional));
                            let is_optional_in_subcommand = self.ctx.argparser_tracker.subcommands.values()
                                .any(|sub_info| sub_info.arguments.iter().any(&check_optional));

                            is_optional_in_parser || is_optional_in_subcommand
                        } else {
                            false
                        };

                        // Don't wrap if arg is already None
                        let is_none = matches!(hir_arg, HirExpr::Literal(Literal::None));
                        let needs_some_wrap = is_optional_param && !is_none && !is_already_optional;

                        // DEPYLER-0779: Check if the optional param is also borrowed (&Option<T>)
                        // vs owned (Option<T>) - this determines if we use &Some() or Some()
                        let optional_is_borrowed = self.ctx
                            .function_param_borrows
                            .get(func)
                            .and_then(|borrows| borrows.get(param_idx))
                            .copied()
                            .unwrap_or(false);

                        // DEPYLER-0635: String literal args need type-aware conversion
                        // - If function param expects &str (borrowed), pass literal directly
                        // - If function param expects String (owned), add .to_string()
                        // Check function_param_borrows to determine expected type
                        // DEPYLER-TYPE-001: Default to true (borrowed) because Type::String params
                        // become &str in generated Rust code, not String. String literals ARE &str.
                        if matches!(hir_arg, HirExpr::Literal(Literal::String(_))) {
                            // Check if function expects borrowed string (&str) at this position
                            let param_expects_borrowed = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(true);

                            if param_expects_borrowed {
                                // Param is &str - string literal works directly
                                // DEPYLER-0779: But wrap in Some if optional param
                                // DEPYLER-TYPE-001: Don't add .to_string() when param expects borrowed
                                if needs_some_wrap {
                                    // For Option<&str>, wrap the literal directly without .to_string()
                                    if optional_is_borrowed {
                                        return parse_quote! { &Some(#arg_expr) };
                                    } else {
                                        return parse_quote! { Some(#arg_expr) };
                                    }
                                }
                                return arg_expr.clone();
                            } else {
                                // Param is String - need .to_string() conversion
                                let expr_str = quote::quote! { #arg_expr }.to_string();
                                let converted: syn::Expr = if !expr_str.contains("to_string") {
                                    parse_quote! { #arg_expr.to_string() }
                                } else {
                                    arg_expr.clone()
                                };
                                // DEPYLER-0779: Wrap in Some if optional param
                                // Use &Some for borrowed (&Option<T>), Some for owned (Option<T>)
                                if needs_some_wrap {
                                    if optional_is_borrowed {
                                        return parse_quote! { &Some(#converted) };
                                    } else {
                                        return parse_quote! { Some(#converted) };
                                    }
                                }
                                return converted;
                            }
                        }

                        // For non-string literals, apply Some wrapping if needed
                        // Use &Some for borrowed (&Option<T>), Some for owned (Option<T>)
                        if needs_some_wrap {
                            if optional_is_borrowed {
                                return parse_quote! { &Some(#arg_expr) };
                            } else {
                                return parse_quote! { Some(#arg_expr) };
                            }
                        }

                        // DEPYLER-1168: Call-site clone insertion for variables used later
                        // When a function takes ownership (doesn't borrow) and the argument
                        // variable is used again later in the same scope, we need to clone it.
                        // This prevents E0382 "use of moved value" errors.
                        if let HirExpr::Var(var_name) = hir_arg {
                            // Only clone if:
                            // 1. Variable is used later in the same scope
                            // 2. Variable type is clonable (List, Dict, Set, String, Custom types)
                            let used_later = self.ctx.vars_used_later.contains(var_name);
                            let is_clonable_type = self.ctx.var_types.get(var_name)
                                .map(|ty| matches!(ty,
                                    Type::List(_) | Type::Dict(_, _) | Type::Set(_) |
                                    Type::String | Type::Tuple(_) | Type::Custom(_)
                                ))
                                .unwrap_or(false);

                            if used_later && is_clonable_type {
                                return parse_quote! { #arg_expr.clone() };
                            }
                        }

                        arg_expr.clone()
                    }
                })
                .collect();

            // DEPYLER-0621: Complete missing arguments with default values
            // When Python calls `f()` but `def f(x=None)`, we need to generate `f(None)` in Rust
            // Look up registered defaults and append any missing arguments
            let mut completed_args = borrowed_args;
            if let Some(defaults) = self.ctx.function_param_defaults.get(func) {
                let num_provided = completed_args.len();
                let num_params = defaults.len();

                if num_provided < num_params {
                    // Need to fill in missing arguments from defaults
                    for i in num_provided..num_params {
                        if let Some(Some(default_expr)) = defaults.get(i) {
                            // Handle common default values directly without calling to_rust_expr
                            // (to_rust_expr requires &mut ctx which we don't have in &self)
                            use crate::hir::{HirExpr, Literal};
                            // DEPYLER-0629: Check if parameter needs borrowing
                            // If the parameter type is &Option<T>, we need &None instead of None
                            // DEPYLER-TYPE-001: Default to true for string params (Type::String → &str)
                            let param_needs_borrow = self
                                .ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(i).copied())
                                .unwrap_or(true);

                            let default_syn: syn::Expr = match default_expr {
                                HirExpr::Literal(Literal::None) => {
                                    if param_needs_borrow {
                                        parse_quote! { &None }
                                    } else {
                                        parse_quote! { None }
                                    }
                                }
                                HirExpr::Literal(Literal::Int(n)) => {
                                    // DEPYLER-0806: Use i32 suffix for default args
                                    // Python int maps to Rust i32 for function params
                                    // Using i64 causes E0308 when param expects i32
                                    let n_i32 = *n as i32;
                                    parse_quote! { #n_i32 }
                                }
                                HirExpr::Literal(Literal::Float(f)) => {
                                    let f = *f;
                                    parse_quote! { #f }
                                }
                                HirExpr::Literal(Literal::Bool(b)) => {
                                    let b = *b;
                                    parse_quote! { #b }
                                }
                                HirExpr::Literal(Literal::String(s)) => {
                                    // DEPYLER-1092: Check if param expects &str
                                    // If so, use "..." directly (string literal IS &str)
                                    // without .to_string(), avoiding E0308
                                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                                    if param_needs_borrow {
                                        // String literal "..." is already &str
                                        parse_quote! { #lit }
                                    } else {
                                        parse_quote! { #lit.to_string() }
                                    }
                                }
                                // For complex defaults, skip - function definition should handle
                                _ => continue,
                            };
                            completed_args.push(default_syn);
                        }
                    }
                }
            }
            let borrowed_args = completed_args;

            // DEPYLER-0648: Handle vararg functions - wrap arguments in slice
            // Python: run_cli("--help") where def run_cli(*args)
            // Rust: run_cli(&["--help".to_string()]) where fn run_cli(args: &[String])
            if self.ctx.vararg_functions.contains(func) && !borrowed_args.is_empty() {
                // DEPYLER-0660: Check if single arg is already a Vec (from starred unpacking)
                // Python: join_paths(*args.parts) where args.parts is List[str]
                // Should become: join_paths(&parts) not join_paths(&[parts])
                if borrowed_args.len() == 1 && hir_args.len() == 1 {
                    let hir_arg = &hir_args[0];
                    let arg_is_collection = match hir_arg {
                        // Attribute access to plural-named field (likely Vec)
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(v) = value.as_ref() {
                                v == "args"
                                    && (attr.ends_with('s')
                                        || attr == "parts"
                                        || attr == "items"
                                        || attr == "values"
                                        || attr == "keys"
                                        || attr == "args")
                            } else {
                                false
                            }
                        }
                        // Variable that's known to be a list
                        HirExpr::Var(v) => {
                            v.ends_with('s') || v == "parts" || v == "items" || v == "args"
                        }
                        // List literal
                        HirExpr::List(_) => true,
                        _ => false,
                    };

                    if arg_is_collection {
                        let arg = &borrowed_args[0];
                        return Ok(parse_quote! { #func_ident(&#arg) });
                    }
                }
                // Wrap all arguments in a slice literal
                return Ok(parse_quote! { #func_ident(&[#(#borrowed_args),*]) });
            }

            // DEPYLER-0422 Fix #6: Remove automatic `?` operator for function calls
            // DEPYLER-0287 was too broad - it added `?` to ALL function calls when inside a Result-returning function.
            // This caused E0277 errors (279 errors!) when calling functions that return plain types (i32, Vec, etc.).
            //
            // Root Cause Analysis:
            // 1. Why: `?` operator applied to i32/Vec (non-Result types)
            // 2. Why: Transpiler adds `?` to all function calls inside Result-returning functions
            // 3. Why: DEPYLER-0287 unconditionally adds `?` when current_function_can_fail is true
            // 4. Why: No check if the CALLED function actually returns Result
            // 5. ROOT CAUSE: Overly aggressive error propagation heuristic
            //
            // Solution: Don't automatically add `?` to function calls. Let explicit error handling
            // in Python (try/except) determine when Result types are needed.
            // If specific cases need `?` for recursive calls, those should be handled specially.
            //
            // DEPYLER-0588: Use try_parse to avoid panics on invalid expressions
            let args_tokens: Vec<_> = borrowed_args.iter().map(|a| quote::quote! { #a }).collect();
            let call_str = format!(
                "{}({})",
                func_ident,
                args_tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let call_expr: syn::Expr = match syn::parse_str(&call_str) {
                Ok(expr) => expr,
                Err(_) => {
                    // DEPYLER-0588: Fallback using syn::parse_str instead of parse_quote!
                    // This avoids panics even with unusual function names
                    let simple_call = format!("{}()", func_ident);
                    syn::parse_str(&simple_call).unwrap_or_else(|_| {
                        // Ultimate fallback: create a unit expression
                        syn::parse_str("()").unwrap()
                    })
                }
            };
            Ok(call_expr)
        }
    }

    // ========================================================================
    // DEPYLER-0142 Phase 1: Preamble Helpers
    // ========================================================================

    /// Try to convert classmethod call (cls.method())
    #[inline]
    pub(crate) fn try_convert_classmethod(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        if let HirExpr::Var(var_name) = object {
            if var_name == "cls" && self.ctx.is_classmethod {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(Some(parse_quote! { Self::#method_ident(#(#arg_exprs),*) }));
            }
        }
        Ok(None)
    }

    /// DEPYLER-0021: Handle struct module methods (pack, unpack, calcsize)
    /// Only supports format codes 'i' (signed 32-bit int) and 'ii' (two ints)
    pub(crate) fn try_convert_struct_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        match method {
            "pack" => {
                if args.is_empty() {
                    bail!("struct.pack() requires at least a format argument");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.pack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    if count != args.len() - 1 {
                        bail!(
                            "struct.pack() format '{}' expects {} values, got {}",
                            format,
                            count,
                            args.len() - 1
                        );
                    }

                    // Convert value arguments
                    let value_exprs: Vec<syn::Expr> = args[1..]
                        .iter()
                        .map(|arg| arg.to_rust_expr(self.ctx))
                        .collect::<Result<Vec<_>>>()?;

                    if count == 1 {
                        // struct.pack('i', value) → (value as i32).to_le_bytes().to_vec()
                        let val = &value_exprs[0];
                        Ok(Some(parse_quote! {
                            (#val as i32).to_le_bytes().to_vec()
                        }))
                    } else {
                        // struct.pack('ii', a, b) → { let mut v = Vec::new(); v.extend_from_slice(&(a as i32).to_le_bytes()); ... }
                        Ok(Some(parse_quote! {
                            {
                                let mut __struct_pack_result = Vec::new();
                                #(__struct_pack_result.extend_from_slice(&(#value_exprs as i32).to_le_bytes());)*
                                __struct_pack_result
                            }
                        }))
                    }
                } else {
                    bail!("struct.pack() requires string literal format (dynamic formats not supported)");
                }
            }
            "unpack" => {
                if args.len() != 2 {
                    bail!("struct.unpack() requires exactly 2 arguments (format, bytes)");
                }

                // First arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.unpack() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let bytes_expr = args[1].to_rust_expr(self.ctx)?;

                    if count == 1 {
                        // struct.unpack('i', bytes) → (i32::from_le_bytes(bytes[0..4].try_into().expect("...")),)
                        Ok(Some(parse_quote! {
                            (i32::from_le_bytes(#bytes_expr[0..4].try_into().expect("operation failed")),)
                        }))
                    } else if count == 2 {
                        // struct.unpack('ii', bytes) → (i32::from_le_bytes(...), i32::from_le_bytes(...))
                        Ok(Some(parse_quote! {
                            (
                                i32::from_le_bytes(#bytes_expr[0..4].try_into().expect("operation failed")),
                                i32::from_le_bytes(#bytes_expr[4..8].try_into().expect("operation failed")),
                            )
                        }))
                    } else {
                        bail!(
                            "struct.unpack() only supports 'i' and 'ii' formats (got {} ints)",
                            count
                        );
                    }
                } else {
                    bail!("struct.unpack() requires string literal format (dynamic formats not supported)");
                }
            }
            "calcsize" => {
                if args.len() != 1 {
                    bail!("struct.calcsize() requires exactly 1 argument");
                }

                // Arg is format string
                if let HirExpr::Literal(Literal::String(format)) = &args[0] {
                    let count = format.chars().filter(|&c| c == 'i').count();

                    if count == 0 {
                        bail!("struct.calcsize() format '{}' not supported (only 'i' and 'ii' implemented)", format);
                    }

                    let size = (count * 4) as i32;
                    Ok(Some(parse_quote! { #size }))
                } else {
                    bail!("struct.calcsize() requires string literal format (dynamic formats not supported)");
                }
            }
            _ => {
                bail!("struct.{} not implemented", method);
            }
        }
    }

    // DEPYLER-COVERAGE-95: try_convert_json_method moved to stdlib_method_gen::json

    // DEPYLER-COVERAGE-95: try_convert_re_method moved to stdlib_method_gen::regex_mod

    // DEPYLER-COVERAGE-95: try_convert_string_method moved to stdlib_method_gen::string

    // DEPYLER-COVERAGE-95: try_convert_time_method moved to stdlib_method_gen::time

    // DEPYLER-COVERAGE-95: try_convert_shutil_method moved to stdlib_method_gen::shutil

    /// Try to convert csv module method calls
    /// DEPYLER-STDLIB-CSV: CSV file reading and writing
    ///
    /// Maps Python csv module to Rust csv crate:
    /// - csv.reader() → csv::Reader::from_reader()
    /// - csv.writer() → csv::Writer::from_writer()
    /// - csv.DictReader → csv with headers
    /// - csv.DictWriter → csv with headers
    ///
    /// # Complexity
    /// 4 (match with 4 branches - simplified for core operations)
    #[inline]
    pub(crate) fn try_convert_csv_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need csv crate
        self.ctx.needs_csv = true;

        let result = match method {
            // CSV Reader
            "reader" => {
                if arg_exprs.is_empty() {
                    bail!("csv.reader() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.reader(file) → csv::Reader::from_reader(file)
                // Note: Real implementation needs more context for delimiter, etc.
                parse_quote! { csv::Reader::from_reader(#file) }
            }

            // CSV Writer
            "writer" => {
                if arg_exprs.is_empty() {
                    bail!("csv.writer() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.writer(file) → csv::Writer::from_writer(file)
                parse_quote! { csv::Writer::from_writer(#file) }
            }

            // DictReader (simplified - actual implementation more complex)
            "DictReader" => {
                if arg_exprs.is_empty() {
                    bail!("csv.DictReader() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // csv.DictReader(file) → csv::ReaderBuilder::new().has_headers(true).from_reader(file)
                parse_quote! {
                    csv::ReaderBuilder::new()
                        .has_headers(true)
                        .from_reader(#file)
                }
            }

            // DictWriter (simplified)
            // DEPYLER-0426: Handle both positional and keyword arguments
            // csv.DictWriter(file, fieldnames=[...]) or csv.DictWriter(file, fieldnames=...)
            "DictWriter" => {
                // Get file argument (first positional arg required)
                if arg_exprs.is_empty() {
                    bail!("csv.DictWriter() requires at least 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // Get fieldnames from either positional arg or kwargs
                let _fieldnames = if arg_exprs.len() >= 2 {
                    // Positional: csv.DictWriter(file, ['col1', 'col2'])
                    Some(&arg_exprs[1])
                } else {
                    // Keyword: csv.DictWriter(file, fieldnames=['col1', 'col2'])
                    kwargs
                        .iter()
                        .find(|(key, _)| key == "fieldnames")
                        .map(|(_, value)| value.to_rust_expr(self.ctx))
                        .transpose()?
                        .as_ref()
                        .map(|_| &arg_exprs[0]) // Placeholder, we don't use fieldnames yet
                };

                if _fieldnames.is_none() {
                    bail!("csv.DictWriter() requires fieldnames argument (positional or keyword)");
                }

                // csv.DictWriter(file, fieldnames) → csv::Writer::from_writer(file)
                // Note: fieldnames handling requires more context
                parse_quote! { csv::Writer::from_writer(#file) }
            }

            _ => {
                bail!("csv.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    // DEPYLER-COVERAGE-95: try_convert_os_method moved to stdlib_method_gen::os

    /// Try to convert os.environ method calls
    /// DEPYLER-0386: os.environ dictionary-like interface for environment variables
    ///
    /// Maps Python os.environ methods to Rust std::env:
    /// - os.environ.get(key) → std::env::var(key).ok()
    /// - os.environ.get(key, default) → std::env::var(key).unwrap_or_else(|_| default.to_string())
    ///
    /// # Complexity
    /// ≤10 (match with few branches)
    #[inline]
    pub(crate) fn try_convert_os_environ_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            "get" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.environ.get() requires 1 or 2 arguments");
                }

                if arg_exprs.len() == 1 {
                    // os.environ.get("KEY") → std::env::var("KEY").ok()
                    // Returns Option<String>: Some(value) if exists, None otherwise
                    // DEPYLER-0486: Handle Option-typed keys (e.g., from argparse nargs="?")
                    // If key is an &Option<String> or Option<String>, unwrap it first
                    let key = &arg_exprs[0];
                    let key_with_unwrap = if let HirExpr::Var(var_name) = &args[0] {
                        // DEPYLER-0644: Check if variable is already unwrapped (inside if-let body)
                        // If so, the key is already a concrete String, not Option<String>
                        // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
                        let is_unwrapped = self.ctx.option_unwrap_map.contains_key(var_name)
                            || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                        if is_unwrapped {
                            // Variable was already unwrapped, don't add .as_ref().unwrap()
                            key.clone()
                        } else if let Some(var_type) = self.ctx.var_types.get(var_name) {
                            if matches!(var_type, Type::Optional(_)) {
                                // Key is an Option type - unwrap it
                                parse_quote! { #key.as_ref().expect("value is None") }
                            } else {
                                key.clone()
                            }
                        } else {
                            key.clone()
                        }
                    } else {
                        key.clone()
                    };
                    parse_quote! { std::env::var(#key_with_unwrap).ok() }
                } else {
                    // os.environ.get("KEY", "default") → std::env::var("KEY").unwrap_or_else(|_| "default".to_string())
                    // Returns String: value if exists, default otherwise
                    // DEPYLER-0486: Auto-borrow variables (not string literals) to avoid move errors
                    let key = &arg_exprs[0];
                    let key_with_borrow = if matches!(&args[0], HirExpr::Var(_)) {
                        // Variable: borrow it to avoid moving in loops
                        parse_quote! { &#key }
                    } else {
                        // String literal or other expression: use as-is
                        key.clone()
                    };
                    let default = &arg_exprs[1];
                    parse_quote! {
                        std::env::var(#key_with_borrow).unwrap_or_else(|_| #default.to_string())
                    }
                }
            }
            _ => {
                return Ok(None);
            }
        };

        Ok(Some(result))
    }


    // DEPYLER-REFACTOR: try_convert_numpy_call, try_convert_numpy_call_nasa_mode moved to stdlib_numpy

    // DEPYLER-REFACTOR: try_convert_os_path_method moved to stdlib_os


    // DEPYLER-REFACTOR: bisect, heapq, copy methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_itertools_method moved to stdlib_method_gen::itertools
    // DEPYLER-COVERAGE-95: try_convert_functools_method moved to stdlib_method_gen::functools
    // DEPYLER-COVERAGE-95: try_convert_warnings_method moved to stdlib_method_gen::warnings

    // DEPYLER-REFACTOR: sys, pickle, pprint, fractions methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_pathlib_method moved to stdlib_method_gen::pathlib

    // DEPYLER-REFACTOR: convert_pathlib_instance_method moved to stdlib_pathlib


    // DEPYLER-REFACTOR: decimal, statistics methods moved to stdlib_misc
    // DEPYLER-COVERAGE-95: try_convert_random_method moved to stdlib_method_gen::random

    // DEPYLER-COVERAGE-95: try_convert_math_method moved to stdlib_method_gen::math

    /// Try to convert module method call (e.g., os.getcwd())
    #[inline]
    pub(crate) fn try_convert_module_method(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<Option<syn::Expr>> {
        // DEPYLER-0493: Handle constructor patterns for imported types
        // tempfile.NamedTempFile() → tempfile::NamedTempFile::new()
        if let HirExpr::Var(module_name) = object {
            // Check if this module is imported and has constructor pattern metadata
            if let Some(module_mapping) = self.ctx.imported_modules.get(module_name) {
                // Look up the Python name → Rust name mapping
                if let Some(rust_name) = module_mapping.item_map.get(method) {
                    // Check if this has a constructor pattern defined
                    if let Some(constructor_pattern) =
                        module_mapping.constructor_patterns.get(rust_name)
                    {
                        use crate::module_mapper::ConstructorPattern;

                        // Clone what we need to avoid borrow checker issues
                        let rust_path_str = format!("{}::{}", module_mapping.rust_path, rust_name);
                        let constructor_pattern_owned = constructor_pattern.clone();
                        let rust_name_owned = rust_name.clone(); // DEPYLER-0534: Clone for later use

                        // Build the full Rust path
                        let path_parts: Vec<&str> = rust_path_str.split("::").collect();
                        let mut path = quote! {};
                        for (i, part) in path_parts.iter().enumerate() {
                            let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                            if i == 0 {
                                path = quote! { #part_ident };
                            } else {
                                path = quote! { #path::#part_ident };
                            }
                        }

                        // Convert arguments
                        let arg_exprs: Vec<syn::Expr> = args
                            .iter()
                            .map(|arg| arg.to_rust_expr(self.ctx))
                            .collect::<Result<Vec<_>>>()?;

                        // GH-204: Handle collections module constructors specially
                        // Counter, deque, and defaultdict need custom conversion, not generic new()
                        if module_name == "collections" {
                            match method {
                                "Counter" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_counter_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                "deque" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_deque_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                "defaultdict" => {
                                    return Ok(Some(
                                        crate::rust_gen::collection_constructors::convert_defaultdict_builtin(
                                            self.ctx,
                                            &arg_exprs,
                                        )?,
                                    ));
                                }
                                _ => {} // Fall through to generic pattern handling
                            }
                        }

                        // Generate call based on constructor pattern
                        let result = match constructor_pattern_owned {
                            ConstructorPattern::New => {
                                // Struct type → use ::new() pattern
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path::new() }
                                } else {
                                    parse_quote! { #path::new(#(#arg_exprs),*) }
                                }
                            }
                            ConstructorPattern::Method(method_name) => {
                                // Custom method (e.g., File::open())
                                let method_ident =
                                    syn::Ident::new(&method_name, proc_macro2::Span::call_site());
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path::#method_ident() }
                                } else {
                                    parse_quote! { #path::#method_ident(#(#arg_exprs),*) }
                                }
                            }
                            ConstructorPattern::Function => {
                                // Regular function call
                                if arg_exprs.is_empty() {
                                    parse_quote! { #path() }
                                } else {
                                    parse_quote! { #path(#(#arg_exprs),*) }
                                }
                            }
                        };

                        // DEPYLER-0534: Unwrap fallible constructors
                        // tempfile functions return Result<T, io::Error>
                        // Use .unwrap() for simplicity (matches Python's exception-on-failure behavior)
                        let is_fallible_constructor = module_name == "tempfile"
                            && (rust_name_owned == "NamedTempFile"
                                || rust_name_owned == "TempFile"
                                || rust_name_owned == "TempDir");

                        // DEPYLER-1002: Set needs_tempfile when using tempfile constructors
                        if module_name == "tempfile" {
                            self.ctx.needs_tempfile = true;
                        }

                        let result = if is_fallible_constructor {
                            parse_quote! { #result.expect("operation failed") }
                        } else {
                            result
                        };

                        return Ok(Some(result));
                    }
                }
            }
        }

        // DEPYLER-0386: Handle os.environ.get() and other os.environ methods
        // os.environ.get('VAR') → std::env::var('VAR').ok()
        // os.environ.get('VAR', 'default') → std::env::var('VAR').unwrap_or_else(|_| 'default'.to_string())
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    return self.try_convert_os_environ_method(method, args);
                }
                // DEPYLER-0430: Handle os.path.exists(), os.path.join(), etc.
                // os.path.exists(path) → Path::new(path).exists()
                // os.path.join(a, b) → PathBuf::from(a).join(b)
                if module_name == "os" && attr == "path" {
                    return self.try_convert_os_path_method(method, args);
                }
                // DEPYLER-0553: Handle datetime.datetime.method() calls
                // datetime.datetime.fromtimestamp(ts) → chrono::DateTime::from_timestamp(ts, 0)
                // datetime.datetime.now() → chrono::Local::now()
                if module_name == "datetime" && attr == "datetime" {
                    return self.try_convert_datetime_method(method, args);
                }
            }
        }

        if let HirExpr::Var(module_name) = object {
            // DEPYLER-0021: Handle struct module (pack, unpack, calcsize)
            if module_name == "struct" {
                return self.try_convert_struct_method(method, args);
            }

            // DEPYLER-STDLIB-MATH: Handle math module functions
            // math.sqrt(x) → x.sqrt()
            // math.sin(x) → x.sin()
            // math.pow(x, y) → x.powf(y)
            if module_name == "math" {
                return stdlib_method_gen::convert_math_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-RANDOM: Handle random module functions
            // random.random() → thread_rng().gen()
            // random.randint(a, b) → thread_rng().gen_range(a..=b)
            if module_name == "random" {
                return stdlib_method_gen::convert_random_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-STATISTICS: Handle statistics module functions
            // statistics.mean(data) → inline calculation
            // statistics.median(data) → sorted median calculation
            if module_name == "statistics" {
                return self.try_convert_statistics_method(method, args);
            }

            // DEPYLER-STDLIB-FRACTIONS: Handle fractions module functions
            // Fraction(1, 2) → Ratio::new(1, 2)
            // f.limit_denominator(100) → approximate with max denominator
            if module_name == "fractions" {
                return self.try_convert_fractions_method(method, args);
            }

            // DEPYLER-STDLIB-PATHLIB: Handle pathlib module functions
            // Path("/foo/bar").exists() → PathBuf::from("/foo/bar").exists()
            // Path("/foo").join("bar") → PathBuf::from("/foo").join("bar")
            if module_name == "pathlib" {
                return stdlib_method_gen::convert_pathlib_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-DATETIME: Handle datetime module functions
            // datetime.datetime.now() → Local::now().naive_local()
            // datetime.datetime.utcnow() → Utc::now().naive_utc()
            // datetime.date.today() → Local::now().date_naive()
            // DEPYLER-0594: Also handle "date" and "time" when imported directly
            // (from datetime import date; date.today())
            // DEPYLER-0188: Don't match module_name == "time" here - that's the time module!
            // Only match "date" for `from datetime import date` pattern.
            // The time module (import time; time.time()) is handled separately below.

            // DEPYLER-1069: Handle date.min/max vs datetime.min/max specially
            // date.min → DepylerDate(1, 1, 1), datetime.min → DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0)
            if (module_name == "datetime" || module_name == "date")
                && (method == "min" || method == "max")
            {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if module_name == "date" {
                    // date.min / date.max
                    if nasa_mode {
                        self.ctx.needs_depyler_date = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { DepylerDate::new(1, 1, 1) }
                        } else {
                            parse_quote! { DepylerDate::new(9999, 12, 31) }
                        }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { chrono::NaiveDate::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDate::MAX }
                        }));
                    }
                } else {
                    // datetime.min / datetime.max
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { DepylerDateTime::new(1, 1, 1, 0, 0, 0, 0) }
                        } else {
                            parse_quote! { DepylerDateTime::new(9999, 12, 31, 23, 59, 59, 999999) }
                        }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(if method == "min" {
                            parse_quote! { chrono::NaiveDateTime::MIN }
                        } else {
                            parse_quote! { chrono::NaiveDateTime::MAX }
                        }));
                    }
                }
            }

            // DEPYLER-1069: Handle date.today() vs datetime.today() separately
            // date.today() → DepylerDate::today(), datetime.today() → DepylerDateTime::today()
            if (module_name == "datetime" || module_name == "date")
                && method == "today"
                && args.is_empty()
            {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if module_name == "date" {
                    if nasa_mode {
                        self.ctx.needs_depyler_date = true;
                        return Ok(Some(parse_quote! { DepylerDate::today() }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(parse_quote! { chrono::Local::now().date_naive() }));
                    }
                } else {
                    // datetime.today()
                    if nasa_mode {
                        self.ctx.needs_depyler_datetime = true;
                        return Ok(Some(parse_quote! { DepylerDateTime::today() }));
                    } else {
                        self.ctx.needs_chrono = true;
                        return Ok(Some(parse_quote! { chrono::Local::now().naive_local() }));
                    }
                }
            }

            if module_name == "datetime" || module_name == "date" {
                return self.try_convert_datetime_method(method, args);
            }

            // DEPYLER-1069: Handle datetime.time class attributes (min, max)
            // These are only valid for datetime.time class, not the time module
            // time.min → (0, 0, 0, 0), time.max → (23, 59, 59, 999999)
            if module_name == "time" && (method == "min" || method == "max") {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if nasa_mode {
                    // Return tuple (hour, minute, second, microsecond)
                    return Ok(Some(if method == "min" {
                        parse_quote! { (0u32, 0u32, 0u32, 0u32) }
                    } else {
                        parse_quote! { (23u32, 59u32, 59u32, 999999u32) }
                    }));
                } else {
                    self.ctx.needs_chrono = true;
                    return Ok(Some(if method == "min" {
                        parse_quote! { chrono::NaiveTime::MIN }
                    } else {
                        parse_quote! { chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999999).expect("invalid time") }
                    }));
                }
            }

            // DEPYLER-0595: Handle bytes class methods
            // bytes.fromhex("aabbcc") → hex string to byte array
            if module_name == "bytes" && method == "fromhex" && args.len() == 1 {
                let hex_str = args[0].to_rust_expr(self.ctx)?;
                // Convert hex string to Vec<u8> using inline parsing
                return Ok(Some(parse_quote! {
                    (#hex_str).as_bytes()
                        .chunks(2)
                        .map(|c| u8::from_str_radix(std::str::from_utf8(c).expect("parse failed"), 16).expect("parse failed"))
                        .collect::<Vec<u8>>()
                }));
            }

            // DEPYLER-STDLIB-DECIMAL: Handle decimal module functions
            // decimal.Decimal("123.45") → Decimal::from_str("123.45")
            // Note: Decimal() constructor is handled separately in convert_call
            if module_name == "decimal" {
                return self.try_convert_decimal_method(method, args);
            }

            // DEPYLER-STDLIB-JSON: Handle json module functions
            // json.dumps(obj) → serde_json::to_string(&obj)
            // json.loads(s) → serde_json::from_str(&s)
            if module_name == "json" {
                return stdlib_method_gen::convert_json_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-RE: Regular expressions module
            if module_name == "re" {
                return stdlib_method_gen::convert_re_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-STRING: String module utilities
            if module_name == "string" {
                return stdlib_method_gen::convert_string_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-TIME: Time module
            if module_name == "time" {
                return stdlib_method_gen::convert_time_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-SHUTIL: Shell utilities for file operations
            // shutil.copy(src, dst) → std::fs::copy(src, dst)
            // shutil.copy2(src, dst) → std::fs::copy(src, dst)
            // shutil.move(src, dst) → std::fs::rename(src, dst)
            if module_name == "shutil" {
                return stdlib_method_gen::convert_shutil_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-CSV: CSV file operations
            // DEPYLER-0426: Pass kwargs for DictWriter(file, fieldnames=...)
            if module_name == "csv" {
                return self.try_convert_csv_method(method, args, kwargs);
            }

            // DEPYLER-0380: os module operations (getenv, etc.)
            // Must be checked before os.path to handle non-path os functions
            if module_name == "os" {
                if let Some(result) = stdlib_method_gen::convert_os_method(method, args, self.ctx)?
                {
                    return Ok(Some(result));
                }
                // Fall through to os.path handler if method not recognized
            }

            // DEPYLER-STDLIB-OSPATH: os.path file system operations
            // Only match the actual module "os.path", not variables named "path"
            // Variables named "path" are typically PathBuf instances from Path() constructor
            if module_name == "os.path" {
                return self.try_convert_os_path_method(method, args);
            }

            // DEPYLER-STDLIB-BASE64: Base64 encoding/decoding operations
            if module_name == "base64" {
                return self.try_convert_base64_method(method, args);
            }

            // DEPYLER-STDLIB-SECRETS: Cryptographically strong random operations
            if module_name == "secrets" {
                return self.try_convert_secrets_method(method, args);
            }

            // DEPYLER-STDLIB-HASHLIB: Cryptographic hash functions
            if module_name == "hashlib" {
                return self.try_convert_hashlib_method(method, args);
            }

            // DEPYLER-STDLIB-UUID: UUID generation (RFC 4122)
            if module_name == "uuid" {
                return self.try_convert_uuid_method(method, args);
            }

            // DEPYLER-STDLIB-HMAC: HMAC authentication
            if module_name == "hmac" {
                return self.try_convert_hmac_method(method, args);
            }

            // DEPYLER-0430: platform module - system information
            if module_name == "platform" {
                return self.try_convert_platform_method(method, args);
            }

            // DEPYLER-STDLIB-BINASCII: Binary/ASCII conversions
            if module_name == "binascii" {
                return self.try_convert_binascii_method(method, args);
            }

            // DEPYLER-STDLIB-URLLIB-PARSE: URL parsing and encoding
            if module_name == "urllib.parse" || module_name == "parse" {
                return self.try_convert_urllib_parse_method(method, args);
            }

            // DEPYLER-STDLIB-FNMATCH: Unix shell-style pattern matching
            if module_name == "fnmatch" {
                return self.try_convert_fnmatch_method(method, args);
            }

            // DEPYLER-STDLIB-SHLEX: Shell command line lexing
            if module_name == "shlex" {
                return self.try_convert_shlex_method(method, args);
            }

            // DEPYLER-STDLIB-TEXTWRAP: Text wrapping and formatting
            if module_name == "textwrap" {
                return self.try_convert_textwrap_method(method, args);
            }

            // DEPYLER-STDLIB-BISECT: Binary search for sorted sequences
            if module_name == "bisect" {
                return self.try_convert_bisect_method(method, args);
            }

            // DEPYLER-STDLIB-HEAPQ: Heap queue algorithm (priority queue)
            if module_name == "heapq" {
                return self.try_convert_heapq_method(method, args);
            }

            // DEPYLER-STDLIB-COPY: Shallow and deep copy operations
            if module_name == "copy" {
                return self.try_convert_copy_method(method, args);
            }

            // DEPYLER-STDLIB-ITERTOOLS: Iterator combinatorics and lazy evaluation
            if module_name == "itertools" {
                return stdlib_method_gen::convert_itertools_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-FUNCTOOLS: Higher-order functions
            if module_name == "functools" {
                return stdlib_method_gen::convert_functools_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-WARNINGS: Warning control
            if module_name == "warnings" {
                return stdlib_method_gen::convert_warnings_method(method, args, self.ctx);
            }

            // DEPYLER-STDLIB-SYS: System-specific parameters and functions
            if module_name == "sys" {
                return self.try_convert_sys_method(method, args);
            }

            // DEPYLER-STDLIB-PICKLE: Object serialization
            if module_name == "pickle" {
                return self.try_convert_pickle_method(method, args);
            }

            // DEPYLER-STDLIB-PPRINT: Pretty printing
            if module_name == "pprint" {
                return self.try_convert_pprint_method(method, args);
            }

            // DEPYLER-0424: Calendar module - date/time calculations
            if module_name == "calendar" {
                return self.try_convert_calendar_method(method, args);
            }

            // DEPYLER-0335 FIX #2: Get rust_path and rust_name before converting args (avoid borrow conflict)
            let module_info = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| {
                    mapping
                        .item_map
                        .get(method)
                        .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
                });

            if let Some((rust_path, rust_name)) = module_info {
                // Convert args
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                // DEPYLER-0335 FIX #2: Special handling for math module functions (use method syntax)
                // Python: math.sqrt(x) → Rust: x.sqrt() or f64::sqrt(x)
                if module_name == "math" && !arg_exprs.is_empty() {
                    let receiver = &arg_exprs[0];
                    let method_ident = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());
                    return Ok(Some(parse_quote! { (#receiver).#method_ident() }));
                }

                // DEPYLER-0335 FIX #2: Use rust_path from mapping instead of hardcoding "std"
                // Build the Rust function path using the module's rust_path

                // DEPYLER-0840: Handle macro names (ending with !) specially
                // Macros like "join!" cannot be split and used as identifiers
                if rust_name.ends_with('!') {
                    // This is a macro - handle it specially
                    // For now, skip macro-based mappings as they need special handling
                    // Note: Implement proper macro invocation support
                    return Ok(None);
                }

                let path_parts: Vec<&str> = rust_name.split("::").collect();

                // Start with the module's rust_path instead of hardcoded "std"
                let base_path: syn::Path =
                    syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                let mut path = quote! { #base_path };

                for part in path_parts {
                    let part_ident = syn::Ident::new(part, proc_macro2::Span::call_site());
                    path = quote! { #path::#part_ident };
                }

                // Special handling for certain functions
                let result = match rust_name.as_str() {
                    "env::current_dir" => {
                        // current_dir returns Result<PathBuf>, we need to convert to String
                        parse_quote! {
                            #path().expect("operation failed").to_string_lossy().to_string()
                        }
                    }
                    "Regex::new" => {
                        // re.compile(pattern) -> Regex::new(pattern)
                        if arg_exprs.is_empty() {
                            bail!("re.compile() requires a pattern argument");
                        }
                        let pattern = &arg_exprs[0];
                        parse_quote! {
                            regex::Regex::new(#pattern).expect("parse failed")
                        }
                    }
                    _ => {
                        if arg_exprs.is_empty() {
                            parse_quote! { #path() }
                        } else {
                            parse_quote! { #path(#(#arg_exprs),*) }
                        }
                    }
                };
                return Ok(Some(result));
            }
        }
        Ok(None)
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

fn literal_to_rust_expr(
    lit: &Literal,
    string_optimizer: &StringOptimizer,
    _needs_cow: &bool,
    ctx: &CodeGenContext,
) -> syn::Expr {
    // Note: We intentionally use unsuffixed literals to let Rust's type inference
    // determine the appropriate type from context. Adding explicit suffixes (like _i32)
    // caused more problems than it solved - trait resolution failures, overflow for
    // large numbers, and type mismatches with DepylerValue containers.
    let _ = ctx; // Suppress unused warning
    match lit {
        Literal::Int(n) => {
            let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
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

mod stdlib_crypto;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests;
