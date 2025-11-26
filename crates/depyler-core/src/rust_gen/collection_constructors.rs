//! Collection constructor code generation
//!
//! This module handles Python collection constructors: set(), frozenset(), dict(),
//! list(), deque(), Counter()
//!
//! Extracted from expr_gen.rs as part of DEPYLER-REFACTOR-001 (God File split)
//!
//! # DEPYLER-REFACTOR-001 Traceability
//! - Original location: expr_gen.rs lines 1816-1951, 2330-2381
//! - Extraction date: 2025-11-25
//! - Tests: tests/refactor_collection_constructors_test.rs

use crate::rust_gen::context::CodeGenContext;
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python set() constructor to Rust HashSet
///
/// - `set()` → `HashSet::<i32>::new()` (DEPYLER-0409: default type for inference)
/// - `set(iterable)` → `iterable.into_iter().collect::<HashSet<_>>()`
///
/// # Complexity: 4
pub fn convert_set_constructor(ctx: &mut CodeGenContext, args: &[syn::Expr]) -> Result<syn::Expr> {
    ctx.needs_hashset = true;
    if args.is_empty() {
        // Empty set: set()
        // DEPYLER-0409: Use default type i32 to avoid "type annotations needed" error
        Ok(parse_quote! { HashSet::<i32>::new() })
    } else if args.len() == 1 {
        // Set from iterable: set([1, 2, 3])
        let arg = &args[0];
        Ok(parse_quote! {
            #arg.into_iter().collect::<HashSet<_>>()
        })
    } else {
        bail!("set() takes at most 1 argument ({} given)", args.len())
    }
}

/// Convert Python frozenset() constructor to Rust Arc<HashSet>
///
/// - `frozenset()` → `Arc::new(HashSet::<i32>::new())`
/// - `frozenset(iterable)` → `Arc::new(iterable.into_iter().collect::<HashSet<_>>())`
///
/// # Complexity: 4
pub fn convert_frozenset_constructor(
    ctx: &mut CodeGenContext,
    args: &[syn::Expr],
) -> Result<syn::Expr> {
    ctx.needs_hashset = true;
    if args.is_empty() {
        // Empty frozenset: frozenset()
        // DEPYLER-0409: Use default type i32 for empty sets
        Ok(parse_quote! { std::sync::Arc::new(HashSet::<i32>::new()) })
    } else if args.len() == 1 {
        // Frozenset from iterable: frozenset([1, 2, 3])
        let arg = &args[0];
        Ok(parse_quote! {
            std::sync::Arc::new(#arg.into_iter().collect::<HashSet<_>>())
        })
    } else {
        bail!(
            "frozenset() takes at most 1 argument ({} given)",
            args.len()
        )
    }
}

/// Convert Python Counter() to Rust HashMap with fold counting
///
/// DEPYLER-0171: Counter(iterable) counts elements
///
/// - `Counter()` → `HashMap::new()`
/// - `Counter(iterable)` → fold with entry().or_insert()
///
/// # Complexity: 4
pub fn convert_counter_builtin(ctx: &mut CodeGenContext, args: &[syn::Expr]) -> Result<syn::Expr> {
    ctx.needs_hashmap = true;
    if args.is_empty() {
        Ok(parse_quote! { HashMap::new() })
    } else if args.len() == 1 {
        let arg = &args[0];
        Ok(parse_quote! {
            #arg.into_iter().fold(HashMap::new(), |mut acc, item| {
                *acc.entry(item).or_insert(0) += 1;
                acc
            })
        })
    } else {
        bail!("Counter() takes at most 1 argument ({} given)", args.len())
    }
}

/// Convert Python defaultdict() to Rust HashMap
///
/// DEPYLER-0556: defaultdict(factory) creates HashMap with default values
///
/// - `defaultdict(int)` → `HashMap::new()` (use entry API for default 0)
/// - `defaultdict(list)` → `HashMap::new()` (use entry API for default vec)
/// - `defaultdict()` → `HashMap::new()`
///
/// Note: Python's defaultdict auto-creates missing values. In Rust, we use
/// the entry API: `map.entry(key).or_insert_with(factory)` or `.or_default()`
///
/// # Complexity: 3
pub fn convert_defaultdict_builtin(
    ctx: &mut CodeGenContext,
    _args: &[syn::Expr],
) -> Result<syn::Expr> {
    ctx.needs_hashmap = true;
    // defaultdict(int), defaultdict(list), defaultdict(str), defaultdict()
    // All translate to HashMap::new() since Rust uses entry API for defaults
    Ok(parse_quote! { HashMap::new() })
}

/// Convert Python dict() constructor to Rust HashMap
///
/// DEPYLER-0172: dict() converts mapping/iterable to HashMap
///
/// - `dict()` → `HashMap::new()`
/// - `dict(mapping)` → `mapping.into_iter().collect::<HashMap<_, _>>()`
///
/// # Complexity: 4
pub fn convert_dict_builtin(ctx: &mut CodeGenContext, args: &[syn::Expr]) -> Result<syn::Expr> {
    ctx.needs_hashmap = true;
    if args.is_empty() {
        Ok(parse_quote! { HashMap::new() })
    } else if args.len() == 1 {
        let arg = &args[0];
        Ok(parse_quote! {
            #arg.into_iter().collect::<HashMap<_, _>>()
        })
    } else {
        bail!("dict() takes at most 1 argument ({} given)", args.len())
    }
}

