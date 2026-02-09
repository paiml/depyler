//! Wave 19 deep coverage tests: direct_rules_convert/expr_methods deep paths
//!
//! Targets uncovered code paths in:
//! - stdlib_method_gen/hashlib.rs: hash constructors with data, hexdigest, digest, update chains
//! - expr_gen/stdlib_crypto.rs: base64, hashlib with data param via ExpressionConverter
//! - stdlib_method_gen/regex_mod.rs: re.sub, subn, findall, search, match, compile, split, escape
//! - expr_gen/stdlib_os.rs: os.path.join, exists, basename, dirname, isfile, isdir
//! - stdlib_method_gen/os.rs: os.getenv, getcwd, mkdir, makedirs, rmdir, rename, walk
//! - stdlib_method_gen/time.rs: time.time, sleep, monotonic, ctime, gmtime, localtime
//! - expr_gen/stdlib_misc.rs: sys.exit, colorsys conversions, statistics, bisect, heapq
//! - expr_gen_instance_methods/sys_io_methods.rs: sys.stdout/stderr/stdin methods
//!
//! 200 transpile-based tests exercising end-to-end codegen paths

#[cfg(test)]
mod tests {
    use crate::ast_bridge::AstBridge;
    use crate::rust_gen::generate_rust_file;
    use crate::type_mapper::TypeMapper;
    use rustpython_parser::{parse, Mode};

    #[allow(unused_variables)]
    fn transpile(python_code: &str) -> String {
        let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
        let (module, _) = AstBridge::new()
            .with_source(python_code.to_string())
            .python_to_hir(ast)
            .expect("hir");
        let tm = TypeMapper::default();
        let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
        result
    }

    // ========================================================================
    // SECTION 1: HASHLIB WITH DATA PARAMETER (tests 001-050)
    // ========================================================================

