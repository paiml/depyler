//! Iterator utility functions for code generation
//!
//! This module provides helper functions for detecting and handling
//! iterators and iterator-producing expressions in Python-to-Rust transpilation.

use crate::hir::{HirExpr, Literal};

/// Check if an expression produces an iterator (not a collection)
///
/// Generator expressions and method chains ending in iterator adapters produce
/// iterators that should NOT have .iter().cloned() added when iterated over.
pub fn is_iterator_producing_expr(expr: &HirExpr) -> bool {
    match expr {
        // Generator expressions always produce iterators
        HirExpr::GeneratorExp { .. } => true,
        // Method chains ending in iterator adapters
        HirExpr::MethodCall { method, object, .. } => {
            // Check if this method produces an iterator
            let is_iterator_method = matches!(
                method.as_str(),
                "iter"
                    | "into_iter"
                    | "chars"
                    | "bytes"
                    | "lines"
                    | "split"
                    | "split_whitespace"
                    | "keys"
                    | "values"
                    | "items"
                    | "map"
                    | "filter"
                    | "filter_map"
                    | "flat_map"
                    | "flatten"
                    | "take"
                    | "skip"
                    | "take_while"
                    | "skip_while"
                    | "enumerate"
                    | "zip"
                    | "chain"
                    | "cycle"
                    | "rev"
                    | "peekable"
            );
            // Either this method produces an iterator, or the chain does
            is_iterator_method || is_iterator_producing_expr(object)
        }
        // Some builtin functions produce iterators
        HirExpr::Call { func, .. } => matches!(
            func.as_str(),
            "iter" | "range" | "enumerate" | "zip" | "map" | "filter" | "reversed"
        ),
        _ => false,
    }
}

/// Check if expression is a range call
pub fn is_range_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "range")
}

/// Check if expression is an enumerate call
pub fn is_enumerate_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "enumerate")
}

/// Check if expression is a zip call
pub fn is_zip_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "zip")
}

/// Check if expression is a map call
pub fn is_map_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "map")
}

/// Check if expression is a filter call
pub fn is_filter_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "filter")
}

/// Check if expression is a reversed call
pub fn is_reversed_call(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Call { func, .. } if func == "reversed")
}

/// Check if expression is a generator expression
pub fn is_generator_expr(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::GeneratorExp { .. })
}

/// Check if a method name is an iterator-producing method
pub fn is_iterator_method(method: &str) -> bool {
    matches!(
        method,
        "iter"
            | "into_iter"
            | "chars"
            | "bytes"
            | "lines"
            | "split"
            | "split_whitespace"
            | "keys"
            | "values"
            | "items"
            | "map"
            | "filter"
            | "filter_map"
            | "flat_map"
            | "flatten"
            | "take"
            | "skip"
            | "take_while"
            | "skip_while"
            | "enumerate"
            | "zip"
            | "chain"
            | "cycle"
            | "rev"
            | "peekable"
    )
}

/// Check if a method name is a collection-consuming method
pub fn is_collection_consuming_method(method: &str) -> bool {
    matches!(
        method,
        "collect" | "count" | "sum" | "product" | "max" | "min" | "last" | "nth" | "all" | "any"
            | "find" | "position" | "fold" | "reduce" | "for_each"
    )
}

/// Check if a method name is an iterator adapter (takes and returns iterator)
pub fn is_iterator_adapter(method: &str) -> bool {
    matches!(
        method,
        "map" | "filter" | "filter_map" | "flat_map" | "flatten" | "take" | "skip" | "take_while"
            | "skip_while" | "enumerate" | "zip" | "chain" | "cycle" | "rev" | "peekable"
            | "inspect" | "fuse" | "by_ref"
    )
}

/// Check if expression is a list/tuple/set literal
pub fn is_collection_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::List(_) | HirExpr::Tuple(_) | HirExpr::Set(_))
}

/// Check if expression is a dict literal
pub fn is_dict_literal(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Dict(_))
}

