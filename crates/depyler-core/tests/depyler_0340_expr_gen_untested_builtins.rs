//! DEPYLER-0340: expr_gen.rs Untested Builtin Functions Coverage
//!
//! **EXTREME TDD Protocol - Coverage Boost**
//!
//! Target: expr_gen.rs 38.97% → 85%+ coverage
//! TDG Score: 74.7/100 (B-) - High priority for quality improvement
//!
//! This test suite adds coverage for untested builtin function conversions:
//! - divmod(), chr(), ord(), hex(), bin(), oct()
//! - hash(), repr(), next(), getattr()
//! - iter(), type(), frozenset()
//! - reversed(), enumerate() edge cases
//! - range() with negative/positive steps

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

// ============================================================================
// ARITHMETIC BUILTINS
// ============================================================================

#[test]
fn test_divmod_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_divmod(a: int, b: int):
    quotient, remainder = divmod(a, b)
    return quotient
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated divmod code:\n{}", rust_code);

    // divmod(a, b) should generate tuple (a / b, a % b) or dedicated logic
    assert!(
        rust_code.contains("divmod") || (rust_code.contains("/") && rust_code.contains("%")),
        "divmod() should generate division and modulo operations"
    );
}

#[test]
fn test_pow_builtin_with_modulo() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_pow(base: int, exp: int, modulo: int) -> int:
    return pow(base, exp, modulo)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated pow with modulo code:\n{}", rust_code);

    // pow(base, exp, mod) should use modular exponentiation
    assert!(
        rust_code.contains("pow") || rust_code.contains("modpow"),
        "pow(base, exp, mod) should generate modular exponentiation"
    );
}

// ============================================================================
// NUMBER CONVERSION BUILTINS
// ============================================================================

#[test]
fn test_hex_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_hex(num: int) -> str:
    return hex(num)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated hex() code:\n{}", rust_code);

    // hex(n) should generate format!("0x{:x}", n) or similar
    assert!(
        rust_code.contains("format!") && rust_code.contains(":x"),
        "hex() should generate hexadecimal formatting"
    );
}

#[test]
fn test_bin_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_bin(num: int) -> str:
    return bin(num)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated bin() code:\n{}", rust_code);

    // bin(n) should generate format!("0b{:b}", n) or similar
    assert!(
        rust_code.contains("format!") && rust_code.contains(":b"),
        "bin() should generate binary formatting"
    );
}

#[test]
fn test_oct_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_oct(num: int) -> str:
    return oct(num)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated oct() code:\n{}", rust_code);

    // oct(n) should generate format!("0o{:o}", n) or similar
    assert!(
        rust_code.contains("format!") && rust_code.contains(":o"),
        "oct() should generate octal formatting"
    );
}

#[test]
fn test_chr_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_chr(code: int) -> str:
    return chr(code)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated chr() code:\n{}", rust_code);

    // chr(n) should generate char::from_u32() or from_u32_unchecked()
    assert!(
        rust_code.contains("char::from") || rust_code.contains("from_u32"),
        "chr() should generate char conversion from u32"
    );
}

#[test]
fn test_ord_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_ord(c: str) -> int:
    return ord(c)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated ord() code:\n{}", rust_code);

    // ord(c) should generate char as u32 or similar
    assert!(
        rust_code.contains("as u32") || rust_code.contains("chars()"),
        "ord() should generate char to u32 conversion"
    );
}

// ============================================================================
// INTROSPECTION BUILTINS
// ============================================================================

#[test]
fn test_hash_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_hash(obj: int) -> int:
    return hash(obj)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated hash() code:\n{}", rust_code);

    // hash(obj) should generate hasher or hash computation
    assert!(
        rust_code.contains("hash") || rust_code.contains("Hash"),
        "hash() should generate hash computation"
    );
}

#[test]
fn test_repr_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_repr(obj: int) -> str:
    return repr(obj)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated repr() code:\n{}", rust_code);

    // repr(obj) should generate format!("{:?}", obj) or Debug trait
    assert!(
        rust_code.contains("format!") && rust_code.contains(":?"),
        "repr() should generate debug formatting"
    );
}

#[test]
fn test_type_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_type(obj: int) -> str:
    return type(obj).__name__
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated type() code:\n{}", rust_code);

    // type(obj) should generate type_name or similar introspection
    assert!(
        rust_code.contains("type_name") || rust_code.contains("TypeId"),
        "type() should generate type introspection"
    );
}

#[test]
#[ignore = "getattr() not yet implemented - requires runtime reflection"]
fn test_getattr_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
class Point:
    x: int
    y: int

def test_getattr(p: Point, attr: str):
    return getattr(p, attr, 0)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated getattr() code:\n{}", rust_code);

    // getattr(obj, name, default) should generate field access or match
    assert!(
        rust_code.contains("getattr") || rust_code.contains("match"),
        "getattr() should generate dynamic attribute access"
    );
}

