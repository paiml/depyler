//! Random Module Code Generation - EXTREME TDD
//!
//! Handles Python `random` module method conversions to Rust rand crate.
//! Extracted from expr_gen.rs for testability and maintainability.
//!
//! Coverage target: 100% line coverage, 100% branch coverage

use crate::hir::HirExpr;
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python random module method calls to Rust rand crate
///
/// # Supported Methods
/// - Basic: random, randint, randrange, uniform
/// - Sequence: choice, shuffle, sample, choices
/// - Distribution: gauss/normalvariate, expovariate
///
/// # Complexity: 10 (match with 10+ branches)
pub fn convert_random_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    ctx.needs_rand = true;

    let result = match method {
        "random" => convert_random(&arg_exprs)?,
        "randint" => convert_randint(&arg_exprs)?,
        "randrange" => convert_randrange(&arg_exprs)?,
        "uniform" => convert_uniform(&arg_exprs)?,
        "choice" => convert_choice(&arg_exprs)?,
        "shuffle" => convert_shuffle(&arg_exprs)?,
        "sample" => convert_sample(&arg_exprs)?,
        "choices" => convert_choices(&arg_exprs)?,
        "gauss" | "normalvariate" => {
            ctx.needs_rand_distr = true;
            convert_gauss(method, &arg_exprs)?
        }
        "expovariate" => {
            ctx.needs_rand_distr = true;
            convert_expovariate(&arg_exprs)?
        }
        "seed" => convert_seed(&arg_exprs)?,
        _ => bail!("random.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// random.random() → rand::random::<f64>()
fn convert_random(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if !arg_exprs.is_empty() {
        bail!("random.random() takes no arguments");
    }
    Ok(parse_quote! { rand::random::<f64>() })
}

/// random.randint(a, b) → inclusive range
fn convert_randint(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("random.randint() requires exactly 2 arguments");
    }
    let a = &arg_exprs[0];
    let b = &arg_exprs[1];
    Ok(parse_quote! {
        {
            use rand::Rng;
            rand::thread_rng().gen_range(#a..=#b)
        }
    })
}

/// random.randrange(stop) or (start, stop) or (start, stop, step)
fn convert_randrange(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() || arg_exprs.len() > 3 {
        bail!("random.randrange() requires 1-3 arguments");
    }

    if arg_exprs.len() == 1 {
        let stop = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use rand::Rng;
                rand::thread_rng().gen_range(0..#stop)
            }
        })
    } else if arg_exprs.len() == 2 {
        let start = &arg_exprs[0];
        let stop = &arg_exprs[1];
        Ok(parse_quote! {
            {
                use rand::Rng;
                rand::thread_rng().gen_range(#start..#stop)
            }
        })
    } else {
        let start = &arg_exprs[0];
        let stop = &arg_exprs[1];
        let step = &arg_exprs[2];
        Ok(parse_quote! {
            {
                use rand::Rng;
                let start = #start;
                let stop = #stop;
                let step: i32 = #step;
                let num_steps = ((stop - start) / step).max(0);
                let offset = rand::thread_rng().gen_range(0..num_steps);
                start + offset * step
            }
        })
    }
}

/// random.uniform(a, b) → float in [a, b]
fn convert_uniform(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("random.uniform() requires exactly 2 arguments");
    }
    let a = &arg_exprs[0];
    let b = &arg_exprs[1];
    Ok(parse_quote! {
        {
            use rand::Rng;
            rand::thread_rng().gen_range((#a as f64)..=(#b as f64))
        }
    })
}

/// random.choice(seq) → random element
fn convert_choice(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("random.choice() requires exactly 1 argument");
    }
    let seq = &arg_exprs[0];
    Ok(parse_quote! {
        {
            use rand::seq::SliceRandom;
            *#seq.choose(&mut rand::thread_rng()).unwrap()
        }
    })
}

/// random.shuffle(seq) → shuffle in place
fn convert_shuffle(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("random.shuffle() requires exactly 1 argument");
    }
    let seq = &arg_exprs[0];
    Ok(parse_quote! {
        {
            use rand::seq::SliceRandom;
            #seq.shuffle(&mut rand::thread_rng())
        }
    })
}

/// random.sample(seq, k) → k unique elements
fn convert_sample(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("random.sample() requires exactly 2 arguments");
    }
    let seq = &arg_exprs[0];
    let k = &arg_exprs[1];
    Ok(parse_quote! {
        {
            use rand::seq::SliceRandom;
            #seq.choose_multiple(&mut rand::thread_rng(), #k as usize)
                .cloned()
                .collect::<Vec<_>>()
        }
    })
}

