//! Comprehension handlers for ExpressionConverter
//!
//! Extracted from mod.rs to reduce file size. Contains handlers for:
//! list comprehensions, set comprehensions, dict comprehensions,
//! set operations, and related helper methods.

use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::expr_gen::ExpressionConverter;
use crate::rust_gen::walrus_helpers;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    pub(crate) fn convert_list_comp(
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
                // DEPYLER-1077: Check if variable is a string type - strings use .chars()
                let is_string_type = if let HirExpr::Var(var_name) = &*gen.iter {
                    self.ctx
                        .var_types
                        .get(var_name)
                        .map(|ty| matches!(ty, crate::hir::Type::String))
                        .unwrap_or(false)
                } else {
                    false
                };
                if is_string_type {
                    // DEPYLER-1077: String iteration uses .chars() not .iter()
                    // Also register target as a char iteration variable for ord() handling
                    self.ctx.char_iter_vars.insert(gen.target.clone());
                    parse_quote! { #iter_expr.chars() }
                } else {
                    // DEPYLER-0674: Variable iteration - use .cloned() for non-Copy types (String, Vec, etc.)
                    // .cloned() works for both Copy and Clone types, .copied() only works for Copy
                    parse_quote! { #iter_expr.iter().cloned() }
                }
            } else {
                // Direct expression (ranges, lists, etc.) - use .into_iter()
                parse_quote! { #iter_expr.into_iter() }
            };

            // DEPYLER-0792: Check if any condition contains a walrus operator (:=)
            // and if the element expression uses that walrus variable.
            // If so, we must use filter_map instead of filter + map, because
            // the walrus variable is defined in the filter closure but needed in map.
            let walrus_vars_in_conditions =
                walrus_helpers::collect_walrus_vars_from_conditions(&gen.conditions);
            let element_uses_walrus = !walrus_vars_in_conditions.is_empty()
                && walrus_helpers::expr_uses_any_var(element, &walrus_vars_in_conditions);

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
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            // After .iter().cloned(), filter receives &T reference, but condition expects T
            // Solution: let target = target.clone() inside closure shadows ref with owned value
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
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

    pub(super) fn convert_nested_generators_for_list_comp(
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

    /// Add dereference (*) to uses of target variable in expression
    /// This is needed because filter closures receive &T even when the iterator yields T
    /// Example: transforms `x > 0` to `*x > 0` when x is the target variable
    ///
    /// Note: Currently unused but kept for potential future use with filter optimization
    #[allow(dead_code)]
    pub(super) fn add_deref_to_var_uses(
        &mut self,
        expr: &HirExpr,
        target: &str,
    ) -> Result<syn::Expr> {
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

    pub(crate) fn convert_set_operation(
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

    pub(crate) fn convert_set_comp(
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
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
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

    pub(crate) fn convert_dict_comp(
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
            // DEPYLER-1000: Clone loop variable inside filter to fix E0308 reference mismatch
            for cond in &gen.conditions {
                let cond_expr = cond.to_rust_expr(self.ctx)?;
                chain = parse_quote! { #chain.filter(|#target_pat| { let #target_pat = #target_pat.clone(); #cond_expr }) };
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

    pub(super) fn convert_nested_generators_for_dict_comp(
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

    pub(super) fn build_nested_chain_for_dict(
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
}
