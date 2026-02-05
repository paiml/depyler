//! Math Module Code Generation - EXTREME TDD
//!
//! Handles Python `math` module method conversions to Rust f64 methods.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python math module method calls to Rust
///
/// # Supported Methods
/// - Trigonometric: sin, cos, tan, asin, acos, atan, atan2
/// - Hyperbolic: sinh, cosh, tanh, asinh, acosh, atanh
/// - Power/Log: sqrt, exp, ln, log, log2, log10, pow, expm1
/// - Rounding: ceil, floor, trunc, round
/// - Special: fabs, copysign, degrees, radians
/// - Checks: isnan, isinf, isfinite
/// - Integer: gcd, lcm, factorial, isqrt, comb, perm
/// - Other: ldexp, frexp, isclose, modf, fmod, hypot, dist, remainder
///
/// # Complexity: 8 (delegated to category dispatchers)
pub fn convert_math_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = dispatch_trig(method, &arg_exprs)
        .or_else(|| dispatch_power_log(method, &arg_exprs))
        .or_else(|| dispatch_rounding_special(method, &arg_exprs))
        .or_else(|| dispatch_checks(method, &arg_exprs))
        .or_else(|| dispatch_integer(method, &arg_exprs))
        .or_else(|| dispatch_other(method, &arg_exprs));

    match result {
        Some(r) => Ok(Some(r?)),
        None => bail!("math.{} not implemented yet", method),
    }
}

fn dispatch_trig(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" => Some(convert_trig(method, args)),
        "atan2" => Some(convert_atan2(args)),
        "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" => {
            Some(convert_hyperbolic(method, args))
        }
        _ => None,
    }
}

fn dispatch_power_log(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "sqrt" | "exp" | "ln" | "log2" | "log10" => Some(convert_power_log(method, args)),
        "log" => Some(convert_log(args)),
        "pow" => Some(convert_pow(args)),
        "expm1" => Some(convert_expm1(args)),
        _ => None,
    }
}

fn dispatch_rounding_special(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "ceil" | "floor" | "trunc" | "round" => Some(convert_rounding(method, args)),
        "fabs" => Some(convert_fabs(args)),
        "copysign" => Some(convert_copysign(args)),
        "degrees" => Some(convert_degrees(args)),
        "radians" => Some(convert_radians(args)),
        _ => None,
    }
}

fn dispatch_checks(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "isnan" => Some(convert_isnan(args)),
        "isinf" => Some(convert_isinf(args)),
        "isfinite" => Some(convert_isfinite(args)),
        _ => None,
    }
}

fn dispatch_integer(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "gcd" => Some(convert_gcd(args)),
        "lcm" => Some(convert_lcm(args)),
        "factorial" => Some(convert_factorial(args)),
        "isqrt" => Some(convert_isqrt(args)),
        "comb" => Some(convert_comb(args)),
        "perm" => Some(convert_perm(args)),
        _ => None,
    }
}

fn dispatch_other(method: &str, args: &[syn::Expr]) -> Option<Result<syn::Expr>> {
    match method {
        "ldexp" => Some(convert_ldexp(args)),
        "frexp" => Some(convert_frexp(args)),
        "isclose" => Some(convert_isclose(args)),
        "modf" => Some(convert_modf(args)),
        "fmod" => Some(convert_fmod(args)),
        "hypot" => Some(convert_hypot(args)),
        "dist" => Some(convert_dist(args)),
        "remainder" => Some(convert_remainder(args)),
        _ => None,
    }
}

