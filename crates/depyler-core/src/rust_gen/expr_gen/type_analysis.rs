//! Type analysis helpers for ExpressionConverter
//!
//! Contains deref_if_borrowed_param, coerce_int_to_float_if_needed,
//! is_int_expr, is_float_var, infer_iterable_element_type, borrow helpers.

use crate::hir::*;
use crate::rust_gen::context::CodeGenContext;
use crate::rust_gen::return_type_expects_float;
use anyhow::Result;
use syn::parse_quote;

use super::ExpressionConverter;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
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


}
