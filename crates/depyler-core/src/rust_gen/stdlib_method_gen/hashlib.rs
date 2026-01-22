//! Hashlib Module Code Generation - EXTREME TDD
//!
//! Handles Python `hashlib` module conversions to Rust digest crates.
//! Single-shot compilation: No manual lifetime annotations needed.
//!
//! Coverage target: 100% line coverage, 100% branch coverage
//!
//! ## Supported Hash Algorithms
//! - `hashlib.md5()` → md5 crate
//! - `hashlib.sha1()` → sha1 crate
//! - `hashlib.sha224()` → sha2 crate (Sha224)
//! - `hashlib.sha256()` → sha2 crate (Sha256)
//! - `hashlib.sha384()` → sha2 crate (Sha384)
//! - `hashlib.sha512()` → sha2 crate (Sha512)
//! - `hashlib.sha3_256()` → sha3 crate
//! - `hashlib.sha3_512()` → sha3 crate
//! - `hashlib.blake2b()` → blake2 crate
//! - `hashlib.blake2s()` → blake2 crate
//! - `hashlib.new(name)` → dynamic hash selection
//!
//! ## Design Principles
//! 1. Use Digest trait for unified interface
//! 2. Strategic Cloning: Clone hasher when needed for multiple updates
//! 3. Hex encoding via hex crate for hexdigest()

use crate::hir::{HirExpr, Literal};
use crate::rust_gen::context::{CodeGenContext, ToRustExpr};
use anyhow::{bail, Result};
use syn::parse_quote;

/// Convert Python hashlib module function calls to Rust
///
/// # Supported Constructors
/// - `hashlib.md5()` / `hashlib.md5(data)`
/// - `hashlib.sha1()` / `hashlib.sha1(data)`
/// - `hashlib.sha224()` / `hashlib.sha224(data)`
/// - `hashlib.sha256()` / `hashlib.sha256(data)`
/// - `hashlib.sha384()` / `hashlib.sha384(data)`
/// - `hashlib.sha512()` / `hashlib.sha512(data)`
/// - `hashlib.sha3_256()` / `hashlib.sha3_256(data)`
/// - `hashlib.sha3_512()` / `hashlib.sha3_512(data)`
/// - `hashlib.blake2b()` / `hashlib.blake2b(data)`
/// - `hashlib.blake2s()` / `hashlib.blake2s(data)`
/// - `hashlib.new(name, data)`
///
/// # Complexity: 8 (within limits)
pub fn convert_hashlib_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    // Convert arguments
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "md5" => convert_md5(&arg_exprs, ctx)?,
        "sha1" => convert_sha1(&arg_exprs, ctx)?,
        "sha224" => convert_sha224(&arg_exprs, ctx)?,
        "sha256" => convert_sha256(&arg_exprs, ctx)?,
        "sha384" => convert_sha384(&arg_exprs, ctx)?,
        "sha512" => convert_sha512(&arg_exprs, ctx)?,
        "sha3_224" => convert_sha3_224(&arg_exprs, ctx)?,
        "sha3_256" => convert_sha3_256(&arg_exprs, ctx)?,
        "sha3_384" => convert_sha3_384(&arg_exprs, ctx)?,
        "sha3_512" => convert_sha3_512(&arg_exprs, ctx)?,
        "blake2b" => convert_blake2b(&arg_exprs, ctx)?,
        "blake2s" => convert_blake2s(&arg_exprs, ctx)?,
        "new" => convert_new(args, &arg_exprs, ctx)?,
        _ => bail!("hashlib.{} not implemented yet", method),
    };

    Ok(Some(result))
}

