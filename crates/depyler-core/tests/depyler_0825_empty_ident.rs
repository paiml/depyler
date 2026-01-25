//! DEPYLER-0825: Test for empty ident panic with functools.partial
//!
//! Problem: When `from functools import partial` is used, the item_map
//! maps `partial` to an empty string (signifying no direct Rust equivalent).
//! This causes the code generator to produce `std::` when joined, which
//! when split by `::` produces `["std", ""]`. `syn::Ident::new("")` then
//! panics with "Ident is not allowed to be empty".
//!
//! Solution: In import_gen.rs, don't insert items with empty rust_name
//! into imported_items. Let them fall through to bail with a cleaner error.

#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

/// Test that functools.partial import doesn't panic
/// Instead it should fail gracefully with a meaningful error
#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0825_functools_partial_no_panic() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
from functools import partial

def apply_partial(f, x):
    return partial(f, x)
"#;

    // This should NOT panic - it should either succeed or fail gracefully
    let result = pipeline.transpile(python);
    // The transpilation may fail (partial isn't supported), but it should NOT panic
    // Just check it doesn't panic - any result is fine
    let _ = result;
}

/// Test functools.reduce still works (it has a real mapping)
#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0825_functools_reduce_works() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
from functools import reduce

def sum_list(items: list[int]) -> int:
    return reduce(lambda a, b: a + b, items, 0)
"#;

    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "reduce() should transpile: {:?}",
        result.err()
    );
}

/// Test that lru_cache (empty mapping) doesn't panic
#[test]
#[allow(non_snake_case)]
fn test_DEPYLER_0825_functools_lru_cache_no_panic() {
    let pipeline = DepylerPipeline::new();

    let python = r#"
from functools import lru_cache

@lru_cache
def fib(n: int) -> int:
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)
"#;

    // This should NOT panic
    let result = pipeline.transpile(python);
    let _ = result;
}
