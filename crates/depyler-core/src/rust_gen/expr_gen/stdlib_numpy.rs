//! Stdlib numpy method converters
//!
//! DEPYLER-REFACTOR: Extracted from expr_gen/mod.rs
//!
//! Contains converters for numpy module calls:
//! - `try_convert_numpy_call` — Maps numpy API to trueno (SIMD-accelerated tensor library)
//! - `try_convert_numpy_call_nasa_mode` — Maps numpy API to std-only Vec<f64> operations

use super::ExpressionConverter;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use crate::rust_gen::numpy_gen;
use anyhow::{bail, Result};
use quote;
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Try to convert numpy module calls to trueno equivalents.
    ///
    /// Phase 3: NumPy→Trueno codegen
    ///
    /// Maps numpy API calls to trueno (SIMD-accelerated tensor library):
    /// - np.array([...]) → Vector::from_slice(&[...])
    /// - np.dot(a, b) → a.dot(&b)?
    /// - np.sum(a) → a.sum()?
    /// - np.mean(a) → a.mean()?
    /// - np.sqrt(a) → a.sqrt()?
    ///
    /// Returns None if the method is not a recognized numpy function.
    pub(crate) fn try_convert_numpy_call(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Check if this is a recognized numpy function
        if numpy_gen::parse_numpy_function(method).is_none() {
            return Ok(None);
        }

        // DEPYLER-1121: In NASA mode, generate std-only numpy emulation
        // NASA mode requires single-shot rustc compilation without external crates
        if self.ctx.type_mapper.nasa_mode {
            return self.try_convert_numpy_call_nasa_mode(method, args);
        }

        // Mark that we need trueno dependency
        self.ctx.needs_trueno = true;

        // Convert arguments to syn::Expr
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Generate trueno code based on the numpy function
        let result = match method {
            "array" => self.convert_np_array_trueno(args, &arg_exprs)?,
            "dot" => self.convert_np_dot_trueno(&arg_exprs)?,
            "sum" => self.convert_np_unary_trueno(&arg_exprs, "sum", "np.sum()")?,
            "mean" => self.convert_np_unary_trueno(&arg_exprs, "mean", "np.mean()")?,
            "sqrt" => self.convert_np_scalar_or_vec(args, &arg_exprs, "sqrt", "np.sqrt()")?,
            "abs" => self.convert_np_scalar_or_vec(args, &arg_exprs, "abs", "np.abs()")?,
            "min" | "amin" => self.convert_np_minmax_trueno(&arg_exprs, "min", "np.min()")?,
            "max" | "amax" => self.convert_np_minmax_trueno(&arg_exprs, "max", "np.max()")?,
            "exp" => self.convert_np_scalar_or_vec(args, &arg_exprs, "exp", "np.exp()")?,
            "log" => self.convert_np_scalar_or_vec_ln(args, &arg_exprs)?,
            "sin" => self.convert_np_scalar_or_vec(args, &arg_exprs, "sin", "np.sin()")?,
            "cos" => self.convert_np_scalar_or_vec(args, &arg_exprs, "cos", "np.cos()")?,
            "clip" => self.convert_np_clip_trueno(&arg_exprs)?,
            "argmax" => self.convert_np_arg_trueno(&arg_exprs, "argmax", "np.argmax()")?,
            "argmin" => self.convert_np_arg_trueno(&arg_exprs, "argmin", "np.argmin()")?,
            "std" => self.convert_np_unary_trueno(&arg_exprs, "stddev", "np.std()")?,
            "var" => self.convert_np_unary_trueno(&arg_exprs, "variance", "np.var()")?,
            "zeros" => self.convert_np_fill_trueno(&arg_exprs, "zeros", "np.zeros()")?,
            "ones" => self.convert_np_fill_trueno(&arg_exprs, "ones", "np.ones()")?,
            "norm" => self.convert_np_norm_trueno(&arg_exprs)?,
            _ => return Ok(None),
        };

        Ok(Some(result))
    }

    fn convert_np_array_trueno(
        &mut self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if let Some(HirExpr::List(elements)) = args.first() {
            let element_exprs: Vec<proc_macro2::TokenStream> = elements
                .iter()
                .map(|e| {
                    let expr = e.to_rust_expr(self.ctx)?;
                    Ok(quote::quote! { #expr })
                })
                .collect::<Result<Vec<_>>>()?;
            let call = numpy_gen::NumpyCall::Array { elements: element_exprs };
            let tokens = numpy_gen::generate_trueno_code(&call);
            return Ok(syn::parse2(tokens)?);
        }
        if let Some(arg) = arg_exprs.first() {
            Ok(parse_quote! { Vector::from_vec(#arg) })
        } else {
            Ok(parse_quote! { Vector::new() })
        }
    }

    fn convert_np_dot_trueno(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() >= 2 {
            let a = &arg_exprs[0];
            let b = &arg_exprs[1];
            Ok(parse_quote! { #a.dot(&#b).expect("dot product failed") })
        } else {
            bail!("np.dot() requires 2 arguments");
        }
    }

    fn convert_np_unary_trueno(
        &self,
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
            Ok(parse_quote! { #arr.#method_ident().expect("operation failed") })
        } else {
            bail!("{} requires 1 argument", err_msg);
        }
    }

    fn convert_np_scalar_or_vec(
        &self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("{} requires 1 argument", err_msg);
        }
        let arr = &arg_exprs[0];
        let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
        if self.is_numpy_array_expr(&args[0]) {
            Ok(parse_quote! { #arr.#method_ident().expect("operation failed") })
        } else {
            Ok(parse_quote! { #arr.#method_ident() })
        }
    }

    fn convert_np_scalar_or_vec_ln(
        &self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("np.log() requires 1 argument");
        }
        let arr = &arg_exprs[0];
        if self.is_numpy_array_expr(&args[0]) {
            Ok(parse_quote! { #arr.ln().expect("operation failed") })
        } else {
            Ok(parse_quote! { #arr.ln() })
        }
    }

    fn convert_np_minmax_trueno(
        &self,
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
            Ok(parse_quote! { #arr.#method_ident().expect("empty collection") })
        } else {
            bail!("{} requires 1 argument", err_msg);
        }
    }

    fn convert_np_clip_trueno(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() >= 3 {
            let arr = &arg_exprs[0];
            let min = &arg_exprs[1];
            let max = &arg_exprs[2];
            Ok(parse_quote! { #arr.clamp(#min as f32, #max as f32).expect("operation failed") })
        } else {
            bail!("np.clip() requires 3 arguments (array, min, max)");
        }
    }

    fn convert_np_arg_trueno(
        &self,
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
            Ok(parse_quote! { #arr.#method_ident().expect("empty collection") })
        } else {
            bail!("{} requires 1 argument", err_msg);
        }
    }

    fn convert_np_fill_trueno(
        &self,
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if let Some(size) = arg_exprs.first() {
            let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
            Ok(parse_quote! { Vector::#method_ident(#size) })
        } else {
            bail!("{} requires 1 argument", err_msg);
        }
    }

    fn convert_np_norm_trueno(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! { (#arr).norm_l2().expect("operation failed") })
        } else {
            bail!("np.norm() requires 1 argument");
        }
    }

    /// DEPYLER-1121: NASA mode numpy conversion using std-only types
    /// Maps numpy operations to Vec<f64> operations without external crates.
    ///
    /// # Mappings (NASA mode):
    /// | NumPy | Rust std-only |
    /// |-------|---------------|
    /// | `np.array([1.0, 2.0])` | `vec![1.0, 2.0]` |
    /// | `np.exp(a)` | `a.iter().map(\|x\| x.exp()).collect()` |
    /// | `np.sum(a)` | `a.iter().sum::<f64>()` |
    /// | `np.dot(a, b)` | `a.iter().zip(b.iter()).map(\|(x, y)\| x * y).sum::<f64>()` |
    fn try_convert_numpy_call_nasa_mode(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments to syn::Expr
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        let result: syn::Expr = match method {
            "array" => self.convert_np_array_nasa(args, &arg_exprs)?,
            "dot" => self.convert_np_dot_nasa(&arg_exprs)?,
            "sum" => self.convert_np_sum_nasa(&arg_exprs)?,
            "mean" => self.convert_np_mean_nasa(&arg_exprs)?,
            "sqrt" => self.convert_np_elemwise_nasa(args, &arg_exprs, "sqrt", "np.sqrt()")?,
            "abs" => self.convert_np_elemwise_nasa(args, &arg_exprs, "abs", "np.abs()")?,
            "min" | "amin" => {
                self.convert_np_fold_nasa(&arg_exprs, "f64::INFINITY", "f64::min", "np.min()")?
            }
            "max" | "amax" => {
                self.convert_np_fold_nasa(&arg_exprs, "f64::NEG_INFINITY", "f64::max", "np.max()")?
            }
            "exp" => self.convert_np_elemwise_nasa(args, &arg_exprs, "exp", "np.exp()")?,
            "log" => self.convert_np_elemwise_ln_nasa(args, &arg_exprs)?,
            "sin" => self.convert_np_elemwise_nasa(args, &arg_exprs, "sin", "np.sin()")?,
            "cos" => self.convert_np_elemwise_nasa(args, &arg_exprs, "cos", "np.cos()")?,
            "argmax" => self.convert_np_argmax_nasa(&arg_exprs)?,
            "argmin" => self.convert_np_argmin_nasa(&arg_exprs)?,
            "std" => self.convert_np_std_nasa(&arg_exprs)?,
            "var" => self.convert_np_var_nasa(&arg_exprs)?,
            "zeros" => self.convert_np_zeros_nasa(&arg_exprs)?,
            "ones" => self.convert_np_ones_nasa(&arg_exprs)?,
            "norm" => self.convert_np_norm_nasa(&arg_exprs)?,
            _ => return Ok(None),
        };

        Ok(Some(result))
    }

    fn convert_np_array_nasa(
        &mut self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if let Some(HirExpr::List(elements)) = args.first() {
            let element_exprs: Vec<syn::Expr> =
                elements.iter().map(|e| e.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;
            Ok(parse_quote! { vec![#(#element_exprs),*] })
        } else if let Some(arg) = arg_exprs.first() {
            Ok(parse_quote! { #arg.to_vec() })
        } else {
            Ok(parse_quote! { Vec::<f64>::new() })
        }
    }

    fn convert_np_dot_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if arg_exprs.len() >= 2 {
            let a = &arg_exprs[0];
            let b = &arg_exprs[1];
            Ok(parse_quote! {
                #a.iter().zip(#b.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum::<f64>()
            })
        } else {
            bail!("np.dot() requires 2 arguments");
        }
    }

    fn convert_np_sum_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! { #arr.iter().map(|&x| x as f64).sum::<f64>() })
        } else {
            bail!("np.sum() requires 1 argument");
        }
    }

    fn convert_np_mean_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {
                (#arr.iter().map(|&x| x as f64).sum::<f64>() / #arr.len() as f64)
            })
        } else {
            bail!("np.mean() requires 1 argument");
        }
    }

    fn convert_np_elemwise_nasa(
        &self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
        method_name: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("{} requires 1 argument", err_msg);
        }
        let arr = &arg_exprs[0];
        let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
        if self.is_numpy_array_expr(&args[0]) {
            Ok(parse_quote! { #arr.iter().map(|x| x.#method_ident()).collect::<Vec<_>>() })
        } else {
            Ok(parse_quote! { #arr.#method_ident() })
        }
    }

    fn convert_np_elemwise_ln_nasa(
        &self,
        args: &[HirExpr],
        arg_exprs: &[syn::Expr],
    ) -> Result<syn::Expr> {
        if args.is_empty() {
            bail!("np.log() requires 1 argument");
        }
        let arr = &arg_exprs[0];
        if self.is_numpy_array_expr(&args[0]) {
            Ok(parse_quote! { #arr.iter().map(|x| x.ln()).collect::<Vec<_>>() })
        } else {
            Ok(parse_quote! { #arr.ln() })
        }
    }

    fn convert_np_fold_nasa(
        &self,
        arg_exprs: &[syn::Expr],
        init: &str,
        op: &str,
        err_msg: &str,
    ) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            let init_expr: syn::Expr = syn::parse_str(init)?;
            let op_expr: syn::Expr = syn::parse_str(op)?;
            Ok(parse_quote! {
                #arr.iter().map(|&x| x as f64).fold(#init_expr, #op_expr)
            })
        } else {
            bail!("{} requires 1 argument", err_msg);
        }
    }

    fn convert_np_argmax_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {
                #arr.iter().enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("operation failed"))
                    .map(|(i, _)| i as i64)
                    .unwrap_or(0)
            })
        } else {
            bail!("np.argmax() requires 1 argument");
        }
    }

    fn convert_np_argmin_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {
                #arr.iter().enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("operation failed"))
                    .map(|(i, _)| i as i64)
                    .unwrap_or(0)
            })
        } else {
            bail!("np.argmin() requires 1 argument");
        }
    }

    fn convert_np_std_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {{
                let data: Vec<f64> = #arr.iter().map(|&x| x as f64).collect();
                let mean = data.iter().sum::<f64>() / data.len() as f64;
                let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
                variance.sqrt()
            }})
        } else {
            bail!("np.std() requires 1 argument");
        }
    }

    fn convert_np_var_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {{
                let data: Vec<f64> = #arr.iter().map(|&x| x as f64).collect();
                let mean = data.iter().sum::<f64>() / data.len() as f64;
                data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
            }})
        } else {
            bail!("np.var() requires 1 argument");
        }
    }

    fn convert_np_zeros_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(size) = arg_exprs.first() {
            Ok(parse_quote! { vec![0.0f64; #size as usize] })
        } else {
            bail!("np.zeros() requires 1 argument");
        }
    }

    fn convert_np_ones_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(size) = arg_exprs.first() {
            Ok(parse_quote! { vec![1.0f64; #size as usize] })
        } else {
            bail!("np.ones() requires 1 argument");
        }
    }

    fn convert_np_norm_nasa(&self, arg_exprs: &[syn::Expr]) -> Result<syn::Expr> {
        if let Some(arr) = arg_exprs.first() {
            Ok(parse_quote! {
                (#arr.iter().map(|&x| { let v = x as f64; v * v }).sum::<f64>()).sqrt()
            })
        } else {
            bail!("np.norm() requires 1 argument");
        }
    }
}