/// Convert hashlib hash object instance methods
///
/// # Supported Methods
/// - `.update(data)` → hasher.update(data)
/// - `.digest()` → hasher.finalize()
/// - `.hexdigest()` → hex::encode(hasher.finalize())
/// - `.copy()` → hasher.clone()
pub fn convert_hashlib_instance_method(
    method: &str,
    args: &[HirExpr],
    ctx: &mut CodeGenContext,
) -> Result<Option<syn::Expr>> {
    let arg_exprs: Vec<syn::Expr> = args
        .iter()
        .map(|arg| arg.to_rust_expr(ctx))
        .collect::<Result<Vec<_>>>()?;

    let result = match method {
        "update" => convert_update(&arg_exprs, ctx)?,
        "digest" => convert_digest(ctx)?,
        "hexdigest" => convert_hexdigest(ctx)?,
        "copy" => convert_copy()?,
        "name" => convert_name()?,
        "digest_size" => convert_digest_size()?,
        "block_size" => convert_block_size()?,
        _ => bail!("hashlib hash object method '{}' not implemented", method),
    };

    Ok(Some(result))
}

// =============================================================================
// Hash Algorithm Constructors
// =============================================================================

fn convert_md5(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_md5 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { md5::Md5::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use md5::Digest;
                let mut hasher = md5::Md5::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha1(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha1 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha1::Sha1::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha1::Digest;
                let mut hasher = sha1::Sha1::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha224(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha2::Sha224::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha2::Digest;
                let mut hasher = sha2::Sha224::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha256(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha2::Sha256::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha2::Digest;
                let mut hasher = sha2::Sha256::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha384(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha2::Sha384::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha2::Digest;
                let mut hasher = sha2::Sha384::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha512(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha2::Sha512::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha2::Digest;
                let mut hasher = sha2::Sha512::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha3_224(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha3 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha3::Sha3_224::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha3::Digest;
                let mut hasher = sha3::Sha3_224::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha3_256(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha3 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha3::Sha3_256::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha3::Digest;
                let mut hasher = sha3::Sha3_256::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha3_384(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha3 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha3::Sha3_384::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha3::Digest;
                let mut hasher = sha3::Sha3_384::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_sha3_512(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_sha3 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { sha3::Sha3_512::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use sha3::Digest;
                let mut hasher = sha3::Sha3_512::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_blake2b(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_blake2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { blake2::Blake2b512::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use blake2::Digest;
                let mut hasher = blake2::Blake2b512::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

fn convert_blake2s(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_blake2 = true;
    ctx.needs_digest = true;

    if arg_exprs.is_empty() {
        Ok(parse_quote! { blake2::Blake2s256::new() })
    } else {
        let data = &arg_exprs[0];
        Ok(parse_quote! {
            {
                use blake2::Digest;
                let mut hasher = blake2::Blake2s256::new();
                hasher.update(#data.as_bytes());
                hasher
            }
        })
    }
}

/// hashlib.new(name, data=b'') - Create hasher by name
fn convert_new(
    args: &[HirExpr],
    arg_exprs: &[syn::Expr],
    ctx: &mut CodeGenContext,
) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("hashlib.new() requires algorithm name argument");
    }

    // Check if we can determine the algorithm at compile time
    if let Some(HirExpr::Literal(Literal::String(alg_name))) = args.first() {
        // Static dispatch based on algorithm name
        let alg_lower = alg_name.to_lowercase();
        let data_expr = arg_exprs.get(1);

        match alg_lower.as_str() {
            "md5" => {
                if let Some(data) = data_expr {
                    convert_md5(std::slice::from_ref(data), ctx)
                } else {
                    convert_md5(&[], ctx)
                }
            }
            "sha1" => {
                if let Some(data) = data_expr {
                    convert_sha1(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha1(&[], ctx)
                }
            }
            "sha224" => {
                if let Some(data) = data_expr {
                    convert_sha224(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha224(&[], ctx)
                }
            }
            "sha256" => {
                if let Some(data) = data_expr {
                    convert_sha256(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha256(&[], ctx)
                }
            }
            "sha384" => {
                if let Some(data) = data_expr {
                    convert_sha384(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha384(&[], ctx)
                }
            }
            "sha512" => {
                if let Some(data) = data_expr {
                    convert_sha512(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha512(&[], ctx)
                }
            }
            "sha3_256" | "sha3-256" => {
                if let Some(data) = data_expr {
                    convert_sha3_256(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha3_256(&[], ctx)
                }
            }
            "sha3_512" | "sha3-512" => {
                if let Some(data) = data_expr {
                    convert_sha3_512(std::slice::from_ref(data), ctx)
                } else {
                    convert_sha3_512(&[], ctx)
                }
            }
            "blake2b" => {
                if let Some(data) = data_expr {
                    convert_blake2b(std::slice::from_ref(data), ctx)
                } else {
                    convert_blake2b(&[], ctx)
                }
            }
            "blake2s" => {
                if let Some(data) = data_expr {
                    convert_blake2s(std::slice::from_ref(data), ctx)
                } else {
                    convert_blake2s(&[], ctx)
                }
            }
            _ => bail!("hashlib.new(): unsupported algorithm '{}'", alg_name),
        }
    } else {
        // Dynamic dispatch - use Box<dyn DynDigest> pattern
        ctx.needs_digest = true;
        ctx.needs_sha2 = true;
        ctx.needs_md5 = true;
        ctx.needs_sha1 = true;

        let name = &arg_exprs[0];
        Ok(parse_quote! {
            depyler_runtime::create_hasher(#name)?
        })
    }
}

// =============================================================================
// Instance Methods
// =============================================================================

fn convert_update(arg_exprs: &[syn::Expr], ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    if arg_exprs.is_empty() {
        bail!("hash.update() requires data argument");
    }
    ctx.needs_digest = true;

    let data = &arg_exprs[0];
    // Note: The actual object reference is handled by the caller
    // This generates the method call portion
    Ok(parse_quote! { self.update(#data.as_bytes()) })
}

fn convert_digest(ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_digest = true;
    // finalize() consumes self, so we need clone() for Python semantics
    Ok(parse_quote! { self.clone().finalize().to_vec() })
}

fn convert_hexdigest(ctx: &mut CodeGenContext) -> Result<syn::Expr> {
    ctx.needs_digest = true;
    ctx.needs_hex = true;
    // finalize() consumes self, so we need clone() for Python semantics
    Ok(parse_quote! { hex::encode(self.clone().finalize()) })
}

fn convert_copy() -> Result<syn::Expr> {
    Ok(parse_quote! { self.clone() })
}

fn convert_name() -> Result<syn::Expr> {
    // Would need runtime type info; provide static name where possible
    Ok(parse_quote! { self.name() })
}

fn convert_digest_size() -> Result<syn::Expr> {
    Ok(parse_quote! { self.output_size() })
}

fn convert_block_size() -> Result<syn::Expr> {
    Ok(parse_quote! { self.block_size() })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_md5(&[], &mut ctx).unwrap();
        assert!(ctx.needs_md5);
        assert!(ctx.needs_digest);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Md5"));
    }

    #[test]
    fn test_md5_with_data() {
        let mut ctx = CodeGenContext::default();
        let data: syn::Expr = parse_quote! { "hello" };
        let result = convert_md5(&[data], &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("update"));
    }

    #[test]
    fn test_sha1_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_sha1(&[], &mut ctx).unwrap();
        assert!(ctx.needs_sha1);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Sha1"));
    }

    #[test]
    fn test_sha256_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_sha256(&[], &mut ctx).unwrap();
        assert!(ctx.needs_sha2);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Sha256"));
    }

    #[test]
    fn test_sha256_with_data() {
        let mut ctx = CodeGenContext::default();
        let data: syn::Expr = parse_quote! { "test" };
        let result = convert_sha256(&[data], &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("update"));
    }

    #[test]
    fn test_sha512_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_sha512(&[], &mut ctx).unwrap();
        assert!(ctx.needs_sha2);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Sha512"));
    }

    #[test]
    fn test_sha3_256_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_sha3_256(&[], &mut ctx).unwrap();
        assert!(ctx.needs_sha3);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Sha3_256"));
    }

    #[test]
    fn test_blake2b_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_blake2b(&[], &mut ctx).unwrap();
        assert!(ctx.needs_blake2);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Blake2b512"));
    }

    #[test]
    fn test_blake2s_no_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_blake2s(&[], &mut ctx).unwrap();
        assert!(ctx.needs_blake2);
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("Blake2s256"));
    }

    #[test]
    fn test_hexdigest() {
        let mut ctx = CodeGenContext::default();
        let result = convert_hexdigest(&mut ctx).unwrap();
        assert!(ctx.needs_hex);
        let code = quote::quote!(#result).to_string();
        // quote! may produce "hex :: encode" with spaces
        assert!(code.contains("hex") && code.contains("encode"));
    }

    #[test]
    fn test_digest() {
        let mut ctx = CodeGenContext::default();
        let result = convert_digest(&mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("finalize"));
    }

    #[test]
    fn test_copy() {
        let result = convert_copy().unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("clone"));
    }

    #[test]
    fn test_update_requires_data() {
        let mut ctx = CodeGenContext::default();
        let result = convert_update(&[], &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_with_data() {
        let mut ctx = CodeGenContext::default();
        let data: syn::Expr = parse_quote! { "data" };
        let result = convert_update(&[data], &mut ctx).unwrap();
        let code = quote::quote!(#result).to_string();
        assert!(code.contains("update"));
    }

    #[test]
    fn test_convert_hashlib_method_dispatch() {
        let mut ctx = CodeGenContext::default();
        let result = convert_hashlib_method("sha256", &[], &mut ctx).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_convert_hashlib_method_unsupported() {
        let mut ctx = CodeGenContext::default();
        let result = convert_hashlib_method("unsupported", &[], &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_requires_name() {
        let mut ctx = CodeGenContext::default();
        let result = convert_new(&[], &[], &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_known_algorithm() {
        let mut ctx = CodeGenContext::default();
        let name_hir = HirExpr::Literal(Literal::String("sha256".to_string()));
        let name_expr: syn::Expr = parse_quote! { "sha256" };
        let _result = convert_new(&[name_hir], &[name_expr], &mut ctx).unwrap();
        assert!(ctx.needs_sha2);
    }

    #[test]
    fn test_new_unsupported_algorithm() {
        let mut ctx = CodeGenContext::default();
        let name_hir = HirExpr::Literal(Literal::String("unsupported".to_string()));
        let name_expr: syn::Expr = parse_quote! { "unsupported" };
        let result = convert_new(&[name_hir], &[name_expr], &mut ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_sha2_variants() {
        let mut ctx = CodeGenContext::default();

        let _ = convert_sha224(&[], &mut ctx).unwrap();
        let _ = convert_sha256(&[], &mut ctx).unwrap();
        let _ = convert_sha384(&[], &mut ctx).unwrap();
        let _ = convert_sha512(&[], &mut ctx).unwrap();

        assert!(ctx.needs_sha2);
    }

    #[test]
    fn test_all_sha3_variants() {
        let mut ctx = CodeGenContext::default();

        let _ = convert_sha3_224(&[], &mut ctx).unwrap();
        let _ = convert_sha3_256(&[], &mut ctx).unwrap();
        let _ = convert_sha3_384(&[], &mut ctx).unwrap();
        let _ = convert_sha3_512(&[], &mut ctx).unwrap();

        assert!(ctx.needs_sha3);
    }

    #[test]
    fn test_instance_method_dispatch() {
        let mut ctx = CodeGenContext::default();
        let result = convert_hashlib_instance_method("hexdigest", &[], &mut ctx).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_instance_method_unsupported() {
        let mut ctx = CodeGenContext::default();
        let result = convert_hashlib_instance_method("unsupported", &[], &mut ctx);
        assert!(result.is_err());
    }
}