/// Convert Python deque() to Rust VecDeque
///
/// DEPYLER-0173: deque(iterable) creates VecDeque from iterable
///
/// - `deque()` → `VecDeque::new()`
/// - `deque(iterable)` → `VecDeque::from(iterable)`
///
/// # Complexity: 4
pub fn convert_deque_builtin(ctx: &mut CodeGenContext, args: &[syn::Expr]) -> Result<syn::Expr> {
    ctx.needs_vecdeque = true;
    if args.is_empty() {
        Ok(parse_quote! { VecDeque::new() })
    } else if args.len() == 1 {
        let arg = &args[0];
        Ok(parse_quote! {
            VecDeque::from(#arg)
        })
    } else {
        bail!("deque() takes at most 1 argument ({} given)", args.len())
    }
}

/// Check if expression already ends with .collect()
///
/// # Complexity: 2
pub fn already_collected(expr: &syn::Expr) -> bool {
    if let syn::Expr::MethodCall(method_call) = expr {
        method_call.method == "collect"
    } else {
        false
    }
}

/// Check if expression is a range (0..5, start..end, etc.)
///
/// # Complexity: 1
pub fn is_range_expr(expr: &syn::Expr) -> bool {
    matches!(expr, syn::Expr::Range(_))
}

/// Check if expression is an iterator-producing expression
///
/// # Complexity: 4
pub fn is_iterator_expr(expr: &syn::Expr) -> bool {
    if let syn::Expr::MethodCall(method_call) = expr {
        let method_name = method_call.method.to_string();
        matches!(
            method_name.as_str(),
            "iter"
                | "iter_mut"
                | "into_iter"
                | "zip"
                | "map"
                | "filter"
                | "enumerate"
                | "chain"
                | "flat_map"
                | "take"
                | "skip"
                | "collect"
        )
    } else {
        false
    }
}

/// Check if expression is a CSV reader variable
///
/// DEPYLER-0452: Uses heuristic name-based detection
///
/// # Complexity: 4
pub fn is_csv_reader_var(expr: &syn::Expr) -> bool {
    if let syn::Expr::Path(path) = expr {
        if let Some(ident) = path.path.get_ident() {
            let var_name = ident.to_string();
            return var_name == "reader"
                || var_name.contains("csv")
                || var_name.ends_with("_reader")
                || var_name.starts_with("reader_");
        }
    }
    false
}

/// Convert Python list() to Rust Vec with smart handling
///
/// DEPYLER-0174: list(iterable) converts iterable to Vec
///
/// Handles special cases:
/// - Empty: `list()` → `Vec::new()`
/// - Already collected: return as-is
/// - Range: `list(range(5))` → `(0..5).collect::<Vec<_>>()`
/// - Iterator: `list(iter)` → `iter.collect::<Vec<_>>()`
/// - CSV reader: special handling for DictReader
/// - Default: `list(x)` → `x.into_iter().collect::<Vec<_>>()`
///
/// # Complexity: 9 (within limit)
pub fn convert_list_builtin(ctx: &mut CodeGenContext, args: &[syn::Expr]) -> Result<syn::Expr> {
    if args.is_empty() {
        return Ok(parse_quote! { Vec::new() });
    }

    if args.len() != 1 {
        bail!("list() takes at most 1 argument ({} given)", args.len());
    }

    let arg = &args[0];

    // DEPYLER-0177: Check if expression already collected
    if already_collected(arg) {
        return Ok(arg.clone());
    }

    // DEPYLER-0179: range(5) → (0..5).collect()
    if is_range_expr(arg) {
        return Ok(parse_quote! {
            (#arg).collect::<Vec<_>>()
        });
    }

    // DEPYLER-0176: zip(), enumerate() return iterators
    if is_iterator_expr(arg) {
        return Ok(parse_quote! {
            #arg.collect::<Vec<_>>()
        });
    }

    // DEPYLER-0452: CSV DictReader → use deserialize()
    if is_csv_reader_var(arg) {
        ctx.needs_csv = true;
        return Ok(parse_quote! {
            #arg.deserialize::<HashMap<String, String>>().collect::<Vec<_>>()
        });
    }

    // Regular iterable → collect to Vec
    Ok(parse_quote! {
        #arg.into_iter().collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_collected_true() {
        let expr: syn::Expr = parse_quote! { items.collect::<Vec<_>>() };
        assert!(already_collected(&expr));
    }

    #[test]
    fn test_already_collected_false() {
        let expr: syn::Expr = parse_quote! { items.iter() };
        assert!(!already_collected(&expr));
    }

    #[test]
    fn test_is_range_expr_true() {
        let expr: syn::Expr = parse_quote! { 0..5 };
        assert!(is_range_expr(&expr));
    }

    #[test]
    fn test_is_range_expr_false() {
        let expr: syn::Expr = parse_quote! { vec![1, 2, 3] };
        assert!(!is_range_expr(&expr));
    }

    #[test]
    fn test_is_iterator_expr_zip() {
        let expr: syn::Expr = parse_quote! { a.iter().zip(b.iter()) };
        assert!(is_iterator_expr(&expr));
    }

    #[test]
    fn test_is_iterator_expr_map() {
        let expr: syn::Expr = parse_quote! { items.map(|x| x + 1) };
        assert!(is_iterator_expr(&expr));
    }

    #[test]
    fn test_is_csv_reader_var_true() {
        let expr: syn::Expr = parse_quote! { reader };
        assert!(is_csv_reader_var(&expr));

        let expr2: syn::Expr = parse_quote! { csv_reader };
        assert!(is_csv_reader_var(&expr2));
    }

    #[test]
    fn test_is_csv_reader_var_false() {
        let expr: syn::Expr = parse_quote! { items };
        assert!(!is_csv_reader_var(&expr));
    }
}
