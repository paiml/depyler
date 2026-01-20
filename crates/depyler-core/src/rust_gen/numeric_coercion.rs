//! Numeric Type Coercion Helpers
//!
//! This module contains helpers for numeric type coercion during code generation.
//! Extracted from expr_gen.rs for better testability.
//!
//! DEPYLER-0582, DEPYLER-0694, DEPYLER-0805: Numeric type coercion handling

use crate::hir::{BinOp, HirExpr, Literal, Type};
use std::collections::HashMap;

/// Check if expression is a variable with integer type
pub fn is_int_var(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    if let HirExpr::Var(name) = expr {
        if let Some(var_type) = var_types.get(name) {
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
pub fn is_float_var(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    if let HirExpr::Var(name) = expr {
        if let Some(var_type) = var_types.get(name) {
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
        if is_common_float_name(&name_lower) {
            return true;
        }
        // DEPYLER-0950: Color channel heuristic
        if is_color_channel_name(name.as_str()) {
            return true;
        }
    }
    false
}

/// Check if name is a common float parameter name
pub fn is_common_float_name(name_lower: &str) -> bool {
    name_lower.contains("beta")
        || name_lower.contains("alpha")
        || name_lower.contains("lr")
        || name_lower.contains("eps")
        || name_lower.contains("rate")
        || name_lower.contains("momentum")
}

/// Check if name is a color channel variable (typically f64)
pub fn is_color_channel_name(name: &str) -> bool {
    matches!(name, "r" | "g" | "h" | "s" | "v" | "l" | "c" | "m" | "k")
}

/// Check if expression evaluates to an integer type
/// Handles variables, literals, and binary operations on integers
pub fn is_int_expr(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    match expr {
        HirExpr::Var(name) => {
            if let Some(var_type) = var_types.get(name) {
                matches!(var_type, Type::Int)
            } else {
                false
            }
        }
        HirExpr::Literal(Literal::Int(_)) => true,
        // Binary operations on integers produce integers
        HirExpr::Binary { left, right, op } => {
            // Division in Python returns Float, so we don't include Div
            if matches!(
                op,
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Mod | BinOp::FloorDiv
            ) {
                is_int_expr(left, var_types) && is_int_expr(right, var_types)
            } else {
                false
            }
        }
        // Unary minus on integer is still integer
        HirExpr::Unary { operand, .. } => is_int_expr(operand, var_types),
        _ => false,
    }
}

/// Check if expression returns a float type
pub fn expr_returns_float(expr: &HirExpr, var_types: &HashMap<String, Type>) -> bool {
    match expr {
        HirExpr::Literal(Literal::Float(_)) => true,
        HirExpr::Var(name) => {
            if let Some(Type::Float) = var_types.get(name) {
                return true;
            }
            if let Some(Type::Custom(s)) = var_types.get(name) {
                return s == "f64" || s == "f32";
            }
            false
        }
        HirExpr::Binary { left, right, op } => {
            // Division always returns float
            if matches!(op, BinOp::Div) {
                return true;
            }
            // Pow with float operand returns float
            if matches!(op, BinOp::Pow)
                && (expr_returns_float(left, var_types) || expr_returns_float(right, var_types))
            {
                return true;
            }
            // Other ops with float operand return float
            expr_returns_float(left, var_types) || expr_returns_float(right, var_types)
        }
        HirExpr::Unary { operand, .. } => expr_returns_float(operand, var_types),
        HirExpr::Call { func, .. } => {
            // Math functions typically return float
            matches!(
                func.as_str(),
                "sin" | "cos" | "tan" | "sqrt" | "log" | "exp" | "pow" | "abs"
            )
        }
        _ => false,
    }
}

/// Determine if we need to cast an integer to float
pub fn needs_float_coercion(
    expr: &HirExpr,
    other: &HirExpr,
    var_types: &HashMap<String, Type>,
) -> bool {
    let is_int = is_int_expr(expr, var_types) || is_int_var(expr, var_types);
    let other_is_float = expr_returns_float(other, var_types) || is_float_var(other, var_types);
    is_int && other_is_float
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::UnaryOp;

    fn make_var_types() -> HashMap<String, Type> {
        let mut map = HashMap::new();
        map.insert("x".to_string(), Type::Int);
        map.insert("y".to_string(), Type::Float);
        map.insert("z".to_string(), Type::String);
        map.insert("i32_var".to_string(), Type::Custom("i32".to_string()));
        map.insert("f64_var".to_string(), Type::Custom("f64".to_string()));
        map
    }

    // ============ is_int_var tests ============

    #[test]
    fn test_is_int_var_type_int() {
        let var_types = make_var_types();
        assert!(is_int_var(&HirExpr::Var("x".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_var_type_float() {
        let var_types = make_var_types();
        assert!(!is_int_var(&HirExpr::Var("y".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_var_type_custom_i32() {
        let var_types = make_var_types();
        assert!(is_int_var(
            &HirExpr::Var("i32_var".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_int_var_type_custom_i64() {
        let mut var_types = HashMap::new();
        var_types.insert("v".to_string(), Type::Custom("i64".to_string()));
        assert!(is_int_var(&HirExpr::Var("v".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_var_type_custom_usize() {
        let mut var_types = HashMap::new();
        var_types.insert("v".to_string(), Type::Custom("usize".to_string()));
        assert!(is_int_var(&HirExpr::Var("v".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_var_type_custom_isize() {
        let mut var_types = HashMap::new();
        var_types.insert("v".to_string(), Type::Custom("isize".to_string()));
        assert!(is_int_var(&HirExpr::Var("v".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_var_not_in_map() {
        let var_types = make_var_types();
        assert!(!is_int_var(
            &HirExpr::Var("unknown".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_int_var_not_var() {
        let var_types = make_var_types();
        assert!(!is_int_var(&HirExpr::Literal(Literal::Int(42)), &var_types));
    }

    // ============ is_float_var tests ============

    #[test]
    fn test_is_float_var_type_float() {
        let var_types = make_var_types();
        assert!(is_float_var(&HirExpr::Var("y".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_type_int() {
        let var_types = make_var_types();
        assert!(!is_float_var(&HirExpr::Var("x".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_type_custom_f64() {
        let var_types = make_var_types();
        assert!(is_float_var(
            &HirExpr::Var("f64_var".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_float_var_type_custom_f32() {
        let mut var_types = HashMap::new();
        var_types.insert("v".to_string(), Type::Custom("f32".to_string()));
        assert!(is_float_var(&HirExpr::Var("v".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_beta() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("beta1".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_alpha() {
        let var_types = HashMap::new();
        assert!(is_float_var(
            &HirExpr::Var("Alpha".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_float_var_heuristic_lr() {
        let var_types = HashMap::new();
        assert!(is_float_var(
            &HirExpr::Var("learning_lr".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_float_var_heuristic_eps() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("eps".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_heuristic_rate() {
        let var_types = HashMap::new();
        assert!(is_float_var(
            &HirExpr::Var("rate_decay".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_float_var_heuristic_momentum() {
        let var_types = HashMap::new();
        assert!(is_float_var(
            &HirExpr::Var("momentum".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_is_float_var_color_channel_r() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("r".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_color_channel_g() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("g".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_color_channel_h() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("h".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_color_channel_s() {
        let var_types = HashMap::new();
        assert!(is_float_var(&HirExpr::Var("s".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_not_color_channel_a() {
        // 'a' and 'b' are NOT considered color channels (too generic)
        let var_types = HashMap::new();
        assert!(!is_float_var(&HirExpr::Var("a".to_string()), &var_types));
    }

    #[test]
    fn test_is_float_var_not_var() {
        let var_types = make_var_types();
        assert!(!is_float_var(
            &HirExpr::Literal(Literal::Float(3.15)),
            &var_types
        ));
    }

    // ============ is_common_float_name tests ============

    #[test]
    fn test_common_float_name_beta() {
        assert!(is_common_float_name("beta1"));
        assert!(is_common_float_name("my_beta"));
    }

    #[test]
    fn test_common_float_name_alpha() {
        assert!(is_common_float_name("alpha"));
        assert!(is_common_float_name("alpha_decay"));
    }

    #[test]
    fn test_common_float_name_lr() {
        assert!(is_common_float_name("lr"));
        assert!(is_common_float_name("learning_lr"));
    }

    #[test]
    fn test_common_float_name_eps() {
        assert!(is_common_float_name("eps"));
        assert!(is_common_float_name("epsilon"));
    }

    #[test]
    fn test_common_float_name_rate() {
        assert!(is_common_float_name("rate"));
        assert!(is_common_float_name("learning_rate"));
    }

    #[test]
    fn test_common_float_name_momentum() {
        assert!(is_common_float_name("momentum"));
    }

    #[test]
    fn test_common_float_name_not_common() {
        assert!(!is_common_float_name("count"));
        assert!(!is_common_float_name("index"));
        assert!(!is_common_float_name("size"));
    }

    // ============ is_color_channel_name tests ============

    #[test]
    fn test_color_channel_rgb() {
        assert!(is_color_channel_name("r"));
        assert!(is_color_channel_name("g"));
    }

    #[test]
    fn test_color_channel_hsv() {
        assert!(is_color_channel_name("h"));
        assert!(is_color_channel_name("s"));
        assert!(is_color_channel_name("v"));
    }

    #[test]
    fn test_color_channel_hsl() {
        assert!(is_color_channel_name("l"));
    }

    #[test]
    fn test_color_channel_cmyk() {
        assert!(is_color_channel_name("c"));
        assert!(is_color_channel_name("m"));
        assert!(is_color_channel_name("k"));
    }

    #[test]
    fn test_color_channel_not_generic() {
        // 'a', 'b', 'x', 'y' are too generic and should NOT be color channels
        assert!(!is_color_channel_name("a"));
        assert!(!is_color_channel_name("b"));
        assert!(!is_color_channel_name("x"));
        assert!(!is_color_channel_name("y"));
    }

    #[test]
    fn test_color_channel_not_multi_char() {
        assert!(!is_color_channel_name("rgb"));
        assert!(!is_color_channel_name("red"));
    }

    // ============ is_int_expr tests ============

    #[test]
    fn test_is_int_expr_literal() {
        let var_types = HashMap::new();
        assert!(is_int_expr(&HirExpr::Literal(Literal::Int(42)), &var_types));
    }

    #[test]
    fn test_is_int_expr_float_literal() {
        let var_types = HashMap::new();
        assert!(!is_int_expr(
            &HirExpr::Literal(Literal::Float(3.15)),
            &var_types
        ));
    }

    #[test]
    fn test_is_int_expr_var() {
        let var_types = make_var_types();
        assert!(is_int_expr(&HirExpr::Var("x".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_expr_var_float() {
        let var_types = make_var_types();
        assert!(!is_int_expr(&HirExpr::Var("y".to_string()), &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_add() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(1))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_sub() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Sub,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_mul() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Mul,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Literal(Literal::Int(2))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_mod() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Mod,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_floordiv() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::FloorDiv,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_div() {
        // Division returns float, not int
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_binary_with_float() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Var("x".to_string())),
            right: Box::new(HirExpr::Var("y".to_string())), // y is float
        };
        assert!(!is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_unary() {
        let var_types = make_var_types();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("x".to_string())),
        };
        assert!(is_int_expr(&expr, &var_types));
    }

    #[test]
    fn test_is_int_expr_call() {
        let var_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_int_expr(&expr, &var_types));
    }

    // ============ expr_returns_float tests ============

    #[test]
    fn test_expr_returns_float_literal() {
        let var_types = HashMap::new();
        assert!(expr_returns_float(
            &HirExpr::Literal(Literal::Float(3.15)),
            &var_types
        ));
    }

    #[test]
    fn test_expr_returns_float_int_literal() {
        let var_types = HashMap::new();
        assert!(!expr_returns_float(
            &HirExpr::Literal(Literal::Int(42)),
            &var_types
        ));
    }

    #[test]
    fn test_expr_returns_float_var() {
        let var_types = make_var_types();
        assert!(expr_returns_float(
            &HirExpr::Var("y".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_expr_returns_float_var_custom_f64() {
        let var_types = make_var_types();
        assert!(expr_returns_float(
            &HirExpr::Var("f64_var".to_string()),
            &var_types
        ));
    }

    #[test]
    fn test_expr_returns_float_division() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Div,
            left: Box::new(HirExpr::Literal(Literal::Int(10))),
            right: Box::new(HirExpr::Literal(Literal::Int(3))),
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_pow_with_float() {
        let var_types = HashMap::new();
        let expr = HirExpr::Binary {
            op: BinOp::Pow,
            left: Box::new(HirExpr::Literal(Literal::Int(2))),
            right: Box::new(HirExpr::Literal(Literal::Float(0.5))),
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_binary_with_float() {
        let var_types = make_var_types();
        let expr = HirExpr::Binary {
            op: BinOp::Add,
            left: Box::new(HirExpr::Literal(Literal::Int(1))),
            right: Box::new(HirExpr::Var("y".to_string())), // y is float
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_unary() {
        let var_types = make_var_types();
        let expr = HirExpr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(HirExpr::Var("y".to_string())),
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_call_sin() {
        let var_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "sin".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_call_cos() {
        let var_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "cos".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_call_sqrt() {
        let var_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "sqrt".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(expr_returns_float(&expr, &var_types));
    }

    #[test]
    fn test_expr_returns_float_call_not_math() {
        let var_types = HashMap::new();
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!expr_returns_float(&expr, &var_types));
    }

    // ============ needs_float_coercion tests ============

    #[test]
    fn test_needs_float_coercion_int_with_float() {
        let var_types = make_var_types();
        let int_expr = HirExpr::Var("x".to_string());
        let float_expr = HirExpr::Var("y".to_string());
        assert!(needs_float_coercion(&int_expr, &float_expr, &var_types));
    }

    #[test]
    fn test_needs_float_coercion_float_with_int() {
        let var_types = make_var_types();
        let float_expr = HirExpr::Var("y".to_string());
        let int_expr = HirExpr::Var("x".to_string());
        assert!(!needs_float_coercion(&float_expr, &int_expr, &var_types));
    }

    #[test]
    fn test_needs_float_coercion_int_with_int() {
        let var_types = make_var_types();
        let int_expr1 = HirExpr::Var("x".to_string());
        let int_expr2 = HirExpr::Literal(Literal::Int(1));
        assert!(!needs_float_coercion(&int_expr1, &int_expr2, &var_types));
    }

    #[test]
    fn test_needs_float_coercion_int_literal_with_float() {
        let var_types = make_var_types();
        let int_lit = HirExpr::Literal(Literal::Int(1));
        let float_var = HirExpr::Var("y".to_string());
        assert!(needs_float_coercion(&int_lit, &float_var, &var_types));
    }
}
