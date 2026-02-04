//! Itertools Module Code Generation - EXTREME TDD
//!
//! Handles Python `itertools` module method conversions to Rust iterator adapters.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python itertools module method calls to Rust iterator adapters
///
/// # Supported Methods
/// - count: Infinite counter with optional start/step
/// - cycle: Cycle through iterable infinitely
/// - repeat: Repeat value n times (or infinitely)
/// - chain: Chain multiple iterables together
/// - islice: Slice iterator with start/stop
/// - takewhile: Take while predicate is true
/// - dropwhile: Drop while predicate is true
/// - accumulate: Running sum/product
/// - compress: Filter by selector booleans
/// - groupby: Group consecutive elements by key
/// - product: Cartesian product of iterables
/// - permutations: Permutations of iterable
/// - combinations: Combinations of iterable
/// - zip_longest: Zip with fill value for shorter iterables
///
/// # Complexity: 15 (match with 14 branches + default)
pub fn convert_itertools_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "count" => convert_count(&arg_exprs)?,
        "cycle" => convert_cycle(&arg_exprs)?,
        "repeat" => convert_repeat(&arg_exprs)?,
        "chain" => convert_chain(&arg_exprs)?,
        "islice" => convert_islice(&arg_exprs)?,
        "takewhile" => convert_takewhile(&arg_exprs)?,
        "dropwhile" => convert_dropwhile(&arg_exprs)?,
        "accumulate" => convert_accumulate(&arg_exprs)?,
        "compress" => convert_compress(&arg_exprs)?,
        "groupby" => convert_groupby(&arg_exprs, ctx)?,
        "product" => convert_product(&arg_exprs, ctx)?,
        "permutations" => convert_permutations(&arg_exprs, ctx)?,
        "combinations" => convert_combinations(&arg_exprs, ctx)?,
        "zip_longest" => convert_zip_longest(&arg_exprs, ctx)?,
        _ => bail!("itertools.{} not implemented yet (available: count, cycle, repeat, chain, islice, takewhile, dropwhile, accumulate, compress, groupby, product, permutations, combinations, zip_longest)", method),
    };

    Ok(Some(result))
}

/// itertools.count(start=0, step=1) - Infinite counter
fn convert_count(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    let start = if !arg_exprs.is_empty() {
        arg_exprs[0].clone()
    } else {
        parse_quote!(0)
    };
    let step = if arg_exprs.len() >= 2 {
        arg_exprs[1].clone()
    } else {
        parse_quote!(1)
    };

    Ok(parse_quote! {
        {
            let start = #start;
            let step: i32 = #step;
            std::iter::successors(Some(start), move |&n| Some(n + step))
        }
    })
}

/// itertools.cycle(iterable) - Cycle through iterable infinitely
fn convert_cycle(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("itertools.cycle() requires at least 1 argument");
    }
    let iterable = &arg_exprs[0];

    Ok(parse_quote! {
        {
            let items = #iterable;
            items.into_iter().cycle()
        }
    })
}

/// itertools.repeat(value, times=None) - Repeat value n times or infinitely
fn convert_repeat(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("itertools.repeat() requires at least 1 argument");
    }
    let value = &arg_exprs[0];

    if arg_exprs.len() >= 2 {
        let times = &arg_exprs[1];
        Ok(parse_quote! {
            {
                let val = #value;
                let n = #times as usize;
                std::iter::repeat(val).take(n)
            }
        })
    } else {
        Ok(parse_quote! {
            {
                let val = #value;
                std::iter::repeat(val)
            }
        })
    }
}

/// itertools.chain(*iterables) - Chain multiple iterables together
fn convert_chain(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.chain() requires at least 2 arguments");
    }

    let first = &arg_exprs[0];
    let second = &arg_exprs[1];

    if arg_exprs.len() == 2 {
        Ok(parse_quote! {
            {
                let a = #first;
                let b = #second;
                a.into_iter().chain(b.into_iter())
            }
        })
    } else {
        // For more than 2, chain them all
        let mut chain_expr: syn::Expr = parse_quote! {
            #first.into_iter().chain(#second.into_iter())
        };

        for item in &arg_exprs[2..] {
            chain_expr = parse_quote! {
                #chain_expr.chain(#item.into_iter())
            };
        }

        Ok(chain_expr)
    }
}

