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
/// DEPYLER-1062: Updated to use depyler_min helper for safe f64/NaN handling
pub fn convert_min_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() {
        bail!("min() requires at least 1 argument");
    }
    if args.len() == 1 {
        // Single iterable: use .min() which now works with DepylerValue (has Ord)
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().min().expect("builtin operation failed") })
    } else {
        // Multiple args: use depyler_min helper for safe f64 comparison
        // depyler_min(a, depyler_min(b, c)) for chained comparisons
        let first = &args[0];
        let mut min_expr: syn::Expr = parse_quote! { #first.clone() };
        for arg in &args[1..] {
            min_expr = parse_quote! { depyler_min(#min_expr, #arg.clone()) };
        }
        Ok(min_expr)
    }
}

/// max(iterable) or max(a, b, c, ...)
/// DEPYLER-1062: Updated to use depyler_max helper for safe f64/NaN handling
pub fn convert_max_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() {
        bail!("max() requires at least 1 argument");
    }
    if args.len() == 1 {
        // Single iterable: use .max() which now works with DepylerValue (has Ord)
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.into_iter().max().expect("builtin operation failed") })
    } else {
        // Multiple args: use depyler_max helper for safe f64 comparison
        // depyler_max(a, depyler_max(b, c)) for chained comparisons
        let first = &args[0];
        let mut max_expr: syn::Expr = parse_quote! { #first.clone() };
        for arg in &args[1..] {
            max_expr = parse_quote! { depyler_max(#max_expr, #arg.clone()) };
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
/// DEPYLER-1045: Wrap code in parentheses to handle arithmetic expressions correctly
/// e.g., chr(base + shifted) → char::from_u32((base + shifted) as u32)
pub fn convert_chr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("chr() requires exactly 1 argument");
    }
    let code = &args[0];
    // Wrap in parens to ensure cast applies to entire expression, not just last operand
    Ok(parse_quote! {
        char::from_u32((#code) as u32).expect("builtin operation failed").to_string()
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
/// DEPYLER-1078: Handle next(iter, None) correctly - just return .next()
pub fn convert_next_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 2 {
        bail!("next() requires 1 or 2 arguments (iterator, optional default)");
    }
    let iterator = &args[0];
    if args.len() == 2 {
        let default = &args[1];
        // DEPYLER-1078: If default is None, just return .next() since it already returns Option<T>
        let is_none = match default {
            syn::Expr::Path(path) => path.path.is_ident("None"),
            _ => false,
        };
        if is_none {
            Ok(parse_quote! { #iterator.next() })
        } else {
            Ok(parse_quote! {
                #iterator.next().unwrap_or(#default)
            })
        }
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

// ============================================
// DEPYLER-1205: E0425 Vocabulary Expansion
// Add input(), hasattr(), super() builtins
// ============================================

/// input() or input(prompt) → Read line from stdin
/// Python: input() reads stdin, input("prompt: ") prints prompt first
/// Rust: Uses std::io::stdin().read_line() with prompt support
pub fn convert_input_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() > 1 {
        bail!("input() requires 0 or 1 arguments");
    }
    if args.len() == 1 {
        let prompt = &args[0];
        // With prompt: print then read
        Ok(parse_quote! {
            {
                use std::io::Write;
                print!("{}", #prompt);
                std::io::stdout().flush().expect("builtin operation failed");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("builtin operation failed");
                input.trim_end().to_string()
            }
        })
    } else {
        // No prompt: just read
        Ok(parse_quote! {
            {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("builtin operation failed");
                input.trim_end().to_string()
            }
        })
    }
}

/// hasattr(obj, name) → Check if object has attribute
/// Python: hasattr(obj, "attr") returns bool
/// Rust: Uses struct field access check pattern with a helper macro
/// Note: In Rust, this is typically a compile-time check. We generate
/// a pattern that works with common struct patterns.
pub fn convert_hasattr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 2 {
        bail!("hasattr() requires exactly 2 arguments (object, name)");
    }
    // For now, return true as a placeholder - proper implementation
    // would require trait-based introspection which Rust doesn't directly support
    // In practice, transpiled code should use Option fields or trait bounds
    Ok(parse_quote! { true })
}

/// getattr(obj, name) or getattr(obj, name, default)
/// Python: getattr(obj, "attr") or getattr(obj, "attr", default)
/// Rust: Direct field access or default value
pub fn convert_getattr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() < 2 || args.len() > 3 {
        bail!("getattr() requires 2 or 3 arguments");
    }
    let obj = &args[0];
    let _name = &args[1];
    if args.len() == 3 {
        let default = &args[2];
        // With default: return field or default
        Ok(parse_quote! { #obj.clone().unwrap_or(#default) })
    } else {
        // Without default: just return obj (assumes direct access)
        Ok(parse_quote! { #obj.clone() })
    }
}

/// setattr(obj, name, value) → Set attribute on object
/// Python: setattr(obj, "attr", value)
/// Rust: Direct field assignment (requires &mut)
pub fn convert_setattr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 3 {
        bail!("setattr() requires exactly 3 arguments (object, name, value)");
    }
    let _obj = &args[0];
    let _name = &args[1];
    let _value = &args[2];
    // Placeholder - in practice this needs context-aware codegen
    Ok(parse_quote! { () })
}

// ============================================
// GH-204: Additional E0425 Vocabulary Expansion
// Add more common Python builtins to reduce
// "cannot find value" errors in transpiled code
// ============================================

/// callable(obj) → Check if object is callable
/// Python: callable(obj) returns True if obj has __call__
/// Rust: Returns true as placeholder (type system handles this at compile time)
pub fn convert_callable_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("callable() requires exactly 1 argument");
    }
    // In Rust, callable check is a compile-time guarantee via Fn traits
    Ok(parse_quote! { true })
}

/// id(obj) → Return identity (memory address) of object
/// Python: id(obj) returns unique integer identifier
/// Rust: Uses raw pointer address cast to usize
pub fn convert_id_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("id() requires exactly 1 argument");
    }
    let obj = &args[0];
    Ok(parse_quote! { (&#obj as *const _ as usize) })
}

/// ascii(obj) → Return ASCII representation with escapes
/// Python: ascii(obj) like repr() but escapes non-ASCII
/// Rust: Uses Debug formatting with escape sequences
pub fn convert_ascii_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("ascii() requires exactly 1 argument");
    }
    let value = &args[0];
    Ok(parse_quote! { format!("{:?}", #value).escape_default().to_string() })
}

/// format(value, format_spec) → Format a value
/// Python: format(value, ".2f") formats with spec
/// Rust: Uses format! macro with appropriate spec
pub fn convert_format_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 2 {
        bail!("format() requires 1 or 2 arguments");
    }
    let value = &args[0];
    if args.len() == 2 {
        // With format spec - simplified handling
        Ok(parse_quote! { format!("{}", #value) })
    } else {
        Ok(parse_quote! { format!("{}", #value) })
    }
}

/// vars(obj) → Return __dict__ attribute
/// Python: vars(obj) returns object's attribute dict
/// Rust: Returns empty HashMap as placeholder
pub fn convert_vars_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() > 1 {
        bail!("vars() requires 0 or 1 arguments");
    }
    // Return empty HashMap as placeholder
    Ok(parse_quote! { std::collections::HashMap::<String, String>::new() })
}

/// dir(obj) → Return list of attributes
/// Python: dir(obj) returns list of attribute names
/// Rust: Returns empty Vec as placeholder
pub fn convert_dir_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() > 1 {
        bail!("dir() requires 0 or 1 arguments");
    }
    // Return empty Vec as placeholder
    Ok(parse_quote! { Vec::<String>::new() })
}

/// globals() → Return global namespace dict
/// Python: globals() returns global symbol table
/// Rust: Returns empty HashMap (no equivalent in Rust)
pub fn convert_globals_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if !args.is_empty() {
        bail!("globals() takes no arguments");
    }
    Ok(parse_quote! { std::collections::HashMap::<String, String>::new() })
}

/// locals() → Return local namespace dict
/// Python: locals() returns local symbol table
/// Rust: Returns empty HashMap (no equivalent in Rust)
pub fn convert_locals_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if !args.is_empty() {
        bail!("locals() takes no arguments");
    }
    Ok(parse_quote! { std::collections::HashMap::<String, String>::new() })
}

/// delattr(obj, name) → Delete attribute from object
/// Python: delattr(obj, "attr") removes attribute
/// Rust: No-op placeholder (struct fields can't be deleted)
pub fn convert_delattr_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 2 {
        bail!("delattr() requires exactly 2 arguments (object, name)");
    }
    // No-op in Rust - fields can't be dynamically deleted
    Ok(parse_quote! { () })
}

/// staticmethod(func) → Return static method
/// Python: @staticmethod decorator
/// Rust: No-op - Rust methods are static by default unless &self
pub fn convert_staticmethod_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("staticmethod() requires exactly 1 argument");
    }
    let func = &args[0];
    // Just return the function as-is
    Ok(parse_quote! { #func })
}

/// classmethod(func) → Return class method
/// Python: @classmethod decorator
/// Rust: No-op placeholder - Rust doesn't have class methods
pub fn convert_classmethod_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() != 1 {
        bail!("classmethod() requires exactly 1 argument");
    }
    let func = &args[0];
    // Just return the function as-is
    Ok(parse_quote! { #func })
}

/// property(fget, fset, fdel, doc) → Create property descriptor
/// Python: @property decorator for getters/setters
/// Rust: No-op placeholder - Rust uses direct field access
pub fn convert_property_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() || args.len() > 4 {
        bail!("property() requires 1-4 arguments");
    }
    let fget = &args[0];
    // Just return the getter function
    Ok(parse_quote! { #fget })
}

/// breakpoint() → Drop into debugger
/// Python: breakpoint() invokes debugger
/// Rust: Uses panic for debugging (or could use dbg!)
pub fn convert_breakpoint_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if !args.is_empty() {
        bail!("breakpoint() takes no arguments");
    }
    // Use panic as a breakpoint alternative
    Ok(parse_quote! { panic!("breakpoint reached") })
}

/// exit(code) → Exit program
/// Python: exit(0) or sys.exit(0)
/// Rust: std::process::exit(code)
pub fn convert_exit_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.len() > 1 {
        bail!("exit() requires 0 or 1 arguments");
    }
    if args.len() == 1 {
        let code = &args[0];
        Ok(parse_quote! { std::process::exit(#code as i32) })
    } else {
        Ok(parse_quote! { std::process::exit(0) })
    }
}

/// quit() → Alias for exit()
pub fn convert_quit_builtin(args: &[syn::Expr]) -> Result<syn::Expr> {
    convert_exit_builtin(args)
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
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("requires exactly 1 argument"));
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
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("requires exactly 2 arguments"));
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
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("requires at least 2 arguments"));
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
        let args: Vec<syn::Expr> = vec![
            parse_quote!(a),
            parse_quote!(b),
            parse_quote!(c),
            parse_quote!(d),
        ];
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

    // ============================================
    // DEPYLER-1205: input() tests
    // ============================================

    #[test]
    fn test_convert_input_builtin_no_prompt() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_input_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("stdin"), "Should use stdin: {}", code);
        assert!(code.contains("read_line"), "Should read line: {}", code);
    }

    #[test]
    fn test_convert_input_builtin_with_prompt() {
        let args: Vec<syn::Expr> = vec![parse_quote!("Enter: ")];
        let result = convert_input_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        // Token stream converts print! to "print !" with space
        assert!(code.contains("print"), "Should print prompt: {}", code);
        assert!(code.contains("flush"), "Should flush stdout: {}", code);
    }

    #[test]
    fn test_convert_input_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!("a"), parse_quote!("b")];
        let result = convert_input_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // DEPYLER-1205: hasattr() tests
    // ============================================

    #[test]
    fn test_convert_hasattr_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj), parse_quote!("attr")];
        let result = convert_hasattr_builtin(&args);
        assert!(result.is_ok());
        // Returns true as placeholder for now
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("true"), "Should return true: {}", code);
    }

    #[test]
    fn test_convert_hasattr_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj)];
        let result = convert_hasattr_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // DEPYLER-1205: getattr() tests
    // ============================================

    #[test]
    fn test_convert_getattr_builtin_two_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj), parse_quote!("attr")];
        let result = convert_getattr_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_getattr_builtin_with_default() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj), parse_quote!("attr"), parse_quote!(0)];
        let result = convert_getattr_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("unwrap_or"), "Should have default: {}", code);
    }

    #[test]
    fn test_convert_getattr_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj)];
        let result = convert_getattr_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // DEPYLER-1205: setattr() tests
    // ============================================

    #[test]
    fn test_convert_setattr_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj), parse_quote!("attr"), parse_quote!(42)];
        let result = convert_setattr_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_setattr_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj), parse_quote!("attr")];
        let result = convert_setattr_builtin(&args);
        assert!(result.is_err());
    }

    // ============================================
    // GH-204: Additional builtin tests
    // ============================================

    #[test]
    fn test_convert_callable_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(func)];
        let result = convert_callable_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_callable_builtin_wrong_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_callable_builtin(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_id_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(obj)];
        let result = convert_id_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("usize"), "Should cast to usize: {}", code);
    }

    #[test]
    fn test_convert_ascii_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(text)];
        let result = convert_ascii_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(
            code.contains("escape_default"),
            "Should use escape_default: {}",
            code
        );
    }

    #[test]
    fn test_convert_vars_builtin() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_vars_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_dir_builtin() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_dir_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_globals_builtin() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_globals_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_locals_builtin() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_locals_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_exit_builtin() {
        let args: Vec<syn::Expr> = vec![parse_quote!(0)];
        let result = convert_exit_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("exit"), "Should call exit: {}", code);
    }

    #[test]
    fn test_convert_exit_builtin_no_args() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_exit_builtin(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_breakpoint_builtin() {
        let args: Vec<syn::Expr> = vec![];
        let result = convert_breakpoint_builtin(&args);
        assert!(result.is_ok());
        let expr = result.unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("panic"), "Should use panic: {}", code);
    }
}
