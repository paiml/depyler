//! Stdlib cryptography and encoding method converters
//!
//! DEPYLER-REFACTOR: Extracted from expr_gen/mod.rs
//!
//! Contains converters for Python standard library modules related to
//! cryptography, encoding, and platform information:
//! - `base64` — Base64 and variants encoding/decoding
//! - `secrets` — Cryptographically strong random operations
//! - `hashlib` — Cryptographic hash functions (MD5, SHA family, BLAKE2)
//! - `uuid` — UUID generation (RFC 4122)
//! - `hmac` — HMAC authentication
//! - `platform` — System/platform information

use super::ExpressionConverter;
use crate::hir::*;
use crate::rust_gen::context::ToRustExpr;
use anyhow::{bail, Result};
use syn::parse_quote;

impl<'a, 'b> ExpressionConverter<'a, 'b> {
    /// Try to convert base64 module method calls
    /// DEPYLER-STDLIB-BASE64: Base64 and variants encoding/decoding
    ///
    /// Maps Python base64 module to Rust base64 crate:
    /// - base64.b64encode() → base64::encode()
    /// - base64.b64decode() → base64::decode()
    /// - base64.urlsafe_b64encode() → URL-safe encoding
    ///
    /// # Complexity
    /// 10 (match with 10 branches for different encodings)
    #[inline]
    pub(crate) fn try_convert_base64_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Mark that we need base64 crate
        self.ctx.needs_base64 = true;

        let result = match method {
            // Standard Base64
            "b64encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b64encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b64encode(data) → Vec<u8> (Python returns bytes)
                // DEPYLER-1003: Return Vec<u8> so .decode('utf-8') works with from_utf8_lossy
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.encode(#data).into_bytes()
                }
            }

            "b64decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b64decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b64decode(data) → base64::engine::general_purpose::STANDARD.decode(data).unwrap()
                parse_quote! {
                    base64::engine::general_purpose::STANDARD.decode(#data).expect("operation failed")
                }
            }

            // URL-safe Base64
            "urlsafe_b64encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.urlsafe_b64encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.urlsafe_b64encode(data) → Vec<u8> (Python returns bytes)
                // DEPYLER-1003: Return Vec<u8> so .decode('utf-8') works with from_utf8_lossy
                parse_quote! {
                    base64::engine::general_purpose::URL_SAFE.encode(#data).into_bytes()
                }
            }

            "urlsafe_b64decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.urlsafe_b64decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.urlsafe_b64decode(data) → base64::engine::general_purpose::URL_SAFE.decode(data).unwrap()
                parse_quote! {
                    base64::engine::general_purpose::URL_SAFE.decode(#data).expect("operation failed")
                }
            }

            // Base32 (note: base64 crate doesn't support base32, would need data-encoding crate)
            "b32encode" | "b32decode" => {
                // Simplified: note that full implementation needs data-encoding crate
                bail!("base64.{} requires data-encoding crate (not yet integrated)", method);
            }

            // Base16 (Hex)
            "b16encode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b16encode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b16encode(data) → hex::encode_upper(data)
                parse_quote! {
                    hex::encode_upper(#data)
                }
            }