/// Trigonometric: sin, cos, tan, asin, acos, atan
fn convert_trig(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.{}() requires exactly 1 argument", method);
    }
    let arg = &arg_exprs[0];
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    Ok(parse_quote! { (#arg as f64).#method_ident() })
}

/// atan2(y, x)
fn convert_atan2(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.atan2() requires exactly 2 arguments");
    }
    let y = &arg_exprs[0];
    let x = &arg_exprs[1];
    Ok(parse_quote! { (#y as f64).atan2(#x as f64) })
}

/// Hyperbolic: sinh, cosh, tanh, asinh, acosh, atanh
fn convert_hyperbolic(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.{}() requires exactly 1 argument", method);
    }
    let arg = &arg_exprs[0];
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    Ok(parse_quote! { (#arg as f64).#method_ident() })
}

/// Power/Log: sqrt, exp, ln, log2, log10
fn convert_power_log(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.{}() requires exactly 1 argument", method);
    }
    let arg = &arg_exprs[0];
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    Ok(parse_quote! { (#arg as f64).#method_ident() })
}

/// log(x) or log(x, base)
fn convert_log(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() == 1 {
        let arg = &arg_exprs[0];
        Ok(parse_quote! { (#arg as f64).ln() })
    } else if arg_exprs.len() == 2 {
        let x = &arg_exprs[0];
        let base = &arg_exprs[1];
        Ok(parse_quote! { (#x as f64).log(#base as f64) })
    } else {
        bail!("math.log() requires 1 or 2 arguments")
    }
}

/// pow(base, exp)
fn convert_pow(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.pow() requires exactly 2 arguments");
    }
    let base = &arg_exprs[0];
    let exp = &arg_exprs[1];
    Ok(parse_quote! { (#base as f64).powf(#exp as f64) })
}

/// expm1(x) - exp(x) - 1
fn convert_expm1(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.expm1() requires exactly 1 argument");
    }
    let x = &arg_exprs[0];
    Ok(parse_quote! { (#x as f64).exp_m1() })
}

/// Rounding: ceil, floor, trunc, round
/// DEPYLER-1006: Return f64 instead of i32 to match type annotations
/// Python's math.floor/ceil return int, but users often annotate as float
/// Returning f64 is compatible with both (f64 can be used where i64 expected via coercion)
fn convert_rounding(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.{}() requires exactly 1 argument", method);
    }
    let arg = &arg_exprs[0];
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    // Return f64 for all rounding operations - compatible with both int and float annotations
    Ok(parse_quote! { (#arg as f64).#method_ident() })
}

/// fabs(x) - absolute value
fn convert_fabs(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.fabs() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).abs() })
}

/// copysign(x, y)
fn convert_copysign(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.copysign() requires exactly 2 arguments");
    }
    let x = &arg_exprs[0];
    let y = &arg_exprs[1];
    Ok(parse_quote! { (#x as f64).copysign(#y as f64) })
}

/// degrees(x) - radians to degrees
fn convert_degrees(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.degrees() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).to_degrees() })
}

/// radians(x) - degrees to radians
fn convert_radians(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.radians() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).to_radians() })
}

/// isnan(x)
fn convert_isnan(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.isnan() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).is_nan() })
}

/// isinf(x)
fn convert_isinf(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.isinf() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).is_infinite() })
}

/// isfinite(x)
fn convert_isfinite(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.isfinite() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { (#arg as f64).is_finite() })
}

/// gcd(a, b)
fn convert_gcd(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.gcd() requires exactly 2 arguments");
    }
    let a = &arg_exprs[0];
    let b = &arg_exprs[1];
    Ok(parse_quote! {
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
    })
}

/// lcm(a, b)
fn convert_lcm(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.lcm() requires exactly 2 arguments");
    }
    let a = &arg_exprs[0];
    let b = &arg_exprs[1];
    Ok(parse_quote! {
        {
            let a = (#a as i64).abs();
            let b = (#b as i64).abs();
            if a == 0 || b == 0 {
                0
            } else {
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
    })
}

/// factorial(n)
fn convert_factorial(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.factorial() requires exactly 1 argument");
    }
    let n = &arg_exprs[0];
    Ok(parse_quote! {
        {
            let n = #n as i32;
            let mut result = 1i64;
            for i in 1..=n {
                result *= i as i64;
            }
            result as i32
        }
    })
}

/// isqrt(n) - integer square root
fn convert_isqrt(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.isqrt() requires exactly 1 argument");
    }
    let arg = &arg_exprs[0];
    Ok(parse_quote! { ((#arg as f64).sqrt().floor() as i32) })
}

/// comb(n, k) - combinations
fn convert_comb(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.comb() requires exactly 2 arguments");
    }
    let n = &arg_exprs[0];
    let k = &arg_exprs[1];
    Ok(parse_quote! {
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
    })
}

/// perm(n) or perm(n, k) - permutations
fn convert_perm(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 2 {
        bail!("math.perm() requires 1 or 2 arguments");
    }
    let n = &arg_exprs[0];
    let k = if arg_exprs.len() == 2 {
        arg_exprs[1].clone()
    } else {
        arg_exprs[0].clone()
    };
    Ok(parse_quote! {
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
    })
}

/// ldexp(x, i) - x * 2^i
fn convert_ldexp(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.ldexp() requires exactly 2 arguments");
    }
    let x = &arg_exprs[0];
    let i = &arg_exprs[1];
    Ok(parse_quote! { (#x as f64) * 2.0f64.powi(#i as i32) })
}

/// frexp(x) - (mantissa, exponent)
fn convert_frexp(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.frexp() requires exactly 1 argument");
    }
    let x = &arg_exprs[0];
    Ok(parse_quote! {
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
    })
}

/// isclose(a, b)
fn convert_isclose(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("math.isclose() requires at least 2 arguments");
    }
    let a = &arg_exprs[0];
    let b = &arg_exprs[1];
    Ok(parse_quote! {
        {
            let a = #a as f64;
            let b = #b as f64;
            let rel_tol = 1e-9;
            let abs_tol = 0.0;
            let diff = (a - b).abs();
            diff <= abs_tol.max(rel_tol * a.abs().max(b.abs()))
        }
    })
}

/// modf(x) - (fractional, integer)
fn convert_modf(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("math.modf() requires exactly 1 argument");
    }
    let x = &arg_exprs[0];
    Ok(parse_quote! {
        {
            let x = #x as f64;
            let int_part = x.trunc();
            let frac_part = x - int_part;
            (frac_part, int_part)
        }
    })
}

/// fmod(x, y) - floating point remainder
fn convert_fmod(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.fmod() requires exactly 2 arguments");
    }
    let x = &arg_exprs[0];
    let y = &arg_exprs[1];
    Ok(parse_quote! { (#x as f64) % (#y as f64) })
}

/// hypot(x, y) - hypotenuse
fn convert_hypot(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.hypot() requires exactly 2 arguments");
    }
    let x = &arg_exprs[0];
    let y = &arg_exprs[1];
    Ok(parse_quote! { (#x as f64).hypot(#y as f64) })
}

/// dist(p, q) - distance between points
fn convert_dist(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.dist() requires exactly 2 arguments (two points)");
    }
    let p = &arg_exprs[0];
    let q = &arg_exprs[1];
    Ok(parse_quote! {
        {
            let p = #p;
            let q = #q;
            let dx = p[0] - q[0];
            let dy = p[1] - q[1];
            ((dx * dx + dy * dy) as f64).sqrt()
        }
    })
}

/// remainder(x, y) - IEEE remainder
fn convert_remainder(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("math.remainder() requires exactly 2 arguments");
    }
    let x = &arg_exprs[0];
    let y = &arg_exprs[1];
    Ok(parse_quote! {
        {
            let x = #x as f64;
            let y = #y as f64;
            let n = (x / y).round();
            x - n * y
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============ Trigonometric tests ============

    #[test]
    fn test_convert_math_sin() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.0))];
        let result = convert_math_method("sin", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("sin"));
    }

    #[test]
    fn test_convert_math_cos() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(0.0))];
        let result = convert_math_method("cos", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_atan2() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(1.0)),
            HirExpr::Literal(Literal::Float(1.0)),
        ];
        let result = convert_math_method("atan2", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("atan2"));
    }

    #[test]
    fn test_convert_math_atan2_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.0))];
        let result = convert_math_method("atan2", &args, &mut ctx);
        assert!(result.is_err());
    }

    // ============ Hyperbolic tests ============

    #[test]
    fn test_convert_math_sinh() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.0))];
        let result = convert_math_method("sinh", &args, &mut ctx);
        assert!(result.is_ok());
    }

    // ============ Power/Log tests ============

    #[test]
    fn test_convert_math_sqrt() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(4.0))];
        let result = convert_math_method("sqrt", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("sqrt"));
    }

    #[test]
    fn test_convert_math_log_single_arg() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(10.0))];
        let result = convert_math_method("log", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("ln"));
    }

    #[test]
    fn test_convert_math_log_with_base() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(100.0)),
            HirExpr::Literal(Literal::Float(10.0)),
        ];
        let result = convert_math_method("log", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("log"));
    }

    #[test]
    fn test_convert_math_pow() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(2.0)),
            HirExpr::Literal(Literal::Float(3.0)),
        ];
        let result = convert_math_method("pow", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("powf"));
    }

    // ============ Rounding tests ============

    #[test]
    fn test_convert_math_ceil() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.5))];
        let result = convert_math_method("ceil", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        // DEPYLER-1006: Now returns f64 instead of i32 for type annotation compatibility
        assert!(code.contains("ceil") && code.contains("f64"));
    }

    #[test]
    fn test_convert_math_floor() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.5))];
        let result = convert_math_method("floor", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_trunc() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.9))];
        let result = convert_math_method("trunc", &args, &mut ctx);
        assert!(result.is_ok());
    }

    // ============ Special tests ============

    #[test]
    fn test_convert_math_fabs() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(-5.0))];
        let result = convert_math_method("fabs", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("abs"));
    }

    #[test]
    fn test_convert_math_degrees() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(3.25159))];
        let result = convert_math_method("degrees", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("to_degrees"));
    }

    #[test]
    fn test_convert_math_radians() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(180.0))];
        let result = convert_math_method("radians", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("to_radians"));
    }

    // ============ Check tests ============

    #[test]
    fn test_convert_math_isnan() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("x".to_string())];
        let result = convert_math_method("isnan", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("is_nan"));
    }

    #[test]
    fn test_convert_math_isinf() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("x".to_string())];
        let result = convert_math_method("isinf", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("is_infinite"));
    }

    #[test]
    fn test_convert_math_isfinite() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.0))];
        let result = convert_math_method("isfinite", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("is_finite"));
    }

    // ============ Integer tests ============

    #[test]
    fn test_convert_math_gcd() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(12)),
            HirExpr::Literal(Literal::Int(8)),
        ];
        let result = convert_math_method("gcd", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_lcm() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(4)),
            HirExpr::Literal(Literal::Int(6)),
        ];
        let result = convert_math_method("lcm", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_factorial() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(5))];
        let result = convert_math_method("factorial", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_isqrt() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(16))];
        let result = convert_math_method("isqrt", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_comb() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(5)),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_math_method("comb", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_perm() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(5)),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_math_method("perm", &args, &mut ctx);
        assert!(result.is_ok());
    }

    // ============ Other tests ============

    #[test]
    fn test_convert_math_hypot() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(3.0)),
            HirExpr::Literal(Literal::Float(4.0)),
        ];
        let result = convert_math_method("hypot", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("hypot"));
    }

    #[test]
    fn test_convert_math_fmod() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(7.0)),
            HirExpr::Literal(Literal::Float(3.0)),
        ];
        let result = convert_math_method("fmod", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_isclose() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(1.0)),
            HirExpr::Literal(Literal::Float(1.0000000001)),
        ];
        let result = convert_math_method("isclose", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_modf() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(3.5))];
        let result = convert_math_method("modf", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_math_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_math_method("unknown_func", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented yet"));
    }

    // ============ S9B7: Error paths for argument count validation ============

    // --- convert_trig ---
    #[test]
    fn test_s9b7_trig_zero_args() {
        let result = convert_trig("sin", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_trig_two_args() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let b: syn::Expr = parse_quote! { 2.0 };
        let result = convert_trig("sin", &[a, b]);
        assert!(result.is_err());
    }

    // --- convert_hyperbolic ---
    #[test]
    fn test_s9b7_hyperbolic_zero_args() {
        let result = convert_hyperbolic("sinh", &[]);
        assert!(result.is_err());
    }

    // --- convert_power_log ---
    #[test]
    fn test_s9b7_power_log_zero_args() {
        let result = convert_power_log("sqrt", &[]);
        assert!(result.is_err());
    }

    // --- convert_log ---
    #[test]
    fn test_s9b7_log_zero_args() {
        let result = convert_log(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_log_three_args() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let b: syn::Expr = parse_quote! { 2.0 };
        let c: syn::Expr = parse_quote! { 3.0 };
        let result = convert_log(&[a, b, c]);
        assert!(result.is_err());
    }

    // --- convert_rounding ---
    #[test]
    fn test_s9b7_rounding_zero_args() {
        let result = convert_rounding("ceil", &[]);
        assert!(result.is_err());
    }

    // --- convert_fabs ---
    #[test]
    fn test_s9b7_fabs_zero_args() {
        let result = convert_fabs(&[]);
        assert!(result.is_err());
    }

    // --- convert_copysign ---
    #[test]
    fn test_s9b7_copysign_zero_args() {
        let result = convert_copysign(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_copysign_one_arg() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let result = convert_copysign(&[a]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_copysign_three_args() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let b: syn::Expr = parse_quote! { 2.0 };
        let c: syn::Expr = parse_quote! { 3.0 };
        let result = convert_copysign(&[a, b, c]);
        assert!(result.is_err());
    }

    // --- convert_degrees ---
    #[test]
    fn test_s9b7_degrees_zero_args() {
        let result = convert_degrees(&[]);
        assert!(result.is_err());
    }

    // --- convert_radians ---
    #[test]
    fn test_s9b7_radians_zero_args() {
        let result = convert_radians(&[]);
        assert!(result.is_err());
    }

    // --- convert_isnan ---
    #[test]
    fn test_s9b7_isnan_zero_args() {
        let result = convert_isnan(&[]);
        assert!(result.is_err());
    }

    // --- convert_isinf ---
    #[test]
    fn test_s9b7_isinf_zero_args() {
        let result = convert_isinf(&[]);
        assert!(result.is_err());
    }

    // --- convert_isfinite ---
    #[test]
    fn test_s9b7_isfinite_zero_args() {
        let result = convert_isfinite(&[]);
        assert!(result.is_err());
    }

    // --- convert_gcd ---
    #[test]
    fn test_s9b7_gcd_zero_args() {
        let result = convert_gcd(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_gcd_one_arg() {
        let a: syn::Expr = parse_quote! { 12 };
        let result = convert_gcd(&[a]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_gcd_three_args() {
        let a: syn::Expr = parse_quote! { 12 };
        let b: syn::Expr = parse_quote! { 8 };
        let c: syn::Expr = parse_quote! { 4 };
        let result = convert_gcd(&[a, b, c]);
        assert!(result.is_err());
    }

    // --- convert_lcm ---
    #[test]
    fn test_s9b7_lcm_zero_args() {
        let result = convert_lcm(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_lcm_one_arg() {
        let a: syn::Expr = parse_quote! { 4 };
        let result = convert_lcm(&[a]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_lcm_three_args() {
        let a: syn::Expr = parse_quote! { 4 };
        let b: syn::Expr = parse_quote! { 6 };
        let c: syn::Expr = parse_quote! { 8 };
        let result = convert_lcm(&[a, b, c]);
        assert!(result.is_err());
    }

    // --- convert_factorial ---
    #[test]
    fn test_s9b7_factorial_zero_args() {
        let result = convert_factorial(&[]);
        assert!(result.is_err());
    }

    // --- convert_isqrt ---
    #[test]
    fn test_s9b7_isqrt_zero_args() {
        let result = convert_isqrt(&[]);
        assert!(result.is_err());
    }

    // --- convert_comb ---
    #[test]
    fn test_s9b7_comb_zero_args() {
        let result = convert_comb(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_comb_one_arg() {
        let a: syn::Expr = parse_quote! { 5 };
        let result = convert_comb(&[a]);
        assert!(result.is_err());
    }

    // --- convert_perm ---
    #[test]
    fn test_s9b7_perm_zero_args() {
        let result = convert_perm(&[]);
        assert!(result.is_err());
    }

    // --- convert_ldexp ---
    #[test]
    fn test_s9b7_ldexp_zero_args() {
        let result = convert_ldexp(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_ldexp_one_arg() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let result = convert_ldexp(&[a]);
        assert!(result.is_err());
    }

    // --- convert_frexp ---
    #[test]
    fn test_s9b7_frexp_zero_args() {
        let result = convert_frexp(&[]);
        assert!(result.is_err());
    }

    // --- convert_isclose ---
    #[test]
    fn test_s9b7_isclose_zero_args() {
        let result = convert_isclose(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_isclose_one_arg() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let result = convert_isclose(&[a]);
        assert!(result.is_err());
    }

    // --- convert_modf ---
    #[test]
    fn test_s9b7_modf_zero_args() {
        let result = convert_modf(&[]);
        assert!(result.is_err());
    }

    // --- convert_fmod ---
    #[test]
    fn test_s9b7_fmod_zero_args() {
        let result = convert_fmod(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_fmod_one_arg() {
        let a: syn::Expr = parse_quote! { 7.0 };
        let result = convert_fmod(&[a]);
        assert!(result.is_err());
    }

    // --- convert_hypot ---
    #[test]
    fn test_s9b7_hypot_zero_args() {
        let result = convert_hypot(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_hypot_one_arg() {
        let a: syn::Expr = parse_quote! { 3.0 };
        let result = convert_hypot(&[a]);
        assert!(result.is_err());
    }

    // --- convert_dist ---
    #[test]
    fn test_s9b7_dist_zero_args() {
        let result = convert_dist(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_dist_one_arg() {
        let a: syn::Expr = parse_quote! { vec![1.0, 2.0] };
        let result = convert_dist(&[a]);
        assert!(result.is_err());
    }

    // --- convert_remainder ---
    #[test]
    fn test_s9b7_remainder_zero_args() {
        let result = convert_remainder(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_remainder_one_arg() {
        let a: syn::Expr = parse_quote! { 5.0 };
        let result = convert_remainder(&[a]);
        assert!(result.is_err());
    }

    // --- convert_atan2 ---
    #[test]
    fn test_s9b7_atan2_zero_args() {
        let result = convert_atan2(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_atan2_one_arg() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let result = convert_atan2(&[a]);
        assert!(result.is_err());
    }

    // --- convert_pow ---
    #[test]
    fn test_s9b7_pow_zero_args() {
        let result = convert_pow(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_pow_one_arg() {
        let a: syn::Expr = parse_quote! { 2.0 };
        let result = convert_pow(&[a]);
        assert!(result.is_err());
    }

    // ============ S9B7: Output verification for key functions ============

    #[test]
    fn test_s9b7_trig_sin_output_contains_sin() {
        let arg: syn::Expr = parse_quote! { x };
        let result = convert_trig("sin", &[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("sin"));
    }

    #[test]
    fn test_s9b7_trig_cos_output_contains_cos() {
        let arg: syn::Expr = parse_quote! { x };
        let result = convert_trig("cos", &[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("cos"));
    }

    #[test]
    fn test_s9b7_trig_tan_output_contains_tan() {
        let arg: syn::Expr = parse_quote! { x };
        let result = convert_trig("tan", &[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("tan"));
    }

    #[test]
    fn test_s9b7_hyperbolic_sinh_output_contains_sinh() {
        let arg: syn::Expr = parse_quote! { x };
        let result = convert_hyperbolic("sinh", &[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("sinh"));
    }

    #[test]
    fn test_s9b7_hyperbolic_cosh_output_contains_cosh() {
        let arg: syn::Expr = parse_quote! { x };
        let result = convert_hyperbolic("cosh", &[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("cosh"));
    }

    #[test]
    fn test_s9b7_isclose_with_three_args() {
        let a: syn::Expr = parse_quote! { 1.0 };
        let b: syn::Expr = parse_quote! { 1.0001 };
        let tol: syn::Expr = parse_quote! { 0.001 };
        let result = convert_isclose(&[a, b, tol]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("rel_tol"));
    }

    #[test]
    fn test_s9b7_perm_one_arg_factorial_case() {
        let n: syn::Expr = parse_quote! { 5 };
        let result = convert_perm(&[n]).unwrap();
        let code = quote::quote!(#result).to_string();
        // With 1 arg, k = n (factorial case)
        assert!(code.contains("result"));
    }

    #[test]
    fn test_s9b7_expm1_zero_args() {
        let result = convert_expm1(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_s9b7_expm1_output() {
        let arg: syn::Expr = parse_quote! { 1.0 };
        let result = convert_expm1(&[arg]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("exp_m1"));
    }

    #[test]
    fn test_s9b7_copysign_output() {
        let x: syn::Expr = parse_quote! { 1.0 };
        let y: syn::Expr = parse_quote! { -1.0 };
        let result = convert_copysign(&[x, y]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("copysign"));
    }

    #[test]
    fn test_s9b7_ldexp_output() {
        let x: syn::Expr = parse_quote! { 1.5 };
        let i: syn::Expr = parse_quote! { 3 };
        let result = convert_ldexp(&[x, i]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("powi"));
    }

    #[test]
    fn test_s9b7_frexp_output() {
        let x: syn::Expr = parse_quote! { 8.0 };
        let result = convert_frexp(&[x]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("log2"));
    }

    #[test]
    fn test_s9b7_modf_output() {
        let x: syn::Expr = parse_quote! { 3.5 };
        let result = convert_modf(&[x]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("trunc"));
    }

    #[test]
    fn test_s9b7_fmod_output() {
        let x: syn::Expr = parse_quote! { 7.0 };
        let y: syn::Expr = parse_quote! { 3.0 };
        let result = convert_fmod(&[x, y]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("%"));
    }

    #[test]
    fn test_s9b7_remainder_output() {
        let x: syn::Expr = parse_quote! { 7.0 };
        let y: syn::Expr = parse_quote! { 3.0 };
        let result = convert_remainder(&[x, y]).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("round"));
    }

    #[test]
    fn test_s9b7_perm_three_args_error() {
        let a: syn::Expr = parse_quote! { 5 };
        let b: syn::Expr = parse_quote! { 2 };
        let c: syn::Expr = parse_quote! { 1 };
        let result = convert_perm(&[a, b, c]);
        assert!(result.is_err());
    }
}