/// itertools.islice(iterable, stop) or islice(iterable, start, stop)
fn convert_islice(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.islice() requires at least 2 arguments");
    }
    let iterable = &arg_exprs[0];

    if arg_exprs.len() == 2 {
        // islice(iterable, stop)
        let stop = &arg_exprs[1];
        Ok(parse_quote! {
            {
                let items = #iterable;
                let n = #stop as usize;
                items.into_iter().take(n)
            }
        })
    } else {
        // islice(iterable, start, stop)
        let start = &arg_exprs[1];
        let stop = &arg_exprs[2];
        Ok(parse_quote! {
            {
                let items = #iterable;
                let start_idx = #start as usize;
                let stop_idx = #stop as usize;
                items.into_iter().skip(start_idx).take(stop_idx - start_idx)
            }
        })
    }
}

/// itertools.takewhile(predicate, iterable) - Take while predicate is true
fn convert_takewhile(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.takewhile() requires at least 2 arguments");
    }
    let predicate = &arg_exprs[0];
    let iterable = &arg_exprs[1];

    Ok(parse_quote! {
        {
            let pred = #predicate;
            let items = #iterable;
            items.into_iter().take_while(pred)
        }
    })
}

/// itertools.dropwhile(predicate, iterable) - Drop while predicate is true
fn convert_dropwhile(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.dropwhile() requires at least 2 arguments");
    }
    let predicate = &arg_exprs[0];
    let iterable = &arg_exprs[1];

    Ok(parse_quote! {
        {
            let pred = #predicate;
            let items = #iterable;
            items.into_iter().skip_while(pred)
        }
    })
}

/// itertools.accumulate(iterable, func=None) - Running sum/product
fn convert_accumulate(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("itertools.accumulate() requires at least 1 argument");
    }
    let iterable = &arg_exprs[0];

    Ok(parse_quote! {
        {
            let items = #iterable;
            let mut acc = None;
            items.into_iter().map(|x| {
                acc = Some(match acc {
                    None => x,
                    Some(a) => a + x,
                });
                acc.expect("iterator operation failed")
            }).collect::<Vec<_>>()
        }
    })
}

/// itertools.compress(data, selectors) - Filter by selector booleans
fn convert_compress(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.compress() requires at least 2 arguments");
    }
    let data = &arg_exprs[0];
    let selectors = &arg_exprs[1];

    Ok(parse_quote! {
        {
            let items = #data;
            let sels = #selectors;
            items.into_iter()
                .zip(sels.into_iter())
                .filter_map(|(item, sel)| if sel { Some(item) } else { None })
                .collect::<Vec<_>>()
        }
    })
}

/// itertools.groupby(iterable, key) - Group consecutive elements by key
fn convert_groupby(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.groupby() requires at least 2 arguments (iterable, key)");
    }
    let iterable = &arg_exprs[0];
    let key_func = &arg_exprs[1];

    ctx.needs_itertools = true;

    Ok(parse_quote! {
        {
            use itertools::Itertools;
            #iterable.into_iter().group_by(#key_func)
        }
    })
}

/// itertools.product(iter1, iter2) - Cartesian product of iterables
fn convert_product(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.product() requires at least 2 arguments");
    }
    ctx.needs_itertools = true;
    let first = &arg_exprs[0];
    let second = &arg_exprs[1];

    Ok(parse_quote! {
        {
            use itertools::Itertools;
            #first.into_iter().cartesian_product(#second.into_iter()).collect::<Vec<_>>()
        }
    })
}

/// itertools.permutations(iterable, r=None) - Permutations of iterable
fn convert_permutations(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("itertools.permutations() requires at least 1 argument");
    }
    ctx.needs_itertools = true;
    let iterable = &arg_exprs[0];

    if arg_exprs.len() >= 2 {
        let r = &arg_exprs[1];
        Ok(parse_quote! {
            {
                use itertools::Itertools;
                #iterable.into_iter().permutations(#r as usize).collect::<Vec<_>>()
            }
        })
    } else {
        Ok(parse_quote! {
            {
                use itertools::Itertools;
                let items: Vec<_> = #iterable.into_iter().collect();
                let n = items.len();
                items.into_iter().permutations(n).collect::<Vec<_>>()
            }
        })
    }
}

