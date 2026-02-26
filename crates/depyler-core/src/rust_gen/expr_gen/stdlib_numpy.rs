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
            "array" => {
                // np.array([1.0, 2.0, 3.0]) → Vector::from_slice(&[1.0f32, 2.0, 3.0])
                // The argument should be a list literal
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
                    return Ok(Some(syn::parse2(tokens)?));
                }
                // Fallback: pass through as vec!
                if let Some(arg) = arg_exprs.first() {
                    parse_quote! { Vector::from_vec(#arg) }
                } else {
                    parse_quote! { Vector::new() }
                }
            }
            "dot" => {
                // np.dot(a, b) → a.dot(&b).unwrap()
                if arg_exprs.len() >= 2 {
                    let a = &arg_exprs[0];
                    let b = &arg_exprs[1];
                    parse_quote! { #a.dot(&#b).expect("dot product failed") }
                } else {
                    bail!("np.dot() requires 2 arguments");
                }
            }
            "sum" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.sum().expect("operation failed") }
                } else {
                    bail!("np.sum() requires 1 argument");
                }
            }
            "mean" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.mean().expect("operation failed") }
                } else {
                    bail!("np.mean() requires 1 argument");
                }
            }
            // DEPYLER-0657: Scalar vs Vector numpy methods
            // f64::sqrt()/abs() returns f64 directly (no Result)
            // Vector::sqrt()/abs() returns Result (needs unwrap)
            "sqrt" => {
                if args.is_empty() {
                    bail!("np.sqrt() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.sqrt().expect("operation failed") }
                } else {
                    parse_quote! { #arr.sqrt() }
                }
            }
            "abs" => {
                if args.is_empty() {
                    bail!("np.abs() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.abs().expect("operation failed") }
                } else {
                    // f64 uses .abs() directly
                    parse_quote! { #arr.abs() }
                }
            }
            "min" | "amin" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.min().expect("empty collection") }
                } else {
                    bail!("np.min() requires 1 argument");
                }
            }
            "max" | "amax" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.max().expect("empty collection") }
                } else {
                    bail!("np.max() requires 1 argument");
                }
            }
            // DEPYLER-0657: exp/log/sin/cos scalar vs vector
            "exp" => {
                if args.is_empty() {
                    bail!("np.exp() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.exp().expect("operation failed") }
                } else {
                    parse_quote! { #arr.exp() }
                }
            }
            "log" => {
                if args.is_empty() {
                    bail!("np.log() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.ln().expect("operation failed") }
                } else {
                    // f64 uses .ln() for natural log
                    parse_quote! { #arr.ln() }
                }
            }
            "sin" => {
                if args.is_empty() {
                    bail!("np.sin() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.sin().expect("operation failed") }
                } else {
                    // f64::sin() returns f64 directly
                    parse_quote! { #arr.sin() }
                }
            }
            "cos" => {
                if args.is_empty() {
                    bail!("np.cos() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.cos().expect("operation failed") }
                } else {
                    // f64::cos() returns f64 directly
                    parse_quote! { #arr.cos() }
                }
            }
            "clip" => {
                // DEPYLER-0920: Cast min/max to f32 for trueno::Vector::clamp compatibility
                if arg_exprs.len() >= 3 {
                    let arr = &arg_exprs[0];
                    let min = &arg_exprs[1];
                    let max = &arg_exprs[2];
                    parse_quote! { #arr.clamp(#min as f32, #max as f32).expect("operation failed") }
                } else {
                    bail!("np.clip() requires 3 arguments (array, min, max)");
                }
            }
            "argmax" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.argmax().expect("empty collection") }
                } else {
                    bail!("np.argmax() requires 1 argument");
                }
            }
            "argmin" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.argmin().expect("empty collection") }
                } else {
                    bail!("np.argmin() requires 1 argument");
                }
            }
            "std" => {
                // trueno uses stddev(), not std()
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.stddev().expect("operation failed") }
                } else {
                    bail!("np.std() requires 1 argument");
                }
            }
            "var" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.variance().expect("operation failed") }
                } else {
                    bail!("np.var() requires 1 argument");
                }
            }
            "zeros" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { Vector::zeros(#size) }
                } else {
                    bail!("np.zeros() requires 1 argument");
                }
            }
            "ones" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { Vector::ones(#size) }
                } else {
                    bail!("np.ones() requires 1 argument");
                }
            }
            "norm" => {
                if let Some(arr) = arg_exprs.first() {
                    // DEPYLER-0583: trueno uses norm_l2() for L2 (Euclidean) norm
                    // DEPYLER-0667: Wrap arg in parens so `a - b` becomes `(a - b).norm_l2()`
                    // Without parens, `a - b.norm_l2()` parses as `a - (b.norm_l2())`
                    parse_quote! { (#arr).norm_l2().expect("operation failed") }
                } else {
                    bail!("np.norm() requires 1 argument");
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(result))
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
            "array" => {
                // np.array([1.0, 2.0, 3.0]) → vec![1.0, 2.0, 3.0]
                if let Some(HirExpr::List(elements)) = args.first() {
                    let element_exprs: Vec<syn::Expr> = elements
                        .iter()
                        .map(|e| e.to_rust_expr(self.ctx))
                        .collect::<Result<Vec<_>>>()?;
                    parse_quote! { vec![#(#element_exprs),*] }
                } else if let Some(arg) = arg_exprs.first() {
                    // Fallback: pass through as vec!
                    parse_quote! { #arg.to_vec() }
                } else {
                    parse_quote! { Vec::<f64>::new() }
                }
            }
            "dot" => {
                // DEPYLER-1135: np.dot(a, b) → numeric coercion for mixed int/float arrays
                // Convert elements to f64 before multiplication to handle integer arrays
                if arg_exprs.len() >= 2 {
                    let a = &arg_exprs[0];
                    let b = &arg_exprs[1];
                    parse_quote! {
                        #a.iter().zip(#b.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum::<f64>()
                    }
                } else {
                    bail!("np.dot() requires 2 arguments");
                }
            }
            "sum" => {
                // DEPYLER-1135: np.sum(a) → coerce elements to f64 for universal numeric promotion
                // This handles both Vec<i32> and Vec<f64> by converting to f64
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! { #arr.iter().map(|&x| x as f64).sum::<f64>() }
                } else {
                    bail!("np.sum() requires 1 argument");
                }
            }
            "mean" => {
                // DEPYLER-1135: np.mean(a) → coerce elements to f64 for proper averaging
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        (#arr.iter().map(|&x| x as f64).sum::<f64>() / #arr.len() as f64)
                    }
                } else {
                    bail!("np.mean() requires 1 argument");
                }
            }
            "sqrt" => {
                if args.is_empty() {
                    bail!("np.sqrt() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    // Vector: map sqrt over elements
                    parse_quote! { #arr.iter().map(|x| x.sqrt()).collect::<Vec<_>>() }
                } else {
                    // Scalar
                    parse_quote! { #arr.sqrt() }
                }
            }
            "abs" => {
                if args.is_empty() {
                    bail!("np.abs() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.iter().map(|x| x.abs()).collect::<Vec<_>>() }
                } else {
                    parse_quote! { #arr.abs() }
                }
            }
            "min" | "amin" => {
                // DEPYLER-1135: Coerce to f64 for numeric promotion
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        #arr.iter().map(|&x| x as f64).fold(f64::INFINITY, f64::min)
                    }
                } else {
                    bail!("np.min() requires 1 argument");
                }
            }
            "max" | "amax" => {
                // DEPYLER-1135: Coerce to f64 for numeric promotion
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        #arr.iter().map(|&x| x as f64).fold(f64::NEG_INFINITY, f64::max)
                    }
                } else {
                    bail!("np.max() requires 1 argument");
                }
            }
            "exp" => {
                if args.is_empty() {
                    bail!("np.exp() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.iter().map(|x| x.exp()).collect::<Vec<_>>() }
                } else {
                    parse_quote! { #arr.exp() }
                }
            }
            "log" => {
                if args.is_empty() {
                    bail!("np.log() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.iter().map(|x| x.ln()).collect::<Vec<_>>() }
                } else {
                    parse_quote! { #arr.ln() }
                }
            }
            "sin" => {
                if args.is_empty() {
                    bail!("np.sin() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.iter().map(|x| x.sin()).collect::<Vec<_>>() }
                } else {
                    parse_quote! { #arr.sin() }
                }
            }
            "cos" => {
                if args.is_empty() {
                    bail!("np.cos() requires 1 argument");
                }
                let arr = &arg_exprs[0];
                if self.is_numpy_array_expr(&args[0]) {
                    parse_quote! { #arr.iter().map(|x| x.cos()).collect::<Vec<_>>() }
                } else {
                    parse_quote! { #arr.cos() }
                }
            }
            "argmax" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        #arr.iter().enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("operation failed"))
                            .map(|(i, _)| i as i64)
                            .unwrap_or(0)
                    }
                } else {
                    bail!("np.argmax() requires 1 argument");
                }
            }
            "argmin" => {
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        #arr.iter().enumerate()
                            .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("operation failed"))
                            .map(|(i, _)| i as i64)
                            .unwrap_or(0)
                    }
                } else {
                    bail!("np.argmin() requires 1 argument");
                }
            }
            "std" => {
                // DEPYLER-1135: std = sqrt(variance) with numeric coercion
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {{
                        let data: Vec<f64> = #arr.iter().map(|&x| x as f64).collect();
                        let mean = data.iter().sum::<f64>() / data.len() as f64;
                        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
                        variance.sqrt()
                    }}
                } else {
                    bail!("np.std() requires 1 argument");
                }
            }
            "var" => {
                // DEPYLER-1135: variance with numeric coercion
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {{
                        let data: Vec<f64> = #arr.iter().map(|&x| x as f64).collect();
                        let mean = data.iter().sum::<f64>() / data.len() as f64;
                        data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
                    }}
                } else {
                    bail!("np.var() requires 1 argument");
                }
            }
            "zeros" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { vec![0.0f64; #size as usize] }
                } else {
                    bail!("np.zeros() requires 1 argument");
                }
            }
            "ones" => {
                if let Some(size) = arg_exprs.first() {
                    parse_quote! { vec![1.0f64; #size as usize] }
                } else {
                    bail!("np.ones() requires 1 argument");
                }
            }
            "norm" => {
                // DEPYLER-1135: L2 norm with numeric coercion
                if let Some(arr) = arg_exprs.first() {
                    parse_quote! {
                        (#arr.iter().map(|&x| { let v = x as f64; v * v }).sum::<f64>()).sqrt()
                    }
                } else {
                    bail!("np.norm() requires 1 argument");
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(result))
    }
}