            "b16decode" => {
                if arg_exprs.len() != 1 {
                    bail!("base64.b16decode() requires exactly 1 argument");
                }
                let data = &arg_exprs[0];

                // base64.b16decode(data) → hex::decode(data).unwrap()
                parse_quote! {
                    hex::decode(#data).expect("operation failed")
                }
            }

            // Base85 (also needs additional crate)
            "b85encode" | "b85decode" => {
                // Simplified: note that full implementation needs additional crate
                bail!("base64.{} requires base85 encoding crate (not yet integrated)", method);
            }

            _ => {
                bail!("base64.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert secrets module method calls
    /// DEPYLER-STDLIB-SECRETS: Cryptographically strong random operations
    ///
    /// Maps Python secrets module to Rust rand crate (cryptographic RNG):
    /// - secrets.randbelow() → rand::thread_rng().gen_range()
    /// - secrets.token_bytes() → Cryptographically secure random bytes
    ///
    /// # Complexity
    /// 5 (match with 5 branches)
    #[inline]
    pub(crate) fn try_convert_secrets_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Mark that we need rand crate (ThreadRng is cryptographically secure)
        self.ctx.needs_rand = true;
        self.ctx.needs_base64 = true; // For token_urlsafe

        let result = match method {
            // Random number generation
            "randbelow" => {
                if arg_exprs.len() != 1 {
                    bail!("secrets.randbelow() requires exactly 1 argument");
                }
                let n = &arg_exprs[0];

                // secrets.randbelow(n) → rand::thread_rng().gen_range(0..n)
                // DEPYLER-0656: Add use rand::Rng for gen_range method
                parse_quote! {
                    {
                        use rand::Rng;
                        rand::thread_rng().gen_range(0..#n)
                    }
                }
            }

            "choice" => {
                if arg_exprs.len() != 1 {
                    bail!("secrets.choice() requires exactly 1 argument");
                }
                let seq = &arg_exprs[0];

                // secrets.choice(seq) → seq.choose(&mut rand::thread_rng()).unwrap()
                // DEPYLER-0656: Add use rand::seq::SliceRandom for choose method
                parse_quote! {
                    {
                        use rand::seq::SliceRandom;
                        *#seq.choose(&mut rand::thread_rng()).expect("empty collection")
                    }
                }
            }

            // Token generation
            "token_bytes" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 } // Default 32 bytes
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_bytes(n) → generate n random bytes
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        bytes
                    }
                }
            }

            "token_hex" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 }
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_hex(n) → generate n random bytes and encode as hex
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        hex::encode(&bytes)
                    }
                }
            }

            "token_urlsafe" => {
                let nbytes = if arg_exprs.is_empty() {
                    parse_quote! { 32 }
                } else {
                    arg_exprs[0].clone()
                };

                // secrets.token_urlsafe(n) → generate n random bytes and encode as URL-safe base64
                parse_quote! {
                    {
                        let mut bytes = vec![0u8; #nbytes];
                        rand::thread_rng().fill(&mut bytes[..]);
                        base64::engine::general_purpose::URL_SAFE.encode(&bytes)
                    }
                }
            }

            _ => {
                bail!("secrets.{} not implemented yet", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert hashlib module method calls
    /// DEPYLER-STDLIB-HASHLIB: Cryptographic hash functions
    ///
    /// Supports: md5, sha1, sha224, sha256, sha384, sha512, blake2b, blake2s
    /// Returns hex digest directly (one-shot hashing pattern)
    ///
    /// # Complexity
    /// Cyclomatic: 9 (match with 8 algorithms + default)
    #[inline]
    pub(crate) fn try_convert_hashlib_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // All hash functions need hex encoding
        self.ctx.needs_hex = true;

        let result = match method {
            // MD5 hash
            // DEPYLER-0558: Support both one-shot and incremental patterns
            // Use Box<dyn DynDigest> for type-erased hasher objects
            "md5" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.md5() accepts 0 or 1 arguments");
                }
                self.ctx.needs_md5 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    // hashlib.md5() with no args → return boxed hasher for incremental use
                    parse_quote! {
                        {
                            use md5::Digest;
                            use digest::DynDigest;
                            Box::new(md5::Md5::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use md5::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(md5::Md5::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-1 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha1" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha1() accepts 0 or 1 arguments");
                }
                // DEPYLER-1001: Fix sha1 dependency - was incorrectly setting needs_sha2
                self.ctx.needs_sha1 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha1::Digest;
                            use digest::DynDigest;
                            Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha1::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha1::Sha1::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-224 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha224" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha224() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha224::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha224::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-256 hash
            // DEPYLER-0558: Support both one-shot and incremental patterns
            // Use Box<dyn DynDigest> for type-erased hasher objects
            // DEPYLER-1002: Always return hasher object, let .hexdigest() finalize
            "sha256" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha256() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    // hashlib.sha256() with no args → return boxed hasher for incremental use
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    // hashlib.sha256(data) → return hasher with data already updated
                    // The .hexdigest() method call will finalize and hex-encode
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-384 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha384" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha384() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha384::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // SHA-512 hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "sha512" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.sha512() accepts 0 or 1 arguments");
                }
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use sha2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(sha2::Sha512::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // BLAKE2b hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "blake2b" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.blake2b() accepts 0 or 1 arguments");
                }
                self.ctx.needs_blake2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            Box::new(blake2::Blake2b512::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(blake2::Blake2b512::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // BLAKE2s hash
            // DEPYLER-1002: Return hasher object, let .hexdigest() finalize
            "blake2s" => {
                if arg_exprs.len() > 1 {
                    bail!("hashlib.blake2s() accepts 0 or 1 arguments");
                }
                self.ctx.needs_blake2 = true;
                self.ctx.needs_digest = true;

                if arg_exprs.is_empty() {
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            Box::new(blake2::Blake2s256::new()) as Box<dyn DynDigest>
                        }
                    }
                } else {
                    let data = &arg_exprs[0];
                    parse_quote! {
                        {
                            use blake2::Digest;
                            use digest::DynDigest;
                            let mut hasher = Box::new(blake2::Blake2s256::new()) as Box<dyn DynDigest>;
                            hasher.update(#data);
                            hasher
                        }
                    }
                }
            }

            // DEPYLER-S12: hashlib.new(algo, data) → dynamic dispatch by algorithm name
            "new" => {
                if arg_exprs.is_empty() {
                    bail!("hashlib.new() requires algorithm name argument");
                }
                // Check if algorithm name is a string literal for static dispatch
                if let HirExpr::Literal(Literal::String(alg_name)) = &args[0] {
                    let alg_lower = alg_name.to_lowercase();
                    let data_args: Vec<HirExpr> =
                        if args.len() > 1 { args[1..].to_vec() } else { vec![] };
                    // Recursively dispatch to the correct algorithm handler
                    return self.try_convert_hashlib_method(&alg_lower, &data_args);
                }
                // Dynamic dispatch: algorithm not known at compile time, default to sha256
                self.ctx.needs_sha2 = true;
                self.ctx.needs_digest = true;
                parse_quote! {
                    {
                        use sha2::Digest;
                        use digest::DynDigest;
                        Box::new(sha2::Sha256::new()) as Box<dyn DynDigest>
                    }
                }
            }

            _ => {
                bail!("hashlib.{} not implemented yet (try: md5, sha1, sha224, sha256, sha384, sha512, blake2b, blake2s, new)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert uuid module method calls
    /// DEPYLER-STDLIB-UUID: UUID generation (RFC 4122)
    ///
    /// Supports: uuid1 (time-based), uuid4 (random)
    /// Returns string representation of UUID
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_uuid_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Mark that we need uuid crate
        self.ctx.needs_uuid = true;

        let result = match method {
            // UUID v1 - time-based
            "uuid1" => {
                if !arg_exprs.is_empty() {
                    bail!("uuid.uuid1() takes no arguments (node/clock_seq not yet supported)");
                }

                // uuid.uuid1() → Uuid::new_v1(...).to_string()
                // Note: Requires context (timestamp + node ID)
                parse_quote! {
                    {
                        use uuid::Uuid;
                        // Generate time-based UUID v1
                        // Note: Using placeholder implementation (actual v1 needs timestamp context)
                        Uuid::new_v4().to_string()  // NOTE: Implement proper UUID v1 with timestamp (tracked in DEPYLER-0424)
                    }
                }
            }

            // UUID v4 - random (most common)
            "uuid4" => {
                if !arg_exprs.is_empty() {
                    bail!("uuid.uuid4() takes no arguments");
                }

                // uuid.uuid4() → Uuid::new_v4().to_string()
                parse_quote! {
                    {
                        use uuid::Uuid;
                        Uuid::new_v4().to_string()
                    }
                }
            }

            _ => {
                bail!("uuid.{} not implemented yet (try: uuid1, uuid4)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert hmac module method calls
    /// DEPYLER-STDLIB-HMAC: HMAC authentication
    ///
    /// Supports: new() with SHA256, compare_digest()
    /// Returns hex digest for one-shot HMAC
    ///
    /// # Complexity
    /// Cyclomatic: 3 (match with 2 functions + default)
    #[inline]
    pub(crate) fn try_convert_hmac_method(
        &mut self,
        method: &str,
        args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        // Convert arguments first
        let arg_exprs: Vec<syn::Expr> =
            args.iter().map(|arg| arg.to_rust_expr(self.ctx)).collect::<Result<Vec<_>>>()?;

        // Mark that we need hmac and related crates
        self.ctx.needs_hmac = true;
        self.ctx.needs_sha2 = true; // For SHA256
        self.ctx.needs_hex = true;

        let result = match method {
            // HMAC creation - simplified to SHA256
            "new" => {
                if arg_exprs.len() < 2 {
                    bail!("hmac.new() requires at least 2 arguments (key, message)");
                }
                let key = &arg_exprs[0];
                let msg = &arg_exprs[1];

                // NOTE: Parse digestmod argument (arg_exprs[2]) to support multiple HMAC algorithms (tracked in DEPYLER-0424)
                // For now, hardcode SHA256 as most common

                // hmac.new(key, msg, hashlib.sha256) → HMAC-SHA256 hex digest
                parse_quote! {
                    {
                        use hmac::{Hmac, Mac};
                        use sha2::Sha256;

                        type HmacSha256 = Hmac<Sha256>;
                        let mut mac = HmacSha256::new_from_slice(#key).expect("HMAC key error");
                        mac.update(#msg);
                        hex::encode(mac.finalize().into_bytes())
                    }
                }
            }

            // Timing-safe comparison
            "compare_digest" => {
                if arg_exprs.len() != 2 {
                    bail!("hmac.compare_digest() requires exactly 2 arguments");
                }
                let a = &arg_exprs[0];
                let b = &arg_exprs[1];

                // hmac.compare_digest(a, b) → constant-time comparison
                parse_quote! {
                    {
                        use subtle::ConstantTimeEq;
                        #a.ct_eq(#b).into()
                    }
                }
            }

            _ => {
                bail!("hmac.{} not implemented yet (try: new, compare_digest)", method);
            }
        };

        Ok(Some(result))
    }

    /// Try to convert platform module method calls
    /// DEPYLER-0430: platform module - system information
    ///
    /// Maps Python platform module to Rust std::env::consts:
    /// - platform.system() → std::env::consts::OS
    /// - platform.machine() → std::env::consts::ARCH
    /// - platform.python_version() → "3.11.0" (hardcoded constant)
    ///
    /// # Complexity
    /// ≤10 (simple match with few branches)
    #[inline]
    pub(crate) fn try_convert_platform_method(
        &mut self,
        method: &str,
        _args: &[HirExpr],
    ) -> Result<Option<syn::Expr>> {
        let result = match method {
            "system" => {
                // platform.system() → std::env::consts::OS
                // Returns "linux", "macos", "windows", etc.
                parse_quote! { std::env::consts::OS.to_string() }
            }

            "machine" => {
                // platform.machine() → std::env::consts::ARCH
                // Returns "x86_64", "aarch64", etc.
                parse_quote! { std::env::consts::ARCH.to_string() }
            }

            "python_version" => {
                // platform.python_version() → "3.11.0"
                // Hardcoded to Python 3.11 for compatibility
                parse_quote! { "3.11.0".to_string() }
            }

            "release" => {
                // platform.release() → OS release version
                // Note: This is OS-specific and may require additional logic
                parse_quote! { std::env::consts::OS.to_string() }
            }

            _ => {
                bail!(
                    "platform.{} not implemented yet (try: system, machine, python_version, release)",
                    method
                );
            }
        };

        Ok(Some(result))
    }
}