/// itertools.combinations(iterable, r) - Combinations of iterable
fn convert_combinations(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.combinations() requires at least 2 arguments (iterable, r)");
    }
    ctx.needs_itertools = true;
    let iterable = &arg_exprs[0];
    let r = &arg_exprs[1];

    Ok(parse_quote! {
        {
            use itertools::Itertools;
            #iterable.into_iter().combinations(#r as usize).collect::<Vec<_>>()
        }
    })
}

/// itertools.zip_longest(iter1, iter2) - Zip with fill value for shorter iterables
fn convert_zip_longest(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.len() < 2 {
        bail!("itertools.zip_longest() requires at least 2 arguments");
    }
    ctx.needs_itertools = true;
    let first = &arg_exprs[0];
    let second = &arg_exprs[1];

    Ok(parse_quote! {
        {
            use itertools::Itertools;
            #first.into_iter().zip_longest(#second.into_iter()).collect::<Vec<_>>()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    // ============================================
    // count() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_count_no_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("count", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("successors"));
    }

    #[test]
    fn test_convert_itertools_count_with_start() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(5))];
        let result = convert_itertools_method("count", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("successors") && code.contains("5"));
    }

    #[test]
    fn test_convert_itertools_count_with_start_and_step() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(0)),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_itertools_method("count", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("successors"));
    }

    #[test]
    fn test_convert_itertools_count_negative_step() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(10)),
            HirExpr::Var("neg_step".to_string()),
        ];
        let result = convert_itertools_method("count", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("i32")); // step is i32 to support negative
    }

    #[test]
    fn test_convert_count_direct() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_count(&arg_exprs);
        assert!(result.is_ok());
    }

    // ============================================
    // cycle() tests - 4 tests
    // ============================================

    #[test]
    fn test_convert_itertools_cycle() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_itertools_method("cycle", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("cycle"));
    }

    #[test]
    fn test_convert_itertools_cycle_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("cycle", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_cycle_direct() {
        let arg_exprs = vec![parse_quote!(vec![1, 2, 3])];
        let result = convert_cycle(&arg_exprs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_cycle_empty_direct() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_cycle(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // repeat() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_repeat_infinite() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(42))];
        let result = convert_itertools_method("repeat", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("repeat"));
        assert!(!code.contains("take")); // infinite, no take
    }

    #[test]
    fn test_convert_itertools_repeat_with_times() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(42)),
            HirExpr::Literal(Literal::Int(5)),
        ];
        let result = convert_itertools_method("repeat", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("repeat") && code.contains("take"));
    }

    #[test]
    fn test_convert_itertools_repeat_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("repeat", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_repeat_with_string() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::String("hello".to_string()))];
        let result = convert_itertools_method("repeat", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_repeat_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_repeat(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // chain() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_chain_two() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())];
        let result = convert_itertools_method("chain", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("chain"));
    }

    #[test]
    fn test_convert_itertools_chain_three() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("a".to_string()),
            HirExpr::Var("b".to_string()),
            HirExpr::Var("c".to_string()),
        ];
        let result = convert_itertools_method("chain", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        // Should have multiple chain calls
        assert!(code.matches("chain").count() >= 2);
    }

    #[test]
    fn test_convert_itertools_chain_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string())];
        let result = convert_itertools_method("chain", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_itertools_chain_many() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("a".to_string()),
            HirExpr::Var("b".to_string()),
            HirExpr::Var("c".to_string()),
            HirExpr::Var("d".to_string()),
        ];
        let result = convert_itertools_method("chain", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_chain_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_chain(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // islice() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_islice_stop_only() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(5)),
        ];
        let result = convert_itertools_method("islice", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("take"));
        assert!(!code.contains("skip"));
    }

    #[test]
    fn test_convert_itertools_islice_start_stop() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(2)),
            HirExpr::Literal(Literal::Int(5)),
        ];
        let result = convert_itertools_method("islice", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("skip") && code.contains("take"));
    }

    #[test]
    fn test_convert_itertools_islice_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_itertools_method("islice", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_islice_with_variables() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Var("start".to_string()),
            HirExpr::Var("end".to_string()),
        ];
        let result = convert_itertools_method("islice", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_islice_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_islice(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // takewhile() tests - 4 tests
    // ============================================

    #[test]
    fn test_convert_itertools_takewhile() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("pred".to_string()),
            HirExpr::Var("items".to_string()),
        ];
        let result = convert_itertools_method("takewhile", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("take_while"));
    }

    #[test]
    fn test_convert_itertools_takewhile_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("pred".to_string())];
        let result = convert_itertools_method("takewhile", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_takewhile_lambda() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("is_positive".to_string()),
            HirExpr::Var("numbers".to_string()),
        ];
        let result = convert_itertools_method("takewhile", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_takewhile_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(f)];
        let result = convert_takewhile(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // dropwhile() tests - 4 tests
    // ============================================

    #[test]
    fn test_convert_itertools_dropwhile() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("pred".to_string()),
            HirExpr::Var("items".to_string()),
        ];
        let result = convert_itertools_method("dropwhile", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("skip_while"));
    }

    #[test]
    fn test_convert_itertools_dropwhile_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("pred".to_string())];
        let result = convert_itertools_method("dropwhile", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_dropwhile_lambda() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("is_whitespace".to_string()),
            HirExpr::Var("chars".to_string()),
        ];
        let result = convert_itertools_method("dropwhile", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_dropwhile_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(f)];
        let result = convert_dropwhile(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // accumulate() tests - 4 tests
    // ============================================

    #[test]
    fn test_convert_itertools_accumulate() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("numbers".to_string())];
        let result = convert_itertools_method("accumulate", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("acc"));
    }

    #[test]
    fn test_convert_itertools_accumulate_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("accumulate", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_accumulate_with_list() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("vec_items".to_string())];
        let result = convert_itertools_method("accumulate", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_accumulate_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_accumulate(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // compress() tests - 4 tests
    // ============================================

    #[test]
    fn test_convert_itertools_compress() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Var("selectors".to_string()),
        ];
        let result = convert_itertools_method("compress", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("zip") && code.contains("filter_map"));
    }

    #[test]
    fn test_convert_itertools_compress_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("data".to_string())];
        let result = convert_itertools_method("compress", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_compress_with_bools() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("letters".to_string()),
            HirExpr::Var("mask".to_string()),
        ];
        let result = convert_itertools_method("compress", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_compress_direct_empty() {
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_compress(&arg_exprs);
        assert!(result.is_err());
    }

    // ============================================
    // groupby() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_groupby() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Var("key_fn".to_string()),
        ];
        let result = convert_itertools_method("groupby", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_itertools);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("group_by") && code.contains("Itertools"));
    }

    #[test]
    fn test_convert_itertools_groupby_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_itertools_method("groupby", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_groupby_sets_needs_itertools() {
        let mut ctx = CodeGenContext::default();
        assert!(!ctx.needs_itertools);
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Var("get_key".to_string()),
        ];
        let _ = convert_itertools_method("groupby", &args, &mut ctx);
        assert!(ctx.needs_itertools);
    }

    #[test]
    fn test_convert_groupby_with_lambda() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("words".to_string()),
            HirExpr::Var("first_char".to_string()),
        ];
        let result = convert_itertools_method("groupby", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_groupby_direct_empty() {
        let mut ctx = CodeGenContext::default();
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_groupby(&arg_exprs, &mut ctx);
        assert!(result.is_err());
    }

    // ============================================
    // product() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_product() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())];
        let result = convert_itertools_method("product", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_itertools);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("cartesian_product"));
    }

    #[test]
    fn test_convert_itertools_product_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string())];
        let result = convert_itertools_method("product", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_product_sets_needs_itertools() {
        let mut ctx = CodeGenContext::default();
        assert!(!ctx.needs_itertools);
        let args = vec![HirExpr::Var("x".to_string()), HirExpr::Var("y".to_string())];
        let _ = convert_itertools_method("product", &args, &mut ctx);
        assert!(ctx.needs_itertools);
    }

    #[test]
    fn test_convert_product_collects() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("list1".to_string()),
            HirExpr::Var("list2".to_string()),
        ];
        let result = convert_itertools_method("product", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("collect"));
    }

    #[test]
    fn test_convert_product_direct_empty() {
        let mut ctx = CodeGenContext::default();
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_product(&arg_exprs, &mut ctx);
        assert!(result.is_err());
    }

    // ============================================
    // permutations() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_permutations_no_r() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_itertools_method("permutations", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_itertools);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("permutations"));
    }

    #[test]
    fn test_convert_itertools_permutations_with_r() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_itertools_method("permutations", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("permutations"));
    }

    #[test]
    fn test_convert_itertools_permutations_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("permutations", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_convert_permutations_sets_needs_itertools() {
        let mut ctx = CodeGenContext::default();
        assert!(!ctx.needs_itertools);
        let args = vec![HirExpr::Var("data".to_string())];
        let _ = convert_itertools_method("permutations", &args, &mut ctx);
        assert!(ctx.needs_itertools);
    }

    #[test]
    fn test_convert_permutations_direct_empty() {
        let mut ctx = CodeGenContext::default();
        let arg_exprs: Vec<syn::Expr> = vec![];
        let result = convert_permutations(&arg_exprs, &mut ctx);
        assert!(result.is_err());
    }

    // ============================================
    // combinations() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_combinations() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_itertools_method("combinations", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_itertools);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("combinations"));
    }

    #[test]
    fn test_convert_itertools_combinations_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_itertools_method("combinations", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_combinations_sets_needs_itertools() {
        let mut ctx = CodeGenContext::default();
        assert!(!ctx.needs_itertools);
        let args = vec![
            HirExpr::Var("abc".to_string()),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let _ = convert_itertools_method("combinations", &args, &mut ctx);
        assert!(ctx.needs_itertools);
    }

    #[test]
    fn test_convert_combinations_with_variable_r() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("data".to_string()),
            HirExpr::Var("r".to_string()),
        ];
        let result = convert_itertools_method("combinations", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_combinations_direct_empty() {
        let mut ctx = CodeGenContext::default();
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_combinations(&arg_exprs, &mut ctx);
        assert!(result.is_err());
    }

    // ============================================
    // zip_longest() tests - 5 tests
    // ============================================

    #[test]
    fn test_convert_itertools_zip_longest() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string()), HirExpr::Var("b".to_string())];
        let result = convert_itertools_method("zip_longest", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_itertools);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("zip_longest"));
    }

    #[test]
    fn test_convert_itertools_zip_longest_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("a".to_string())];
        let result = convert_itertools_method("zip_longest", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("requires at least 2 arguments"));
    }

    #[test]
    fn test_convert_zip_longest_sets_needs_itertools() {
        let mut ctx = CodeGenContext::default();
        assert!(!ctx.needs_itertools);
        let args = vec![HirExpr::Var("x".to_string()), HirExpr::Var("y".to_string())];
        let _ = convert_itertools_method("zip_longest", &args, &mut ctx);
        assert!(ctx.needs_itertools);
    }

    #[test]
    fn test_convert_zip_longest_collects() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("list1".to_string()),
            HirExpr::Var("list2".to_string()),
        ];
        let result = convert_itertools_method("zip_longest", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("collect"));
    }

    #[test]
    fn test_convert_zip_longest_direct_empty() {
        let mut ctx = CodeGenContext::default();
        let arg_exprs: Vec<syn::Expr> = vec![parse_quote!(x)];
        let result = convert_zip_longest(&arg_exprs, &mut ctx);
        assert!(result.is_err());
    }

    // ============================================
    // Unknown method tests - 2 tests
    // ============================================

    #[test]
    fn test_convert_itertools_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_itertools_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }

    #[test]
    fn test_convert_itertools_unknown_with_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("x".to_string())];
        let result = convert_itertools_method("starmap", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }
}
