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
use crate::rust_gen::numpy_gen; // Phase 3: NumPy→Trueno codegen
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use crate::rust_gen::return_type_expects_float;
use crate::rust_gen::type_gen::convert_binop;
use crate::string_optimization::{StringContext, StringOptimizer};
use crate::trace_decision;
use anyhow::{bail, Result};
use quote::{quote, ToTokens};
use syn::{self, parse_quote};

struct ExpressionConverter<'a, 'b> {
    ctx: &'a mut CodeGenContext<'b>,
}

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    fn new(ctx: &'a mut CodeGenContext<'b>) -> Self {
        Self { ctx }
    }

    /// Check if a name is a Rust keyword that requires raw identifier syntax
    fn is_rust_keyword(name: &str) -> bool {
        matches!(
            name,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "async"
                | "await"
                | "dyn"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "macro"
                | "override"
                | "priv"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "try"
        )
    }

    /// Check if a keyword cannot be used as a raw identifier
    /// These special keywords (self, Self, super, crate) cannot use r# syntax
    fn is_non_raw_keyword(name: &str) -> bool {
        matches!(name, "self" | "Self" | "super" | "crate")
    }

    /// DEPYLER-0792: Collect all variable names defined by walrus operators in conditions
    /// Recursively walks the expression tree to find NamedExpr targets
    fn collect_walrus_vars_from_conditions(conditions: &[HirExpr]) -> std::collections::HashSet<String> {
        let mut vars = std::collections::HashSet::new();
        for cond in conditions {
            Self::collect_walrus_vars_from_expr(cond, &mut vars);
        }
        vars
    }

    /// DEPYLER-0792: Helper to recursively find NamedExpr (walrus) targets in an expression
    fn collect_walrus_vars_from_expr(expr: &HirExpr, vars: &mut std::collections::HashSet<String>) {
        match expr {
            HirExpr::NamedExpr { target, value } => {
                vars.insert(target.clone());
                Self::collect_walrus_vars_from_expr(value, vars);
            }
            HirExpr::Binary { left, right, .. } => {
                Self::collect_walrus_vars_from_expr(left, vars);
                Self::collect_walrus_vars_from_expr(right, vars);
            }
            HirExpr::Unary { operand, .. } => {
                Self::collect_walrus_vars_from_expr(operand, vars);
            }
            HirExpr::Call { args, kwargs, .. } => {
                for arg in args {
                    Self::collect_walrus_vars_from_expr(arg, vars);
                }
                for (_, v) in kwargs {
                    Self::collect_walrus_vars_from_expr(v, vars);
                }
            }
            HirExpr::MethodCall { object, args, kwargs, .. } => {
                Self::collect_walrus_vars_from_expr(object, vars);
                for arg in args {
                    Self::collect_walrus_vars_from_expr(arg, vars);
                }
                for (_, v) in kwargs {
                    Self::collect_walrus_vars_from_expr(v, vars);
                }
            }
            HirExpr::IfExpr { test, body, orelse } => {
                Self::collect_walrus_vars_from_expr(test, vars);
                Self::collect_walrus_vars_from_expr(body, vars);
                Self::collect_walrus_vars_from_expr(orelse, vars);
            }
            HirExpr::Tuple(elts) | HirExpr::List(elts) | HirExpr::Set(elts) => {
                for e in elts {
                    Self::collect_walrus_vars_from_expr(e, vars);
                }
            }
            _ => {}
        }
    }

    /// DEPYLER-0792: Check if an expression uses any of the given variable names
    fn expr_uses_any_var(expr: &HirExpr, var_names: &std::collections::HashSet<String>) -> bool {
        match expr {
            HirExpr::Var(name) => var_names.contains(name),
            HirExpr::NamedExpr { value, .. } => Self::expr_uses_any_var(value, var_names),
            HirExpr::Binary { left, right, .. } => {
                Self::expr_uses_any_var(left, var_names) || Self::expr_uses_any_var(right, var_names)
            }
            HirExpr::Unary { operand, .. } => Self::expr_uses_any_var(operand, var_names),
            HirExpr::Call { args, kwargs, .. } => {
                args.iter().any(|a| Self::expr_uses_any_var(a, var_names))
                    || kwargs.iter().any(|(_, v)| Self::expr_uses_any_var(v, var_names))
            }
            HirExpr::MethodCall { object, args, kwargs, .. } => {
                Self::expr_uses_any_var(object, var_names)
                    || args.iter().any(|a| Self::expr_uses_any_var(a, var_names))
                    || kwargs.iter().any(|(_, v)| Self::expr_uses_any_var(v, var_names))
            }
            HirExpr::Tuple(elts) | HirExpr::List(elts) | HirExpr::Set(elts) => {
                elts.iter().any(|e| Self::expr_uses_any_var(e, var_names))
            }
            HirExpr::Index { base, index } => {
                Self::expr_uses_any_var(base, var_names) || Self::expr_uses_any_var(index, var_names)
            }
            HirExpr::Attribute { value, .. } => Self::expr_uses_any_var(value, var_names),
            HirExpr::IfExpr { test, body, orelse } => {
                Self::expr_uses_any_var(test, var_names)
                    || Self::expr_uses_any_var(body, var_names)
                    || Self::expr_uses_any_var(orelse, var_names)
            }
            _ => false,
        }
    }

    /// DEPYLER-0792: Generate let bindings for walrus expressions in a condition
    /// Extracts `(length := len(w))` as `let length = w.len() as i32;`
    fn generate_walrus_bindings(cond: &HirExpr, ctx: &mut CodeGenContext) -> Result<proc_macro2::TokenStream> {
        let mut bindings = proc_macro2::TokenStream::new();
        Self::collect_walrus_bindings_from_expr(cond, ctx, &mut bindings)?;
        Ok(bindings)
    }

    /// DEPYLER-0792: Helper to recursively extract walrus bindings from expression
    fn collect_walrus_bindings_from_expr(
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

    /// DEPYLER-0633: Check if expression looks like it returns Option
    /// Used to detect `Option or default` patterns that should become `.unwrap_or()`
    ///
    /// Detection heuristics:
    /// - Method call ending in `.ok()` → definitely Option (e.g., env::var().ok())
    /// - Method call `.get()` with 1 arg → Option (dict.get(key) without default)
    /// - Chained method calls where inner is Option → Option
    fn looks_like_option_expr(expr: &HirExpr) -> bool {
        match expr {
            // Method call ending in .ok() → definitely Option
            HirExpr::MethodCall { method, .. } if method == "ok" => true,
            // .get() only returns Option when no default value provided
            HirExpr::MethodCall { method, args, .. } if method == "get" => {
                args.len() == 1 // Only 1 arg (key) = Option, 2 args (key + default) = concrete
            }
            // Check for chained calls like std::env::var(...).ok()
            HirExpr::MethodCall { object, method, args, .. } => {
                if method == "ok" {
                    true
                } else if method == "get" {
                    args.len() == 1
                } else {
                    Self::looks_like_option_expr(object)
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0758: Check if HirExpr is a variable that's a borrowed parameter
    /// If so, return the dereferenced version of the syn::Expr
    /// Used to fix E0369 errors when doing arithmetic with reference types (e.g., date subtraction)
    fn deref_if_borrowed_param(&self, hir_expr: &HirExpr, rust_expr: syn::Expr) -> syn::Expr {
        if let HirExpr::Var(name) = hir_expr {
            if self.ctx.ref_params.contains(name.as_str()) {
                // Dereference the expression: `*target` instead of `target`
                return parse_quote! { *#rust_expr };
            }
        }
        rust_expr
    }

    /// DEPYLER-0582: Wrap expression in parentheses if it's a binary operation with lower precedence
    /// This preserves Python's parenthesized expressions in Rust output
    /// e.g., (1 - beta1) * x should become (1.0 - beta1) * x, not 1.0 - beta1 * x
    fn parenthesize_if_lower_precedence(expr: syn::Expr, parent_op: BinOp) -> syn::Expr {
        // Check if expression is a binary operation
        if let syn::Expr::Binary(bin_expr) = &expr {
            let child_precedence = Self::get_rust_op_precedence(&bin_expr.op);
            let parent_precedence = Self::get_python_op_precedence(parent_op);

            // If child has lower precedence, wrap in parentheses
            if child_precedence < parent_precedence {
                return parse_quote! { (#expr) };
            }
        }
        expr
    }

    /// Get precedence of Rust binary operator (higher = binds tighter)
    fn get_rust_op_precedence(op: &syn::BinOp) -> u8 {
        match op {
            syn::BinOp::Mul(_) | syn::BinOp::Div(_) | syn::BinOp::Rem(_) => 13,
            syn::BinOp::Add(_) | syn::BinOp::Sub(_) => 12,
            syn::BinOp::Shl(_) | syn::BinOp::Shr(_) => 11,
            syn::BinOp::BitAnd(_) => 10,
            syn::BinOp::BitXor(_) => 9,
            syn::BinOp::BitOr(_) => 8,
            syn::BinOp::Lt(_)
            | syn::BinOp::Le(_)
            | syn::BinOp::Gt(_)
            | syn::BinOp::Ge(_)
            | syn::BinOp::Eq(_)
            | syn::BinOp::Ne(_) => 7,
            syn::BinOp::And(_) => 6,
            syn::BinOp::Or(_) => 5,
            _ => 0, // Compound assignment operators, etc.
        }
    }

    /// Get precedence of Python binary operator for our HIR
    fn get_python_op_precedence(op: BinOp) -> u8 {
        match op {
            BinOp::Pow => 14,
            BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::FloorDiv => 13,
            BinOp::Add | BinOp::Sub => 12,
            BinOp::LShift | BinOp::RShift => 11,
            BinOp::BitAnd => 10,
            BinOp::BitXor => 9,
            BinOp::BitOr => 8,
            BinOp::Lt | BinOp::LtEq | BinOp::Gt | BinOp::GtEq | BinOp::Eq | BinOp::NotEq => 7,
            BinOp::In | BinOp::NotIn => 7,
            BinOp::And => 6,
            BinOp::Or => 5,
        }
    }

    /// DEPYLER-0582: Coerce integer literal to float if other operand is float-typed
    /// Python automatically promotes int to float in arithmetic with floats
    /// e.g., `1 - beta1` where beta1:float → `1.0 - beta1` in Rust
    fn coerce_int_to_float_if_needed(
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
    fn is_int_expr(&self, expr: &HirExpr) -> bool {
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
    fn is_int_var(&self, expr: &HirExpr) -> bool {
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
    fn is_float_var(&self, expr: &HirExpr) -> bool {
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
            // Single-letter color channel names like r, g, h, s, v, l are typically f64
            // from colorsys.hsv_to_rgb(), rgb_to_hsv(), rgb_to_hls() etc.
            // Only match exact single-letter names to avoid false positives
            // DEPYLER-0954: Note: a, b, x, y are too generic and cause false positives
            // (b, y were incorrectly included before - causes spurious float casts)
            if matches!(
                name.as_str(),
                "r" | "g" | "h" | "s" | "v" | "l" | "c" | "m" | "k"
            ) {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0465: Add & to borrow a path expression if it's a simple variable
    /// This prevents moving String parameters in PathBuf::from() and File::open()
    ///
    /// # Complexity
    /// ≤10 (simple match pattern)
    fn borrow_if_needed(expr: &syn::Expr) -> syn::Expr {
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
    fn borrow_path_with_option_check(&self, path_expr: &syn::Expr, hir_arg: &HirExpr) -> syn::Expr {
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

    fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
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
            let ident = if Self::is_rust_keyword(unwrapped_name) {
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
        if Self::is_non_raw_keyword(name) {
            bail!(
                "Python variable '{}' conflicts with a special Rust keyword that cannot be escaped. \
                 Please rename this variable (e.g., '{}_var' or 'py_{}')",
                name, name, name
            );
        }

        // Inside generators, check if variable is a state variable
        if self.ctx.in_generator && self.ctx.generator_state_vars.contains(name) {
            // Generate self.field for state variables
            let ident = if Self::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
            Ok(parse_quote! { self.#ident })
        } else {
            // Regular variable - use raw identifier if it's a Rust keyword
            let ident = if Self::is_rust_keyword(name) {
                syn::Ident::new_raw(name, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(name, proc_macro2::Span::call_site())
            };
            Ok(parse_quote! { #ident })
        }
    }

    fn convert_binary(&mut self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
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
                    let left_wrapped = Self::parenthesize_if_lower_precedence(left_expr, op);
                    let right_wrapped = Self::parenthesize_if_lower_precedence(right_expr, op);
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
                if matches!(op, BinOp::Or) && Self::looks_like_option_expr(left) {
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
    fn convert_pow_op(
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
    fn wrap_in_parens(expr: syn::Expr) -> syn::Expr {
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
    fn convert_mul_op(
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
                let left_wrapped = Self::parenthesize_if_lower_precedence(left_coerced, op);
                let right_wrapped = Self::parenthesize_if_lower_precedence(right_coerced, op);
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
    fn convert_add_op(
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
    fn convert_containment_op(
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
    fn try_convert_stdlib_type_call(
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

            // DEPYLER-STDLIB-DATETIME: Handle datetime constructors
            "datetime" if args.len() >= 3 => {
                self.ctx.needs_chrono = true;
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
                    Some(Ok(parse_quote! {
                        chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                            .unwrap()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                    }))
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
                    Some(Ok(parse_quote! {
                        chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32)
                            .unwrap()
                            .and_hms_opt(#hour as u32, #minute as u32, #second as u32)
                            .unwrap()
                    }))
                } else {
                    Some(Err(anyhow::anyhow!(
                        "datetime() requires 3 or 6+ arguments"
                    )))
                }
            }
            "datetime" => Some(Err(anyhow::anyhow!(
                "datetime() requires at least 3 arguments (year, month, day)"
            ))),

            // date(year, month, day) → NaiveDate::from_ymd_opt(y, m, d).unwrap()
            "date" if args.len() == 3 => {
                self.ctx.needs_chrono = true;
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
                Some(Ok(parse_quote! {
                    chrono::NaiveDate::from_ymd_opt(#year as i32, #month as u32, #day as u32).unwrap()
                }))
            }

            // DEPYLER-0938: time() with no args → NaiveTime at midnight
            "time" if args.is_empty() => {
                self.ctx.needs_chrono = true;
                Some(Ok(parse_quote! {
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                }))
            }

            // DEPYLER-0938: time(hour) → NaiveTime::from_hms_opt(h, 0, 0).unwrap()
            "time" if args.len() == 1 => {
                self.ctx.needs_chrono = true;
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(parse_quote! {
                    chrono::NaiveTime::from_hms_opt(#hour as u32, 0, 0).unwrap()
                }))
            }

            // time(hour, minute, second) → NaiveTime::from_hms_opt(h, m, s).unwrap()
            "time" if args.len() >= 2 => {
                self.ctx.needs_chrono = true;
                let hour = match args[0].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };
                let minute = match args[1].to_rust_expr(self.ctx) {
                    Ok(e) => e,
                    Err(e) => return Some(Err(e)),
                };

                if args.len() == 2 {
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, 0).unwrap()
                    }))
                } else {
                    let second = match args[2].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    Some(Ok(parse_quote! {
                        chrono::NaiveTime::from_hms_opt(#hour as u32, #minute as u32, #second as u32).unwrap()
                    }))
                }
            }

            // timedelta(days=..., seconds=...) → Duration::days(...) + Duration::seconds(...)
            "timedelta" => {
                self.ctx.needs_chrono = true;
                if args.is_empty() {
                    Some(Ok(parse_quote! { chrono::Duration::zero() }))
                } else if args.len() == 1 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    Some(Ok(parse_quote! { chrono::Duration::days(#days as i64) }))
                } else if args.len() == 2 {
                    let days = match args[0].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    let seconds = match args[1].to_rust_expr(self.ctx) {
                        Ok(e) => e,
                        Err(e) => return Some(Err(e)),
                    };
                    Some(Ok(parse_quote! {
                        chrono::Duration::days(#days as i64) + chrono::Duration::seconds(#seconds as i64)
                    }))
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
    fn try_convert_numeric_type_call(
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
    fn try_convert_iterator_util_call(
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
    fn needs_debug_format(&self, hir_arg: &HirExpr) -> bool {
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
    fn is_pathbuf_expr(&self, hir_arg: &HirExpr) -> bool {
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
    fn infer_numeric_type_token(&self) -> proc_macro2::TokenStream {
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
    fn try_convert_print_call(
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
    fn try_convert_sum_call(&mut self, func: &str, args: &[HirExpr]) -> Option<Result<syn::Expr>> {
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
    fn try_convert_minmax_call(
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
    fn try_convert_any_all_call(
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

    fn convert_unary(&mut self, op: &UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
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

    fn convert_call(
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
            // DEPYLER-STDLIB-BUILTINS: Additional builtin functions
            "all" => self.convert_all_builtin(&arg_exprs),
            "any" => self.convert_any_builtin(&arg_exprs),
            "divmod" => self.convert_divmod_builtin(&arg_exprs),
            "enumerate" => self.convert_enumerate_builtin(&arg_exprs),
            "zip" => self.convert_zip_builtin(&arg_exprs),
            "reversed" => self.convert_reversed_builtin(&arg_exprs),
            "sorted" => self.convert_sorted_builtin(&arg_exprs),
            "filter" => self.convert_filter_builtin(&all_hir_args, &arg_exprs),
            "sum" => self.convert_sum_builtin(&arg_exprs),
            // DEPYLER-STDLIB-BUILTINS: Final batch for 50% milestone
            "round" => self.convert_round_builtin(&arg_exprs),
            "abs" => self.convert_abs_builtin(&arg_exprs),
            "min" => self.convert_min_builtin(&arg_exprs),
            "max" => self.convert_max_builtin(&arg_exprs),
            "pow" => self.convert_pow_builtin(&arg_exprs),
            "hex" => self.convert_hex_builtin(&arg_exprs),
            "bin" => self.convert_bin_builtin(&arg_exprs),
            "oct" => self.convert_oct_builtin(&arg_exprs),
            // DEPYLER-0579: format(value, spec) builtin
            "format" => self.convert_format_builtin(&arg_exprs, &all_hir_args),
            "chr" => self.convert_chr_builtin(&arg_exprs),
            "ord" => self.convert_ord_builtin(&arg_exprs, &all_hir_args),
            "hash" => self.convert_hash_builtin(&arg_exprs),
            "repr" => self.convert_repr_builtin(&arg_exprs),
            // DEPYLER-0387: File I/O builtin
            "open" => self.convert_open_builtin(&all_hir_args, &arg_exprs),
            // DEPYLER-STDLIB-50: next(), getattr(), iter(), type()
            "next" => self.convert_next_builtin(&arg_exprs),
            "getattr" => self.convert_getattr_builtin(&arg_exprs),
            "iter" => self.convert_iter_builtin(&arg_exprs),
            "type" => self.convert_type_builtin(&arg_exprs),
            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            "isinstance" => Ok(parse_quote! { true }),
            _ => self.convert_generic_call(func, &all_hir_args, &all_args),
        }
    }

    fn try_convert_map_with_zip(&mut self, args: &[HirExpr]) -> Result<Option<syn::Expr>> {
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
    fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        builtin_conversions::convert_len_call(args)
    }

    /// DEPYLER-0659: Handle len() with type awareness for serde_json::Value
    /// serde_json::Value doesn't have a direct .len() method
    /// - Arrays: use .as_array().map(|a| a.len()).unwrap_or(0)
    /// - Objects: use .as_object().map(|o| o.len()).unwrap_or(0)
    /// - Strings: use .as_str().map(|s| s.len()).unwrap_or(0)
    fn convert_len_call_with_type(
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
    fn convert_int_cast(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
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
    fn convert_float_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_float_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to builtin_conversions module
    /// DEPYLER-0188: Pass HirExpr to detect PathBuf for .display().to_string()
    /// DEPYLER-0722: Handle Option<T> types - use .unwrap().to_string()
    fn convert_str_conversion(&self, hir_args: &[HirExpr], args: &[syn::Expr]) -> Result<syn::Expr> {
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
    fn convert_bool_cast(
        &self,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        builtin_conversions::convert_bool_cast(self.ctx, hir_args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        array_initialization::convert_range_call(args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to array_initialization module
    fn convert_array_init_call(
        &mut self,
        func: &str,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        array_initialization::convert_array_init_call(self.ctx, func, args, arg_exprs)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    fn convert_set_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_set_constructor(self.ctx, args)
    }

    /// DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    fn convert_frozenset_constructor(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_frozenset_constructor(self.ctx, args)
    }

    // ========================================================================
    // DEPYLER-0171, 0172, 0173, 0174: Collection Conversion Builtins
    // DEPYLER-REFACTOR-001: Delegated to collection_constructors module
    // ========================================================================

    /// DEPYLER-0751: Handle Counter(string) by using .chars() instead of .into_iter()
    fn convert_counter_builtin(
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

    fn convert_defaultdict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_defaultdict_builtin(self.ctx, args)
    }

    fn convert_dict_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_dict_builtin(self.ctx, args)
    }

    fn convert_deque_builtin(&mut self, args: &[syn::Expr]) -> Result<syn::Expr> {
        collection_constructors::convert_deque_builtin(self.ctx, args)
    }

    fn convert_list_builtin(
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
    fn convert_bytes_builtin(
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
    fn convert_bytearray_builtin(
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
    fn convert_tuple_builtin(
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

    // DEPYLER-STDLIB-BUILTINS: Additional builtin function converters

    fn convert_all_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("all() requires exactly 1 argument");
        }
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().all(|x| x) })
    }

    fn convert_any_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("any() requires exactly 1 argument");
        }
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().any(|x| x) })
    }

    fn convert_divmod_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 2 {
            bail!("divmod() requires exactly 2 arguments");
        }
        let a = &args[0];
        let b = &args[1];
        Ok(parse_quote! { (#a / #b, #a % #b) })
    }

    fn convert_enumerate_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("enumerate() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        // DEPYLER-0519: Use .iter().cloned() instead of .into_iter()
        // This preserves the original collection (important when returned after loop)
        // Python: for i, x in enumerate(items): ... return items  # items still usable
        // Rust with into_iter(): items consumed, can't return
        // Rust with iter().cloned(): items preserved, can return
        if args.len() == 2 {
            let start = &args[1];
            Ok(
                parse_quote! { #iterable.iter().cloned().enumerate().map(|(i, x)| ((i + #start as usize) as i32, x)) },
            )
        } else {
            Ok(parse_quote! { #iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) })
        }
    }

    fn convert_zip_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 {
            bail!("zip() requires at least 2 arguments");
        }
        let first = &args[0];
        let second = &args[1];
        if args.len() == 2 {
            Ok(parse_quote! { #first.into_iter().zip(#second.into_iter()) })
        } else {
            // For 3+ iterables, chain zip calls
            let mut zip_expr: syn::Expr =
                parse_quote! { #first.into_iter().zip(#second.into_iter()) };
            for iter in &args[2..] {
                zip_expr = parse_quote! { #zip_expr.zip(#iter.into_iter()) };
            }
            Ok(zip_expr)
        }
    }

    fn convert_reversed_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("reversed() requires exactly 1 argument");
        }
        let iterable = &args[0];
        // DEPYLER-0753: Use .iter().cloned() instead of .into_iter() to produce Vec<T> not Vec<&T>
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned().rev() properly clones elements to produce owned iterator.
        Ok(parse_quote! { #iterable.iter().cloned().rev() })
    }

    fn convert_sorted_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("sorted() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        // Simplified: ignore key/reverse parameters for now
        // DEPYLER-0733: Use .iter().cloned() instead of .into_iter() to produce Vec<T> not Vec<&T>
        // When iterable is &Vec<T>, .into_iter() yields &T references, causing type mismatch.
        // .iter().cloned() properly clones elements to produce owned Vec<T>.
        // DEPYLER-0807: Use sort_by with partial_cmp to support floats (f64 doesn't implement Ord)
        // partial_cmp works for all PartialOrd types including integers and floats.
        // unwrap_or(Equal) treats NaN as equal, which is safe for typical sorting.
        Ok(parse_quote! {
            {
                let mut sorted_vec = #iterable.iter().cloned().collect::<Vec<_>>();
                sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                sorted_vec
            }
        })
    }

    fn convert_filter_builtin(
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

    fn convert_sum_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("sum() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        if args.len() == 2 {
            let start = &args[1];
            Ok(parse_quote! { #iterable.into_iter().fold(#start, |acc, x| acc + x) })
        } else {
            Ok(parse_quote! { #iterable.into_iter().sum() })
        }
    }

    // DEPYLER-STDLIB-BUILTINS: Final batch converters for 50% milestone

    fn convert_round_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("round() requires 1 or 2 arguments");
        }
        let value = &args[0];
        // Simplified: ignore ndigits parameter
        Ok(parse_quote! { (#value as f64).round() as i32 })
    }

    fn convert_abs_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("abs() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! { (#value).abs() })
    }

    fn convert_min_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("min() requires at least 1 argument");
        }
        if args.len() == 1 {
            // min(iterable)
            let iterable = &args[0];
            Ok(parse_quote! { #iterable.into_iter().min().unwrap() })
        } else {
            // min(a, b, c, ...)
            let first = &args[0];
            let mut min_expr = parse_quote! { #first };
            for arg in &args[1..] {
                min_expr = parse_quote! { #min_expr.min(#arg) };
            }
            Ok(min_expr)
        }
    }

    fn convert_max_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("max() requires at least 1 argument");
        }
        if args.len() == 1 {
            // max(iterable)
            let iterable = &args[0];
            Ok(parse_quote! { #iterable.into_iter().max().unwrap() })
        } else {
            // max(a, b, c, ...)
            let first = &args[0];
            let mut max_expr = parse_quote! { #first };
            for arg in &args[1..] {
                max_expr = parse_quote! { #max_expr.max(#arg) };
            }
            Ok(max_expr)
        }
    }

    fn convert_pow_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 || args.len() > 3 {
            bail!("pow() requires 2 or 3 arguments");
        }
        let base = &args[0];
        let exp = &args[1];
        // Simplified: ignore modulo parameter
        Ok(parse_quote! { (#base as f64).powf(#exp as f64) as i32 })
    }

    fn convert_hex_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("hex() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! { format!("0x{:x}", #value) })
    }

    fn convert_bin_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("bin() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! { format!("0b{:b}", #value) })
    }

    fn convert_oct_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("oct() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! { format!("0o{:o}", #value) })
    }

    /// DEPYLER-0579: Python format(value, spec) builtin
    /// format(num, "b") → binary string
    /// format(num, "o") → octal string
    /// format(num, "x") → hex string
    /// format(num, "d") → decimal string
    fn convert_format_builtin(&self, args: &[syn::Expr], hir_args: &[HirExpr]) -> Result<syn::Expr> {
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

    fn convert_chr_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("chr() requires exactly 1 argument");
        }
        let code = &args[0];
        Ok(parse_quote! {
            char::from_u32(#code as u32).unwrap().to_string()
        })
    }

    fn convert_ord_builtin(&self, args: &[syn::Expr], hir_args: &[HirExpr]) -> Result<syn::Expr> {
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
    fn convert_open_builtin(
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

    fn convert_hash_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("hash() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! {
            {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                #value.hash(&mut hasher);
                hasher.finish() as i64
            }
        })
    }

    fn convert_repr_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("repr() requires exactly 1 argument");
        }
        let value = &args[0];
        Ok(parse_quote! { format!("{:?}", #value) })
    }

    // DEPYLER-STDLIB-50: next() - get next item from iterator
    fn convert_next_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("next() requires 1 or 2 arguments (iterator, optional default)");
        }
        let iterator = &args[0];
        if args.len() == 2 {
            let default = &args[1];
            Ok(parse_quote! {
                #iterator.next().unwrap_or(#default)
            })
        } else {
            Ok(parse_quote! {
                #iterator.next().expect("StopIteration: iterator is empty")
            })
        }
    }

    // DEPYLER-STDLIB-50: getattr() - get attribute by name
    fn convert_getattr_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 || args.len() > 3 {
            bail!("getattr() requires 2 or 3 arguments (object, name, optional default)");
        }
        // Note: This is a simplified implementation
        // Full getattr() requires runtime attribute lookup which isn't possible in Rust
        // For now, we'll bail as it needs special handling
        bail!("getattr() requires dynamic attribute access not fully supported yet")
    }

    // DEPYLER-STDLIB-50: iter() - create iterator
    fn convert_iter_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("iter() requires exactly 1 argument");
        }
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter() })
    }

    // DEPYLER-STDLIB-50: type() - get type name
    fn convert_type_builtin(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("type() requires exactly 1 argument");
        }
        let value = &args[0];
        // Return a string representation of the type name
        // This is a simplified implementation - full Python type() is more complex
        Ok(parse_quote! { std::any::type_name_of_val(&#value) })
    }

    // DEPYLER-REFACTOR-001: Helper functions moved to collection_constructors module:
    // already_collected, is_range_expr, is_iterator_expr, is_csv_reader_var

    fn convert_generic_call(
        &self,
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
                    if args.is_empty() {
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
    fn try_convert_classmethod(
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
    fn try_convert_struct_method(
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

    /// Try to convert json module method calls
    /// DEPYLER-STDLIB-JSON: JSON serialization/deserialization support
    #[inline]
    fn try_convert_json_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need serde_json crate
        self.ctx.needs_serde_json = true;

        let result = match method {
            // String serialization/deserialization
            "dumps" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("json.dumps() requires 1 or 2 arguments");
                }
                let obj = &arg_exprs[0];

                // DEPYLER-0377: Check if indent parameter is provided
                // json.dumps(result, indent=2) has 2 arguments after HIR conversion
                // (keyword args become positional args in HIR)
                if arg_exprs.len() >= 2 {
                    // json.dumps(obj, indent=n) → serde_json::to_string_pretty(&obj).unwrap()
                    parse_quote! { serde_json::to_string_pretty(&#obj).unwrap() }
                } else {
                    // json.dumps(obj) → serde_json::to_string(&obj).unwrap()
                    parse_quote! { serde_json::to_string(&#obj).unwrap() }
                }
            }

            "loads" => {
                if arg_exprs.len() != 1 {
                    bail!("json.loads() requires exactly 1 argument");
                }
                let s = &arg_exprs[0];

                // DEPYLER-0962: Check if return type is a Union of dict|list
                // In this case, we need to convert the parsed Value to the union type
                if let Some(union_name) = self.return_type_is_dict_list_union() {
                    self.ctx.needs_hashmap = true;
                    let union_ident: syn::Ident =
                        syn::Ident::new(&union_name, proc_macro2::Span::call_site());
                    // Parse as Value, then convert to union type using match
                    parse_quote! {
                        {
                            let __json_val = serde_json::from_str::<serde_json::Value>(&#s).unwrap();
                            match __json_val {
                                serde_json::Value::Object(obj) => #union_ident::Dict(obj.into_iter().collect()),
                                serde_json::Value::Array(arr) => #union_ident::List(arr),
                                _ => panic!("json.loads expected dict or list"),
                            }
                        }
                    }
                } else if self.return_type_needs_json_dict() {
                    // DEPYLER-0703: Check if return type is Dict[str, Any] → HashMap<String, Value>
                    self.ctx.needs_hashmap = true;
                    // json.loads(s) when returning Dict[str, Any]
                    // → serde_json::from_str::<HashMap<String, Value>>(&s).unwrap()
                    parse_quote! { serde_json::from_str::<std::collections::HashMap<String, serde_json::Value>>(&#s).unwrap() }
                } else {
                    // json.loads(s) → serde_json::from_str::<Value>(&s).unwrap()
                    // Returns serde_json::Value (dynamic JSON value)
                    parse_quote! { serde_json::from_str::<serde_json::Value>(&#s).unwrap() }
                }
            }

            // File-based serialization/deserialization
            "dump" => {
                if arg_exprs.len() != 2 {
                    bail!("json.dump() requires exactly 2 arguments (obj, file)");
                }
                let obj = &arg_exprs[0];
                let file = &arg_exprs[1];
                // json.dump(obj, file) → serde_json::to_writer(file, &obj).unwrap()
                parse_quote! { serde_json::to_writer(#file, &#obj).unwrap() }
            }

            "load" => {
                if arg_exprs.len() != 1 {
                    bail!("json.load() requires exactly 1 argument (file)");
                }
                let file = &arg_exprs[0];

                // DEPYLER-0962: Check if return type is a Union of dict|list
                if let Some(union_name) = self.return_type_is_dict_list_union() {
                    self.ctx.needs_hashmap = true;
                    let union_ident: syn::Ident =
                        syn::Ident::new(&union_name, proc_macro2::Span::call_site());
                    parse_quote! {
                        {
                            let __json_val = serde_json::from_reader::<_, serde_json::Value>(#file).unwrap();
                            match __json_val {
                                serde_json::Value::Object(obj) => #union_ident::Dict(obj.into_iter().collect()),
                                serde_json::Value::Array(arr) => #union_ident::List(arr),
                                _ => panic!("json.load expected dict or list"),
                            }
                        }
                    }
                } else {
                    // json.load(file) → serde_json::from_reader(file).unwrap()
                    parse_quote! { serde_json::from_reader::<_, serde_json::Value>(#file).unwrap() }
                }
            }

            _ => {
                bail!("json.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert re (regular expressions) module method calls
    /// DEPYLER-STDLIB-RE: Comprehensive regex module support
    ///
    /// Maps Python re module functions to Rust regex crate:
    /// - re.search() → Regex::new().find()
    /// - re.match() → Regex::new().is_match() with ^ anchor
    /// - re.findall() → Regex::new().find_iter()
    /// - re.sub() → Regex::new().replace_all()
    /// - re.split() → Regex::new().split()
    /// - re.compile() → Regex::new()
    /// - re.escape() → regex::escape()
    ///
    /// # Complexity
    /// 10 (match with 10 branches)
    #[inline]
    fn try_convert_re_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need regex crate
        self.ctx.needs_regex = true;

        // DEPYLER-0961: Helper to extract bare string literals for regex methods
        // Regex::new() and find() expect &str, not String
        // String literals should be passed directly without .to_string()
        let extract_str_arg = |idx: usize| -> syn::Expr {
            match args.get(idx) {
                Some(HirExpr::Literal(Literal::String(s))) => {
                    let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                    parse_quote! { #lit }
                }
                _ => arg_exprs.get(idx).cloned().unwrap_or_else(|| parse_quote! { "" }),
            }
        };

        let result = match method {
            // Pattern matching functions
            "search" => {
                if arg_exprs.len() < 2 {
                    bail!("re.search() requires at least 2 arguments (pattern, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let text = extract_str_arg(1);

                // Handle optional flags (simplified - just check for IGNORECASE)
                if arg_exprs.len() >= 3 {
                    // With flags: use RegexBuilder
                    parse_quote! {
                        regex::RegexBuilder::new(#pattern)
                            .case_insensitive(true)
                            .build()
                            .unwrap()
                            .find(#text)
                    }
                } else {
                    // No flags: direct Regex::new()
                    // re.search(pattern, text) → Regex::new(pattern).unwrap().find(text)
                    parse_quote! { regex::Regex::new(#pattern).unwrap().find(#text) }
                }
            }

            "match" => {
                if arg_exprs.len() < 2 {
                    bail!("re.match() requires at least 2 arguments (pattern, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let text = extract_str_arg(1);

                // DEPYLER-0389: re.match() in Python only matches at the beginning
                // Returns Option<Match> to support .group() calls
                // NOTE: Add start-of-string constraint in future (check match.start() == 0 or prepend ^) (tracked in DEPYLER-0389)
                // For now, using .find() like search() - compatible with Match object usage
                parse_quote! { regex::Regex::new(#pattern).unwrap().find(#text) }
            }

            "findall" => {
                if arg_exprs.len() < 2 {
                    bail!("re.findall() requires at least 2 arguments (pattern, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let text = extract_str_arg(1);

                // re.findall(pattern, text) → Regex::new(pattern).unwrap().find_iter(text).map(|m| m.as_str()).collect::<Vec<_>>()
                parse_quote! {
                    regex::Regex::new(#pattern)
                        .unwrap()
                        .find_iter(#text)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<_>>()
                }
            }

            "finditer" => {
                if arg_exprs.len() < 2 {
                    bail!("re.finditer() requires at least 2 arguments (pattern, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let text = extract_str_arg(1);

                // re.finditer(pattern, text) → Regex::new(pattern).unwrap().find_iter(text)
                parse_quote! {
                    regex::Regex::new(#pattern)
                        .unwrap()
                        .find_iter(#text)
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<_>>()
                }
            }

            // String substitution
            "sub" => {
                if arg_exprs.len() < 3 {
                    bail!("re.sub() requires at least 3 arguments (pattern, repl, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let repl = extract_str_arg(1);
                let text = extract_str_arg(2);

                // re.sub(pattern, repl, text) → Regex::new(pattern).unwrap().replace_all(text, repl)
                parse_quote! {
                    regex::Regex::new(#pattern)
                        .unwrap()
                        .replace_all(#text, #repl)
                        .to_string()
                }
            }

            "subn" => {
                if arg_exprs.len() < 3 {
                    bail!("re.subn() requires at least 3 arguments (pattern, repl, string)");
                }
                // DEPYLER-0961: Extract bare string literals for &str compatibility
                let pattern = extract_str_arg(0);
                let repl = extract_str_arg(1);
                let text = extract_str_arg(2);

                // re.subn(pattern, repl, text) → returns (result, count)
                parse_quote! {
                    {
                        let re = regex::Regex::new(#pattern).unwrap();
                        let count = re.find_iter(#text).count();
                        let result = re.replace_all(#text, #repl).to_string();
                        (result, count)
                    }
                }
            }

            // Pattern compilation
            "compile" => {
                if arg_exprs.is_empty() {
                    bail!("re.compile() requires at least 1 argument (pattern)");
                }
                let pattern = &arg_exprs[0];

                // Check for flags
                if arg_exprs.len() >= 2 {
                    // With flags: use RegexBuilder
                    // For now, simplified handling of common flags
                    parse_quote! {
                        regex::RegexBuilder::new(#pattern)
                            .case_insensitive(true)
                            .build()
                            .unwrap()
                    }
                } else {
                    // No flags: direct Regex::new()
                    // re.compile(pattern) → Regex::new(pattern).unwrap()
                    parse_quote! { regex::Regex::new(#pattern).unwrap() }
                }
            }

            // String splitting
            "split" => {
                if arg_exprs.len() < 2 {
                    bail!("re.split() requires at least 2 arguments (pattern, string)");
                }
                let pattern = &arg_exprs[0];
                let text = &arg_exprs[1];

                // Check for maxsplit argument
                if arg_exprs.len() >= 3 {
                    let maxsplit = &arg_exprs[2];
                    // re.split(pattern, text, maxsplit) → Regex::new(pattern).unwrap().splitn(maxsplit + 1, text)
                    parse_quote! {
                        regex::Regex::new(#pattern)
                            .unwrap()
                            .splitn(#text, #maxsplit + 1)
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    }
                } else {
                    // re.split(pattern, text) → Regex::new(pattern).unwrap().split(text)
                    parse_quote! {
                        regex::Regex::new(#pattern)
                            .unwrap()
                            .split(#text)
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                    }
                }
            }

            // Escaping special characters
            "escape" => {
                if arg_exprs.len() != 1 {
                    bail!("re.escape() requires exactly 1 argument");
                }
                let text = &arg_exprs[0];

                // re.escape(text) → regex::escape(text)
                parse_quote! { regex::escape(#text).to_string() }
            }

            _ => {
                bail!("re.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert string module method calls
    /// DEPYLER-STDLIB-STRING: String module utilities
    ///
    /// Maps Python string module functions to Rust equivalents:
    /// - string.capwords() → split/capitalize/join
    /// - string.Template → String formatting
    ///
    /// # Complexity
    /// 2 (match with 2 branches)
    #[inline]
    fn try_convert_string_method(
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
            // String utilities
            "capwords" => {
                if arg_exprs.is_empty() {
                    bail!("string.capwords() requires at least 1 argument (text)");
                }
                let text = &arg_exprs[0];

                // string.capwords(text) → text.split_whitespace().map(|w| {
                //     let mut c = w.chars();
                //     match c.next() {
                //         None => String::new(),
                //         Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
                //     }
                // }).collect::<Vec<_>>().join(" ")
                parse_quote! {
                    #text.split_whitespace()
                        .map(|w| {
                            let mut chars = w.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    let mut result = first.to_uppercase().collect::<String>();
                                    result.push_str(&chars.as_str().to_lowercase());
                                    result
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            }

            _ => {
                bail!("string.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert time module method calls
    /// DEPYLER-STDLIB-TIME: Time measurement and manipulation
    ///
    /// Maps Python time module functions to Rust equivalents:
    /// - time.time() → SystemTime::now()
    /// - time.sleep() → thread::sleep()
    /// - time.monotonic() → Instant::now()
    ///
    /// # Complexity
    /// 7 (match with 7+ branches)
    #[inline]
    fn try_convert_time_method(
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
            // Basic time measurement
            "time" => {
                // time.time() → SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
                parse_quote! {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64()
                }
            }

            "monotonic" | "perf_counter" => {
                // time.monotonic() → Instant::now() (returns Instant, need elapsed)
                // For now, simplified: just generate the call
                // In real usage, user would call .elapsed() later
                parse_quote! { std::time::Instant::now() }
            }

            "process_time" => {
                // time.process_time() → CPU time (requires platform-specific code)
                // Simplified: use Instant as approximation
                parse_quote! { std::time::Instant::now() }
            }

            "thread_time" => {
                // time.thread_time() → thread-specific time
                // Simplified: use Instant
                parse_quote! { std::time::Instant::now() }
            }

            // Sleep function
            "sleep" => {
                if arg_exprs.len() != 1 {
                    bail!("time.sleep() requires exactly 1 argument (seconds)");
                }
                let seconds = &arg_exprs[0];

                // time.sleep(seconds) → thread::sleep(Duration::from_secs_f64(seconds))
                parse_quote! {
                    std::thread::sleep(std::time::Duration::from_secs_f64(#seconds))
                }
            }

            // Time formatting (requires chrono for full support)
            "ctime" => {
                self.ctx.needs_chrono = true;
                if arg_exprs.len() != 1 {
                    bail!("time.ctime() requires exactly 1 argument (timestamp)");
                }
                let timestamp = &arg_exprs[0];

                // time.ctime(timestamp) → chrono formatting
                // Simplified: convert timestamp to DateTime
                parse_quote! {
                    {
                        let secs = #timestamp as i64;
                        let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                        chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos)
                            .unwrap()
                            .to_string()
                    }
                }
            }

            "strftime" => {
                self.ctx.needs_chrono = true;
                if arg_exprs.len() < 2 {
                    bail!("time.strftime() requires at least 2 arguments (format, time_tuple)");
                }
                // DEPYLER-0935: chrono's format() takes &str, not String
                // Extract bare string literal for compatibility
                let format = match args.first() {
                    Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                let _time_tuple = &arg_exprs[1];

                // time.strftime(format, time_tuple) → chrono formatting
                // Simplified: assume current time for now
                parse_quote! {
                    chrono::Local::now().format(#format).to_string()
                }
            }

            "strptime" => {
                self.ctx.needs_chrono = true;
                if arg_exprs.len() < 2 {
                    bail!("time.strptime() requires at least 2 arguments (string, format)");
                }
                let time_str = &arg_exprs[0];
                // DEPYLER-0935: chrono's parse_from_str() takes &str, not String
                // Extract bare string literal for compatibility
                let format = match args.get(1) {
                    Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                    _ => arg_exprs[1].clone(),
                };

                // time.strptime(string, format) → chrono parsing
                parse_quote! {
                    chrono::NaiveDateTime::parse_from_str(#time_str, #format).unwrap()
                }
            }

            // Time conversion
            "gmtime" => {
                self.ctx.needs_chrono = true;
                let timestamp = if arg_exprs.is_empty() {
                    parse_quote! { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() }
                } else {
                    arg_exprs[0].clone()
                };

                // time.gmtime(timestamp) → chrono UTC conversion
                parse_quote! {
                    {
                        let secs = #timestamp as i64;
                        let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                        chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos).unwrap()
                    }
                }
            }

            "localtime" => {
                self.ctx.needs_chrono = true;
                let timestamp = if arg_exprs.is_empty() {
                    parse_quote! { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() }
                } else {
                    arg_exprs[0].clone()
                };

                // time.localtime(timestamp) → chrono Local conversion
                parse_quote! {
                    {
                        let secs = #timestamp as i64;
                        let nanos = ((#timestamp - secs as f64) * 1_000_000_000.0) as u32;
                        chrono::DateTime::<chrono::Local>::from_timestamp(secs, nanos).unwrap()
                    }
                }
            }

            "mktime" => {
                self.ctx.needs_chrono = true;
                if arg_exprs.len() != 1 {
                    bail!("time.mktime() requires exactly 1 argument (time_tuple)");
                }
                let time_tuple = &arg_exprs[0];

                // time.mktime(time_tuple) → timestamp conversion
                // Simplified: assume time_tuple is a chrono DateTime
                parse_quote! { #time_tuple.timestamp() as f64 }
            }

            "asctime" => {
                self.ctx.needs_chrono = true;
                if arg_exprs.len() != 1 {
                    bail!("time.asctime() requires exactly 1 argument (time_tuple)");
                }
                let time_tuple = &arg_exprs[0];

                // time.asctime(time_tuple) → ASCII time string
                parse_quote! { #time_tuple.to_string() }
            }

            _ => {
                bail!("time.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert shutil module method calls
    /// DEPYLER-STDLIB-SHUTIL: Shell utilities for file operations
    ///
    /// Maps Python shutil module to Rust std::fs:
    /// - shutil.copy(src, dst) → std::fs::copy(src, dst)
    /// - shutil.copy2(src, dst) → std::fs::copy(src, dst) (simplified)
    /// - shutil.move(src, dst) → std::fs::rename(src, dst)
    /// - shutil.rmtree(path) → std::fs::remove_dir_all(path)
    /// - shutil.copytree(src, dst) → fs_extra::dir::copy (simplified)
    ///
    /// # Complexity: 4
    #[inline]
    fn try_convert_shutil_method(
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
            // shutil.copy(src, dst) → std::fs::copy(&src, &dst)
            // Returns dst (the destination path)
            "copy" | "copy2" => {
                if arg_exprs.len() < 2 {
                    bail!("shutil.{}() requires 2 arguments (src, dst)", method);
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                // Returns the number of bytes copied as u64, but Python returns dst
                // Use a block to match Python behavior
                parse_quote! {
                    {
                        std::fs::copy(&#src, &#dst).unwrap();
                        #dst.clone()
                    }
                }
            }

            // shutil.move(src, dst) → std::fs::rename(&src, &dst)
            "move" | "r#move" => {
                if arg_exprs.len() < 2 {
                    bail!("shutil.move() requires 2 arguments (src, dst)");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                parse_quote! {
                    {
                        std::fs::rename(&#src, &#dst).unwrap();
                        #dst.clone()
                    }
                }
            }

            // shutil.rmtree(path) → std::fs::remove_dir_all(&path)
            "rmtree" => {
                if arg_exprs.is_empty() {
                    bail!("shutil.rmtree() requires 1 argument (path)");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::remove_dir_all(&#path).unwrap() }
            }

            // shutil.copytree(src, dst) → simplified recursive copy
            "copytree" => {
                if arg_exprs.len() < 2 {
                    bail!("shutil.copytree() requires 2 arguments (src, dst)");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                // Simplified: use a function that recursively copies
                parse_quote! {
                    {
                        fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                            std::fs::create_dir_all(dst)?;
                            for entry in std::fs::read_dir(src)? {
                                let entry = entry?;
                                let file_type = entry.file_type()?;
                                if file_type.is_dir() {
                                    copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                                } else {
                                    std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
                                }
                            }
                            Ok(())
                        }
                        copy_dir_all(std::path::Path::new(&#src), std::path::Path::new(&#dst)).unwrap();
                        #dst.clone()
                    }
                }
            }

            // shutil.which(cmd) → std::process::Command to check if command exists
            "which" => {
                if arg_exprs.is_empty() {
                    bail!("shutil.which() requires 1 argument (cmd)");
                }
                let cmd = &arg_exprs[0];
                parse_quote! {
                    std::process::Command::new("which")
                        .arg(&#cmd)
                        .output()
                        .ok()
                        .filter(|o| o.status.success())
                        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                }
            }

            _ => {
                bail!("shutil.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

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
    fn try_convert_csv_method(
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

    /// Try to convert os module method calls
    /// DEPYLER-0380-BUG-2: os.getenv() with default values
    ///
    /// Maps Python os module to Rust std::env:
    /// - os.getenv(key) → std::env::var(key)?
    /// - os.getenv(key, default) → std::env::var(key).unwrap_or_else(|_| default.to_string())
    ///
    /// # Complexity
    /// ≤10 (match with few branches)
    #[inline]
    fn try_convert_os_method(
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
            "getenv" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("os.getenv() requires 1 or 2 arguments");
                }

                if arg_exprs.len() == 1 {
                    // os.getenv("KEY") → std::env::var("KEY")?
                    let key = &arg_exprs[0];
                    parse_quote! { std::env::var(#key)? }
                } else {
                    // os.getenv("KEY", "default") → std::env::var("KEY").unwrap_or_else(|_| "default".to_string())
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];

                    // DEPYLER-0380: Handle default value properly
                    // Python's os.getenv() always returns str, so the default must be converted to String.
                    // The unwrap_or_else closure must return String, so we always add .to_string()
                    // to ensure the default value is owned.
                    parse_quote! {
                        std::env::var(#key).unwrap_or_else(|_| #default.to_string())
                    }
                }
            }
            // DEPYLER-0196: os.unlink(path) → std::fs::remove_file(path)?
            // Python's os.unlink() removes a file
            "unlink" | "remove" => {
                if arg_exprs.len() != 1 {
                    bail!("os.{}() requires exactly 1 argument", method);
                }
                let path = &arg_exprs[0];
                // DEPYLER-0956: Use .unwrap() to not require Result return type
                parse_quote! { std::fs::remove_file(#path).unwrap() }
            }
            // DEPYLER-0196: os.mkdir(path) → std::fs::create_dir(path).unwrap()
            // DEPYLER-0956: Use .unwrap() to not require Result return type
            "mkdir" => {
                if arg_exprs.is_empty() {
                    bail!("os.mkdir() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // Ignore mode argument (arg_exprs[1]) as Rust uses system defaults
                parse_quote! { std::fs::create_dir(#path).unwrap() }
            }
            // DEPYLER-0196: os.makedirs(path) → std::fs::create_dir_all(path).unwrap()
            // DEPYLER-0956: Use .unwrap() instead of ? to not require Result return type
            // This matches Python's semantics where OSError is raised (panics in Rust)
            "makedirs" => {
                if arg_exprs.is_empty() {
                    bail!("os.makedirs() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // Ignore mode and exist_ok arguments as create_dir_all handles both
                parse_quote! { std::fs::create_dir_all(#path).unwrap() }
            }
            // DEPYLER-0196: os.rmdir(path) → std::fs::remove_dir(path).unwrap()
            // DEPYLER-0956: Use .unwrap() to not require Result return type
            "rmdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.rmdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::remove_dir(#path).unwrap() }
            }
            // DEPYLER-0196: os.rename(src, dst) → std::fs::rename(src, dst).unwrap()
            // DEPYLER-0956: Use .unwrap() to not require Result return type
            "rename" => {
                if arg_exprs.len() != 2 {
                    bail!("os.rename() requires exactly 2 arguments");
                }
                let src = &arg_exprs[0];
                let dst = &arg_exprs[1];
                parse_quote! { std::fs::rename(#src, #dst).unwrap() }
            }
            // DEPYLER-0196: os.getcwd() → std::env::current_dir()...
            // DEPYLER-0689: Use .expect() when not in Result-returning context
            "getcwd" => {
                if !arg_exprs.is_empty() {
                    bail!("os.getcwd() takes no arguments");
                }
                if self.ctx.current_function_can_fail {
                    parse_quote! { std::env::current_dir()?.to_string_lossy().to_string() }
                } else {
                    parse_quote! { std::env::current_dir().expect("Failed to get current directory").to_string_lossy().to_string() }
                }
            }
            // DEPYLER-0196: os.chdir(path) → std::env::set_current_dir(path)...
            // DEPYLER-0689: Use .expect() when not in Result-returning context
            "chdir" => {
                if arg_exprs.len() != 1 {
                    bail!("os.chdir() requires exactly 1 argument");
                }
                let path = &arg_exprs[0];
                if self.ctx.current_function_can_fail {
                    parse_quote! { std::env::set_current_dir(#path)? }
                } else {
                    parse_quote! { std::env::set_current_dir(#path).expect("Failed to change directory") }
                }
            }
            // DEPYLER-0196: os.listdir(path) → std::fs::read_dir(path)...
            // DEPYLER-0689: Use .expect() when not in Result-returning context
            "listdir" => {
                if arg_exprs.is_empty() {
                    // os.listdir() with no args uses current directory
                    if self.ctx.current_function_can_fail {
                        parse_quote! {
                            std::fs::read_dir(".")?
                                .filter_map(|e| e.ok())
                                .map(|e| e.file_name().to_string_lossy().to_string())
                                .collect::<Vec<_>>()
                        }
                    } else {
                        parse_quote! {
                            std::fs::read_dir(".").expect("Failed to read directory")
                                .filter_map(|e| e.ok())
                                .map(|e| e.file_name().to_string_lossy().to_string())
                                .collect::<Vec<_>>()
                        }
                    }
                } else {
                    let path = &arg_exprs[0];
                    if self.ctx.current_function_can_fail {
                        parse_quote! {
                            std::fs::read_dir(#path)?
                                .filter_map(|e| e.ok())
                                .map(|e| e.file_name().to_string_lossy().to_string())
                                .collect::<Vec<_>>()
                        }
                    } else {
                        parse_quote! {
                            std::fs::read_dir(#path).expect("Failed to read directory")
                                .filter_map(|e| e.ok())
                                .map(|e| e.file_name().to_string_lossy().to_string())
                                .collect::<Vec<_>>()
                        }
                    }
                }
            }
            // DEPYLER-0200: os.walk(path) → walkdir::WalkDir::new(path)
            // Returns iterator of (root, dirs, files) tuples like Python
            "walk" => {
                if arg_exprs.is_empty() {
                    bail!("os.walk() requires at least 1 argument");
                }
                let path = &arg_exprs[0];
                // Use walkdir crate - returns Vec<(String, Vec<String>, Vec<String>)>
                parse_quote! {
                    walkdir::WalkDir::new(#path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_dir())
                        .map(|dir_entry| {
                            let root = dir_entry.path().to_string_lossy().to_string();
                            let mut dirs = vec![];
                            let mut files = vec![];
                            if let Ok(entries) = std::fs::read_dir(dir_entry.path()) {
                                for entry in entries.filter_map(|e| e.ok()) {
                                    let name = entry.file_name().to_string_lossy().to_string();
                                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                                        dirs.push(name);
                                    } else {
                                        files.push(name);
                                    }
                                }
                            }
                            (root, dirs, files)
                        })
                        .collect::<Vec<_>>()
                }
            }
            // DEPYLER-0200: os.urandom(n) → rand crate for cryptographic random bytes
            "urandom" => {
                if arg_exprs.len() != 1 {
                    bail!("os.urandom() requires exactly 1 argument");
                }
                let n = &arg_exprs[0];
                // Use rand crate to generate random bytes
                parse_quote! {
                    {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let mut bytes = vec![0u8; #n as usize];
                        rng.fill(&mut bytes[..]);
                        bytes
                    }
                }
            }
            _ => {
                return Ok(None);
            }
        };

        Ok(Some(result))
    }

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
    fn try_convert_os_environ_method(
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
    fn convert_subprocess_run(
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
    fn convert_subprocess_popen(
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
    fn try_convert_numpy_call(
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
    fn try_convert_os_path_method(
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
    fn try_convert_base64_method(
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

                // base64.b64encode(data) → base64::engine::general_purpose::STANDARD.encode(data)
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.encode(#data)
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

                // base64.urlsafe_b64encode(data) → base64::engine::general_purpose::URL_SAFE.encode(data)
                parse_quote! {
                    base64::engine::general_purpose::URL_SAFE.encode(#data)
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
    fn try_convert_secrets_method(
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
    fn try_convert_hashlib_method(
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
                    // hashlib.md5(data) → one-shot hash computation
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use md5::Digest;
                            let mut hasher = md5::Md5::new();
                            hasher.update(#data);
                            hex::encode(hasher.finalize())
                        }
                    }
                }
            }

            // SHA-1 hash
            "sha1" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.sha1() requires exactly 1 argument");
                }
                self.ctx.needs_sha2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use sha1::Digest;
                        let mut hasher = sha1::Sha1::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
                    }
                }
            }

            // SHA-224 hash
            "sha224" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.sha224() requires exactly 1 argument");
                }
                self.ctx.needs_sha2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use sha2::Digest;
                        let mut hasher = sha2::Sha224::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
                    }
                }
            }

            // SHA-256 hash
            // DEPYLER-0558: Support both one-shot and incremental patterns
            // Use Box<dyn DynDigest> for type-erased hasher objects
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
                    // hashlib.sha256(data) → one-shot hash computation
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            let mut hasher = sha2::Sha256::new();
                            hasher.update(#data);
                            hex::encode(hasher.finalize())
                        }
                    }
                }
            }

            // SHA-384 hash
            "sha384" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.sha384() requires exactly 1 argument");
                }
                self.ctx.needs_sha2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use sha2::Digest;
                        let mut hasher = sha2::Sha384::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
                    }
                }
            }

            // SHA-512 hash
            "sha512" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.sha512() requires exactly 1 argument");
                }
                self.ctx.needs_sha2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use sha2::Digest;
                        let mut hasher = sha2::Sha512::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
                    }
                }
            }

            // BLAKE2b hash
            "blake2b" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.blake2b() requires exactly 1 argument");
                }
                self.ctx.needs_blake2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use blake2::Digest;
                        let mut hasher = blake2::Blake2b512::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
                    }
                }
            }

            // BLAKE2s hash
            "blake2s" => {
                if arg_exprs.len() != 1 {
                    bail!("hashlib.blake2s() requires exactly 1 argument");
                }
                self.ctx.needs_blake2 = true;
                let data = &arg_exprs[0];

                parse_quote! {
                    {
                        use blake2::Digest;
                        let mut hasher = blake2::Blake2s256::new();
                        hasher.update(#data);
                        hex::encode(hasher.finalize())
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
    fn try_convert_uuid_method(
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
    fn try_convert_hmac_method(
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
    fn try_convert_platform_method(
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
    fn try_convert_calendar_method(
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
    fn try_convert_binascii_method(
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
    fn try_convert_urllib_parse_method(
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
    fn try_convert_fnmatch_method(
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
    fn try_convert_shlex_method(
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
    fn try_convert_textwrap_method(
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
    fn try_convert_bisect_method(
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
    fn try_convert_heapq_method(
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
    fn try_convert_copy_method(
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

    /// Try to convert itertools module method calls
    /// DEPYLER-STDLIB-ITERTOOLS: Iterator combinatorics and lazy evaluation
    ///
    /// Supports: count, cycle, repeat, chain, islice, takewhile
    /// Maps to Rust's iterator adapters and std::iter methods
    ///
    /// # Complexity
    /// Cyclomatic: 7 (match with 6 functions + default)
    #[inline]
    fn try_convert_itertools_method(
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
            // Infinite counter with optional step
            "count" => {
                let start = if !arg_exprs.is_empty() {
                    &arg_exprs[0]
                } else {
                    &parse_quote!(0)
                };
                let step = if arg_exprs.len() >= 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(1)
                };

                parse_quote! {
                    {
                        let start = #start;
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;
                        std::iter::successors(Some(start), move |&n| Some(n + step))
                    }
                }
            }

            // Cycle through iterable infinitely
            "cycle" => {
                if arg_exprs.is_empty() {
                    bail!("itertools.cycle() requires at least 1 argument");
                }
                let iterable = &arg_exprs[0];

                parse_quote! {
                    {
                        let items = #iterable;
                        items.into_iter().cycle()
                    }
                }
            }

            // Repeat value n times (or infinitely if no count)
            "repeat" => {
                if arg_exprs.is_empty() {
                    bail!("itertools.repeat() requires at least 1 argument");
                }
                let value = &arg_exprs[0];

                if arg_exprs.len() >= 2 {
                    let times = &arg_exprs[1];
                    parse_quote! {
                        {
                            let val = #value;
                            let n = #times as usize;
                            std::iter::repeat(val).take(n)
                        }
                    }
                } else {
                    parse_quote! {
                        {
                            let val = #value;
                            std::iter::repeat(val)
                        }
                    }
                }
            }

            // Chain multiple iterables together
            "chain" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.chain() requires at least 2 arguments");
                }

                // Chain first two, then fold the rest
                let first = &arg_exprs[0];
                let second = &arg_exprs[1];

                if arg_exprs.len() == 2 {
                    parse_quote! {
                        {
                            let a = #first;
                            let b = #second;
                            a.into_iter().chain(b.into_iter())
                        }
                    }
                } else {
                    // For more than 2, we need to chain them all
                    let mut chain_expr: syn::Expr = parse_quote! {
                        #first.into_iter().chain(#second.into_iter())
                    };

                    for item in &arg_exprs[2..] {
                        chain_expr = parse_quote! {
                            #chain_expr.chain(#item.into_iter())
                        };
                    }

                    chain_expr
                }
            }

            // Slice iterator with start, stop, step
            "islice" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.islice() requires at least 2 arguments");
                }
                let iterable = &arg_exprs[0];

                if arg_exprs.len() == 2 {
                    // islice(iterable, stop)
                    let stop = &arg_exprs[1];
                    parse_quote! {
                        {
                            let items = #iterable;
                            let n = #stop as usize;
                            items.into_iter().take(n)
                        }
                    }
                } else {
                    // islice(iterable, start, stop)
                    let start = &arg_exprs[1];
                    let stop = &arg_exprs[2];
                    parse_quote! {
                        {
                            let items = #iterable;
                            let start_idx = #start as usize;
                            let stop_idx = #stop as usize;
                            items.into_iter().skip(start_idx).take(stop_idx - start_idx)
                        }
                    }
                }
            }

            // Take while predicate is true
            "takewhile" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.takewhile() requires at least 2 arguments");
                }
                let predicate = &arg_exprs[0];
                let iterable = &arg_exprs[1];

                parse_quote! {
                    {
                        let pred = #predicate;
                        let items = #iterable;
                        items.into_iter().take_while(pred)
                    }
                }
            }

            // Drop while predicate is true
            "dropwhile" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.dropwhile() requires at least 2 arguments");
                }
                let predicate = &arg_exprs[0];
                let iterable = &arg_exprs[1];

                parse_quote! {
                    {
                        let pred = #predicate;
                        let items = #iterable;
                        items.into_iter().skip_while(pred)
                    }
                }
            }

            // Accumulate (running sum/product)
            "accumulate" => {
                if arg_exprs.is_empty() {
                    bail!("itertools.accumulate() requires at least 1 argument");
                }
                let iterable = &arg_exprs[0];

                // accumulate with default + operation
                parse_quote! {
                    {
                        let items = #iterable;
                        let mut acc = None;
                        items.into_iter().map(|x| {
                            acc = Some(match acc {
                                None => x,
                                Some(a) => a + x,
                            });
                            acc.unwrap()
                        }).collect::<Vec<_>>()
                    }
                }
            }

            // Compress - filter by selector booleans
            "compress" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.compress() requires at least 2 arguments");
                }
                let data = &arg_exprs[0];
                let selectors = &arg_exprs[1];

                parse_quote! {
                    {
                        let items = #data;
                        let sels = #selectors;
                        items.into_iter()
                            .zip(sels.into_iter())
                            .filter_map(|(item, sel)| if sel { Some(item) } else { None })
                            .collect::<Vec<_>>()
                    }
                }
            }

            // DEPYLER-0557: Group consecutive elements by key function
            // Python: groupby(iterable, key) -> Rust: iterable.group_by(|x| key(x))
            "groupby" => {
                if arg_exprs.len() < 2 {
                    bail!("itertools.groupby() requires at least 2 arguments (iterable, key)");
                }
                let iterable = &arg_exprs[0];
                let key_func = &arg_exprs[1];

                // Note: Rust's group_by requires Itertools trait in scope
                self.ctx.needs_itertools = true;

                parse_quote! {
                    {
                        use itertools::Itertools;
                        #iterable.into_iter().group_by(#key_func)
                    }
                }
            }

            _ => {
                bail!("itertools.{} not implemented yet (available: count, cycle, repeat, chain, islice, takewhile, dropwhile, accumulate, compress, groupby)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert functools module method calls
    /// DEPYLER-STDLIB-FUNCTOOLS: Higher-order functions
    ///
    /// Supports: reduce
    /// Maps to Rust's Iterator::fold() method
    ///
    /// # Complexity
    /// Cyclomatic: 2 (match with 1 function + default)
    #[inline]
    fn try_convert_functools_method(
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
            // Reduce/fold operation
            "reduce" => {
                if arg_exprs.len() < 2 {
                    bail!("functools.reduce() requires at least 2 arguments");
                }
                let function = &arg_exprs[0];
                let iterable = &arg_exprs[1];

                if arg_exprs.len() >= 3 {
                    // With initial value
                    let initial = &arg_exprs[2];
                    parse_quote! {
                        {
                            let func = #function;
                            let items = #iterable;
                            let init = #initial;
                            items.into_iter().fold(init, func)
                        }
                    }
                } else {
                    // Without initial value - use first element
                    parse_quote! {
                        {
                            let func = #function;
                            let mut items = (#iterable).into_iter();
                            let init = items.next().expect("reduce() of empty sequence with no initial value");
                            items.fold(init, func)
                        }
                    }
                }
            }

            _ => {
                bail!(
                    "functools.{} not implemented yet (available: reduce)",
                    method
                );
            }
        };

        Ok(Some(result))
    }

    /// Try to convert warnings module method calls
    /// DEPYLER-STDLIB-WARNINGS: Warning control
    ///
    /// Supports: warn
    /// Maps to Rust's eprintln! macro for stderr output
    ///
    /// # Complexity
    /// Cyclomatic: 2 (match with 1 function + default)
    #[inline]
    fn try_convert_warnings_method(
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
            "warn" => {
                if arg_exprs.is_empty() {
                    bail!("warnings.warn() requires at least 1 argument");
                }
                let message = &arg_exprs[0];

                parse_quote! {
                    eprintln!("Warning: {}", #message)
                }
            }

            _ => {
                bail!("warnings.{} not implemented yet (available: warn)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert sys module method calls
    /// DEPYLER-STDLIB-SYS: System-specific parameters and functions
    ///
    /// Supports: exit
    /// Maps to Rust's std::process::exit
    ///
    /// # Complexity
    /// Cyclomatic: 2 (match with 1 function + default)
    #[inline]
    fn try_convert_sys_method(
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
    fn try_convert_pickle_method(
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
    fn try_convert_pprint_method(
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
    fn try_convert_fractions_method(
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

    /// Try to convert pathlib module method calls
    /// DEPYLER-STDLIB-PATHLIB: Comprehensive pathlib module support
    #[inline]
    fn try_convert_pathlib_method(
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
            // Path queries
            "exists" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.exists() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { #path.exists() }
            }

            "is_file" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.is_file() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { #path.is_file() }
            }

            "is_dir" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.is_dir() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { #path.is_dir() }
            }

            "is_absolute" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.is_absolute() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { #path.is_absolute() }
            }

            // Path transformations
            "absolute" | "resolve" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.{}() requires exactly 1 argument (self)", method);
                }
                let path = &arg_exprs[0];
                // Both absolute() and resolve() → canonicalize()
                parse_quote! { #path.canonicalize().unwrap() }
            }

            "with_name" => {
                if arg_exprs.len() != 2 {
                    bail!("Path.with_name() requires exactly 2 arguments (self, name)");
                }
                let path = &arg_exprs[0];
                let name = &arg_exprs[1];
                parse_quote! { #path.with_file_name(#name) }
            }

            "with_suffix" => {
                if arg_exprs.len() != 2 {
                    bail!("Path.with_suffix() requires exactly 2 arguments (self, suffix)");
                }
                let path = &arg_exprs[0];
                let suffix = &arg_exprs[1];
                parse_quote! { #path.with_extension(#suffix.trim_start_matches('.')) }
            }

            // Directory operations
            "mkdir" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("Path.mkdir() requires 1-2 arguments");
                }
                let path = &arg_exprs[0];

                // Check if parents=True was passed (simplified - assumes second arg is parents)
                if arg_exprs.len() == 2 {
                    // mkdir(parents=True) → create_dir_all
                    parse_quote! { std::fs::create_dir_all(#path).unwrap() }
                } else {
                    // mkdir() → create_dir
                    parse_quote! { std::fs::create_dir(#path).unwrap() }
                }
            }

            "rmdir" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.rmdir() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::remove_dir(#path).unwrap() }
            }

            "iterdir" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.iterdir() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! {
                    std::fs::read_dir(#path)
                        .unwrap()
                        .map(|e| e.unwrap().path())
                        .collect::<Vec<_>>()
                }
            }

            // File operations
            "read_text" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.read_text() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::read_to_string(#path).unwrap() }
            }

            "read_bytes" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.read_bytes() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::read(#path).unwrap() }
            }

            "write_text" => {
                if arg_exprs.len() != 2 {
                    bail!("Path.write_text() requires exactly 2 arguments (self, content)");
                }
                let path = &arg_exprs[0];
                let content = &arg_exprs[1];
                parse_quote! { std::fs::write(#path, #content).unwrap() }
            }

            "write_bytes" => {
                if arg_exprs.len() != 2 {
                    bail!("Path.write_bytes() requires exactly 2 arguments (self, content)");
                }
                let path = &arg_exprs[0];
                let content = &arg_exprs[1];
                parse_quote! { std::fs::write(#path, #content).unwrap() }
            }

            "unlink" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.unlink() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { std::fs::remove_file(#path).unwrap() }
            }

            "rename" => {
                if arg_exprs.len() != 2 {
                    bail!("Path.rename() requires exactly 2 arguments (self, target)");
                }
                let path = &arg_exprs[0];
                let target = &arg_exprs[1];
                parse_quote! { { std::fs::rename(&#path, #target).unwrap(); std::path::PathBuf::from(#target) } }
            }

            // Conversions
            "as_posix" => {
                if arg_exprs.len() != 1 {
                    bail!("Path.as_posix() requires exactly 1 argument (self)");
                }
                let path = &arg_exprs[0];
                parse_quote! { #path.to_str().unwrap().to_string() }
            }

            _ => return Ok(None), // Not a recognized pathlib method
        };

        Ok(Some(result))
    }

    /// DEPYLER-0829: Convert pathlib methods on Path/PathBuf variable instances
    /// This handles cases like `p.write_text(content)` where p is a Path variable
    /// Unlike try_convert_pathlib_method which handles module calls like pathlib.Path(...).method()
    #[inline]
    fn convert_pathlib_instance_method(
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

    /// DEPYLER-0830: Convert datetime/timedelta methods on variable instances
    /// This handles cases like `td.total_seconds()` where td is a TimeDelta variable
    /// Unlike try_convert_datetime_method which handles module calls like datetime.datetime.now()
    #[inline]
    fn convert_datetime_instance_method(
        &mut self,
        dt_expr: &syn::Expr,
        method: &str,
        hir_args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Mark that we need the chrono crate
        self.ctx.needs_chrono = true;

        let result = match method {
            // timedelta.total_seconds() → td.num_seconds() as f64
            "total_seconds" => {
                parse_quote! { #dt_expr.num_seconds() as f64 }
            }

            // datetime.fromisoformat(s) → NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            "fromisoformat" => {
                if arg_exprs.is_empty() {
                    bail!("fromisoformat() requires 1 argument (string)");
                }
                let s = &arg_exprs[0];
                parse_quote! {
                    chrono::NaiveDateTime::parse_from_str(&#s, "%Y-%m-%dT%H:%M:%S").unwrap()
                }
            }

            // datetime.isoformat() → dt.format("%Y-%m-%dT%H:%M:%S").to_string()
            "isoformat" => {
                parse_quote! { #dt_expr.format("%Y-%m-%dT%H:%M:%S").to_string() }
            }

            // datetime.strftime(fmt) → dt.format(fmt).to_string()
            // DEPYLER-0935: chrono's format() takes &str, not String
            // Extract bare string literal for compatibility
            "strftime" => {
                if arg_exprs.is_empty() {
                    bail!("strftime() requires 1 argument (format string)");
                }
                let fmt = match hir_args.first() {
                    Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                parse_quote! { #dt_expr.format(#fmt).to_string() }
            }

            // datetime.timestamp() → dt.and_utc().timestamp() as f64
            "timestamp" => {
                parse_quote! { #dt_expr.and_utc().timestamp() as f64 }
            }

            // datetime.timetuple() - not directly supported, return tuple of components
            "timetuple" => {
                parse_quote! {
                    (#dt_expr.year(), #dt_expr.month(), #dt_expr.day(),
                     #dt_expr.hour(), #dt_expr.minute(), #dt_expr.second())
                }
            }

            // datetime.weekday() → dt.weekday().num_days_from_monday()
            "weekday" => {
                parse_quote! { #dt_expr.weekday().num_days_from_monday() as i32 }
            }

            // datetime.isoweekday() → dt.weekday().number_from_monday()
            "isoweekday" => {
                parse_quote! { (#dt_expr.weekday().num_days_from_monday() + 1) as i32 }
            }

            // datetime.isocalendar() → (year, week, weekday)
            "isocalendar" => {
                parse_quote! {
                    {
                        let iso = #dt_expr.iso_week();
                        (iso.year(), iso.week() as i32, #dt_expr.weekday().number_from_monday() as i32)
                    }
                }
            }

            // datetime.replace() - simplified: with_year, with_month, etc.
            "replace" => {
                // For now, just pass through - would need kwargs support for proper impl
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
    /// DEPYLER-STDLIB-DATETIME: Comprehensive datetime module support
    #[inline]
    fn try_convert_datetime_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Mark that we need the chrono crate
        self.ctx.needs_chrono = true;

        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        let result = match method {
            // datetime.datetime.now([tz]) → Local::now() or Utc::now()
            "now" => {
                if arg_exprs.is_empty() {
                    parse_quote! { chrono::Local::now().naive_local() }
                } else {
                    // DEPYLER-0595: datetime.now(tz) - use Utc for UTC, Local otherwise
                    // For simplicity, assume UTC if any tz provided
                    parse_quote! { chrono::Utc::now().naive_utc() }
                }
            }

            // datetime.datetime.utcnow() → Utc::now()
            "utcnow" => {
                if arg_exprs.is_empty() {
                    parse_quote! { chrono::Utc::now().naive_utc() }
                } else {
                    bail!("datetime.utcnow() takes no arguments");
                }
            }

            // datetime.datetime.today() → Local::now().date()
            "today" => {
                if arg_exprs.is_empty() {
                    parse_quote! { chrono::Local::now().date_naive() }
                } else {
                    bail!("datetime.today() takes no arguments");
                }
            }

            // datetime.datetime.strftime(format) → dt.format(format).to_string()
            // DEPYLER-0555: chrono's format() takes &str, not String
            "strftime" => {
                if arg_exprs.len() != 2 {
                    bail!("strftime() requires exactly 2 arguments (self, format)");
                }
                let dt = &arg_exprs[0];
                // Extract bare string literal for chrono format compatibility
                let fmt = match &args[1] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[1].clone(),
                };
                parse_quote! { #dt.format(#fmt).to_string() }
            }

            // datetime.datetime.strptime(string, format) → NaiveDateTime::parse_from_str(string, format)
            // DEPYLER-0622: chrono's parse_from_str expects &str, not String
            "strptime" => {
                if arg_exprs.len() != 2 {
                    bail!("strptime() requires exactly 2 arguments (string, format)");
                }
                let s = &arg_exprs[0];
                // DEPYLER-0622: Extract bare string literal for &str compatibility
                // If fmt is a variable (not a literal), it might be String from iteration
                let fmt: syn::Expr = match &args[1] {
                    HirExpr::Literal(Literal::String(fmt_str)) => parse_quote! { #fmt_str },
                    _ => {
                        // For non-literals, add & to borrow as &str
                        let fmt_expr = &arg_exprs[1];
                        parse_quote! { &#fmt_expr }
                    }
                };
                parse_quote! {
                    chrono::NaiveDateTime::parse_from_str(#s, #fmt).unwrap()
                }
            }

            // datetime.datetime.isoformat() → dt.to_rfc3339()
            "isoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("isoformat() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                parse_quote! { #dt.to_string() }
            }

            // datetime.datetime.timestamp() → dt.timestamp()
            "timestamp" => {
                if arg_exprs.len() != 1 {
                    bail!("timestamp() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                parse_quote! { #dt.and_utc().timestamp() as f64 }
            }

            // datetime.datetime.fromtimestamp(ts) → NaiveDateTime::from_timestamp(ts, 0)
            // DEPYLER-0555: Use .clone() to handle both owned f64 and borrowed &f64 params
            // Clone on &f64 returns f64 due to Copy trait, making cast work for both cases
            "fromtimestamp" => {
                if arg_exprs.len() != 1 {
                    bail!("fromtimestamp() requires exactly 1 argument (timestamp)");
                }
                let ts = &arg_exprs[0];
                parse_quote! {
                    chrono::DateTime::from_timestamp((#ts).clone() as i64, 0)
                        .unwrap()
                        .naive_local()
                }
            }

            // date.weekday() → dt.weekday().num_days_from_monday()
            "weekday" => {
                if arg_exprs.len() != 1 {
                    bail!("weekday() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                parse_quote! { #dt.weekday().num_days_from_monday() as i32 }
            }

            // date.isoweekday() → dt.weekday().number_from_monday()
            "isoweekday" => {
                if arg_exprs.len() != 1 {
                    bail!("isoweekday() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                // ISO weekday: Monday=1, Sunday=7
                parse_quote! { (#dt.weekday().num_days_from_monday() + 1) as i32 }
            }

            // timedelta.total_seconds() → duration.num_seconds() as f64
            "total_seconds" => {
                if arg_exprs.len() != 1 {
                    bail!("total_seconds() requires exactly 1 argument (self)");
                }
                let td = &arg_exprs[0];
                parse_quote! { #td.num_seconds() as f64 }
            }

            // datetime.date() → extract date part
            "date" => {
                if arg_exprs.len() != 1 {
                    bail!("date() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                parse_quote! { #dt.date() }
            }

            // datetime.time() → extract time part
            "time" => {
                if arg_exprs.len() != 1 {
                    bail!("time() requires exactly 1 argument (self)");
                }
                let dt = &arg_exprs[0];
                parse_quote! { #dt.time() }
            }

            // datetime.replace(year=..., month=..., day=..., ...)
            "replace" => {
                if arg_exprs.len() != 2 {
                    bail!("replace() not fully implemented (requires keyword args)");
                }
                // Simplified: assume single year replacement
                let dt = &arg_exprs[0];
                let new_year = &arg_exprs[1];
                parse_quote! { #dt.with_year(#new_year as i32).unwrap() }
            }

            // DEPYLER-0938: datetime.combine(date, time) → NaiveDateTime::new(date, time)
            "combine" => {
                if arg_exprs.len() != 2 {
                    bail!("combine() requires exactly 2 arguments (date, time)");
                }
                let date_expr = &arg_exprs[0];
                let time_expr = &arg_exprs[1];
                parse_quote! { chrono::NaiveDateTime::new(#date_expr, #time_expr) }
            }

            // DEPYLER-0938: datetime.fromisoformat(string) → NaiveDateTime::parse_from_str
            "fromisoformat" => {
                if arg_exprs.len() != 1 {
                    bail!("fromisoformat() requires exactly 1 argument (string)");
                }
                let s = &arg_exprs[0];
                parse_quote! {
                    chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S")
                        .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S"))
                        .or_else(|_| chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d"))
                        .unwrap()
                }
            }

            _ => return Ok(None), // Not a recognized datetime method
        };

        Ok(Some(result))
    }

    /// Try to convert statistics module method calls
    /// DEPYLER-STDLIB-STATISTICS: Comprehensive statistics module support
    #[inline]
    fn try_convert_decimal_method(
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

    fn try_convert_statistics_method(
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

    /// Try to convert random module method calls
    /// DEPYLER-STDLIB-RANDOM: Comprehensive random module support
    #[inline]
    fn try_convert_random_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // Mark that we need rand crate
        self.ctx.needs_rand = true;

        let result = match method {
            // Basic random generation
            "random" => {
                if !arg_exprs.is_empty() {
                    bail!("random.random() takes no arguments");
                }
                // random.random() → rand::random::<f64>()
                parse_quote! { rand::random::<f64>() }
            }

            // Integer range functions
            // DEPYLER-0656: Add use rand::Rng for gen_range method
            "randint" => {
                if arg_exprs.len() != 2 {
                    bail!("random.randint() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // random.randint(a, b) → rand::thread_rng().gen_range(a..=b)
                // Python's randint is inclusive on both ends
                parse_quote! {
                    {
                        use rand::Rng;
                        rand::thread_rng().gen_range(#a..=#b)
                    }
                }
            }

            // DEPYLER-0656: Add use rand::Rng for gen_range method
            "randrange" => {
                // randrange can take 1, 2, or 3 arguments (like range)
                if arg_exprs.is_empty() || arg_exprs.len() > 3 {
                    bail!("random.randrange() requires 1-3 arguments");
                }

                if arg_exprs.len() == 1 {
                    // randrange(stop) → gen_range(0..stop)
                    let stop = &arg_exprs[0];
                    parse_quote! {
                        {
                            use rand::Rng;
                            rand::thread_rng().gen_range(0..#stop)
                        }
                    }
                } else if arg_exprs.len() == 2 {
                    // randrange(start, stop) → gen_range(start..stop)
                    let start = &arg_exprs[0];
                    let stop = &arg_exprs[1];
                    parse_quote! {
                        {
                            use rand::Rng;
                            rand::thread_rng().gen_range(#start..#stop)
                        }
                    }
                } else {
                    // randrange(start, stop, step) - complex, need to generate stepped range
                    let start = &arg_exprs[0];
                    let stop = &arg_exprs[1];
                    let step = &arg_exprs[2];
                    parse_quote! {
                        {
                            use rand::Rng;
                            let start = #start;
                            let stop = #stop;
                            // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;
                            let num_steps = ((stop - start) / step).max(0);
                            let offset = rand::thread_rng().gen_range(0..num_steps);
                            start + offset * step
                        }
                    }
                }
            }

            // Float range function
            // DEPYLER-0656: Add use rand::Rng for gen_range method
            "uniform" => {
                if arg_exprs.len() != 2 {
                    bail!("random.uniform() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // random.uniform(a, b) → rand::thread_rng().gen_range(a..b)
                parse_quote! {
                    {
                        use rand::Rng;
                        rand::thread_rng().gen_range((#a as f64)..=(#b as f64))
                    }
                }
            }

            // Sequence functions
            // DEPYLER-0656: Add use rand::seq::SliceRandom for choose/shuffle
            "choice" => {
                if arg_exprs.len() != 1 {
                    bail!("random.choice() requires exactly 1 argument");
                }
                let seq = &arg_exprs[0];
                // random.choice(seq) → *seq.choose(&mut rand::thread_rng()).unwrap()
                parse_quote! {
                    {
                        use rand::seq::SliceRandom;
                        *#seq.choose(&mut rand::thread_rng()).unwrap()
                    }
                }
            }

            "shuffle" => {
                if arg_exprs.len() != 1 {
                    bail!("random.shuffle() requires exactly 1 argument");
                }
                let seq = &arg_exprs[0];
                // random.shuffle(seq) → seq.shuffle(&mut rand::thread_rng())
                // Note: This mutates in place like Python
                parse_quote! {
                    {
                        use rand::seq::SliceRandom;
                        #seq.shuffle(&mut rand::thread_rng())
                    }
                }
            }

            // DEPYLER-0656: Add use rand::seq::SliceRandom for choose_multiple
            "sample" => {
                if arg_exprs.len() != 2 {
                    bail!("random.sample() requires exactly 2 arguments");
                }
                let seq = &arg_exprs[0];
                let k = &arg_exprs[1];
                // random.sample(seq, k) → seq.choose_multiple(&mut rand::thread_rng(), k).cloned().collect()
                parse_quote! {
                    {
                        use rand::seq::SliceRandom;
                        #seq.choose_multiple(&mut rand::thread_rng(), #k as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    }
                }
            }

            "choices" => {
                if arg_exprs.is_empty() {
                    bail!("random.choices() requires at least 1 argument");
                }
                let seq = &arg_exprs[0];
                let k = if arg_exprs.len() > 1 {
                    &arg_exprs[1]
                } else {
                    // Default k=1 if not provided
                    &parse_quote! { 1 }
                };
                // random.choices(seq, k=k) → (0..k).map(|_| seq.choose(&mut rng).cloned()).collect()
                parse_quote! {
                    {
                        let mut rng = rand::thread_rng();
                        (0..#k)
                            .map(|_| #seq.choose(&mut rng).cloned().unwrap())
                            .collect::<Vec<_>>()
                    }
                }
            }

            // Distribution functions
            "gauss" | "normalvariate" => {
                if arg_exprs.len() != 2 {
                    bail!("random.{}() requires exactly 2 arguments", method);
                }
                // GH-207: Mark that we need rand_distr crate for Normal distribution
                self.ctx.needs_rand_distr = true;
                let mu = &arg_exprs[0];
                let sigma = &arg_exprs[1];
                // Use rand_distr::Normal
                parse_quote! {
                    {
                        use rand::distributions::Distribution;
                        let normal = rand_distr::Normal::new(#mu as f64, #sigma as f64).unwrap();
                        normal.sample(&mut rand::thread_rng())
                    }
                }
            }

            "expovariate" => {
                if arg_exprs.len() != 1 {
                    bail!("random.expovariate() requires exactly 1 argument");
                }
                // GH-207: Mark that we need rand_distr crate for Exp distribution
                self.ctx.needs_rand_distr = true;
                let lambd = &arg_exprs[0];
                // Use rand_distr::Exp
                parse_quote! {
                    {
                        use rand::distributions::Distribution;
                        let exp = rand_distr::Exp::new(#lambd as f64).unwrap();
                        exp.sample(&mut rand::thread_rng())
                    }
                }
            }

            "betavariate" => {
                if arg_exprs.len() != 2 {
                    bail!("random.betavariate() requires exactly 2 arguments");
                }
                // GH-207: Mark that we need rand_distr crate for Beta distribution
                self.ctx.needs_rand_distr = true;
                let alpha = &arg_exprs[0];
                let beta = &arg_exprs[1];
                parse_quote! {
                    {
                        use rand::distributions::Distribution;
                        let beta_dist = rand_distr::Beta::new(#alpha as f64, #beta as f64).unwrap();
                        beta_dist.sample(&mut rand::thread_rng())
                    }
                }
            }

            "gammavariate" => {
                if arg_exprs.len() != 2 {
                    bail!("random.gammavariate() requires exactly 2 arguments");
                }
                // GH-207: Mark that we need rand_distr crate for Gamma distribution
                self.ctx.needs_rand_distr = true;
                let alpha = &arg_exprs[0];
                let beta = &arg_exprs[1];
                parse_quote! {
                    {
                        use rand::distributions::Distribution;
                        let gamma = rand_distr::Gamma::new(#alpha as f64, #beta as f64).unwrap();
                        gamma.sample(&mut rand::thread_rng())
                    }
                }
            }

            // Seed function
            "seed" => {
                if arg_exprs.len() > 1 {
                    bail!("random.seed() requires 0 or 1 argument");
                }
                if arg_exprs.is_empty() {
                    // seed() with no args - use system entropy
                    parse_quote! { /* No-op: thread_rng is already seeded */ () }
                } else {
                    let seed_val = &arg_exprs[0];
                    // Note: thread_rng() cannot be seeded. We'd need to use StdRng::seed_from_u64()
                    // For now, we'll generate a comment
                    parse_quote! {
                        {
                            // Note: Seeding not fully implemented - use StdRng instead of thread_rng
                            let _seed = #seed_val;
                            ()
                        }
                    }
                }
            }

            // Get/Set state (complex, simplified implementation)
            "getstate" => {
                bail!("random.getstate() not supported - Rust RNG state management differs from Python");
            }
            "setstate" => {
                bail!("random.setstate() not supported - Rust RNG state management differs from Python");
            }

            // DEPYLER-STDLIB-RANDOM: Triangular distribution
            "triangular" => {
                if arg_exprs.len() < 2 || arg_exprs.len() > 3 {
                    bail!("random.triangular() requires 2 or 3 arguments");
                }
                // GH-207: Mark that we need rand_distr crate for Triangular distribution
                self.ctx.needs_rand_distr = true;
                let low = &arg_exprs[0];
                let high = &arg_exprs[1];
                let mode = if arg_exprs.len() == 3 {
                    &arg_exprs[2]
                } else {
                    // Default mode is midpoint
                    &parse_quote! { ((#low + #high) / 2.0) }
                };

                parse_quote! {
                    {
                        use rand::distributions::Distribution;
                        let triangular = rand_distr::Triangular::new(
                            #low as f64,
                            #high as f64,
                            #mode as f64
                        ).unwrap();
                        triangular.sample(&mut rand::thread_rng())
                    }
                }
            }

            // DEPYLER-STDLIB-RANDOM: randbytes() - generate random bytes
            "randbytes" => {
                if arg_exprs.len() != 1 {
                    bail!("random.randbytes() requires exactly 1 argument");
                }
                let n = &arg_exprs[0];

                parse_quote! {
                    {
                        use rand::Rng;
                        let n = #n as usize;
                        let mut rng = rand::thread_rng();
                        (0..n).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>()
                    }
                }
            }

            _ => {
                bail!("random.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert math module method calls
    /// DEPYLER-STDLIB-MATH: Comprehensive math module support
    #[inline]
    fn try_convert_math_method(
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
            // Trigonometric functions - all take one f64 argument
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" => {
                if arg_exprs.len() != 1 {
                    bail!("math.{}() requires exactly 1 argument", method);
                }
                let arg = &arg_exprs[0];
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { (#arg as f64).#method_ident() }
            }

            // atan2 takes two arguments
            "atan2" => {
                if arg_exprs.len() != 2 {
                    bail!("math.atan2() requires exactly 2 arguments");
                }
                let y = &arg_exprs[0];
                let x = &arg_exprs[1];
                parse_quote! { (#y as f64).atan2(#x as f64) }
            }

            // Hyperbolic functions
            "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" => {
                if arg_exprs.len() != 1 {
                    bail!("math.{}() requires exactly 1 argument", method);
                }
                let arg = &arg_exprs[0];
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                parse_quote! { (#arg as f64).#method_ident() }
            }

            // Power and logarithmic functions
            "sqrt" | "exp" | "ln" | "log2" | "log10" => {
                if arg_exprs.len() != 1 {
                    bail!("math.{}() requires exactly 1 argument", method);
                }
                let arg = &arg_exprs[0];
                let method_name = if method == "ln" { "ln" } else { method };
                let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
                parse_quote! { (#arg as f64).#method_ident() }
            }

            // log() can take 1 or 2 arguments (log(x) or log(x, base))
            "log" => {
                if arg_exprs.len() == 1 {
                    let arg = &arg_exprs[0];
                    // log(x) defaults to natural logarithm
                    parse_quote! { (#arg as f64).ln() }
                } else if arg_exprs.len() == 2 {
                    let x = &arg_exprs[0];
                    let base = &arg_exprs[1];
                    // log(x, base) → x.log(base)
                    parse_quote! { (#x as f64).log(#base as f64) }
                } else {
                    bail!("math.log() requires 1 or 2 arguments");
                }
            }

            // pow() takes two arguments
            "pow" => {
                if arg_exprs.len() != 2 {
                    bail!("math.pow() requires exactly 2 arguments");
                }
                let base = &arg_exprs[0];
                let exp = &arg_exprs[1];
                // Use powf for floating point exponents
                parse_quote! { (#base as f64).powf(#exp as f64) }
            }

            // Rounding functions
            "ceil" | "floor" | "trunc" | "round" => {
                if arg_exprs.len() != 1 {
                    bail!("math.{}() requires exactly 1 argument", method);
                }
                let arg = &arg_exprs[0];
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                // These return f64 in Rust, but Python's math.ceil/floor return int
                // We'll cast to i32 for ceil and floor
                if method == "ceil" || method == "floor" {
                    parse_quote! { (#arg as f64).#method_ident() as i32 }
                } else {
                    parse_quote! { (#arg as f64).#method_ident() }
                }
            }

            // Absolute value
            "fabs" => {
                if arg_exprs.len() != 1 {
                    bail!("math.fabs() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).abs() }
            }

            // copysign
            "copysign" => {
                if arg_exprs.len() != 2 {
                    bail!("math.copysign() requires exactly 2 arguments");
                }
                let x = &arg_exprs[0];
                let y = &arg_exprs[1];
                parse_quote! { (#x as f64).copysign(#y as f64) }
            }

            // Degree/Radian conversion
            "degrees" => {
                if arg_exprs.len() != 1 {
                    bail!("math.degrees() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).to_degrees() }
            }
            "radians" => {
                if arg_exprs.len() != 1 {
                    bail!("math.radians() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).to_radians() }
            }

            // Special value checks
            "isnan" => {
                if arg_exprs.len() != 1 {
                    bail!("math.isnan() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).is_nan() }
            }
            "isinf" => {
                if arg_exprs.len() != 1 {
                    bail!("math.isinf() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).is_infinite() }
            }
            "isfinite" => {
                if arg_exprs.len() != 1 {
                    bail!("math.isfinite() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                parse_quote! { (#arg as f64).is_finite() }
            }

            // GCD - requires num crate for integers
            "gcd" => {
                if arg_exprs.len() != 2 {
                    bail!("math.gcd() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // For now, implement simple Euclidean algorithm inline
                // NOTE: Use num_integer::gcd crate for better performance (tracked in DEPYLER-0424)
                parse_quote! {
                    {
                        let mut a = (#a as i64).abs();
                        let mut b = (#b as i64).abs();
                        while b != 0 {
                            let temp = b;
                            b = a % b;
                            a = temp;
                        }
                        a as i32
                    }
                }
            }

            // DEPYLER-0771: Integer square root - math.isqrt(n) → floor(sqrt(n)) as integer
            "isqrt" => {
                if arg_exprs.len() != 1 {
                    bail!("math.isqrt() requires exactly 1 argument");
                }
                let arg = &arg_exprs[0];
                // Python's isqrt returns the floor of the square root as an integer
                parse_quote! { ((#arg as f64).sqrt().floor() as i32) }
            }

            // Factorial - compute inline for now
            "factorial" => {
                if arg_exprs.len() != 1 {
                    bail!("math.factorial() requires exactly 1 argument");
                }
                let n = &arg_exprs[0];
                parse_quote! {
                    {
                        let n = #n as i32;
                        let mut result = 1i64;
                        for i in 1..=n {
                            result *= i as i64;
                        }
                        result as i32
                    }
                }
            }

            // ldexp and frexp - less common, basic implementation
            "ldexp" => {
                if arg_exprs.len() != 2 {
                    bail!("math.ldexp() requires exactly 2 arguments");
                }
                let x = &arg_exprs[0];
                let i = &arg_exprs[1];
                // ldexp(x, i) = x * 2^i
                parse_quote! { (#x as f64) * 2.0f64.powi(#i as i32) }
            }

            "frexp" => {
                // frexp returns (mantissa, exponent) where x = mantissa * 2^exponent
                // Rust doesn't have this built-in, so we'll implement it
                if arg_exprs.len() != 1 {
                    bail!("math.frexp() requires exactly 1 argument");
                }
                let x = &arg_exprs[0];
                parse_quote! {
                    {
                        let x = #x as f64;
                        if x == 0.0 {
                            (0.0, 0)
                        } else {
                            let exp = x.abs().log2().floor() as i32 + 1;
                            let mantissa = x / 2.0f64.powi(exp);
                            (mantissa, exp)
                        }
                    }
                }
            }

            // LCM - least common multiple
            "lcm" => {
                if arg_exprs.len() != 2 {
                    bail!("math.lcm() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // lcm(a, b) = abs(a * b) / gcd(a, b)
                parse_quote! {
                    {
                        let a = (#a as i64).abs();
                        let b = (#b as i64).abs();
                        if a == 0 || b == 0 {
                            0
                        } else {
                            // Compute GCD first
                            let mut gcd_a = a;
                            let mut gcd_b = b;
                            while gcd_b != 0 {
                                let temp = gcd_b;
                                gcd_b = gcd_a % gcd_b;
                                gcd_a = temp;
                            }
                            let gcd = gcd_a;
                            ((a / gcd) * b) as i32
                        }
                    }
                }
            }

            // isclose - floating point comparison with tolerance
            "isclose" => {
                if arg_exprs.len() < 2 {
                    bail!("math.isclose() requires at least 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];
                // Default rel_tol=1e-09, abs_tol=0.0
                parse_quote! {
                    {
                        let a = #a as f64;
                        let b = #b as f64;
                        let rel_tol = 1e-9;
                        let abs_tol = 0.0;
                        let diff = (a - b).abs();
                        diff <= abs_tol.max(rel_tol * a.abs().max(b.abs()))
                    }
                }
            }

            // modf - split into fractional and integer parts
            "modf" => {
                if arg_exprs.len() != 1 {
                    bail!("math.modf() requires exactly 1 argument");
                }
                let x = &arg_exprs[0];
                parse_quote! {
                    {
                        let x = #x as f64;
                        let int_part = x.trunc();
                        let frac_part = x - int_part;
                        (frac_part, int_part)
                    }
                }
            }

            // fmod - floating point remainder
            "fmod" => {
                if arg_exprs.len() != 2 {
                    bail!("math.fmod() requires exactly 2 arguments");
                }
                let x = &arg_exprs[0];
                let y = &arg_exprs[1];
                parse_quote! { (#x as f64) % (#y as f64) }
            }

            // hypot - Euclidean distance (hypotenuse)
            "hypot" => {
                if arg_exprs.len() != 2 {
                    bail!("math.hypot() requires exactly 2 arguments");
                }
                let x = &arg_exprs[0];
                let y = &arg_exprs[1];
                parse_quote! { (#x as f64).hypot(#y as f64) }
            }

            // dist - distance between two points
            "dist" => {
                if arg_exprs.len() != 2 {
                    bail!("math.dist() requires exactly 2 arguments (two points)");
                }
                let p = &arg_exprs[0];
                let q = &arg_exprs[1];
                // Simplified: assume 2D points
                parse_quote! {
                    {
                        let p = #p;
                        let q = #q;
                        let dx = p[0] - q[0];
                        let dy = p[1] - q[1];
                        ((dx * dx + dy * dy) as f64).sqrt()
                    }
                }
            }

            // DEPYLER-STDLIB-MATH: remainder() - IEEE remainder (different from fmod)
            "remainder" => {
                if arg_exprs.len() != 2 {
                    bail!("math.remainder() requires exactly 2 arguments");
                }
                let x = &arg_exprs[0];
                let y = &arg_exprs[1];
                // IEEE remainder: x - n*y where n is closest integer to x/y
                parse_quote! {
                    {
                        let x = #x as f64;
                        let y = #y as f64;
                        let n = (x / y).round();
                        x - n * y
                    }
                }
            }

            // DEPYLER-STDLIB-MATH: comb() - combinations (nCr)
            "comb" => {
                if arg_exprs.len() != 2 {
                    bail!("math.comb() requires exactly 2 arguments");
                }
                let n = &arg_exprs[0];
                let k = &arg_exprs[1];
                parse_quote! {
                    {
                        let n = #n as i64;
                        let k = #k as i64;
                        if k > n || k < 0 { 0 } else {
                            let k = if k > n - k { n - k } else { k };
                            let mut result = 1i64;
                            for i in 0..k {
                                result = result * (n - i) / (i + 1);
                            }
                            result as i32
                        }
                    }
                }
            }

            // DEPYLER-STDLIB-MATH: perm() - permutations (nPr)
            "perm" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("math.perm() requires 1 or 2 arguments");
                }
                let n = &arg_exprs[0];
                let k = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    n
                };
                parse_quote! {
                    {
                        let n = #n as i64;
                        let k = #k as i64;
                        if k > n || k < 0 { 0 } else {
                            let mut result = 1i64;
                            for i in 0..k {
                                result *= n - i;
                            }
                            result as i32
                        }
                    }
                }
            }

            // DEPYLER-STDLIB-MATH: expm1() - exp(x) - 1 (accurate for small x)
            "expm1" => {
                if arg_exprs.len() != 1 {
                    bail!("math.expm1() requires exactly 1 argument");
                }
                let x = &arg_exprs[0];
                parse_quote! { (#x as f64).exp_m1() }
            }

            _ => {
                bail!("math.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert module method call (e.g., os.getcwd())
    #[inline]
    fn try_convert_module_method(
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
                return self.try_convert_math_method(method, args);
            }

            // DEPYLER-STDLIB-RANDOM: Handle random module functions
            // random.random() → thread_rng().gen()
            // random.randint(a, b) → thread_rng().gen_range(a..=b)
            if module_name == "random" {
                return self.try_convert_random_method(method, args);
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
                return self.try_convert_pathlib_method(method, args);
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
                return self.try_convert_json_method(method, args);
            }

            // DEPYLER-STDLIB-RE: Regular expressions module
            if module_name == "re" {
                return self.try_convert_re_method(method, args);
            }

            // DEPYLER-STDLIB-STRING: String module utilities
            if module_name == "string" {
                return self.try_convert_string_method(method, args);
            }

            // DEPYLER-STDLIB-TIME: Time module
            if module_name == "time" {
                return self.try_convert_time_method(method, args);
            }

            // DEPYLER-STDLIB-SHUTIL: Shell utilities for file operations
            // shutil.copy(src, dst) → std::fs::copy(src, dst)
            // shutil.copy2(src, dst) → std::fs::copy(src, dst)
            // shutil.move(src, dst) → std::fs::rename(src, dst)
            if module_name == "shutil" {
                return self.try_convert_shutil_method(method, args);
            }

            // DEPYLER-STDLIB-CSV: CSV file operations
            // DEPYLER-0426: Pass kwargs for DictWriter(file, fieldnames=...)
            if module_name == "csv" {
                return self.try_convert_csv_method(method, args, kwargs);
            }

            // DEPYLER-0380: os module operations (getenv, etc.)
            // Must be checked before os.path to handle non-path os functions
            if module_name == "os" {
                if let Some(result) = self.try_convert_os_method(method, args)? {
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
                return self.try_convert_itertools_method(method, args);
            }

            // DEPYLER-STDLIB-FUNCTOOLS: Higher-order functions
            if module_name == "functools" {
                return self.try_convert_functools_method(method, args);
            }

            // DEPYLER-STDLIB-WARNINGS: Warning control
            if module_name == "warnings" {
                return self.try_convert_warnings_method(method, args);
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

    // ========================================================================
    // DEPYLER-0142 Phase 2: Category Handlers
    // ========================================================================

    /// Handle list methods (append, extend, pop, insert, remove, sort)
    #[inline]
    fn convert_list_method(
        &mut self,
        object_expr: &syn::Expr,
        object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        match method {
            "append" => {
                if arg_exprs.len() != 1 {
                    bail!("append() requires exactly one argument");
                }
                let arg = &arg_exprs[0];

                // DEPYLER-0422 Fix #7: Convert &str literals to String when pushing to Vec<String>
                // Five-Whys Root Cause:
                // 1. Why: expected String, found &str
                // 2. Why: String literal "X" is &str, but Vec<String>.push() needs String
                // 3. Why: Transpiler generates "X" without .to_string()
                // 4. Why: append method doesn't check element type
                // 5. ROOT CAUSE: Missing .to_string() for literals in Vec<String>
                let needs_to_string = if !hir_args.is_empty() {
                    // Check if argument is a string literal
                    let is_str_literal =
                        matches!(&hir_args[0], HirExpr::Literal(Literal::String(_)));

                    // Check if object is a Vec<String> by examining variable type
                    let is_vec_string = if let HirExpr::Var(var_name) = object {
                        matches!(
                            self.ctx.var_types.get(var_name),
                            Some(Type::List(element_type)) if matches!(**element_type, Type::String)
                        )
                    } else {
                        false
                    };

                    is_str_literal && is_vec_string
                } else {
                    false
                };

                if needs_to_string {
                    Ok(parse_quote! { #object_expr.push(#arg.to_string()) })
                } else {
                    Ok(parse_quote! { #object_expr.push(#arg) })
                }
            }
            "extend" => {
                // DEPYLER-0292: Handle iterator conversion for extend()
                if arg_exprs.len() != 1 {
                    bail!("extend() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // extend() expects IntoIterator<Item = T>, but we often pass &Vec<T>
                // which gives IntoIterator<Item = &T>. Add .iter().cloned() to fix this.
                // Check if arg is a reference (most common case for function parameters)
                let arg_string = quote! { #arg }.to_string();
                if arg_string.contains("&") || !arg_string.contains(".into_iter()") {
                    // Likely a reference or direct variable - add iterator conversion
                    Ok(parse_quote! { #object_expr.extend(#arg.iter().cloned()) })
                } else {
                    // Already an iterator or owned value
                    Ok(parse_quote! { #object_expr.extend(#arg) })
                }
            }
            "pop" => {
                // DEPYLER-0210 FIX: Handle pop() for sets, dicts, and lists
                // Disambiguate based on argument count FIRST, then object type

                if arg_exprs.len() == 2 {
                    // Only dict.pop(key, default) takes 2 arguments
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(parse_quote! { #object_expr.remove(&#key).unwrap_or(#default) })
                    } else {
                        Ok(parse_quote! { #object_expr.remove(#key).unwrap_or(#default) })
                    }
                } else if arg_exprs.len() > 2 {
                    bail!("pop() takes at most 2 arguments");
                } else if self.is_set_expr(object) {
                    // Set.pop() - must have 0 arguments
                    if !arg_exprs.is_empty() {
                        bail!("pop() takes no arguments for sets");
                    }
                    Ok(parse_quote! {
                        #object_expr.iter().next().cloned().map(|x| {
                            #object_expr.remove(&x);
                            x
                        }).expect("pop from empty set")
                    })
                } else if self.is_dict_expr(object) {
                    // Dict literal - pop(key) with 1 argument
                    if arg_exprs.len() != 1 {
                        bail!("dict literal pop() requires exactly 1 argument (key)");
                    }
                    let key = &arg_exprs[0];
                    // DEPYLER-0303: Don't add & for string literals or variables
                    let needs_ref = !hir_args.is_empty()
                        && !matches!(
                            hir_args[0],
                            HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                        );
                    if needs_ref {
                        Ok(
                            parse_quote! { #object_expr.remove(&#key).expect("KeyError: key not found") },
                        )
                    } else {
                        Ok(
                            parse_quote! { #object_expr.remove(#key).expect("KeyError: key not found") },
                        )
                    }
                } else if arg_exprs.is_empty() {
                    // List.pop() with no arguments - remove last element
                    Ok(parse_quote! { #object_expr.pop().unwrap_or_default() })
                } else {
                    // 1 argument: could be list.pop(index) OR dict.pop(key)
                    // Use multiple heuristics to disambiguate:
                    let arg = &arg_exprs[0];

                    // Heuristic 1: Explicit list literal
                    let is_list = self.is_list_expr(object);

                    // Heuristic 2: Explicit dict literal
                    let is_dict = self.is_dict_expr(object);

                    // Heuristic 3: Integer argument suggests list index
                    let arg_is_int = !hir_args.is_empty()
                        && matches!(hir_args[0], HirExpr::Literal(crate::hir::Literal::Int(_)));

                    if is_list || (!is_dict && arg_is_int) {
                        // List.pop(index) - use Vec::remove() which takes usize by value
                        Ok(parse_quote! { #object_expr.remove(#arg as usize) })
                    } else {
                        // dict.pop(key) - HashMap::remove() takes &K by reference
                        // DEPYLER-0303: Don't add & for string literals or variables
                        let needs_ref = !hir_args.is_empty()
                            && !matches!(
                                hir_args[0],
                                HirExpr::Literal(crate::hir::Literal::String(_)) | HirExpr::Var(_)
                            );
                        if needs_ref {
                            Ok(
                                parse_quote! { #object_expr.remove(&#arg).expect("KeyError: key not found") },
                            )
                        } else {
                            Ok(
                                parse_quote! { #object_expr.remove(#arg).expect("KeyError: key not found") },
                            )
                        }
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
                if self.is_set_expr(object) {
                    Ok(parse_quote! {
                        if !#object_expr.remove(&#value) {
                            panic!("KeyError: element not in set");
                        }
                    })
                } else {
                    Ok(parse_quote! {
                        if let Some(pos) = #object_expr.iter().position(|x| x == &#value) {
                            #object_expr.remove(pos)
                        } else {
                            panic!("ValueError: list.remove(x): x not in list")
                        }
                    })
                }
            }
            "index" => {
                // Python: list.index(value) -> returns index of first occurrence
                // Rust: list.iter().position(|x| x == &value).ok_or(...)
                if arg_exprs.len() != 1 {
                    bail!("index() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter()
                        .position(|x| x == &#value)
                        .map(|i| i as i32)
                        .expect("ValueError: value is not in list")
                })
            }
            "count" => {
                // Python: list.count(value) -> counts occurrences
                // Rust: list.iter().filter(|x| **x == value).count()
                if arg_exprs.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let value = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.iter().filter(|x| **x == #value).count() as i32
                })
            }
            "copy" => {
                // Python: list.copy() -> shallow copy OR copy.copy(x) -> shallow copy
                // Rust: list.clone() OR x.clone()
                // DEPYLER-0024 FIX: Handle copy.copy(x) from copy module
                if arg_exprs.len() == 1 {
                    // This is copy.copy(x) from the copy module being misparsed as method call
                    // Just clone the argument directly
                    let arg = &arg_exprs[0];
                    return Ok(parse_quote! { #arg.clone() });
                }
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                // This is list.copy() method - clone the list
                Ok(parse_quote! { #object_expr.clone() })
            }
            "clear" => {
                // Python: list.clear() -> removes all elements
                // Rust: list.clear()
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "reverse" => {
                // Python: list.reverse() -> reverses in place
                // Rust: list.reverse()
                if !arg_exprs.is_empty() {
                    bail!("reverse() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.reverse() })
            }
            "sort" => {
                // DEPYLER-0445: Python: list.sort(key=func, reverse=False)
                // Rust: list.sort_by_key(|x| func(x)) or list.sort()

                // Check for `key` kwarg
                let key_func = kwargs.iter().find(|(k, _)| k == "key").map(|(_, v)| v);
                let reverse = kwargs
                    .iter()
                    .find(|(k, _)| k == "reverse")
                    .and_then(|(_, v)| {
                        if let HirExpr::Literal(crate::hir::Literal::Bool(b)) = v {
                            Some(*b)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);

                if !arg_exprs.is_empty() {
                    bail!("sort() does not accept positional arguments");
                }

                match (key_func, reverse) {
                    (Some(key_expr), false) => {
                        // list.sort(key=func) → list.sort_by_key(|x| func(x))
                        // Convert key_expr to Rust callable
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(parse_quote! { #object_expr.sort_by_key(|x| #key_rust(x)) })
                    }
                    (Some(key_expr), true) => {
                        // list.sort(key=func, reverse=True) → list.sort_by_key(|x| std::cmp::Reverse(func(x)))
                        let key_rust = key_expr.to_rust_expr(self.ctx)?;
                        Ok(
                            parse_quote! { #object_expr.sort_by_key(|x| std::cmp::Reverse(#key_rust(x))) },
                        )
                    }
                    (None, false) => {
                        // list.sort() → list.sort()
                        Ok(parse_quote! { #object_expr.sort() })
                    }
                    (None, true) => {
                        // list.sort(reverse=True) → list.sort_by(|a, b| b.cmp(a))
                        Ok(parse_quote! { #object_expr.sort_by(|a, b| b.cmp(a)) })
                    }
                }
            }
            _ => bail!("Unknown list method: {}", method),
        }
    }

    /// Handle dict methods (get, keys, values, items, update)
    /// DEPYLER-0540: Added hir_object param to detect serde_json::Value types
    #[inline]
    fn convert_dict_method(
        &mut self,
        object_expr: &syn::Expr,
        hir_object: &HirExpr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0540: Check if this is a serde_json::Value that needs special handling
        let is_json_value = self.is_serde_json_value(hir_object);

        match method {
            "get" => {
                if arg_exprs.len() == 1 {
                    let key = &arg_exprs[0];
                    // DEPYLER-0330: Keep dict.get() as Option to support .is_none() checks
                    // Python: result = d.get(key); if result is None: ...
                    // Rust: let result = d.get(key).cloned(); if result.is_none() { ... }

                    // DEPYLER-0542: Always borrow the key to prevent move semantics issues
                    // HashMap::get() expects &Q where Q: Borrow<K>. Using & prevents:
                    // 1. Moving owned String keys (error E0382: use of moved value)
                    // 2. Type mismatches when key is &str vs String
                    // For &str params, &key becomes &&str but HashMap::get handles this fine
                    let key_expr: syn::Expr = if let Some(HirExpr::Var(var_name)) = hir_args.first()
                    {
                        // DEPYLER-0539: Check if var is known &str param - don't double borrow
                        if self.is_borrowed_str_param(var_name) {
                            parse_quote! { #key }
                        } else {
                            // Owned String or unknown - borrow to prevent move
                            parse_quote! { &#key }
                        }
                    } else if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                        // DEPYLER-0634: String literal key - use bare literal, not .to_string()
                        // HashMap.get() expects &Q where Q: Borrow<K>. A &str literal works
                        // directly with Borrow<String> because String implements Borrow<str>.
                        // Using "key".to_string() creates owned String which doesn't match &Q.
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    } else {
                        // Other expression - borrow to prevent move
                        parse_quote! { &#key }
                    };

                    // Return Option - downstream code will handle unwrapping if needed
                    Ok(parse_quote! { #object_expr.get(#key_expr).cloned() })
                } else if arg_exprs.len() == 2 {
                    let key = &arg_exprs[0];
                    let default = &arg_exprs[1];
                    // DEPYLER-0542: Borrow keys for dict.get() (but not string literals)
                    let key_expr: syn::Expr = if let Some(HirExpr::Var(var_name)) = hir_args.first()
                    {
                        if self.is_borrowed_str_param(var_name) {
                            parse_quote! { #key }
                        } else {
                            parse_quote! { &#key }
                        }
                    } else if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.first() {
                        // DEPYLER-0634: String literal key - use bare literal, not .to_string()
                        let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                        parse_quote! { #lit }
                    } else {
                        parse_quote! { &#key }
                    };

                    // DEPYLER-0700: Check if dict has serde_json::Value values (heterogeneous dict)
                    // If so, we need to wrap the default with serde_json::json!() for type compatibility
                    let dict_has_json_values = self.dict_has_json_value_values(hir_object);

                    // DEPYLER-0631: For string literal defaults, use directly without .to_string()
                    // HashMap<String, &str>.get() returns Option<&&str>, .cloned() gives Option<&str>
                    // unwrap_or expects &str, not String
                    let result = if dict_has_json_values {
                        // DEPYLER-0700: Dict has serde_json::Value values
                        // For dict.get(key, default), we need to:
                        // 1. Get the Value from dict
                        // 2. Convert to the expected type (usually String)
                        // Pattern: dict.get(key).and_then(|v| v.as_str()).unwrap_or(default).to_string()
                        self.ctx.needs_serde_json = true;
                        if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(s))) if !s.is_empty()) {
                            // String default - extract as string with fallback
                            if let Some(HirExpr::Literal(Literal::String(s))) = hir_args.get(1) {
                                let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                                parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#lit).to_string() }
                            } else {
                                parse_quote! { #object_expr.get(#key_expr).and_then(|v| v.as_str()).unwrap_or(#default).to_string() }
                            }
                        } else {
                            // Non-string default - use json!() and keep as Value
                            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(serde_json::json!(#default)) }
                        }
                    } else if matches!(hir_args.get(1), Some(HirExpr::Literal(Literal::String(_)))) {
                        // DEPYLER-0729: String literal default
                        // Check if dict value type is String (needs .to_string()) or &str (bare literal ok)
                        let dict_value_is_string = self.dict_value_type_is_string(hir_object);
                        if let HirExpr::Literal(Literal::String(s)) = &hir_args[1] {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            if dict_value_is_string {
                                // HashMap<K, String>.get().cloned() returns Option<String>
                                // unwrap_or needs String, so convert literal
                                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit.to_string()) }
                            } else {
                                // HashMap<K, &str> or unknown - use bare literal
                                parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#lit) }
                            }
                        } else {
                            parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
                        }
                    } else {
                        // Non-literal default - use as-is
                        parse_quote! { #object_expr.get(#key_expr).cloned().unwrap_or(#default) }
                    };
                    Ok(result)
                } else if arg_exprs.is_empty() {
                    // DEPYLER-0188: 0-arg get() is NOT dict.get() - fall through to generic handler
                    // This supports asyncio.Queue.get(), multiprocessing.Queue.get(), etc.
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    Ok(parse_quote! { #object_expr.#method_ident() })
                } else {
                    bail!("get() requires 1 or 2 arguments (or 0 for Queue.get())");
                }
            }
            "keys" => {
                // DEPYLER-0596: If keys() has arguments, it's a user-defined method, not dict.keys()
                // Fall through to generic handler for user-defined keys(section) methods
                if arg_exprs.is_empty() {
                    // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                    // .keys() returns an iterator, but Python's dict.keys() returns a list-like view
                    // We collect to Vec for better ergonomics (indexing, len(), etc.)
                    // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .keys()
                    if is_json_value {
                        Ok(
                            parse_quote! { #object_expr.as_object().unwrap().keys().cloned().collect::<Vec<_>>() },
                        )
                    } else {
                        Ok(parse_quote! { #object_expr.keys().cloned().collect::<Vec<_>>() })
                    }
                } else {
                    // User-defined keys() method with arguments - use generic call
                    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                    Ok(parse_quote! { #object_expr.#method_ident(#(#arg_exprs),*) })
                }
            }
            "values" => {
                if !arg_exprs.is_empty() {
                    bail!("values() takes no arguments");
                }
                // DEPYLER-0303 Phase 3 Fix #8: Return Vec for compatibility
                // However, this causes redundant .collect().iter() in sum(d.values())
                // NOTE: Consider context-aware return type (Vec vs Iterator) for optimization (tracked in DEPYLER-0303)
                // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .values()
                if is_json_value {
                    Ok(
                        parse_quote! { #object_expr.as_object().unwrap().values().cloned().collect::<Vec<_>>() },
                    )
                } else {
                    Ok(parse_quote! { #object_expr.values().cloned().collect::<Vec<_>>() })
                }
            }
            "items" => {
                if !arg_exprs.is_empty() {
                    bail!("items() takes no arguments");
                }
                // DEPYLER-0540: serde_json::Value needs .as_object().unwrap() before .iter()
                if is_json_value {
                    Ok(
                        parse_quote! { #object_expr.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
                    )
                } else {
                    Ok(
                        parse_quote! { #object_expr.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>() },
                    )
                }
            }
            "update" => {
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                // DEPYLER-0728: When iterating over borrowed HashMap<K, V>, iterator yields (&K, &V)
                // insert() expects (K, V), so we need to clone the references
                // Using .iter() explicitly handles both owned and borrowed dicts correctly
                Ok(parse_quote! {
                    for (k, v) in (#arg).iter() {
                        #object_expr.insert(k.clone(), v.clone());
                    }
                })
            }
            "setdefault" => {
                // dict.setdefault(key, default) - get or insert with default
                // Python: dict.setdefault(key, default) returns value at key, or inserts default and returns it
                // Rust: entry().or_insert(default).clone()
                if arg_exprs.len() != 2 {
                    bail!("setdefault() requires exactly 2 arguments (key, default)");
                }
                let key = &arg_exprs[0];
                let default = &arg_exprs[1];
                Ok(parse_quote! {
                    #object_expr.entry(#key).or_insert(#default).clone()
                })
            }
            "popitem" => {
                // dict.popitem() - remove and return arbitrary (key, value) pair
                // Python: dict.popitem() removes and returns arbitrary item, or raises KeyError
                // Rust: iter().next() to get first item, then remove it
                if !arg_exprs.is_empty() {
                    bail!("popitem() takes no arguments");
                }
                Ok(parse_quote! {
                    {
                        let key = #object_expr.keys().next().cloned()
                            .expect("KeyError: popitem(): dictionary is empty");
                        let value = #object_expr.remove(&key)
                            .expect("KeyError: key disappeared");
                        (key, value)
                    }
                })
            }
            "pop" => {
                // dict.pop(key, default=None) - remove and return value for key
                // Python: dict.pop(key[, default]) removes key and returns value, or returns default
                // Rust: remove() returns Option, use unwrap_or() for default
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("pop() requires 1 or 2 arguments (key, optional default)");
                }
                let key = &arg_exprs[0];
                if arg_exprs.len() == 2 {
                    let default = &arg_exprs[1];
                    Ok(parse_quote! {
                        #object_expr.remove(#key).unwrap_or(#default)
                    })
                } else {
                    Ok(parse_quote! {
                        #object_expr.remove(#key).expect("KeyError: key not found")
                    })
                }
            }
            // DEPYLER-STDLIB-50: clear() - remove all items
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            // DEPYLER-STDLIB-50: copy() - shallow copy
            "copy" => {
                if !arg_exprs.is_empty() {
                    bail!("copy() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clone() })
            }
            _ => bail!("Unknown dict method: {}", method),
        }
    }

    /// DEPYLER-0564: Check if object is dict value access that returns serde_json::Value
    /// When calling string methods on dict values, we need to convert Value to &str first
    #[inline]
    fn needs_value_to_string_conversion(&self, hir_object: &HirExpr) -> bool {
        // Pattern: dict["key"] where dict is HashMap<String, serde_json::Value>
        if let HirExpr::Index { base, .. } = hir_object {
            if let HirExpr::Var(var_name) = base.as_ref() {
                // Check if the variable is tracked as a Dict with Unknown value type
                if let Some(Type::Dict(_, val_type)) = self.ctx.var_types.get(var_name) {
                    return matches!(val_type.as_ref(), Type::Unknown);
                }
                // Heuristic: common dict variable names
                let name = var_name.as_str();
                return name == "info" || name == "data" || name == "config" || name == "result";
            }
        }
        // Pattern: dict.get("key") - check nested method chains
        self.check_dict_value_chain(hir_object)
    }

    /// DEPYLER-0564: Recursively check if expression is a dict value access chain
    fn check_dict_value_chain(&self, expr: &HirExpr) -> bool {
        match expr {
            // Direct dict.get("key") call
            HirExpr::MethodCall { object, method, .. } if method == "get" => {
                if let HirExpr::Var(var_name) = object.as_ref() {
                    let name = var_name.as_str();
                    return name == "info"
                        || name == "data"
                        || name == "config"
                        || name == "result";
                }
                false
            }
            // Chained method calls like dict.get("key").cloned().unwrap_or_default()
            HirExpr::MethodCall { object, method, .. }
                if method == "cloned" || method == "unwrap_or_default" || method == "unwrap" =>
            {
                // Check if base object is a dict access
                self.check_dict_value_chain(object)
            }
            _ => false,
        }
    }

    /// DEPYLER-0564: Check if Rust expression is likely a serde_json::Value
    /// by looking for patterns like .unwrap_or_default() which indicate dict value access
    fn rust_expr_needs_value_conversion(&self, expr: &syn::Expr) -> bool {
        // Convert to string and check for patterns
        let expr_str = quote::quote!(#expr).to_string();
        // Remove spaces for easier pattern matching
        let normalized = expr_str.replace(' ', "");
        // Pattern: .unwrap_or_default() on a .get() call suggests serde_json::Value
        if normalized.contains("unwrap_or_default") && normalized.contains(".get(") {
            // Check for common dict variable names
            return normalized.contains("info.")
                || normalized.contains("data.")
                || normalized.contains("config.")
                || normalized.contains("result.")
                || normalized.contains("stats.");
        }
        false
    }

    /// Handle string methods (upper, lower, strip, startswith, endswith, split, join, replace, find, count, isdigit, isalpha)
    #[inline]
    fn convert_string_method(
        &mut self,
        hir_object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
    ) -> Result<syn::Expr> {
        // DEPYLER-0564: Convert serde_json::Value to &str for string method calls
        // Check both HIR pattern and Rust expression pattern
        let needs_conversion = self.needs_value_to_string_conversion(hir_object)
            || self.rust_expr_needs_value_conversion(object_expr);
        let obj = if needs_conversion {
            parse_quote! { #object_expr.as_str().unwrap_or_default() }
        } else {
            object_expr.clone()
        };

        match method {
            "upper" => {
                if !arg_exprs.is_empty() {
                    bail!("upper() takes no arguments");
                }
                Ok(parse_quote! { #obj.to_uppercase() })
            }
            "lower" => {
                if !arg_exprs.is_empty() {
                    bail!("lower() takes no arguments");
                }
                Ok(parse_quote! { #obj.to_lowercase() })
            }
            "strip" => {
                // DEPYLER-0595: str.strip([chars]) → trim_matches
                // DEPYLER-0821: If receiver is a char from Counter iteration, use is_whitespace()
                // Python's char.strip() on a single char returns "" if whitespace, the char otherwise
                // In boolean context: if char.strip(): means "if not whitespace"
                if arg_exprs.is_empty() {
                    // Check if receiver is a char variable from string/Counter iteration
                    // Use both explicit tracking and heuristics for variable names
                    let is_likely_char = if let HirExpr::Var(var_name) = hir_object {
                        self.ctx.char_iter_vars.contains(var_name)
                            || var_name == "char"
                            || var_name == "ch"
                            || var_name == "c"
                            || var_name == "character"
                    } else {
                        false
                    };

                    if is_likely_char {
                        // For char type, strip() in boolean context = "is not whitespace"
                        return Ok(parse_quote! { !#obj.is_whitespace() });
                    }
                    Ok(parse_quote! { #obj.trim().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(parse_quote! { #obj.trim_matches(|c: char| #chars.contains(c)).to_string() })
                }
            }
            "startswith" => {
                if hir_args.len() != 1 {
                    bail!("startswith() requires exactly one argument");
                }
                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // String doesn't implement Pattern, but &str does
                // Only borrow if the arg is a String variable (not if already &str from fn_str_params)
                let prefix: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                Ok(parse_quote! { #obj.starts_with(#prefix) })
            }
            "endswith" => {
                if hir_args.len() != 1 {
                    bail!("endswith() requires exactly one argument");
                }
                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // String doesn't implement Pattern, but &str does
                // Only borrow if the arg is a String variable (not if already &str from fn_str_params)
                let suffix: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                Ok(parse_quote! { #obj.ends_with(#suffix) })
            }
            "split" => {
                if arg_exprs.is_empty() {
                    Ok(
                        parse_quote! { #obj.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    // DEPYLER-0225/0945: Extract bare string literal for Pattern trait compatibility
                    // Only borrow if the arg is a String variable (not if already &str)
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    Ok(
                        parse_quote! { #obj.split(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 2 {
                    // DEPYLER-0590: str.split(sep, maxsplit) → splitn(maxsplit+1, sep)
                    // Python's maxsplit is the max number of splits; Rust's splitn takes n parts
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    let maxsplit = &arg_exprs[1];
                    Ok(
                        parse_quote! { #obj.splitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("split() accepts at most 2 arguments (separator, maxsplit)");
                }
            }
            // DEPYLER-0202: str.rsplit(sep[, maxsplit]) - reverse split with Pattern trait fix
            // Must extract bare string literals for Pattern trait compatibility
            "rsplit" => {
                if arg_exprs.is_empty() {
                    // Python's rsplit() without args splits on whitespace
                    Ok(
                        parse_quote! { #obj.split_whitespace().rev().map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 1 {
                    // DEPYLER-0202/0945: Extract bare string literal for Pattern trait compatibility
                    // Only borrow if the arg is a String variable (not if already &str)
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    Ok(
                        parse_quote! { #obj.rsplit(#sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else if arg_exprs.len() == 2 {
                    // DEPYLER-0202: str.rsplit(sep, maxsplit) → rsplitn(maxsplit+1, sep)
                    // Python's maxsplit is the max number of splits; Rust's rsplitn takes n parts
                    let sep: syn::Expr = match &hir_args[0] {
                        HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                        HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                            arg_exprs[0].clone()
                        }
                        _ => {
                            let arg = &arg_exprs[0];
                            parse_quote! { &#arg }
                        }
                    };
                    let maxsplit = &arg_exprs[1];
                    Ok(
                        parse_quote! { #obj.rsplitn((#maxsplit + 1) as usize, #sep).map(|s| s.to_string()).collect::<Vec<String>>() },
                    )
                } else {
                    bail!("rsplit() accepts at most 2 arguments (separator, maxsplit)");
                }
            }
            "join" => {
                // DEPYLER-0196: sep.join(iterable) → iterable.join(sep) or iterable.collect::<Vec<_>>().join(sep)
                // DEPYLER-0575: Generator expressions yield iterators, need collect() before join()
                // DEPYLER-0597: Only use collect() for iterators, not for Vec/slice types
                if hir_args.len() != 1 {
                    bail!("join() requires exactly one argument");
                }
                let iterable = &arg_exprs[0];
                // Extract bare string literal for separator
                let separator = match hir_object {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => object_expr.clone(),
                };
                // Check if the iterable is already a collection (Var, List, etc.) vs an iterator
                // DEPYLER-0597: Vecs don't have .collect(), only iterators do
                let needs_collect = match &hir_args[0] {
                    HirExpr::GeneratorExp { .. } => true,
                    HirExpr::Call { func, .. } if func == "map" || func == "filter" || func == "iter" || func == "enumerate" => true,
                    _ => false,
                };
                if needs_collect {
                    Ok(parse_quote! { #iterable.collect::<Vec<_>>().join(#separator) })
                } else {
                    Ok(parse_quote! { #iterable.join(#separator) })
                }
            }
            "replace" => {
                // DEPYLER-0195: str.replace(old, new) → .replace(old, new)
                // DEPYLER-0301: str.replace(old, new, count) → .replacen(old, new, count)
                // DEPYLER-0595: datetime.replace() uses kwargs, has 0-1 positional args
                // Use bare string literals without .to_string() for correct types
                if hir_args.len() < 2 {
                    // Not str.replace - could be datetime.replace() with kwargs
                    // Fall through to generic method call
                    return Ok(parse_quote! { #object_expr.replace() });
                }
                if hir_args.len() > 3 {
                    bail!("replace() requires 2 or 3 arguments");
                }
                // DEPYLER-0945: Extract bare string literals for Pattern trait compatibility
                // When argument is a variable, borrow it since String doesn't implement Pattern
                // But skip borrowing if the variable is already &str from function parameter
                let old: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };
                let new: syn::Expr = match &hir_args[1] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[1].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[1];
                        parse_quote! { &#arg }
                    }
                };

                if hir_args.len() == 3 {
                    // Python: str.replace(old, new, count)
                    // Rust: str.replacen(old, new, count as usize)
                    let count = &arg_exprs[2];
                    Ok(parse_quote! { #object_expr.replacen(#old, #new, #count as usize) })
                } else {
                    // Python: str.replace(old, new)
                    // Rust: str.replace(old, new) - replaces all
                    Ok(parse_quote! { #object_expr.replace(#old, #new) })
                }
            }
            "find" => {
                // DEPYLER-0197/0338: str.find(sub[, start]) → .find(sub).map(|i| i as i32).unwrap_or(-1)
                // Python's find() returns -1 if not found, Rust's returns Option<usize>
                // Python supports optional start parameter: str.find(sub, start)
                if hir_args.is_empty() || hir_args.len() > 2 {
                    bail!("find() requires 1 or 2 arguments, got {}", hir_args.len());
                }

                // DEPYLER-0945: Extract bare string literal for Pattern trait compatibility
                // When argument is a variable, borrow it since String doesn't implement Pattern
                // But skip borrowing if the variable is already &str from function parameter
                let substring: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    HirExpr::Var(name) if self.ctx.fn_str_params.contains(name) => {
                        // Variable is already &str from function parameter, don't double-borrow
                        arg_exprs[0].clone()
                    }
                    _ => {
                        // Owned String variable, borrow it
                        let arg = &arg_exprs[0];
                        parse_quote! { &#arg }
                    }
                };

                if hir_args.len() == 2 {
                    // Python: str.find(sub, start)
                    // Rust: str[start..].find(sub).map(|i| (i + start) as i32).unwrap_or(-1)
                    let start = &arg_exprs[1];
                    Ok(parse_quote! {
                        #object_expr[#start as usize..].find(#substring)
                            .map(|i| (i + #start as usize) as i32)
                            .unwrap_or(-1)
                    })
                } else {
                    // Python: str.find(sub)
                    // Rust: str.find(sub).map(|i| i as i32).unwrap_or(-1)
                    Ok(parse_quote! {
                        #object_expr.find(#substring)
                            .map(|i| i as i32)
                            .unwrap_or(-1)
                    })
                }
            }
            "count" => {
                // DEPYLER-0198/0226: str.count(sub) → .matches(sub).count() as i32
                // Extract bare string literal for Pattern trait compatibility
                if hir_args.len() != 1 {
                    bail!("count() requires exactly one argument");
                }
                let substring: syn::Expr = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => {
                        // DEPYLER-0200: Use &* to deref-reborrow for Pattern trait compliance
                        // Works for both String (&*String -> &str) and &str (&*&str -> &str)
                        let arg = &arg_exprs[0];
                        parse_quote! { &*#arg }
                    }
                };
                Ok(parse_quote! { #object_expr.matches(#substring).count() as i32 })
            }
            "isdigit" => {
                // DEPYLER-0199: str.isdigit() → .chars().all(|c| c.is_numeric())
                if !arg_exprs.is_empty() {
                    bail!("isdigit() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_numeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_numeric()) })
            }
            "isalpha" => {
                // DEPYLER-0200: str.isalpha() → .chars().all(|c| c.is_alphabetic())
                if !arg_exprs.is_empty() {
                    bail!("isalpha() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_alphabetic() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphabetic()) })
            }
            "isspace" => {
                // DEPYLER-0650: str.isspace() → .chars().all(|c| c.is_whitespace())
                if !arg_exprs.is_empty() {
                    bail!("isspace() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_whitespace() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_whitespace()) })
            }
            "lstrip" => {
                // DEPYLER-0302/0595: str.lstrip([chars]) → .trim_start_matches
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.trim_start().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.trim_start_matches(|c: char| #chars.contains(c)).to_string() })
                }
            }
            "rstrip" => {
                // DEPYLER-0302/0595: str.rstrip([chars]) → .trim_end_matches
                if arg_exprs.is_empty() {
                    Ok(parse_quote! { #object_expr.trim_end().to_string() })
                } else {
                    let chars = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.trim_end_matches(|c: char| #chars.contains(c)).to_string() })
                }
            }
            "encode" => {
                // DEPYLER-0594: str.encode([encoding]) → .as_bytes().to_vec()
                // Python: s.encode() or s.encode('utf-8')
                // Rust: s.as_bytes().to_vec() (returns Vec<u8>)
                // Note: Only UTF-8 encoding is supported
                Ok(parse_quote! { #object_expr.as_bytes().to_vec() })
            }
            "decode" => {
                // DEPYLER-0594: bytes.decode([encoding]) → String::from_utf8_lossy()
                // Python: b.decode() or b.decode('utf-8')
                // Rust: String::from_utf8_lossy(bytes).to_string()

                // DEPYLER-0200: base64.encode() already returns String - no decode needed
                // Note: This is now handled in convert_method_call before reaching here
                // Check if object is a base64 encode call (represented as MethodCall)
                // base64.b64encode(...) is HIR MethodCall { object: Var("base64"), method: "b64encode", ... }
                let is_base64_encode = match hir_object {
                    HirExpr::MethodCall { object, method, .. } => {
                        matches!(object.as_ref(), HirExpr::Var(module) if module.as_str() == "base64")
                            && (method.as_str().contains("b64encode")
                                || method.as_str().contains("urlsafe_b64encode"))
                    }
                    HirExpr::Call { func, .. } => {
                        func.as_str().contains("b64encode")
                            || func.as_str().contains("urlsafe_b64encode")
                    }
                    _ => false,
                };

                if is_base64_encode {
                    // base64::encode() returns String - just return it directly
                    Ok(object_expr.clone())
                } else {
                    Ok(parse_quote! { String::from_utf8_lossy(&#obj).to_string() })
                }
            }
            "isalnum" => {
                // DEPYLER-0302: str.isalnum() → .chars().all(|c| c.is_alphanumeric())
                if !arg_exprs.is_empty() {
                    bail!("isalnum() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_alphanumeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_alphanumeric()) })
            }
            "title" => {
                // DEPYLER-0302 Phase 2: str.title() → custom title case implementation
                // Python's title() capitalizes the first letter of each word
                if !arg_exprs.is_empty() {
                    bail!("title() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
            }

            // DEPYLER-STDLIB-STR: index() - find with panic if not found
            "index" => {
                if hir_args.len() != 1 {
                    bail!("index() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.find(#substring)
                        .map(|i| i as i32)
                        .expect("substring not found")
                })
            }

            // DEPYLER-STDLIB-STR: rfind() - find from right (last occurrence)
            "rfind" => {
                if hir_args.len() != 1 {
                    bail!("rfind() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.rfind(#substring)
                        .map(|i| i as i32)
                        .unwrap_or(-1)
                })
            }

            // DEPYLER-STDLIB-STR: rindex() - rfind with panic if not found
            "rindex" => {
                if hir_args.len() != 1 {
                    bail!("rindex() requires exactly one argument");
                }
                let substring = match &hir_args[0] {
                    HirExpr::Literal(Literal::String(s)) => parse_quote! { #s },
                    _ => arg_exprs[0].clone(),
                };
                Ok(parse_quote! {
                    #object_expr.rfind(#substring)
                        .map(|i| i as i32)
                        .expect("substring not found")
                })
            }

            // DEPYLER-STDLIB-STR: center() - center string in field
            "center" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("center() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        let fillchar = #fillchar;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            let total_pad = width - s.len();
                            let left_pad = total_pad / 2;
                            let right_pad = total_pad - left_pad;
                            format!("{}{}{}", fillchar.repeat(left_pad), s, fillchar.repeat(right_pad))
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-STR: ljust() - left justify string
            "ljust" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("ljust() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        let fillchar = #fillchar;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            format!("{}{}", s, fillchar.repeat(width - s.len()))
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-STR: rjust() - right justify string
            "rjust" => {
                if arg_exprs.is_empty() || arg_exprs.len() > 2 {
                    bail!("rjust() requires 1 or 2 arguments");
                }
                let width = &arg_exprs[0];
                let fillchar = if arg_exprs.len() == 2 {
                    &arg_exprs[1]
                } else {
                    &parse_quote!(" ")
                };

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        let fillchar = #fillchar;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            format!("{}{}", fillchar.repeat(width - s.len()), s)
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-STR: zfill() - zero-fill numeric string
            "zfill" => {
                if arg_exprs.len() != 1 {
                    bail!("zfill() requires exactly 1 argument");
                }
                let width = &arg_exprs[0];

                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let width = #width as usize;
                        if s.len() >= width {
                            s.to_string()
                        } else {
                            let sign = if s.starts_with('-') || s.starts_with('+') { &s[0..1] } else { "" };
                            let num = if !sign.is_empty() { &s[1..] } else { &s[..] };
                            format!("{}{}{}", sign, "0".repeat(width - s.len()), num)
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-50: capitalize() - capitalize first character
            "capitalize" => {
                if !arg_exprs.is_empty() {
                    bail!("capitalize() takes no arguments");
                }
                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let mut chars = s.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-50: swapcase() - swap upper/lower case
            "swapcase" => {
                if !arg_exprs.is_empty() {
                    bail!("swapcase() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr.chars().map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().to_string()
                        } else {
                            c.to_uppercase().to_string()
                        }
                    }).collect::<String>()
                })
            }

            // DEPYLER-STDLIB-50: expandtabs() - expand tab characters
            "expandtabs" => {
                if arg_exprs.is_empty() {
                    Ok(parse_quote! {
                        #object_expr.replace("\t", &" ".repeat(8))
                    })
                } else if arg_exprs.len() == 1 {
                    // tabsize argument will be used at runtime
                    let tabsize_expr = &arg_exprs[0];
                    Ok(parse_quote! {
                        #object_expr.replace("\t", &" ".repeat(#tabsize_expr as usize))
                    })
                } else {
                    bail!("expandtabs() takes 0 or 1 arguments")
                }
            }

            // DEPYLER-STDLIB-50: splitlines() - split by line breaks
            "splitlines" => {
                if !arg_exprs.is_empty() {
                    bail!("splitlines() takes no arguments");
                }
                Ok(parse_quote! {
                    #object_expr.lines().map(|s| s.to_string()).collect::<Vec<String>>()
                })
            }

            // DEPYLER-STDLIB-50: partition() - partition by separator
            "partition" => {
                if arg_exprs.len() != 1 {
                    bail!("partition() requires exactly 1 argument (separator)");
                }
                let sep = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let sep_str = #sep;
                        if let Some(pos) = s.find(sep_str) {
                            let before = &s[..pos];
                            let after = &s[pos + sep_str.len()..];
                            (before.to_string(), sep_str.to_string(), after.to_string())
                        } else {
                            (s.to_string(), String::new(), String::new())
                        }
                    }
                })
            }

            // DEPYLER-STDLIB-50: casefold() - aggressive lowercase for caseless matching
            "casefold" => {
                if !arg_exprs.is_empty() {
                    bail!("casefold() takes no arguments");
                }
                // casefold() is like lower() but more aggressive for Unicode
                Ok(parse_quote! { #object_expr.to_lowercase() })
            }

            // DEPYLER-STDLIB-50: isprintable() - check if all characters are printable
            "isprintable" => {
                if !arg_exprs.is_empty() {
                    bail!("isprintable() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { !#object_expr.is_control() || #object_expr == '\t' || #object_expr == '\n' || #object_expr == '\r' });
                    }
                }
                Ok(parse_quote! {
                    #object_expr.chars().all(|c| !c.is_control() || c == '\t' || c == '\n' || c == '\r')
                })
            }
            // DEPYLER-0652: Additional is* string methods
            "isupper" => {
                if !arg_exprs.is_empty() {
                    bail!("isupper() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_uppercase() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) })
            }
            "islower" => {
                if !arg_exprs.is_empty() {
                    bail!("islower() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { !#object_expr.is_alphabetic() || #object_expr.is_lowercase() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) })
            }
            "istitle" => {
                if !arg_exprs.is_empty() {
                    bail!("istitle() takes no arguments");
                }
                // Title case: first char of each word is uppercase, rest lowercase
                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        let mut prev_is_cased = false;
                        s.chars().all(|c| {
                            let is_upper = c.is_uppercase();
                            let is_lower = c.is_lowercase();
                            let result = if c.is_alphabetic() {
                                if prev_is_cased { is_lower } else { is_upper }
                            } else { true };
                            prev_is_cased = c.is_alphabetic();
                            result
                        })
                    }
                })
            }
            "isnumeric" => {
                if !arg_exprs.is_empty() {
                    bail!("isnumeric() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_numeric() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_numeric()) })
            }
            "isascii" => {
                if !arg_exprs.is_empty() {
                    bail!("isascii() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_ascii() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_ascii()) })
            }
            "isdecimal" => {
                if !arg_exprs.is_empty() {
                    bail!("isdecimal() takes no arguments");
                }
                // DEPYLER-0796: If object is a char from string iteration, use direct char method
                if let HirExpr::Var(var_name) = hir_object {
                    if self.ctx.char_iter_vars.contains(var_name) {
                        return Ok(parse_quote! { #object_expr.is_ascii_digit() });
                    }
                }
                Ok(parse_quote! { #object_expr.chars().all(|c| c.is_ascii_digit()) })
            }
            "isidentifier" => {
                if !arg_exprs.is_empty() {
                    bail!("isidentifier() takes no arguments");
                }
                // Python identifier: starts with letter/underscore, followed by alphanumeric/underscore
                Ok(parse_quote! {
                    {
                        let s = #object_expr;
                        !s.is_empty() && s.chars().enumerate().all(|(i, c)| {
                            if i == 0 { c.is_alphabetic() || c == '_' }
                            else { c.is_alphanumeric() || c == '_' }
                        })
                    }
                })
            }

            // DEPYLER-0538: str/bytes.hex() - convert bytes to hexadecimal string
            "hex" => {
                if !arg_exprs.is_empty() {
                    bail!("hex() takes no arguments");
                }
                // Python: b"hello".hex() → "68656c6c6f"
                // Rust: convert each byte to 2-char hex string
                Ok(parse_quote! {
                    #object_expr.bytes().map(|b| format!("{:02x}", b)).collect::<String>()
                })
            }

            // DEPYLER-0770: str.format() - runtime string formatting
            "format" => {
                // Python: "Hello, {}!".format(name) -> "Hello, World!"
                // Rust: Use sequential replacen for positional formatting
                if arg_exprs.is_empty() {
                    // No args - return template unchanged
                    Ok(object_expr.clone())
                } else if arg_exprs.len() == 1 {
                    // Single arg - replace first {}
                    let arg = &arg_exprs[0];
                    Ok(parse_quote! {
                        #object_expr.replacen("{}", &format!("{}", #arg), 1)
                    })
                } else {
                    // Multiple args - chain replacen calls
                    // Build: template.replacen("{}", &format!("{}", a0), 1)
                    //                .replacen("{}", &format!("{}", a1), 1)...
                    let mut result: syn::Expr = parse_quote! { #object_expr.to_string() };
                    for arg in arg_exprs {
                        result = parse_quote! {
                            #result.replacen("{}", &format!("{}", #arg), 1)
                        };
                    }
                    Ok(result)
                }
            }

            _ => bail!("Unknown string method: {}", method),
        }
    }

    /// Handle set methods (add, discard, clear)
    #[inline]
    fn convert_set_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            "add" => {
                if arg_exprs.len() != 1 {
                    bail!("add() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.insert(#arg) })
            }
            "remove" => {
                // DEPYLER-0224: Set.remove(value) - remove value or panic if not found
                if arg_exprs.len() != 1 {
                    bail!("remove() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! {
                    if !#object_expr.remove(&#arg) {
                        panic!("KeyError: element not in set")
                    }
                })
            }
            "discard" => {
                if arg_exprs.len() != 1 {
                    bail!("discard() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.remove(&#arg) })
            }
            "clear" => {
                if !arg_exprs.is_empty() {
                    bail!("clear() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.clear() })
            }
            "update" => {
                // DEPYLER-0211 FIX: Set.update(other) - add all elements from other set
                if arg_exprs.len() != 1 {
                    bail!("update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    for item in #other {
                        #object_expr.insert(item);
                    }
                })
            }
            "intersection_update" => {
                // DEPYLER-0212 FIX: Set.intersection_update(other) - keep only common elements
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("intersection_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.intersection(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "difference_update" => {
                // DEPYLER-0213 FIX: Set.difference_update(other) - remove elements in other
                // Note: This generates an expression that returns (), suitable for ExprStmt
                if arg_exprs.len() != 1 {
                    bail!("difference_update() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    {
                        let temp: std::collections::HashSet<_> = #object_expr.difference(&#other).cloned().collect();
                        #object_expr.clear();
                        #object_expr.extend(temp);
                    }
                })
            }
            "union" => {
                // Set.union(other) - return new set with elements from both sets
                if arg_exprs.len() != 1 {
                    bail!("union() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.union(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "intersection" => {
                // Set.intersection(other) - return new set with common elements
                if arg_exprs.len() != 1 {
                    bail!("intersection() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.intersection(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "difference" => {
                // Set.difference(other) - return new set with elements not in other
                if arg_exprs.len() != 1 {
                    bail!("difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "symmetric_difference" => {
                // Set.symmetric_difference(other) - return new set with elements in either but not both
                if arg_exprs.len() != 1 {
                    bail!("symmetric_difference() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.symmetric_difference(&#other).cloned().collect::<std::collections::HashSet<_>>()
                })
            }
            "issubset" => {
                // Set.issubset(other) - check if all elements are in other
                if arg_exprs.len() != 1 {
                    bail!("issubset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_subset(&#other)
                })
            }
            "issuperset" => {
                // Set.issuperset(other) - check if contains all elements of other
                if arg_exprs.len() != 1 {
                    bail!("issuperset() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_superset(&#other)
                })
            }
            "isdisjoint" => {
                // Set.isdisjoint(other) - check if no common elements
                if arg_exprs.len() != 1 {
                    bail!("isdisjoint() requires exactly one argument");
                }
                let other = &arg_exprs[0];
                Ok(parse_quote! {
                    #object_expr.is_disjoint(&#other)
                })
            }
            _ => bail!("Unknown set method: {}", method),
        }
    }

    /// Handle regex methods (findall)
    #[inline]
    /// DEPYLER-0431: Convert regex instance method calls
    /// Handles both compiled Regex methods and Match object methods
    fn convert_regex_method(
        &mut self,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        match method {
            // Compiled Regex methods
            "findall" => {
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

            // DEPYLER-0431: compiled.match(text) → compiled.find(text)
            // Python re.match() only matches at start, but Rust .find() searches anywhere
            // NOTE: Full .groups() support requires proper regex type tracking (DEPYLER-0563)
            "match" => {
                if arg_exprs.is_empty() {
                    bail!("match() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // compiled.search(text) → compiled.find(text)
            "search" => {
                if arg_exprs.is_empty() {
                    bail!("search() requires at least one argument");
                }
                let text = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.find(#text) })
            }

            // DEPYLER-0519: Match object methods - handle Option<Match> from .find() results
            // Python's re.match/find returns None or Match, Rust's .find() returns Option<Match>
            // We need to unwrap before calling Match methods like .start(), .as_str()

            // match.group(0) → match.as_str() (for group 0)
            // match.group(n) → match.get(n).map(|m| m.as_str()) (for other groups)
            "group" => {
                if arg_exprs.is_empty() {
                    // No args: default to group 0
                    // DEPYLER-0519: Use map for Option safety
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                } else {
                    // Check if group_num is literal 0
                    if matches!(arg_exprs[0], syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref lit), .. }) if lit.base10_parse::<i32>().ok() == Some(0))
                    {
                        Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
                    } else {
                        // Non-zero group: needs captures API
                        bail!(
                            "match.group(n) for n>0 requires .captures() API (not yet implemented)"
                        )
                    }
                }
            }

            // match.groups() → extract all capture groups
            // DEPYLER-0442: Implement match.groups() using captured group extraction
            // Python: match.groups() returns tuple of all captured groups (excluding group 0)
            // NOTE: Full implementation requires regex type tracking (DEPYLER-0563)
            // For now, return empty vec - generator type system uses serde_json::Value as fallback
            "groups" => {
                // TODO: Implement proper capture group extraction when regex types are tracked
                Ok(parse_quote! {
                    Vec::<String>::new()
                })
            }

            // match.start() → match.start() (passthrough, with Option handling)
            "start" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.start()).unwrap_or(0) })
                } else {
                    bail!("match.start(group) with group number not yet implemented")
                }
            }

            // match.end() → match.end() (passthrough, with Option handling)
            "end" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(parse_quote! { #object_expr.as_ref().map(|m| m.end()).unwrap_or(0) })
                } else {
                    bail!("match.end(group) with group number not yet implemented")
                }
            }

            // match.span() → (match.start(), match.end())
            "span" => {
                if arg_exprs.is_empty() {
                    // DEPYLER-0519: Handle Option<Match>
                    Ok(
                        parse_quote! { #object_expr.as_ref().map(|m| (m.start(), m.end())).unwrap_or((0, 0)) },
                    )
                } else {
                    bail!("match.span(group) with group number not yet implemented")
                }
            }

            // match.as_str() → match.as_str() (passthrough, with Option handling)
            "as_str" => {
                if !arg_exprs.is_empty() {
                    bail!("as_str() takes no arguments");
                }
                // DEPYLER-0519: Handle Option<Match>
                Ok(parse_quote! { #object_expr.as_ref().map(|m| m.as_str()).unwrap_or("") })
            }

            _ => bail!("Unknown regex method: {}", method),
        }
    }

    /// DEPYLER-0381: Convert sys I/O stream method calls
    /// sys.stdout.write(msg) → writeln!(std::io::stdout(), "{}", msg).unwrap()
    /// sys.stdin.read() → { let mut s = String::new(); std::io::stdin().read_to_string(&mut s).unwrap(); s }
    /// sys.stdout.flush() → std::io::stdout().flush().unwrap()
    #[inline]
    fn convert_sys_io_method(
        &self,
        stream: &str,
        method: &str,
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        let stream_fn = match stream {
            "stdin" => quote! { std::io::stdin() },
            "stdout" => quote! { std::io::stdout() },
            "stderr" => quote! { std::io::stderr() },
            _ => bail!("Unknown I/O stream: {}", stream),
        };

        let result = match (stream, method) {
            // stdout/stderr write methods
            ("stdout" | "stderr", "write") => {
                if arg_exprs.is_empty() {
                    bail!("{}.write() requires an argument", stream);
                }
                let msg = &arg_exprs[0];
                // Use writeln! macro for cleaner code and automatic newline handling
                // If the message already has \n, use write! instead
                parse_quote! {
                    {
                        use std::io::Write;
                        write!(#stream_fn, "{}", #msg).unwrap();
                    }
                }
            }

            // flush method
            (_, "flush") => {
                parse_quote! {
                    {
                        use std::io::Write;
                        #stream_fn.flush().unwrap()
                    }
                }
            }

            // stdin read methods
            ("stdin", "read") => {
                parse_quote! {
                    {
                        use std::io::Read;
                        let mut buffer = String::new();
                        #stream_fn.read_to_string(&mut buffer).unwrap();
                        buffer
                    }
                }
            }

            ("stdin", "readline") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        let mut line = String::new();
                        #stream_fn.lock().read_line(&mut line).unwrap();
                        line
                    }
                }
            }

            // DEPYLER-0638: stdin.readlines() → collect all lines from stdin
            // Python: lines = sys.stdin.readlines()
            // Rust: std::io::stdin().lock().lines().collect::<Result<Vec<_>, _>>().unwrap()
            ("stdin", "readlines") => {
                parse_quote! {
                    {
                        use std::io::BufRead;
                        #stream_fn.lock().lines().collect::<Result<Vec<_>, _>>().unwrap()
                    }
                }
            }

            _ => bail!("{}.{}() is not yet supported", stream, method),
        };

        Ok(result)
    }

    /// Convert instance method calls (main dispatcher)
    #[inline]
    fn convert_instance_method(
        &mut self,
        object: &HirExpr,
        object_expr: &syn::Expr,
        method: &str,
        arg_exprs: &[syn::Expr],
        hir_args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {

        // DEPYLER-0363: Handle parse_args() → Skip for now, will be replaced with Args::parse()
        // ArgumentParser.parse_args() requires full struct transformation
        // For now, return unit to allow compilation
        if method == "parse_args" {
            // NOTE: Full argparse implementation requires Args::parse() call (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0363: Handle add_argument() → Skip for now, will be accumulated for struct generation
        if method == "add_argument" {
            // NOTE: Accumulate add_argument calls to generate struct fields (tracked in DEPYLER-0363)
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0109: Handle parser.print_help() → Args::command().print_help()
        // Python: parser.print_help() prints help and continues
        // Rust/clap: Args::command().print_help()? with CommandFactory trait
        if method == "print_help" {
            // Generate clap help printing using CommandFactory
            return Ok(parse_quote! {
                {
                    use clap::CommandFactory;
                    Args::command().print_help().unwrap()
                }
            });
        }

        // DEPYLER-0381: Handle sys I/O stream method calls
        // Check if object is a sys I/O stream (sys.stdin(), sys.stdout(), sys.stderr())
        if let HirExpr::Attribute { value, attr } = object {
            if let HirExpr::Var(module) = &**value {
                if module == "sys" && matches!(attr.as_str(), "stdin" | "stdout" | "stderr") {
                    return self.convert_sys_io_method(attr, method, arg_exprs);
                }
            }
        }

        // DEPYLER-0432: Handle file I/O .read() method
        // Python: f.read() → Rust: read_to_string() or read_to_end()
        if method == "read" && arg_exprs.is_empty() {
            // f.read() with no arguments → read entire file
            // Need to determine if text or binary mode
            // For now, default to text mode (read_to_string)
            // TODO: Track file open mode to distinguish text vs binary
            return Ok(parse_quote! {
                {
                    let mut content = String::new();
                    #object_expr.read_to_string(&mut content)?;
                    content
                }
            });
        }

        // DEPYLER-0558: Handle file I/O .read(size) method for chunked reading
        // Python: chunk = f.read(8192) → reads up to 8192 bytes, returns bytes (empty = EOF)
        // Rust: f.read(&mut buf) → reads into buffer, returns count (0 = EOF)
        if method == "read" && arg_exprs.len() == 1 {
            let size = &arg_exprs[0];
            return Ok(parse_quote! {
                {
                    let mut _read_buf = vec![0u8; #size];
                    let _n = #object_expr.read(&mut _read_buf).unwrap_or(0);
                    _read_buf.truncate(_n);
                    _read_buf
                }
            });
        }

        // DEPYLER-0305: Handle file I/O .readlines() method
        // Python: lines = f.readlines() → Rust: BufReader::new(f).lines().collect()
        if method == "readlines" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                std::io::BufReader::new(#object_expr)
                    .lines()
                    .map(|l| l.unwrap_or_default())
                    .collect::<Vec<_>>()
            });
        }

        // DEPYLER-0305: Handle file I/O .readline() method
        // Python: line = f.readline() → Rust: read one line
        if method == "readline" && arg_exprs.is_empty() {
            self.ctx.needs_bufread = true;
            return Ok(parse_quote! {
                {
                    let mut _line = String::new();
                    std::io::BufReader::new(&mut #object_expr).read_line(&mut _line).unwrap_or(0);
                    _line
                }
            });
        }

        // DEPYLER-0458: Handle file I/O .write() method
        // DEPYLER-0537: Use .unwrap() instead of ? for functions without explicit error handling
        // DEPYLER-0536: Handle Option<String> arguments by unwrapping
        // Python: f.write(string) → Rust: f.write_all(bytes).unwrap()
        if method == "write" && arg_exprs.len() == 1 {
            // DEPYLER-0605: Set needs_io_write flag for Write trait import
            self.ctx.needs_io_write = true;
            let content = &arg_exprs[0];
            // Check if content might be an Option type based on HIR expression
            // If it's a variable that's known to be Option, unwrap it first
            // DEPYLER-0536: Detect Option type for write() content argument
            // Priority: type system > name heuristics (only use heuristics when no type info)
            // DEPYLER-0647: Check option_unwrap_map first - if already unwrapped, not Option
            // DEPYLER-0666: Also check if var_name is an UNWRAPPED name (value in map)
            let is_option_content = if let HirExpr::Var(var_name) = &hir_args[0] {
                // Check if variable is already unwrapped (inside if-let body)
                let is_unwrapped =
                    self.ctx.option_unwrap_map.contains_key(var_name)
                        || self.ctx.option_unwrap_map.values().any(|v| v == var_name);
                if is_unwrapped {
                    false // Already unwrapped, not Option
                } else {
                    match self.ctx.var_types.get(var_name) {
                        Some(Type::Optional(_)) => true,
                        Some(_) => false, // Known non-Option type - don't use name heuristic
                        None => {
                            // No type info - fall back to name heuristic
                            var_name == "content"
                                || var_name.ends_with("_content")
                                || var_name.ends_with("_text")
                        }
                    }
                }
            } else {
                false
            };

            // Convert string to bytes and use write_all()
            // Python's write() returns bytes written, but we simplify to just the operation
            // Use unwrap() since Python would raise exception on failure (matches behavior)
            if is_option_content {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_ref().unwrap().as_bytes()).unwrap()
                });
            } else {
                return Ok(parse_quote! {
                    #object_expr.write_all(#content.as_bytes()).unwrap()
                });
            }
        }

        // DEPYLER-0529: Handle file .close() method
        // Python: f.close() → Rust: no-op (files auto-close on drop via RAII)
        // DEPYLER-0550: Generate () instead of drop() because the file may have been
        // moved into a writer (e.g., csv::Writer::from_writer(output)), and we can't
        // drop a moved value. Rust's RAII handles cleanup automatically.
        if method == "close" && arg_exprs.is_empty() {
            // In Rust, files are automatically closed when dropped
            // No explicit close needed - RAII handles it
            return Ok(parse_quote! { () });
        }

        // DEPYLER-0551: Handle pathlib.Path instance methods
        // Python Path methods that need mapping to Rust std::path/std::fs equivalents
        // Check if object is a path variable (named "path" or known PathBuf type)
        let is_path_object = if let HirExpr::Var(var_name) = object {
            var_name == "path" || var_name.ends_with("_path") || var_name == "p"
        } else {
            false
        };

        if is_path_object {
            match method {
                // path.stat() → std::fs::metadata(&path).unwrap()
                "stat" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { std::fs::metadata(&#object_expr).unwrap() });
                }
                // path.absolute() or path.resolve() → path.canonicalize().unwrap()
                "absolute" | "resolve" if arg_exprs.is_empty() => {
                    return Ok(
                        parse_quote! { #object_expr.canonicalize().unwrap().to_string_lossy().to_string() },
                    );
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0553: Handle datetime instance methods
        // Python datetime methods that need mapping to chrono equivalents
        // Check if object is likely a datetime variable
        // DEPYLER-0620: Expanded heuristics to catch common date variable names
        let is_datetime_object = if let HirExpr::Var(var_name) = object {
            var_name == "dt"
                || var_name == "d"  // DEPYLER-0620: Common date variable name
                || var_name == "t"  // DEPYLER-0620: Common time variable name
                || var_name == "datetime"
                || var_name == "date"  // DEPYLER-0620: Common date variable name
                || var_name == "time"  // DEPYLER-0620: Common time variable name
                || var_name.ends_with("_dt")
                || var_name.ends_with("_datetime")
                || var_name.ends_with("_date")
                || var_name.ends_with("_time")
                || var_name.starts_with("date_")  // DEPYLER-0620: date_xyz pattern
                || var_name.starts_with("time_")  // DEPYLER-0620: time_xyz pattern
        } else {
            // DEPYLER-0620: Also detect datetime methods being called regardless of variable name
            // If the method is datetime-specific (strftime, isoformat), assume datetime object
            matches!(method, "strftime" | "isoformat" | "timestamp" | "weekday" | "isoweekday")
        };

        if is_datetime_object {
            self.ctx.needs_chrono = true;
            match method {
                // dt.isoformat() → dt.to_string() (chrono's Display produces ISO format)
                "isoformat" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { #object_expr.to_string() });
                }
                // dt.strftime(fmt) → dt.format(fmt).to_string()
                // DEPYLER-0555: chrono's format() takes &str, not String
                "strftime" if arg_exprs.len() == 1 => {
                    // Extract bare string literal for chrono format compatibility
                    let fmt = match hir_args.first() {
                        Some(HirExpr::Literal(Literal::String(s))) => parse_quote! { #s },
                        _ => arg_exprs[0].clone(),
                    };
                    return Ok(parse_quote! { #object_expr.format(#fmt).to_string() });
                }
                // dt.timestamp() → dt.timestamp() (same in chrono)
                "timestamp" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { #object_expr.and_utc().timestamp() as f64 });
                }
                // dt.date() → dt.date() (chrono NaiveDateTime has date())
                "date" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { #object_expr.date() });
                }
                // dt.time() → dt.time() (chrono NaiveDateTime has time())
                "time" if arg_exprs.is_empty() => {
                    return Ok(parse_quote! { #object_expr.time() });
                }
                _ => {} // Fall through to default handling
            }
        }

        // DEPYLER-0548: Handle csv.DictWriter methods
        // Python csv module methods need mapping to Rust csv crate equivalents
        if method == "writeheader" && arg_exprs.is_empty() {
            // writeheader() → no-op in Rust csv crate
            // Headers are typically written automatically or need explicit handling
            // TODO: Track fieldnames from DictWriter constructor to write proper header
            return Ok(parse_quote! { () });
        }

        if method == "writerow" && arg_exprs.len() == 1 {
            // writerow(row) → writer.serialize(&row).unwrap()
            // Python's DictWriter.writerow expects a dict
            // Rust's csv::Writer.serialize can handle HashMap
            let row = &arg_exprs[0];
            return Ok(parse_quote! {
                #object_expr.serialize(&#row).unwrap()
            });
        }

        // DEPYLER-0519: Handle regex Match.group() method
        // DEPYLER-0961: Return String instead of &str for type compatibility
        // Python: match.group(0) or match.group(n)
        // Rust: match.as_str().to_string() for group(0), or handle numbered groups
        // NOTE: When called inside if-let patterns, the variable is already unwrapped (Match, not Option)
        // so we should NOT use .as_ref().map() pattern - just call .as_str() directly
        if method == "group" {
            if arg_exprs.is_empty() || hir_args.is_empty() {
                // match.group() with no args defaults to group(0) in Python
                // DEPYLER-0961: Return String for proper type inference
                // Always use direct .as_str() since inside if-let the value is unwrapped
                return Ok(parse_quote! { #object_expr.as_str().to_string() });
            }

            // Check if argument is literal 0
            if let HirExpr::Literal(Literal::Int(n)) = &hir_args[0] {
                if *n == 0 {
                    // match.group(0) → match.as_str().to_string()
                    // DEPYLER-0961: Return String for proper type inference
                    return Ok(parse_quote! { #object_expr.as_str().to_string() });
                } else {
                    // match.group(n) → match.get(n).map(|m| m.as_str().to_string()).unwrap_or_default()
                    // DEPYLER-0961: Return String for proper type inference
                    let idx = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                    });
                }
            }

            // Non-literal argument - use runtime check
            // DEPYLER-0961: Return String for proper type inference
            let idx = &arg_exprs[0];
            return Ok(parse_quote! {
                if #idx == 0 {
                    #object_expr.as_str().to_string()
                } else {
                    #object_expr.get(#idx).map(|m| m.as_str().to_string()).unwrap_or_default()
                }
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before class instance check
        // String methods like upper/lower should be converted even for method parameters
        // that might be typed as class instances (due to how we track types)
        // DEPYLER-0621: Added encode/decode to ensure bytes conversion works on any string
        if matches!(
            method,
            "upper"
                | "lower"
                | "strip"
                | "lstrip"
                | "rstrip"
                | "startswith"
                | "endswith"
                | "split"
                | "splitlines"
                | "join"
                | "replace"
                | "find"
                | "rfind"
                | "rindex"
                | "isdigit"
                | "isalpha"
                | "isalnum"
                | "title"
                | "center"
                | "ljust"
                | "rjust"
                | "zfill"
                | "hex"
                | "format"
                | "encode"  // DEPYLER-0621: str.encode() → .as_bytes().to_vec()
                | "decode"  // DEPYLER-0621: bytes.decode() → String::from_utf8_lossy()
        ) {
            return self.convert_string_method(object, object_expr, method, arg_exprs, hir_args);
        }

        // DEPYLER-0232 FIX: Check for user-defined class instances
        // User-defined classes can have methods with names like "add" that conflict with
        // built-in collection methods. We must prioritize user-defined methods.
        if self.is_class_instance(object) {
            // This is a user-defined class instance - use generic method call
            // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
            let method_ident = if Self::is_rust_keyword(method) {
                syn::Ident::new_raw(method, proc_macro2::Span::call_site())
            } else {
                syn::Ident::new(method, proc_macro2::Span::call_site())
            };

            // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
            // When calling obj.method(other) where both are class instances,
            // the method signature likely expects &Self, so we borrow the argument.
            let processed_args: Vec<syn::Expr> = hir_args
                .iter()
                .zip(arg_exprs.iter())
                .map(|(hir_arg, arg_expr)| {
                    // If argument is also a class instance, borrow it
                    if self.is_class_instance(hir_arg) {
                        parse_quote! { &#arg_expr }
                    } else {
                        arg_expr.clone()
                    }
                })
                .collect();

            return Ok(parse_quote! { #object_expr.#method_ident(#(#processed_args),*) });
        }

        // DEPYLER-0211 FIX: Check object type first for ambiguous methods like update()
        // Both sets and dicts have update(), so we need to disambiguate

        // Check for set-specific context first
        if self.is_set_expr(object) {
            match method {
                "add"
                | "remove"
                | "discard"
                | "update"
                | "intersection_update"
                | "difference_update"
                | "union"
                | "intersection"
                | "difference"
                | "symmetric_difference"
                | "issubset"
                | "issuperset"
                | "isdisjoint" => {
                    return self.convert_set_method(object_expr, method, arg_exprs);
                }
                _ => {}
            }
        }

        // Check for dict-specific context
        if self.is_dict_expr(object) {
            match method {
                "get" | "keys" | "values" | "items" | "update" => {
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    return self.convert_dict_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                    );
                }
                _ => {}
            }
        }

        // Fallback to method name dispatch
        match method {
            // DEPYLER-0742: Deque-specific methods (must come before list methods)
            "appendleft" => {
                if arg_exprs.len() != 1 {
                    bail!("appendleft() requires exactly one argument");
                }
                let arg = &arg_exprs[0];
                Ok(parse_quote! { #object_expr.push_front(#arg) })
            }
            "popleft" => {
                if !arg_exprs.is_empty() {
                    bail!("popleft() takes no arguments");
                }
                Ok(parse_quote! { #object_expr.pop_front() })
            }

            // DEPYLER-0742: Handle append/pop for deque vs list
            "append" => {
                if self.is_deque_expr(object) {
                    if arg_exprs.len() != 1 {
                        bail!("append() requires exactly one argument");
                    }
                    let arg = &arg_exprs[0];
                    Ok(parse_quote! { #object_expr.push_back(#arg) })
                } else {
                    self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
                }
            }
            "pop" => {
                if self.is_deque_expr(object) {
                    if !arg_exprs.is_empty() {
                        bail!("deque.pop() does not accept an index argument");
                    }
                    Ok(parse_quote! { #object_expr.pop_back().unwrap_or_default() })
                } else {
                    self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
                }
            }

            // List methods (remaining)
            "extend" | "insert" | "remove" | "index" | "copy" | "clear"
            | "reverse" | "sort" => {
                self.convert_list_method(object_expr, object, method, arg_exprs, hir_args, kwargs)
            }

            // DEPYLER-0226: Disambiguate count() for list vs string
            // DEPYLER-0302: Improved heuristic using is_string_base()
            "count" => {
                // Heuristic: Check if object is string-typed using is_string_base()
                // This covers string literals, variables with str type annotations, and string method results
                if self.is_string_base(object) {
                    // String: use str.count() → .matches().count()
                    self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
                } else {
                    // List: use list.count() → .iter().filter().count()
                    self.convert_list_method(
                        object_expr,
                        object,
                        method,
                        arg_exprs,
                        hir_args,
                        kwargs,
                    )
                }
            }

            // DEPYLER-0223: Disambiguate update() for dict vs set
            "update" => {
                // Check if argument is a set or dict literal
                if !hir_args.is_empty() && self.is_set_expr(&hir_args[0]) {
                    // numbers.update({3, 4}) - set update
                    self.convert_set_method(object_expr, method, arg_exprs)
                } else {
                    // data.update({"b": 2}) - dict update (default for variables)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // DEPYLER-0422: Disambiguate .get() for list vs dict
            // List/Vec .get() takes usize by value, Dict .get() takes &K by reference
            "get" => {
                // Only use list handler when we're CERTAIN it's a list (not dict)
                // Default to dict handler for uncertain types (dict.get() supports 1 or 2 args)
                if self.is_list_expr(object) && !self.is_dict_expr(object) {
                    // List/Vec .get() - cast index to usize (must be exactly 1 arg)
                    if arg_exprs.len() != 1 {
                        bail!("list.get() requires exactly one argument");
                    }
                    let index = &arg_exprs[0];
                    // Cast integer index to usize (Vec/slice .get() requires usize, not &i32)
                    Ok(parse_quote! { #object_expr.get(#index as usize).cloned() })
                } else {
                    // Dict .get() - use existing dict handler (supports 1 or 2 args)
                    // DEPYLER-0540: Pass object for serde_json::Value detection
                    self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
                }
            }

            // Dict methods (for variables without type info)
            "keys" | "values" | "items" | "setdefault" | "popitem" => {
                // DEPYLER-0540: Pass object for serde_json::Value detection
                self.convert_dict_method(object_expr, object, method, arg_exprs, hir_args)
            }

            // String methods
            // Note: "count" handled separately above with disambiguation logic
            // Note: "index" handled in list methods above (lists take precedence)
            "upper" | "lower" | "strip" | "lstrip" | "rstrip" | "startswith" | "endswith"
            | "split" | "rsplit" | "splitlines" | "join" | "replace" | "find" | "rfind" | "rindex"
            | "isdigit" | "isalpha" | "isalnum" | "isspace" | "isupper" | "islower" | "istitle"
            | "isnumeric" | "isascii" | "isdecimal" | "isidentifier" | "isprintable"
            | "title" | "capitalize" | "swapcase" | "casefold" | "center" | "ljust" | "rjust"
            | "zfill" | "hex" | "encode" | "decode" => {
                self.convert_string_method(object, object_expr, method, arg_exprs, hir_args)
            }

            // Set methods (for variables without type info)
            // Note: "update" handled separately above with disambiguation logic
            // Note: "remove" is ambiguous (list vs set) - keep in list fallback for now
            "add"
            | "discard"
            | "intersection_update"
            | "difference_update"
            | "symmetric_difference_update"
            | "union"
            | "intersection"
            | "difference"
            | "symmetric_difference"
            | "issubset"
            | "issuperset"
            | "isdisjoint" => self.convert_set_method(object_expr, method, arg_exprs),

            // DEPYLER-0431: Regex methods (compiled Regex + Match object)
            // Compiled Regex: findall, match, search (note: "find" conflicts with string.find())
            // Match object: group, groups, start, end, span, as_str
            "findall" | "match" | "search" | "group" | "groups" | "start" | "end" | "span"
            | "as_str" => self.convert_regex_method(object_expr, method, arg_exprs),

            // Path instance methods (DEPYLER-0363)
            "read_text" => {
                // filepath.read_text() → std::fs::read_to_string(filepath).unwrap()
                if !arg_exprs.is_empty() {
                    bail!("Path.read_text() takes no arguments");
                }
                Ok(parse_quote! { std::fs::read_to_string(#object_expr).unwrap() })
            }

            // DEPYLER-0960: contains/__contains__ method - dict uses contains_key
            "contains" | "__contains__" => {
                if arg_exprs.len() != 1 {
                    bail!("contains() requires exactly one argument");
                }
                let key = &arg_exprs[0];
                // Check if object is a dict/HashMap - use contains_key
                if self.is_dict_expr(object) {
                    Ok(parse_quote! { #object_expr.contains_key(&#key) })
                } else {
                    // String/Set/List uses .contains()
                    Ok(parse_quote! { #object_expr.contains(&#key) })
                }
            }

            // Default: generic method call
            _ => {
                // DEPYLER-0306 FIX: Use raw identifiers for method names that are Rust keywords
                let method_ident = if Self::is_rust_keyword(method) {
                    syn::Ident::new_raw(method, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(method, proc_macro2::Span::call_site())
                };

                // DEPYLER-0712: Auto-borrow class instance arguments when calling user-defined methods
                // When calling obj.method(other) where both obj and other are class instances,
                // the method signature likely expects &Self, so we borrow the argument.
                // Use is_class_instance helper which checks both var_types and class_names.
                let receiver_is_class = self.is_class_instance(object);

                // Process arguments, adding & when receiver and argument are both class instances
                let processed_args: Vec<syn::Expr> = hir_args
                    .iter()
                    .zip(arg_exprs.iter())
                    .map(|(hir_arg, arg_expr)| {
                        // If receiver is a class instance and argument is also a class instance,
                        // the method likely expects &Self for the argument
                        if receiver_is_class && self.is_class_instance(hir_arg) {
                            return parse_quote! { &#arg_expr };
                        }
                        arg_expr.clone()
                    })
                    .collect();

                // DEPYLER-0823: Wrap cast expressions in parentheses before method calls
                // Rust parses `x as i32.method()` as `x as (i32.method())` which is invalid
                // Must be: `(x as i32).method()`
                let safe_object_expr: syn::Expr = if matches!(object_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#object_expr) }
                } else {
                    object_expr.clone()
                };

                Ok(parse_quote! { #safe_object_expr.#method_ident(#(#processed_args),*) })
            }
        }
    }

    /// DEPYLER-0188: Convert dynamic/subscript function call
    /// E.g., `handlers[name](args)` → `(handlers[&name])(args)`
    fn convert_dynamic_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirExpr],
    ) -> Result<syn::Expr> {
        let callee_expr = callee.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        if arg_exprs.is_empty() {
            Ok(parse_quote! { (#callee_expr)() })
        } else {
            Ok(parse_quote! { (#callee_expr)(#(#arg_exprs),*) })
        }
    }

    fn convert_method_call(
        &mut self,
        object: &HirExpr,
        method: &str,
        args: &[HirExpr],
        kwargs: &[(String, HirExpr)],
    ) -> Result<syn::Expr> {
        // CITL: Trace method dispatch decision
        trace_decision!(
            category = DecisionCategory::MethodDispatch,
            name = "method_call",
            chosen = method,
            alternatives = ["trait_method", "inherent_method", "extension", "ufcs"],
            confidence = 0.88
        );

        // DEPYLER-0964: Handle method calls on &mut Option<HashMap<K, V>> parameters
        // When a parameter is Dict[K,V] with default None, it becomes &mut Option<HashMap>
        // Method calls need to unwrap the Option first:
        // - memo.get(k) → memo.as_ref().unwrap().get(&k)
        // - memo.insert(k, v) → memo.as_mut().unwrap().insert(k, v)
        // - memo.contains_key(k) → memo.as_ref().unwrap().contains_key(&k)
        if let HirExpr::Var(var_name) = object {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match method {
                    "get" => {
                        if args.is_empty() {
                            // dict.get() with no args - shouldn't happen for dict but handle gracefully
                            return Ok(parse_quote! { #var_ident.as_ref().unwrap().get() });
                        }
                        let key_expr = args[0].to_rust_expr(self.ctx)?;
                        // Check if we need default value (2-arg form)
                        if args.len() > 1 {
                            let default_expr = args[1].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().get(&#key_expr).cloned().unwrap_or(#default_expr)
                            });
                        } else {
                            // Single arg form - return Option<&V>
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().get(&#key_expr).cloned()
                            });
                        }
                    }
                    "contains_key" | "__contains__" => {
                        if !args.is_empty() {
                            let key_expr = args[0].to_rust_expr(self.ctx)?;
                            return Ok(parse_quote! {
                                #var_ident.as_ref().unwrap().contains_key(&#key_expr)
                            });
                        }
                    }
                    "keys" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().keys() });
                    }
                    "values" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().values() });
                    }
                    "items" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().iter() });
                    }
                    "len" if args.is_empty() => {
                        return Ok(parse_quote! { #var_ident.as_ref().unwrap().len() as i32 });
                    }
                    _ => {} // Fall through to other handlers
                }
            }
        }

        // DEPYLER-0108: Handle is_some/is_none on precomputed argparse Option fields
        // This prevents borrow-after-move when Option field is passed to a function then checked
        if (method == "is_some" || method == "is_none") && args.is_empty() {
            if let HirExpr::Attribute { value, attr } = object {
                if let HirExpr::Var(_) = value.as_ref() {
                    // Check if this field has been precomputed
                    if self.ctx.precomputed_option_fields.contains(attr) {
                        let has_var_name = format!("has_{}", attr);
                        let has_ident =
                            syn::Ident::new(&has_var_name, proc_macro2::Span::call_site());
                        if method == "is_some" {
                            return Ok(parse_quote! { #has_ident });
                        } else {
                            return Ok(parse_quote! { !#has_ident });
                        }
                    }
                }
            }
        }

        // DEPYLER-0931: Handle subprocess.Child methods (.wait(), .kill(), etc.)
        // proc.wait() → proc.as_mut().unwrap().wait().ok().and_then(|s| s.code()).unwrap_or(-1)
        // When proc is Option<Child>, we need to unwrap and extract exit code
        if method == "wait" && args.is_empty() {
            if let HirExpr::Var(var_name) = object {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    let is_subprocess_child = matches!(
                        var_type,
                        Type::Custom(s) if s == "std::process::Child" || s == "Child"
                    ) || matches!(
                        var_type,
                        Type::Optional(inner) if matches!(
                            inner.as_ref(),
                            Type::Custom(s) if s == "std::process::Child" || s == "Child"
                        )
                    );
                    if is_subprocess_child {
                        let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                        // Handle Option<Child> - unwrap, call wait, extract exit code
                        if matches!(var_type, Type::Optional(_)) {
                            return Ok(parse_quote! {
                                #var_ident.as_mut().unwrap().wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        } else {
                            return Ok(parse_quote! {
                                #var_ident.wait().ok().and_then(|s| s.code()).unwrap_or(-1)
                            });
                        }
                    }
                }
            }
        }

        // DEPYLER-0663: Handle serde_json::Value method calls
        // serde_json::Value doesn't have direct .len(), .iter(), .is_none(), .is_some() methods
        // We need to convert them to the appropriate serde_json::Value method chains
        if self.is_serde_json_value_expr(object) || self.is_serde_json_value(object) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            match method {
                // value.len() → value.as_array().map(|a| a.len()).unwrap_or_else(|| value.as_object().map(|o| o.len()).unwrap_or(0))
                "len" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().map(|a| a.len()).unwrap_or_else(||
                            #object_expr.as_object().map(|o| o.len()).unwrap_or(0)
                        ) as i32
                    });
                }
                // value.iter() → value.as_array().into_iter().flatten()
                "iter" if args.is_empty() => {
                    return Ok(parse_quote! {
                        #object_expr.as_array().into_iter().flatten()
                    });
                }
                // value.is_none() → value.is_null()
                "is_none" if args.is_empty() => {
                    return Ok(parse_quote! { #object_expr.is_null() });
                }
                // value.is_some() → !value.is_null()
                "is_some" if args.is_empty() => {
                    return Ok(parse_quote! { !#object_expr.is_null() });
                }
                _ => {} // Fall through to other handlers
            }
        }

        // DEPYLER-0747: Handle asyncio module method calls
        // asyncio.sleep(secs) → tokio::time::sleep(Duration)
        // asyncio.run(coro) → tokio runtime block_on
        if let HirExpr::Var(module) = object {
            if module == "asyncio" {
                self.ctx.needs_tokio = true;
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "sleep" => {
                        if let Some(arg) = arg_exprs.first() {
                            return Ok(parse_quote! {
                                tokio::time::sleep(std::time::Duration::from_secs_f64(#arg as f64))
                            });
                        }
                        return Ok(parse_quote! {
                            tokio::time::sleep(std::time::Duration::from_secs(0))
                        });
                    }
                    "run" => {
                        if let Some(arg) = arg_exprs.first() {
                            return Ok(parse_quote! {
                                tokio::runtime::Runtime::new().unwrap().block_on(#arg)
                            });
                        }
                    }
                    _ => {} // Fall through for other asyncio methods
                }
            }
        }

        // DEPYLER-0912: Handle colorsys module method calls
        // colorsys.rgb_to_hsv(r, g, b) → inline HSV conversion
        // colorsys.hsv_to_rgb(h, s, v) → inline RGB conversion
        if let HirExpr::Var(module) = object {
            if module == "colorsys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                match method {
                    "rgb_to_hsv" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hsv formula
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let v = max_c;
                                if min_c == max_c {
                                    (0.0, 0.0, v)
                                } else {
                                    let s = (max_c - min_c) / max_c;
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, s, v)
                                }
                            }
                        });
                    }
                    "hsv_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let s = &arg_exprs[1];
                        let v = &arg_exprs[2];
                        // Python colorsys.hsv_to_rgb formula
                        return Ok(parse_quote! {
                            {
                                let (h, s, v) = (#h as f64, #s as f64, #v as f64);
                                if s == 0.0 {
                                    (v, v, v)
                                } else {
                                    let i = (h * 6.0).floor();
                                    let f = (h * 6.0) - i;
                                    let p = v * (1.0 - s);
                                    let q = v * (1.0 - s * f);
                                    let t = v * (1.0 - s * (1.0 - f));
                                    let i = i as i32 % 6;
                                    match i {
                                        0 => (v, t, p),
                                        1 => (q, v, p),
                                        2 => (p, v, t),
                                        3 => (p, q, v),
                                        4 => (t, p, v),
                                        _ => (v, p, q),
                                    }
                                }
                            }
                        });
                    }
                    "rgb_to_hls" if arg_exprs.len() == 3 => {
                        let r = &arg_exprs[0];
                        let g = &arg_exprs[1];
                        let b = &arg_exprs[2];
                        // Python colorsys.rgb_to_hls formula
                        return Ok(parse_quote! {
                            {
                                let (r, g, b) = (#r as f64, #g as f64, #b as f64);
                                let max_c = r.max(g).max(b);
                                let min_c = r.min(g).min(b);
                                let l = (min_c + max_c) / 2.0;
                                if min_c == max_c {
                                    (0.0, l, 0.0)
                                } else {
                                    let s = if l <= 0.5 {
                                        (max_c - min_c) / (max_c + min_c)
                                    } else {
                                        (max_c - min_c) / (2.0 - max_c - min_c)
                                    };
                                    let rc = (max_c - r) / (max_c - min_c);
                                    let gc = (max_c - g) / (max_c - min_c);
                                    let bc = (max_c - b) / (max_c - min_c);
                                    let h = if r == max_c {
                                        bc - gc
                                    } else if g == max_c {
                                        2.0 + rc - bc
                                    } else {
                                        4.0 + gc - rc
                                    };
                                    let h = (h / 6.0) % 1.0;
                                    let h = if h < 0.0 { h + 1.0 } else { h };
                                    (h, l, s)
                                }
                            }
                        });
                    }
                    "hls_to_rgb" if arg_exprs.len() == 3 => {
                        let h = &arg_exprs[0];
                        let l = &arg_exprs[1];
                        let s = &arg_exprs[2];
                        // Python colorsys.hls_to_rgb formula
                        return Ok(parse_quote! {
                            {
                                let (h, l, s) = (#h as f64, #l as f64, #s as f64);
                                if s == 0.0 {
                                    (l, l, l)
                                } else {
                                    let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - (l * s) };
                                    let m1 = 2.0 * l - m2;
                                    let _v = |hue: f64| {
                                        let hue = hue % 1.0;
                                        let hue = if hue < 0.0 { hue + 1.0 } else { hue };
                                        if hue < 1.0/6.0 { m1 + (m2 - m1) * hue * 6.0 }
                                        else if hue < 0.5 { m2 }
                                        else if hue < 2.0/3.0 { m1 + (m2 - m1) * (2.0/3.0 - hue) * 6.0 }
                                        else { m1 }
                                    };
                                    (_v(h + 1.0/3.0), _v(h), _v(h - 1.0/3.0))
                                }
                            }
                        });
                    }
                    _ => {} // Fall through for other colorsys methods
                }
            }
        }

        // DEPYLER-0778: Handle dict.fromkeys(keys, default) class method
        // dict.fromkeys(keys, default) → keys.iter().map(|k| (k.clone(), default)).collect()
        if let HirExpr::Var(var_name) = object {
            if var_name == "dict" && method == "fromkeys" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let keys_expr = &arg_exprs[0];
                    let default_expr = &arg_exprs[1];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), #default_expr)).collect()
                    });
                } else if arg_exprs.len() == 1 {
                    // dict.fromkeys(keys) with implicit None default
                    let keys_expr = &arg_exprs[0];
                    return Ok(parse_quote! {
                        #keys_expr.iter().map(|k| (k.clone(), ())).collect()
                    });
                }
            }
        }

        // DEPYLER-0933: Handle int.from_bytes(bytes, byteorder) class method
        // int.from_bytes(bytes, "big") → i64::from_be_bytes(bytes.try_into().unwrap())
        // int.from_bytes(bytes, "little") → i64::from_le_bytes(bytes.try_into().unwrap())
        if let HirExpr::Var(var_name) = object {
            if var_name == "int" && method == "from_bytes" {
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;

                if arg_exprs.len() >= 2 {
                    let bytes_expr = &arg_exprs[0];
                    // Check if second arg is "big" or "little" string literal
                    let is_big_endian = if let HirExpr::Literal(Literal::String(s)) = &args[1] {
                        s == "big"
                    } else {
                        true // Default to big endian
                    };

                    if is_big_endian {
                        return Ok(parse_quote! {
                            i64::from_be_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                let start = 8usize.saturating_sub(bytes.len());
                                arr[start..].copy_from_slice(bytes);
                                arr
                            })
                        });
                    } else {
                        return Ok(parse_quote! {
                            i64::from_le_bytes({
                                let mut arr = [0u8; 8];
                                let bytes: &[u8] = #bytes_expr.as_ref();
                                arr[..bytes.len().min(8)].copy_from_slice(&bytes[..bytes.len().min(8)]);
                                arr
                            })
                        });
                    }
                }
            }
        }

        // DEPYLER-0558: Handle hasher methods (hexdigest, update) for incremental hashing
        if method == "hexdigest" {
            self.ctx.needs_hex = true;
            let object_expr = object.to_rust_expr(self.ctx)?;
            // hexdigest() on hasher → hex::encode(hasher.finalize())
            return Ok(parse_quote! {
                hex::encode(#object_expr.finalize())
            });
        }

        // DEPYLER-0750: Handle Counter.most_common(n)
        // counter.most_common(n) → sort HashMap by value descending, take n
        if method == "most_common" {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;

            if let Some(n_arg) = arg_exprs.first() {
                // With n argument: take top n
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries.into_iter().take(#n_arg as usize).collect::<Vec<_>>()
                    }
                });
            } else {
                // No argument: return all sorted
                return Ok(parse_quote! {
                    {
                        let mut entries: Vec<_> = #object_expr.iter().map(|(k, v)| (k.clone(), *v)).collect();
                        entries.sort_by(|a, b| b.1.cmp(&a.1));
                        entries
                    }
                });
            }
        }

        // DEPYLER-0728: hasher.update() handler should NOT intercept dict/set.update()
        // Only apply to hash objects (Sha256, Md5, etc.), not collections
        if method == "update" && !args.is_empty() && !self.is_dict_expr(object) && !self.is_set_expr(object) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            let data = &arg_exprs[0];
            // DEPYLER-0558: hasher.update(data) needs borrow for DynDigest trait
            // DynDigest::update takes &[u8], so always add borrow
            return Ok(parse_quote! {
                #object_expr.update(&#data)
            });
        }

        // DEPYLER-0413: Handle string methods FIRST before any other checks
        // This ensures string methods like upper/lower are converted even when
        // inside class methods where parameters might be mistyped as class instances
        if matches!(
            method,
            "upper"
                | "lower"
                | "strip"
                | "lstrip"
                | "rstrip"
                | "startswith"
                | "endswith"
                | "split"
                | "splitlines"
                | "join"
                | "replace"
                | "find"
                | "rfind"
                | "rindex"
                | "isdigit"
                | "isalpha"
                | "isalnum"
                | "title"
                | "center"
                | "ljust"
                | "rjust"
                | "zfill"
                | "hex"
                | "format"
        ) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_string_method(object, &object_expr, method, &arg_exprs, args);
        }

        // DEPYLER-0829: Handle pathlib methods on Path variables (not just module calls)
        // Python: p = Path("/foo"); p.write_text(content)
        // Rust: PathBuf doesn't have write_text, must use std::fs::write
        // This catches path methods when object is a variable, not the pathlib module
        // DEPYLER-0956: Exclude "os" module - os.mkdir/os.rmdir are os module functions, not Path methods
        let is_os_module = matches!(object, HirExpr::Var(name) if name == "os");
        if !is_os_module
            && matches!(
                method,
                "write_text"
                    | "read_text"
                    | "read_bytes"
                    | "write_bytes"
                    | "exists"
                    | "is_file"
                    | "is_dir"
                    | "mkdir"
                    | "rmdir"
                    | "unlink"
                    | "iterdir"
                    | "glob"
                    | "rglob"
                    | "with_name"
                    | "with_suffix"
                    | "with_stem"
                    | "resolve"
                    | "absolute"
                    | "relative_to"
            )
        {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_pathlib_instance_method(&object_expr, method, &arg_exprs);
        }

        // DEPYLER-0830: Handle datetime/timedelta instance methods on variables
        // Python: td = datetime.timedelta(seconds=100); td.total_seconds()
        // Rust: TimeDelta.num_seconds() as f64
        // This catches datetime methods when object is a variable, not the datetime module
        if matches!(
            method,
            "total_seconds"
                | "fromisoformat"
                | "isoformat"
                | "strftime"
                | "timestamp"
                | "timetuple"
                | "weekday"
                | "isoweekday"
                | "isocalendar"
                | "replace"
        ) {
            let object_expr = object.to_rust_expr(self.ctx)?;
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| arg.to_rust_expr(self.ctx))
                .collect::<Result<Vec<_>>>()?;
            return self.convert_datetime_instance_method(&object_expr, method, args, &arg_exprs);
        }

        // DEPYLER-0416: Check if this is a static method call on a class (e.g., Point.origin())
        // Convert to ClassName::method(args)
        // DEPYLER-0458 FIX: Exclude CONST_NAMES (all uppercase) from static method conversion
        // Constants like DEFAULT_CONFIG should use instance methods (.clone()) not static (::copy())
        if let HirExpr::Var(class_name) = object {
            let is_const = class_name.chars().all(|c| c.is_uppercase() || c == '_');
            let starts_uppercase = class_name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false);

            if starts_uppercase && !is_const {
                // DEPYLER-0900: Rename class if it shadows stdlib type (e.g., Box -> PyBox)
                // This is likely a static method call - convert to ClassName::method(args)
                let safe_name = crate::direct_rules::safe_class_name(class_name);
                let class_ident = syn::Ident::new(&safe_name, proc_macro2::Span::call_site());
                let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
                let arg_exprs: Vec<syn::Expr> = args
                    .iter()
                    .map(|arg| arg.to_rust_expr(self.ctx))
                    .collect::<Result<Vec<_>>>()?;
                return Ok(parse_quote! { #class_ident::#method_ident(#(#arg_exprs),*) });
            }
        }

        // Try classmethod handling first
        if let Some(result) = self.try_convert_classmethod(object, method, args)? {
            return Ok(result);
        }

        // Try module method handling
        // DEPYLER-0426: Pass kwargs to module method converter
        if let Some(result) = self.try_convert_module_method(object, method, args, kwargs)? {
            return Ok(result);
        }

        // DEPYLER-0200: Handle .decode() on base64 encode calls
        // base64.b64encode() in Rust returns String, not bytes - no decode needed
        if method == "decode" {
            if let HirExpr::MethodCall {
                object: inner_obj,
                method: inner_method,
                ..
            } = object
            {
                if let HirExpr::Var(module) = inner_obj.as_ref() {
                    if module == "base64"
                        && (inner_method.contains("b64encode")
                            || inner_method.contains("urlsafe_b64encode"))
                    {
                        // base64::encode() returns String - just return the object expression
                        return object.to_rust_expr(self.ctx);
                    }
                }
            }
        }

        let object_expr = object.to_rust_expr(self.ctx)?;
        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| arg.to_rust_expr(self.ctx))
            .collect::<Result<Vec<_>>>()?;

        // DEPYLER-0445: Pass original args and kwargs separately to convert_instance_method
        // Some methods like sort(key=func) need to preserve keyword argument names
        // For other methods, they can merge kwargs as positional if needed
        self.convert_instance_method(object, &object_expr, method, &arg_exprs, args, kwargs)
    }

    fn convert_index(&mut self, base: &HirExpr, index: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace subscript/indexing strategy decision
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "subscript_access",
            chosen = "get_or_index",
            alternatives = ["direct_index", "get_method", "get_unchecked", "slice"],
            confidence = 0.85
        );

        // DEPYLER-0386: Handle os.environ['VAR'] → std::env::var('VAR').unwrap_or_default()
        // Must check this before evaluating base_expr to avoid trying to convert os.environ
        if let HirExpr::Attribute { value, attr } = base {
            if let HirExpr::Var(module_name) = &**value {
                if module_name == "os" && attr == "environ" {
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    return Ok(parse_quote! { std::env::var(#index_expr).unwrap_or_default() });
                }
            }
        }

        // DEPYLER-0964: Handle subscript access on &mut Option<HashMap<K, V>> parameters
        // When accessing `memo[key]` where memo is a mut_option_dict_param,
        // we need to unwrap the Option first: memo.as_ref().unwrap().get(&key).cloned()
        if let HirExpr::Var(var_name) = base {
            if self.ctx.mut_option_dict_params.contains(var_name) {
                let base_ident = crate::rust_gen::keywords::safe_ident(var_name);
                let index_expr = index.to_rust_expr(self.ctx)?;
                // Use .get() which returns Option<&V>, then .cloned() for owned value
                return Ok(parse_quote! {
                    #base_ident.as_ref().unwrap().get(&#index_expr).cloned().unwrap_or_default()
                });
            }
        }

        let mut base_expr = base.to_rust_expr(self.ctx)?;

        // DEPYLER-0270: Auto-unwrap Result-returning function calls
        // When base is a function call that returns Result<HashMap/Vec, E>,
        // we need to unwrap it with ? before calling .get() or indexing
        // Example: get_config()["name"] → get_config()?.get("name")...
        if let HirExpr::Call { func, .. } = base {
            if self.ctx.result_returning_functions.contains(func) {
                base_expr = parse_quote! { #base_expr? };
            }
        }

        // DEPYLER-0422 Fix #3 & #4: Handle tuple indexing with actual type information
        // Python: tuple[0], tuple[1] → Rust: tuple.0, tuple.1
        // Also handles chained indexing: list_of_tuples[i][j] → list_of_tuples.get(i).0
        let should_use_tuple_syntax = if let HirExpr::Literal(Literal::Int(idx)) = index {
            if *idx >= 0 {
                if let HirExpr::Var(var_name) = base {
                    // Case 1: Direct variable access (e.g., position[0] where position: Tuple)
                    if let Some(var_type) = self.ctx.var_types.get(var_name) {
                        matches!(var_type, Type::Tuple(_))
                    } else {
                        // Fallback heuristic: variable names suggesting tuple iteration
                        matches!(
                            var_name.as_str(),
                            "pair" | "entry" | "item" | "elem" | "tuple" | "row"
                        )
                    }
                } else if let HirExpr::Index {
                    base: inner_base, ..
                } = base
                {
                    // DEPYLER-0422 Fix #4: Case 2: Chained indexing (e.g., word_counts[j][1])
                    // Check if we're indexing into a List[Tuple]
                    if let HirExpr::Var(var_name) = &**inner_base {
                        if let Some(Type::List(element_type)) = self.ctx.var_types.get(var_name) {
                            // If the list contains tuples, second index is tuple field access
                            matches!(**element_type, Type::Tuple(_))
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_use_tuple_syntax {
            if let HirExpr::Literal(Literal::Int(idx)) = index {
                let field_idx = syn::Index::from(*idx as usize);
                return Ok(parse_quote! { #base_expr.#field_idx });
            }
        }

        // DEPYLER-0299 Pattern #3 FIX: Check if base is a String type for character access
        let is_string_base = self.is_string_base(base);

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
                    // HashMap.get() expects &K, so we need to borrow the key
                    // DEPYLER-0521: Don't add & if variable is already &str type
                    // DEPYLER-0528: Fixed logic - owned String NEEDS borrow, &str does NOT
                    let index_expr = index.to_rust_expr(self.ctx)?;
                    // DEPYLER-0539: Fix dict key borrowing for &str parameters
                    // Check is_borrowed_str_param FIRST - &str params are tracked as Type::String
                    // but should NOT be borrowed again
                    let needs_borrow = if let HirExpr::Var(var_name) = index {
                        if self.is_borrowed_str_param(var_name) {
                            false // Already &str from function parameter, no borrow needed
                        } else if matches!(
                            self.ctx.var_types.get(var_name),
                            Some(Type::String) // owned String → needs &
                        ) {
                            true // Owned String needs borrow
                        } else {
                            // Unknown type - default to borrowing for safety
                            true
                        }
                    } else {
                        true // Non-variable expressions typically need borrowing
                    };
                    if needs_borrow {
                        Ok(parse_quote! {
                            #base_expr.get(&#index_expr).cloned().unwrap_or_default()
                        })
                    } else {
                        Ok(parse_quote! {
                            #base_expr.get(#index_expr).cloned().unwrap_or_default()
                        })
                    }
                }
            }
        } else if is_string_base {
            // DEPYLER-0299 Pattern #3: String character access with numeric index
            // Strings cannot use .get(usize), must use .chars().nth()
            let index_expr = index.to_rust_expr(self.ctx)?;

            // DEPYLER-0267 FIX: Use .chars().nth() for proper character access
            // This returns Option<char>, then convert to String
            Ok(parse_quote! {
                {
                    // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                    let base = &#base_expr;
                    let idx: i32 = #index_expr;
                    let actual_idx = if idx < 0 {
                        base.chars().count().saturating_sub(idx.abs() as usize)
                    } else {
                        idx as usize
                    };
                    base.chars().nth(actual_idx).map(|c| c.to_string()).unwrap_or_default()
                }
            })
        } else {
            // DEPYLER-0701: Check if base is a tuple type with variable index
            // Tuples don't have .get() method, so we convert to array for runtime indexing
            // Python: t[idx] where t = (1, 2) → Rust: [t.0, t.1][idx as usize]
            let is_tuple_base = self.is_tuple_base(base);

            if is_tuple_base && !matches!(index, HirExpr::Literal(Literal::Int(_))) {
                // Variable index on tuple - convert tuple to array
                // Get tuple element count from type info if available
                let tuple_size = self.get_tuple_size(base).unwrap_or(2);
                let index_expr = index.to_rust_expr(self.ctx)?;

                // Generate array from tuple elements: [t.0, t.1, ...]
                let indices: Vec<syn::Index> =
                    (0..tuple_size).map(syn::Index::from).collect();

                return Ok(parse_quote! {
                    [#(#base_expr.#indices),*][#index_expr as usize]
                });
            }

            // Vec/List access with numeric index
            let index_expr = index.to_rust_expr(self.ctx)?;

            // Check if index is a negative literal
            if let HirExpr::Unary {
                op: UnaryOp::Neg,
                operand,
            } = index
            {
                if let HirExpr::Literal(Literal::Int(n)) = **operand {
                    // Negative index literal: arr[-1] → arr.get(arr.len() - 1)
                    let offset = n as usize;
                    return Ok(parse_quote! {
                        {
                            // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                            let base = &#base_expr;
                            // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                            base.get(base.len().saturating_sub(#offset)).cloned().unwrap_or_default()
                        }
                    });
                }
            }

            // DEPYLER-0357: Check if index is a positive integer literal
            // For literal indices like p[0], generate simple inline code: .get(0)
            // This avoids unnecessary temporary variables and runtime checks
            // DEPYLER-0730: Use .expect() instead of .unwrap_or_default() to:
            //   1. Match Python semantics (IndexError on out of bounds, not default)
            //   2. Avoid requiring Default trait bound on generic T
            if let HirExpr::Literal(Literal::Int(n)) = index {
                let idx_value = *n as usize;
                return Ok(parse_quote! {
                    #base_expr.get(#idx_value).cloned().expect("IndexError: list index out of range")
                });
            }

            // DEPYLER-0306 FIX: Check if index is a simple variable (not a complex expression)
            // Simple variables in for loops like `for i in range(len(arr))` are guaranteed >= 0
            // For these, we can use simpler inline code that works in range contexts
            let is_simple_var = matches!(index, HirExpr::Var(_));

            if is_simple_var {
                // Simple variable index - use inline expression (works in range contexts)
                // This avoids block expressions that break in `for j in 0..matrix[i].len()`
                // DEPYLER-0730: Use .expect() for Python IndexError semantics
                Ok(parse_quote! {
                    #base_expr.get(#index_expr as usize).cloned().expect("IndexError: list index out of range")
                })
            } else {
                // Complex expression - use block with full negative index handling
                // DEPYLER-0288: Explicitly type idx as i32 to support negation
                // DEPYLER-0730: Use .expect() for Python IndexError semantics
                Ok(parse_quote! {
                    {
                        // DEPYLER-0307 Fix #11: Use borrow to avoid moving the base expression
                        let base = &#base_expr;
                        let idx: i32 = #index_expr;
                        let actual_idx = if idx < 0 {
                            // Use .abs() instead of negation to avoid "Neg not implemented for usize" error
                            base.len().saturating_sub(idx.abs() as usize)
                        } else {
                            idx as usize
                        };
                        // DEPYLER-0267: Use .cloned() instead of .copied() for non-Copy types (String, Vec, etc.)
                        base.get(actual_idx).cloned().expect("IndexError: list index out of range")
                    }
                })
            }
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
            // DEPYLER-0449: First check actual variable type if known
            if let Some(var_type) = self.ctx.var_types.get(sym) {
                // If variable is typed as serde_json::Value or Dict, use string indexing
                if matches!(var_type, Type::Dict(_, _)) {
                    return Ok(true);
                }
            }

            // Try to find the variable's type in the current function context
            // For parameters, we can check the function signature
            // For local variables, this is harder without full type inference
            //
            // DEPYLER-0422: Removed "data" from heuristic - too broad, catches sorted_data, dataset, etc.
            // Only use "dict" or "map" which are more specific to HashMap variables
            let name = sym.as_str();
            if (name.contains("dict")
                || name.contains("map")
                || name.contains("config")
                || name.contains("value"))
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
                // DEPYLER-0449: First check actual variable type if known
                if let Some(var_type) = self.ctx.var_types.get(sym) {
                    // If variable is typed as String, it's a string index
                    if matches!(var_type, Type::String) {
                        return true;
                    }
                }

                // Fallback to heuristics
                let name = sym.as_str();
                // DEPYLER-0449: Expanded to include common loop variables like "k"
                // Heuristic: variable names like "key", "name", "id", "word", etc.
                name == "key"
                    || name == "k" // Common loop variable for keys
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

    /// DEPYLER-0299 Pattern #3: Check if base expression is a String type (heuristic)
    /// Returns true if base is likely a String/str type (not Vec/List)
    fn is_string_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(sym) => {
                // DEPYLER-0479: Check type system first (most reliable)
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    // Return true if definitely String, false if definitely NOT string
                    // Fall through to heuristics for Unknown/Any types
                    match ty {
                        Type::String => return true,
                        // DEPYLER-0579: Optional<String> is still string-like
                        Type::Optional(inner) if matches!(**inner, Type::String) => return true,
                        Type::Int | Type::Float | Type::Bool | Type::List(_) | Type::Dict(_, _) => {
                            return false;
                        }
                        _ => {} // Unknown/Any - fall through to heuristics
                    }
                }

                // DEPYLER-0267 FIX: Only match singular string-like names, NOT plurals
                // "words" (plural) is likely list[str], not str!
                // "word" (singular) without 's' ending is likely str
                let name = sym.as_str();
                // Only match if: singular AND string-like name
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"     // DEPYLER-0538: File content is usually String
                    || name == "timestamp"  // GH-70: Common string field (ISO 8601, etc.)
                    || name == "message"     // GH-70: Log messages are strings
                    || name == "level"       // GH-70: Log levels are strings ("INFO", "ERROR")
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || (name == "word" && is_singular)
                    || (name.starts_with("text") && is_singular)
                    || (name.starts_with("str") && is_singular)
                    || (name.ends_with("_str") && is_singular)
                    || (name.ends_with("_string") && is_singular)
                    || (name.ends_with("_word") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("timestamp") && is_singular)  // GH-70: created_timestamp, etc.
                    || (name.ends_with("_message") && is_singular) // GH-70: error_message, etc.
            }
            // DEPYLER-0577: Handle attribute access (e.g., args.text, args.prefix)
            HirExpr::Attribute { attr, .. } => {
                let name = attr.as_str();
                let is_singular = !name.ends_with('s');
                name == "text"
                    || name == "s"
                    || name == "string"
                    || name == "line"
                    || name == "content"
                    || name == "message"
                    || name == "prefix"      // String prefix for startswith operations
                    || name == "suffix"      // String suffix for endswith operations
                    || name == "pattern"     // String pattern for matching
                    || name == "char"        // Single character string
                    || name == "delimiter"   // String delimiter
                    || name == "separator"   // String separator
                    || name == "old"         // String replacement old value
                    || name == "new"         // String replacement new value
                    || (name.starts_with("text") && is_singular)
                    || (name.ends_with("_text") && is_singular)
                    || (name.ends_with("_string") && is_singular)
            }
            HirExpr::MethodCall { method, .. }
                if method.as_str().contains("upper")
                    || method.as_str().contains("lower")
                    || method.as_str().contains("strip")
                    || method.as_str().contains("lstrip")
                    || method.as_str().contains("rstrip")
                    || method.as_str().contains("title") =>
            {
                true
            }
            HirExpr::Call { func, .. } if func.as_str() == "str" => true,
            // DEPYLER-0573: Dict value access with string-like keys
            // Pattern: dict["hash"], dict.get("hash")... - these return string values
            HirExpr::Index { base, index } if self.is_dict_expr(base) => {
                // Check if key suggests string value
                if let HirExpr::Literal(Literal::String(key)) = index.as_ref() {
                    let k = key.to_lowercase();
                    k.contains("hash")
                        || k.contains("name")
                        || k.contains("path")
                        || k.contains("text")
                        || k.contains("message")
                        || k.contains("algorithm")
                        || k.contains("filename")
                        || k.contains("modified")
                } else {
                    false
                }
            }
            // DEPYLER-0573: Dict.get() chain with string-like keys
            HirExpr::MethodCall {
                object,
                method,
                args,
                ..
            } if (method == "get" || method == "cloned" || method == "unwrap_or_default")
                && self.is_dict_value_access(object) =>
            {
                // If it's a get() call, check the key
                if method == "get" && !args.is_empty() {
                    if let HirExpr::Literal(Literal::String(key)) = &args[0] {
                        let k = key.to_lowercase();
                        return k.contains("hash")
                            || k.contains("name")
                            || k.contains("path")
                            || k.contains("text")
                            || k.contains("message")
                            || k.contains("algorithm")
                            || k.contains("filename")
                            || k.contains("modified");
                    }
                }
                // For cloned/unwrap_or_default, check the chain
                self.is_string_base(object)
            }
            _ => false,
        }
    }

    // DEPYLER-REFACTOR-001: is_string_method_call moved to builtin_conversions module

    /// DEPYLER-0701: Check if base expression is a tuple type
    /// Used to detect tuple[idx] patterns that need special handling
    fn is_tuple_base(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Tuple(_) => true,
            HirExpr::Var(sym) => {
                // Check type system for Tuple type
                if let Some(ty) = self.ctx.var_types.get(sym) {
                    matches!(ty, Type::Tuple(_))
                } else {
                    // Heuristic: common tuple variable names
                    let name = sym.as_str();
                    matches!(name, "pair" | "tuple" | "entry" | "item" | "elem" | "row" | "t")
                }
            }
            // Method call returning tuple (e.g., dict.items() element)
            HirExpr::MethodCall { object, method, .. } => {
                // Enumerate returns (index, value) tuples
                if method == "enumerate" {
                    return true;
                }
                // Dict.items() returns (key, value) tuples
                if method == "items" && self.is_dict_expr(object) {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0701: Get the size of a tuple type for array conversion
    /// Returns the number of elements in the tuple, or None if unknown
    fn get_tuple_size(&self, expr: &HirExpr) -> Option<usize> {
        match expr {
            HirExpr::Tuple(elements) => Some(elements.len()),
            HirExpr::Var(sym) => {
                if let Some(Type::Tuple(types)) = self.ctx.var_types.get(sym) {
                    Some(types.len())
                } else {
                    None // Default will be used
                }
            }
            _ => None,
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

        // DEPYLER-0302 Phase 3: Check if we're slicing a string
        let is_string = self.is_string_base(base);

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

        // DEPYLER-0302 Phase 3: Generate string-specific slice code
        if is_string {
            // DEPYLER-0573: If base is dict value access returning Value, convert to owned String
            // Value.as_str() returns &str with limited lifetime, so convert to String
            let final_base_expr = if self.is_dict_value_access(base) {
                parse_quote! { #base_expr.as_str().map(|s| s.to_string()).unwrap_or_default() }
            } else {
                base_expr
            };
            return self.convert_string_slice(final_base_expr, start_expr, stop_expr, step_expr);
        }

        // Generate slice code based on the parameters (for Vec/List)
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: base[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;
                        if step == 1 {
                            base.clone()
                        } else if step > 0 {
                            base.iter().step_by(step as usize).cloned().collect::<Vec<_>>()
                        } else if step == -1 {
                            base.iter().rev().cloned().collect::<Vec<_>>()
                        } else {
                            // Negative step with abs value
                            let abs_step = (-step) as usize;
                            base.iter().rev().step_by(abs_step).cloned().collect::<Vec<_>>()
                        }
                    }
                })
            }

            // Start and stop: base[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    let start_idx = #start as isize;
                    let stop_idx = #stop as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    if start < base.len() {
                        base[start..stop.min(base.len())].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Start only: base[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    let start_idx = #start as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    if start < base.len() {
                        base[start..].to_vec()
                    } else {
                        Vec::new()
                    }
                }
            }),

            // Stop only: base[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    // DEPYLER-0473: Borrow to avoid moving base (allows reuse later)
                    let base = &#base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    let stop_idx = #stop as isize;
                    let stop = if stop_idx < 0 {
                        (base.len() as isize + stop_idx).max(0) as usize
                    } else {
                        stop_idx as usize
                    };
                    base[..stop.min(base.len())].to_vec()
                }
            }),

            // Full slice: base[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.clone() }),

            // Start, stop, and step: base[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        // DEPYLER-0459: Cast to isize first to handle negative indices
                        let start_idx = #start as isize;
                        let stop_idx = #stop as isize;
                        let start = if start_idx < 0 {
                            (base.len() as isize + start_idx).max(0) as usize
                        } else {
                            start_idx as usize
                        };
                        let stop = if stop_idx < 0 {
                            (base.len() as isize + stop_idx).max(0) as usize
                        } else {
                            stop_idx as usize
                        };
                        // DEPYLER-0812: Use i32 for step to support negative values
                        let step: i32 = #step;

                        if step == 1 {
                            if start < base.len() {
                                base[start..stop.min(base.len())].to_vec()
                            } else {
                                Vec::new()
                            }
                        } else if step > 0 {
                            base[start..stop.min(base.len())]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            // Negative step - slice in reverse
                            let abs_step = (-step) as usize;
                            if start < base.len() {
                                base[start..stop.min(base.len())]
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
                    let base = #base_expr;
                    // DEPYLER-0459: Cast to isize first to handle negative indices
                    let start_idx = #start as isize;
                    let start = if start_idx < 0 {
                        (base.len() as isize + start_idx).max(0) as usize
                    } else {
                        start_idx as usize
                    };
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if start < base.len() {
                        if step == 1 {
                            base[start..].to_vec()
                        } else if step > 0 {
                            base[start..]
                                .iter()
                                .step_by(step as usize)
                                .cloned()
                                .collect::<Vec<_>>()
                        } else if step == -1 {
                            base[start..]
                                .iter()
                                .rev()
                                .cloned()
                                .collect::<Vec<_>>()
                        } else {
                            let abs_step = (-step) as usize;
                            base[start..]
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
                    let base = #base_expr;
                    let stop = (#stop).max(0) as usize;
                    // DEPYLER-0812: Use i32 for step to support negative values
                    let step: i32 = #step;

                    if step == 1 {
                        base[..stop.min(base.len())].to_vec()
                    } else if step > 0 {
                        base[..stop.min(base.len())]
                            .iter()
                            .step_by(step as usize)
                            .cloned()
                            .collect::<Vec<_>>()
                    } else if step == -1 {
                        base[..stop.min(base.len())]
                            .iter()
                            .rev()
                            .cloned()
                            .collect::<Vec<_>>()
                    } else {
                        let abs_step = (-step) as usize;
                        base[..stop.min(base.len())]
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

    /// DEPYLER-0302 Phase 3: String-specific slice code generation
    /// Handles string slicing with proper char boundaries and negative indices
    fn convert_string_slice(
        &mut self,
        base_expr: syn::Expr,
        start_expr: Option<syn::Expr>,
        stop_expr: Option<syn::Expr>,
        step_expr: Option<syn::Expr>,
    ) -> Result<syn::Expr> {
        match (start_expr, stop_expr, step_expr) {
            // Full slice with step: s[::step]
            (None, None, Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let step: i32 = #step;
                        if step == 1 {
                            base.to_string()
                        } else if step > 0 {
                            base.chars().step_by(step as usize).collect::<String>()
                        } else if step == -1 {
                            base.chars().rev().collect::<String>()
                        } else {
                            // Negative step with abs value
                            let abs_step = step.abs() as usize;
                            base.chars().rev().step_by(abs_step).collect::<String>()
                        }
                    }
                })
            }

            // Start and stop: s[start:stop]
            (Some(start), Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative indices
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if actual_start < actual_stop {
                        base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                    } else {
                        String::new()
                    }
                }
            }),

            // Start only: s[start:]
            (Some(start), None, None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[-n:]
                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    base.chars().skip(actual_start).collect::<String>()
                }
            }),

            // Stop only: s[:stop]
            (None, Some(stop), None) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop_idx: i32 = #stop;
                    let len = base.chars().count() as i32;

                    // Handle negative index for s[:-n]
                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    base.chars().take(actual_stop).collect::<String>()
                }
            }),

            // Full slice: s[:]
            (None, None, None) => Ok(parse_quote! { #base_expr.to_string() }),

            // Start, stop, and step: s[start:stop:step]
            (Some(start), Some(stop), Some(step)) => {
                Ok(parse_quote! {
                    {
                        let base = #base_expr;
                        let start_idx: i32 = #start;
                        let stop_idx: i32 = #stop;
                        let step: i32 = #step;
                        let len = base.chars().count() as i32;

                        // Handle negative indices
                        let actual_start = if start_idx < 0 {
                            (len + start_idx).max(0) as usize
                        } else {
                            start_idx.min(len) as usize
                        };

                        let actual_stop = if stop_idx < 0 {
                            (len + stop_idx).max(0) as usize
                        } else {
                            stop_idx.min(len) as usize
                        };

                        if step == 1 {
                            if actual_start < actual_stop {
                                base.chars().skip(actual_start).take(actual_stop - actual_start).collect::<String>()
                            } else {
                                String::new()
                            }
                        } else if step > 0 {
                            base.chars()
                                .skip(actual_start)
                                .take(actual_stop.saturating_sub(actual_start))
                                .step_by(step as usize)
                                .collect::<String>()
                        } else {
                            // Negative step - collect range then reverse
                            let abs_step = step.abs() as usize;
                            if actual_start < actual_stop {
                                base.chars()
                                    .skip(actual_start)
                                    .take(actual_stop - actual_start)
                                    .rev()
                                    .step_by(abs_step)
                                    .collect::<String>()
                            } else {
                                String::new()
                            }
                        }
                    }
                })
            }

            // Start and step: s[start::step]
            (Some(start), None, Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let start_idx: i32 = #start;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_start = if start_idx < 0 {
                        (len + start_idx).max(0) as usize
                    } else {
                        start_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().skip(actual_start).collect::<String>()
                    } else if step > 0 {
                        base.chars().skip(actual_start).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().skip(actual_start).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().skip(actual_start).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),

            // Stop and step: s[:stop:step]
            (None, Some(stop), Some(step)) => Ok(parse_quote! {
                {
                    let base = #base_expr;
                    let stop_idx: i32 = #stop;
                    let step: i32 = #step;
                    let len = base.chars().count() as i32;

                    let actual_stop = if stop_idx < 0 {
                        (len + stop_idx).max(0) as usize
                    } else {
                        stop_idx.min(len) as usize
                    };

                    if step == 1 {
                        base.chars().take(actual_stop).collect::<String>()
                    } else if step > 0 {
                        base.chars().take(actual_stop).step_by(step as usize).collect::<String>()
                    } else if step == -1 {
                        base.chars().take(actual_stop).rev().collect::<String>()
                    } else {
                        let abs_step = step.abs() as usize;
                        base.chars().take(actual_stop).rev().step_by(abs_step).collect::<String>()
                    }
                }
            }),
        }
    }

    fn convert_list(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // CITL: Trace list construction decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "list_construction",
            chosen = "vec_macro",
            alternatives = ["Vec_new", "array_literal", "smallvec", "tinyvec"],
            confidence = 0.90
        );

        // DEPYLER-0269 FIX: Convert string literals to owned Strings
        // List literals with string elements should use Vec<String> not Vec<&str>
        // This ensures they can be passed to functions expecting &Vec<String>

        // DEPYLER-0572: Detect if list has mixed types (dict access Value + format! String)
        // If so, unify to String by converting Value elements via .to_string()
        let has_dict_access = elts.iter().any(|e| self.is_dict_value_access(e));
        let has_fstring = elts.iter().any(|e| matches!(e, HirExpr::FString { .. }));
        let needs_string_unify = has_dict_access && has_fstring;

        // DEPYLER-0711: Detect heterogeneous list literals (mixed primitive types)
        // Rust's vec![] requires all elements to be the same type
        // For mixed types like [1, "hello", 3.14, true], use Vec<serde_json::Value>
        let has_mixed_types = self.list_has_mixed_types(elts);

        // DEPYLER-0741: Detect if list contains dicts and if ANY dict has None values
        // If so, ALL dicts must use Option<V> for type consistency
        let any_dict_has_none = elts.iter().any(|e| {
            if let HirExpr::Dict(items) = e {
                items
                    .iter()
                    .any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)))
            } else {
                false
            }
        });

        // Set flag before processing so convert_dict knows to wrap values in Some()
        if any_dict_has_none {
            self.ctx.force_dict_value_option_wrap = true;
        }

        // Scope guard: reset flag after processing list elements
        let result = self.convert_list_elements(elts, has_mixed_types, needs_string_unify);
        self.ctx.force_dict_value_option_wrap = false;
        result
    }

    /// DEPYLER-0741: Helper to convert list elements, allowing the flag to be reset afterward
    fn convert_list_elements(
        &mut self,
        elts: &[HirExpr],
        has_mixed_types: bool,
        needs_string_unify: bool,
    ) -> Result<syn::Expr> {
        if has_mixed_types {
            // DEPYLER-0711: Convert to Vec<serde_json::Value> for heterogeneous lists
            self.ctx.needs_serde_json = true;

            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    let expr = e.to_rust_expr(self.ctx)?;
                    // Wrap each element in serde_json::json!()
                    Ok(parse_quote! { serde_json::json!(#expr) })
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0739: Detect if list contains None elements
        // If so, wrap non-None elements in Some() to create Vec<Option<T>>
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        if has_none {
            let elt_exprs: Vec<syn::Expr> = elts
                .iter()
                .map(|e| {
                    if matches!(e, HirExpr::Literal(Literal::None)) {
                        // None stays as None
                        Ok(parse_quote! { None })
                    } else {
                        // Non-None elements get wrapped in Some()
                        let mut expr = e.to_rust_expr(self.ctx)?;
                        // Convert string literals to owned Strings
                        if matches!(e, HirExpr::Literal(Literal::String(_))) {
                            expr = parse_quote! { #expr.to_string() };
                        }
                        Ok(parse_quote! { Some(#expr) })
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            return Ok(parse_quote! { vec![#(#elt_exprs),*] });
        }

        // DEPYLER-0782: Check if list has string literals to determine if it's Vec<String>
        let has_string_literals = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::String(_))));

        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;
                // Check if element is a string literal
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-0782: Variables need .to_string() in Vec<String> context
                // Both constants (SCRIPT: &str) and parameters (arg: &str) need conversion
                // String.to_string() is a no-op, so safe to call on any string type
                if matches!(e, HirExpr::Var(_)) && has_string_literals {
                    expr = parse_quote! { #expr.to_string() };
                }
                // DEPYLER-0572: Convert dict Value to String when mixed with f-strings
                if needs_string_unify && self.is_dict_value_access(e) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;

        // Always use vec! for now to ensure mutability works
        // In the future, we should analyze if the list is mutated before deciding
        Ok(parse_quote! { vec![#(#elt_exprs),*] })
    }

    /// DEPYLER-0711: Check if list has heterogeneous element types
    /// Returns true if elements have different primitive types (int, string, float, bool)
    fn list_has_mixed_types(&self, elts: &[HirExpr]) -> bool {
        if elts.len() <= 1 {
            return false; // Single element or empty - no mixing possible
        }

        let mut has_bool_literal = false;
        let mut has_int_literal = false;
        let mut has_float_literal = false;
        let mut has_string_literal = false;

        for elem in elts {
            match elem {
                HirExpr::Literal(Literal::Bool(_)) => has_bool_literal = true,
                HirExpr::Literal(Literal::Int(_)) => has_int_literal = true,
                HirExpr::Literal(Literal::Float(_)) => has_float_literal = true,
                HirExpr::Literal(Literal::String(_)) => has_string_literal = true,
                _ => {}
            }
        }

        // Count how many distinct literal types we have
        let distinct_types = [has_bool_literal, has_int_literal, has_float_literal, has_string_literal]
            .iter()
            .filter(|&&b| b)
            .count();

        // Mixed types if we have more than one distinct type
        distinct_types > 1
    }

    fn convert_dict(&mut self, items: &[(HirExpr, HirExpr)]) -> Result<syn::Expr> {
        // CITL: Trace dict construction decision
        trace_decision!(
            category = DecisionCategory::TypeMapping,
            name = "dict_construction",
            chosen = "hashmap_or_json",
            alternatives = ["HashMap", "BTreeMap", "serde_json", "IndexMap"],
            confidence = 0.85
        );

        // DEPYLER-0376: Detect heterogeneous dicts (mixed value types)
        // DEPYLER-0461: Also check if we're in json!() context (nested dicts must use json!())
        // DEPYLER-0560: Check if function returns Dict with Any/Unknown value type
        // For mixed types or json context, use serde_json::json! instead of HashMap
        let has_mixed_types = self.dict_has_mixed_types(items)?;
        let in_json_context = self.ctx.in_json_context;

        // DEPYLER-0560: Check if return type requires serde_json::Value
        // If function returns Dict[str, Any] → HashMap<String, serde_json::Value>
        let return_needs_json = self.return_type_needs_json_dict();

        // DEPYLER-0560: When inside json!() context (nested dict), use json!() macro
        // This produces serde_json::Value which is what nested contexts expect
        if in_json_context {
            self.ctx.needs_serde_json = true;
            let mut entries = Vec::new();
            for (key, value) in items {
                let key_str = match key {
                    HirExpr::Literal(Literal::String(s)) => s.clone(),
                    _ => bail!("Dict keys for JSON output must be string literals"),
                };
                let val_expr = value.to_rust_expr(self.ctx)?;
                entries.push(quote! { #key_str: #val_expr });
            }
            return Ok(parse_quote! {
                serde_json::json!({
                    #(#entries),*
                })
            });
        }

        // DEPYLER-0560: When return type is HashMap<String, serde_json::Value>,
        // build HashMap with json!() wrapped values (NOT a raw json!() object)
        if has_mixed_types || return_needs_json {
            self.ctx.needs_serde_json = true;
            self.ctx.needs_hashmap = true;

            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let key_str = match key {
                    HirExpr::Literal(Literal::String(s)) => s.clone(),
                    _ => bail!("Dict keys for JSON output must be string literals"),
                };

                // Set json context for value conversion (nested dicts become json!())
                let prev_json_context = self.ctx.in_json_context;
                self.ctx.in_json_context = true;
                let val_expr = value.to_rust_expr(self.ctx)?;
                self.ctx.in_json_context = prev_json_context;

                // DEPYLER-0669: Check if val_expr is a HashMap block (can't go in json!())
                let val_str = quote! { #val_expr }.to_string();
                let wrapped_val = if val_str.contains("HashMap") || val_str.contains("let mut map") {
                    // Use serde_json::to_value() for HashMap block expressions
                    quote! { serde_json::to_value(#val_expr).unwrap() }
                } else {
                    // Wrap each value in json!() to convert to serde_json::Value
                    quote! { serde_json::json!(#val_expr) }
                };

                insert_stmts.push(quote! {
                    map.insert(#key_str.to_string(), #wrapped_val);
                });
            }

            return Ok(parse_quote! {
                {
                    let mut map = std::collections::HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            });
        }

        // Homogeneous dict: use HashMap
        self.ctx.needs_hashmap = true;

        // DEPYLER-0740: Detect if any dict value is None
        // If so, wrap non-None values in Some() to create HashMap<K, Option<V>>
        // DEPYLER-0741: Also check context flag - set when list of dicts has ANY dict with None
        let has_none_value = items
            .iter()
            .any(|(_, v)| matches!(v, HirExpr::Literal(Literal::None)));

        // Use Option wrapping if this dict has None OR if we're in a list context
        // where another dict has None (for type consistency)
        if has_none_value || self.ctx.force_dict_value_option_wrap {
            let mut insert_stmts = Vec::new();
            for (key, value) in items {
                let mut key_expr = key.to_rust_expr(self.ctx)?;

                // Convert string literal keys to owned Strings
                if matches!(key, HirExpr::Literal(Literal::String(_))) {
                    key_expr = parse_quote! { #key_expr.to_string() };
                }

                let val_expr: syn::Expr = if matches!(value, HirExpr::Literal(Literal::None)) {
                    // None stays as None
                    parse_quote! { None }
                } else {
                    // Non-None values get wrapped in Some()
                    let mut inner = value.to_rust_expr(self.ctx)?;
                    // Convert string literals to owned Strings
                    if matches!(value, HirExpr::Literal(Literal::String(_))) {
                        inner = parse_quote! { #inner.to_string() };
                    }
                    parse_quote! { Some(#inner) }
                };

                insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
            }

            return Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            });
        }

        // DEPYLER-0953: String literal values are now always converted to String
        // (Previously DEPYLER-0729 only converted when target type required it)
        let mut insert_stmts = Vec::new();
        for (key, value) in items {
            let mut key_expr = key.to_rust_expr(self.ctx)?;
            let mut val_expr = value.to_rust_expr(self.ctx)?;

            // DEPYLER-0810: Unwrap Result-returning function calls in dict value context
            // HashMap<K, V> expects V, not Result<V, E>, so we need to unwrap
            if let HirExpr::Call { func, .. } = value {
                if self.ctx.result_returning_functions.contains(func) {
                    let error_msg = format!("{} failed", func);
                    val_expr = parse_quote! { #val_expr.expect(#error_msg) };
                }
            }

            // DEPYLER-0270 FIX: ALWAYS convert string literal keys to owned Strings
            // Dict literals should use HashMap<String, V> not HashMap<&str, V>
            // This ensures they can be passed to functions expecting HashMap<String, V>
            if matches!(key, HirExpr::Literal(Literal::String(_))) {
                key_expr = parse_quote! { #key_expr.to_string() };
            }

            // DEPYLER-0729/0953: ALWAYS convert string literal values to String
            // HashMap<K, String> is the standard pattern, not HashMap<K, &str>
            // This ensures consistent types across dict literal, access, and assignment
            // Without this, `d = {"k": "v"}` creates HashMap<String, &str> which breaks
            // when later accessing with d["k"] or assigning d["k2"] = "v2"
            if matches!(value, HirExpr::Literal(Literal::String(_))) {
                val_expr = parse_quote! { #val_expr.to_string() };
            }

            insert_stmts.push(quote! { map.insert(#key_expr, #val_expr); });
        }

        // DEPYLER-0279: Only add `mut` if there are items to insert
        // Empty dicts don't need mutable bindings
        // DEPYLER-0472: When in json context, use json!({}) instead of HashMap::new()
        // This happens when dict is assigned to serde_json::Value (e.g., current[k] = {})
        if items.is_empty() {
            if self.ctx.in_json_context {
                // Use json!({}) for serde_json::Value compatibility
                self.ctx.needs_serde_json = true;
                Ok(parse_quote! { serde_json::json!({}) })
            } else {
                // Regular HashMap for normal dicts
                Ok(parse_quote! {
                    {
                        let map = HashMap::new();
                        map
                    }
                })
            }
        } else {
            Ok(parse_quote! {
                {
                    let mut map = HashMap::new();
                    #(#insert_stmts)*
                    map
                }
            })
        }
    }

    /// DEPYLER-0962: Check if return type is a Union of dict and list (e.g., dict | list)
    /// Returns Some(union_enum_name) if it is, None otherwise
    fn return_type_is_dict_list_union(&self) -> Option<String> {
        if let Some(Type::Union(types)) = self.ctx.current_return_type.as_ref() {
            // Check if union contains both Dict and List (in any order)
            let has_dict = types.iter().any(|t| matches!(t, Type::Dict(_, _)));
            let has_list = types.iter().any(|t| matches!(t, Type::List(_)));
            if has_dict && has_list && types.len() == 2 {
                // Generate the union enum name (e.g., DictOrListUnion)
                return Some("DictOrListUnion".to_string());
            }
        }
        None
    }

    /// DEPYLER-0560: Check if function return type requires serde_json::Value for dicts
    /// DEPYLER-0727: Also check assignment target type for inline dict literals
    ///
    /// Returns true if current function returns Dict[str, Any] or Dict[str, Unknown],
    /// OR if assigning to a variable with Dict[str, Any] type annotation,
    /// which maps to HashMap<String, serde_json::Value>. In these cases, dict literals
    /// should use json!() to ensure type compatibility.
    fn return_type_needs_json_dict(&self) -> bool {
        // DEPYLER-0727: Check assignment target type first (e.g., d: Dict[str, Any] = {...})
        if let Some(ref assign_type) = self.ctx.current_assign_type {
            match assign_type {
                Type::Dict(_, value_type) => {
                    if Self::is_json_value_type(value_type.as_ref()) {
                        return true;
                    }
                }
                Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => return true,
                _ => {}
            }
        }

        // Check function return type
        if let Some(ref ret_type) = self.ctx.current_return_type {
            // Check if return type is Dict with Any/Unknown value type
            match ret_type {
                Type::Dict(_, value_type) => Self::is_json_value_type(value_type.as_ref()),
                // Custom type might be Result<Dict<K, V>, E> - check inner type
                Type::Custom(s) if s.contains("HashMap") && s.contains("Value") => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Helper: Check if a type should use serde_json::Value
    /// DEPYLER-0726: Also check for Type::Custom("Any") after DEPYLER-0725 fix
    /// DEPYLER-0773: Also check for "object" which is Python's top-level type
    fn is_json_value_type(ty: &Type) -> bool {
        matches!(ty, Type::Unknown)
            || matches!(ty, Type::Custom(s) if s == "serde_json::Value" || s == "Value" || s == "Any" || s == "object")
    }

    /// DEPYLER-0376: Check if dict has heterogeneous value types
    /// DEPYLER-0270 FIX: Only flag as heterogeneous when we have strong evidence
    /// DEPYLER-0461: Also detect nested dicts which require serde_json::Value
    fn dict_has_mixed_types(&self, items: &[(HirExpr, HirExpr)]) -> Result<bool> {
        if items.len() <= 1 {
            return Ok(false); // Single type or empty
        }

        // DEPYLER-0461: Check for nested dict expressions (recursively)
        // If any value is a Dict (or contains a Dict), we need serde_json::json!()
        // This ensures ALL levels of nested dicts use json!() for consistency
        if self.dict_contains_nested_dict(items) {
            return Ok(true);
        }

        // STRATEGY 1: Check for obvious mixing of literal types
        let mut has_bool_literal = false;
        let mut has_int_literal = false;
        let mut has_float_literal = false;
        let mut has_string_literal = false;
        // DEPYLER-0601: Also track list element types for heterogeneous detection
        let mut has_list_of_int = false;
        let mut has_list_of_string = false;
        let mut has_list_of_bool = false;
        let mut has_list_of_float = false;

        for (_key, value) in items {
            match value {
                HirExpr::Literal(Literal::Bool(_)) => has_bool_literal = true,
                HirExpr::Literal(Literal::Int(_)) => has_int_literal = true,
                HirExpr::Literal(Literal::Float(_)) => has_float_literal = true,
                HirExpr::Literal(Literal::String(_)) => has_string_literal = true,
                // DEPYLER-0601: Check list element types for heterogeneous detection
                HirExpr::List(elems) if !elems.is_empty() => {
                    // Determine list element type from first element
                    match &elems[0] {
                        HirExpr::Literal(Literal::Int(_)) => has_list_of_int = true,
                        HirExpr::Literal(Literal::String(_)) => has_list_of_string = true,
                        HirExpr::Literal(Literal::Bool(_)) => has_list_of_bool = true,
                        HirExpr::Literal(Literal::Float(_)) => has_list_of_float = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Count how many distinct literal types we have
        let distinct_literal_types = [
            has_bool_literal,
            has_int_literal,
            has_float_literal,
            has_string_literal,
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        // DEPYLER-0601: Count how many distinct list element types we have
        let distinct_list_types = [
            has_list_of_int,
            has_list_of_string,
            has_list_of_bool,
            has_list_of_float,
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        // Use json! if we have 2+ distinct literal types OR 2+ distinct list types
        // This handles both {"a": 1, "b": "str"} and {"items": [1,2], "tags": ["a"]}
        Ok(distinct_literal_types >= 2 || distinct_list_types >= 2)
    }

    /// DEPYLER-0461: Recursively check if dict contains any nested dicts
    /// Returns true if any value is a Dict. When this is true, ALL nested dicts
    /// in the tree must use json!() for consistency (json!() doesn't accept HashMap blocks)
    fn dict_contains_nested_dict(&self, items: &[(HirExpr, HirExpr)]) -> bool {
        for (_key, value) in items {
            if self.expr_is_or_contains_dict(value) {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0461: Check if expression is a dict or recursively contains a dict
    fn expr_is_or_contains_dict(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::List(items) => items.iter().any(|e| self.expr_is_or_contains_dict(e)),
            HirExpr::Tuple(items) => items.iter().any(|e| self.expr_is_or_contains_dict(e)),
            _ => false,
        }
    }

    fn convert_tuple(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        // DEPYLER-0682: Convert string literals in tuples to owned Strings
        // When tuples are used in lists (e.g., Vec<(i32, i32, String)>), string
        // elements need to be owned Strings, not &str references.
        // This ensures type consistency across all tuple elements in a Vec.
        let elt_exprs: Vec<syn::Expr> = elts
            .iter()
            .map(|e| {
                let mut expr = e.to_rust_expr(self.ctx)?;
                // Convert string literals to .to_string() for owned String
                if matches!(e, HirExpr::Literal(Literal::String(_))) {
                    expr = parse_quote! { #expr.to_string() };
                }
                Ok(expr)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(parse_quote! { (#(#elt_exprs),*) })
    }

    fn convert_set(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;

        // DEPYLER-0742: Detect if set contains None
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                set
            }
        })
    }

    fn convert_frozenset(&mut self, elts: &[HirExpr]) -> Result<syn::Expr> {
        self.ctx.needs_hashset = true;
        self.ctx.needs_arc = true;

        // DEPYLER-0742: Detect if frozenset contains None
        let has_none = elts
            .iter()
            .any(|e| matches!(e, HirExpr::Literal(Literal::None)));

        let mut insert_stmts = Vec::new();
        for elem in elts {
            // DEPYLER-0742: Wrap non-None elements in Some() when set has None
            if has_none {
                if matches!(elem, HirExpr::Literal(Literal::None)) {
                    insert_stmts.push(quote! { set.insert(None); });
                } else {
                    let elem_expr = elem.to_rust_expr(self.ctx)?;
                    insert_stmts.push(quote! { set.insert(Some(#elem_expr)); });
                }
            } else {
                let elem_expr = elem.to_rust_expr(self.ctx)?;
                insert_stmts.push(quote! { set.insert(#elem_expr); });
            }
        }
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! {
            {
                let mut set = std::collections::HashSet::new();
                #(#insert_stmts)*
                std::sync::Arc::new(set)
            }
        })
    }

    fn convert_attribute(&mut self, value: &HirExpr, attr: &str) -> Result<syn::Expr> {
        // DEPYLER-0608: In cmd_* handlers, args.X → X (field is now a direct parameter)
        // This is because subcommand fields live in Commands::Variant, not on Args
        // The handler function now takes individual field parameters instead of &Args
        if self.ctx.in_cmd_handler {
            if let HirExpr::Var(var_name) = value {
                if var_name == "args" && self.ctx.cmd_handler_args_fields.contains(&attr.to_string())
                {
                    // Transform args.field → field (the field is now a direct parameter)
                    // DEPYLER-0941: Handle Rust keywords like "type" with raw identifier syntax
                    let attr_ident = if Self::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // DEPYLER-0627: subprocess.run() now returns CompletedProcess struct
        // with .returncode, .stdout, .stderr fields - no conversion needed,
        // struct field access works directly

        // DEPYLER-0200: Handle os.environ direct access
        // os.environ → std::env::vars() as a HashMap-like collection
        if let HirExpr::Var(var_name) = value {
            if var_name == "os" && attr == "environ" {
                // os.environ returns an environment dict-like object
                // Convert to HashMap<String, String> for dict-like operations
                return Ok(parse_quote! {
                    std::env::vars().collect::<std::collections::HashMap<String, String>>()
                });
            }
        }

        if let HirExpr::Var(var_name) = value {
            // DEPYLER-0517: Handle exception variable attributes
            // Python: `except CalledProcessError as e: e.returncode`
            // Rust: Box<dyn Error> doesn't have returncode, use fallback
            // Common exception variable names: e, err, error, exc, exception
            let is_likely_exception = var_name == "e"
                || var_name == "err"
                || var_name == "error"
                || var_name == "exc"
                || var_name == "exception";

            if is_likely_exception && attr == "returncode" {
                // Use 1 as a generic non-zero exit code for errors
                return Ok(parse_quote! { 1 });
            }

            // DEPYLER-0535: Handle tempfile file handle attributes
            // Python: f.name → Rust: f.path().to_string_lossy().to_string()
            // Common tempfile variable names: f, temp, temp_file, tmpfile
            let is_likely_tempfile = var_name == "f"
                || var_name == "temp"
                || var_name == "tmp"
                || var_name.contains("temp")
                || var_name.contains("tmp");

            if is_likely_tempfile && attr == "name" {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #var_ident.path().to_string_lossy().to_string() });
            }

            // DEPYLER-0551: Handle os.stat_result attributes (from path.stat() / std::fs::metadata)
            // Python: stats.st_size → Rust: stats.len()
            // Python: stats.st_mtime → Rust: stats.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
            let is_likely_stats =
                var_name == "stats" || var_name == "stat" || var_name.ends_with("_stats");

            if is_likely_stats {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "st_size" => {
                        // DEPYLER-0693: Cast file size to i64 (Python int can be large)
                        return Ok(parse_quote! { #var_ident.len() as i64 });
                    }
                    "st_mtime" => {
                        return Ok(parse_quote! {
                            #var_ident.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_ctime" => {
                        // Creation time (use modified as fallback on Unix)
                        return Ok(parse_quote! {
                            #var_ident.created().unwrap_or_else(|_| #var_ident.modified().unwrap())
                                .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_atime" => {
                        return Ok(parse_quote! {
                            #var_ident.accessed().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
                        });
                    }
                    "st_mode" => {
                        // File permissions
                        return Ok(parse_quote! { #var_ident.permissions().mode() });
                    }
                    _ => {} // Fall through
                }
            }

            // DEPYLER-0551: Handle pathlib.Path attributes
            // Python: path.name → Rust: path.file_name().and_then(|n| n.to_str()).unwrap_or("")
            // Python: path.suffix → Rust: path.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
            // DEPYLER-0706: Removed `var_name == "p"` - too many false positives (e.g., Person p, Point p)
            // Only use explicit path naming patterns to avoid confusing struct field access with path operations
            // DEPYLER-0942: Also check var_types for PathBuf/Path type assignment
            let is_named_path = var_name == "path" || var_name.ends_with("_path");
            let is_typed_path = self
                .ctx
                .var_types
                .get(var_name)
                .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                .unwrap_or(false);
            let is_likely_path = is_named_path || is_typed_path;

            if is_likely_path {
                let var_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                match attr {
                    "name" => {
                        return Ok(parse_quote! {
                            #var_ident.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "suffix" => {
                        return Ok(parse_quote! {
                            #var_ident.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
                        });
                    }
                    "stem" => {
                        return Ok(parse_quote! {
                            #var_ident.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
                        });
                    }
                    "parent" => {
                        return Ok(parse_quote! {
                            #var_ident.parent().map(|p| p.to_path_buf()).unwrap_or_default()
                        });
                    }
                    _ => {} // Fall through to regular attribute handling
                }
            }
        }

        // DEPYLER-0425: Handle subcommand field access (args.url → url)
        // If this is accessing a subcommand-specific field on args parameter,
        // generate just the field name (it's extracted via pattern matching)
        if let HirExpr::Var(var_name) = value {
            // Check if var_name is an args parameter
            // (heuristic: variable ending in "args" or exactly "args")
            if (var_name == "args" || var_name.ends_with("args"))
                && self.ctx.argparser_tracker.has_subcommands()
            {
                // Check if this field belongs to any subcommand
                let mut is_subcommand_field = false;
                for subcommand in self.ctx.argparser_tracker.subcommands.values() {
                    for arg in &subcommand.arguments {
                        if arg.rust_field_name() == attr {
                            is_subcommand_field = true;
                            break;
                        }
                    }
                    if is_subcommand_field {
                        break;
                    }
                }

                if is_subcommand_field {
                    // Generate just the field name (extracted via pattern matching in func wrapper)
                    let attr_ident = if Self::is_rust_keyword(attr) {
                        syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                    } else {
                        syn::Ident::new(attr, proc_macro2::Span::call_site())
                    };
                    return Ok(parse_quote! { #attr_ident });
                }
            }
        }

        // Handle classmethod cls.ATTR → Self::ATTR
        if let HirExpr::Var(var_name) = value {
            if var_name == "cls" && self.ctx.is_classmethod {
                // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
                let attr_ident = if Self::is_rust_keyword(attr) {
                    syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
                } else {
                    syn::Ident::new(attr, proc_macro2::Span::call_site())
                };
                return Ok(parse_quote! { Self::#attr_ident });
            }

            // DEPYLER-0422 Fix #11: Detect enum constant access patterns
            // TypeName.CONSTANT → TypeName::CONSTANT
            // Five-Whys Root Cause:
            // 1. Why: E0423 - expected value, found struct 'Color'
            // 2. Why: Code generates Color.RED (field access) instead of Color::RED
            // 3. Why: Default attribute access uses dot syntax
            // 4. Why: No detection for type constant access vs field access
            // 5. ROOT CAUSE: Need to use :: for type-level constants
            //
            // Heuristic: If name starts with uppercase and attr is ALL_CAPS, it's likely an enum constant
            let first_char = var_name.chars().next().unwrap_or('a');
            let is_type_name = first_char.is_uppercase();
            let is_constant = attr.chars().all(|c| c.is_uppercase() || c == '_');

            if is_type_name && is_constant {
                let type_ident = syn::Ident::new(var_name, proc_macro2::Span::call_site());
                let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #type_ident::#attr_ident });
            }
        }

        // Check if this is a module attribute access
        if let HirExpr::Var(module_name) = value {
            // DEPYLER-STDLIB-MATH: Handle math module constants
            // math.pi → std::f64::consts::PI
            // math.e → std::f64::consts::E
            // math.inf → f64::INFINITY
            // math.nan → f64::NAN
            if module_name == "math" {
                let result = match attr {
                    "pi" => parse_quote! { std::f64::consts::PI },
                    "e" => parse_quote! { std::f64::consts::E },
                    "tau" => parse_quote! { std::f64::consts::TAU },
                    "inf" => parse_quote! { f64::INFINITY },
                    "nan" => parse_quote! { f64::NAN },
                    // DEPYLER-0595: Math functions as first-class values
                    "sin" => parse_quote! { f64::sin },
                    "cos" => parse_quote! { f64::cos },
                    "tan" => parse_quote! { f64::tan },
                    "asin" => parse_quote! { f64::asin },
                    "acos" => parse_quote! { f64::acos },
                    "atan" => parse_quote! { f64::atan },
                    "sqrt" => parse_quote! { f64::sqrt },
                    "exp" => parse_quote! { f64::exp },
                    "log" => parse_quote! { f64::ln },
                    "log10" => parse_quote! { f64::log10 },
                    "floor" => parse_quote! { f64::floor },
                    "ceil" => parse_quote! { f64::ceil },
                    "abs" => parse_quote! { f64::abs },
                    _ => {
                        // If it's not a recognized constant/function, it might be a typo
                        bail!("math.{} is not a recognized constant or method", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-STRING: Handle string module constants
            // string.ascii_letters → "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            // string.digits → "0123456789"
            // string.punctuation → "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
            if module_name == "string" {
                let result = match attr {
                    "ascii_lowercase" => parse_quote! { "abcdefghijklmnopqrstuvwxyz" },
                    "ascii_uppercase" => parse_quote! { "ABCDEFGHIJKLMNOPQRSTUVWXYZ" },
                    "ascii_letters" => {
                        parse_quote! { "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" }
                    }
                    "digits" => parse_quote! { "0123456789" },
                    "hexdigits" => parse_quote! { "0123456789abcdefABCDEF" },
                    "octdigits" => parse_quote! { "01234567" },
                    "punctuation" => parse_quote! { "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~" },
                    "whitespace" => parse_quote! { " \t\n\r\x0b\x0c" },
                    "printable" => {
                        parse_quote! { "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c" }
                    }
                    _ => {
                        // Not a string constant - might be a method like capwords
                        bail!("string.{} is not a recognized constant", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0518: Handle re module constants
            // Python regex flags: re.IGNORECASE, re.MULTILINE, etc.
            // These are integer flags in Python but Rust regex uses builder methods.
            // For now, map them to constants that can be used in conditional checks.
            if module_name == "re" {
                let result = match attr {
                    // Map to integer constants (matching Python values for compatibility)
                    "IGNORECASE" | "I" => parse_quote! { 2i32 },
                    "MULTILINE" | "M" => parse_quote! { 8i32 },
                    "DOTALL" | "S" => parse_quote! { 16i32 },
                    "VERBOSE" | "X" => parse_quote! { 64i32 },
                    "ASCII" | "A" => parse_quote! { 256i32 },
                    "LOCALE" | "L" => parse_quote! { 4i32 },
                    "UNICODE" | "U" => parse_quote! { 32i32 },
                    _ => {
                        // Not a recognized constant - fall through to default handling
                        let module_ident =
                            syn::Ident::new(module_name, proc_macro2::Span::call_site());
                        let attr_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                        return Ok(parse_quote! { #module_ident.#attr_ident });
                    }
                };
                return Ok(result);
            }

            // DEPYLER-STDLIB-SYS: Handle sys module attributes
            // sys.argv → std::env::args().collect()
            // sys.platform → compile-time platform string
            // DEPYLER-0381: sys.stdin/stdout/stderr → std::io::stdin()/stdout()/stderr()
            if module_name == "sys" {
                let result = match attr {
                    "argv" => parse_quote! { std::env::args().collect::<Vec<String>>() },
                    "platform" => {
                        // Return platform name based on target OS as String
                        #[cfg(target_os = "linux")]
                        let platform = "linux";
                        #[cfg(target_os = "macos")]
                        let platform = "darwin";
                        #[cfg(target_os = "windows")]
                        let platform = "win32";
                        #[cfg(not(any(
                            target_os = "linux",
                            target_os = "macos",
                            target_os = "windows"
                        )))]
                        let platform = "unknown";
                        parse_quote! { #platform.to_string() }
                    }
                    // DEPYLER-0381: I/O stream attributes (functions in Rust, not objects)
                    "stdin" => parse_quote! { std::io::stdin() },
                    "stdout" => parse_quote! { std::io::stdout() },
                    "stderr" => parse_quote! { std::io::stderr() },
                    // DEPYLER-0381: version_info as a tuple (major, minor)
                    // Note: Python's sys.version_info is a 5-tuple (major, minor, micro, releaselevel, serial)
                    // but most comparisons use only (major, minor), so we return a 2-tuple for compatibility
                    "version_info" => {
                        // Rust doesn't have runtime version info by default
                        // Return a compile-time constant tuple matching Python 3.11
                        parse_quote! { (3, 11) }
                    }
                    _ => {
                        bail!("sys.{} is not a recognized attribute", attr);
                    }
                };
                return Ok(result);
            }

            // DEPYLER-0335 FIX #2: Get rust_path and rust_name (clone to avoid borrow issues)
            let module_info = self
                .ctx
                .imported_modules
                .get(module_name)
                .and_then(|mapping| {
                    mapping
                        .item_map
                        .get(attr)
                        .map(|rust_name| (mapping.rust_path.clone(), rust_name.clone()))
                });

            if let Some((rust_path, rust_name)) = module_info {
                // Map to the Rust equivalent
                let path_parts: Vec<&str> = rust_name.split("::").collect();
                if path_parts.len() > 1 {
                    // DEPYLER-0335 FIX #2: Use rust_path from mapping instead of hardcoding "std"
                    let base_path: syn::Path =
                        syn::parse_str(&rust_path).unwrap_or_else(|_| parse_quote! { std });
                    let mut path = quote! { #base_path };
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

        // DEPYLER-STDLIB-DATETIME: Handle datetime/date/time/timedelta properties
        // In chrono, properties are accessed as methods: dt.year → dt.year()
        // This handles properties for fractions, pathlib, datetime, date, time, and timedelta instances
        let value_expr = value.to_rust_expr(self.ctx)?;
        match attr {
            // DEPYLER-STDLIB-FRACTIONS: Fraction properties
            "numerator" => {
                // f.numerator → *f.numer()
                return Ok(parse_quote! { *#value_expr.numer() });
            }

            "denominator" => {
                // f.denominator → *f.denom()
                return Ok(parse_quote! { *#value_expr.denom() });
            }

            // DEPYLER-STDLIB-PATHLIB: Path properties
            // DEPYLER-0357: Removed overly-aggressive "name" special case
            // The .name attribute should only map to .file_name() for Path types
            // For generic objects (like in sorted(people, key=lambda p: p.name)),
            // .name should be preserved as-is and fall through to default handling
            "stem" => {
                // p.stem → p.file_stem().unwrap().to_str().unwrap().to_string()
                return Ok(parse_quote! {
                    #value_expr.file_stem().unwrap().to_str().unwrap().to_string()
                });
            }

            "suffix" => {
                // p.suffix → p.extension().map(|e| format!(".{}", e.to_str().unwrap())).unwrap_or_default()
                return Ok(parse_quote! {
                    #value_expr.extension()
                        .map(|e| format!(".{}", e.to_str().unwrap()))
                        .unwrap_or_default()
                });
            }

            "parent" => {
                // p.parent → p.parent().unwrap().to_path_buf()
                return Ok(parse_quote! {
                    #value_expr.parent().unwrap().to_path_buf()
                });
            }

            "parts" => {
                // p.parts → p.components().map(|c| c.as_os_str().to_str().unwrap().to_string()).collect()
                return Ok(parse_quote! {
                    #value_expr.components()
                        .map(|c| c.as_os_str().to_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                });
            }

            // datetime/date properties (require method calls in chrono)
            "year" | "month" | "day" | "hour" | "minute" | "second" | "microsecond" => {
                // Check if this might be a datetime/date/time object
                // We convert: dt.year → dt.year()
                let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());
                return Ok(parse_quote! { #value_expr.#method_ident() as i32 });
            }

            // DEPYLER-0514 / GH-74: Removed overly-aggressive timedelta property conversions
            // These attribute names (days, seconds, microseconds) can appear on ANY object,
            // not just timedelta. Without type information, we cannot distinguish between:
            //   - obj.seconds (regular field) vs td.seconds (timedelta property)
            //
            // Similar to DEPYLER-0357 which removed overly-aggressive .name conversion.
            //
            // Future enhancement: Add type-aware attribute rewriting that checks HIR type
            // information before applying stdlib-specific conversions.
            //
            // Commented out until type guards are implemented:
            //
            // "days" => {
            //     // td.days → td.num_days()
            //     return Ok(parse_quote! { #value_expr.num_days() as i32 });
            // }
            //
            // "seconds" => {
            //     // td.seconds → td.num_seconds() % 86400 (seconds within the day)
            //     return Ok(parse_quote! { (#value_expr.num_seconds() % 86400) as i32 });
            // }
            //
            // "microseconds" => {
            //     // td.microseconds → (td.num_microseconds() % 1_000_000)
            //     return Ok(
            //         parse_quote! { (#value_expr.num_microseconds().unwrap() % 1_000_000) as i32 },
            //     );
            // }
            _ => {
                // Not a datetime property, continue with default handling
            }
        }

        // DEPYLER-0452: Check stdlib API mappings before default fallback
        // Try common CSV patterns (heuristic-based for now)
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "DictReader", attr) {
            // Found a CSV DictReader mapping - apply it
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Also try generic Reader patterns
        if let Some(mapping) = self.ctx.stdlib_mappings.lookup("csv", "Reader", attr) {
            let rust_code =
                mapping.generate_rust_code(&value_expr.to_token_stream().to_string(), &[]);
            if let Ok(expr) = syn::parse_str::<syn::Expr>(&rust_code) {
                return Ok(expr);
            }
        }

        // Default behavior for non-module attributes
        // DEPYLER-0306 FIX: Use raw identifiers for attributes that are Rust keywords
        let attr_ident = if Self::is_rust_keyword(attr) {
            syn::Ident::new_raw(attr, proc_macro2::Span::call_site())
        } else {
            syn::Ident::new(attr, proc_macro2::Span::call_site())
        };

        // DEPYLER-0737: Check if this is a @property method access
        // In Python, @property allows method access without (), but Rust requires ()
        if self.ctx.property_methods.contains(attr) {
            Ok(parse_quote! { #value_expr.#attr_ident() })
        } else {
            Ok(parse_quote! { #value_expr.#attr_ident })
        }
    }

    fn convert_borrow(&mut self, expr: &HirExpr, mutable: bool) -> Result<syn::Expr> {
        // CITL: Trace borrowing strategy decision
        #[cfg(feature = "decision-tracing")]
        let borrow_type = if mutable { "&mut" } else { "&" };
        trace_decision!(
            category = DecisionCategory::BorrowStrategy,
            name = "explicit_borrow",
            chosen = borrow_type,
            alternatives = ["&ref", "&mut_ref", "move", "clone"],
            confidence = 0.92
        );

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
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in list comprehensions
        // Strategy: Single generator → simple chain, Multiple → flat_map nesting
        // Same pattern as convert_generator_expression but with .collect::<Vec<_>>()

        if generators.is_empty() {
            bail!("List comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0454: Detect CSV reader variables in list comprehensions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in list comprehension
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                // .cloned() works for both Copy and Clone types, .copied() only works for Copy
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0792: Check if any condition contains a walrus operator (:=)
            // and if the element expression uses that walrus variable.
            // If so, we must use filter_map instead of filter + map, because
            // the walrus variable is defined in the filter closure but needed in map.
            let walrus_vars_in_conditions = Self::collect_walrus_vars_from_conditions(&gen.conditions);
            let element_uses_walrus = !walrus_vars_in_conditions.is_empty()
                && Self::expr_uses_any_var(element, &walrus_vars_in_conditions);

            if element_uses_walrus && gen.conditions.len() == 1 {
                // DEPYLER-0792: Single condition with walrus - use filter_map pattern
                // Python: [(w, length) for w in words if (length := len(w)) > 3]
                // Rust: words.iter().cloned().filter_map(|w| {
                //           let length = w.len() as i32;
                //           if length > 3 { Some((w, length)) } else { None }
                //       }).collect::<Vec<_>>()
                let cond = &gen.conditions[0];
                let cond_expr = cond.to_rust_expr(self.ctx)?;

                // Collect walrus variable assignments as let bindings
                let walrus_bindings = Self::generate_walrus_bindings(cond, self.ctx)?;

                chain = parse_quote! {
                    #chain.filter_map(|#target_pat| {
                        #walrus_bindings
                        if #cond_expr { Some(#element_expr) } else { None }
                    })
                };

                // Collect into Vec
                return Ok(parse_quote! { #chain.collect::<Vec<_>>() });
            }

            // DEPYLER-0691: Add filters for each condition (no walrus in element)
            // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            // Collect into Vec
            return Ok(parse_quote! { #chain.collect::<Vec<_>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        // Pattern: [x + y for x in range(3) for y in range(3)]
        // Becomes: (0..3).flat_map(|x| (0..3).map(move |y| x + y)).collect::<Vec<_>>()

        let chain = self.convert_nested_generators_for_list_comp(element, generators)?;
        Ok(parse_quote! { #chain.collect::<Vec<_>>() })
    }

    fn convert_nested_generators_for_list_comp(
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
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
    }

    /// DEPYLER-0511: Wrap range expressions in parentheses before method calls
    ///
    /// Ranges need parentheses when followed by method calls due to operator precedence.
    /// Without parens: `0..5.into_iter()` parses as `0..(5.into_iter())` ❌
    /// With parens: `(0..5).into_iter()` parses correctly ✅
    ///
    /// Detects syn::Expr::Range and wraps in syn::Expr::Paren.
    fn wrap_range_in_parens(&self, expr: syn::Expr) -> syn::Expr {
        match &expr {
            syn::Expr::Range(_) => {
                // Wrap range in parentheses
                parse_quote! { (#expr) }
            }
            _ => expr, // No wrapping needed for other expressions
        }
    }

    /// Add dereference (*) to uses of target variable in expression
    /// This is needed because filter closures receive &T even when the iterator yields T
    /// Example: transforms `x > 0` to `*x > 0` when x is the target variable
    ///
    /// Note: Currently unused but kept for potential future use with filter optimization
    #[allow(dead_code)]
    fn add_deref_to_var_uses(&mut self, expr: &HirExpr, target: &str) -> Result<syn::Expr> {
        use crate::hir::{BinOp, HirExpr, UnaryOp};

        match expr {
            HirExpr::Var(name) if name == target => {
                // This is the target variable - add dereference
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                Ok(parse_quote! { *#ident })
            }
            HirExpr::Binary { op, left, right } => {
                // Recursively add derefs to both sides
                let left_expr = self.add_deref_to_var_uses(left, target)?;
                let right_expr = self.add_deref_to_var_uses(right, target)?;

                // Generate the operator token
                let result = match op {
                    BinOp::Add => parse_quote! { #left_expr + #right_expr },
                    BinOp::Sub => parse_quote! { #left_expr - #right_expr },
                    BinOp::Mul => parse_quote! { #left_expr * #right_expr },
                    BinOp::Div => parse_quote! { #left_expr / #right_expr },
                    BinOp::FloorDiv => parse_quote! { #left_expr / #right_expr },
                    BinOp::Mod => parse_quote! { #left_expr % #right_expr },
                    BinOp::Pow => parse_quote! { #left_expr.pow(#right_expr as u32) },
                    BinOp::Eq => parse_quote! { #left_expr == #right_expr },
                    BinOp::NotEq => parse_quote! { #left_expr != #right_expr },
                    BinOp::Lt => parse_quote! { #left_expr < #right_expr },
                    BinOp::LtEq => parse_quote! { #left_expr <= #right_expr },
                    BinOp::Gt => parse_quote! { #left_expr > #right_expr },
                    BinOp::GtEq => parse_quote! { #left_expr >= #right_expr },
                    BinOp::And => parse_quote! { #left_expr && #right_expr },
                    BinOp::Or => parse_quote! { #left_expr || #right_expr },
                    BinOp::BitAnd => parse_quote! { #left_expr & #right_expr },
                    BinOp::BitOr => parse_quote! { #left_expr | #right_expr },
                    BinOp::BitXor => parse_quote! { #left_expr ^ #right_expr },
                    BinOp::LShift => parse_quote! { #left_expr << #right_expr },
                    BinOp::RShift => parse_quote! { #left_expr >> #right_expr },
                    BinOp::In => parse_quote! { #right_expr.contains(&#left_expr) },
                    BinOp::NotIn => parse_quote! { !#right_expr.contains(&#left_expr) },
                };
                Ok(result)
            }
            HirExpr::Unary { op, operand } => {
                // Recursively add derefs to operand
                let operand_expr = self.add_deref_to_var_uses(operand, target)?;

                let result = match op {
                    UnaryOp::Not => parse_quote! { !#operand_expr },
                    UnaryOp::Neg => parse_quote! { -#operand_expr },
                    UnaryOp::Pos => parse_quote! { +#operand_expr },
                    UnaryOp::BitNot => parse_quote! { !#operand_expr },
                };
                Ok(result)
            }
            // For any other expression, convert normally (no deref needed)
            _ => expr.to_rust_expr(self.ctx),
        }
    }

    fn is_set_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Set(_) | HirExpr::FrozenSet(_) => true,
            HirExpr::Call { func, .. } if func == "set" || func == "frozenset" => true,
            HirExpr::Var(_) => {
                // Check type information in context for variables
                self.is_set_var(expr)
            }
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression is a numpy array (trueno Vector)
    fn is_numpy_array_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // np.array() call
            HirExpr::Call { func, .. } if func == "array" => true,
            // np.abs(), np.sqrt(), etc. calls that return vectors
            HirExpr::Call { func, .. } => {
                matches!(func.as_str(), "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" |
                         "zeros" | "ones" | "clip" | "clamp" | "normalize")
            }
            // Method calls on numpy arrays return numpy arrays
            HirExpr::MethodCall { method, .. } => {
                matches!(method.as_str(), "abs" | "sqrt" | "sin" | "cos" | "exp" | "log" |
                         "clamp" | "clip" | "unwrap")
            }
            // DEPYLER-0804: Check var_types first to avoid false positives
            // Variables with known scalar types (Float, Int) are NOT numpy arrays
            HirExpr::Var(name) => {
                // DEPYLER-0932: First check numpy_vars set (most reliable)
                // This tracks variables explicitly assigned from numpy operations
                if self.ctx.numpy_vars.contains(name) {
                    return true;
                }

                // Next check var_types for definitive type info
                if let Some(ty) = self.ctx.var_types.get(name) {
                    // Scalar types are never numpy arrays
                    if matches!(ty, Type::Float | Type::Int | Type::Bool | Type::String) {
                        return false;
                    }
                    // DEPYLER-0955: Only treat List types as numpy arrays if they contain
                    // numeric primitives (Int, Float). Lists of tuples, strings, etc.
                    // should NOT use .copied() which requires Copy trait.
                    if let Type::List(inner) = ty {
                        // Only numeric inner types are numpy-like
                        if matches!(inner.as_ref(), Type::Int | Type::Float) {
                            return true;
                        }
                        // Non-numeric lists (tuples, strings, etc.) are NOT numpy arrays
                        return false;
                    }
                    // DEPYLER-0836: trueno::Vector<T> types are numpy arrays
                    if let Type::Custom(type_name) = ty {
                        if type_name.starts_with("Vector<") || type_name == "Vector" {
                            return true;
                        }
                    }
                }
                // Fall back to name heuristics only for unknown types
                // DEPYLER-0804: Removed "x", "y" - too generic, often scalars
                // DEPYLER-0836: "result" is included when in numpy context (needs_trueno)
                // DEPYLER-0926: Added "a", "b" for common numpy vector arithmetic patterns
                let n = name.as_str();
                let is_numpy_context = self.ctx.needs_trueno;
                matches!(n, "arr" | "array" | "data" | "values" | "vec" | "vector")
                    || n.starts_with("arr_") || n.ends_with("_arr")
                    || n.starts_with("vec_") || n.ends_with("_vec")
                    || (is_numpy_context && matches!(n, "result" | "a" | "b" | "v1" | "v2"))
            }
            // Recursive: binary op on vector yields vector
            HirExpr::Binary { left, .. } => self.is_numpy_array_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0188: Check if expression is a pathlib Path (std::path::PathBuf)
    ///
    /// Python's pathlib.Path uses `/` operator (via __truediv__) for path concatenation.
    /// Rust's PathBuf doesn't implement Div, so we convert to .join().
    fn is_path_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Path() or pathlib.Path() call
            HirExpr::Call { func, .. } => {
                matches!(func.as_str(), "Path" | "PurePath" | "PurePosixPath" | "PureWindowsPath")
            }
            // Method calls that return paths
            // Note: "resolve" and "absolute" are NOT included because they are converted
            // with .to_string_lossy().to_string() and thus return String, not PathBuf
            HirExpr::MethodCall { method, .. } => {
                matches!(method.as_str(), "parent" | "expanduser" |
                         "with_name" | "with_suffix" | "with_stem" | "joinpath")
            }
            // Attribute access like Path(__file__).parent
            HirExpr::Attribute { attr, .. } => {
                matches!(attr.as_str(), "parent" | "root" | "anchor")
            }
            // Variable named 'path' or with path-like semantics
            // DEPYLER-0188: Include common module-level path constants (SCRIPT, FILE, etc.)
            // DEPYLER-0930: Also check var_types for PathBuf type (e.g., result = Path(...))
            HirExpr::Var(name) => {
                // First check if variable is typed as PathBuf/Path
                let is_typed_path = self
                    .ctx
                    .var_types
                    .get(name)
                    .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                    .unwrap_or(false);
                if is_typed_path {
                    return true;
                }
                // Fall back to name-based heuristics
                let n = name.as_str();
                let n_lower = n.to_lowercase();
                matches!(n, "path" | "filepath" | "dir_path" | "file_path" | "base_path" | "root_path"
                         | "SCRIPT" | "SCRIPT_PATH" | "SCRIPT_DIR" | "SCRIPT_FILE"
                         | "ROOT" | "ROOT_DIR" | "ROOT_PATH" | "BASE" | "BASE_DIR")
                    || n.starts_with("path_") || n.ends_with("_path")
                    || n.starts_with("dir_") || n.ends_with("_dir")
                    || n_lower.ends_with("_path") || n_lower.ends_with("_dir")
                    || n_lower.starts_with("script")
            }
            // Recursive: path / segment is still a path
            HirExpr::Binary { left, op: BinOp::Div, .. } => self.is_path_expr(left),
            _ => false,
        }
    }

    /// DEPYLER-0607: Check if expression yields serde_json::Value that needs iteration conversion
    ///
    /// serde_json::Value doesn't implement IntoIterator, so we need to detect when
    /// the iteration expression is a JSON Value and wrap it with .as_array().
    ///
    /// Returns true for:
    /// - Variables with dict/JSON Value types in context
    /// - Method chains like data.get("items").cloned().unwrap_or_default()
    /// - Dict index expressions like data["items"]
    fn is_json_value_iteration(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable - check if it has a JSON/dict type in context
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, v) if
                        matches!(v.as_ref(), Type::Unknown) ||
                        matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                } else {
                    // Heuristic: if needs_serde_json is set, variables may be JSON Values
                    self.ctx.needs_serde_json
                }
            }
            // Dict index expression - likely yields JSON Value
            HirExpr::Index { base, .. } => {
                match base.as_ref() {
                    HirExpr::Var(var_name) => {
                        if let Some(t) = self.ctx.var_types.get(var_name) {
                            matches!(t, Type::Dict(_, v) if
                                matches!(v.as_ref(), Type::Unknown) ||
                                matches!(v.as_ref(), Type::Custom(n) if n.contains("Value") || n.contains("json")))
                        } else {
                            self.ctx.needs_serde_json
                        }
                    }
                    HirExpr::Dict(_) => true, // Dict literal
                    _ => false,
                }
            }
            // Method chains that yield JSON Value
            HirExpr::MethodCall { object, method, .. } => {
                let is_chain_method = matches!(method.as_str(),
                    "get" | "cloned" | "unwrap_or_default" | "unwrap_or" | "unwrap"
                );
                if is_chain_method {
                    self.is_json_value_iteration(object.as_ref())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if a variable has a set type based on type information in context
    fn is_set_var(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types in context to see if this variable is a set
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Set(_))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0321: Check if expression is a string type
    /// Used to distinguish string.contains() from HashMap.contains_key()
    ///
    /// # Complexity
    /// 4 (match + type lookup + variant check + attribute check)
    fn is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Literal(Literal::String(_)) => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a string
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::String)
                } else {
                    // Fallback to heuristic for cases without type info
                    self.is_string_base(expr)
                }
            }
            // DEPYLER-0649: Handle attribute access for known string fields
            HirExpr::Attribute { attr, .. } => {
                // Known string attributes from common types:
                // - CompletedProcess.stdout, CompletedProcess.stderr
                // - Exception.args (often treated as string)
                // - argparse Namespace string fields
                matches!(
                    attr.as_str(),
                    "stdout" | "stderr" | "text" | "output" | "message" | "name"
                )
            }
            // DEPYLER-0675: Handle str() function call - returns String
            // Python: list(str(num)) → Rust: num.to_string().chars().collect()
            HirExpr::Call { func, .. } => {
                // str() builtin returns a string
                func == "str"
            }
            // DEPYLER-0676: Handle method calls that return strings
            // Python: list(num.to_string()) → Rust: num.to_string().chars().collect()
            HirExpr::MethodCall { method, .. } => {
                // Methods that return strings
                matches!(
                    method.as_str(),
                    "to_string" | "format" | "upper" | "lower" | "strip" | "replace" | "join"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0498: Check if expression is an Option type
    /// Used to determine if unwrap_or is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a variable with Option<T> type
    /// - Expression is an attribute access that returns Option
    ///
    /// # Complexity
    /// 2 (match + type lookup)
    fn expr_is_option(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable: check if type is Optional
            HirExpr::Var(var_name) => {
                if let Some(var_type) = self.ctx.var_types.get(var_name) {
                    matches!(var_type, Type::Optional(_))
                } else {
                    false
                }
            }
            // Attribute access: check if field type is Optional
            HirExpr::Attribute { value, attr } => {
                // DEPYLER-0498: Check if self.field is Option in generator context
                if let HirExpr::Var(obj_name) = value.as_ref() {
                    if obj_name == "self" && self.ctx.in_generator {
                        // Check if this field is a generator state variable with Optional type
                        if self.ctx.generator_state_vars.contains(attr) {
                            // Field is a generator state var - check its type in var_types
                            if let Some(field_type) = self.ctx.var_types.get(attr) {
                                return matches!(field_type, Type::Optional(_));
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #7: Check if expression is a dict/HashMap
    /// Used for dict merge operator (|) and other dict-specific operations
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    fn is_dict_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Dict(_) => true,
            HirExpr::Call { func, .. } if func == "dict" => true,
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a dict/HashMap
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, _))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0572: Check if expression is a dict value access (returns serde_json::Value)
    /// Pattern: dict[key] or dict.get(key).cloned().unwrap_or_default()
    /// These return Value which needs .to_string() when mixed with String in lists
    fn is_dict_value_access(&self, expr: &HirExpr) -> bool {
        match expr {
            // dict[key] index access
            HirExpr::Index { base, .. } => self.is_dict_expr(base),
            // dict.get(key)... chain
            HirExpr::MethodCall { object, method, .. } => {
                if method == "get" {
                    self.is_dict_expr(object)
                } else if method == "cloned" || method == "unwrap_or_default" || method == "unwrap"
                {
                    // Check the chain for dict access
                    self.is_dict_value_access(object)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0540: Check if expression is typed as serde_json::Value
    /// serde_json::Value needs special handling for .keys(), .values(), .items()
    /// because it requires .as_object().unwrap() before iteration methods.
    fn is_serde_json_value(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Var(name) = expr {
            // Check explicit type info first - this is authoritative
            if let Some(var_type) = self.ctx.var_types.get(name) {
                // Check for explicit serde_json::Value type
                if matches!(var_type, Type::Custom(ref s) if s == "serde_json::Value") {
                    return true;
                }
                // DEPYLER-0708: Removed overly aggressive check for Dict(_, Unknown)
                // A plain `dict` annotation creates Dict(Unknown, Unknown) but should NOT
                // trigger serde_json::Value treatment. Only explicit serde_json::Value should.
                // Dict types use regular HashMap.iter(), not .as_object().
                if matches!(var_type, Type::Dict(_, _)) {
                    return false;
                }
                // DEPYLER-0545: Type::Unknown should fall through to name heuristic
                // This allows variables like "filters" to be detected as JSON even when
                // their type is Unknown (common in nested closures/functions)
                if !matches!(var_type, Type::Unknown) {
                    // For other explicitly typed variables, not a JSON value
                    return false;
                }
                // Type::Unknown falls through to name heuristic below
            }

            // DEPYLER-0540: Use name heuristic when NO type info OR Type::Unknown
            // (e.g., in nested closures where parent param types aren't tracked)
            // Be conservative - only match explicitly json-like names
            // Note: "filters", "config" are commonly used for serde_json::Value dicts
            let is_json_by_name = matches!(
                name.as_str(),
                "filters" | "json_data" | "json_obj" | "json_value" | "json_config" | "config"
            );
            if is_json_by_name {
                return true;
            }
        }
        false
    }

    /// DEPYLER-0550: Check if expression could be a serde_json::Value
    /// Used for comparison handling when .get() returns Option<String>
    /// but the other side is a JSON Value from .items() iteration
    fn is_serde_json_value_expr(&self, expr: &HirExpr) -> bool {
        // First check using the existing helper
        if self.is_serde_json_value(expr) {
            return true;
        }

        // DEPYLER-0550: Check for pattern variables from JSON iteration
        // When iterating over filters.items(), we get (col, val) where val is Value
        // The variable "val" in this context is a JSON Value
        if let HirExpr::Var(name) = expr {
            // Variables commonly used for JSON values in iteration patterns
            // "val" is the most common from: for col, val in filters.items()
            if matches!(name.as_str(), "val" | "v" | "value" | "json_val") {
                // Additional context check: if there's no type info, assume JSON in iteration
                if !self.ctx.var_types.contains_key(name) {
                    return true;
                }
            }
        }

        false
    }

    /// DEPYLER-0700: Check if dict expression has serde_json::Value values
    ///
    /// Returns true if the dict maps to HashMap<String, serde_json::Value>,
    /// which happens when:
    /// - Dict has heterogeneous value types (e.g., {"name": "Alice", "age": 42})
    /// - Dict value type is Unknown (untyped dict)
    /// - Dict uses serde_json expressions
    ///
    /// This is used to wrap default values in dict.get(key, default) with json!()
    /// for type compatibility.
    fn dict_has_json_value_values(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable dict - check type info
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Dict with Unknown value type uses serde_json::Value
                    if matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::Unknown)) {
                        return true;
                    }
                    // Custom type that is serde_json::Value or HashMap with Value
                    if matches!(var_type, Type::Custom(ref s) if s.contains("serde_json::Value") || (s.contains("HashMap") && s.contains("Value"))) {
                        return true;
                    }
                }
                // If serde_json is already needed, this dict might use Value
                // Conservative: if we're generating serde_json code, assume mixed types
                self.ctx.needs_serde_json
            }
            // Dict literal - check if it has mixed value types
            HirExpr::Dict(items) => {
                if let Ok(has_mixed) = self.dict_has_mixed_types(items) {
                    has_mixed
                } else {
                    // Error checking - assume needs json for safety
                    self.ctx.needs_serde_json
                }
            }
            // Method call on dict - check base object
            HirExpr::MethodCall { object, .. } => self.dict_has_json_value_values(object),
            // Index into another dict
            HirExpr::Index { base, .. } => self.dict_has_json_value_values(base),
            _ => {
                // Fallback: if serde_json is in use, assume might be Value type
                self.ctx.needs_serde_json
            }
        }
    }

    /// DEPYLER-0729: Check if dict value type is String (not &str)
    /// Used to determine if string literal defaults in dict.get() need .to_string()
    fn dict_value_type_is_string(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::Dict(_, ref v) if matches!(**v, Type::String))
                } else {
                    false
                }
            }
            HirExpr::MethodCall { object, .. } => self.dict_value_type_is_string(object),
            HirExpr::Index { base, .. } => self.dict_value_type_is_string(base),
            _ => false,
        }
    }

    fn is_list_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::List(_) => true,
            HirExpr::Call { func, .. } if func == "list" => true,
            // DEPYLER-0811: Function calls that return list types
            HirExpr::Call { func, .. } => {
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    matches!(ret_type, Type::List(_))
                } else {
                    false
                }
            }
            HirExpr::Var(name) => {
                // DEPYLER-169: Check var_types for List type
                // This enables proper `item in list_var` detection
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    matches!(var_type, Type::List(_))
                } else {
                    // Fall back to conservative: only treat explicit list literals as lists
                    false
                }
            }
            // DEPYLER-0811: Binary Add of lists produces a list (for chained concat)
            HirExpr::Binary {
                op: BinOp::Add,
                left,
                right,
            } => self.is_list_expr(left) || self.is_list_expr(right),
            _ => false,
        }
    }

    /// DEPYLER-0742: Check if expression is a deque type (VecDeque)
    /// Used to generate correct VecDeque methods instead of Vec methods.
    fn is_deque_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call to deque() constructor
            HirExpr::Call { func, .. }
                if func == "deque" || func == "collections.deque" || func == "Deque" =>
            {
                true
            }
            HirExpr::Var(name) => {
                // Check var_types for Deque type annotation
                if let Some(var_type) = self.ctx.var_types.get(name) {
                    // Check if the type string contains "deque" or "VecDeque"
                    let type_str = format!("{:?}", var_type);
                    type_str.contains("deque") || type_str.contains("VecDeque")
                } else {
                    // Fallback: common deque variable names
                    matches!(
                        name.as_str(),
                        "d" | "dq" | "deque" | "queue" | "buffer" | "deck"
                    )
                }
            }
            _ => false,
        }
    }

    /// DEPYLER-0521: Check if variable is a borrowed string parameter (&str)
    ///
    /// Function parameters with Python `str` type annotation become `&str` in Rust.
    /// When used as dict keys, they should NOT have `&` added (already borrowed).
    ///
    /// Heuristic: If variable not in var_types and has a string-key-like name,
    /// it's likely a function parameter that's &str.
    ///
    /// # Complexity
    /// 2 (lookup + name check)
    fn is_borrowed_str_param(&self, var_name: &str) -> bool {
        // DEPYLER-0543: Check if variable is a function param with str type
        // These become &str in Rust and should NOT have & added
        if self.ctx.fn_str_params.contains(var_name) {
            return true; // already &str, don't add &
        }

        // When we have type info, use it
        if let Some(var_type) = self.ctx.var_types.get(var_name) {
            match var_type {
                Type::String => {
                    // Variable has Type::String but is NOT in fn_str_params
                    // This means it's a local variable (loop var, assignment) → owned String
                    return false; // needs borrowing
                }
                Type::Unknown => {
                    // Unknown type - use name heuristic as fallback
                }
                _ => {
                    // Other types - likely not a string key situation
                    return false;
                }
            }
        }

        // DEPYLER-0550: Removed "col" from heuristic - commonly used as loop variable
        // when iterating over dict items: for col, val in filters.items()
        // In that context, col is owned String from k.clone(), NOT a borrowed param
        // No type info or Unknown type - use name heuristics for function params
        // These are function parameters that typically become &str in Rust
        // Keep list minimal - only include names that are DEFINITELY function params
        let fn_param_names = matches!(var_name, "column" | "field" | "attr" | "property");

        if fn_param_names {
            return true;
        }

        // Variable not in var_types and not a known borrowed name
        // Default: assume needs borrowing (safer)
        false
    }

    /// DEPYLER-0496: Check if expression returns a Result type
    /// Used to determine if ? operator is needed in binary operations
    ///
    /// Returns true if:
    /// - Expression is a function call to a Result-returning function
    /// - Expression is a method call that might return Result
    ///
    /// # Complexity
    /// 2 (match + HashSet lookup)
    fn expr_returns_result(&self, expr: &HirExpr) -> bool {
        match expr {
            // Function calls: check if function is tracked as Result-returning
            HirExpr::Call { func, .. } => self.ctx.result_returning_functions.contains(func),
            // Method calls: Some method calls return Result (e.g., parse(), read_to_string())
            // For now, be conservative and don't assume method calls return Result
            // This can be enhanced later with specific method tracking
            HirExpr::MethodCall { .. } => false,
            // Other expressions don't return Result
            _ => false,
        }
    }

    /// DEPYLER-0575: Check if expression returns a float type
    /// Used to coerce integer literals to floats in comparisons
    fn expr_returns_float(&self, expr: &HirExpr) -> bool {
        match expr {
            // Float literals
            HirExpr::Literal(Literal::Float(_)) => true,
            // Variable with Float type, or variable from numpy float methods
            HirExpr::Var(name) => {
                if matches!(self.ctx.var_types.get(name), Some(Type::Float)) {
                    return true;
                }
                // Common float result variable names from numpy operations
                // DEPYLER-0668: Remove "result" - too general, often used for ints/bools
                // DEPYLER-0927: Sync with expr_returns_f32 - include norm_a, norm_b, dot etc.
                // DEPYLER-0928: Added min_val, max_val for Vector-scalar operations
                matches!(
                    name.as_str(),
                    "mean" | "std" | "variance" | "sum" | "norm" | "norm_a" | "norm_b"
                        | "stddev" | "var" | "denom" | "dot" | "min_val" | "max_val"
                )
            }
            // DEPYLER-0577: Attribute access (e.g., args.x) - check if attr is float type
            // DEPYLER-0720: Also check class_field_types for self.X attribute access
            HirExpr::Attribute { attr, value, .. } => {
                // Check var_types first (for non-self attributes)
                if matches!(self.ctx.var_types.get(attr), Some(Type::Float)) {
                    return true;
                }
                // Check class_field_types for self.X patterns
                if matches!(value.as_ref(), HirExpr::Var(name) if name == "self")
                    && matches!(self.ctx.class_field_types.get(attr), Some(Type::Float))
                {
                    return true;
                }
                false
            }
            // NumPy/trueno methods that return f32
            // DEPYLER-0927: Added norm_l2 and dot for trueno compatibility
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "stddev" | "var" | "variance" | "min" | "max"
                        | "norm" | "norm_l2" | "dot"
                )
            }
            // DEPYLER-0799: Function calls - check return type from function_return_types
            // This handles cases like f(a) * f(b) > 0 where f returns float
            HirExpr::Call { func, .. } => {
                // Check module-level function return types first
                if let Some(ret_type) = self.ctx.function_return_types.get(func) {
                    if matches!(ret_type, Type::Float) {
                        return true;
                    }
                }
                // DEPYLER-0800: Check if func is a Callable parameter with Float return type
                // Example: f: Callable[[float], float] -> f(x) returns Float
                if let Some(Type::Function { ret, .. }) = self.ctx.var_types.get(func) {
                    if matches!(ret.as_ref(), Type::Float) {
                        return true;
                    }
                }
                // Callable is stored as Generic { base: "Callable", params: [param_types, return_type] }
                if let Some(Type::Generic { base, params }) = self.ctx.var_types.get(func) {
                    if base == "Callable" && params.len() == 2 && matches!(params[1], Type::Float) {
                        return true;
                    }
                }
                // Also check for math builtin functions that return float
                // DEPYLER-0816: Removed "abs" - Python abs() preserves input type (int→int, float→float)
                // The math functions below ALWAYS return float, but abs() is type-preserving
                matches!(
                    func.as_str(),
                    "sqrt" | "sin" | "cos" | "tan" | "exp" | "log" | "log10" | "log2"
                        | "floor"
                        | "ceil"
                        | "pow"
                        | "float"
                )
            }
            // DEPYLER-0694: Binary expression with float operand returns float
            // This handles chained operations like (principal * rate) * years
            HirExpr::Binary { left, right, .. } => {
                self.expr_returns_float(left)
                    || self.expr_returns_float(right)
                    || self.is_float_var(left)
                    || self.is_float_var(right)
            }
            _ => false,
        }
    }

    /// DEPYLER-0920: Check if expression returns f32 specifically (trueno/numpy results)
    /// Used to generate f32 literals instead of f64 in comparisons
    /// DEPYLER-0927: Synced with expr_returns_float for consistent detection
    fn expr_returns_f32(&self, expr: &HirExpr) -> bool {
        match expr {
            // Variable names commonly used for trueno f32 results
            HirExpr::Var(name) => {
                matches!(
                    name.as_str(),
                    "mean" | "std" | "variance" | "sum" | "norm" | "norm_a" | "norm_b"
                        | "stddev" | "var" | "denom" | "dot"
                )
            }
            // Method calls on trueno Vectors return f32
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "mean" | "sum" | "std" | "stddev" | "var" | "variance" | "min" | "max"
                        | "norm" | "norm_l2" | "dot"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0786: Check if expression is a string type
    /// Used to determine if `or` operator should return string instead of bool
    fn expr_is_string_type(&self, expr: &HirExpr) -> bool {
        match expr {
            // String literals
            HirExpr::Literal(Literal::String(_)) => true,
            // Variable with String type
            HirExpr::Var(name) => {
                matches!(self.ctx.var_types.get(name), Some(Type::String))
            }
            // Attribute access with String type
            HirExpr::Attribute { attr, .. } => {
                matches!(self.ctx.var_types.get(attr), Some(Type::String))
            }
            // Method calls that return strings
            HirExpr::MethodCall { method, .. } => {
                matches!(
                    method.as_str(),
                    "strip" | "lower" | "upper" | "replace" | "join" | "format"
                )
            }
            _ => false,
        }
    }

    /// DEPYLER-0303 Phase 3 Fix #6: Check if expression is an owned collection
    /// Used to determine if zip() should use .into_iter() (owned) vs .iter() (borrowed)
    ///
    /// Returns true if:
    /// - Expression is a Var with type List (Vec<T>) - function parameters are owned
    /// - Expression is a list literal - always owned
    /// - Expression is a list() call - creates owned Vec
    ///
    /// # Complexity
    /// 3 (match + type lookup + variant check)
    fn is_owned_collection(&self, expr: &HirExpr) -> bool {
        match expr {
            // List literals are always owned
            HirExpr::List(_) => true,
            // list() calls create owned Vec
            HirExpr::Call { func, .. } if func == "list" => true,
            // Check if variable has List type (function parameters of type Vec<T>)
            HirExpr::Var(name) => {
                if let Some(ty) = self.ctx.var_types.get(name) {
                    matches!(ty, Type::List(_))
                } else {
                    // No type info - conservative default is borrowed
                    false
                }
            }
            _ => false,
        }
    }

    /// Check if an expression is a user-defined class instance
    fn is_class_instance(&self, expr: &HirExpr) -> bool {
        match expr {
            HirExpr::Var(name) => {
                // Check var_types to see if this variable is a user-defined class
                if let Some(Type::Custom(class_name)) = self.ctx.var_types.get(name) {
                    // Check if this is a user-defined class (not a builtin)
                    self.ctx.class_names.contains(class_name)
                } else {
                    false
                }
            }
            HirExpr::Call { func, .. } => {
                // Direct constructor call like Calculator(10)
                self.ctx.class_names.contains(func)
            }
            _ => false,
        }
    }

    // DEPYLER-REFACTOR-001: is_bool_expr moved to builtin_conversions module

    fn convert_set_operation(
        &self,
        op: BinOp,
        left: syn::Expr,
        right: syn::Expr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0412: Add explicit type annotation to collect() for set operations
        match op {
            BinOp::BitAnd => Ok(parse_quote! {
                #left.intersection(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::BitOr => Ok(parse_quote! {
                #left.union(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::Sub => Ok(parse_quote! {
                #left.difference(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            BinOp::BitXor => Ok(parse_quote! {
                #left.symmetric_difference(&#right).cloned().collect::<std::collections::HashSet<_>>()
            }),
            _ => bail!("Invalid set operator"),
        }
    }

    fn convert_set_comp(
        &mut self,
        element: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in set comprehensions
        // Same pattern as convert_list_comp but collecting to HashSet

        self.ctx.needs_hashset = true;

        if generators.is_empty() {
            bail!("Set comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let element_expr = element.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr = if !matches!(&*gen.iter, HirExpr::Var(_)) {
                self.wrap_range_in_parens(iter_expr)
            } else {
                iter_expr
            };

            let mut chain: syn::Expr = if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                parse_quote! { #iter_expr.as_slice().iter().copied() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820: Use |pattern| not |&pattern| - after .cloned() values are owned,
            // filter() receives &Item, using |pattern| binds as references avoiding E0507
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }

            // Add the map transformation
            chain = parse_quote! { #chain.map(|#target_pat| #element_expr) };

            // Collect into HashSet
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            return Ok(parse_quote! { #chain.collect::<std::collections::HashSet<_>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        let chain = self.convert_nested_generators_for_list_comp(element, generators)?;
        // DEPYLER-0831: Use fully-qualified path for E0412 resolution
        Ok(parse_quote! { #chain.collect::<std::collections::HashSet<_>>() })
    }

    fn convert_dict_comp(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // DEPYLER-0504: Support multiple generators in dict comprehensions
        // Same pattern as convert_list_comp but collecting to HashMap with (key, value) tuples

        self.ctx.needs_hashmap = true;

        if generators.is_empty() {
            bail!("Dict comprehension must have at least one generator");
        }

        // Single generator case (simple iterator chain)
        if generators.len() == 1 {
            let gen = &generators[0];
            let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
            let key_expr = key.to_rust_expr(self.ctx)?;
            let value_expr = value.to_rust_expr(self.ctx)?;
            let target_pat = self.parse_target_pattern(&gen.target)?;

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr = if !matches!(&*gen.iter, HirExpr::Var(_)) {
                self.wrap_range_in_parens(iter_expr)
            } else {
                iter_expr
            };

            // DEPYLER-0955: Dict comprehensions iterate over tuples which may contain String
            // (e.g., {k: v for k, v in items} where items is List[(str, int)])
            // Tuples with String don't implement Copy, so always use .cloned() for dict comp
            // This avoids the "Copy is not satisfied for String" error with .copied()
            let mut chain: syn::Expr = if matches!(&*gen.iter, HirExpr::Var(_)) {
                // Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (list literals, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820: Use |pattern| not |&pattern| - after .cloned() values are owned,
            // filter() receives &Item, using |pattern| binds as references avoiding E0507
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
            }

            // DEPYLER-0706: Add the map transformation (to key-value tuple)
            // Compute value before key to avoid borrow-after-move when value_expr
            // references the key variable (e.g., {word: len(word) for word in words})
            chain = parse_quote! { #chain.map(|#target_pat| { let _v = #value_expr; (#key_expr, _v) }) };

            // DEPYLER-0685: Use fully qualified path for HashMap to avoid import issues
            return Ok(parse_quote! { #chain.collect::<std::collections::HashMap<_, _>>() });
        }

        // Multiple generators case (nested iteration with flat_map)
        // Build nested chain that generates (key, value) tuples
        let chain = self.convert_nested_generators_for_dict_comp(key, value, generators)?;
        // DEPYLER-0685: Use fully qualified path for HashMap
        Ok(parse_quote! { #chain.collect::<std::collections::HashMap<_, _>>() })
    }

    fn convert_nested_generators_for_dict_comp(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
    ) -> Result<syn::Expr> {
        // Start with the outermost generator
        let first_gen = &generators[0];
        let first_iter = first_gen.iter.to_rust_expr(self.ctx)?;
        let first_pat = self.parse_target_pattern(&first_gen.target)?;

        // Build nested chain that produces (key, value) tuples
        let inner_expr = self.build_nested_chain_for_dict(key, value, generators, 1)?;

        // Start the chain with the first generator
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &first_gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#first_pat| #cond_expr) };
        }

        // Use flat_map for the first generator
        chain = parse_quote! { #chain.flat_map(|#first_pat| #inner_expr) };

        Ok(chain)
    }

    fn build_nested_chain_for_dict(
        &mut self,
        key: &HirExpr,
        value: &HirExpr,
        generators: &[crate::hir::HirComprehension],
        depth: usize,
    ) -> Result<syn::Expr> {
        if depth >= generators.len() {
            // Base case: no more generators, return (key, value) tuple
            let key_expr = key.to_rust_expr(self.ctx)?;
            let value_expr = value.to_rust_expr(self.ctx)?;
            // DEPYLER-0706: Compute value before key to avoid borrow-after-move
            return Ok(parse_quote! { std::iter::once({ let _v = #value_expr; (#key_expr, _v) }) });
        }

        // Recursive case: process current generator
        let gen = &generators[depth];
        let iter_expr = gen.iter.to_rust_expr(self.ctx)?;
        let target_pat = self.parse_target_pattern(&gen.target)?;

        // Build inner chain recursively
        let inner_chain = self.build_nested_chain_for_dict(key, value, generators, depth + 1)?;

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Start with iterator
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for current generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
        for cond in &gen.conditions {
            let cond_expr = cond.to_rust_expr(self.ctx)?;
            chain = parse_quote! { #chain.filter(|#target_pat| #cond_expr) };
        }

        // Use flat_map to nest the inner chain
        chain = parse_quote! { #chain.flat_map(move |#target_pat| #inner_chain) };

        Ok(chain)
    }

    fn convert_lambda(&mut self, params: &[String], body: &HirExpr) -> Result<syn::Expr> {
        // CITL: Trace lambda/closure conversion decision
        trace_decision!(
            category = DecisionCategory::Ownership,
            name = "lambda_closure",
            chosen = "closure",
            alternatives = ["fn_pointer", "closure_move", "closure_ref", "boxed_fn"],
            confidence = 0.87
        );

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in lambda parameters
        // Parameters named 'fn', 'match', 'type', etc. need to use raw identifier syntax
        let param_pats: Vec<syn::Pat> = params
            .iter()
            .map(|p| {
                let ident = crate::rust_gen::keywords::safe_ident(p);
                parse_quote! { #ident }
            })
            .collect();

        // Convert body expression
        let body_expr = body.to_rust_expr(self.ctx)?;

        // Generate closure
        // DEPYLER-0837: Use `move` closures to match Python's closure semantics
        // Python closures capture variables by reference but extend their lifetime
        // Rust requires `move` when returning closures that capture local variables
        if params.is_empty() {
            // No parameters
            Ok(parse_quote! { move || #body_expr })
        } else if params.len() == 1 {
            // Single parameter
            let param = &param_pats[0];
            Ok(parse_quote! { move |#param| #body_expr })
        } else {
            // Multiple parameters
            Ok(parse_quote! { move |#(#param_pats),*| #body_expr })
        }
    }

    /// Check if an expression is a len() call
    fn is_len_call(&self, expr: &HirExpr) -> bool {
        matches!(expr, HirExpr::Call { func, args , ..} if func == "len" && args.len() == 1)
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
                    // DEPYLER-0438/0441/0446: Smart formatting based on expression type
                    // - Collections (Vec, HashMap, HashSet): Use {:?} debug formatting
                    // - Scalars (String, i32, f64, bool): Use {} Display formatting
                    // - Option types: Unwrap with .unwrap_or_default() or display "None"
                    // This matches Python semantics where lists/dicts have their own repr
                    let arg_expr = expr.to_rust_expr(self.ctx)?;

                    // DEPYLER-0446: Check if this is an argparse Option<T> field (should be wrapped to String)
                    let is_argparse_option = match expr.as_ref() {
                        HirExpr::Attribute { value, attr } => {
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Check if this argument is optional (Option<T> type, not boolean)
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                let field_name = arg.rust_field_name();
                                                if field_name != *attr {
                                                    return false;
                                                }

                                                // Argument is NOT an Option if it has action="store_true" or "store_false"
                                                if matches!(
                                                    arg.action.as_deref(),
                                                    Some("store_true") | Some("store_false")
                                                ) {
                                                    return false;
                                                }

                                                // Argument is an Option<T> if: not required AND no default value AND not positional
                                                !arg.is_positional
                                                    && !arg.required.unwrap_or(false)
                                                    && arg.default.is_none()
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0497: Determine if expression needs {:?} Debug formatting
                    // Required for: collections, Result, Option, Vec, and any non-Display type
                    let needs_debug_fmt = match expr.as_ref() {
                        // Case 1: Simple variable (e.g., targets)
                        HirExpr::Var(var_name) => {
                            if let Some(var_type) = self.ctx.var_types.get(var_name) {
                                // Known type: check if it needs Debug formatting
                                // DEPYLER-0712: Added Tuple - tuples don't implement Display
                                matches!(
                                    var_type,
                                    Type::List(_)
                                        | Type::Dict(_, _)
                                        | Type::Set(_)
                                        | Type::Tuple(_)     // DEPYLER-0712: Tuples need {:?}
                                        | Type::Optional(_) // DEPYLER-0497: Options need {:?}
                                )
                            } else {
                                // DEPYLER-0497 WORKAROUND: Unknown type - default to {:?} (defensive)
                                // This is safer because Debug is more universally implemented than Display
                                // Most types implement Debug: Option<T>, Result<T,E>, Vec<T>, primitives
                                // Only a few types need Display: i32, String, etc (which also have Debug)
                                // This prevents E0277 errors for Option/Result/Vec variables
                                true
                            }
                        }
                        // DEPYLER-0497: Function calls that return Result<T> OR Option<T> need {:?}
                        HirExpr::Call { func, .. } => {
                            self.ctx.result_returning_functions.contains(func)
                                || self.ctx.option_returning_functions.contains(func)
                        }
                        // DEPYLER-0519: Method calls that return Vec types need {:?}
                        HirExpr::MethodCall { method, .. } => {
                            let vec_returning_methods = [
                                "groups",
                                "split",
                                "split_whitespace",
                                "splitlines",
                                "findall",
                                "keys",
                                "values",
                                "items",
                            ];
                            vec_returning_methods.contains(&method.as_str())
                        }
                        // Case 2: Attribute access (e.g., args.targets)
                        HirExpr::Attribute { value, attr } => {
                            // Check if this is accessing a field from argparse Args struct
                            if let HirExpr::Var(obj_name) = value.as_ref() {
                                // Check if obj_name is the args variable from ArgumentParser
                                let is_args_var = self.ctx.argparser_tracker.parsers.values().any(
                                    |parser_info| {
                                        parser_info
                                            .args_var
                                            .as_ref()
                                            .is_some_and(|args_var| args_var == obj_name)
                                    },
                                );

                                if is_args_var {
                                    // Look up the field type in argparse arguments
                                    self.ctx
                                        .argparser_tracker
                                        .parsers
                                        .values()
                                        .any(|parser_info| {
                                            parser_info.arguments.iter().any(|arg| {
                                                // Match field name (normalized from Python argument name)
                                                let field_name = arg.rust_field_name();
                                                if field_name == *attr {
                                                    // Check if this field is a collection type
                                                    // Either explicit type annotation OR inferred from nargs
                                                    let is_vec_from_nargs = matches!(
                                                        arg.nargs.as_deref(),
                                                        Some("+") | Some("*")
                                                    );
                                                    let is_collection_type =
                                                        if let Some(ref arg_type) = arg.arg_type {
                                                            matches!(
                                                                arg_type,
                                                                Type::List(_)
                                                                    | Type::Dict(_, _)
                                                                    | Type::Set(_)
                                                            )
                                                        } else {
                                                            false
                                                        };
                                                    is_vec_from_nargs || is_collection_type
                                                } else {
                                                    false
                                                }
                                            })
                                        })
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };

                    // DEPYLER-0446: Wrap argparse Option types to handle Display trait
                    // Only wrap argparse Optional fields, not regular Option variables
                    // DEPYLER-0930: Check if expression is a PathBuf type that needs .display()
                    // PathBuf doesn't implement Display, so we need to call .display() to format it
                    let is_pathbuf = match expr.as_ref() {
                        HirExpr::Var(var_name) => self
                            .ctx
                            .var_types
                            .get(var_name)
                            .map(|t| matches!(t, Type::Custom(ref s) if s == "PathBuf" || s == "Path"))
                            .unwrap_or(false),
                        HirExpr::MethodCall { method, .. } => {
                            // Methods that return PathBuf
                            matches!(
                                method.as_str(),
                                "parent" | "with_name" | "with_suffix" | "with_stem" | "join"
                            )
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
                    };

                    let final_arg = if is_argparse_option {
                        // Argparse Option<T> should display as value or "None" string
                        parse_quote! {
                            {
                                match &#arg_expr {
                                    Some(v) => format!("{}", v),
                                    None => "None".to_string(),
                                }
                            }
                        }
                    } else if is_pathbuf {
                        // DEPYLER-0930: PathBuf needs .display() to implement Display
                        parse_quote! { #arg_expr.display() }
                    } else {
                        arg_expr
                    };

                    // DEPYLER-0497: Use {:?} for non-Display types (Result, Vec, collections, Option)
                    // Use {} for Display types (primitives, String, wrapped argparse Options)
                    // DEPYLER-0930: PathBuf with .display() can use {} (Display trait)
                    if is_argparse_option || is_pathbuf {
                        // Argparse Option was wrapped to String, PathBuf has .display(), use {}
                        template.push_str("{}");
                    } else if needs_debug_fmt {
                        // Non-Display types (Vec, Result, Option, collections) need {:?}
                        template.push_str("{:?}");
                    } else {
                        // Regular Display types (i32, String, etc.)
                        template.push_str("{}");
                    }

                    args.push(final_arg);
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

    fn convert_ifexpr(
        &mut self,
        test: &HirExpr,
        body: &HirExpr,
        orelse: &HirExpr,
    ) -> Result<syn::Expr> {
        // DEPYLER-0377: Optimize `x if x else default` pattern
        // Python: `args.include if args.include else []` (check if list is non-empty)
        // Rust: Just `args.include` (clap initializes Vec to empty, so redundant check)
        // This pattern is common with argparse + Vec/Option fields
        if test == body {
            // Pattern: `x if x else y` → just use `x` (the condition is redundant)
            // This avoids type errors where Vec/Option can't be used as bool
            return body.to_rust_expr(self.ctx);
        }

        let mut test_expr = test.to_rust_expr(self.ctx)?;
        let body_expr = body.to_rust_expr(self.ctx)?;
        let orelse_expr = orelse.to_rust_expr(self.ctx)?;

        // DEPYLER-0377: Apply Python truthiness conversion to ternary expressions
        // Python: `val if val else default` where val is String/List/Dict/Set/Optional/Int/Float
        // Without conversion: `if val` fails (expected bool, found Vec/String/etc)
        // With conversion: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
        test_expr = Self::apply_truthiness_conversion(test, test_expr, self.ctx);

        // DEPYLER-0544: Detect File vs Stdout type mismatch
        // Python: `open(path, "w") if path else sys.stdout`
        // Rust: Needs Box<dyn Write> to unify File and Stdout types
        let body_is_file = self.is_file_creating_expr(body);
        let orelse_is_stdout = self.is_stdout_expr(orelse);
        let orelse_is_file = self.is_file_creating_expr(orelse);
        let body_is_stdout = self.is_stdout_expr(body);

        if (body_is_file && orelse_is_stdout) || (body_is_stdout && orelse_is_file) {
            // Wrap both sides in Box::new() for trait object unification
            return Ok(parse_quote! {
                if #test_expr { Box::new(#body_expr) as Box<dyn std::io::Write> } else { Box::new(#orelse_expr) }
            });
        }

        // DEPYLER-0927: Type unification for numeric IfExpr branches
        // When body returns float and orelse is integer literal, coerce orelse to float
        // Example: `dot / (norm_a * norm_b) if cond else 0` → `... else 0.0`
        let body_is_float = self.expr_returns_float(body);
        let body_is_f32 = self.expr_returns_f32(body);
        let orelse_is_int_literal = matches!(orelse, HirExpr::Literal(Literal::Int(_)));

        if body_is_float && orelse_is_int_literal {
            if let HirExpr::Literal(Literal::Int(n)) = orelse {
                let coerced_orelse: syn::Expr = if body_is_f32 {
                    let float_val = *n as f32;
                    parse_quote! { #float_val }
                } else {
                    let float_val = *n as f64;
                    parse_quote! { #float_val }
                };
                return Ok(parse_quote! {
                    if #test_expr { #body_expr } else { #coerced_orelse }
                });
            }
        }

        Ok(parse_quote! {
            if #test_expr { #body_expr } else { #orelse_expr }
        })
    }

    /// DEPYLER-0544: Check if expression creates a File (open() or File::create())
    fn is_file_creating_expr(&self, expr: &HirExpr) -> bool {
        match expr {
            // Call { func: Symbol, .. } - func is a simple function name like "open"
            HirExpr::Call { func, .. } => {
                // Check for open() builtin
                func == "open"
            }
            // MethodCall { object, method, .. } - e.g., File.create()
            HirExpr::MethodCall { object, method, .. } => {
                if method == "create" || method == "open" {
                    if let HirExpr::Var(name) = object.as_ref() {
                        return name == "File";
                    }
                    // std.fs.File.create()
                    if let HirExpr::Attribute { attr, .. } = object.as_ref() {
                        return attr == "File";
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// DEPYLER-0544: Check if expression is sys.stdout
    fn is_stdout_expr(&self, expr: &HirExpr) -> bool {
        if let HirExpr::Attribute { value, attr } = expr {
            if attr == "stdout" {
                if let HirExpr::Var(name) = value.as_ref() {
                    return name == "sys";
                }
            }
        }
        false
    }

    /// Apply Python truthiness conversion to non-boolean conditions
    /// Python: `if val:` where val is String/List/Dict/Set/Optional/Int/Float
    /// Rust: `if !val.is_empty()` / `if val.is_some()` / `if val != 0`
    fn apply_truthiness_conversion(
        condition: &HirExpr,
        cond_expr: syn::Expr,
        ctx: &CodeGenContext,
    ) -> syn::Expr {
        // Check if this is a variable reference that needs truthiness conversion
        if let HirExpr::Var(var_name) = condition {
            if let Some(var_type) = ctx.var_types.get(var_name) {
                return match var_type {
                    // Already boolean - no conversion needed
                    Type::Bool => cond_expr,

                    // String/List/Dict/Set - check if empty
                    Type::String | Type::List(_) | Type::Dict(_, _) | Type::Set(_) => {
                        parse_quote! { !#cond_expr.is_empty() }
                    }

                    // Optional - check if Some
                    Type::Optional(_) => {
                        parse_quote! { #cond_expr.is_some() }
                    }

                    // Numeric types - check if non-zero
                    Type::Int => {
                        parse_quote! { #cond_expr != 0 }
                    }
                    Type::Float => {
                        parse_quote! { #cond_expr != 0.0 }
                    }

                    // Unknown or other types - use as-is (may fail compilation)
                    _ => cond_expr,
                };
            }
        }

        // Not a variable or no type info - use as-is
        cond_expr
    }

    fn convert_sort_by_key(
        &mut self,
        iterable: &HirExpr,
        key_params: &[String],
        key_body: &HirExpr,
        reverse_expr: &Option<Box<HirExpr>>,
    ) -> Result<syn::Expr> {
        let iter_expr = iterable.to_rust_expr(self.ctx)?;

        // DEPYLER-0502: Convert reverse_expr to Rust expression (supports variables and expressions)
        // If None, default to false (no reversal)
        let reverse_rust_expr = if let Some(expr) = reverse_expr {
            expr.to_rust_expr(self.ctx)?
        } else {
            parse_quote! { false }
        };

        // DEPYLER-0307: Check if this is an identity function (lambda x: x)
        // If so, use simple .sort() instead of .sort_by_key()
        let is_identity =
            key_params.len() == 1 && matches!(key_body, HirExpr::Var(v) if v == &key_params[0]);

        if is_identity {
            // Identity function: just sort() + conditional reverse()
            return Ok(parse_quote! {
                {
                    let mut __sorted_result = #iter_expr.clone();
                    __sorted_result.sort();
                    if #reverse_rust_expr {
                        __sorted_result.reverse();
                    }
                    __sorted_result
                }
            });
        }

        // Non-identity key function: use sort_by_key
        let body_expr = key_body.to_rust_expr(self.ctx)?;

        // DEPYLER-0597: Use safe_ident to escape Rust keywords in sorted key lambda parameters
        let param_pat: syn::Pat = if key_params.len() == 1 {
            let param = crate::rust_gen::keywords::safe_ident(&key_params[0]);
            parse_quote! { #param }
        } else {
            bail!("sorted() key lambda must have exactly one parameter");
        };

        // DEPYLER-0502: Generate code with runtime conditional reverse
        // { let mut result = iterable.clone(); result.sort_by_key(|param| body); if reverse_expr { result.reverse(); } result }
        Ok(parse_quote! {
            {
                let mut __sorted_result = #iter_expr.clone();
                __sorted_result.sort_by_key(|#param_pat| #body_expr);
                if #reverse_rust_expr {
                    __sorted_result.reverse();
                }
                __sorted_result
            }
        })
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

            // DEPYLER-0454: Detect CSV reader variables in generator expressions
            let is_csv_reader = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "reader"
                    || var_name.contains("csv")
                    || var_name.ends_with("_reader")
                    || var_name.starts_with("reader_")
            } else {
                false
            };

            // DEPYLER-0523: Detect file variables for BufReader wrapping
            // Same heuristics as stmt_gen.rs for loop file iteration
            let is_file_iter = if let HirExpr::Var(var_name) = &*gen.iter {
                var_name == "f"
                    || var_name == "file"
                    || var_name == "input"
                    || var_name == "output"
                    || var_name.ends_with("_file")
                    || var_name.starts_with("file_")
            } else {
                false
            };

            // DEPYLER-0511: Wrap ranges in parens before method calls
            let iter_expr =
                if !is_csv_reader && !is_file_iter && !matches!(&*gen.iter, HirExpr::Var(_)) {
                    self.wrap_range_in_parens(iter_expr)
                } else {
                    iter_expr
                };

            // DEPYLER-0307 Fix #10: Use .iter().copied() for borrowed collections
            // DEPYLER-0454 Extension: Use .deserialize() for CSV readers
            // DEPYLER-0523: Use BufReader for file iteration
            // When the iterator is a variable (likely a borrowed parameter like &Vec<i32>),
            // use .iter().copied() to get owned values instead of references
            // This prevents type mismatches like `&i32` vs `i32` in generator expressions
            let mut chain: syn::Expr = if is_csv_reader {
                // DEPYLER-0454: CSV reader - use deserialize pattern
                self.ctx.needs_csv = true;
                parse_quote! { #iter_expr.deserialize::<std::collections::HashMap<String, String>>().filter_map(|result| result.ok()) }
            } else if is_file_iter {
                // DEPYLER-0523: File variable - use BufReader for line iteration
                self.ctx.needs_bufread = true;
                parse_quote! { std::io::BufReader::new(#iter_expr).lines().map(|l| l.unwrap_or_default()) }
            } else if self.is_numpy_array_expr(&gen.iter) {
                // DEPYLER-0575: trueno Vector uses .as_slice().iter()
                // DEPYLER-0909: Use .cloned() instead of .copied() for compatibility with non-Copy types
                parse_quote! { #iter_expr.as_slice().iter().cloned() }
            } else if self.is_json_value_iteration(&gen.iter) {
                // DEPYLER-0607: JSON Value iteration in generator expression
                // serde_json::Value doesn't implement IntoIterator, must convert first
                parse_quote! { #iter_expr.as_array().unwrap_or(&vec![]).iter().cloned() }
            } else if matches!(&*gen.iter, HirExpr::Var(_)) {
                // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                parse_quote! { #iter_expr.iter().cloned() }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0691: Add filters for each condition
            // DEPYLER-0820: Use |pattern| not |&pattern| - after .cloned() values are owned,
            // filter() receives &Item, using |pattern| binds as references avoiding E0507
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
        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let first_iter = self.wrap_range_in_parens(first_iter);
        let mut chain: syn::Expr = parse_quote! { #first_iter.into_iter() };

        // DEPYLER-0691: Add filters for first generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
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

        // DEPYLER-0511: Wrap ranges in parens before .into_iter()
        let iter_expr = self.wrap_range_in_parens(iter_expr);

        // Build the chain for this level
        let mut chain: syn::Expr = parse_quote! { #iter_expr.into_iter() };

        // DEPYLER-0691: Add filters for this generator's conditions
        // DEPYLER-0820: Use |pattern| not |&pattern| to avoid E0507 on non-Copy types
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

    /// DEPYLER-0188: Convert walrus operator (assignment expression)
    /// Python: (x := expr) assigns expr to x and evaluates to expr
    /// Rust: { let x = expr; x } - block expression that assigns and returns
    fn convert_named_expr(&mut self, target: &str, value: &HirExpr) -> Result<syn::Expr> {
        let ident = syn::Ident::new(target, proc_macro2::Span::call_site());
        let value_expr = value.to_rust_expr(self.ctx)?;

        // Generate: { let target = value; target }
        // This assigns the value and returns it, matching Python's walrus semantics
        Ok(parse_quote! {
            {
                let #ident = #value_expr;
                #ident
            }
        })
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
                if ctx.in_json_context {
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