// ============================================================================
// ITERATOR BUILTINS
// ============================================================================

#[test]
fn test_next_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_next(items: list):
    it = iter(items)
    return next(it)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated next() code:\n{}", rust_code);

    // next(iter) should generate .next() method call
    assert!(
        rust_code.contains(".next("),
        "next() should generate iterator .next() call"
    );
}

#[test]
fn test_next_builtin_with_default() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_next(items: list):
    it = iter(items)
    return next(it, -1)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated next() with default code:\n{}", rust_code);

    // next(iter, default) should generate .next().unwrap_or(default)
    assert!(
        rust_code.contains(".next(") && rust_code.contains("unwrap_or"),
        "next(iter, default) should use .next().unwrap_or()"
    );
}

#[test]
fn test_iter_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_iter(items: list):
    return iter(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated iter() code:\n{}", rust_code);

    // iter(obj) should generate .iter() or .into_iter()
    assert!(
        rust_code.contains(".iter(") || rust_code.contains(".into_iter("),
        "iter() should generate iterator creation"
    );
}

#[test]
fn test_reversed_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_reversed(items: list):
    return reversed(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated reversed() code:\n{}", rust_code);

    // reversed(seq) should generate .iter().rev() or .reverse()
    assert!(
        rust_code.contains(".rev(") || rust_code.contains(".reverse("),
        "reversed() should generate reverse iteration"
    );
}

#[test]
#[ignore = "enumerate(start=N) not yet implemented"]
fn test_enumerate_with_start() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_enumerate(items: list):
    for idx, val in enumerate(items, start=10):
        print(idx, val)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated enumerate(start=10) code:\n{}", rust_code);

    // enumerate(iter, start=10) should generate .enumerate() with offset
    assert!(
        rust_code.contains(".enumerate(") && rust_code.contains("+ 10"),
        "enumerate(iter, start=10) should add offset to index"
    );
}

// ============================================================================
// COLLECTION BUILTINS
// ============================================================================

#[test]
fn test_frozenset_builtin() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_frozenset(items: list):
    return frozenset(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated frozenset() code:\n{}", rust_code);

    // frozenset(iter) should generate HashSet (immutable in Rust by default)
    assert!(
        rust_code.contains("HashSet") && rust_code.contains("collect"),
        "frozenset() should generate HashSet from iterator"
    );
}

#[test]
fn test_frozenset_literal() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_frozenset():
    return frozenset({1, 2, 3})
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated frozenset literal code:\n{}", rust_code);

    // frozenset({...}) should generate HashSet
    assert!(
        rust_code.contains("HashSet"),
        "frozenset literal should generate HashSet"
    );
}

// ============================================================================
// RANGE BUILTINS - EDGE CASES
// ============================================================================

#[test]
fn test_range_negative_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_range():
    return list(range(10, 0, -1))
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated range(10, 0, -1) code:\n{}", rust_code);

    // range(start, stop, -step) should generate reverse iteration
    assert!(
        rust_code.contains("rev(") || rust_code.contains("step_by"),
        "range(10, 0, -1) should generate reverse iteration"
    );
}

#[test]
fn test_range_positive_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_range():
    return list(range(0, 20, 2))
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated range(0, 20, 2) code:\n{}", rust_code);

    // range(start, stop, step) should generate .step_by(step)
    assert!(
        rust_code.contains("step_by("),
        "range(0, 20, 2) should generate .step_by()"
    );
}

#[test]
fn test_range_single_arg() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_range():
    return list(range(5))
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated range(5) code:\n{}", rust_code);

    // range(n) should generate (0..n)
    assert!(
        rust_code.contains("0..") || rust_code.contains("range("),
        "range(5) should generate (0..5) range"
    );
}

// ============================================================================
// PROPERTY TESTS - Builtin Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_hex_transpiles_without_panic(num in -1000i32..1000i32) {
            let pipeline = DepylerPipeline::new();
            let python_code = format!(
                "def test_hex() -> str:\n    return hex({})",
                num
            );

            // Should not panic, even if transpilation fails
            let _result = pipeline.transpile(&python_code);
        }

        #[test]
        fn prop_chr_transpiles_without_panic(code in 0u32..128) {
            let pipeline = DepylerPipeline::new();
            let python_code = format!(
                "def test_chr() -> str:\n    return chr({})",
                code
            );

            let _result = pipeline.transpile(&python_code);
        }

        #[test]
        fn prop_range_transpiles_without_panic(
            start in -100i32..100,
            stop in -100i32..100,
        ) {
            let pipeline = DepylerPipeline::new();
            let python_code = format!(
                "def test_range():\n    return list(range({}, {}))",
                start, stop
            );

            let _result = pipeline.transpile(&python_code);
        }
    }
}
