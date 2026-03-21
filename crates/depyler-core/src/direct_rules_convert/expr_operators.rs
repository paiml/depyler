//! Expression operator conversion (binary, unary, truthiness) for ExprConverter

use crate::direct_rules::make_ident;
use crate::hir::*;
use crate::rust_gen::precedence;
use anyhow::Result;
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

    pub(super) fn convert_binary(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
    ) -> Result<syn::Expr> {
        let left_expr = self.convert(left)?;
        let right_expr = self.convert(right)?;

        match op {
            BinOp::In => self.convert_in_op(left, right, left_expr, right_expr),
            BinOp::NotIn => self.convert_not_in_op(left, right, left_expr, right_expr),
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor
                if self.is_set_expr(left) && self.is_set_expr(right) =>
            {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub if self.is_set_expr(left) && self.is_set_expr(right) => {
                self.convert_set_operation(op, left_expr, right_expr)
            }
            BinOp::Sub => self.convert_sub_op(left, right, left_expr, right_expr),
            BinOp::FloorDiv => convert_floor_div(left_expr, right_expr),
            BinOp::Mul => self.convert_mul_op(op, left, right, left_expr, right_expr),
            BinOp::Pow => self.convert_pow_op(left, right, left_expr, right_expr),
            BinOp::Gt | BinOp::GtEq | BinOp::Lt | BinOp::LtEq | BinOp::Eq | BinOp::NotEq => {
                self.convert_comparison_op(op, left, right, left_expr, right_expr)
            }
            BinOp::Add if self.is_string_expr(left) || self.is_string_expr(right) => {
                Ok(parse_quote! { format!("{}{}", #left_expr, #right_expr) })
            }
            _ => convert_default_binary(op, left_expr, right_expr),
        }
    }

    fn convert_in_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        if self.is_dict_expr(right) {
            Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() })
        } else if self.is_tuple_or_list_expr(right) {
            let elements = self.convert_collection_elements(right, &right_expr)?;
            Ok(parse_quote! { [#(#elements),*].contains(&#left_expr) })
        } else if self.is_string_expr(right) {
            let pattern = make_string_pattern(left, &left_expr);
            Ok(parse_quote! { #right_expr.contains(#pattern) })
        } else if self.is_set_expr(right) {
            Ok(parse_quote! { #right_expr.contains(&#left_expr) })
        } else {
            Ok(parse_quote! { #right_expr.get(&#left_expr).is_some() })
        }
    }

    fn convert_not_in_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        if self.is_dict_expr(right) {
            Ok(parse_quote! { #right_expr.get(&#left_expr).is_none() })
        } else if self.is_tuple_or_list_expr(right) {
            let elements = self.convert_collection_elements(right, &right_expr)?;
            Ok(parse_quote! { ![#(#elements),*].contains(&#left_expr) })
        } else if self.is_string_expr(right) {
            let pattern = make_string_pattern(left, &left_expr);
            Ok(parse_quote! { !#right_expr.contains(#pattern) })
        } else if self.is_set_expr(right) {
            Ok(parse_quote! { !#right_expr.contains(&#left_expr) })
        } else {
            Ok(parse_quote! { #right_expr.get(&#left_expr).is_none() })
        }
    }

    fn convert_collection_elements(
        &self,
        right: &HirExpr,
        right_expr: &syn::Expr,
    ) -> Result<Vec<syn::Expr>> {
        match right {
            HirExpr::Tuple(elems) | HirExpr::List(elems) => {
                elems.iter().map(|e| self.convert(e)).collect::<Result<Vec<_>>>()
            }
            _ => Ok(vec![right_expr.clone()]),
        }
    }

    fn convert_sub_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        if is_len_call(left) {
            return Ok(parse_quote! { (#left_expr).saturating_sub(#right_expr) });
        }

        let rust_op = convert_binop(BinOp::Sub)?;
        let left_is_float = self.expr_returns_float_direct(left);
        let right_is_float = self.expr_returns_float_direct(right);
        let safe_left = wrap_cast_in_parens(left_expr);
        let safe_right = wrap_cast_in_parens(right_expr);

        if left_is_float && !right_is_float {
            Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) })
        } else if right_is_float && !left_is_float {
            Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right })
        } else {
            Ok(parse_quote! { #safe_left #rust_op #safe_right })
        }
    }

    fn convert_mul_op(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        match (left, right) {
            (HirExpr::List(elts), HirExpr::Literal(Literal::Int(size)))
                if elts.len() == 1 && *size > 0 && *size <= 32 =>
            {
                let elem = self.convert(&elts[0])?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { [#elem; #size_lit] })
            }
            (HirExpr::Literal(Literal::Int(size)), HirExpr::List(elts))
                if elts.len() == 1 && *size > 0 && *size <= 32 =>
            {
                let elem = self.convert(&elts[0])?;
                let size_lit = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
                Ok(parse_quote! { [#elem; #size_lit] })
            }
            _ => {
                let rust_op = convert_binop(op)?;
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
                let left_wrapped = precedence::parenthesize_if_lower_precedence(final_left, op);
                let right_wrapped = precedence::parenthesize_if_lower_precedence(final_right, op);
                Ok(parse_quote! { #left_wrapped #rust_op #right_wrapped })
            }
        }
    }

    fn convert_pow_op(
        &self,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        let left_paren = wrap_expr_in_block(left_expr.clone());
        let right_paren = wrap_expr_in_block(right_expr.clone());

        match (left, right) {
            (HirExpr::Literal(Literal::Int(_)), HirExpr::Literal(Literal::Int(exp))) => {
                if *exp < 0 {
                    Ok(parse_quote! {
                        (#left_paren as f64).powf(#right_paren as f64)
                    })
                } else {
                    Ok(parse_quote! {
                        (#left_expr).checked_pow(#right_expr as u32)
                            .expect("Power operation overflowed")
                    })
                }
            }
            (HirExpr::Literal(Literal::Float(_)), _) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            (_, HirExpr::Literal(Literal::Float(_))) => Ok(parse_quote! {
                (#left_paren as f64).powf(#right_paren as f64)
            }),
            _ => Ok(parse_quote! {
                {
                    if #right_expr >= 0 && (#right_expr as i64) <= (u32::MAX as i64) {
                        (#left_paren as i32).checked_pow(#right_paren as u32)
                            .expect("Power operation overflowed")
                    } else {
                        (#left_paren as f64).powf(#right_paren as f64) as i32
                    }
                }
            }),
        }
    }

    fn convert_comparison_op(
        &self,
        op: BinOp,
        left: &HirExpr,
        right: &HirExpr,
        left_expr: syn::Expr,
        right_expr: syn::Expr,
    ) -> Result<syn::Expr> {
        let rust_op = convert_binop(op)?;
        let safe_left = wrap_cast_in_parens(left_expr);
        let safe_right = wrap_cast_in_parens(right_expr);

        let left_is_float = self.expr_returns_float_direct(left);
        let right_is_float = self.expr_returns_float_direct(right);

        if left_is_float && !right_is_float {
            if let Some(n) = extract_int_value(right) {
                let float_lit =
                    syn::LitFloat::new(&format!("{}f64", n), proc_macro2::Span::call_site());
                return Ok(parse_quote! { #safe_left #rust_op #float_lit });
            }
            return Ok(parse_quote! { #safe_left #rust_op (#safe_right as f64) });
        }

        if right_is_float && !left_is_float {
            if let Some(n) = extract_int_value(left) {
                let float_lit =
                    syn::LitFloat::new(&format!("{}f64", n), proc_macro2::Span::call_site());
                return Ok(parse_quote! { #float_lit #rust_op #safe_right });
            }
            return Ok(parse_quote! { (#safe_left as f64) #rust_op #safe_right });
        }

        Ok(parse_quote! { #safe_left #rust_op #safe_right })
    }

    /// DEPYLER-1096: Apply truthiness coercion to an expression for use in if/while conditions.
    /// Python allows any type in conditions (truthy/falsy), Rust requires bool.
    /// Returns: A boolean expression suitable for use as a condition.
    pub(super) fn apply_truthiness_coercion(
        &self,
        expr: &HirExpr,
        rust_expr: syn::Expr,
    ) -> syn::Expr {
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
        if matches!(expr, HirExpr::Unary { op: UnaryOp::Not, .. }) {
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
                    // DEPYLER-99MODE: Heuristic fallback for collection/string attribute names
                    crate::rust_gen::truthiness_helpers::is_collection_attr_name(attr)
                        || crate::rust_gen::truthiness_helpers::is_string_attr_name(attr)
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

fn make_string_pattern(left: &HirExpr, left_expr: &syn::Expr) -> syn::Expr {
    match left {
        HirExpr::Literal(Literal::String(s)) => {
            let lit = syn::LitStr::new(s, proc_macro2::Span::call_site());
            parse_quote! { #lit }
        }
        _ => {
            parse_quote! { &*#left_expr }
        }
    }
}

fn wrap_cast_in_parens(expr: syn::Expr) -> syn::Expr {
    if matches!(expr, syn::Expr::Cast(_)) {
        parse_quote! { (#expr) }
    } else {
        expr
    }
}

fn convert_floor_div(left_expr: syn::Expr, right_expr: syn::Expr) -> Result<syn::Expr> {
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

fn convert_default_binary(
    op: BinOp,
    left_expr: syn::Expr,
    right_expr: syn::Expr,
) -> Result<syn::Expr> {
    let rust_op = convert_binop(op)?;
    let safe_left = wrap_cast_in_parens(left_expr);
    let safe_right = wrap_cast_in_parens(right_expr);
    Ok(parse_quote! { #safe_left #rust_op #safe_right })
}

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

fn extract_int_value(expr: &HirExpr) -> Option<i64> {
    match expr {
        HirExpr::Literal(Literal::Int(n)) => Some(*n),
        HirExpr::Unary { op: UnaryOp::Neg, operand } => {
            if let HirExpr::Literal(Literal::Int(n)) = operand.as_ref() {
                Some(-*n)
            } else {
                None
            }
        }
        _ => None,
    }
}