/// Check if expression is any kind of literal collection (list, tuple, set, dict)
pub fn is_any_collection_literal(expr: &HirExpr) -> bool {
    is_collection_literal(expr) || is_dict_literal(expr)
}

/// Check if expression produces a string
pub fn is_string_expr(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::String(_)))
        || matches!(expr, HirExpr::FString { .. })
        || matches!(expr, HirExpr::MethodCall { method, .. }
            if matches!(method.as_str(), "join" | "format" | "upper" | "lower" | "strip" | "replace"))
}

/// Check if expression produces bytes
pub fn is_bytes_expr(expr: &HirExpr) -> bool {
    matches!(expr, HirExpr::Literal(Literal::Bytes(_)))
}

/// Get the estimated length of a range call if all args are literals
pub fn get_range_length(expr: &HirExpr) -> Option<i64> {
    if let HirExpr::Call { func, args, .. } = expr {
        if func == "range" {
            match args.len() {
                1 => {
                    // range(stop) -> 0..stop
                    if let HirExpr::Literal(Literal::Int(stop)) = &args[0] {
                        return Some(*stop);
                    }
                }
                2 => {
                    // range(start, stop) -> start..stop
                    if let (
                        HirExpr::Literal(Literal::Int(start)),
                        HirExpr::Literal(Literal::Int(stop)),
                    ) = (&args[0], &args[1])
                    {
                        return Some(stop - start);
                    }
                }
                3 => {
                    // range(start, stop, step) -> (start..stop).step_by(step)
                    if let (
                        HirExpr::Literal(Literal::Int(start)),
                        HirExpr::Literal(Literal::Int(stop)),
                        HirExpr::Literal(Literal::Int(step)),
                    ) = (&args[0], &args[1], &args[2])
                    {
                        if *step != 0 {
                            return Some((stop - start).abs() / step.abs());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // is_iterator_producing_expr tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_generator_expr_is_iterator() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_iter_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_into_iter_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "into_iter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_chars_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "chars".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_bytes_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "bytes".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_lines_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "lines".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_split_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "split".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_keys_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "keys".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_values_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "values".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_items_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("dict".to_string())),
            method: "items".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_map_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_filter_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_enumerate_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_zip_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_chain_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "chain".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_rev_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "rev".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_take_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "take".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_skip_method_is_iterator() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("iter".to_string())),
            method: "skip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_chained_iterator_methods() {
        // list.iter().map(...)
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::MethodCall {
                object: Box::new(HirExpr::Var("list".to_string())),
                method: "iter".to_string(),
                args: vec![],
                kwargs: vec![],
            }),
            method: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_range_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_enumerate_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_zip_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_map_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_filter_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_reversed_call_is_iterator() {
        let expr = HirExpr::Call {
            func: "reversed".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_non_iterator_method() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("list".to_string())),
            method: "append".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_non_iterator_call() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_variable_is_not_iterator() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_iterator_producing_expr(&expr));
    }

    #[test]
    fn test_literal_is_not_iterator() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_iterator_producing_expr(&expr));
    }

    // ============================================================================
    // is_*_call tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_is_range_call_true() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_range_call(&expr));
    }

    #[test]
    fn test_is_range_call_false() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_range_call(&expr));
    }

    #[test]
    fn test_is_enumerate_call_true() {
        let expr = HirExpr::Call {
            func: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_enumerate_call(&expr));
    }

    #[test]
    fn test_is_enumerate_call_false() {
        let expr = HirExpr::Var("enumerate".to_string());
        assert!(!is_enumerate_call(&expr));
    }

    #[test]
    fn test_is_zip_call_true() {
        let expr = HirExpr::Call {
            func: "zip".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_zip_call(&expr));
    }

    #[test]
    fn test_is_zip_call_false() {
        let expr = HirExpr::Call {
            func: "enumerate".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(!is_zip_call(&expr));
    }

    #[test]
    fn test_is_map_call_true() {
        let expr = HirExpr::Call {
            func: "map".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_map_call(&expr));
    }

    #[test]
    fn test_is_filter_call_true() {
        let expr = HirExpr::Call {
            func: "filter".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_filter_call(&expr));
    }

    #[test]
    fn test_is_reversed_call_true() {
        let expr = HirExpr::Call {
            func: "reversed".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_reversed_call(&expr));
    }

    #[test]
    fn test_is_generator_expr_true() {
        let expr = HirExpr::GeneratorExp {
            element: Box::new(HirExpr::Var("x".to_string())),
            generators: vec![],
        };
        assert!(is_generator_expr(&expr));
    }

    #[test]
    fn test_is_generator_expr_false() {
        let expr = HirExpr::List(vec![]);
        assert!(!is_generator_expr(&expr));
    }

    // ============================================================================
    // is_iterator_method tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_is_iterator_method_iter() {
        assert!(is_iterator_method("iter"));
    }

    #[test]
    fn test_is_iterator_method_into_iter() {
        assert!(is_iterator_method("into_iter"));
    }

    #[test]
    fn test_is_iterator_method_chars() {
        assert!(is_iterator_method("chars"));
    }

    #[test]
    fn test_is_iterator_method_bytes() {
        assert!(is_iterator_method("bytes"));
    }

    #[test]
    fn test_is_iterator_method_lines() {
        assert!(is_iterator_method("lines"));
    }

    #[test]
    fn test_is_iterator_method_split() {
        assert!(is_iterator_method("split"));
    }

    #[test]
    fn test_is_iterator_method_split_whitespace() {
        assert!(is_iterator_method("split_whitespace"));
    }

    #[test]
    fn test_is_iterator_method_keys() {
        assert!(is_iterator_method("keys"));
    }

    #[test]
    fn test_is_iterator_method_values() {
        assert!(is_iterator_method("values"));
    }

    #[test]
    fn test_is_iterator_method_items() {
        assert!(is_iterator_method("items"));
    }

    #[test]
    fn test_is_iterator_method_map() {
        assert!(is_iterator_method("map"));
    }

    #[test]
    fn test_is_iterator_method_filter() {
        assert!(is_iterator_method("filter"));
    }

    #[test]
    fn test_is_iterator_method_filter_map() {
        assert!(is_iterator_method("filter_map"));
    }

    #[test]
    fn test_is_iterator_method_flat_map() {
        assert!(is_iterator_method("flat_map"));
    }

    #[test]
    fn test_is_iterator_method_flatten() {
        assert!(is_iterator_method("flatten"));
    }

    #[test]
    fn test_is_iterator_method_take() {
        assert!(is_iterator_method("take"));
    }

    #[test]
    fn test_is_iterator_method_skip() {
        assert!(is_iterator_method("skip"));
    }

    #[test]
    fn test_is_iterator_method_take_while() {
        assert!(is_iterator_method("take_while"));
    }

    #[test]
    fn test_is_iterator_method_skip_while() {
        assert!(is_iterator_method("skip_while"));
    }

    #[test]
    fn test_is_iterator_method_enumerate() {
        assert!(is_iterator_method("enumerate"));
    }

    #[test]
    fn test_is_iterator_method_zip() {
        assert!(is_iterator_method("zip"));
    }

    #[test]
    fn test_is_iterator_method_chain() {
        assert!(is_iterator_method("chain"));
    }

    #[test]
    fn test_is_iterator_method_cycle() {
        assert!(is_iterator_method("cycle"));
    }

    #[test]
    fn test_is_iterator_method_rev() {
        assert!(is_iterator_method("rev"));
    }

    #[test]
    fn test_is_iterator_method_peekable() {
        assert!(is_iterator_method("peekable"));
    }

    #[test]
    fn test_not_iterator_method() {
        assert!(!is_iterator_method("append"));
        assert!(!is_iterator_method("push"));
        assert!(!is_iterator_method("pop"));
        assert!(!is_iterator_method("collect"));
    }

    // ============================================================================
    // is_collection_consuming_method tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_consuming_collect() {
        assert!(is_collection_consuming_method("collect"));
    }

    #[test]
    fn test_consuming_count() {
        assert!(is_collection_consuming_method("count"));
    }

    #[test]
    fn test_consuming_sum() {
        assert!(is_collection_consuming_method("sum"));
    }

    #[test]
    fn test_consuming_product() {
        assert!(is_collection_consuming_method("product"));
    }

    #[test]
    fn test_consuming_max() {
        assert!(is_collection_consuming_method("max"));
    }

    #[test]
    fn test_consuming_min() {
        assert!(is_collection_consuming_method("min"));
    }

    #[test]
    fn test_consuming_last() {
        assert!(is_collection_consuming_method("last"));
    }

    #[test]
    fn test_consuming_nth() {
        assert!(is_collection_consuming_method("nth"));
    }

    #[test]
    fn test_consuming_all() {
        assert!(is_collection_consuming_method("all"));
    }

    #[test]
    fn test_consuming_any() {
        assert!(is_collection_consuming_method("any"));
    }

    #[test]
    fn test_consuming_find() {
        assert!(is_collection_consuming_method("find"));
    }

    #[test]
    fn test_consuming_position() {
        assert!(is_collection_consuming_method("position"));
    }

    #[test]
    fn test_consuming_fold() {
        assert!(is_collection_consuming_method("fold"));
    }

    #[test]
    fn test_consuming_reduce() {
        assert!(is_collection_consuming_method("reduce"));
    }

    #[test]
    fn test_consuming_for_each() {
        assert!(is_collection_consuming_method("for_each"));
    }

    #[test]
    fn test_not_consuming() {
        assert!(!is_collection_consuming_method("map"));
        assert!(!is_collection_consuming_method("filter"));
    }

    // ============================================================================
    // is_iterator_adapter tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_adapter_map() {
        assert!(is_iterator_adapter("map"));
    }

    #[test]
    fn test_adapter_filter() {
        assert!(is_iterator_adapter("filter"));
    }

    #[test]
    fn test_adapter_filter_map() {
        assert!(is_iterator_adapter("filter_map"));
    }

    #[test]
    fn test_adapter_flat_map() {
        assert!(is_iterator_adapter("flat_map"));
    }

    #[test]
    fn test_adapter_flatten() {
        assert!(is_iterator_adapter("flatten"));
    }

    #[test]
    fn test_adapter_take() {
        assert!(is_iterator_adapter("take"));
    }

    #[test]
    fn test_adapter_skip() {
        assert!(is_iterator_adapter("skip"));
    }

    #[test]
    fn test_adapter_take_while() {
        assert!(is_iterator_adapter("take_while"));
    }

    #[test]
    fn test_adapter_skip_while() {
        assert!(is_iterator_adapter("skip_while"));
    }

    #[test]
    fn test_adapter_enumerate() {
        assert!(is_iterator_adapter("enumerate"));
    }

    #[test]
    fn test_adapter_zip() {
        assert!(is_iterator_adapter("zip"));
    }

    #[test]
    fn test_adapter_chain() {
        assert!(is_iterator_adapter("chain"));
    }

    #[test]
    fn test_adapter_cycle() {
        assert!(is_iterator_adapter("cycle"));
    }

    #[test]
    fn test_adapter_rev() {
        assert!(is_iterator_adapter("rev"));
    }

    #[test]
    fn test_adapter_peekable() {
        assert!(is_iterator_adapter("peekable"));
    }

    #[test]
    fn test_adapter_inspect() {
        assert!(is_iterator_adapter("inspect"));
    }

    #[test]
    fn test_adapter_fuse() {
        assert!(is_iterator_adapter("fuse"));
    }

    #[test]
    fn test_adapter_by_ref() {
        assert!(is_iterator_adapter("by_ref"));
    }

    #[test]
    fn test_not_adapter() {
        assert!(!is_iterator_adapter("collect"));
        assert!(!is_iterator_adapter("sum"));
    }

    // ============================================================================
    // is_collection_literal tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_list_is_collection_literal() {
        let expr = HirExpr::List(vec![]);
        assert!(is_collection_literal(&expr));
    }

    #[test]
    fn test_tuple_is_collection_literal() {
        let expr = HirExpr::Tuple(vec![]);
        assert!(is_collection_literal(&expr));
    }

    #[test]
    fn test_set_is_collection_literal() {
        let expr = HirExpr::Set(vec![]);
        assert!(is_collection_literal(&expr));
    }

    #[test]
    fn test_dict_is_not_simple_collection_literal() {
        let expr = HirExpr::Dict(vec![]);
        assert!(!is_collection_literal(&expr));
    }

    #[test]
    fn test_var_is_not_collection_literal() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_collection_literal(&expr));
    }

    // ============================================================================
    // is_dict_literal tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_dict_is_dict_literal() {
        let expr = HirExpr::Dict(vec![]);
        assert!(is_dict_literal(&expr));
    }

    #[test]
    fn test_list_is_not_dict_literal() {
        let expr = HirExpr::List(vec![]);
        assert!(!is_dict_literal(&expr));
    }

    // ============================================================================
    // is_any_collection_literal tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_list_is_any_collection() {
        let expr = HirExpr::List(vec![]);
        assert!(is_any_collection_literal(&expr));
    }

    #[test]
    fn test_dict_is_any_collection() {
        let expr = HirExpr::Dict(vec![]);
        assert!(is_any_collection_literal(&expr));
    }

    #[test]
    fn test_var_is_not_any_collection() {
        let expr = HirExpr::Var("x".to_string());
        assert!(!is_any_collection_literal(&expr));
    }

    // ============================================================================
    // is_string_expr tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_string_literal_is_string() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(is_string_expr(&expr));
    }

    #[test]
    fn test_fstring_is_string() {
        let expr = HirExpr::FString { parts: vec![] };
        assert!(is_string_expr(&expr));
    }

    #[test]
    fn test_join_method_is_string() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Literal(Literal::String(",".to_string()))),
            method: "join".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_string_expr(&expr));
    }

    #[test]
    fn test_upper_method_is_string() {
        let expr = HirExpr::MethodCall {
            object: Box::new(HirExpr::Var("s".to_string())),
            method: "upper".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert!(is_string_expr(&expr));
    }

    #[test]
    fn test_int_is_not_string() {
        let expr = HirExpr::Literal(Literal::Int(42));
        assert!(!is_string_expr(&expr));
    }

    // ============================================================================
    // is_bytes_expr tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_bytes_literal_is_bytes() {
        let expr = HirExpr::Literal(Literal::Bytes(b"hello".to_vec()));
        assert!(is_bytes_expr(&expr));
    }

    #[test]
    fn test_string_is_not_bytes() {
        let expr = HirExpr::Literal(Literal::String("hello".to_string()));
        assert!(!is_bytes_expr(&expr));
    }

    // ============================================================================
    // get_range_length tests - 100% coverage
    // ============================================================================

    #[test]
    fn test_range_one_arg() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Literal(Literal::Int(10))],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), Some(10));
    }

    #[test]
    fn test_range_two_args() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(5)),
                HirExpr::Literal(Literal::Int(15)),
            ],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), Some(10));
    }

    #[test]
    fn test_range_three_args() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(0)),
                HirExpr::Literal(Literal::Int(10)),
                HirExpr::Literal(Literal::Int(2)),
            ],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), Some(5));
    }

    #[test]
    fn test_range_zero_step() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![
                HirExpr::Literal(Literal::Int(0)),
                HirExpr::Literal(Literal::Int(10)),
                HirExpr::Literal(Literal::Int(0)),
            ],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), None);
    }

    #[test]
    fn test_range_non_literal_arg() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![HirExpr::Var("n".to_string())],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), None);
    }

    #[test]
    fn test_non_range_call() {
        let expr = HirExpr::Call {
            func: "len".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), None);
    }

    #[test]
    fn test_range_empty_args() {
        let expr = HirExpr::Call {
            func: "range".to_string(),
            args: vec![],
            kwargs: vec![],
        };
        assert_eq!(get_range_length(&expr), None);
    }
}