/// random.choices(seq, k=1) → k elements with replacement
fn convert_choices(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("random.choices() requires at least 1 argument");
    }
    let seq = &arg_exprs[0];
    let k = if arg_exprs.len() > 1 {
        arg_exprs[1].clone()
    } else {
        parse_quote! { 1 }
    };
    Ok(parse_quote! {
        {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            (0..#k)
                .map(|_| #seq.choose(&mut rng).cloned().unwrap())
                .collect::<Vec<_>>()
        }
    })
}

/// random.gauss(mu, sigma) → normal distribution
fn convert_gauss(method: &str, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 2 {
        bail!("random.{}() requires exactly 2 arguments", method);
    }
    let mu = &arg_exprs[0];
    let sigma = &arg_exprs[1];
    Ok(parse_quote! {
        {
            use rand::distributions::Distribution;
            let normal = rand_distr::Normal::new(#mu as f64, #sigma as f64).unwrap();
            normal.sample(&mut rand::thread_rng())
        }
    })
}

/// random.expovariate(lambd) → exponential distribution
fn convert_expovariate(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() != 1 {
        bail!("random.expovariate() requires exactly 1 argument");
    }
    let lambd = &arg_exprs[0];
    Ok(parse_quote! {
        {
            use rand::distributions::Distribution;
            let exp = rand_distr::Exp::new(#lambd as f64).unwrap();
            exp.sample(&mut rand::thread_rng())
        }
    })
}

/// random.seed([n]) → set RNG seed (simplified: no-op)
fn convert_seed(arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
    if arg_exprs.len() > 1 {
        bail!("random.seed() requires 0 or 1 argument");
    }
    if arg_exprs.is_empty() {
        // seed() with no args - use system entropy (no-op, thread_rng is already seeded)
        Ok(parse_quote! { () })
    } else {
        let seed_val = &arg_exprs[0];
        // Note: thread_rng() cannot be seeded. For now, we generate a no-op.
        Ok(parse_quote! {
            {
                // Note: Seeding not fully implemented - use StdRng instead of thread_rng
                let _seed = #seed_val;
                ()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::Literal;

    #[test]
    fn test_convert_random_random() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("random", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_rand);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("rand") && code.contains("random"));
    }

    #[test]
    fn test_convert_random_random_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(1))];
        let result = convert_random_method("random", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_randint() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(1)),
            HirExpr::Literal(Literal::Int(10)),
        ];
        let result = convert_random_method("randint", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("gen_range"));
    }

    #[test]
    fn test_convert_random_randint_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(1))];
        let result = convert_random_method("randint", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_randrange_single() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Int(10))];
        let result = convert_random_method("randrange", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_randrange_two() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(5)),
            HirExpr::Literal(Literal::Int(10)),
        ];
        let result = convert_random_method("randrange", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_randrange_three() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Int(0)),
            HirExpr::Literal(Literal::Int(10)),
            HirExpr::Literal(Literal::Int(2)),
        ];
        let result = convert_random_method("randrange", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_randrange_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("randrange", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_uniform() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(0.0)),
            HirExpr::Literal(Literal::Float(1.0)),
        ];
        let result = convert_random_method("uniform", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_uniform_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(0.0))];
        let result = convert_random_method("uniform", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_choice() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_random_method("choice", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("choose"));
    }

    #[test]
    fn test_convert_random_choice_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("choice", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_shuffle() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_random_method("shuffle", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("shuffle"));
    }

    #[test]
    fn test_convert_random_shuffle_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("shuffle", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_sample() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(3)),
        ];
        let result = convert_random_method("sample", &args, &mut ctx);
        assert!(result.is_ok());
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("choose_multiple"));
    }

    #[test]
    fn test_convert_random_sample_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_random_method("sample", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_choices() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Var("items".to_string())];
        let result = convert_random_method("choices", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_choices_with_k() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Var("items".to_string()),
            HirExpr::Literal(Literal::Int(5)),
        ];
        let result = convert_random_method("choices", &args, &mut ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_random_choices_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("choices", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_gauss() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(0.0)),
            HirExpr::Literal(Literal::Float(1.0)),
        ];
        let result = convert_random_method("gauss", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_rand_distr);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Normal"));
    }

    #[test]
    fn test_convert_random_normalvariate() {
        let mut ctx = CodeGenContext::default();
        let args = vec![
            HirExpr::Literal(Literal::Float(0.0)),
            HirExpr::Literal(Literal::Float(1.0)),
        ];
        let result = convert_random_method("normalvariate", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_rand_distr);
    }

    #[test]
    fn test_convert_random_gauss_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(0.0))];
        let result = convert_random_method("gauss", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_expovariate() {
        let mut ctx = CodeGenContext::default();
        let args = vec![HirExpr::Literal(Literal::Float(1.0))];
        let result = convert_random_method("expovariate", &args, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.needs_rand_distr);
        let expr = result.unwrap().unwrap();
        let code = quote::quote!(#expr).to_string();
        assert!(code.contains("Exp"));
    }

    #[test]
    fn test_convert_random_expovariate_wrong_args() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("expovariate", &args, &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_random_unknown() {
        let mut ctx = CodeGenContext::default();
        let args: Vec<HirExpr> = vec![];
        let result = convert_random_method("unknown", &args, &mut ctx);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not implemented"));
    }
}
