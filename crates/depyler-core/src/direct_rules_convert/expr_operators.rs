//! Expression operator conversion (binary, unary, truthiness) for ExprConverter

use crate::direct_rules::make_ident;
use crate::hir::*;
use crate::rust_gen::precedence;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

use super::operators::*;
use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_literal(&self, lit: &Literal) -> Result<syn::Expr> {
        Ok(convert_literal(lit))
    }

    pub(super) fn convert_variable(&self, name: &str) -> Result<syn::Expr> {
        // DEPYLER-0597: In method context (not classmethod), 'self' should be Rust keyword
        // Python `self.x` in instance method must become Rust `self.x`, not `self_.x`
        if name == "self" && !self.is_classmethod {
            return Ok(parse_quote! { self });
        }
        // DEPYLER-0596: Use make_ident to handle keywords like "match"
        let ident = make_ident(name);
        Ok(parse_quote! { #ident })
    }

    pub(super) fn convert_binary(&self, op: BinOp, left: &HirExpr, right: &HirExpr) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;

        match op {
            BinOp::In => {
                // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
                if self.is_dict_expr(right) {
                    // Convert "x in dict" to "dict.contains_key(&x)" for dicts/maps
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                } else if self.is_tuple_or_list_expr(right) {
                    // DEPYLER-0832: For tuples/lists, convert to array and use .contains()
                    // Python: x in (A, B, C) -> Rust: [A, B, C].contains(&x)
                    let elements: Vec<syn::Expr> = match right {
                        HirExpr::Tuple(elems) | HirExpr::List(elems) => elems
                            .iter()
                            .map(|e| self.convert(e))
                            .collect::<Result<Vec<_>>>()?,
                        _ => vec![right_expr.clone()],
                    };
                    Ok(parse_quote! { [#(#elements),*].contains(&#left_expr) })
                } else if self.is_string_expr(right) {
                    // DEPYLER-0601: For strings, use .contains() instead of .contains_key()
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String (&*String -> &str)
                            // and &str (&*&str -> &str), avoiding unstable str_as_str feature
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { #right_expr.contains(#pattern) })
                } else {
                    // Fallback: assume dict/HashMap
                    Ok(parse_quote! { #right_expr.contains_key(&#left_expr) })
                }
            }
            BinOp::NotIn => {
                // DEPYLER-0960: Check dict FIRST before string (overlapping names like "data", "result")
                if self.is_dict_expr(right) {
                    // Convert "x not in dict" to "!dict.contains_key(&x)"
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                } else if self.is_tuple_or_list_expr(right) {
                    // DEPYLER-0832: For tuples/lists, convert to array and use !.contains()
                    // Python: x not in (A, B, C) -> Rust: ![A, B, C].contains(&x)
                    let elements: Vec<syn::Expr> = match right {
                        HirExpr::Tuple(elems) | HirExpr::List(elems) => elems
                            .iter()
                            .map(|e| self.convert(e))
                            .collect::<Result<Vec<_>>>()?,
                        _ => vec![right_expr.clone()],
                    };
                    Ok(parse_quote! { ![#(#elements),*].contains(&#left_expr) })
                } else if self.is_string_expr(right) {
                    // DEPYLER-0601: For strings, use !.contains() instead of !.contains_key()
                    // DEPYLER-0200: Use raw string literal or &* for Pattern trait
                    let pattern: syn::Expr = match left {
                        HirExpr::Literal(Literal::String(s)) => {
                            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
                            parse_quote! { #lit }
                        }
                        _ => {
                            // Use &* to deref-reborrow - works for both String and &str
                            parse_quote! { &*#left_expr }
                        }
                    };
                    Ok(parse_quote! { !#right_expr.contains(#pattern) })
                } else {
                    // Fallback: assume dict/HashMap
                    Ok(parse_quote! { !#right_expr.contains_key(&#left_expr) })
                }
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
                if is_len_call(left) {
                    // Use saturating_sub to prevent underflow when subtracting from array length
                    // DEPYLER-0746: Wrap in parens to handle cast expressions like `x as usize`
                    Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) })
                } else {
                    let rust_op = convert_binop(op)?;

                    // DEPYLER-1094: Handle mixed i32/f64 subtraction
                    // Python promotes to float: int - float → float, float - int → float
                    let left_is_float = self.expr_returns_float_direct(left);
                    let right_is_float = self.expr_returns_float_direct(right);

                    // DEPYLER-0824: Wrap cast expressions in parentheses
                    let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                        parse_quote! { (#left_expr) }
                    } else {
                        left_expr.clone()
                    };
                    let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                        parse_quote! { (#right_expr) }
                    } else {
                        right_expr.clone()
                    };

                    if left_is_float && !right_is_float {
                        // Float - Int: cast right to f64
                        Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) })
                    } else if right_is_float && !left_is_float {
                        // Int - Float: cast left to f64
                        Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right })
                    } else {
                        // Same types: no coercion needed
                        Ok(parse_quote! { #safe_left #rust_op #safe_right })
                    }
                }
            }
            BinOp::FloorDiv => {
                // Python floor division semantics differ from Rust integer division
                // Python: rounds towards negative infinity (floor)
                // Rust: truncates towards zero
                // Note: This implementation works for integers with proper floor semantics.
                // Type-based dispatch for float division (using .floor()) would be ideal
                // but requires full type inference integration. This is a known limitation.
                // DEPYLER-0236: Use intermediate variables to avoid formatting issues with != operator

                Ok(parse_quote! {
                    {
                        let a = #left_expr;
                        let b = #right_expr;
                        let q = a / b;
                        let r = a % b;
                        let r_negative = r < 0;
                        let b_negative = b < 0;
                        let r_nonzero = r != 0;
                        let signs_differ = r_negative != b_negative;
                        let needs_adjustment = r_nonzero && signs_differ;
                        if needs_adjustment { q - 1 } else { q }
                    }
                })
            }
            BinOp::Mul => {
                // Special case: [value] * n or n * [value] creates an array
                match (left, right) {
                    // Pattern: [x] * n
                    (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Pattern: n * [x]
                    (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                        if elts.len() == 1 && *size > 0 && *size <= 32 =>
                    {
                        let elem = self.convert(&elts[0])?;
                        let size_lit =
                            syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                        Ok(parse_quote! { [#elem; #size_lit] })
                    }
                    // Default multiplication
                    _ => {
                        let rust_op = convert_binop(op)?;
                        // DEPYLER-0704: Type coercion for mixed int/float multiplication
                        // Rust doesn't auto-coerce, so we need explicit casts
                        let left_is_float = self.expr_returns_float_direct(left);
                        let right_is_float = self.expr_returns_float_direct(right);
                        let left_is_int = self.is_int_expr(left);
                        let right_is_int = self.is_int_expr(right);

                        let final_left = if right_is_float && left_is_int {
                            parse_quote! { (#left_expr as f64) }
                        } else {
                            left_expr
                        };
                        let final_right = if left_is_float && right_is_int {
                            parse_quote! { (#right_expr as f64) }
                        } else {
                            right_expr
                        };
                        // DEPYLER-CLASS-001: Preserve parentheses for operator precedence
                        // 2 * (a + b) must become 2 * (a + b), not 2 * a + b
                        let left_wrapped =
                            precedence::parenthesize_if_lower_precedence(final_left, op);
                        let right_wrapped =
                            precedence::parenthesize_if_lower_precedence(final_right, op);
                        Ok(parse_quote! { #left_wrapped #rust_op #right_wrapped })
                    }
                }
            }
            BinOp::Pow => {
                // Python power operator ** needs type-specific handling in Rust
                // For integers: use .pow() with u32 exponent
                // For floats: use .powf() with f64 exponent
                // For negative integer exponents: convert to float

                // DEPYLER-0699: Wrap expressions in block to ensure correct operator precedence
                // Without this, `a + b as f64` parses as `a + (b as f64)` instead of `(a + b) as f64`
                // DEPYLER-0707: Construct block directly instead of using parse_quote!
                // parse_quote! re-parses tokens which can fail with complex expressions
                fn wrap_expr_in_block(expr: syn::Expr) -> syn::Expr {
                    syn::Expr::Block(syn::ExprBlock {
                        attrs: vec![],
                        label: None,
                        block: syn::Block {
                            brace_token: syn::token::Brace::default(),
                            stmts: vec![syn::Stmt::Expr(expr, None)],
                        },
                    })
                }
                let left_paren = wrap_expr_in_block(left_expr.clone());
                let right_paren = wrap_expr_in_block(right_expr.clone());

                // Check if we have literals to determine types
                match (left, right) {
                    // Integer literal base with integer literal exponent
                    (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                        if *exp < 0 {
                            // Negative exponent: convert to float operation
                            Ok(parse_quote! {
                                (#left_paren as f64).powf(#right_paren as f64)
                            })
                        } else {
                            // Positive integer exponent: use .pow() with u32
                            // Add checked_pow for overflow safety
                            // DEPYLER-0746: Wrap in parens to handle cast expressions
                            Ok(parse_quote! {
                                (#left_expr).checked_pow(#right_expr as u32)
                                    .expect("Power operation overflowed")
                            })
                        }
                    }
                    // Float literal base: always use .powf()
                    // DEPYLER-0408: Cast float literal to f64 for concrete type
                    (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Any base with float exponent: use .powf()
                    // DEPYLER-0408: Cast float literal exponent to f64 for concrete type
                    (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    }),
                    // Variables or complex expressions: generate type-safe code
                    _ => {
                        // For non-literal expressions, we need runtime type checking
                        // This is a conservative approach that works for common cases
                        // DEPYLER-0405: Cast both sides to i64 for type-safe comparison
                        Ok(parse_quote! {
                            {
                                // Try integer power first if exponent can be u32
                                if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                                    (#left_paren as i32).checked_pow(#right_paren as u32)
                                        .expect("Power operation overflowed")
                                } else {
                                    // Fall back to float power for negative or large exponents
                                    // DEPYLER-0401: Use i32 to match common Python int mapping
                                    (#left_paren as f64).powf(#right_paren as f64) as i32
                                }
                            }
                        })
                    }
                }
            }
            // DEPYLER-0720: Handle comparison operators with int-to-float coercion
            BinOp::Gt | BinOp::GtEq | BinOp::Lt | BinOp::LtEq | BinOp::Eq | BinOp::NotEq => {
                let rust_op = convert_binop(op)?;

                // DEPYLER-0824: Wrap cast expressions in parentheses before binary operators
                // Rust parses `x as i32 < y` incorrectly. Must be: `(x as i32) < y`
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };

                // Check if either side is float
                let left_is_float = self.expr_returns_float_direct(left);
                let right_is_float = self.expr_returns_float_direct(right);

                // DEPYLER-0828: Handle float/int comparisons with proper coercion
                // DEPYLER-1051: Helper to extract integer value from literal or negative literal
                fn extract_int_value(expr: &HirExpr) -> Option<i64> {
                    match expr {
                        HirExpr::Literal(Literal::Int(n)) => Some(*n),
                        // Handle -N which is represented as Unary(Neg, Literal(Int(N)))
                        HirExpr::Unary {
                            op: UnaryOp::Neg,
                            operand,
                        } => {
                            if let HirExpr::Literal(Literal::Int(n)) = operand.as_ref() {
                                Some(-*n)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }

                // If left is float and right is integer (literal or variable), convert right to float
                if left_is_float && !right_is_float {
                    if let Some(n) = extract_int_value(right) {
                        // Integer literal: convert at compile time
                        // DEPYLER-1051: Use explicit f64 suffix to avoid tokenization issues with negative numbers
                        let float_lit = syn::LitFloat::new(
                            &format!("{}f64", n),
                            proc_macro2::Span::call_site(),
                        );
                        return Ok(parse_quote! { #safe_left #rust_op #float_lit });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) });
                }

                // If right is float and left is integer (literal or variable), convert left to float
                if right_is_float && !left_is_float {
                    if let Some(n) = extract_int_value(left) {
                        // Integer literal: convert at compile time
                        // DEPYLER-1051: Use explicit f64 suffix to avoid tokenization issues with negative numbers
                        let float_lit = syn::LitFloat::new(
                            &format!("{}f64", n),
                            proc_macro2::Span::call_site(),
                        );
                        return Ok(parse_quote! { #float_lit #rust_op #safe_right });
                    }
                    // Integer variable or expression: cast to f64 at runtime
                    return Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right });
                }

                // No coercion needed (both same type)
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
            // DEPYLER-E0308-001: Handle string concatenation with format!()
            // Python: "a" + "b" → Rust: format!("{}{}", a, b)
            // The Rust + operator on strings requires &str on the right side
            BinOp::Add if self.is_string_expr(left) || self.is_string_expr(right) => {
                Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
            }
            _ => {
                let rust_op = convert_binop(op)?;
                // DEPYLER-0824: Wrap cast expressions in parentheses
                let safe_left: syn::Expr = if matches!(left_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#left_expr) }
                } else {
                    left_expr.clone()
                };
                let safe_right: syn::Expr = if matches!(right_expr, syn::Expr::Cast(_)) {
                    parse_quote! { (#right_expr) }
                } else {
                    right_expr.clone()
                };
                Ok(parse_quote! { #safe_left #rust_op #safe_right })
            }
        }
    }

    /// DEPYLER-1096: Apply truthiness coercion to an expression for use in if/while conditions.
    /// Python allows any type in conditions (truthy/falsy), Rust requires bool.
    /// Returns: A boolean expression suitable for use as a condition.
    pub(super) fn apply_truthiness_coercion(&self, expr: &HirExpr, rust_expr: syn::Expr) -> syn::Expr {
        // Check for already-boolean expressions (comparisons, boolean ops, bool literals)
        let is_already_bool = matches!(
            expr,
            // Comparison operators return bool
            HirExpr::Binary {
                op: BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq
                    | BinOp::In
                    | BinOp::NotIn
                    | BinOp::And
                    | BinOp::Or,
                ..
            } | HirExpr::Literal(Literal::Bool(_))
        );

        if is_already_bool {
            return rust_expr;
        }

        // Check for Not unary - it already returns bool
        if matches!(
            expr,
            HirExpr::Unary {
                op: UnaryOp::Not,
                ..
            }
        ) {
            return rust_expr;
        }

        // Check if this is a method call returning bool (e.g., .startswith(), .endswith())
        let is_bool_method = if let HirExpr::MethodCall { method, .. } = expr {
            matches!(
                method.as_str(),
                "startswith"
                    | "endswith"
                    | "contains"
                    | "is_empty"
                    | "is_some"
                    | "is_none"
                    | "is_ok"
                    | "is_err"
                    | "isalpha"
                    | "isdigit"
                    | "isalnum"
                    | "isupper"
                    | "islower"
            )
        } else {
            false
        };

        if is_bool_method {
            return rust_expr;
        }

        // Check for collection types (need !is_empty())
        let is_collection = if let HirExpr::Attribute { value, attr } = expr {
            if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                if let Some(field_type) = self.class_field_types.get(attr) {
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
        } else if let HirExpr::Var(name) = expr {
            // Check if variable name suggests it's a collection
            crate::rust_gen::truthiness_helpers::is_collection_var_name(name)
                || crate::rust_gen::truthiness_helpers::is_string_var_name(name)
        } else {
            false
        };

        if is_collection {
            return parse_quote! { !#rust_expr.is_empty() };
        }

        // Check for Option types (need is_some())
        let is_option = if let HirExpr::Attribute { value, attr } = expr {
            if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                if let Some(field_type) = self.class_field_types.get(attr) {
                    matches!(field_type, Type::Optional(_))
                } else {
                    false
                }
            } else {
                false
            }
        } else if let HirExpr::Var(name) = expr {
            crate::rust_gen::truthiness_helpers::is_option_var_name(name)
        } else {
            false
        };

        if is_option {
            return parse_quote! { #rust_expr.is_some() };
        }

        // Check for numeric types (need != 0)
        let is_numeric = if let HirExpr::Var(name) = expr {
            if let Some(var_type) = self.param_types.get(name) {
                matches!(var_type, Type::Int | Type::Float)
            } else {
                false
            }
        } else if let HirExpr::Attribute { value, attr } = expr {
            if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                if let Some(field_type) = self.class_field_types.get(attr) {
                    matches!(field_type, Type::Int | Type::Float)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if is_numeric {
            return parse_quote! { #rust_expr != 0 };
        }

        // Default: assume the expression already evaluates to bool or use as-is
        // This covers direct bool variables and method calls returning bool
        rust_expr
    }

    pub(super) fn convert_unary(&self, op: UnaryOp, operand: &HirExpr) -> Result<syn::Expr> {
        let operand_expr = self.convert(operand)?;
        match op {
            UnaryOp::Not => {
                // DEPYLER-0966: Check if operand is a collection type for truthiness transformation
                // Python: `if not self.heap:` where self.heap is list[int]
                // Rust: Must use `.is_empty()` instead of `!` for Vec types
                let is_collection = if let HirExpr::Attribute { value, attr } = operand {
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.class_field_types.get(attr) {
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

                // DEPYLER-0966: Check if operand is an Optional class field
                let is_optional = if let HirExpr::Attribute { value, attr } = operand {
                    if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                        if let Some(field_type) = self.class_field_types.get(attr) {
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

                if is_collection {
                    Ok(parse_quote! { #operand_expr.is_empty() })
                } else if is_optional {
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


}
