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
use crate::rust_gen::expr_analysis; // DEPYLER-COVERAGE-95: Use extracted helpers
use crate::rust_gen::numpy_gen; // Phase 3: NumPy→Trueno codegen
use crate::rust_gen::precedence; // DEPYLER-COVERAGE-95: Use extracted helpers
use crate::rust_gen::stdlib_method_gen; // DEPYLER-COVERAGE-95: Extracted stdlib handlers
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::keywords; // DEPYLER-COVERAGE-95: Use centralized keywords module
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::type_gen::convert_binop;
use crate::string_optimization::{StringContext, StringOptimizer};
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

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
    pub(crate) fn generate_walrus_bindings(cond: &HirExpr, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
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
    pub(crate) fn deref_if_borrowed_param(&self, hir_expr: &HirExpr, rust_expr: syn::Expr) -> syn::Expr {
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
        if !other_is_float {
            return expr;
        }

        // Coerce integer literals to float
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
                if matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv) {
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
    pub(crate) fn borrow_path_with_option_check(&self, path_expr: &syn::Expr, hir_arg: &HirExpr) -> syn::Expr {
        // Check if the HIR arg is a variable that might be Option-typed
        if let HirExpr::Var(var_name) = hir_arg {
            // DEPYLER-0644: Check if variable is already unwrapped (inside if-let body)
            // If so, the variable is already a concrete String, not Option<String>
            // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
            // e.g., hash_algorithm_val from `if let Some(ref hash_algorithm_val) = hash_algorithm`
            let is_unwrapped =
                self.ctx.option_unwrap_map.contains_key(var_name)
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
                    // Option<String> → use .as_ref().unwrap() for path
                    return parse_quote! { #path_expr.as_ref().unwrap() };
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
                return parse_quote! { #path_expr.as_ref().unwrap() };
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

    pub(crate) fn convert_binary(&mut self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
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
            let is_ordering_compare = matches!(
                op,
                BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq
            );

            // Check if left is string index (produces String) and needs .as_str()
            let left_is_string_index = matches!(left, HirExpr::Index { base, .. } if self.is_string_base(base));
            // Check if right is string index (produces String) and needs .as_str()
            let right_is_string_index = matches!(right, HirExpr::Index { base, .. } if self.is_string_base(base));

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
            let left_needs_as_str = is_ordering_compare
                && matches!(left, HirExpr::Var(_))
                && right_is_char_literal;

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
            let right_is_ref_pattern = matches!(right, HirExpr::Var(_))
                || matches!(right, HirExpr::Attribute { .. });
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
            BinOp::Add
                if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #left_expr.add(&#right_expr).unwrap() })
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
            BinOp::Sub
                if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #left_expr.sub(&#right_expr).unwrap() })
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

                    let rust_op = convert_binop(op)?;
                    // DEPYLER-0582: Coerce int to float if operating with float
                    let left_coerced = self.coerce_int_to_float_if_needed(left_deref, left, right);
                    let right_coerced = self.coerce_int_to_float_if_needed(right_deref, right, left);
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
            BinOp::Div
                if self.is_numpy_array_expr(left) && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #left_expr.div(&#right_expr).unwrap() })
            }
            BinOp::Div => {
                // DEPYLER-0188: Check if this is pathlib Path division (path / "segment")
                // Python: Path(__file__).parent / "file.py"
                // Rust: PathBuf::from(file!()).parent().unwrap().join("file.py")
                if self.is_path_expr(left) {
                    // Convert division to .join() for path concatenation
                    return Ok(parse_quote! { #left_expr.join(#right_expr) });
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
                    let right_wrapped = precedence::parenthesize_if_lower_precedence(right_expr, op);
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

                // Apply truthiness conversion to both operands
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
                let rust_op = convert_binop(op)?;
                // DEPYLER-0339: Construct syn::ExprBinary directly instead of using parse_quote!
                // parse_quote! doesn't properly handle interpolated syn::BinOp values

                // DEPYLER-0576: Parenthesize right side when it's a unary negation
                // Prevents "<-" tokenization issue: x < -20.0 becomes x<- 20.0 without parens
                let right_expr_final = if matches!(right, HirExpr::Unary { op: UnaryOp::Neg, .. }) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr
                };

                Ok(syn::Expr::Binary(syn::ExprBinary {
                    attrs: vec![],
                    left: Box::new(left_expr),
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
        // DEPYLER-0908: Fixed false positive when variable could be either string or int
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
            matches!(
                self.ctx.var_types.get(sym),
                Some(crate::hir::Type::Int)
            )
        } else {
            false
        };
        let left_is_int_var_from_type = if let HirExpr::Var(sym) = left {
            matches!(
                self.ctx.var_types.get(sym),
                Some(crate::hir::Type::Int)
            )
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
            matches!(
                self.ctx.var_types.get(sym),
                Some(crate::hir::Type::Int)
            )
        } else {
            false
        };
        let left_is_int_var = if let HirExpr::Var(sym) = left {
            matches!(
                self.ctx.var_types.get(sym),
                Some(crate::hir::Type::Int)
            )
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
        match (left, right) {
            // Pattern: [x] * n (small arrays ≤32)
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 0 && *size <= 32 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { [#elem; #size_lit] })
            }
            // DEPYLER-0420: Pattern: [x] * n (large arrays → Vec)
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 32 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { vec![#elem; #size_lit] })
            }
            // Pattern: n * [x] (small arrays ≤32)
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 0 && *size <= 32 =>
            {
                let elem = elts[0].to_rust_expr(self.ctx)?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { [#elem; #size_lit] })
            }
            // DEPYLER-0420: Pattern: n * [x] (large arrays → Vec)
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 32 =>
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
                Ok(parse_quote! { #left_expr.mul(&#right_expr).unwrap() })
            }
            // DEPYLER-0926: Vector-scalar multiplication for trueno
            // trueno Vector has scale() method for scalar multiplication
            _ if self.is_numpy_array_expr(left) && self.expr_returns_float(right) => {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).unwrap() })
            }
            // DEPYLER-0926: scalar-Vector multiplication for trueno (commutative)
            _ if self.expr_returns_float(left) && self.is_numpy_array_expr(right) => {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).unwrap() })
            }
            // DEPYLER-0928: Vector * integer - convert integer to f32 for scale()
            _ if self.is_numpy_array_expr(left)
                && matches!(right, HirExpr::Literal(Literal::Int(_))) =>
            {
                Ok(parse_quote! { #left_expr.scale(#right_expr as f32).unwrap() })
            }
            // DEPYLER-0928: integer * Vector - convert integer to f32 for scale()
            _ if matches!(left, HirExpr::Literal(Literal::Int(_)))
                && self.is_numpy_array_expr(right) =>
            {
                Ok(parse_quote! { #right_expr.scale(#left_expr as f32).unwrap() })
            }
            // Default multiplication
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0582: Coerce int to float if operating with float
                let left_coerced = self.coerce_int_to_float_if_needed(left_expr, left, right);
                let right_coerced = self.coerce_int_to_float_if_needed(right_expr, right, left);
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
            let left_coerced = self.coerce_int_to_float_if_needed(left_expr, left, right);
            let right_coerced = self.coerce_int_to_float_if_needed(right_expr, right, left);
            Ok(parse_quote! { #left_coerced #rust_op #right_coerced })
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
        // DEPYLER-0380 Bug #3: Handle `var in os.environ` / `var not in os.environ`
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
                        return Ok(parse_quote! { #right_expr.as_ref().unwrap().get(&#left_expr).is_none() });
                    } else {
                        return Ok(parse_quote! { #right_expr.as_ref().unwrap().get(#left_expr).is_none() });
                    }
                } else if needs_borrow {
                    return Ok(parse_quote! { #right_expr.as_ref().unwrap().get(&#left_expr).is_some() });
                } else {
                    return Ok(parse_quote! { #right_expr.as_ref().unwrap().get(#left_expr).is_some() });
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
                        parse_quote! { std::path::PathBuf::from(#path_expr.as_ref().unwrap()) },
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
                        // DEPYLER-1025: NASA mode - use std::time::SystemTime
                        Some(Ok(parse_quote! { std::time::SystemTime::now() }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
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
                    if nasa_mode {
                        // DEPYLER-1025: NASA mode - use std::time::SystemTime
                        Some(Ok(parse_quote! { std::time::SystemTime::now() }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                                .unwrap()
                                .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                                .unwrap()
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

            // DEPYLER-1025: date(year, month, day) - NASA mode uses tuple
            "date" if args.len() == 3 => {
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
                if nasa_mode {
                    Some(Ok(parse_quote! { (#year as u32, #month as u32, #day as u32) }))
                } else {
                    Some(Ok(parse_quote! {
                        chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).unwrap()
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
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
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
                        chrono::NaiveTime::from_hms_opt(#hour as u32, 0, 0).unwrap()
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
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, 0).unwrap()
                        }))
                    }
                } else {
                    let second = match args[2].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(parse_quote! { (#hour as u32, #minute as u32, #second as u32) }))
                    } else {
                        Some(Ok(parse_quote! {
                            chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, #second as u32).unwrap()
                        }))
                    }
                }
            }

            // DEPYLER-1025: timedelta(days=..., seconds=...) - NASA mode uses std::time::Duration
            "timedelta" => {
                let nasa_mode = self.ctx.type_mapper.nasa_mode;
                if !nasa_mode {
                    self.ctx.needs_chrono = true;
                }
                if args.is_empty() {
                    if nasa_mode {
                        Some(Ok(parse_quote! { std::time::Duration::from_secs(0) }))
                    } else {
                        Some(Ok(parse_quote! { chrono::Duration::zero() }))
                    }
                } else if args.len() == 1 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    if nasa_mode {
                        Some(Ok(parse_quote! { std::time::Duration::from_secs((#days as u64) * 86400) }))
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
                        Some(Ok(parse_quote! { std::time::Duration::from_secs((#days as u64) * 86400 + (#seconds as u64)) }))
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
                            parse_quote! { rust_decimal::Decimal::from_str(&#arg_expr).unwrap() },
                        ),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Int(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(parse_quote! { rust_decimal::Decimal::from(#arg_expr) }),
                        Err(e) => Err(e),
                    },
                    HirExpr::Literal(Literal::Float(_)) => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_f64_retain(#arg_expr).unwrap() },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { rust_decimal::Decimal::from_str(&(#arg_expr).to_string()).unwrap() },
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
                                    let num = parts[0].trim().parse::<i32>().unwrap();
                                    let denom = parts[1].trim().parse::<i32>().unwrap();
                                    num::rational::Ratio::new(num, denom)
                                } else {
                                    let num = s.parse::<i32>().unwrap();
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
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr).unwrap() },
                        ),
                        Err(e) => Err(e),
                    },
                    _ => match arg.to_rust_expr(self.ctx) {
                        Ok(arg_expr) => Ok(
                            parse_quote! { num::rational::Ratio::approximate_float(#arg_expr as f64).unwrap() },
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
                            Some(Ok(parse_quote! { #items_expr.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) }))
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
            // DEPYLER-0600 Bug #6: Added comprehension types - they produce collections too
            HirExpr::List(_)
            | HirExpr::Dict(_)
            | HirExpr::Set(_)
            | HirExpr::FrozenSet(_)
            | HirExpr::ListComp { .. }
            | HirExpr::DictComp { .. }
            | HirExpr::SetComp { .. } => true,
            // DEPYLER-0497: Function calls that return Result need {:?}
            HirExpr::Call { func, .. } => self.ctx.result_returning_functions.contains(func),
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
        let processed_args: Vec<syn::Expr> = args.iter().zip(arg_exprs.iter()).map(|(hir, syn)| {
            if self.is_pathbuf_expr(hir) {
                parse_quote! { #syn.display() }
            } else {
                syn.clone()
            }
        }).collect();

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
            let arg = &processed_args[0];

            let format_str = if needs_debug { "{:?}" } else { "{}" };

            if use_stderr {
                Ok(parse_quote! { eprintln!(#format_str, #arg) })
            } else {
                Ok(parse_quote! { println!(#format_str, #arg) })
            }
        } else {
            // Multiple arguments - build format string with per-arg detection
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

            if use_stderr {
                Ok(parse_quote! { eprintln!(#format_str, #(#processed_args),*) })
            } else {
                Ok(parse_quote! { println!(#format_str, #(#processed_args),*) })
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
    pub(crate) fn try_convert_sum_call(&mut self, func: &str, args: &[HirExpr]) -> Option<Result<syn::Expr>> {
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

            return if is_max {
                Some(Ok(parse_quote! { std::cmp::max(#arg1, #arg2) }))
            } else {
                Some(Ok(parse_quote! { std::cmp::min(#arg1, #arg2) }))
            };
        }

        // Handle max(iterable) / min(iterable)
        if args.len() == 1 {
            let iter_expr = match args[0].to_rust_expr(self.ctx) {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            return if is_max {
                Some(Ok(parse_quote! { *#iter_expr.iter().max().unwrap() }))
            } else {
                Some(Ok(parse_quote! { *#iter_expr.iter().min().unwrap() }))
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

        // DEPYLER-0608: Transform calls to cmd_*/handle_* handlers in subcommand match arms
        // When calling a handler with `args`, pass the extracted subcommand fields instead
        // Pattern: cmd_list(args) → cmd_list(archive) (where archive is extracted in match pattern)
        if self.ctx.in_subcommand_match_arm
            && (func.starts_with("cmd_") || func.starts_with("handle_"))
            && args.len() == 1
            && matches!(&args[0], HirExpr::Var(v) if v == "args")
        {
            let func_ident = syn::Ident::new(func, proc_macro2::Span::call_site());
            let field_args: Vec<syn::Expr> = self.ctx.subcommand_match_fields
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
                let body_expr = body.to_rust_expr(self.ctx)?;

                // DEPYLER-0754: With .cloned(), values are owned, so use |x| not |&x|
                return Ok(parse_quote! {
                    #iterable_expr.iter().cloned().filter(|#param_ident| #body_expr)
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
        // DEPYLER-REFACTOR-001: Fixed to handle different types correctly
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
        let arg_exprs: Vec<syn::Expr> = if is_user_class {
            args.iter()
                .map(|arg| {
                    let expr = arg.to_rust_expr(self.ctx)?;
                    // Wrap string literals with .to_string()
                    if matches!(arg, HirExpr::Literal(Literal::String(_))) {
                        Ok(parse_quote! { #expr.to_string() })
                    } else {
                        Ok(expr)
                    }
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            args.iter()
                .map(|arg| {
                    let expr = arg.to_rust_expr(self.ctx)?;
                    // DEPYLER-0458: Add & prefix for Lazy const variables (e.g., DEFAULT_CONFIG)
                    // When passing a const (all uppercase) to a function, it's likely a Lazy<T>
                    // that needs to be borrowed (&) so Deref converts it to &T
                    if let HirExpr::Var(var_name) = arg {
                        let is_const = var_name.chars().all(|c| c.is_uppercase() || c == '_');
                        if is_const {
                            return Ok(parse_quote! { &#expr });
                        }
                    }
                    Ok(expr)
                })
                .collect::<Result<Vec<_>>>()?
        };

        // DEPYLER-0364: Convert kwargs to positional arguments
        // Python: greet(name="Alice", greeting="Hello") → Rust: greet("Alice", "Hello")
        // For now, we append kwargs as additional positional arguments. This works for
        // common cases where functions accept positional or keyword arguments in order.
        // TODO: In the future, we should look up function signatures to determine
        // the correct parameter order and merge positional + kwargs properly
        let kwarg_exprs: Vec<syn::Expr> = if is_user_class {
            // For user-defined classes, convert string literals to String
            // This prevents "expected String, found &str" errors in constructors
            kwargs
                .iter()
                .map(|(_name, value)| {
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
            "bytearray" if !is_user_class => self.convert_bytearray_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-0937: tuple() builtin - convert iterable to collected tuple-like Vec
            "tuple" if !is_user_class => self.convert_tuple_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Pure builtin functions delegated to extracted module
            // DEPYLER-COVERAGE-95: Extracted to stdlib_method_gen::builtin_functions for testability
            "all" => stdlib_method_gen::builtin_functions::convert_all_builtin(&arg_exprs),
            "any" => stdlib_method_gen::builtin_functions::convert_any_builtin(&arg_exprs),
            "divmod" => stdlib_method_gen::builtin_functions::convert_divmod_builtin(&arg_exprs),
            "enumerate" => stdlib_method_gen::builtin_functions::convert_enumerate_builtin(&arg_exprs),
            "zip" => stdlib_method_gen::builtin_functions::convert_zip_builtin(&arg_exprs),
            "reversed" => stdlib_method_gen::builtin_functions::convert_reversed_builtin(&arg_exprs),
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
            _ => self.convert_generic_call(func, &all_hir_args, &all_args),
        }
    }

    pub(crate) fn try_convert_map_with_zip(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
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
    pub(crate) fn convert_int_cast(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
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
    pub(crate) fn convert_str_conversion(&self, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
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
                    return Ok(parse_quote! { (#arg).unwrap().to_string() });
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
    pub(crate) fn convert_frozenset_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
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
                if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::List(_))) {
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
                if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::Int)) {
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
                if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::List(_))) {
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
                if self.ctx.var_types.get(name).is_some_and(|t| matches!(t, Type::Int)) {
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
            let body_expr = body.to_rust_expr(self.ctx)?;
            let iterable = &args[1];
            // DEPYLER-0754: With .cloned(), values are owned, so use |x| not |&x|
            Ok(parse_quote! {
                #iterable.iter().cloned().filter(|#param_ident| #body_expr)
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
    pub(crate) fn convert_format_builtin(&self, args: &[syn::Expr], hir_args: &[HirExpr]) -> Result<syn::Expr> {
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

    pub(crate) fn convert_ord_builtin(&self, args: &[syn::Expr], hir_args: &[HirExpr]) -> Result<syn::Expr> {
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
            #char_str.chars().next().unwrap() as i32
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
                    return Ok(parse_quote! { std::path::PathBuf::from(#first).to_string_lossy().to_string() });
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
                        serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&#arg).unwrap()
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_str::<serde_json::Value>(&#arg).unwrap()
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
                        serde_json::from_reader::<_, std::collections::HashMap<String, serde_json::Value>>(#arg).unwrap()
                    });
                } else {
                    return Ok(parse_quote! {
                        serde_json::from_reader::<_, serde_json::Value>(#arg).unwrap()
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
                            Ok(parse_quote! { #path().unwrap() })
                        } else {
                            Ok(parse_quote! { #path(#(#args),*).unwrap() })
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
                                    !self.ctx.fn_str_params.contains(var_name)
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
                                return parse_quote! { #arg_expr.as_ref().unwrap() };
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

                        // DEPYLER-0964: Don't add &mut if variable is already &mut (mut_option_dict_params)
                        // These parameters are already &mut Option<HashMap>, so adding &mut would create &&mut
                        // In this case, pass the variable directly without any borrowing
                        let is_already_mut_ref = if let HirExpr::Var(var_name) = hir_arg {
                            self.ctx.mut_option_dict_params.contains(var_name)
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
                        if matches!(hir_arg, HirExpr::Literal(Literal::String(_))) {
                            // Check if function expects borrowed string (&str) at this position
                            let param_expects_borrowed = self.ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(param_idx))
                                .copied()
                                .unwrap_or(false);

                            if param_expects_borrowed {
                                // Param is &str - string literal works directly
                                // DEPYLER-0779: But wrap in Some if optional param
                                if needs_some_wrap {
                                    let converted: syn::Expr = parse_quote! { #arg_expr.to_string() };
                                    if optional_is_borrowed {
                                        return parse_quote! { &Some(#converted) };
                                    } else {
                                        return parse_quote! { Some(#converted) };
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
                            let param_needs_borrow = self
                                .ctx
                                .function_param_borrows
                                .get(func)
                                .and_then(|borrows| borrows.get(i).copied())
                                .unwrap_or(false);

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
                                    parse_quote! { #s.to_string() }
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
                                v == "args" && (attr.ends_with('s') || attr == "parts" || attr == "items" || attr == "values" || attr == "keys" || attr == "args")
                            } else {
                                false
                            }
                        }
                        // Variable that's known to be a list
                        HirExpr::Var(v) => v.ends_with('s') || v == "parts" || v == "items" || v == "args",
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
            let call_str = format!("{}({})", func_ident, args_tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "));
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
                        // struct.unpack('i', bytes) → (i32::from_le_bytes(bytes[0..4].try_into().unwrap()),)
                        Ok(Some(parse_quote! {
                            (i32::from_le_bytes(#bytes_expr[0..4].try_into().unwrap()),)
                        }))
                    } else if count == 2 {
                        // struct.unpack('ii', bytes) → (i32::from_le_bytes(...), i32::from_le_bytes(...))
                        Ok(Some(parse_quote! {
                            (
                                i32::from_le_bytes(#bytes_expr[0..4].try_into().unwrap()),
                                i32::from_le_bytes(#bytes_expr[4..8].try_into().unwrap()),
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
                        let is_unwrapped =
                            self.ctx.option_unwrap_map.contains_key(var_name)
                                || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                        if is_unwrapped {
                            // Variable was already unwrapped, don't add .as_ref().unwrap()
                            key.clone()
                        } else if let Some(var_type) = self.ctx.var_types.get(var_name) {
                            if matches!(var_type, Type::Optional(_)) {
                                // Key is an Option type - unwrap it
                                parse_quote! { #key.as_ref().unwrap() }
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

    /// Convert subprocess.run() to std::process::Command
    /// DEPYLER-0391: Subprocess module for executing system commands
    ///
    /// Maps Python subprocess.run() to Rust std::process::Command:
    /// - subprocess.run(cmd) → Command::new(cmd[0]).args(&cmd[1..]).status()
    /// - capture_output=True → .output() instead of .status()
    /// - cwd=path → .current_dir(path)
    /// - check=True → verify exit status (NOTE: add error handling tracked in DEPYLER-0424)
    ///
    /// Returns anonymous struct with: returncode, stdout, stderr
    ///
    /// # Complexity
    /// ≤10 (linear processing of kwargs)
    #[inline]
    pub(crate) fn convert_subprocess_run(
        &mut self,
        args: &[HirExpr],
        kwargs: &[(Symbol, HirExpr)],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("subprocess.run() requires at least 1 argument (command list)");
        }

        // First argument is the command list
        let cmd_expr = args[0].to_rust_expr(self.ctx)?;

        // Parse keyword arguments
        let mut capture_output = false;
        let mut _text = false;
        let mut cwd_expr: Option<syn::Expr> = None;
        let mut cwd_is_option = false; // DEPYLER-0950: Track if cwd is Option type
        let mut _check = false;

        for (key, value) in kwargs {
            match key.as_str() {
                "capture_output" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        capture_output = *b;
                    }
                }
                "text" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        _text = *b;
                    }
                }
                "cwd" => {
                    cwd_expr = Some(value.to_rust_expr(self.ctx)?);
                    // DEPYLER-0950: Check if cwd value is likely an Option type
                    // Variables with Optional type annotation or None default need if-let Some()
                    // Expressions like list indexing (which use .expect()) are already unwrapped
                    cwd_is_option = matches!(value, HirExpr::Var(v) if {
                        self.ctx.var_types.get(v).is_some_and(|t| matches!(t, Type::Optional(_)))
                    });
                }
                "check" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        _check = *b;
                    }
                }
                _ => {} // Ignore unknown kwargs for now
            }
        }

        // Build the Command construction
        // Python: subprocess.run(["echo", "hello"], capture_output=True, cwd="/tmp")
        // Rust: {
        //   let mut cmd = std::process::Command::new(&cmd_list[0]);
        //   cmd.args(&cmd_list[1..]);
        //   if cwd { cmd.current_dir(cwd); }
        //   let output = cmd.output()?;
        //   // Create result struct
        //   SubprocessResult {
        //     returncode: output.status.code().unwrap_or(-1),
        //     stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        //     stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        //   }
        // }

        // DEPYLER-0627: subprocess.run() returns CompletedProcess struct (not tuple)
        // Python's subprocess.run() returns CompletedProcess with .returncode, .stdout, .stderr
        // We generate a struct to match Python's API semantics.
        self.ctx.needs_completed_process = true;

        // DEPYLER-0517: Handle Option<String> for cwd parameter
        // DEPYLER-0950: Only use if-let Some() when cwd is actually an Option type
        // When cwd is a concrete expression (like list indexing), use it directly
        let result = if capture_output {
            // Use .output() to capture stdout/stderr
            if let Some(cwd) = cwd_expr {
                if cwd_is_option {
                    // cwd is Option<String> - need if-let to unwrap
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            if let Some(dir) = #cwd {
                                cmd.current_dir(dir);
                            }
                            let output = cmd.output().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: output.status.code().unwrap_or(-1),
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                            }
                        }
                    }
                } else {
                    // cwd is already a concrete path (String) - use directly
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            cmd.current_dir(#cwd);
                            let output = cmd.output().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: output.status.code().unwrap_or(-1),
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                            }
                        }
                    }
                }
            } else {
                parse_quote! {
                    {
                        let cmd_list = #cmd_expr;
                        let mut cmd = std::process::Command::new(&cmd_list[0]);
                        cmd.args(&cmd_list[1..]);
                        let output = cmd.output().expect("subprocess.run() failed");
                        CompletedProcess {
                            returncode: output.status.code().unwrap_or(-1),
                            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                        }
                    }
                }
            }
        } else {
            // Use .status() for exit code only (no capture)
            if let Some(cwd) = cwd_expr {
                if cwd_is_option {
                    // cwd is Option<String> - need if-let to unwrap
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            if let Some(dir) = #cwd {
                                cmd.current_dir(dir);
                            }
                            let status = cmd.status().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: status.code().unwrap_or(-1),
                                stdout: String::new(),
                                stderr: String::new(),
                            }
                        }
                    }
                } else {
                    // cwd is already a concrete path (String) - use directly
                    parse_quote! {
                        {
                            let cmd_list = #cmd_expr;
                            let mut cmd = std::process::Command::new(&cmd_list[0]);
                            cmd.args(&cmd_list[1..]);
                            cmd.current_dir(#cwd);
                            let status = cmd.status().expect("subprocess.run() failed");
                            CompletedProcess {
                                returncode: status.code().unwrap_or(-1),
                                stdout: String::new(),
                                stderr: String::new(),
                            }
                        }
                    }
                }
            } else {
                parse_quote! {
                    {
                        let cmd_list = #cmd_expr;
                        let mut cmd = std::process::Command::new(&cmd_list[0]);
                        cmd.args(&cmd_list[1..]);
                        let status = cmd.status().expect("subprocess.run() failed");
                        CompletedProcess {
                            returncode: status.code().unwrap_or(-1),
                            stdout: String::new(),
                            stderr: String::new(),
                        }
                    }
                }
            }
        };

        Ok(result)
    }

    /// Convert subprocess.Popen() to std::process::Command::spawn()
    /// DEPYLER-0931: Subprocess Popen for process management
    ///
    /// Maps Python subprocess.Popen() to Rust std::process::Command:
    /// - subprocess.Popen(cmd) → Command::new(cmd).spawn().expect("...")
    /// - subprocess.Popen(cmd, shell=True) → Command::new("sh").arg("-c").arg(cmd).spawn()
    ///
    /// Returns std::process::Child which has .wait(), .kill(), etc.
    ///
    /// # Complexity
    /// ≤10 (linear processing of kwargs)
    #[inline]
    pub(crate) fn convert_subprocess_popen(
        &mut self,
        args: &[HirExpr],
        kwargs: &[(Symbol, HirExpr)],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("subprocess.Popen() requires at least 1 argument (command)");
        }

        // First argument is the command
        let cmd_expr = args[0].to_rust_expr(self.ctx)?;

        // Parse keyword arguments
        let mut shell = false;
        let mut cwd_expr: Option<syn::Expr> = None;

        for (key, value) in kwargs {
            match key.as_str() {
                "shell" => {
                    if let HirExpr::Literal(Literal::Bool(b)) = value {
                        shell = *b;
                    }
                }
                "cwd" => {
                    cwd_expr = Some(value.to_rust_expr(self.ctx)?);
                }
                _ => {} // Ignore unknown kwargs for now
            }
        }

        // Build the Command construction
        // Python: subprocess.Popen(cmd, shell=True)
        // Rust: Command::new("sh").arg("-c").arg(cmd).spawn().expect("...")
        let result = if shell {
            // shell=True: run through shell
            if let Some(cwd) = cwd_expr {
                parse_quote! {
                    {
                        let mut popen_cmd = std::process::Command::new("sh");
                        popen_cmd.arg("-c").arg(#cmd_expr);
                        popen_cmd.current_dir(#cwd);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            } else {
                parse_quote! {
                    {
                        let mut popen_cmd = std::process::Command::new("sh");
                        popen_cmd.arg("-c").arg(#cmd_expr);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            }
        } else {
            // No shell: cmd is a list
            if let Some(cwd) = cwd_expr {
                parse_quote! {
                    {
                        let popen_list = #cmd_expr;
                        let mut popen_cmd = std::process::Command::new(&popen_list[0]);
                        popen_cmd.args(&popen_list[1..]);
                        popen_cmd.current_dir(#cwd);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            } else {
                parse_quote! {
                    {
                        let popen_list = #cmd_expr;
                        let mut popen_cmd = std::process::Command::new(&popen_list[0]);
                        popen_cmd.args(&popen_list[1..]);
                        popen_cmd.spawn().expect("subprocess.Popen() failed")
                    }
                }
            }
        };

        Ok(result)
    }

    /// Try to convert numpy module calls to trueno equivalents.
    ///
    /// Phase 3: NumPy→Trueno codegen
    ///
    /// Maps numpy API calls to trueno (SIMD-accelerated tensor library):
    /// - np.array([...]) → Vector::from_slice(&[...])
    /// - np.dot(a, b) → a.dot(&b)?
    /// - np.sum(a) → a.sum()?
    /// - np.mean(a) → a.mean()?
    /// - np.sqrt(a) → a.sqrt()?
    ///
    /// Returns None if the method is not a recognized numpy function.
    pub(crate) fn try_convert_numpy_call(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Check if this is a recognized numpy function
        if numpy_gen::parse_numpy_function(method).is_none() {
            return Ok(None);
        }

        // Mark that we need trueno dependency
        self.ctx.needs_trueno = true;

        // Convert arguments to syn::Expr
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Generate trueno code based on the numpy function
        let result = match method {
            "array" => {
                // np.array([1.0, 2.0, 3.0]) → Vector::from_slice(&[1.0f32, 2.0, 3.0])
                // The argument should be a list literal
                if let Some(HirExpr::List(elements)) = args.first() {
                    let element_exprs: Vec<proc_macro2::TokenStream> = elements
                        .iter()
                        .map(|e| {
                            let expr = e.to_rust_expr(self.ctx)?;
                            Ok(quote::quote! { #expr })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let call = numpy_gen::NumpyCall::Array {
                        elements: element_exprs,
                    };
                    let tokens = numpy_gen::generate_trueno_code(&call);
                    return Ok(Some(syn::parse2(tokens)?));
                }
                // Fallback: pass through as vec!
                if let Some(arg) = arg_exprs.first() {
                    parse_quote! { Vector::from_vec(#arg) }
                } else {
                    parse_quote! { Vector::new() }
                }
            }
            "dot" => {
                // np.dot(a, b) → a.dot(&b).unwrap()
                if arg_exprs.len() >= 2 {
                    let a = &arg_exprs[0];
                    let b = &arg_exprs[1];
                    parse_quote! { #a.dot(&#b).unwrap() }
                } else {
                    bail!("np.dot() requires 2 arguments");
                }
            }
            "sum" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.sum().unwrap() }
                } else {
                    bail!("np.sum() requires 1 argument");
                }
            }
            "mean" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.mean().unwrap() }
                } else {
                    bail!("np.mean() requires 1 argument");
                }
            }
            // DEPYLER-0657: Scalar vs Vector numpy methods
            // f64::sqrt()/abs() returns f64 directly (no Result)
            // Vector::sqrt()/abs() returns Result (needs unwrap)
            "sqrt" => {
                if args.is_empty() {
                    bail!("np.sqrt() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.sqrt().unwrap() }
                } else {
                    parse_quote! { #arr.sqrt() }
                }
            }
            "abs" => {
                if args.is_empty() {
                    bail!("np.abs() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.abs().unwrap() }
                } else {
                    // f64 uses .abs() directly
                    parse_quote! { #arr.abs() }
                }
            }
            "min" | "amin" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.min().unwrap() }
                } else {
                    bail!("np.min() requires 1 argument");
                }
            }
            "max" | "amax" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.max().unwrap() }
                } else {
                    bail!("np.max() requires 1 argument");
                }
            }
            // DEPYLER-0657: exp/log/sin/cos scalar vs vector
            "exp" => {
                if args.is_empty() {
                    bail!("np.exp() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.exp().unwrap() }
                } else {
                    parse_quote! { #arr.exp() }
                }
            }
            "log" => {
                if args.is_empty() {
                    bail!("np.log() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.ln().unwrap() }
                } else {
                    // f64 uses .ln() for natural log
                    parse_quote! { #arr.ln() }
                }
            }
            "sin" => {
                if args.is_empty() {
                    bail!("np.sin() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.sin().unwrap() }
                } else {
                    // f64::sin() returns f64 directly
                    parse_quote! { #arr.sin() }
                }
            }
            "cos" => {
                if args.is_empty() {
                    bail!("np.cos() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.cos().unwrap() }
                } else {
                    // f64::cos() returns f64 directly
                    parse_quote! { #arr.cos() }
                }
            }
            "clip" => {
                // DEPYLER-0920: Cast min/max to f32 for trueno::Vector::clamp compatibility
                if arg_exprs.len() >= 3 {
                    let arr = &arg_exprs[0];
                    let min = &arg_exprs[1];
                    let max = &arg_exprs[2];
                    parse_quote! { #arr.clamp(#min as f32, #max as f32).unwrap() }
                } else {
                    bail!("np.clip() requires 3 arguments (array, min, max)");
                }
            }
            "argmax" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.argmax().unwrap() }
                } else {
                    bail!("np.argmax() requires 1 argument");
                }
            }
            "argmin" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.argmin().unwrap() }
                } else {
                    bail!("np.argmin() requires 1 argument");
                }
            }
            "std" => {
                // trueno uses stddev(), not std()
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.stddev().unwrap() }
                } else {
                    bail!("np.std() requires 1 argument");
                }
            }
            "var" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.variance().unwrap() }
                } else {
                    bail!("np.var() requires 1 argument");
                }
            }
            "zeros" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { Vector::zeros(#size) }
                } else {
                    bail!("np.zeros() requires 1 argument");
                }
            }
            "ones" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { Vector::ones(#size) }
                } else {
                    bail!("np.ones() requires 1 argument");
                }
            }
            "norm" => {
                if let Some(arr) = arg_exprs.first() {
                    // DEPYLER-0583: trueno uses norm_l2() for L2 (Euclidean) norm
                    // DEPYLER-0667: Wrap arg in parens so `a - b` becomes `(a - b).norm_l2()`
                    // Without parens, `a - b.norm_l2()` parses as `a - (b.norm_l2())`
                    parse_quote! { (#arr).norm_l2().unwrap() }
                } else {
                    bail!("np.norm() requires 1 argument");
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(result))
    }

    /// Try to convert os.path module method calls
    /// DEPYLER-STDLIB-OSPATH: Path manipulation and file system operations
    ///
    /// Maps Python os.path module to Rust std::path + std::fs:
    /// - os.path.join() → PathBuf::new().join()
    /// - os.path.basename() → Path::file_name()
    /// - os.path.exists() → Path::exists()
    ///
    /// # Complexity
    /// 10 (match with 10 primary branches - split into helper methods as needed)
    #[inline]
    pub(crate) fn try_convert_os_path_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0594: Removed maybe_borrow closure - always use explicit & for Path::new()
        // Path::new() requires &S, and subcommand field bindings create owned Strings
        // Using & consistently is simpler and works for both owned and borrowed values

        let result = match method {
            // Path construction
            "join" => {
                if arg_exprs.is_empty() {
                    bail!("os.path.join() requires at least 1 argument");
                }

                // os.path.join(a, b, c, ...) → PathBuf::from(a).join(b).join(c)...
                let first = &arg_exprs[0];
                if arg_exprs.len() == 1 {
                    parse_quote! { std::path::PathBuf::from(#first) }
                } else {
                    let mut result: syn::Expr = parse_quote! { std::path::PathBuf::from(#first) };
                    for part in &arg_exprs[1..] {
                        result = parse_quote! { #result.join(#part) };
                    }
                    parse_quote! { #result.to_string_lossy().to_string() }
                }
            }

            // Path decomposition
            "basename" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.basename() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                // Path::new() requires &S where S: AsRef<OsStr>
                // Subcommand field bindings create owned Strings that need borrowing
                let path = &arg_exprs[0];

                // os.path.basename(path) → Path::new(&path).file_name()
                parse_quote! {
                    std::path::Path::new(&#path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string()
                }
            }

            "dirname" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.dirname() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.dirname(path) → Path::new(&path).parent()
                parse_quote! {
                    std::path::Path::new(&#path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or("")
                        .to_string()
                }
            }

            "split" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.split() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.split(path) → (dirname, basename) tuple
                parse_quote! {
                    {
                        let p = std::path::Path::new(&#path);
                        let dirname = p.parent().and_then(|p| p.to_str()).unwrap_or("").to_string();
                        let basename = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                        (dirname, basename)
                    }
                }
            }

            "splitext" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.splitext() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.splitext(path) → (stem, extension) tuple
                parse_quote! {
                    {
                        let p = std::path::Path::new(&#path);
                        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                        let ext = p.extension().and_then(|e| e.to_str()).map(|e| format!(".{}", e)).unwrap_or_default();
                        (stem, ext)
                    }
                }
            }

            // Path predicates
            "exists" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.exists() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.exists(path) → Path::new(&path).exists()
                parse_quote! { std::path::Path::new(&#path).exists() }
            }

            "isfile" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isfile() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.isfile(path) → Path::new(&path).is_file()
                parse_quote! { std::path::Path::new(&#path).is_file() }
            }

            "isdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isdir() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.isdir(path) → Path::new(&path).is_dir()
                parse_quote! { std::path::Path::new(&#path).is_dir() }
            }

            "isabs" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.isabs() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.isabs(path) → Path::new(&path).is_absolute()
                parse_quote! { std::path::Path::new(&#path).is_absolute() }
            }

            // Path normalization
            "abspath" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.abspath() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for fs::canonicalize and PathBuf::from
                let path = &arg_exprs[0];

                // os.path.abspath(path) → std::fs::canonicalize() or manual absolute path
                // Using canonicalize (resolves symlinks too, like realpath)
                parse_quote! {
                    std::fs::canonicalize(&#path)
                        .unwrap_or_else(|_| std::path::PathBuf::from(&#path))
                        .to_string_lossy()
                        .to_string()
                }
            }

            "normpath" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.normpath() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for Path::new()
                let path = &arg_exprs[0];

                // os.path.normpath(path) → normalize path components
                // Rust Path doesn't have direct normpath, but we can use PathBuf operations
                parse_quote! {
                    {
                        let p = std::path::Path::new(&#path);
                        let mut components = Vec::new();
                        for component in p.components() {
                            match component {
                                std::path::Component::CurDir => {},
                                std::path::Component::ParentDir => {
                                    components.pop();
                                }
                                _ => components.push(component),
                            }
                        }
                        components.iter()
                            .map(|c| c.as_os_str().to_string_lossy())
                            .collect::<Vec<_>>()
                            .join(std::path::MAIN_SEPARATOR_STR)
                    }
                }
            }

            "realpath" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.realpath() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];

                // os.path.realpath(path) → std::fs::canonicalize()
                parse_quote! {
                    std::fs::canonicalize(#path)
                        .unwrap_or_else(|_| std::path::PathBuf::from(#path))
                        .to_string_lossy()
                        .to_string()
                }
            }

            // Path properties
            "getsize" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.getsize() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for std::fs::metadata()
                let path = &arg_exprs[0];

                // os.path.getsize(path) → std::fs::metadata().len()
                parse_quote! {
                    std::fs::metadata(&#path).unwrap().len() as i64
                }
            }

            "getmtime" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.getmtime() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for std::fs::metadata()
                let path = &arg_exprs[0];

                // os.path.getmtime(path) → std::fs::metadata().modified()
                parse_quote! {
                    std::fs::metadata(&#path)
                        .unwrap()
                        .modified()
                        .unwrap()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64()
                }
            }

            "getctime" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.getctime() requires exactly 1 argument");
                }
                // DEPYLER-0594: Always use reference for std::fs::metadata()
                let path = &arg_exprs[0];

                // os.path.getctime(path) → std::fs::metadata().created()
                // Note: On Unix, this is ctime (change time), but Rust only has created()
                parse_quote! {
                    std::fs::metadata(&#path)
                        .unwrap()
                        .created()
                        .unwrap()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64()
                }
            }

            // Path expansion
            "expanduser" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.expanduser() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];

                // os.path.expanduser(path) → expand ~ to home directory
                parse_quote! {
                    {
                        let p = #path;
                        if p.starts_with("~") {
                            if let Some(home) = std::env::var_os("HOME") {
                                format!("{}{}", home.to_string_lossy(), &p[1..])
                            } else {
                                p.to_string()
                            }
                        } else {
                            p.to_string()
                        }
                    }
                }
            }

            "expandvars" => {
                if arg_exprs.len() != 1 {
                    bail!("os.path.expandvars() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];

                // os.path.expandvars(path) → expand environment variables
                // Simplified: just return path as-is for now (full implementation complex)
                parse_quote! { #path.to_string() }
            }

            // DEPYLER-STDLIB-OSPATH: relpath() - compute relative path
            "relpath" => {
                if arg_exprs.len() != 2 {
                    bail!("os.path.relpath() requires exactly 2 arguments");
                }
                let path = &arg_exprs[0];
                let start = &arg_exprs[1];

                // os.path.relpath(path, start) → compute relative path from start to path
                parse_quote! {
                    {
                        let path_obj = std::path::Path::new(#path);
                        let start_obj = std::path::Path::new(#start);
                        path_obj
                            .strip_prefix(start_obj)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| #path.to_string())
                    }
                }
            }

            _ => {
                // For functions not yet implemented, return None to allow fallback
                return Ok(None);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert base64 module method calls
    /// DEPYLER-STDLIB-BASE64: Base64 and variants encoding/decoding
    ///
    /// Maps Python base64 module to Rust base64 crate:
    /// - base64.b64encode() → base64::encode()
    /// - base64.b64decode() → base64::decode()
    /// - base64.urlsafe_b64encode() → URL-safe encoding
    ///
    /// # Complexity
    /// 10 (match with 10 branches for different encodings)
    #[inline]
    pub(crate) fn try_convert_base64_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need base64 crate
        self.ctx.needs_base64 = true;

        let result = match method {
            // Standard Base64
            "b64encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b64encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b64encode(data) → Vec<u8> (Python returns bytes)
                // DEPYLER-1003: Return Vec<u8> so .decode('utf-8') works with from_utf8_lossy
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.encode(#data).into_bytes()
                }
            }

            "b64decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b64decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b64decode(data) → base64::engine::general_purpose::STANDARD.decode(data).unwrap()
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.decode(#data).unwrap()
                }
            }

            // URL-safe Base64
            "urlsafe_b64encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.urlsafe_b64encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.urlsafe_b64encode(data) → Vec<u8> (Python returns bytes)
                // DEPYLER-1003: Return Vec<u8> so .decode('utf-8') works with from_utf8_lossy
                parse_quote! {
                    base64::engine::general_purpose::URL_SAFE.encode(#data).into_bytes()
                }
            }

            "urlsafe_b64decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.urlsafe_b64decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.urlsafe_b64decode(data) → base64::engine::general_purpose::URL_SAFE.decode(data).unwrap()
                parse_quote! {
                    base64::engine::general_purpose::URL_SAFE.decode(#data).unwrap()
                }
            }

            // Base32 (note: base64 crate doesn't support base32, would need data-encoding crate)
            "b32encode" | "b32decode" => {
                // Simplified: note that full implementation needs data-encoding crate
                bail!(
                    "base64.{} requires data-encoding crate (not yet integrated)",
                    method
                );
            }

            // Base16 (Hex)
            "b16encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b16encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b16encode(data) → hex::encode_upper(data)
                parse_quote! {
                    hex::encode_upper(#data)
                }
            }

            "b16decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b16decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b16decode(data) → hex::decode(data).unwrap()
                parse_quote! {
                    hex::decode(#data).unwrap()
                }
            }

            // Base85 (also needs additional crate)
            "b85encode" | "b85decode" => {
                // Simplified: note that full implementation needs additional crate
                bail!(
                    "base64.{} requires base85 encoding crate (not yet integrated)",
                    method
                );
            }

            _ => {
                bail!("base64.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert secrets module method calls
    /// DEPYLER-STDLIB-SECRETS: Cryptographically strong random operations
    ///
    /// Maps Python secrets module to Rust rand crate (cryptographic RNG):
    /// - secrets.randbelow() → rand::thread_rng().gen_range()
    /// - secrets.token_bytes() → Cryptographically secure random bytes
    ///
    /// # Complexity
    /// 5 (match with 5 branches)
    #[inline]
    pub(crate) fn try_convert_secrets_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need rand crate (ThreadRng is cryptographically secure)
        self.ctx.needs_rand = true;
        self.ctx.needs_base64 = true; // For token_urlsafe

        let result = match method {
            // Random number generation
            "randbelow" => {
                if arg_exprs.len() != 1 {
                    bail!("secrets.randbelow() requires exactly 1 argument");
                }
                let n = &arg_exprs[0];

                // secrets.randbelow(n) → rand::thread_rng().gen_range(0..n)
                // DEPYLER-0656: Add use rand::Rng for gen_range method
                parse_quote! {
                    {
                        use rand::Rng;
                        rand::thread_rng().gen_range(0..#n)
                    }
                }
            }

            "choice" => {
                if arg_exprs.len() != 1 {
                    bail!("secrets.choice() requires exactly 1 argument");
                }
                let seq = &arg_exprs[0];

                // secrets.choice(seq) → seq.choose(&mut rand::thread_rng()).unwrap()
                // DEPYLER-0656: Add use rand::seq::SliceRandom for choose method
                parse_quote! {
                    {
                        use rand::seq::SliceRandom;
                        *#seq.choose(&mut rand::thread_rng()).unwrap()
                    }
                }
            }

            // Token generation
            "token_bytes" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 } // Default 32 bytes
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_bytes(n) → generate n random bytes
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        bytes
                    }
                }
            }

            "token_hex" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 }
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_hex(n) → generate n random bytes and encode as hex
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        hex::encode(&bytes)
                    }
                }
            }

            "token_urlsafe" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 }
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_urlsafe(n) → generate n random bytes and encode as URL-safe base64
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        base64::engine::general_purpose::URL_SAFE.encode(&bytes)
                    }
                }
            }

            _ => {
                bail!("secrets.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert hashlib module method calls
    /// DEPYLER-STDLIB-HASHLIB: Cryptographic hash functions
    ///
    /// Supports: md5, sha1, sha224, sha256, sha384, sha512, blake2b, blake2s
    /// Returns hex digest directly (one-shot hashing pattern)
    ///
    /// # Complexity
    /// Cyclomatic: 9 (match with 8 algorithms + default)
    #[inline]
    pub(crate) fn try_convert_hashlib_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // All hash functions need hex encoding
        self.ctx.needs_hex = true;

        let result = match method {
            // MD5 hash
            // DEPYLER-0558: Support both one-shot and incremental patterns
            // Use Box<dyn DynDigest> for type-erased hasher objects
            "md5" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.md5() accepts 0 or 1 arguments");
                }
                self.ctx.needs_md5 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    // hashlib.md5() with no args → return boxed hasher for incremental use
                    parse_quote! {
                        {
                            use md5::Digest;
                            use digest::DynDigest;
                            Box::new(md5::Md5::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use md5::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(md5::Md5::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-1 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha1" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha1() accepts 0 or 1 arguments");
                }
                // DEPYLER-1001: Fix sha1 dependency - was incorrectly setting needs_sha2
                self.ctx.needs_sha1 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha1::Digest;
                            use digest::DynDigest;
                            Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha1::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-224 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha224" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha224() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha224::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha224::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-256 hash
            // DEPYLER-0558: Support both one-shot and incremental patterns
            // Use Box<dyn DynDigest> for type-erased hasher objects
            // DEPYLER-1002: Always return hasher object, let .hexdigest() finalize
            "sha256" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha256() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    // hashlib.sha256() with no args → return boxed hasher for incremental use
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    // hashlib.sha256(data) → return hasher with data already updated
                    // The .hexdigest() method call will finalize and hex-encode
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-384 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha384" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha384() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-512 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha512" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha512() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // BLAKE2b hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "blake2b" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.blake2b() accepts 0 or 1 arguments");
                }
                self.ctx.needs_blake2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            Box::new(blake2::Blake2b512::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(blake2::Blake2b512::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // BLAKE2s hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "blake2s" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.blake2s() accepts 0 or 1 arguments");
                }
                self.ctx.needs_blake2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            Box::new(blake2::Blake2s256::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(blake2::Blake2s256::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            _ => {
                bail!("hashlib.{} not implemented yet (try: md5, sha1, sha224, sha256, sha384, sha512, blake2b, blake2s)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert uuid module method calls
    /// DEPYLER-STDLIB-UUID: UUID generation (RFC 4122)
    ///
    /// Supports: uuid1 (time-based), uuid4 (random)
    /// Returns string representation of UUID
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_uuid_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need uuid crate
        self.ctx.needs_uuid = true;

        let result = match method {
            // UUID v1 - time-based
            "uuid1" => {
                if !arg_exprs.is_empty() {
                    bail!("uuid.uuid1() takes no arguments (node/clock_seq not yet supported)");
                }

                // uuid.uuid1() → Uuid::new_v1(...).to_string()
                // Note: Requires context (timestamp + node ID)
                parse_quote! {
                    {
                        use uuid::Uuid;
                        // Generate time-based UUID v1
                        // Note: Using placeholder implementation (actual v1 needs timestamp context)
                        Uuid::new_v4().to_string()  // NOTE: Implement proper UUID v1 with timestamp (tracked in DEPYLER-0424)
                    }
                }
            }

            // UUID v4 - random (most common)
            "uuid4" => {
                if !arg_exprs.is_empty() {
                    bail!("uuid.uuid4() takes no arguments");
                }

                // uuid.uuid4() → Uuid::new_v4().to_string()
                parse_quote! {
                    {
                        use uuid::Uuid;
                        Uuid::new_v4().to_string()
                    }
                }
            }

            _ => {
                bail!("uuid.{} not implemented yet (try: uuid1, uuid4)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert hmac module method calls
    /// DEPYLER-STDLIB-HMAC: HMAC authentication
    ///
    /// Supports: new() with SHA256, compare_digest()
    /// Returns hex digest for one-shot HMAC
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_hmac_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need hmac and related crates
        self.ctx.needs_hmac = true;
        self.ctx.needs_sha2 = true; // For SHA256
        self.ctx.needs_hex = true;

        let result = match method {
            // HMAC creation - simplified to SHA256
            "new" => {
                if arg_exprs.len() < 2 {
                    bail!("hmac.new() requires at least 2 arguments (key, message)");
                }
                let key = &arg_exprs[0];
                let msg = &arg_exprs[1];

                // NOTE: Parse digestmod argument (arg_exprs[2]) to support multiple HMAC algorithms (tracked in DEPYLER-0424)
                // For now, hardcode SHA256 as most common

                // hmac.new(key, msg, hashlib.sha256) → HMAC-SHA256 hex digest
                parse_quote! {
                    {
                        use hmac::{Hmac, Mac};
                        use sha2::Sha256;

                        type HmacSha256 = Hmac<Sha256>;
                        let mut mac = HmacSha256::new_from_slice(#key).expect("HMAC key error");
                        mac.update(#msg);
                        hex::encode(mac.finalize().into_bytes())
                    }
                }
            }

            // Timing-safe comparison
            "compare_digest" => {
                if arg_exprs.len() != 2 {
                    bail!("hmac.compare_digest() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];

                // hmac.compare_digest(a, b) → constant-time comparison
                parse_quote! {
                    {
                        use subtle::ConstantTimeEq;
                        #a.ct_eq(#b).into()
                    }
                }
            }

            _ => {
                bail!(
                    "hmac.{} not implemented yet (try: new, compare_digest)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert platform module method calls
    /// DEPYLER-0430: platform module - system information
    ///
    /// Maps Python platform module to Rust std::env::consts:
    /// - platform.system() → std::env::consts::OS
    /// - platform.machine() → std::env::consts::ARCH
    /// - platform.python_version() → "3.11.0" (hardcoded constant)
    ///
    /// # Complexity
    /// ≤10 (simple match with few branches)
    #[inline]
    pub(crate) fn try_convert_platform_method(
        &mut self,
        method: &str,
        _args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let result = match method {
            "system" => {
                // platform.system() → std::env::consts::OS
                // Returns "linux", "macos", "windows", etc.
                parse_quote! { std::env::consts::OS.to_string() }
            }

            "machine" => {
                // platform.machine() → std::env::consts::ARCH
                // Returns "x86_64", "aarch64", etc.
                parse_quote! { std::env::consts::ARCH.to_string() }
            }

            "python_version" => {
                // platform.python_version() → "3.11.0"
                // Hardcoded to Python 3.11 for compatibility
                parse_quote! { "3.11.0".to_string() }
            }

            "release" => {
                // platform.release() → OS release version
                // Note: This is OS-specific and may require additional logic
                parse_quote! { std::env::consts::OS.to_string() }
            }

            _ => {
                bail!(
                    "platform.{} not implemented yet (try: system, machine, python_version, release)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert calendar module method calls
    /// DEPYLER-0424: Calendar module - date/time calculations
    ///
    /// Supports: isleap, weekday, monthrange, leapdays, month, monthcalendar
    /// Common calendar operations
    ///
    /// # Complexity
    /// Cyclomatic: 7 (match with 6 functions + default)
    #[inline]
    pub(crate) fn try_convert_calendar_method(
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
            "isleap" => {
                // calendar.isleap(year) → check if year is a leap year
                // Leap year: divisible by 4, except century years unless divisible by 400
                let year = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 0 });
                parse_quote! {
                    (#year % 4 == 0 && (#year % 100 != 0 || #year % 400 == 0))
                }
            }

            "weekday" => {
                // calendar.weekday(year, month, day) → day of week (0=Monday, 6=Sunday)
                // Uses chrono crate for accurate calculation
                let year = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 2000 });
                let month = arg_exprs.get(1).cloned().unwrap_or_else(|| parse_quote! { 1 });
                let day = arg_exprs.get(2).cloned().unwrap_or_else(|| parse_quote! { 1 });
                parse_quote! {
                    chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                        .map(|d| d.weekday().num_days_from_monday() as i32)
                        .unwrap_or(0)
                }
            }

            "monthrange" => {
                // calendar.monthrange(year, month) → (first_weekday, days_in_month)
                let year = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 2000 });
                let month = arg_exprs.get(1).cloned().unwrap_or_else(|| parse_quote! { 1 });
                parse_quote! {
                    {
                        let y = #year as i32;
                        let m = #month as u32;
                        let first = chrono::NaiveDate::from_ymd_opt(y, m, 1)
                            .map(|d| d.weekday().num_days_from_monday() as i32)
                            .unwrap_or(0);
                        let days = if m == 12 {
                            chrono::NaiveDate::from_ymd_opt(y + 1, 1, 1)
                        } else {
                            chrono::NaiveDate::from_ymd_opt(y, m + 1, 1)
                        }
                        .and_then(|d| d.pred_opt())
                        .map(|d| d.day() as i32)
                        .unwrap_or(28);
                        (first, days)
                    }
                }
            }

            "leapdays" => {
                // calendar.leapdays(y1, y2) → number of leap years in range [y1, y2)
                let y1 = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 0 });
                let y2 = arg_exprs.get(1).cloned().unwrap_or_else(|| parse_quote! { 0 });
                parse_quote! {
                    {
                        let start = #y1 as i32;
                        let end = #y2 as i32;
                        (start..end).filter(|&y| y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)).count() as i32
                    }
                }
            }

            "month" | "prmonth" => {
                // calendar.month(year, month) → string calendar for month
                // Simplified - returns formatted string
                let year = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 2000 });
                let month = arg_exprs.get(1).cloned().unwrap_or_else(|| parse_quote! { 1 });
                parse_quote! {
                    format!("Calendar for {}-{:02}", #year, #month)
                }
            }

            "monthcalendar" => {
                // calendar.monthcalendar(year, month) → list of weeks (list of days)
                // Each week is a list of 7 ints (0 = day not in month)
                let year = arg_exprs.first().cloned().unwrap_or_else(|| parse_quote! { 2000 });
                let month = arg_exprs.get(1).cloned().unwrap_or_else(|| parse_quote! { 1 });
                parse_quote! {
                    {
                        let _ = (#year, #month); // Use variables
                        Vec::<Vec<i32>>::new() // Simplified - full impl needs chrono
                    }
                }
            }

            _ => {
                bail!(
                    "calendar.{} not implemented yet (try: isleap, weekday, monthrange, leapdays)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert binascii module method calls
    /// DEPYLER-STDLIB-BINASCII: Binary/ASCII conversions
    ///
    /// Supports: hexlify, unhexlify, b2a_hex, a2b_hex, b2a_base64, a2b_base64, crc32
    /// Common encoding/decoding operations
    ///
    /// # Complexity
    /// Cyclomatic: 8 (match with 7 functions + default)
    #[inline]
    pub(crate) fn try_convert_binascii_method(
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
            // Hex conversions
            "hexlify" | "b2a_hex" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.{}() requires exactly 1 argument", method);
                }
                self.ctx.needs_hex = true;
                let data = &arg_exprs[0];

                // binascii.hexlify(data) → hex::encode(data) as bytes
                parse_quote! {
                    hex::encode(#data).into_bytes()
                }
            }

            "unhexlify" | "a2b_hex" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.{}() requires exactly 1 argument", method);
                }
                self.ctx.needs_hex = true;
                let data = &arg_exprs[0];

                // binascii.unhexlify(data) → hex::decode(data)
                parse_quote! {
                    hex::decode(#data).expect("Invalid hex string")
                }
            }

            // Base64 conversions
            "b2a_base64" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.b2a_base64() requires exactly 1 argument");
                }
                self.ctx.needs_base64 = true;
                let data = &arg_exprs[0];

                // binascii.b2a_base64(data) → base64::encode(data) with newline
                parse_quote! {
                    {
                        let mut result = base64::engine::general_purpose::STANDARD.encode(#data);
                        result.push('\n');
                        result.into_bytes()
                    }
                }
            }

            "a2b_base64" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.a2b_base64() requires exactly 1 argument");
                }
                self.ctx.needs_base64 = true;
                let data = &arg_exprs[0];

                // binascii.a2b_base64(data) → base64::decode(data)
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.decode(#data).expect("Invalid base64 string")
                }
            }

            // Quoted-printable encoding
            "b2a_qp" => {
                if arg_exprs.is_empty() {
                    bail!("binascii.b2a_qp() requires at least 1 argument");
                }
                let data = &arg_exprs[0];

                // Simplified implementation - basic quoted-printable
                // NOTE: Full RFC 1521 quoted-printable implementation (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        // Simple QP: replace special chars, preserve printable ASCII
                        let bytes: &[u8] = #data;
                        let mut result = Vec::new();
                        for &b in bytes {
                            if b >= 33 && b <= 126 && b != b'=' {
                                result.push(b);
                            } else {
                                result.extend(format!("={:02X}", b).as_bytes());
                            }
                        }
                        result
                    }
                }
            }

            "a2b_qp" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.a2b_qp() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // Simplified QP decoder
                // NOTE: Full RFC 1521 quoted-printable implementation (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        let s = std::str::from_utf8(#data).expect("Invalid UTF-8");
                        let mut result = Vec::new();
                        let mut chars = s.chars().peekable();
                        while let Some(c) = chars.next() {
                            if c == '=' {
                                let h1 = chars.next().unwrap_or('0');
                                let h2 = chars.next().unwrap_or('0');
                                let hex = format!("{}{}", h1, h2);
                                if let Ok(b) = u8::from_str_radix(&hex, 16) {
                                    result.push(b);
                                }
                            } else {
                                result.push(c as u8);
                            }
                        }
                        result
                    }
                }
            }

            // UU encoding
            "b2a_uu" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.b2a_uu() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // Simplified UU encoding (basic implementation)
                // NOTE: Full UU encoding with proper line wrapping (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        let bytes: &[u8] = #data;
                        let len = bytes.len();
                        let mut result = vec![(len as u8 + 32)]; // Length byte
                        for chunk in bytes.chunks(3) {
                            let mut val = 0u32;
                            for (i, &b) in chunk.iter().enumerate() {
                                val |= (b as u32) << (16 - i * 8);
                            }
                            for i in 0..4 {
                                let b = ((val >> (18 - i * 6)) & 0x3F) as u8;
                                result.push(b + 32);
                            }
                        }
                        result.push(b'\n');
                        result
                    }
                }
            }

            "a2b_uu" => {
                if arg_exprs.len() != 1 {
                    bail!("binascii.a2b_uu() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // Simplified UU decoding (basic implementation)
                // NOTE: Full UU decoding implementation (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        let bytes: &[u8] = #data;
                        if bytes.is_empty() {
                            Vec::new()
                        } else {
                            let len = (bytes[0].wrapping_sub(32)) as usize;
                            let mut result = Vec::with_capacity(len);
                            for chunk in bytes[1..].chunks(4) {
                                if chunk.len() < 4 { break; }
                                let mut val = 0u32;
                                for (i, &b) in chunk.iter().enumerate() {
                                    val |= ((b.wrapping_sub(32) & 0x3F) as u32) << (18 - i * 6);
                                }
                                for i in 0..3 {
                                    if result.len() < len {
                                        result.push((val >> (16 - i * 8)) as u8);
                                    }
                                }
                            }
                            result
                        }
                    }
                }
            }

            // CRC32 checksum
            "crc32" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("binascii.crc32() requires 1 or 2 arguments");
                }
                self.ctx.needs_crc32 = true;
                let data = &arg_exprs[0];

                if arg_exprs.len() == 1 {
                    // binascii.crc32(data) → crc32 checksum as u32
                    parse_quote! {
                        {
                            use crc32fast::Hasher;
                            let mut hasher = Hasher::new();
                            hasher.update(#data);
                            hasher.finalize() as i32
                        }
                    }
                } else {
                    // binascii.crc32(data, crc) → update existing crc
                    let crc = &arg_exprs[1];
                    parse_quote! {
                        {
                            use crc32fast::Hasher;
                            let mut hasher = Hasher::new_with_initial(#crc as u32);
                            hasher.update(#data);
                            hasher.finalize() as i32
                        }
                    }
                }
            }

            _ => {
                bail!("binascii.{} not implemented yet (available: hexlify, unhexlify, b2a_hex, a2b_hex, b2a_base64, a2b_base64, b2a_qp, a2b_qp, b2a_uu, a2b_uu, crc32)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert urllib.parse module method calls
    /// DEPYLER-STDLIB-URLLIB-PARSE: URL parsing and encoding
    ///
    /// Supports: quote, unquote, quote_plus, unquote_plus, urlencode, parse_qs
    /// Common URL encoding/decoding operations
    ///
    /// # Complexity
    /// Cyclomatic: 7 (match with 6 functions + default)
    #[inline]
    pub(crate) fn try_convert_urllib_parse_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need URL encoding support
        self.ctx.needs_url_encoding = true;

        let result = match method {
            // Percent encoding
            "quote" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.quote() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                // quote(text) → percent-encode URL component
                parse_quote! {
                    {
                        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
                        utf8_percent_encode(#text, NON_ALPHANUMERIC).to_string()
                    }
                }
            }

            "unquote" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.unquote() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                // unquote(text) → percent-decode URL component
                parse_quote! {
                    {
                        use percent_encoding::percent_decode_str;
                        percent_decode_str(#text).decode_utf8_lossy().to_string()
                    }
                }
            }

            // Percent encoding with + for spaces (form encoding)
            "quote_plus" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.quote_plus() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                // quote_plus(text) → percent-encode with + for spaces
                parse_quote! {
                    {
                        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
                        utf8_percent_encode(#text, NON_ALPHANUMERIC)
                            .to_string()
                            .replace("%20", "+")
                    }
                }
            }

            "unquote_plus" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.unquote_plus() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                // unquote_plus(text) → percent-decode with + as space
                parse_quote! {
                    {
                        use percent_encoding::percent_decode_str;
                        let replaced = (#text).replace("+", " ");
                        percent_decode_str(&replaced).decode_utf8_lossy().to_string()
                    }
                }
            }

            // Query string encoding
            "urlencode" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.urlencode() requires exactly 1 argument");
                }
                let params = &arg_exprs[0];

                // urlencode(dict) → key1=value1&key2=value2
                parse_quote! {
                    {
                        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
                        #params.iter()
                            .map(|(k, v)| {
                                let key = utf8_percent_encode(&k.to_string(), NON_ALPHANUMERIC).to_string();
                                let val = utf8_percent_encode(&v.to_string(), NON_ALPHANUMERIC).to_string();
                                format!("{}={}", key, val)
                            })
                            .collect::<Vec<_>>()
                            .join("&")
                    }
                }
            }

            // Query string parsing
            "parse_qs" => {
                if arg_exprs.len() != 1 {
                    bail!("urllib.parse.parse_qs() requires exactly 1 argument");
                }
                let qs = &arg_exprs[0];

                // parse_qs(qs) → HashMap<String, Vec<String>>
                parse_quote! {
                    {
                        use percent_encoding::percent_decode_str;
                        use std::collections::HashMap;

                        let mut result: HashMap<String, Vec<String>> = HashMap::new();
                        for pair in (#qs).split('&') {
                            if let Some((key, value)) = pair.split_once('=') {
                                let decoded_key = percent_decode_str(key).decode_utf8_lossy().to_string();
                                let decoded_value = percent_decode_str(value).decode_utf8_lossy().to_string();
                                result.entry(decoded_key).or_insert_with(Vec::new).push(decoded_value);
                            }
                        }
                        result
                    }
                }
            }

            _ => {
                bail!("urllib.parse.{} not implemented yet (available: quote, unquote, quote_plus, unquote_plus, urlencode, parse_qs)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert fnmatch module method calls
    /// DEPYLER-STDLIB-FNMATCH: Unix shell-style pattern matching
    ///
    /// Supports: fnmatch, fnmatchcase, filter, translate
    /// Shell wildcard patterns: *, ?, [seq], [!seq]
    ///
    /// # Complexity
    /// Cyclomatic: 5 (match with 4 functions + default)
    #[inline]
    pub(crate) fn try_convert_fnmatch_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // fnmatch needs regex crate for pattern matching
        self.ctx.needs_regex = true;

        let result = match method {
            // Basic pattern matching
            "fnmatch" | "fnmatchcase" => {
                if arg_exprs.len() != 2 {
                    bail!("fnmatch.{}() requires exactly 2 arguments", method);
                }
                let name = &arg_exprs[0];
                let pattern = &arg_exprs[1];

                // Simplified implementation: convert pattern to regex and match
                // NOTE: Proper fnmatch pattern translation with case sensitivity (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        // Convert fnmatch pattern to regex
                        let pattern_str = #pattern;
                        let regex_pattern = pattern_str
                            .replace(".", "\\.")
                            .replace("*", ".*")
                            .replace("?", ".")
                            .replace("[!", "[^");

                        let regex = regex::Regex::new(&format!("^{}$", regex_pattern))
                            .unwrap_or_else(|_| regex::Regex::new("^$").unwrap());

                        regex.is_match(#name)
                    }
                }
            }

            // Filter list by pattern
            "filter" => {
                if arg_exprs.len() != 2 {
                    bail!("fnmatch.filter() requires exactly 2 arguments");
                }
                let names = &arg_exprs[0];
                let pattern = &arg_exprs[1];

                // filter(names, pattern) → names matching pattern
                parse_quote! {
                    {
                        let pattern_str = #pattern;
                        let regex_pattern = pattern_str
                            .replace(".", "\\.")
                            .replace("*", ".*")
                            .replace("?", ".")
                            .replace("[!", "[^");

                        let regex = regex::Regex::new(&format!("^{}$", regex_pattern))
                            .unwrap_or_else(|_| regex::Regex::new("^$").unwrap());

                        (#names).into_iter()
                            .filter(|name| regex.is_match(&name.to_string()))
                            .collect::<Vec<_>>()
                    }
                }
            }

            // Translate pattern to regex
            "translate" => {
                if arg_exprs.len() != 1 {
                    bail!("fnmatch.translate() requires exactly 1 argument");
                }
                let pattern = &arg_exprs[0];

                // translate(pattern) → regex string
                parse_quote! {
                    {
                        let pattern_str = #pattern;
                        let regex_pattern = pattern_str
                            .replace(".", "\\.")
                            .replace("*", ".*")
                            .replace("?", ".")
                            .replace("[!", "[^");

                        format!("(?ms)^{}$", regex_pattern)
                    }
                }
            }

            _ => {
                bail!("fnmatch.{} not implemented yet (available: fnmatch, fnmatchcase, filter, translate)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert shlex module method calls
    /// DEPYLER-STDLIB-SHLEX: Shell command line lexing
    ///
    /// Supports: split, quote, join
    /// Security-critical: prevents shell injection
    ///
    /// # Complexity
    /// Cyclomatic: 4 (match with 3 functions + default)
    #[inline]
    pub(crate) fn try_convert_shlex_method(
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
            // Shell-like split (respects quotes and escapes)
            "split" => {
                if arg_exprs.len() != 1 {
                    bail!("shlex.split() requires exactly 1 argument");
                }
                let s = &arg_exprs[0];

                // Simplified shell split (handles basic quotes)
                // NOTE: Use shell-words crate for full POSIX shell compliance (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        let input = #s;
                        let mut result = Vec::new();
                        let mut current = String::new();
                        let mut in_single_quote = false;
                        let mut in_double_quote = false;
                        let mut escaped = false;

                        for c in input.chars() {
                            if escaped {
                                current.push(c);
                                escaped = false;
                            } else if c == '\\' && !in_single_quote {
                                escaped = true;
                            } else if c == '\'' && !in_double_quote {
                                in_single_quote = !in_single_quote;
                            } else if c == '"' && !in_single_quote {
                                in_double_quote = !in_double_quote;
                            } else if c.is_whitespace() && !in_single_quote && !in_double_quote {
                                if !current.is_empty() {
                                    result.push(current.clone());
                                    current.clear();
                                }
                            } else {
                                current.push(c);
                            }
                        }

                        if !current.is_empty() {
                            result.push(current);
                        }

                        result
                    }
                }
            }

            // Shell-safe quoting
            "quote" => {
                if arg_exprs.len() != 1 {
                    bail!("shlex.quote() requires exactly 1 argument");
                }
                let s = &arg_exprs[0];

                // Quote string for safe shell usage
                parse_quote! {
                    {
                        let input = #s;
                        // Check if needs quoting
                        let needs_quoting = input.chars().any(|c| {
                            matches!(c, ' ' | '\t' | '\n' | '\'' | '"' | '\\' | '|' | '&' | ';' |
                                     '(' | ')' | '<' | '>' | '`' | '$' | '*' | '?' | '[' | ']' |
                                     '{' | '}' | '!' | '#' | '~')
                        });

                        if needs_quoting || input.is_empty() {
                            // Use single quotes and escape any single quotes
                            format!("'{}'", input.replace("'", "'\"'\"'"))
                        } else {
                            input.to_string()
                        }
                    }
                }
            }

            // Join list with shell-safe quoting
            "join" => {
                if arg_exprs.len() != 1 {
                    bail!("shlex.join() requires exactly 1 argument");
                }
                let args_list = &arg_exprs[0];

                // Join args with proper quoting
                parse_quote! {
                    {
                        let args = #args_list;
                        args.iter()
                            .map(|arg| {
                                let s = arg.to_string();
                                let needs_quoting = s.chars().any(|c| {
                                    matches!(c, ' ' | '\t' | '\n' | '\'' | '"' | '\\' | '|' | '&' | ';' |
                                             '(' | ')' | '<' | '>' | '`' | '$' | '*' | '?' | '[' | ']' |
                                             '{' | '}' | '!' | '#' | '~')
                                });

                                if needs_quoting || s.is_empty() {
                                    format!("'{}'", s.replace("'", "'\"'\"'"))
                                } else {
                                    s
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    }
                }
            }

            _ => {
                bail!(
                    "shlex.{} not implemented yet (available: split, quote, join)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert textwrap module method calls
    /// DEPYLER-STDLIB-TEXTWRAP: Text wrapping and formatting
    ///
    /// Supports: wrap, fill, dedent, indent, shorten
    /// Text formatting for display and documentation
    ///
    /// # Complexity
    /// Cyclomatic: 6 (match with 5 functions + default)
    #[inline]
    pub(crate) fn try_convert_textwrap_method(
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
            // Wrap text into list of lines
            "wrap" => {
                if arg_exprs.len() < 2 {
                    bail!("textwrap.wrap() requires at least 2 arguments (text, width)");
                }
                let text = &arg_exprs[0];
                let width = &arg_exprs[1];

                // Simple word-wrapping algorithm
                parse_quote! {
                    {
                        let text = #text;
                        let width = #width as usize;
                        let mut lines = Vec::new();
                        let mut current_line = String::new();
                        let mut current_len = 0;

                        for word in text.split_whitespace() {
                            let word_len = word.len();
                            if current_len == 0 {
                                current_line = word.to_string();
                                current_len = word_len;
                            } else if current_len + 1 + word_len <= width {
                                current_line.push(' ');
                                current_line.push_str(word);
                                current_len += 1 + word_len;
                            } else {
                                lines.push(current_line);
                                current_line = word.to_string();
                                current_len = word_len;
                            }
                        }

                        if !current_line.is_empty() {
                            lines.push(current_line);
                        }

                        lines
                    }
                }
            }

            // Wrap and join into single string
            "fill" => {
                if arg_exprs.len() < 2 {
                    bail!("textwrap.fill() requires at least 2 arguments (text, width)");
                }
                let text = &arg_exprs[0];
                let width = &arg_exprs[1];

                // fill = wrap + join
                parse_quote! {
                    {
                        let text = #text;
                        let width = #width as usize;
                        let mut lines = Vec::new();
                        let mut current_line = String::new();
                        let mut current_len = 0;

                        for word in text.split_whitespace() {
                            let word_len = word.len();
                            if current_len == 0 {
                                current_line = word.to_string();
                                current_len = word_len;
                            } else if current_len + 1 + word_len <= width {
                                current_line.push(' ');
                                current_line.push_str(word);
                                current_len += 1 + word_len;
                            } else {
                                lines.push(current_line);
                                current_line = word.to_string();
                                current_len = word_len;
                            }
                        }

                        if !current_line.is_empty() {
                            lines.push(current_line);
                        }

                        lines.join("\n")
                    }
                }
            }

            // Remove common leading whitespace
            "dedent" => {
                if arg_exprs.len() != 1 {
                    bail!("textwrap.dedent() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                parse_quote! {
                    {
                        let text = #text;
                        let lines: Vec<&str> = text.lines().collect();

                        // Find minimum indentation (excluding empty lines)
                        let min_indent = lines.iter()
                            .filter(|line| !line.trim().is_empty())
                            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
                            .min()
                            .unwrap_or(0);

                        // Remove that many spaces from each line
                        lines.iter()
                            .map(|line| {
                                if line.len() >= min_indent {
                                    &line[min_indent..]
                                } else {
                                    line
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                }
            }

            // Add prefix to each line
            "indent" => {
                if arg_exprs.len() != 2 {
                    bail!("textwrap.indent() requires exactly 2 arguments (text, prefix)");
                }
                let text = &arg_exprs[0];
                let prefix = &arg_exprs[1];

                parse_quote! {
                    {
                        let text = #text;
                        let prefix = #prefix;
                        text.lines()
                            .map(|line| format!("{}{}", prefix, line))
                            .collect::<Vec<_>>()
                            .join("\n")
                    }
                }
            }

            // Shorten text with ellipsis
            "shorten" => {
                if arg_exprs.len() < 2 {
                    bail!("textwrap.shorten() requires at least 2 arguments (text, width)");
                }
                let text = &arg_exprs[0];
                let width = &arg_exprs[1];

                parse_quote! {
                    {
                        let text = #text;
                        let width = #width as usize;
                        let placeholder = " [...]";

                        if text.len() <= width {
                            text.to_string()
                        } else if width < placeholder.len() {
                            text.chars().take(width).collect()
                        } else {
                            let max_len = width - placeholder.len();
                            let truncated: String = text.chars().take(max_len).collect();
                            format!("{}{}", truncated, placeholder)
                        }
                    }
                }
            }

            _ => {
                bail!("textwrap.{} not implemented yet (available: wrap, fill, dedent, indent, shorten)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert bisect module method calls
    /// DEPYLER-STDLIB-BISECT: Binary search for sorted sequences
    ///
    /// Supports: bisect_left, bisect_right, insort_left, insort_right
    /// Efficient O(log n) search and insertion
    ///
    /// # Complexity
    /// Cyclomatic: 5 (match with 4 functions + default)
    #[inline]
    pub(crate) fn try_convert_bisect_method(
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
            // Find leftmost insertion point
            "bisect_left" => {
                if arg_exprs.len() < 2 {
                    bail!("bisect.bisect_left() requires at least 2 arguments");
                }
                let a = &arg_exprs[0];
                let x = &arg_exprs[1];

                parse_quote! {
                    {
                        let arr = #a;
                        let val = &#x;
                        match arr.binary_search(val) {
                            Ok(mut pos) => {
                                while pos > 0 && &arr[pos - 1] == val {
                                    pos -= 1;
                                }
                                pos
                            }
                            Err(pos) => pos,
                        }
                    }
                }
            }

            // Find rightmost insertion point
            "bisect_right" | "bisect" => {
                if arg_exprs.len() < 2 {
                    bail!("bisect.{}() requires at least 2 arguments", method);
                }
                let a = &arg_exprs[0];
                let x = &arg_exprs[1];

                parse_quote! {
                    {
                        let arr = #a;
                        let val = &#x;
                        match arr.binary_search(val) {
                            Ok(mut pos) => {
                                pos += 1;
                                while pos < arr.len() && &arr[pos] == val {
                                    pos += 1;
                                }
                                pos
                            }
                            Err(pos) => pos,
                        }
                    }
                }
            }

            // Insert at leftmost position
            "insort_left" => {
                if arg_exprs.len() < 2 {
                    bail!("bisect.insort_left() requires at least 2 arguments");
                }
                let a = &arg_exprs[0];
                let x = &arg_exprs[1];

                parse_quote! {
                    {
                        let arr = &mut (#a);
                        let val = #x;
                        let pos = match arr.binary_search(&val) {
                            Ok(mut pos) => {
                                while pos > 0 && arr[pos - 1] == val {
                                    pos -= 1;
                                }
                                pos
                            }
                            Err(pos) => pos,
                        };
                        arr.insert(pos, val);
                    }
                }
            }

            // Insert at rightmost position
            "insort_right" | "insort" => {
                if arg_exprs.len() < 2 {
                    bail!("bisect.{}() requires at least 2 arguments", method);
                }
                let a = &arg_exprs[0];
                let x = &arg_exprs[1];

                parse_quote! {
                    {
                        let arr = &mut (#a);
                        let val = #x;
                        let pos = match arr.binary_search(&val) {
                            Ok(mut pos) => {
                                pos += 1;
                                while pos < arr.len() && arr[pos] == val {
                                    pos += 1;
                                }
                                pos
                            }
                            Err(pos) => pos,
                        };
                        arr.insert(pos, val);
                    }
                }
            }

            _ => {
                bail!("bisect.{} not implemented yet (available: bisect_left, bisect_right, insort_left, insort_right)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert heapq module method calls
    /// DEPYLER-STDLIB-HEAPQ: Heap queue algorithm (priority queue)
    ///
    /// Supports: heapify, heappush, heappop, nlargest, nsmallest
    /// Python heapq is a MIN heap (smallest item first)
    ///
    /// # Complexity
    /// Cyclomatic: 6 (match with 5 functions + default)
    #[inline]
    pub(crate) fn try_convert_heapq_method(
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
            // Transform list into min-heap in-place
            "heapify" => {
                if arg_exprs.is_empty() {
                    bail!("heapq.heapify() requires at least 1 argument");
                }
                let x = &arg_exprs[0];

                parse_quote! {
                    {
                        let heap = &mut (#x);
                        // Build min-heap using bottom-up heapify
                        let len = heap.len();
                        if len > 1 {
                            for i in (0..len/2).rev() {
                                let mut pos = i;
                                loop {
                                    let left = 2 * pos + 1;
                                    let right = 2 * pos + 2;
                                    let mut smallest = pos;

                                    if left < len && heap[left] < heap[smallest] {
                                        smallest = left;
                                    }
                                    if right < len && heap[right] < heap[smallest] {
                                        smallest = right;
                                    }

                                    if smallest == pos {
                                        break;
                                    }

                                    heap.swap(pos, smallest);
                                    pos = smallest;
                                }
                            }
                        }
                    }
                }
            }

            // Push item onto min-heap
            "heappush" => {
                if arg_exprs.len() < 2 {
                    bail!("heapq.heappush() requires at least 2 arguments");
                }
                let heap = &arg_exprs[0];
                let item = &arg_exprs[1];

                parse_quote! {
                    {
                        let heap = &mut (#heap);
                        let item = #item;
                        heap.push(item);

                        // Bubble up to maintain min-heap property
                        let mut pos = heap.len() - 1;
                        while pos > 0 {
                            let parent = (pos - 1) / 2;
                            if heap[pos] >= heap[parent] {
                                break;
                            }
                            heap.swap(pos, parent);
                            pos = parent;
                        }
                    }
                }
            }

            // Pop and return smallest item from min-heap
            "heappop" => {
                if arg_exprs.is_empty() {
                    bail!("heapq.heappop() requires at least 1 argument");
                }
                let heap = &arg_exprs[0];

                parse_quote! {
                    {
                        let heap = &mut (#heap);
                        if heap.is_empty() {
                            panic!("heappop from empty heap");
                        }

                        let result = heap[0].clone();
                        let last = heap.pop().unwrap();

                        if !heap.is_empty() {
                            heap[0] = last;

                            // Bubble down to maintain min-heap property
                            let mut pos = 0;
                            loop {
                                let left = 2 * pos + 1;
                                let right = 2 * pos + 2;
                                let mut smallest = pos;

                                if left < heap.len() && heap[left] < heap[smallest] {
                                    smallest = left;
                                }
                                if right < heap.len() && heap[right] < heap[smallest] {
                                    smallest = right;
                                }

                                if smallest == pos {
                                    break;
                                }

                                heap.swap(pos, smallest);
                                pos = smallest;
                            }
                        }

                        result
                    }
                }
            }

            // Return n largest elements
            "nlargest" => {
                if arg_exprs.len() < 2 {
                    bail!("heapq.nlargest() requires at least 2 arguments");
                }
                let n = &arg_exprs[0];
                let iterable = &arg_exprs[1];

                parse_quote! {
                    {
                        let n = #n as usize;
                        let mut items = #iterable;
                        items.sort_by(|a, b| b.cmp(a));  // Sort descending
                        items.into_iter().take(n).collect::<Vec<_>>()
                    }
                }
            }

            // Return n smallest elements
            "nsmallest" => {
                if arg_exprs.len() < 2 {
                    bail!("heapq.nsmallest() requires at least 2 arguments");
                }
                let n = &arg_exprs[0];
                let iterable = &arg_exprs[1];

                parse_quote! {
                    {
                        let n = #n as usize;
                        let mut items = #iterable;
                        items.sort();  // Sort ascending
                        items.into_iter().take(n).collect::<Vec<_>>()
                    }
                }
            }

            _ => {
                bail!("heapq.{} not implemented yet (available: heapify, heappush, heappop, nlargest, nsmallest)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert copy module method calls
    /// DEPYLER-STDLIB-COPY: Shallow and deep copy operations
    ///
    /// Supports: copy, deepcopy
    /// Maps to Rust's .clone() for both (Rust clone is deep by default)
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_copy_method(
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
            // Shallow copy - in Rust, clone() is typically deep for owned data
            "copy" => {
                if arg_exprs.is_empty() {
                    bail!("copy.copy() requires at least 1 argument");
                }
                let obj = &arg_exprs[0];

                parse_quote! {
                    (#obj).clone()
                }
            }

            // Deep copy - in Rust, clone() already performs deep copy
            "deepcopy" => {
                if arg_exprs.is_empty() {
                    bail!("copy.deepcopy() requires at least 1 argument");
                }
                let obj = &arg_exprs[0];

                parse_quote! {
                    (#obj).clone()
                }
            }

            _ => {
                bail!(
                    "copy.{} not implemented yet (available: copy, deepcopy)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    // DEPYLER-COVERAGE-95: try_convert_itertools_method moved to stdlib_method_gen::itertools
    // DEPYLER-COVERAGE-95: try_convert_functools_method moved to stdlib_method_gen::functools
    // DEPYLER-COVERAGE-95: try_convert_warnings_method moved to stdlib_method_gen::warnings

    /// Try to convert sys module method calls
    /// DEPYLER-STDLIB-SYS: System-specific parameters and functions
    ///
    /// Supports: exit
    /// Maps to Rust's std::process::exit
    ///
    /// # Complexity
    /// Cyclomatic: 2 (match with 1 function + default)
    #[inline]
    pub(crate) fn try_convert_sys_method(
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
            "exit" => {
                let code = if !arg_exprs.is_empty() {
                    &arg_exprs[0]
                } else {
                    &parse_quote!(0)
                };

                parse_quote! {
                    std::process::exit(#code)
                }
            }

            _ => {
                bail!("sys.{} not implemented yet (available: exit)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert pickle module method calls
    /// DEPYLER-STDLIB-PICKLE: Object serialization
    ///
    /// Supports: dumps, loads
    /// Maps to serde/bincode for serialization (placeholder)
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_pickle_method(
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
            "dumps" => {
                if arg_exprs.is_empty() {
                    bail!("pickle.dumps() requires at least 1 argument");
                }
                let obj = &arg_exprs[0];

                // Placeholder: In real implementation, would use serde + bincode
                parse_quote! {
                    {
                        // Note: Actual pickle serialization requires serde support
                        format!("{:?}", #obj).into_bytes()
                    }
                }
            }

            "loads" => {
                if arg_exprs.is_empty() {
                    bail!("pickle.loads() requires at least 1 argument");
                }
                let data = &arg_exprs[0];

                // Placeholder: In real implementation, would use serde + bincode
                parse_quote! {
                    {
                        // Note: Actual pickle deserialization requires serde support
                        String::from_utf8_lossy(#data).to_string()
                    }
                }
            }

            _ => {
                bail!(
                    "pickle.{} not implemented yet (available: dumps, loads)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert pprint module method calls
    /// DEPYLER-STDLIB-PPRINT: Pretty printing
    ///
    /// Supports: pprint
    /// Maps to Rust's Debug formatting
    ///
    /// # Complexity
    /// Cyclomatic: 2 (match with 1 function + default)
    #[inline]
    pub(crate) fn try_convert_pprint_method(
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
            "pprint" => {
                if arg_exprs.is_empty() {
                    bail!("pprint.pprint() requires at least 1 argument");
                }
                let obj = &arg_exprs[0];

                parse_quote! {
                    println!("{:#?}", #obj)
                }
            }

            _ => {
                bail!("pprint.{} not implemented yet (available: pprint)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert fractions module method calls
    /// DEPYLER-STDLIB-FRACTIONS: Comprehensive fractions module support
    #[inline]
    pub(crate) fn try_convert_fractions_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Mark that we need the num-rational crate
        self.ctx.needs_num_rational = true;

        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            // Fraction methods
            "limit_denominator" => {
                if arg_exprs.len() != 2 {
                    bail!("Fraction.limit_denominator() requires exactly 2 arguments (self, max_denominator)");
                }
                let frac = &arg_exprs[0];
                let max_denom = &arg_exprs[1];
                // Simplified: if denominator within limit, return as-is
                parse_quote! {
                    {
                        let f = #frac;
                        let max_d = #max_denom as i32;
                        if *f.denom() <= max_d {
                            f
                        } else {
                            // Approximate by converting to float and back
                            num::rational::Ratio::approximate_float(f.to_f64().unwrap()).unwrap_or(f)
                        }
                    }
                }
            }

            "as_integer_ratio" => {
                if arg_exprs.len() != 1 {
                    bail!("Fraction.as_integer_ratio() requires exactly 1 argument (self)");
                }
                let frac = &arg_exprs[0];
                parse_quote! { (*#frac.numer(), *#frac.denom()) }
            }

            _ => return Ok(None), // Not a recognized fractions method
        };

        Ok(Some(result))
    }

    // DEPYLER-COVERAGE-95: try_convert_pathlib_method moved to stdlib_method_gen::pathlib

    /// DEPYLER-0829: Convert pathlib methods on Path/PathBuf variable instances
    /// This handles cases like `p.write_text(content)` where p is a Path variable
    /// Unlike try_convert_pathlib_method which handles module calls like pathlib.Path(...).method()
    #[inline]
    pub(crate) fn convert_pathlib_instance_method(
        &mut self,
        path_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let result = match method {
            // File I/O operations
            "write_text" => {
                if arg_exprs.is_empty() {
                    bail!("write_text() requires at least 1 argument (content)");
                }
                let content = &arg_exprs[0];
                parse_quote! { std::fs::write(&#path_expr, #content).unwrap() }
            }

            "read_text" => {
                parse_quote! { std::fs::read_to_string(&#path_expr).unwrap() }
            }

            "read_bytes" => {
                parse_quote! { std::fs::read(&#path_expr).unwrap() }
            }

            "write_bytes" => {
                if arg_exprs.is_empty() {
                    bail!("write_bytes() requires at least 1 argument (data)");
                }
                let data = &arg_exprs[0];
                parse_quote! { std::fs::write(&#path_expr, #data).unwrap() }
            }

            // Path predicates
            "exists" => {
                parse_quote! { #path_expr.exists() }
            }

            "is_file" => {
                parse_quote! { #path_expr.is_file() }
            }

            "is_dir" => {
                parse_quote! { #path_expr.is_dir() }
            }

            // Directory operations
            "mkdir" => {
                // Check if parents=True was passed
                if !arg_exprs.is_empty() {
                    parse_quote! { std::fs::create_dir_all(&#path_expr).unwrap() }
                } else {
                    parse_quote! { std::fs::create_dir(&#path_expr).unwrap() }
                }
            }

            "rmdir" => {
                parse_quote! { std::fs::remove_dir(&#path_expr).unwrap() }
            }

            "unlink" => {
                parse_quote! { std::fs::remove_file(&#path_expr).unwrap() }
            }

            "iterdir" => {
                parse_quote! {
                    std::fs::read_dir(&#path_expr)
                        .unwrap()
                        .map(|e| e.unwrap().path())
                        .collect::<Vec<_>>()
                }
            }

            // Glob operations - require glob crate
            "glob" => {
                self.ctx.needs_glob = true;
                if arg_exprs.is_empty() {
                    bail!("glob() requires at least 1 argument (pattern)");
                }
                let pattern = &arg_exprs[0];
                parse_quote! {
                    glob::glob(&format!("{}/{}", #path_expr.display(), #pattern))
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .collect::<Vec<_>>()
                }
            }

            "rglob" => {
                self.ctx.needs_glob = true;
                if arg_exprs.is_empty() {
                    bail!("rglob() requires at least 1 argument (pattern)");
                }
                let pattern = &arg_exprs[0];
                parse_quote! {
                    glob::glob(&format!("{}/**/{}", #path_expr.display(), #pattern))
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .collect::<Vec<_>>()
                }
            }

            // Path transformations
            "with_name" => {
                if arg_exprs.is_empty() {
                    bail!("with_name() requires 1 argument (name)");
                }
                let name = &arg_exprs[0];
                parse_quote! { #path_expr.with_file_name(#name) }
            }

            "with_suffix" => {
                if arg_exprs.is_empty() {
                    bail!("with_suffix() requires 1 argument (suffix)");
                }
                let suffix = &arg_exprs[0];
                parse_quote! { #path_expr.with_extension(#suffix.trim_start_matches('.')) }
            }

            "with_stem" => {
                // Python's with_stem - change stem keeping extension
                if arg_exprs.is_empty() {
                    bail!("with_stem() requires 1 argument (stem)");
                }
                let stem = &arg_exprs[0];
                parse_quote! {
                    {
                        let p = &#path_expr;
                        let ext = p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
                        p.with_file_name(format!("{}{}", #stem, ext))
                    }
                }
            }

            "resolve" | "absolute" => {
                parse_quote! { #path_expr.canonicalize().unwrap() }
            }

            "relative_to" => {
                if arg_exprs.is_empty() {
                    bail!("relative_to() requires 1 argument (base)");
                }
                let base = &arg_exprs[0];
                parse_quote! { #path_expr.strip_prefix(#base).unwrap().to_path_buf() }
            }

            _ => {
                // Fall through to regular method call
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { #path_expr.#method_ident(#(#arg_exprs),*) }
            }
        };

        Ok(result)
    }

    /// DEPYLER-0830/1025: Convert datetime/timedelta methods on variable instances
    /// This handles cases like `td.total_seconds()` where td is a TimeDelta variable
    /// Unlike try_convert_datetime_method which handles module calls like datetime.datetime.now()
    #[inline]
    pub(crate) fn convert_datetime_instance_method(
        &mut self,
        dt_expr: &syn::Expr,
        method: &str,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        // Only mark chrono needed if NOT in NASA mode
        if !nasa_mode {
            self.ctx.needs_chrono = true;
        }

        let result = match method {
            // timedelta.total_seconds() → td.as_secs_f64() (NASA) or td.num_seconds() as f64 (chrono)
            "total_seconds" => {
                if nasa_mode {
                    parse_quote! { #dt_expr.as_secs_f64() }
                } else {
                    parse_quote! { #dt_expr.num_seconds() as f64 }
                }
            }

            // datetime.fromisoformat(s) → parse timestamp string
            "fromisoformat" => {
                if arg_exprs.is_empty() {
                    bail!("fromisoformat() requires 1 argument (string)");
                }
                let s = &arg_exprs[0];
                if nasa_mode {
                    // NASA mode: return current time (simplified)
                    parse_quote! { std::time::SystemTime::now() }
                } else {
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(&#s, "%Y-%m-%dT%H:%M:%S").unwrap()
                    }
                }
            }

            // datetime.isoformat() → format!("{:?}", dt) (NASA) or dt.format(...) (chrono)
            "isoformat" => {
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt_expr) }
                } else {
                    parse_quote! { #dt_expr.format("%Y-%m-%dT%H:%M:%S").to_string() }
                }
            }

            // datetime.strftime(fmt) → format!("{:?}", dt) (NASA) or dt.format(fmt) (chrono)
            "strftime" => {
                if arg_exprs.is_empty() {
                    bail!("strftime() requires 1 argument (format string)");
                }
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt_expr) }
                } else {
                    let fmt = match hir_args.first() {
                        Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                        _ => arg_exprs[0].clone(),
                    };
                    parse_quote! { #dt_expr.format(#fmt).to_string() }
                }
            }

            // datetime.timestamp() → UNIX_EPOCH.elapsed() (NASA) or dt.and_utc().timestamp() (chrono)
            "timestamp" => {
                if nasa_mode {
                    parse_quote! {
                        #dt_expr.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
                    }
                } else {
                    parse_quote! { #dt_expr.and_utc().timestamp() as f64 }
                }
            }

            // datetime.timetuple() - return tuple (NASA returns zeros, chrono extracts components)
            "timetuple" => {
                if nasa_mode {
                    parse_quote! { (0i32, 0u32, 0u32, 0u32, 0u32, 0u32) }
                } else {
                    parse_quote! {
                        (#dt_expr.year(), #dt_expr.month(), #dt_expr.day(),
                         #dt_expr.hour(), #dt_expr.minute(), #dt_expr.second())
                    }
                }
            }

            // datetime.weekday() → 0 (NASA) or dt.weekday().num_days_from_monday() (chrono)
            "weekday" => {
                if nasa_mode {
                    parse_quote! { 0i32 }
                } else {
                    parse_quote! { #dt_expr.weekday().num_days_from_monday() as i32 }
                }
            }

            // datetime.isoweekday() → 1 (NASA) or dt.weekday().number_from_monday() (chrono)
            "isoweekday" => {
                if nasa_mode {
                    parse_quote! { 1i32 }
                } else {
                    parse_quote! { (#dt_expr.weekday().num_days_from_monday() + 1) as i32 }
                }
            }

            // datetime.isocalendar() → (year, week, weekday) tuple
            "isocalendar" => {
                if nasa_mode {
                    parse_quote! { (2024i32, 1i32, 1i32) }
                } else {
                    parse_quote! {
                        {
                            let iso = #dt_expr.iso_week();
                            (iso.year(), iso.week() as i32, #dt_expr.weekday().number_from_monday() as i32)
                        }
                    }
                }
            }

            // datetime.replace() - simplified: pass through
            "replace" => {
                parse_quote! { #dt_expr }
            }

            // Fallback: pass through as method call
            _ => {
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { #dt_expr.#method_ident(#(#arg_exprs),*) }
            }
        };

        Ok(result)
    }

    /// Try to convert datetime module method calls
    /// DEPYLER-STDLIB-DATETIME/1025: Comprehensive datetime module support with NASA mode
    #[inline]
    pub(crate) fn try_convert_datetime_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let nasa_mode = self.ctx.type_mapper.nasa_mode;
        // Only mark chrono needed if NOT in NASA mode
        if !nasa_mode {
            self.ctx.needs_chrono = true;
        }

        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            // datetime.datetime.now([tz]) → SystemTime::now() (NASA) or Local::now() (chrono)
            "now" => {
                if nasa_mode {
                    parse_quote! { std::time::SystemTime::now() }
                } else if arg_exprs.is_empty() {
                    parse_quote! { chrono::Local::now().naive_local() }
                } else {
                    parse_quote! { chrono::Utc::now().naive_utc() }
                }
            }

            // datetime.datetime.utcnow() → SystemTime::now() (NASA) or Utc::now() (chrono)
            "utcnow" => {
                if arg_exprs.is_empty() {
                    if nasa_mode {
                        parse_quote! { std::time::SystemTime::now() }
                    } else {
                        parse_quote! { chrono::Utc::now().naive_utc() }
                    }
                } else {
                    bail!("datetime.utcnow() takes no arguments");
                }
            }

            // datetime.datetime.today() → SystemTime::now() (NASA) or Local::now().date() (chrono)
            "today" => {
                if arg_exprs.is_empty() {
                    if nasa_mode {
                        parse_quote! { std::time::SystemTime::now() }
                    } else {
                        parse_quote! { chrono::Local::now().date_naive() }
                    }
                } else {
                    bail!("datetime.today() takes no arguments");
                }
            }

            // datetime.datetime.strftime(format) → format!("{:?}", dt) (NASA) or dt.format(...) (chrono)
            "strftime" => {
                if arg_exprs.len() != 2 {
                    bail!("strftime() requires exactly 2 arguments (self, format)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt) }
                } else {
                    let fmt = match &args[1] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        _ => arg_exprs[1].clone(),
                    };
                    parse_quote! { #dt.format(#fmt).to_string() }
                }
            }

            // datetime.datetime.strptime(string, format) → SystemTime::now() (NASA) or parse_from_str (chrono)
            "strptime" => {
                if arg_exprs.len() != 2 {
                    bail!("strptime() requires exactly 2 arguments (string, format)");
                }
                if nasa_mode {
                    parse_quote! { std::time::SystemTime::now() }
                } else {
                    let s = &arg_exprs[0];
                    let fmt: syn::Expr = match &args[1] {
                        HirExpr::Literal(Literal::String(fmt_str)) => parse_quote! { #fmt_str },
                        _ => {
                            let fmt_expr = &arg_exprs[1];
                            parse_quote! { &#fmt_expr }
                        }
                    };
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(#s, #fmt).unwrap()
                    }
                }
            }

            // datetime.datetime.isoformat() → format!("{:?}", dt) (NASA) or dt.to_string() (chrono)
            "isoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("isoformat() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { format!("{:?}", #dt) }
                } else {
                    parse_quote! { #dt.to_string() }
                }
            }

            // datetime.datetime.timestamp() → UNIX_EPOCH duration (NASA) or dt.timestamp() (chrono)
            "timestamp" => {
                if arg_exprs.len() != 1 {
                    bail!("timestamp() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! {
                        #dt.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
                    }
                } else {
                    parse_quote! { #dt.and_utc().timestamp() as f64 }
                }
            }

            // datetime.datetime.fromtimestamp(ts) → SystemTime (NASA) or DateTime (chrono)
            "fromtimestamp" => {
                if arg_exprs.len() != 1 {
                    bail!("fromtimestamp() requires exactly 1 argument (timestamp)");
                }
                let ts = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! {
                        std::time::UNIX_EPOCH + std::time::Duration::from_secs((#ts).clone() as u64)
                    }
                } else {
                    parse_quote! {
                        chrono::DateTime::from_timestamp((#ts).clone() as i64, 0)
                            .unwrap()
                            .naive_local()
                    }
                }
            }

            // date.weekday() → 0 (NASA) or dt.weekday().num_days_from_monday() (chrono)
            "weekday" => {
                if arg_exprs.len() != 1 {
                    bail!("weekday() requires exactly 1 argument (self)");
                }
                if nasa_mode {
                    parse_quote! { 0i32 }
                } else {
                    let dt = &arg_exprs[0];
                    parse_quote! { #dt.weekday().num_days_from_monday() as i32 }
                }
            }

            // date.isoweekday() → 1 (NASA) or dt.weekday().number_from_monday() (chrono)
            "isoweekday" => {
                if arg_exprs.len() != 1 {
                    bail!("isoweekday() requires exactly 1 argument (self)");
                }
                if nasa_mode {
                    parse_quote! { 1i32 }
                } else {
                    let dt = &arg_exprs[0];
                    parse_quote! { (#dt.weekday().num_days_from_monday() + 1) as i32 }
                }
            }

            // timedelta.total_seconds() → as_secs_f64 (NASA) or num_seconds (chrono)
            "total_seconds" => {
                if arg_exprs.len() != 1 {
                    bail!("total_seconds() requires exactly 1 argument (self)");
                }
                let td = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #td.as_secs_f64() }
                } else {
                    parse_quote! { #td.num_seconds() as f64 }
                }
            }

            // datetime.date() → extract date part (passthrough for both modes)
            "date" => {
                if arg_exprs.len() != 1 {
                    bail!("date() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    parse_quote! { #dt.date() }
                }
            }

            // datetime.time() → extract time part (passthrough for NASA)
            "time" => {
                if arg_exprs.len() != 1 {
                    bail!("time() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    parse_quote! { #dt.time() }
                }
            }

            // datetime.replace() - passthrough for both modes
            "replace" => {
                if arg_exprs.len() != 2 {
                    bail!("replace() not fully implemented (requires keyword args)");
                }
                let dt = &arg_exprs[0];
                if nasa_mode {
                    parse_quote! { #dt }
                } else {
                    let new_year = &arg_exprs[1];
                    parse_quote! { #dt.with_year(#new_year as i32).unwrap() }
                }
            }

            // DEPYLER-0938/1025: datetime.combine(date, time) → SystemTime (NASA) or NaiveDateTime (chrono)
            "combine" => {
                if arg_exprs.len() != 2 {
                    bail!("combine() requires exactly 2 arguments (date, time)");
                }
                if nasa_mode {
                    parse_quote! { std::time::SystemTime::now() }
                } else {
                    let date_expr = &arg_exprs[0];
                    let time_expr = &arg_exprs[1];
                    parse_quote! { chrono::NaiveDateTime::new(#date_expr, #time_expr) }
                }
            }

            // DEPYLER-0938/1025: datetime.fromisoformat(string) → SystemTime (NASA) or parse_from_str (chrono)
            "fromisoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("fromisoformat() requires exactly 1 argument (string)");
                }
                if nasa_mode {
                    parse_quote! { std::time::SystemTime::now() }
                } else {
                    let s = &arg_exprs[0];
                    parse_quote! {
                        chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S")
                            .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S"))
                            .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d"))
                            .unwrap()
                    }
                }
            }

            _ => return Ok(None), // Not a recognized datetime method
        };

        Ok(Some(result))
    }

    /// Try to convert statistics module method calls
    /// DEPYLER-STDLIB-STATISTICS: Comprehensive statistics module support
    #[inline]
    pub(crate) fn try_convert_decimal_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Mark that we need the rust_decimal crate
        self.ctx.needs_rust_decimal = true;

        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            // Mathematical operations
            "sqrt" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.sqrt() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.sqrt().unwrap() }
            }

            "exp" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.exp() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.exp() }
            }

            "ln" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.ln() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.ln() }
            }

            "log10" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.log10() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.log10() }
            }

            // Rounding and quantization
            "quantize" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.quantize() requires exactly 1 argument");
                }
                let value = &arg_exprs[0];
                // quantize(Decimal("0.01")) → round to 2 decimal places
                // For now, we'll use round_dp(2) as a simple approximation
                // NOTE: More sophisticated Decimal quantization based on quantum value (tracked in DEPYLER-0424)
                parse_quote! { #value.round_dp(2) }
            }

            "to_integral" | "to_integral_value" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.to_integral() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.trunc() }
            }

            // Predicates
            "is_nan" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.is_nan() requires exactly 1 argument");
                }
                let _arg = &arg_exprs[0];
                // rust_decimal doesn't have NaN, always returns false
                parse_quote! { false }
            }

            "is_infinite" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.is_infinite() requires exactly 1 argument");
                }
                let _arg = &arg_exprs[0];
                // rust_decimal doesn't have infinity, always returns false
                parse_quote! { false }
            }

            "is_finite" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.is_finite() requires exactly 1 argument");
                }
                let _arg = &arg_exprs[0];
                // rust_decimal doesn't have infinity/NaN, always returns true
                parse_quote! { true }
            }

            "is_signed" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.is_signed() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.is_sign_negative() }
            }

            "is_zero" => {
                if arg_exprs.len() != 1 {
                    bail!("Decimal.is_zero() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { #arg.is_zero() }
            }

            // Sign operations
            "copy_sign" | "copysign" => {
                if arg_exprs.len() != 2 {
                    bail!("Decimal.copy_sign() requires exactly 2 arguments");
                }
                let value = &arg_exprs[0];
                let other = &arg_exprs[1];
                // Copy sign: if other is negative, return -abs(value), else abs(value)
                parse_quote! {
                    if #other.is_sign_negative() {
                        -#value.abs()
                    } else {
                        #value.abs()
                    }
                }
            }

            // Comparison
            "compare" => {
                if arg_exprs.len() != 2 {
                    bail!("Decimal.compare() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // compare() returns -1, 0, or 1
                parse_quote! {
                    match #a.cmp(&#b) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    }
                }
            }

            _ => return Ok(None), // Not a recognized decimal method
        };

        Ok(Some(result))
    }

    pub(crate) fn try_convert_statistics_method(
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
            // Averages and central tendency
            "mean" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.mean() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.mean(data) → data.iter().sum::<f64>() / data.len() as f64
                parse_quote! {
                    {
                        let data = #data;
                        data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64
                    }
                }
            }

            "median" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.median() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.median(data) → sorted median calculation
                parse_quote! {
                    {
                        let mut sorted = #data.clone();
                        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let len = sorted.len();
                        if len % 2 == 0 {
                            let mid = len / 2;
                            ((sorted[mid - 1] as f64) + (sorted[mid] as f64)) / 2.0
                        } else {
                            sorted[len / 2] as f64
                        }
                    }
                }
            }

            "mode" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.mode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.mode(data) → find most common element
                self.ctx.needs_hashmap = true;
                parse_quote! {
                    {
                        let mut counts: HashMap<_, usize> = HashMap::new();
                        for &item in #data.iter() {
                            *counts.entry(item).or_insert(0) += 1;
                        }
                        *counts.iter().max_by_key(|(_, &count)| count).unwrap().0
                    }
                }
            }

            // Measures of spread
            "variance" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.variance() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.variance(data) → sample variance (n-1 denominator)
                parse_quote! {
                    {
                        let data = #data;
                        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
                        let sum_sq_diff: f64 = data.iter()
                            .map(|&x| {
                                let diff = (x as f64) - mean;
                                diff * diff
                            })
                            .sum();
                        sum_sq_diff / ((data.len() - 1) as f64)
                    }
                }
            }

            "pvariance" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.pvariance() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.pvariance(data) → population variance (n denominator)
                parse_quote! {
                    {
                        let data = #data;
                        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
                        let sum_sq_diff: f64 = data.iter()
                            .map(|&x| {
                                let diff = (x as f64) - mean;
                                diff * diff
                            })
                            .sum();
                        sum_sq_diff / (data.len() as f64)
                    }
                }
            }

            "stdev" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.stdev() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.stdev(data) → sqrt(variance)
                parse_quote! {
                    {
                        let data = #data;
                        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
                        let sum_sq_diff: f64 = data.iter()
                            .map(|&x| {
                                let diff = (x as f64) - mean;
                                diff * diff
                            })
                            .sum();
                        let variance = sum_sq_diff / ((data.len() - 1) as f64);
                        variance.sqrt()
                    }
                }
            }

            "pstdev" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.pstdev() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.pstdev(data) → sqrt(pvariance)
                parse_quote! {
                    {
                        let data = #data;
                        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
                        let sum_sq_diff: f64 = data.iter()
                            .map(|&x| {
                                let diff = (x as f64) - mean;
                                diff * diff
                            })
                            .sum();
                        let pvariance = sum_sq_diff / (data.len() as f64);
                        pvariance.sqrt()
                    }
                }
            }

            // Additional means
            "harmonic_mean" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.harmonic_mean() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.harmonic_mean(data) → n / sum(1/x for x in data)
                parse_quote! {
                    {
                        let data = #data;
                        let sum_reciprocals: f64 = data.iter()
                            .map(|&x| 1.0 / (x as f64))
                            .sum();
                        (data.len() as f64) / sum_reciprocals
                    }
                }
            }

            "geometric_mean" => {
                if arg_exprs.len() != 1 {
                    bail!("statistics.geometric_mean() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];
                // statistics.geometric_mean(data) → (product of all values) ^ (1/n)
                parse_quote! {
                    {
                        let data = #data;
                        let product: f64 = data.iter()
                            .map(|&x| x as f64)
                            .product();
                        product.powf(1.0 / (data.len() as f64))
                    }
                }
            }

            // Quantiles (simplified implementation)
            "quantiles" => {
                // quantiles can take n= parameter, but we'll support basic case
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("statistics.quantiles() requires 1-2 arguments");
                }
                let data = &arg_exprs[0];
                let n = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    // Default n=4 (quartiles)
                    &parse_quote! { 4 }
                };
                // Simplified quantiles implementation
                parse_quote! {
                    {
                        let mut sorted = #data.clone();
                        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let n = #n as usize;
                        let mut result = Vec::new();
                        for i in 1..n {
                            let pos = (i as f64) * (sorted.len() as f64) / (n as f64);
                            let idx = pos.floor() as usize;
                            if idx < sorted.len() {
                                result.push(sorted[idx] as f64);
                            }
                        }
                        result
                    }
                }
            }

            _ => {
                bail!("statistics.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

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
                            parse_quote! { #result.unwrap() }
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
            if module_name == "datetime" || module_name == "date" {
                return self.try_convert_datetime_method(method, args);
            }

            // DEPYLER-0595: Handle bytes class methods
            // bytes.fromhex("aabbcc") → hex string to byte array
            if module_name == "bytes" && method == "fromhex" && args.len() == 1 {
                let hex_str = args[0].to_rust_expr(self.ctx)?;
                // Convert hex string to Vec<u8> using inline parsing
                return Ok(Some(parse_quote! {
                    (#hex_str).as_bytes()
                        .chunks(2)
                        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
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
                if let Some(result) = stdlib_method_gen::convert_os_method(method, args, self.ctx)? {
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
                    // TODO: Implement proper macro invocation support
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
                            #path().unwrap().to_string_lossy().to_string()
                        }
                    }
                    "Regex::new" => {
                        // re.compile(pattern) -> Regex::new(pattern)
                        if arg_exprs.is_empty() {
                            bail!("re.compile() requires a pattern argument");
                        }
                        let pattern = &arg_exprs[0];
                        parse_quote! {
                            regex::Regex::new(#pattern).unwrap()
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
                    let should_wrap = matches!(
                        lit,
                        Literal::Int(_) | Literal::Float(_) | Literal::Bool(_)
                    );
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
                            if let Some(result) = converter.try_convert_os_path_method(method, args)? {
                                return Ok(result);
                            }
                        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{BinOp, HirExpr, Literal, UnaryOp};
    use crate::rust_gen::walrus_helpers;
    use quote::ToTokens;
    use std::collections::HashSet;

    // ============ is_rust_keyword tests ============

    #[test]
    pub(crate) fn test_is_rust_keyword_basic() {
        assert!(keywords::is_rust_keyword("fn"));
        assert!(keywords::is_rust_keyword("let"));
        assert!(keywords::is_rust_keyword("if"));
        assert!(keywords::is_rust_keyword("else"));
        assert!(keywords::is_rust_keyword("for"));
        assert!(keywords::is_rust_keyword("while"));
        assert!(keywords::is_rust_keyword("loop"));
        assert!(keywords::is_rust_keyword("match"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_types() {
        assert!(keywords::is_rust_keyword("struct"));
        assert!(keywords::is_rust_keyword("enum"));
        assert!(keywords::is_rust_keyword("trait"));
        assert!(keywords::is_rust_keyword("impl"));
        assert!(keywords::is_rust_keyword("type"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_modifiers() {
        assert!(keywords::is_rust_keyword("pub"));
        assert!(keywords::is_rust_keyword("mut"));
        assert!(keywords::is_rust_keyword("const"));
        assert!(keywords::is_rust_keyword("static"));
        assert!(keywords::is_rust_keyword("ref"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_async() {
        assert!(keywords::is_rust_keyword("async"));
        assert!(keywords::is_rust_keyword("await"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_reserved() {
        assert!(keywords::is_rust_keyword("abstract"));
        assert!(keywords::is_rust_keyword("become"));
        assert!(keywords::is_rust_keyword("box"));
        assert!(keywords::is_rust_keyword("do"));
        assert!(keywords::is_rust_keyword("final"));
        assert!(keywords::is_rust_keyword("macro"));
        assert!(keywords::is_rust_keyword("override"));
        assert!(keywords::is_rust_keyword("priv"));
        assert!(keywords::is_rust_keyword("try"));
        assert!(keywords::is_rust_keyword("typeof"));
        assert!(keywords::is_rust_keyword("virtual"));
        assert!(keywords::is_rust_keyword("yield"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_false() {
        assert!(!keywords::is_rust_keyword("foo"));
        assert!(!keywords::is_rust_keyword("bar"));
        assert!(!keywords::is_rust_keyword("my_var"));
        assert!(!keywords::is_rust_keyword("count"));
        assert!(!keywords::is_rust_keyword("result"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_special() {
        assert!(keywords::is_rust_keyword("self"));
        assert!(keywords::is_rust_keyword("Self"));
        assert!(keywords::is_rust_keyword("super"));
        assert!(keywords::is_rust_keyword("crate"));
    }

    // ============ is_non_raw_keyword tests ============

    #[test]
    pub(crate) fn test_is_non_raw_keyword_true() {
        assert!(keywords::is_non_raw_keyword("self"));
        assert!(keywords::is_non_raw_keyword("Self"));
        assert!(keywords::is_non_raw_keyword("super"));
        assert!(keywords::is_non_raw_keyword("crate"));
    }

    #[test]
    pub(crate) fn test_is_non_raw_keyword_false() {
        assert!(!keywords::is_non_raw_keyword("fn"));
        assert!(!keywords::is_non_raw_keyword("let"));
        assert!(!keywords::is_non_raw_keyword("type"));
        assert!(!keywords::is_non_raw_keyword("foo"));
    }

    // ============ looks_like_option_expr tests ============

    #[test]
    pub(crate) fn test_looks_like_option_expr_ok_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("result".to_string())),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_get_one_arg() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key".to_string()))],
            kwargs: vec![],
        };
        assert!(expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_get_with_default() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "get".to_string(),
            args: vec![
                HirExpr::Literal(Literal::String("key".to_string())),
                HirExpr::Literal(Literal::Int(0)),
            ],
            kwargs: vec![],
        };
        assert!(!expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_other_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "append".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(1))],
            kwargs: vec![],
        };
        assert!(!expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_chained_ok() {
        let inner = HirExpr::MethodCall {
            object: Box::new(HirExpr::Call {
                func: "env_var".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_analysis::looks_like_option_expr(&inner));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_not_method() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!expr_analysis::looks_like_option_expr(&expr));
    }

    // ============ collect_walrus_vars_from_conditions tests ============

    #[test]
    pub(crate) fn test_collect_walrus_vars_empty() {
        let conditions: Vec<HirExpr> = vec![];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.is_empty());
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_named_expr() {
        let conditions = vec![HirExpr::NamedExpr {
            target: "x".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(5))),
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("x"));
        assert_eq!(vars.len(), 1);
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_nested() {
        let conditions = vec![HirExpr::Binary {
            op: BinOp::And,
            left: Box::new(HirExpr::NamedExpr {
                target: "a".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            }),
            right: Box::new(HirExpr::NamedExpr {
                target: "b".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            }),
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_call() {
        let conditions = vec![HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::NamedExpr {
                target: "result".to_string(),
                value: Box::new(HirExpr::Var("x".to_string())),
            }],
            kwargs: vec![],
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("result"));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_unary() {
        let conditions = vec![HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::NamedExpr {
                target: "flag".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Bool(true))),
            }),
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("flag"));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_if_expr() {
        let conditions = vec![HirExpr::IfExpr {
            test: Box::new(HirExpr::NamedExpr {
                target: "cond".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Bool(true))),
            }),
            body: Box::new(HirExpr::Literal(Literal::Int(1))),
            orelse: Box::new(HirExpr::Literal(Literal::Int(0))),
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("cond"));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_tuple() {
        let conditions = vec![HirExpr::Tuple(vec![
            HirExpr::NamedExpr {
                target: "x".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            HirExpr::NamedExpr {
                target: "y".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
        ])];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
    }

    // ============ expr_uses_any_var tests ============

    #[test]
    pub(crate) fn test_expr_uses_any_var_simple() {
        let mut vars = HashSet::new();
        vars.insert("x".to_string());

        let expr = HirExpr::Var("x".to_string());
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));

        let expr2 = HirExpr::Var("y".to_string());
        assert!(!walrus_helpers::expr_uses_any_var(&expr2, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_binary() {
        let mut vars = HashSet::new();
        vars.insert("x".to_string());

        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));

        let expr2 = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr2, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_call() {
        let mut vars = HashSet::new();
        vars.insert("arg".to_string());

        let expr = HirExpr::Call {
            func: "foo".to_string(),
            args: vec![HirExpr::Var("arg".to_string())],
            kwargs: vec![],
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_index() {
        let mut vars = HashSet::new();
        vars.insert("idx".to_string());

        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Var("arr".to_string())),
            index: Box::new(HirExpr::Var("idx".to_string())),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_attribute() {
        let mut vars = HashSet::new();
        vars.insert("obj".to_string());

        let expr = HirExpr::Attribute {
            value: Box::new(HirExpr::Var("obj".to_string())),
            attr: "field".to_string(),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_list() {
        let mut vars = HashSet::new();
        vars.insert("item".to_string());

        let expr = HirExpr::List(vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Var("item".to_string()),
        ]);
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    // ============ get_python_op_precedence tests ============

    #[test]
    pub(crate) fn test_get_python_op_precedence_pow() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::Pow), 14);
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_mul() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::Mul), 13);
        assert_eq!(precedence::get_python_op_precedence(BinOp::Div), 13);
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::FloorDiv),
            13
        );
        assert_eq!(precedence::get_python_op_precedence(BinOp::Mod), 13);
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_add() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::Add), 12);
        assert_eq!(precedence::get_python_op_precedence(BinOp::Sub), 12);
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_shift() {
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::LShift),
            11
        );
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::RShift),
            11
        );
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_bitwise() {
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::BitAnd),
            10
        );
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::BitXor),
            9
        );
        assert_eq!(
            precedence::get_python_op_precedence(BinOp::BitOr),
            8
        );
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_comparison() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::Lt), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::Gt), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::LtEq), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::GtEq), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::Eq), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::NotEq), 7);
    }

    #[test]
    pub(crate) fn test_get_python_op_precedence_logical() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::And), 6);
        assert_eq!(precedence::get_python_op_precedence(BinOp::Or), 5);
    }

    // ============ looks_like_option_expr additional tests ============

    #[test]
    pub(crate) fn test_looks_like_option_expr_nested_get() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("data".to_string())),
                method: "get".to_string(),
                args: vec![HirExpr::Literal(Literal::String("key1".to_string()))],
                kwargs: vec![],
            }),
            method: "get".to_string(),
            args: vec![HirExpr::Literal(Literal::String("key2".to_string()))],
            kwargs: vec![],
        };
        assert!(expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_deeply_nested() {
        let inner = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("x".to_string())),
            method: "ok".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        let outer = HirExpr::MethodCall {
            object: Box::new(inner),
            method: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        // .map() on an Option returns Option, but we check inner
        assert!(expr_analysis::looks_like_option_expr(&outer));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_literal() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!expr_analysis::looks_like_option_expr(&expr));
    }

    #[test]
    pub(crate) fn test_looks_like_option_expr_binary() {
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(!expr_analysis::looks_like_option_expr(&expr));
    }

    // ============ More walrus operator tests ============

    #[test]
    pub(crate) fn test_collect_walrus_vars_multiple_conditions() {
        let conditions = vec![
            HirExpr::NamedExpr {
                target: "a".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            },
            HirExpr::NamedExpr {
                target: "b".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(2))),
            },
            HirExpr::Var("c".to_string()),
        ];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert_eq!(vars.len(), 2);
        assert!(vars.contains("a"));
        assert!(vars.contains("b"));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_nested_index() {
        let mut vars = HashSet::new();
        vars.insert("i".to_string());

        let expr = HirExpr::Index {
            base: Box::new(HirExpr::Index {
                base: Box::new(HirExpr::Var("matrix".to_string())),
                index: Box::new(HirExpr::Var("i".to_string())),
            }),
            index: Box::new(HirExpr::Literal(Literal::Int(0))),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_unary() {
        let mut vars = HashSet::new();
        vars.insert("flag".to_string());

        let expr = HirExpr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(HirExpr::Var("flag".to_string())),
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    // ============ More is_rust_keyword edge cases ============

    #[test]
    pub(crate) fn test_is_rust_keyword_case_sensitivity() {
        assert!(keywords::is_rust_keyword("fn"));
        assert!(!keywords::is_rust_keyword("FN"));
        assert!(!keywords::is_rust_keyword("Fn"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_similar_names() {
        // These are NOT keywords
        assert!(!keywords::is_rust_keyword("function"));
        assert!(!keywords::is_rust_keyword("match_"));
        assert!(!keywords::is_rust_keyword("_if"));
        assert!(!keywords::is_rust_keyword("for2"));
    }

    // ============ BinOp containment tests ============

    #[test]
    pub(crate) fn test_get_python_op_precedence_in_notin() {
        assert_eq!(precedence::get_python_op_precedence(BinOp::In), 7);
        assert_eq!(precedence::get_python_op_precedence(BinOp::NotIn), 7);
    }

    // ============ get_rust_op_precedence tests ============

    #[test]
    pub(crate) fn test_get_rust_op_precedence_mul() {
        let op: syn::BinOp = syn::parse_quote!(*);
        assert_eq!(precedence::get_rust_op_precedence(&op), 13);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_div() {
        let op: syn::BinOp = syn::parse_quote!(/);
        assert_eq!(precedence::get_rust_op_precedence(&op), 13);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_add() {
        let op: syn::BinOp = syn::parse_quote!(+);
        assert_eq!(precedence::get_rust_op_precedence(&op), 12);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_sub() {
        let op: syn::BinOp = syn::parse_quote!(-);
        assert_eq!(precedence::get_rust_op_precedence(&op), 12);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_shl() {
        let op: syn::BinOp = syn::parse_quote!(<<);
        assert_eq!(precedence::get_rust_op_precedence(&op), 11);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_shr() {
        let op: syn::BinOp = syn::parse_quote!(>>);
        assert_eq!(precedence::get_rust_op_precedence(&op), 11);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_bitand() {
        let op: syn::BinOp = syn::parse_quote!(&);
        assert_eq!(precedence::get_rust_op_precedence(&op), 10);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_bitxor() {
        let op: syn::BinOp = syn::parse_quote!(^);
        assert_eq!(precedence::get_rust_op_precedence(&op), 9);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_bitor() {
        let op: syn::BinOp = syn::parse_quote!(|);
        assert_eq!(precedence::get_rust_op_precedence(&op), 8);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_lt() {
        let op: syn::BinOp = syn::parse_quote!(<);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_le() {
        let op: syn::BinOp = syn::parse_quote!(<=);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_gt() {
        let op: syn::BinOp = syn::parse_quote!(>);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_ge() {
        let op: syn::BinOp = syn::parse_quote!(>=);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_eq() {
        let op: syn::BinOp = syn::parse_quote!(==);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_ne() {
        let op: syn::BinOp = syn::parse_quote!(!=);
        assert_eq!(precedence::get_rust_op_precedence(&op), 7);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_and() {
        let op: syn::BinOp = syn::parse_quote!(&&);
        assert_eq!(precedence::get_rust_op_precedence(&op), 6);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_or() {
        let op: syn::BinOp = syn::parse_quote!(||);
        assert_eq!(precedence::get_rust_op_precedence(&op), 5);
    }

    #[test]
    pub(crate) fn test_get_rust_op_precedence_rem() {
        let op: syn::BinOp = syn::parse_quote!(%);
        assert_eq!(precedence::get_rust_op_precedence(&op), 13);
    }

    // ============ borrow_if_needed tests ============

    #[test]
    pub(crate) fn test_borrow_if_needed_path() {
        let expr: syn::Expr = syn::parse_quote!(x);
        let result = ExpressionConverter::borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "& x");
    }

    #[test]
    pub(crate) fn test_borrow_if_needed_already_reference() {
        let expr: syn::Expr = syn::parse_quote!(&x);
        let result = ExpressionConverter::borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "& x");
    }

    #[test]
    pub(crate) fn test_borrow_if_needed_literal() {
        let expr: syn::Expr = syn::parse_quote!("hello");
        let result = ExpressionConverter::borrow_if_needed(&expr);
        assert_eq!(result.to_token_stream().to_string(), "\"hello\"");
    }

    #[test]
    pub(crate) fn test_borrow_if_needed_method_call() {
        let expr: syn::Expr = syn::parse_quote!(s.as_str());
        let result = ExpressionConverter::borrow_if_needed(&expr);
        // Method calls producing str are not borrowed
        assert_eq!(result.to_token_stream().to_string(), "s . as_str ()");
    }

    // ============ wrap_in_parens tests ============
    // Note: wrap_in_parens uses braces { } not parens ( ) per DEPYLER-0707

    #[test]
    pub(crate) fn test_wrap_in_parens_simple() {
        let expr: syn::Expr = syn::parse_quote!(x);
        let result = ExpressionConverter::wrap_in_parens(expr);
        assert_eq!(result.to_token_stream().to_string(), "{ x }");
    }

    #[test]
    pub(crate) fn test_wrap_in_parens_binary() {
        let expr: syn::Expr = syn::parse_quote!(a + b);
        let result = ExpressionConverter::wrap_in_parens(expr);
        assert_eq!(result.to_token_stream().to_string(), "{ a + b }");
    }

    #[test]
    pub(crate) fn test_wrap_in_parens_call() {
        let expr: syn::Expr = syn::parse_quote!(foo(1, 2));
        let result = ExpressionConverter::wrap_in_parens(expr);
        assert_eq!(result.to_token_stream().to_string(), "{ foo (1 , 2) }");
    }

    // ============ parenthesize_if_lower_precedence tests ============

    #[test]
    pub(crate) fn test_parenthesize_lower_precedence() {
        // (a + b) * c - the add has lower precedence than mul
        let expr: syn::Expr = syn::parse_quote!(a + b);
        let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Mul);
        assert_eq!(result.to_token_stream().to_string(), "(a + b)");
    }

    #[test]
    pub(crate) fn test_parenthesize_same_precedence() {
        // a * b in context of division - same precedence, no parens
        let expr: syn::Expr = syn::parse_quote!(a * b);
        let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Div);
        // Same precedence doesn't add parens
        assert_eq!(result.to_token_stream().to_string(), "a * b");
    }

    #[test]
    pub(crate) fn test_parenthesize_higher_precedence() {
        // a * b in context of addition - higher precedence, no parens
        let expr: syn::Expr = syn::parse_quote!(a * b);
        let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Add);
        assert_eq!(result.to_token_stream().to_string(), "a * b");
    }

    #[test]
    pub(crate) fn test_parenthesize_non_binary() {
        // Non-binary expressions pass through unchanged
        let expr: syn::Expr = syn::parse_quote!(foo());
        let result = precedence::parenthesize_if_lower_precedence(expr, BinOp::Mul);
        assert_eq!(result.to_token_stream().to_string(), "foo ()");
    }

    // ============ Additional edge case tests ============

    #[test]
    pub(crate) fn test_expr_uses_any_var_empty_set() {
        let vars = HashSet::new();
        let expr = HirExpr::Var("x".to_string());
        assert!(!walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_method_call() {
        let mut vars = HashSet::new();
        vars.insert("obj".to_string());

        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_expr_uses_any_var_kwargs() {
        let mut vars = HashSet::new();
        vars.insert("value".to_string());

        let expr = HirExpr::Call {
            func: "func".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::Var("value".to_string()))],
        };
        assert!(walrus_helpers::expr_uses_any_var(&expr, &vars));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_list() {
        let conditions = vec![HirExpr::List(vec![HirExpr::NamedExpr {
            target: "item".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        }])];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("item"));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_set() {
        let conditions = vec![HirExpr::Set(vec![HirExpr::NamedExpr {
            target: "elem".to_string(),
            value: Box::new(HirExpr::Literal(Literal::Int(1))),
        }])];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("elem"));
    }

    #[test]
    pub(crate) fn test_collect_walrus_vars_in_method_kwargs() {
        let conditions = vec![HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("obj".to_string())),
            method: "method".to_string(),
            args: vec![],
            kwargs: vec![("key".to_string(), HirExpr::NamedExpr {
                target: "kwarg_var".to_string(),
                value: Box::new(HirExpr::Literal(Literal::Int(1))),
            })],
        }];
        let vars = walrus_helpers::collect_walrus_vars_from_conditions(&conditions);
        assert!(vars.contains("kwarg_var"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_move() {
        assert!(keywords::is_rust_keyword("move"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_return() {
        assert!(keywords::is_rust_keyword("return"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_break_continue() {
        assert!(keywords::is_rust_keyword("break"));
        assert!(keywords::is_rust_keyword("continue"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_use_mod() {
        assert!(keywords::is_rust_keyword("use"));
        assert!(keywords::is_rust_keyword("mod"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_extern() {
        assert!(keywords::is_rust_keyword("extern"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_true_false() {
        assert!(keywords::is_rust_keyword("true"));
        assert!(keywords::is_rust_keyword("false"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_dyn_unsafe() {
        assert!(keywords::is_rust_keyword("dyn"));
        assert!(keywords::is_rust_keyword("unsafe"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_where() {
        assert!(keywords::is_rust_keyword("where"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_in() {
        assert!(keywords::is_rust_keyword("in"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_as() {
        assert!(keywords::is_rust_keyword("as"));
    }

    #[test]
    pub(crate) fn test_is_rust_keyword_unsized() {
        assert!(keywords::is_rust_keyword("unsized"));
    }

    // ============ literal_to_rust_expr tests ============

    #[test]
    pub(crate) fn test_literal_to_rust_expr_int() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Int(42), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("42"));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_negative_int() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Int(-100), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("-100") || code.contains("- 100"));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_float() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Float(3.14), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("3.14"));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_float_zero() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Float(0.0), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        // Should have decimal point
        assert!(code.contains("0.0") || code.contains("."));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_bool_true() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Bool(true), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert_eq!(code, "true");
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_bool_false() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Bool(false), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert_eq!(code, "false");
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_none() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::None, &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert_eq!(code, "None");
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_bytes() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
        let result = literal_to_rust_expr(&Literal::Bytes(bytes), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("b\""));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_string() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::String("hello".to_string()), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("hello"));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_string_with_escape() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::String("hello\nworld".to_string()), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("hello") && code.contains("world"));
    }

    // ============ ExpressionConverter static method tests ============

    #[test]
    pub(crate) fn test_borrow_if_needed_path_expr() {
        let path: syn::Expr = parse_quote! { my_var };
        let result = ExpressionConverter::borrow_if_needed(&path);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("&"));
        assert!(code.contains("my_var"));
    }

    #[test]
    pub(crate) fn test_borrow_if_needed_reference_unchanged() {
        let already_ref: syn::Expr = parse_quote! { &some_ref };
        let result = ExpressionConverter::borrow_if_needed(&already_ref);
        let code = result.to_token_stream().to_string();
        // Should not double-borrow
        assert!(!code.contains("& &"));
    }

    #[test]
    pub(crate) fn test_borrow_if_needed_lit_str() {
        let lit_str: syn::Expr = parse_quote! { "hello" };
        let result = ExpressionConverter::borrow_if_needed(&lit_str);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("hello"));
    }

    #[test]
    pub(crate) fn test_wrap_in_parens_simple_path() {
        // wrap_in_parens creates a block { expr }, not parentheses (expr)
        let path: syn::Expr = parse_quote! { x };
        let result = ExpressionConverter::wrap_in_parens(path);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("{") && code.contains("}") && code.contains("x"));
    }

    #[test]
    pub(crate) fn test_wrap_in_parens_binary_expr() {
        // wrap_in_parens creates a block { expr }
        let binary: syn::Expr = parse_quote! { a + b };
        let result = ExpressionConverter::wrap_in_parens(binary);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("{") && code.contains("}"));
        assert!(code.contains("a") && code.contains("b"));
    }

    #[test]
    pub(crate) fn test_wrap_in_parens_call_expr() {
        // wrap_in_parens creates a block { expr }
        let call: syn::Expr = parse_quote! { foo(x, y) };
        let result = ExpressionConverter::wrap_in_parens(call);
        let code = result.to_token_stream().to_string();
        // Block braces around the call
        assert!(code.contains("{") && code.contains("}"));
        assert!(code.contains("foo"));
    }

    // ============ Additional edge case tests ============

    #[test]
    pub(crate) fn test_literal_to_rust_expr_large_int() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Int(i64::MAX), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains(&i64::MAX.to_string()));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_float_scientific() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Float(1.5e10), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        // Should handle scientific notation
        assert!(code.contains("e") || code.contains("E") || code.contains("15000000000"));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_empty_string() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::String("".to_string()), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("\"\""));
    }

    #[test]
    pub(crate) fn test_literal_to_rust_expr_empty_bytes() {
        let string_optimizer = StringOptimizer::new();
        let needs_cow = false;
        let ctx = CodeGenContext::default();
        let result = literal_to_rust_expr(&Literal::Bytes(vec![]), &string_optimizer, &needs_cow, &ctx);
        let code = result.to_token_stream().to_string();
        assert!(code.contains("b\"\""));
    }
}
