//! Builtin function call conversion for ExprConverter
//!
//! Handles len, ord, chr, list, bytes, tuple, enumerate, zip, sorted, sum, etc.

use crate::direct_rules::make_ident;
use crate::hir::*;
use anyhow::{bail, Result};
use quote::quote;
use syn::parse_quote;

use super::ExprConverter;

impl<'a> ExprConverter<'a> {
    pub(super) fn convert_call(&self, func: &str, args: &[HirExpr]) -> Result<syn::Expr> {
        // Handle classmethod cls(args) → Self::new(args)
        if func == "cls" && self.is_classmethod {
            let arg_exprs: Vec<syn::Expr> = args
                .iter()
                .map(|arg| self.convert(arg))
                .collect::<Result<Vec<_>>>()?;
            return Ok(parse_quote! { Self::new(#(#arg_exprs),*) });
        }

        let arg_exprs: Vec<syn::Expr> = args
            .iter()
            .map(|arg| self.convert(arg))
            .collect::<Result<Vec<_>>>()?;

        match func {
            "len" => self.convert_len_call(&arg_exprs),
            "range" => self.convert_range_call(&arg_exprs),
            // DEPYLER-1001: enumerate(iterable) → iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
            "enumerate" => self.convert_enumerate_call(&arg_exprs),
            // DEPYLER-1001: zip(a, b) → a.into_iter().zip(b.into_iter())
            "zip" => self.convert_zip_call(&arg_exprs),
            // DEPYLER-1001: reversed(iterable) → iterable.iter().cloned().rev()
            "reversed" => self.convert_reversed_call(&arg_exprs),
            // DEPYLER-1001: sorted(iterable) → sorted Vec
            "sorted" => self.convert_sorted_call(&arg_exprs),
            "zeros" | "ones" | "full" => self.convert_array_init_call(func, args, &arg_exprs),
            "set" => self.convert_set_constructor(&arg_exprs),
            "frozenset" => self.convert_frozenset_constructor(&arg_exprs),
            // DEPYLER-0200: File I/O builtins
            "open" => self.convert_open_call(args, &arg_exprs),
            // DEPYLER-0200: datetime builtins
            "date" => self.convert_date_call(&arg_exprs),
            "datetime" => self.convert_datetime_call(&arg_exprs),
            // DEPYLER-0721: os.path functions imported via `from os.path import X`
            "splitext" => self.convert_splitext_call(&arg_exprs),
            "basename" => self.convert_basename_call(&arg_exprs),
            "dirname" => self.convert_dirname_call(&arg_exprs),
            "split" => self.convert_path_split_call(&arg_exprs),
            "exists" => self.convert_path_exists_call(&arg_exprs),
            "isfile" => self.convert_path_isfile_call(&arg_exprs),
            "isdir" => self.convert_path_isdir_call(&arg_exprs),
            // DEPYLER-0844: isinstance(x, T) → true (Rust's type system guarantees correctness)
            "isinstance" => Ok(parse_quote! { true }),
            // DEPYLER-0906: ord(c) → c.chars().next().unwrap() as i32
            // Python ord() returns Unicode code point as int
            "ord" => self.convert_ord_call(&arg_exprs),
            // DEPYLER-0906: chr(n) → char::from_u32(n as u32).unwrap().to_string()
            // Python chr() returns single character string from Unicode code point
            "chr" => self.convert_chr_call(&arg_exprs),
            // DEPYLER-0931: list() builtin for class method bodies
            // list() → Vec::new()
            // list(iterable) → iterable.into_iter().collect::<Vec<_>>()
            "list" => self.convert_list_call(&arg_exprs),
            // DEPYLER-0935: bytes() builtin for class method bodies
            // bytes() → Vec::<u8>::new()
            // bytes(iterable) → iterable.into_iter().map(|x| x as u8).collect::<Vec<u8>>()
            "bytes" => self.convert_bytes_call(&arg_exprs),
            // DEPYLER-0936: bytearray() builtin for class method bodies
            // bytearray() → Vec::<u8>::new()
            // bytearray(n) → vec![0u8; n]
            "bytearray" => self.convert_bytearray_call(&arg_exprs),
            // DEPYLER-0937: tuple() builtin for class method bodies
            // tuple() → Vec::new()
            // tuple(iterable) → iterable.into_iter().collect::<Vec<_>>()
            "tuple" => self.convert_tuple_call(&arg_exprs),
            // DEPYLER-0968: sum() builtin for class method bodies
            // sum(iterable) → iterable.iter().sum::<T>()
            // sum(generator_expr) → generator_expr.sum::<T>()
            "sum" => self.convert_sum_call(args, &arg_exprs),
            // DEPYLER-1097: all() builtin - Python all(iterable) → Rust iterable.iter().all(|x| *x)
            "all" => self.convert_all_call(args, &arg_exprs),
            // DEPYLER-1097: any() builtin - Python any(iterable) → Rust iterable.iter().any(|x| *x)
            "any" => self.convert_any_call(args, &arg_exprs),
            // DEPYLER-1097: dict() builtin - Python dict() → Rust HashMap::new()
            "dict" => self.convert_dict_call(&arg_exprs),
            // DEPYLER-0780: Pass HIR args for auto-borrowing detection
            _ => self.convert_generic_call(func, args, &arg_exprs),
        }
    }

    pub(super) fn convert_len_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("len() requires exactly one argument");
        }
        let arg = &args[0];
        // DEPYLER-0693: Cast len() to i32 for Python compatibility
        // Python int maps to Rust i32, and len() in Python returns int
        Ok(parse_quote! { #arg.len() as i32 })
    }

    /// DEPYLER-0906: Convert Python ord(c) to Rust char code point
    ///
    /// Python: ord('a') → 97
    /// Rust: 'a'.chars().next().unwrap() as i32
    ///
    /// For single-char strings, get first char and convert to i32.
    pub(super) fn convert_ord_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("ord() requires exactly one argument");
        }
        let char_str = &args[0];
        Ok(parse_quote! { #char_str.chars().next().expect("empty string") as i32 })
    }

    /// DEPYLER-0906: Convert Python chr(n) to Rust char string
    ///
    /// Python: chr(97) → 'a'
    /// Rust: char::from_u32(97u32).unwrap().to_string()
    ///
    /// Converts Unicode code point to single-character String.
    pub(super) fn convert_chr_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("chr() requires exactly one argument");
        }
        let code = &args[0];
        Ok(parse_quote! { char::from_u32(#code as u32).expect("invalid char").to_string() })
    }

    /// DEPYLER-0931: Convert Python list() builtin to Rust Vec
    ///
    /// list() → Vec::new()
    /// list(iterable) → iterable.into_iter().collect::<Vec<_>>()
    /// list(dict.keys()) → dict.keys().cloned().collect::<Vec<_>>()
    pub(super) fn convert_list_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // list() → Vec::new()
            Ok(parse_quote! { Vec::new() })
        } else if args.len() == 1 {
            let iterable = &args[0];
            // DEPYLER-0931: Check if the iterable is a method call returning references
            // dict.keys(), dict.values(), list.iter() all return iterators of references
            // that need .cloned() before .collect()
            let needs_clone = if let syn::Expr::MethodCall(method_call) = iterable {
                matches!(
                    method_call.method.to_string().as_str(),
                    "keys" | "values" | "iter" | "items"
                )
            } else {
                false
            };

            if needs_clone {
                // For reference iterators: use .cloned().collect()
                Ok(parse_quote! { #iterable.cloned().collect::<Vec<_>>() })
            } else {
                // For owned iterators: use .into_iter().collect()
                Ok(parse_quote! { #iterable.into_iter().collect::<Vec<_>>() })
            }
        } else {
            bail!("list() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-0935: bytes() builtin for class method bodies
    /// In Python, bytes(n) creates n zero bytes, bytes([list]) collects the list
    pub(super) fn convert_bytes_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytes() → Vec::<u8>::new()
            Ok(parse_quote! { Vec::<u8>::new() })
        } else if args.len() == 1 {
            let arg = &args[0];
            // Default to bytes(n) → vec![0u8; n as usize] for numeric expressions
            // This is the most common case in real code
            Ok(parse_quote! { vec![0u8; (#arg) as usize] })
        } else {
            bail!("bytes() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-0936: bytearray() builtin for class method bodies
    /// In Python, bytearray(n) creates n zero bytes, bytearray([list]) collects the list
    pub(super) fn convert_bytearray_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // bytearray() → Vec::<u8>::new()
            Ok(parse_quote! { Vec::<u8>::new() })
        } else if args.len() == 1 {
            let arg = &args[0];
            // Default to bytearray(n) → vec![0u8; n as usize] for numeric expressions
            // This is the most common case in real code
            Ok(parse_quote! { vec![0u8; (#arg) as usize] })
        } else {
            bail!(
                "bytearray() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

    /// DEPYLER-0937: tuple() builtin for class method bodies
    pub(super) fn convert_tuple_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // tuple() → Vec::new()
            Ok(parse_quote! { Vec::new() })
        } else if args.len() == 1 {
            let iterable = &args[0];
            // tuple(iterable) → iterable.into_iter().collect()
            Ok(parse_quote! { #iterable.into_iter().collect::<Vec<_>>() })
        } else {
            bail!("tuple() takes at most 1 argument ({} given)", args.len())
        }
    }

    /// DEPYLER-1001: enumerate(iterable) → iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
    /// enumerate(iterable, start) → iterable.iter().cloned().enumerate().map(|(i, x)| ((i + start) as i32, x))
    pub(super) fn convert_enumerate_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() || args.len() > 2 {
            bail!("enumerate() requires 1 or 2 arguments");
        }
        let iterable = &args[0];
        if args.len() == 2 {
            let start = &args[1];
            Ok(parse_quote! {
                #iterable.iter().cloned().enumerate().map(|(i, x)| ((i + #start as usize) as i32, x))
            })
        } else {
            Ok(parse_quote! {
                #iterable.iter().cloned().enumerate().map(|(i, x)| (i as i32, x))
            })
        }
    }

    /// DEPYLER-1001: zip(a, b) → a.into_iter().zip(b.into_iter())
    pub(super) fn convert_zip_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() < 2 {
            bail!("zip() requires at least 2 arguments");
        }
        let first = &args[0];
        let second = &args[1];
        if args.len() == 2 {
            Ok(parse_quote! { #first.into_iter().zip(#second.into_iter()) })
        } else {
            let mut zip_expr: syn::Expr =
                parse_quote! { #first.into_iter().zip(#second.into_iter()) };
            for iter in &args[2..] {
                zip_expr = parse_quote! { #zip_expr.zip(#iter.into_iter()) };
            }
            Ok(zip_expr)
        }
    }

    /// DEPYLER-1001: reversed(iterable) → iterable.iter().cloned().rev()
    pub(super) fn convert_reversed_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.len() != 1 {
            bail!("reversed() requires exactly 1 argument");
        }
        let iterable = &args[0];
        Ok(parse_quote! { #iterable.iter().cloned().rev() })
    }

    /// DEPYLER-1001: sorted(iterable) → sorted Vec with partial_cmp for float support
    pub(super) fn convert_sorted_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
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

    /// DEPYLER-0968: sum() builtin for class method bodies
    ///
    /// Handles Python sum() function conversion to Rust iterator patterns.
    ///
    /// Variants:
    /// - sum(generator_exp) → generator_expr.sum::<T>()
    /// - sum(range(...)) → (range_expr).sum::<T>()
    /// - sum(d.values()) / sum(d.keys()) → d.values().cloned().sum::<T>()
    /// - sum(iterable) → iterable.iter().sum::<T>()
    pub(super) fn convert_sum_call(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if hir_args.len() != 1 || arg_exprs.len() != 1 {
            bail!("sum() requires exactly one argument");
        }

        let arg_expr = &arg_exprs[0];

        // Detect target type from class field types or default to f64
        let target_type: syn::Type = self.infer_sum_type(&hir_args[0]);

        // Check if argument is a generator expression (already converted to .iter().map())
        // or a method call like .values()/.keys()
        match &hir_args[0] {
            // Generator expression: sum(x*x for x in items) → items.iter().map(|x| x*x).sum::<T>()
            // The generator is already converted to .iter().map(), so just append .sum()
            HirExpr::GeneratorExp { .. } => Ok(parse_quote! { #arg_expr.sum::<#target_type>() }),
            // Range call: sum(range(n)) → (0..n).sum::<T>()
            HirExpr::Call { func, .. } if func == "range" => {
                Ok(parse_quote! { (#arg_expr).sum::<#target_type>() })
            }
            // Method call: sum(d.values()) → d.values().cloned().sum::<T>()
            HirExpr::MethodCall {
                method,
                args: method_args,
                ..
            } if (method == "values" || method == "keys") && method_args.is_empty() => {
                Ok(parse_quote! { #arg_expr.cloned().sum::<#target_type>() })
            }
            // Default: sum(iterable) → iterable.iter().sum::<T>()
            _ => {
                // Check if the converted expression already has .iter()/.map()
                let expr_str = quote::quote!(#arg_expr).to_string();
                if expr_str.contains(".iter()") || expr_str.contains(".map(") {
                    // Already an iterator, just append .sum()
                    Ok(parse_quote! { #arg_expr.sum::<#target_type>() })
                } else {
                    // Need to add .iter()
                    Ok(parse_quote! { #arg_expr.iter().sum::<#target_type>() })
                }
            }
        }
    }

    /// Infer the target type for sum() based on the expression context
    pub(super) fn infer_sum_type(&self, expr: &HirExpr) -> syn::Type {
        // Check if we can determine the type from class field types
        match expr {
            HirExpr::Attribute { value, attr } => {
                if matches!(value.as_ref(), HirExpr::Var(v) if v == "self") {
                    if let Some(field_type) = self.class_field_types.get(attr) {
                        return match field_type {
                            Type::List(elem_type) => match elem_type.as_ref() {
                                Type::Int => parse_quote! { i32 },
                                Type::Float => parse_quote! { f64 },
                                _ => parse_quote! { f64 },
                            },
                            _ => parse_quote! { f64 },
                        };
                    }
                }
            }
            HirExpr::GeneratorExp { generators, .. } => {
                // Try to infer from the iteration target (first generator)
                if let Some(gen) = generators.first() {
                    return self.infer_sum_type(&gen.iter);
                }
            }
            _ => {}
        }
        // Default to f64 for floating point operations
        parse_quote! { f64 }
    }

    /// DEPYLER-1097: Convert Python all() builtin to Rust
    ///
    /// Python: all(iterable) → True if all elements are truthy
    /// Rust: iterable.iter().all(|x| truthiness_check(x))
    ///
    /// For boolean iterables: iterable.iter().all(|&x| x)
    /// For other types: iterable.iter().all(|x| !x.is_empty()) etc.
    pub(super) fn convert_all_call(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if hir_args.len() != 1 || arg_exprs.len() != 1 {
            bail!("all() requires exactly one argument");
        }

        let arg_expr = &arg_exprs[0];

        // Check if this is a generator expression
        match &hir_args[0] {
            // Generator expression: all(x > 0 for x in items) → items.iter().all(|x| x > 0)
            // Already converted to .iter().map(), so transform to .all()
            HirExpr::GeneratorExp { .. } => {
                // Generator is converted to iterator chain, add .all(|x| x)
                Ok(parse_quote! { #arg_expr.all(|x| x) })
            }
            _ => {
                // Regular iterable: all(items) → items.iter().all(|&x| x)
                // For bool slice: iter().all(|&x| x)
                Ok(parse_quote! { #arg_expr.iter().all(|&x| x) })
            }
        }
    }

    /// DEPYLER-1097: Convert Python any() builtin to Rust
    ///
    /// Python: any(iterable) → True if any element is truthy
    /// Rust: iterable.iter().any(|x| truthiness_check(x))
    pub(super) fn convert_any_call(&self, hir_args: &[HirExpr], arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if hir_args.len() != 1 || arg_exprs.len() != 1 {
            bail!("any() requires exactly one argument");
        }

        let arg_expr = &arg_exprs[0];

        // Check if this is a generator expression
        match &hir_args[0] {
            // Generator expression: any(x > 0 for x in items) → items.iter().any(|x| x > 0)
            HirExpr::GeneratorExp { .. } => Ok(parse_quote! { #arg_expr.any(|x| x) }),
            _ => {
                // Regular iterable: any(items) → items.iter().any(|&x| x)
                Ok(parse_quote! { #arg_expr.iter().any(|&x| x) })
            }
        }
    }

    /// DEPYLER-1097: Convert Python dict() builtin to Rust
    ///
    /// Python: dict() → {} (empty dict)
    /// Python: dict(iterable) → dict from key-value pairs
    /// Rust: HashMap::new() or HashMap::from_iter()
    pub(super) fn convert_dict_call(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.is_empty() {
            // Empty dict: dict() → HashMap::new()
            Ok(parse_quote! { std::collections::HashMap::new() })
        } else if arg_exprs.len() == 1 {
            // Convert from iterable: dict(pairs) → pairs.into_iter().collect()
            let arg = &arg_exprs[0];
            Ok(parse_quote! { #arg.into_iter().collect::<std::collections::HashMap<_, _>>() })
        } else {
            bail!("dict() takes at most 1 argument")
        }
    }

    pub(super) fn convert_range_call(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        match args.len() {
            1 => {
                let end = &args[0];
                Ok(parse_quote! { 0..#end })
            }
            2 => {
                let start = &args[0];
                let end = &args[1];
                Ok(parse_quote! { #start..#end })
            }
            3 => {
                // Step parameter requires custom iterator implementation
                bail!("range() with step parameter not yet supported")
            }
            _ => bail!("Invalid number of arguments for range()"),
        }
    }

    pub(super) fn convert_array_init_call(
        &self,
        func: &str,
        args: &[HirExpr],
        _arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        // Handle zeros(n), ones(n), full(n, value) patterns
        if args.is_empty() {
            bail!("{} requires at least one argument", func);
        }

        // DEPYLER-0695: Always use vec![] for zeros/ones/full to ensure consistent
        // Vec<T> return type. Using fixed arrays [0; N] causes type mismatches when
        // functions return Vec<T> (Python lists are always dynamically sized).
        let size_expr = self.convert(&args[0])?;
        match func {
            "zeros" => Ok(parse_quote! { vec![0; #size_expr as usize] }),
            "ones" => Ok(parse_quote! { vec![1; #size_expr as usize] }),
            "full" => {
                if args.len() >= 2 {
                    let value = self.convert(&args[1])?;
                    Ok(parse_quote! { vec![#value; #size_expr as usize] })
                } else {
                    bail!("full() requires a value argument");
                }
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn convert_set_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty set: set()
            // DEPYLER-0409: Use default type i32 to avoid "type annotations needed" error
            // when the variable is unused or type can't be inferred from context
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::collections::HashSet::<i32>::new() })
        } else if args.len() == 1 {
            // Set from iterable: set([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    vec![#elems].into_iter().collect::<std::collections::HashSet<_>>()
                })
            } else {
                Ok(parse_quote! {
                    #arg.into_iter().collect::<std::collections::HashSet<_>>()
                })
            }
        } else {
            bail!("set() takes at most 1 argument ({} given)", args.len())
        }
    }

    pub(super) fn convert_frozenset_constructor(&self, args: &[syn::Expr]) -> Result<syn::Expr> {
        if args.is_empty() {
            // Empty frozenset: frozenset()
            // DEPYLER-0409: Use default type i32 for empty sets
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            Ok(parse_quote! { std::sync::Arc::new(std::collections::HashSet::<i32>::new()) })
        } else if args.len() == 1 {
            // Frozenset from iterable: frozenset([1, 2, 3])
            let arg = &args[0];
            // DEPYLER-0797: Check if arg is a tuple - tuples don't implement IntoIterator in Rust
            // Convert tuple to vec! for iteration
            // DEPYLER-0831: Use fully-qualified path for E0412 resolution
            if let syn::Expr::Tuple(tuple) = arg {
                let elems = &tuple.elems;
                Ok(parse_quote! {
                    std::sync::Arc::new(vec![#elems].into_iter().collect::<std::collections::HashSet<_>>())
                })
            } else {
                Ok(parse_quote! {
                    std::sync::Arc::new(#arg.into_iter().collect::<std::collections::HashSet<_>>())
                })
            }
        } else {
            bail!(
                "frozenset() takes at most 1 argument ({} given)",
                args.len()
            )
        }
    }

}