    #[test]
    fn test_w19dr_001_hashlib_md5_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.md5(b\"hello\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_002_hashlib_sha1_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha1(b\"test\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_003_hashlib_sha256_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha256(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_004_hashlib_sha512_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha512(b\"x\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_005_hashlib_sha384_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha384(b\"hello\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_006_hashlib_blake2b_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2b(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_007_hashlib_blake2s_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2s(b\"msg\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_008_hashlib_new_sha256_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"sha256\", b\"test\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_009_hashlib_md5_no_data_hexdigest() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.md5()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_010_hashlib_sha256_no_data_hexdigest() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha256()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_011_hashlib_sha1_no_data_hexdigest() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha1()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_012_hashlib_sha512_no_data_hexdigest() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha512()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_013_hashlib_sha224_bytes_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha224(b\"abc\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_014_hashlib_md5_string_encode() {
        let code = "import hashlib\ndef hash_str(s: str) -> str:\n    h = hashlib.md5(s.encode())\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_015_hashlib_sha256_string_encode() {
        let code = "import hashlib\ndef hash_str(s: str) -> str:\n    h = hashlib.sha256(s.encode())\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_016_hashlib_sha256_digest() {
        let code = "import hashlib\ndef get_digest() -> bytes:\n    h = hashlib.sha256(b\"data\")\n    return h.digest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_017_hashlib_md5_digest() {
        let code = "import hashlib\ndef get_digest() -> bytes:\n    h = hashlib.md5(b\"data\")\n    return h.digest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_018_hashlib_sha384_no_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha384()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_019_hashlib_blake2b_no_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2b()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_020_hashlib_blake2s_no_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2s()\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_021_hashlib_new_md5_no_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"md5\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_022_hashlib_new_sha1_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"sha1\", b\"test\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_023_hashlib_new_sha384_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"sha384\", b\"x\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_024_hashlib_new_sha512_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"sha512\", b\"y\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_025_hashlib_new_blake2b_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"blake2b\", b\"z\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_026_hashlib_new_blake2s_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"blake2s\", b\"w\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_027_hashlib_sha256_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_028_hashlib_md5_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.md5(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_029_hashlib_sha1_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.sha1(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_030_hashlib_sha224_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.sha224(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_031_hashlib_sha384_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.sha384(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_032_hashlib_sha512_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.sha512(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_033_hashlib_blake2b_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.blake2b(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_034_hashlib_blake2s_in_func_return() {
        let code = "import hashlib\ndef compute_hash(data: bytes) -> str:\n    return hashlib.blake2s(data).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_035_hashlib_md5_assigned_then_hexdigest() {
        let code = "import hashlib\ndef hash_it(msg: bytes) -> str:\n    hasher = hashlib.md5(msg)\n    result = hasher.hexdigest()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_036_hashlib_sha256_assigned_then_hexdigest() {
        let code = "import hashlib\ndef hash_it(msg: bytes) -> str:\n    hasher = hashlib.sha256(msg)\n    result = hasher.hexdigest()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_037_hashlib_sha512_assigned_then_digest() {
        let code = "import hashlib\ndef hash_it(msg: bytes) -> bytes:\n    hasher = hashlib.sha512(msg)\n    result = hasher.digest()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_038_hashlib_new_sha224_data() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"sha224\", b\"ab\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_039_hashlib_sha256_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha256()\n    h.update(b\"part1\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_040_hashlib_md5_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.md5()\n    h.update(b\"part1\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_041_hashlib_sha1_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha1()\n    h.update(b\"chunk\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_042_hashlib_sha256_update_multiple() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha256()\n    h.update(b\"a\")\n    h.update(b\"b\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_043_hashlib_sha512_update_then_digest() {
        let code = "import hashlib\ndef hash_it() -> bytes:\n    h = hashlib.sha512()\n    h.update(b\"data\")\n    return h.digest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_044_hashlib_blake2b_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2b()\n    h.update(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_045_hashlib_blake2s_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.blake2s()\n    h.update(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_046_hashlib_sha256_var_input() {
        let code = "import hashlib\ndef hash_it(val: str) -> str:\n    h = hashlib.sha256(val.encode())\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_047_hashlib_md5_var_input() {
        let code = "import hashlib\ndef hash_it(val: str) -> str:\n    h = hashlib.md5(val.encode())\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_048_hashlib_sha384_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha384()\n    h.update(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_049_hashlib_sha224_update_then_hex() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.sha224()\n    h.update(b\"data\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_050_hashlib_new_md5_data_hexdigest() {
        let code = "import hashlib\ndef hash_it() -> str:\n    h = hashlib.new(\"md5\", b\"test\")\n    return h.hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: BASE64 METHODS (tests 051-080)
    // ========================================================================

    #[test]
    fn test_w19dr_051_base64_b64encode() {
        let code = "import base64\ndef encode_it(data: bytes) -> bytes:\n    return base64.b64encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_052_base64_b64decode() {
        let code = "import base64\ndef decode_it(data: bytes) -> bytes:\n    return base64.b64decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_053_base64_urlsafe_b64encode() {
        let code = "import base64\ndef encode_url(data: bytes) -> bytes:\n    return base64.urlsafe_b64encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_054_base64_urlsafe_b64decode() {
        let code = "import base64\ndef decode_url(data: bytes) -> bytes:\n    return base64.urlsafe_b64decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_055_base64_b16encode() {
        let code = "import base64\ndef hex_encode(data: bytes) -> str:\n    return base64.b16encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_056_base64_b16decode() {
        let code = "import base64\ndef hex_decode(data: bytes) -> bytes:\n    return base64.b16decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_057_base64_b64encode_literal() {
        let code = "import base64\ndef encode_it() -> bytes:\n    return base64.b64encode(b\"hello world\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_058_base64_b64decode_literal() {
        let code = "import base64\ndef decode_it() -> bytes:\n    return base64.b64decode(b\"aGVsbG8=\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_059_base64_b64encode_in_func() {
        let code = "import base64\ndef process(msg: bytes) -> bytes:\n    encoded = base64.b64encode(msg)\n    return encoded";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_060_base64_b64decode_in_func() {
        let code = "import base64\ndef process(msg: bytes) -> bytes:\n    decoded = base64.b64decode(msg)\n    return decoded";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_061_base64_urlsafe_encode_in_func() {
        let code = "import base64\ndef process(data: bytes) -> bytes:\n    result = base64.urlsafe_b64encode(data)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_062_base64_urlsafe_decode_in_func() {
        let code = "import base64\ndef process(data: bytes) -> bytes:\n    result = base64.urlsafe_b64decode(data)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_063_base64_b16encode_in_func() {
        let code = "import base64\ndef process(data: bytes) -> str:\n    result = base64.b16encode(data)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_064_base64_b16decode_in_func() {
        let code = "import base64\ndef process(data: bytes) -> bytes:\n    result = base64.b16decode(data)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_065_base64_encode_string_input() {
        let code = "import base64\ndef encode_str(s: str) -> bytes:\n    return base64.b64encode(s.encode())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_066_base64_encode_var_bytes() {
        let code = "import base64\ndef encode_var(msg: bytes) -> bytes:\n    encoded = base64.b64encode(msg)\n    return encoded";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_067_base64_decode_var_bytes() {
        let code = "import base64\ndef decode_var(msg: bytes) -> bytes:\n    decoded = base64.b64decode(msg)\n    return decoded";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_068_base64_urlsafe_encode_literal() {
        let code = "import base64\ndef encode_it() -> bytes:\n    return base64.urlsafe_b64encode(b\"hello+world/test\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_069_base64_urlsafe_decode_literal() {
        let code = "import base64\ndef decode_it() -> bytes:\n    return base64.urlsafe_b64decode(b\"aGVsbG8\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_070_base64_b64encode_assigned() {
        let code = "import base64\ndef encode_it(data: bytes) -> bytes:\n    x = base64.b64encode(data)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_071_base64_b64decode_assigned() {
        let code = "import base64\ndef decode_it(data: bytes) -> bytes:\n    x = base64.b64decode(data)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_072_base64_b16encode_literal() {
        let code = "import base64\ndef hex_it() -> str:\n    return base64.b16encode(b\"abc\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_073_base64_b16decode_literal() {
        let code = "import base64\ndef unhex_it() -> bytes:\n    return base64.b16decode(b\"414243\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_074_base64_encode_return_direct() {
        let code = "import base64\ndef enc(b: bytes) -> bytes:\n    return base64.b64encode(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_075_base64_decode_return_direct() {
        let code = "import base64\ndef dec(b: bytes) -> bytes:\n    return base64.b64decode(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_076_base64_urlsafe_assigned_var() {
        let code = "import base64\ndef enc(b: bytes) -> bytes:\n    out = base64.urlsafe_b64encode(b)\n    return out";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_077_base64_urlsafe_decode_assigned_var() {
        let code = "import base64\ndef dec(b: bytes) -> bytes:\n    out = base64.urlsafe_b64decode(b)\n    return out";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_078_base64_b16encode_assigned() {
        let code = "import base64\ndef hex_it(d: bytes) -> str:\n    x = base64.b16encode(d)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_079_base64_b16decode_assigned() {
        let code = "import base64\ndef unhex_it(d: bytes) -> bytes:\n    x = base64.b16decode(d)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_080_base64_encode_decode_roundtrip() {
        let code = "import base64\ndef roundtrip(data: bytes) -> bytes:\n    encoded = base64.b64encode(data)\n    decoded = base64.b64decode(encoded)\n    return decoded";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: RE MODULE METHODS (tests 081-110)
    // ========================================================================

    #[test]
    fn test_w19dr_081_re_sub_literal() {
        let code = "import re\ndef clean(text: str) -> str:\n    return re.sub(\"\\\\d+\", \"X\", text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_082_re_sub_var_args() {
        let code = "import re\ndef clean(pattern: str, repl: str, text: str) -> str:\n    return re.sub(pattern, repl, text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_083_re_subn_literal() {
        let code = "import re\ndef clean(text: str) -> str:\n    result = re.subn(\"\\\\d+\", \"X\", text)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_084_re_findall_literal() {
        let code = "import re\ndef find_nums(text: str) -> list:\n    return re.findall(\"\\\\d+\", text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_085_re_findall_var_pattern() {
        let code = "import re\ndef find_pat(pattern: str, text: str) -> list:\n    return re.findall(pattern, text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_086_re_search_literal() {
        let code = "import re\ndef search_it(text: str) -> bool:\n    m = re.search(\"\\\\d+\", text)\n    return m is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_087_re_search_var_pattern() {
        let code = "import re\ndef search_it(pattern: str, text: str) -> bool:\n    m = re.search(pattern, text)\n    return m is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_088_re_match_literal() {
        let code = "import re\ndef match_it(text: str) -> bool:\n    m = re.match(\"\\\\d+\", text)\n    return m is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_089_re_match_var_pattern() {
        let code = "import re\ndef match_it(pattern: str, text: str) -> bool:\n    m = re.match(pattern, text)\n    return m is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_090_re_split_literal() {
        let code = "import re\ndef split_it(text: str) -> list:\n    return re.split(\"\\\\s+\", text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_091_re_split_var_pattern() {
        let code = "import re\ndef split_it(pattern: str, text: str) -> list:\n    return re.split(pattern, text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_092_re_compile_literal() {
        let code = "import re\ndef compile_it() -> str:\n    pat = re.compile(\"\\\\d+\")\n    return pat";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_093_re_compile_var() {
        let code = "import re\ndef compile_it(pattern: str) -> str:\n    pat = re.compile(pattern)\n    return pat";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_094_re_sub_in_func_return() {
        let code = "import re\ndef sanitize(s: str) -> str:\n    cleaned = re.sub(\"[^a-zA-Z]\", \"\", s)\n    return cleaned";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_095_re_findall_in_func_return() {
        let code = "import re\ndef extract_words(s: str) -> list:\n    words = re.findall(\"\\\\w+\", s)\n    return words";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_096_re_search_assigned() {
        let code = "import re\ndef has_digits(s: str) -> bool:\n    found = re.search(\"\\\\d\", s)\n    return found is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_097_re_match_assigned() {
        let code = "import re\ndef starts_digit(s: str) -> bool:\n    found = re.match(\"\\\\d\", s)\n    return found is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_098_re_split_assigned() {
        let code = "import re\ndef split_lines(s: str) -> list:\n    parts = re.split(\"\\\\n\", s)\n    return parts";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_099_re_subn_var_args() {
        let code = "import re\ndef clean(pat: str, rep: str, txt: str) -> str:\n    result = re.subn(pat, rep, txt)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_100_re_escape_literal() {
        let code = "import re\ndef escape_it(text: str) -> str:\n    return re.escape(text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_101_re_finditer_literal() {
        let code = "import re\ndef find_all(text: str) -> list:\n    return re.finditer(\"\\\\w+\", text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_102_re_fullmatch_literal() {
        let code = "import re\ndef full_match(text: str) -> bool:\n    m = re.fullmatch(\"\\\\d+\", text)\n    return m is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_103_re_sub_empty_repl() {
        let code = "import re\ndef strip_digits(s: str) -> str:\n    return re.sub(\"\\\\d\", \"\", s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_104_re_findall_word_boundary() {
        let code = "import re\ndef words(s: str) -> list:\n    return re.findall(\"\\\\b\\\\w+\\\\b\", s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_105_re_search_return_direct() {
        let code = "import re\ndef search_direct(p: str, t: str) -> str:\n    return re.search(p, t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_106_re_match_return_direct() {
        let code = "import re\ndef match_direct(p: str, t: str) -> str:\n    return re.match(p, t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_107_re_split_return_direct() {
        let code = "import re\ndef split_direct(p: str, t: str) -> list:\n    return re.split(p, t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_108_re_compile_assigned_var() {
        let code = "import re\ndef make_regex(p: str) -> str:\n    compiled = re.compile(p)\n    return compiled";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_109_re_sub_assigned_var() {
        let code = "import re\ndef do_sub(s: str) -> str:\n    result = re.sub(\"a\", \"b\", s)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_110_re_escape_assigned_var() {
        let code = "import re\ndef esc(s: str) -> str:\n    result = re.escape(s)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: COLORSYS METHODS (tests 111-130)
    // ========================================================================

    #[test]
    fn test_w19dr_111_colorsys_rgb_to_hsv_vars() {
        let code = "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    return colorsys.rgb_to_hsv(r, g, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_112_colorsys_hsv_to_rgb_vars() {
        let code = "import colorsys\ndef convert(h: float, s: float, v: float) -> tuple:\n    return colorsys.hsv_to_rgb(h, s, v)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_113_colorsys_rgb_to_hls_vars() {
        let code = "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    return colorsys.rgb_to_hls(r, g, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_114_colorsys_hls_to_rgb_vars() {
        let code = "import colorsys\ndef convert(h: float, l: float, s: float) -> tuple:\n    return colorsys.hls_to_rgb(h, l, s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_115_colorsys_rgb_to_hsv_assigned() {
        let code = "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    result = colorsys.rgb_to_hsv(r, g, b)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_116_colorsys_hsv_to_rgb_assigned() {
        let code = "import colorsys\ndef convert(h: float, s: float, v: float) -> tuple:\n    result = colorsys.hsv_to_rgb(h, s, v)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_117_colorsys_rgb_to_hls_assigned() {
        let code = "import colorsys\ndef convert(r: float, g: float, b: float) -> tuple:\n    result = colorsys.rgb_to_hls(r, g, b)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_118_colorsys_hls_to_rgb_assigned() {
        let code = "import colorsys\ndef convert(h: float, l: float, s: float) -> tuple:\n    result = colorsys.hls_to_rgb(h, l, s)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_119_colorsys_rgb_to_hsv_literal() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.rgb_to_hsv(0.5, 0.3, 0.8)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_120_colorsys_hsv_to_rgb_literal() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.hsv_to_rgb(0.7, 0.8, 0.9)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_121_colorsys_rgb_to_hls_literal() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.rgb_to_hls(0.1, 0.2, 0.3)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_122_colorsys_hls_to_rgb_literal() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.hls_to_rgb(0.4, 0.5, 0.6)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_123_colorsys_rgb_to_hsv_return_direct() {
        let code = "import colorsys\ndef conv(a: float, b: float, c: float) -> tuple:\n    return colorsys.rgb_to_hsv(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_124_colorsys_hsv_to_rgb_return_direct() {
        let code = "import colorsys\ndef conv(a: float, b: float, c: float) -> tuple:\n    return colorsys.hsv_to_rgb(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_125_colorsys_rgb_to_hls_return_direct() {
        let code = "import colorsys\ndef conv(a: float, b: float, c: float) -> tuple:\n    return colorsys.rgb_to_hls(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_126_colorsys_hls_to_rgb_return_direct() {
        let code = "import colorsys\ndef conv(a: float, b: float, c: float) -> tuple:\n    return colorsys.hls_to_rgb(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_127_colorsys_rgb_to_hsv_zero_vals() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.rgb_to_hsv(0.0, 0.0, 0.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_128_colorsys_hsv_to_rgb_zero_vals() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.hsv_to_rgb(0.0, 0.0, 0.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_129_colorsys_rgb_to_hls_one_vals() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.rgb_to_hls(1.0, 1.0, 1.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_130_colorsys_hls_to_rgb_one_vals() {
        let code = "import colorsys\ndef convert() -> tuple:\n    return colorsys.hls_to_rgb(1.0, 1.0, 1.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: TIME MODULE METHODS (tests 131-150)
    // ========================================================================

    #[test]
    fn test_w19dr_131_time_time() {
        let code = "import time\ndef get_time() -> float:\n    return time.time()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_132_time_sleep_literal() {
        let code = "import time\ndef wait() -> None:\n    time.sleep(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_133_time_sleep_var() {
        let code = "import time\ndef wait(n: float) -> None:\n    time.sleep(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_134_time_monotonic() {
        let code = "import time\ndef get_mono() -> float:\n    return time.monotonic()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_135_time_perf_counter() {
        let code = "import time\ndef get_perf() -> float:\n    return time.perf_counter()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_136_time_ctime_literal() {
        let code = "import time\ndef get_ctime() -> str:\n    return time.ctime(1234567890.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_137_time_ctime_var() {
        let code = "import time\ndef get_ctime(ts: float) -> str:\n    return time.ctime(ts)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_138_time_gmtime_no_args() {
        let code = "import time\ndef get_gmt() -> str:\n    return time.gmtime()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_139_time_gmtime_with_timestamp() {
        let code = "import time\ndef get_gmt(ts: float) -> str:\n    return time.gmtime(ts)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_140_time_localtime_no_args() {
        let code = "import time\ndef get_local() -> str:\n    return time.localtime()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_141_time_localtime_with_timestamp() {
        let code = "import time\ndef get_local(ts: float) -> str:\n    return time.localtime(ts)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_142_time_time_assigned() {
        let code = "import time\ndef measure() -> float:\n    start = time.time()\n    return start";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_143_time_monotonic_assigned() {
        let code = "import time\ndef measure() -> float:\n    start = time.monotonic()\n    return start";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_144_time_process_time() {
        let code = "import time\ndef cpu_time() -> float:\n    return time.process_time()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_145_time_thread_time() {
        let code = "import time\ndef thread_t() -> float:\n    return time.thread_time()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_146_time_sleep_float_literal() {
        let code = "import time\ndef wait() -> None:\n    time.sleep(0.5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_147_time_time_in_expression() {
        let code = "import time\ndef elapsed() -> float:\n    start = time.time()\n    end = time.time()\n    return end";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_148_time_perf_counter_assigned() {
        let code = "import time\ndef measure() -> float:\n    t = time.perf_counter()\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_149_time_ctime_assigned() {
        let code = "import time\ndef show(ts: float) -> str:\n    s = time.ctime(ts)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_150_time_gmtime_assigned() {
        let code = "import time\ndef show(ts: float) -> str:\n    t = time.gmtime(ts)\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 6: OS MODULE METHODS (tests 151-170)
    // ========================================================================

    #[test]
    fn test_w19dr_151_os_getenv_single() {
        let code = "import os\ndef get_key() -> str:\n    return os.getenv(\"HOME\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_152_os_getenv_with_default() {
        let code = "import os\ndef get_key() -> str:\n    return os.getenv(\"KEY\", \"default\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_153_os_path_join_two() {
        let code = "import os\ndef join_path(a: str, b: str) -> str:\n    return os.path.join(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_154_os_path_exists() {
        let code = "import os\ndef check(p: str) -> bool:\n    return os.path.exists(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_155_os_path_basename() {
        let code = "import os\ndef name(p: str) -> str:\n    return os.path.basename(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_156_os_path_dirname() {
        let code = "import os\ndef parent(p: str) -> str:\n    return os.path.dirname(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_157_os_getcwd() {
        let code = "import os\ndef cwd() -> str:\n    return os.getcwd()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_158_os_path_isfile() {
        let code = "import os\ndef check(p: str) -> bool:\n    return os.path.isfile(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_159_os_path_isdir() {
        let code = "import os\ndef check(p: str) -> bool:\n    return os.path.isdir(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_160_os_getenv_assigned() {
        let code = "import os\ndef get_key() -> str:\n    val = os.getenv(\"HOME\")\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_161_os_getenv_default_assigned() {
        let code = "import os\ndef get_key() -> str:\n    val = os.getenv(\"KEY\", \"fallback\")\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_162_os_path_join_three() {
        let code = "import os\ndef join_path(a: str, b: str, c: str) -> str:\n    return os.path.join(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_163_os_path_exists_assigned() {
        let code = "import os\ndef check(p: str) -> bool:\n    result = os.path.exists(p)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_164_os_path_basename_assigned() {
        let code = "import os\ndef name(p: str) -> str:\n    n = os.path.basename(p)\n    return n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_165_os_path_dirname_assigned() {
        let code = "import os\ndef parent(p: str) -> str:\n    d = os.path.dirname(p)\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_166_os_path_isabs() {
        let code = "import os\ndef check_abs(p: str) -> bool:\n    return os.path.isabs(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_167_os_path_splitext() {
        let code = "import os\ndef split_ext(p: str) -> tuple:\n    return os.path.splitext(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_168_os_path_split() {
        let code = "import os\ndef split_path(p: str) -> tuple:\n    return os.path.split(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_169_os_path_abspath() {
        let code = "import os\ndef abs_path(p: str) -> str:\n    return os.path.abspath(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_170_os_path_getsize() {
        let code = "import os\ndef size(p: str) -> int:\n    return os.path.getsize(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 7: SYS MODULE METHODS (tests 171-185)
    // ========================================================================

    #[test]
    fn test_w19dr_171_sys_exit_zero() {
        let code = "import sys\ndef quit_app() -> None:\n    sys.exit(0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_172_sys_exit_one() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_173_sys_exit_var() {
        let code = "import sys\ndef fail(code: int) -> None:\n    sys.exit(code)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_174_sys_exit_no_args() {
        let code = "import sys\ndef quit_app() -> None:\n    sys.exit()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_175_sys_exit_negative() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(-1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_176_sys_exit_string_msg() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(\"error\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_177_sys_exit_in_condition() {
        let code = "import sys\ndef maybe_exit(flag: bool) -> None:\n    if flag:\n        sys.exit(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_178_sys_exit_42() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_179_sys_exit_255() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(255)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_180_sys_exit_zero_explicit() {
        let code = "import sys\ndef ok() -> None:\n    rc = 0\n    sys.exit(rc)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_181_sys_exit_code_var() {
        let code = "import sys\ndef fail(rc: int) -> None:\n    sys.exit(rc)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_182_sys_exit_in_else() {
        let code = "import sys\ndef process(ok: bool) -> None:\n    if ok:\n        pass\n    else:\n        sys.exit(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_183_sys_exit_2() {
        let code = "import sys\ndef fail() -> None:\n    sys.exit(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_184_sys_exit_127() {
        let code = "import sys\ndef not_found() -> None:\n    sys.exit(127)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_185_sys_exit_assigned_code() {
        let code = "import sys\ndef fail() -> None:\n    code = 3\n    sys.exit(code)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 8: INT/DICT CLASS + MISC METHODS (tests 186-200)
    // ========================================================================

    #[test]
    fn test_w19dr_186_dict_fromkeys_list() {
        let code = "def make_dict(keys: list) -> dict:\n    return dict.fromkeys(keys)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_187_dict_fromkeys_with_value() {
        let code = "def make_dict(keys: list) -> dict:\n    return dict.fromkeys(keys, 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_188_os_path_expanduser() {
        let code = "import os\ndef home(p: str) -> str:\n    return os.path.expanduser(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_189_os_path_realpath() {
        let code = "import os\ndef real(p: str) -> str:\n    return os.path.realpath(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_190_os_path_normpath() {
        let code = "import os\ndef norm(p: str) -> str:\n    return os.path.normpath(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_191_os_path_getmtime() {
        let code = "import os\ndef mtime(p: str) -> float:\n    return os.path.getmtime(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_192_os_path_getctime() {
        let code = "import os\ndef ctime(p: str) -> float:\n    return os.path.getctime(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_193_os_path_join_single() {
        let code = "import os\ndef p(a: str) -> str:\n    return os.path.join(a)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_194_os_path_expandvars() {
        let code = "import os\ndef expand(p: str) -> str:\n    return os.path.expandvars(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_195_os_mkdir() {
        let code = "import os\ndef make(p: str) -> None:\n    os.mkdir(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_196_os_makedirs() {
        let code = "import os\ndef make_all(p: str) -> None:\n    os.makedirs(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_197_os_rename() {
        let code = "import os\ndef mv(src: str, dst: str) -> None:\n    os.rename(src, dst)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_198_os_rmdir() {
        let code = "import os\ndef rm(p: str) -> None:\n    os.rmdir(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_199_os_unlink() {
        let code = "import os\ndef rm(p: str) -> None:\n    os.unlink(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19dr_200_os_listdir() {
        let code = "import os\ndef ls(p: str) -> list:\n    return os.listdir(p)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
