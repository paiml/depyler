//! Python Builtin Functions Code Generation - EXTREME TDD
//!
//! Handles Python builtin function conversions to Rust.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use anyhow::{bail, Result};
use syn::parse_quote;

/// all(iterable) → iterable.into_iter().all(|x| x)
pub fn convert_all_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("all() requires exactly 1 argument");
    }
    let iterable = &args[0];
    Ok(parse_quote! { #iterable.into_iter().all(|x| x) })
}

/// any(iterable) → iterable.into_iter().any(|x| x)
pub fn convert_any_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("any() requires exactly 1 argument");
    }
    let iterable = &args[0];
    Ok(parse_quote! { #iterable.into_iter().any(|x| x) })
}

/// divmod(a, b) → (a / b, a % b)
pub fn convert_divmod_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 2 {
        bail!("divmod() requires exactly 2 arguments");
    }
    let a = &args[0];
    let b = &args[1];
    Ok(parse_quote! { (#a / #b, #a % #b) })
}

/// enumerate(iterable) or enumerate(iterable, start)
pub fn convert_enumerate_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 2 {
        bail!("enumerate() requires 1 or 2 arguments");
    }
    let iterable = &args[0];
    if args.len() == 2 {
        let start = &args[1];
        Ok(
            parse_quote! { #iterable.iter().cloned().enumerate().map(|(i, x)| ((i + #start as usize) as i32, x)) },
        )
    } else {
        Ok(parse_quote! { #iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x)) })
    }
}

/// zip(iter1, iter2, ...) → iter1.into_iter().zip(iter2.into_iter())
pub fn convert_zip_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() < 2 {
        bail!("zip() requires at least 2 arguments");
    }
    let first = &args[0];
    let second = &args[1];
    if args.len() == 2 {
        Ok(parse_quote! { #first.into_iter().zip(#second.into_iter()) })
    } else {
        let mut zip_expr: syn::Expr = parse_quote! { #first.into_iter().zip(#second.into_iter()) };
        for iter in &args[2..] {
            zip_expr = parse_quote! { #zip_expr.zip(#iter.into_iter()) };
        }
        Ok(zip_expr)
    }
}

/// reversed(iterable) → iterable.iter().cloned().rev()
pub fn convert_reversed_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("reversed() requires exactly 1 argument");
    }
    let iterable = &args[0];
    Ok(parse_quote! { #iterable.iter().cloned().rev() })
}

/// sorted(iterable) → sorted Vec with partial_cmp for float support
pub fn convert_sorted_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 2 {
        bail!("sorted() requires 1 or 2 arguments");
    }
    let iterable = &args[0];
    Ok(parse_quote! {
        {
            let mut sorted_vec = #iterable.iter().cloned().collect::<Vec<_>>();
            sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted_vec
        }
    })
}

/// sum(iterable) or sum(iterable, start)
pub fn convert_sum_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
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

/// round(value) → (value as f64).round() as i32
pub fn convert_round_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 2 {
        bail!("round() requires 1 or 2 arguments");
    }
    let value = &args[0];
    Ok(parse_quote! { (#value as f64).round() as i32 })
}

/// abs(value) → value.abs()
pub fn convert_abs_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("abs() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { (#value).abs() })
}

/// min(iterable) or min(a, b, c, ...)
pub fn convert_min_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() {
        bail!("min() requires at least 1 argument");
    }
    if args.len() == 1 {
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().min().unwrap() })
    } else {
        let first = &args[0];
        let mut min_expr = parse_quote! { #first };
        for arg in &args[1..] {
            min_expr = parse_quote! { #min_expr.min(#arg) };
        }
        Ok(min_expr)
    }
}

/// max(iterable) or max(a, b, c, ...)
pub fn convert_max_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() {
        bail!("max() requires at least 1 argument");
    }
    if args.len() == 1 {
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().max().unwrap() })
    } else {
        let first = &args[0];
        let mut max_expr = parse_quote! { #first };
        for arg in &args[1..] {
            max_expr = parse_quote! { #max_expr.max(#arg) };
        }
        Ok(max_expr)
    }
}

/// pow(base, exp) → (base as f64).powf(exp as f64) as i32
pub fn convert_pow_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() < 2 || args.len() > 3 {
        bail!("pow() requires 2 or 3 arguments");
    }
    let base = &args[0];
    let exp = &args[1];
    Ok(parse_quote! { (#base as f64).powf(#exp as f64) as i32 })
}

/// hex(value) → format!("0x{:x}", value)
pub fn convert_hex_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("hex() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { format!("0x{:x}", #value) })
}

/// bin(value) → format!("0b{:b}", value)
pub fn convert_bin_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("bin() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { format!("0b{:b}", #value) })
}

/// oct(value) → format!("0o{:o}", value)
pub fn convert_oct_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("oct() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { format!("0o{:o}", #value) })
}

/// chr(code) → char::from_u32(code).unwrap().to_string()
pub fn convert_chr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("chr() requires exactly 1 argument");
    }
    let code = &args[0];
    Ok(parse_quote! {
        char::from_u32(#code as u32).unwrap().to_string()
    })
}

/// hash(value) → DefaultHasher hash
pub fn convert_hash_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
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

/// repr(value) → format!("{:?}", value)
pub fn convert_repr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("repr() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { format!("{:?}", #value) })
}

/// next(iterator) or next(iterator, default)
pub fn convert_next_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
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

/// iter(iterable) → iterable.into_iter()
pub fn convert_iter_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("iter() requires exactly 1 argument");
    }
    let iterable = &args[0];
    Ok(parse_quote! { #iterable.into_iter() })
}

/// type(value) → std::any::type_name_of_val(&value)
pub fn convert_type_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("type() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { std::any::type_name_of_val(&#value) })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================
    // all() tests
    // ============================================

    #[test]
    fn test_convert_all_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_all_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("all"));
    }

    #[test]
    fn test_convert_all_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_all_builtin(&args);
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("requires exactly 1 argument"));
    }

    #[test]
    fn test_convert_all_builtin_too_many_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b)];
        let result = convert_all_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // any() tests
    // ============================================

    #[test]
    fn test_convert_any_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_any_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("any"));
    }

    #[test]
    fn test_convert_any_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_any_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // divmod() tests
    // ============================================

    #[test]
    fn test_convert_divmod_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(10), parse_quote!(3)];
        let result = convert_divmod_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_divmod_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(10)];
        let result = convert_divmod_builtin(&args);
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("requires exactly 2 arguments"));
    }

    // ============================================
    // enumerate() tests
    // ============================================

    #[test]
    fn test_convert_enumerate_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_enumerate_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("enumerate"));
    }

    #[test]
    fn test_convert_enumerate_builtin_with_start() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items), parse_quote!(1)];
        let result = convert_enumerate_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_enumerate_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_enumerate_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_enumerate_builtin_too_many_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b), parse_quote!(c)];
        let result = convert_enumerate_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // zip() tests
    // ============================================

    #[test]
    fn test_convert_zip_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b)];
        let result = convert_zip_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("zip"));
    }

    #[test]
    fn test_convert_zip_builtin_three_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b), parse_quote!(c)];
        let result = convert_zip_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_zip_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a)];
        let result = convert_zip_builtin(&args);
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("requires at least 2 arguments"));
    }

    // ============================================
    // reversed() tests
    // ============================================

    #[test]
    fn test_convert_reversed_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_reversed_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("rev"));
    }

    #[test]
    fn test_convert_reversed_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_reversed_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // sorted() tests
    // ============================================

    #[test]
    fn test_convert_sorted_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_sorted_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("sort_by"));
    }

    #[test]
    fn test_convert_sorted_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_sorted_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_sorted_builtin_too_many_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b), parse_quote!(c)];
        let result = convert_sorted_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // sum() tests
    // ============================================

    #[test]
    fn test_convert_sum_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_sum_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("sum"));
    }

    #[test]
    fn test_convert_sum_builtin_with_start() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items), parse_quote!(10)];
        let result = convert_sum_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("fold"));
    }

    #[test]
    fn test_convert_sum_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_sum_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // round() tests
    // ============================================

    #[test]
    fn test_convert_round_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(3.7)];
        let result = convert_round_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("round"));
    }

    #[test]
    fn test_convert_round_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_round_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // abs() tests
    // ============================================

    #[test]
    fn test_convert_abs_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_abs_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("abs"));
    }

    #[test]
    fn test_convert_abs_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_abs_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // min() tests
    // ============================================

    #[test]
    fn test_convert_min_builtin_iterable() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_min_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("min"));
    }

    #[test]
    fn test_convert_min_builtin_multiple_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b), parse_quote!(c)];
        let result = convert_min_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_min_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_min_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // max() tests
    // ============================================

    #[test]
    fn test_convert_max_builtin_iterable() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_max_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("max"));
    }

    #[test]
    fn test_convert_max_builtin_multiple_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b)];
        let result = convert_max_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_max_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_max_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // pow() tests
    // ============================================

    #[test]
    fn test_convert_pow_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(2), parse_quote!(3)];
        let result = convert_pow_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("powf"));
    }

    #[test]
    fn test_convert_pow_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(2)];
        let result = convert_pow_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_pow_builtin_too_many_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(a), parse_quote!(b), parse_quote!(c), parse_quote!(d)];
        let result = convert_pow_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // hex(), bin(), oct() tests
    // ============================================

    #[test]
    fn test_convert_hex_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(255)];
        let result = convert_hex_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("0x"));
    }

    #[test]
    fn test_convert_hex_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_hex_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_bin_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(10)];
        let result = convert_bin_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("0b"));
    }

    #[test]
    fn test_convert_bin_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_bin_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_oct_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(64)];
        let result = convert_oct_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("0o"));
    }

    #[test]
    fn test_convert_oct_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_oct_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // chr() tests
    // ============================================

    #[test]
    fn test_convert_chr_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(65)];
        let result = convert_chr_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("from_u32"));
    }

    #[test]
    fn test_convert_chr_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_chr_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // hash() tests
    // ============================================

    #[test]
    fn test_convert_hash_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(value)];
        let result = convert_hash_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("DefaultHasher"));
    }

    #[test]
    fn test_convert_hash_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_hash_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // repr() tests
    // ============================================

    #[test]
    fn test_convert_repr_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(value)];
        let result = convert_repr_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains(":?"));
    }

    #[test]
    fn test_convert_repr_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_repr_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // next() tests
    // ============================================

    #[test]
    fn test_convert_next_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(iter)];
        let result = convert_next_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("next"));
    }

    #[test]
    fn test_convert_next_builtin_with_default() {
        let args: Vec<syn::Expr> = vec![parse_quote!(iter), parse_quote!(0)];
        let result = convert_next_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("unwrap_or"));
    }

    #[test]
    fn test_convert_next_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_next_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // iter() tests
    // ============================================

    #[test]
    fn test_convert_iter_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(items)];
        let result = convert_iter_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("into_iter"));
    }

    #[test]
    fn test_convert_iter_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_iter_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // type() tests
    // ============================================

    #[test]
    fn test_convert_type_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(value)];
        let result = convert_type_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("type_name_of_val"));
    }

    #[test]
    fn test_convert_type_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_type_builtin(&args);
        assert!(result.is_err());
    }
}
