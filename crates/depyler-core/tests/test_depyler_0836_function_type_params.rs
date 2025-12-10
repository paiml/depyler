// DEPYLER-0836: Generic Type Parameter Extraction for Standalone Functions
// RED PHASE TESTS - These tests should FAIL until implementation is complete
//
// Problem: When a standalone function (not a method) has type variables in its
// signature (parameters or return type), the type parameters are not added to
// the Rust function signature.
//
// Example E0412 Error Pattern:
// ```python
// def nothing() -> Maybe[T]:
//     return Nothing()
//
// def left(value: L) -> Either[L, R]:
//     return Left(value)
// ```
// Generates:
// ```rust
// pub fn nothing() -> Maybe<T> {  // E0412: cannot find type `T`
// pub fn left(value: L) -> Either<L, R> {  // E0412: cannot find type `R`
// ```
//
// Solution: Extract type parameters from:
// 1. Return type annotations containing type variables
// 2. Parameter type annotations containing type variables
// 3. Handle cases where only some type vars are in params (need remaining in generics)

use depyler_core::DepylerPipeline;

/// Helper to check for E0412 specifically
fn has_e0412_error(rust_code: &str) -> bool {
    use std::io::Write;
    use std::process::Command;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let rust_file = temp_dir.path().join("test_func_generic.rs");
    let mut file = std::fs::File::create(&rust_file).expect("Failed to create file");
    writeln!(file, "{}", rust_code).expect("Failed to write file");
    drop(file);

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--emit=metadata", "-o"])
        .arg(temp_dir.path().join("test_func_generic.rmeta"))
        .arg(&rust_file)
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.contains("E0412") || stderr.contains("cannot find type")
}

// ============================================================================
// SECTION 1: Type Variables in Return Type Only
// ============================================================================

/// Test: Function returns generic type, type var only in return
/// Python: def nothing() -> Maybe[T]
/// Expected: fn nothing<T>() -> Maybe<T>
#[test]
fn test_depyler_0836_return_type_only_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")

class Maybe(Generic[T]):
    pass

@dataclass
class Nothing(Maybe[T]):
    pass

def nothing() -> Maybe[T]:
    return Nothing()
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Generated code should NOT have E0412
    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error: cannot find type 'T'\n\nGenerated code:\n{}",
        rust_code
    );

    // Function should have type parameter
    assert!(
        rust_code.contains("fn nothing<T") || rust_code.contains("fn nothing <T"),
        "nothing() should have type parameter <T>\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Function returns Either with two type vars
/// Python: def left(value: L) -> Either[L, R]
/// Expected: fn left<L, R>(value: L) -> Either<L, R>
#[test]
fn test_depyler_0836_return_type_extra_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

L = TypeVar("L")
R = TypeVar("R")

class Either(Generic[L, R]):
    pass

@dataclass
class Left(Either[L, R]):
    value: L

def left(value: L) -> Either[L, R]:
    return Left(value)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    // left function should have both L and R type params
    // L is in parameter, R is only in return type
    assert!(
        rust_code.contains("fn left<") && rust_code.contains("R"),
        "left() should have type parameter R\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Function returns generic with right type var
/// Python: def right(value: R) -> Either[L, R]
/// Expected: fn right<L, R>(value: R) -> Either<L, R>
#[test]
fn test_depyler_0836_return_type_missing_left_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

L = TypeVar("L")
R = TypeVar("R")

class Either(Generic[L, R]):
    pass

@dataclass
class Right(Either[L, R]):
    value: R

def right(value: R) -> Either[L, R]:
    return Right(value)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    // right function should have both L and R type params
    // R is in parameter, L is only in return type
    assert!(
        rust_code.contains("fn right<") && rust_code.contains("L"),
        "right() should have type parameter L\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 2: Type Variables in Parameters Only
// ============================================================================

/// Test: Type var in parameter, concrete return
/// Python: def identity(x: T) -> T
/// Expected: fn identity<T>(x: T) -> T
#[test]
fn test_depyler_0836_param_type_var_same_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar("T")

def identity(x: T) -> T:
    return x
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn identity<T"),
        "identity() should have type parameter <T>\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Multiple type vars in parameters
/// Python: def swap(a: T, b: U) -> tuple[U, T]
/// Expected: fn swap<T, U>(a: T, b: U) -> (U, T)
#[test]
fn test_depyler_0836_multiple_param_type_vars() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar("T")
U = TypeVar("U")

def swap(a: T, b: U) -> tuple[U, T]:
    return (b, a)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    // Should have both T and U
    assert!(
        rust_code.contains("fn swap<") && rust_code.contains("T") && rust_code.contains("U"),
        "swap() should have type parameters T and U\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 3: Callable Type Parameters
// ============================================================================

/// Test: Higher-order function with type vars in Callable
/// Python: def lift(f: Callable[[T], U]) -> ...
/// Expected: fn lift<T, U>(f: impl Fn(T) -> U) -> ...
#[test]
fn test_depyler_0836_callable_type_vars() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Callable, Generic
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

class Maybe(Generic[T]):
    pass

def lift(f: Callable[[T], U]) -> Callable[[Maybe[T]], Maybe[U]]:
    def lifted(m: Maybe[T]) -> Maybe[U]:
        return m
    return lifted
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    // Should have T and U in function signature
    assert!(
        rust_code.contains("fn lift<"),
        "lift() should have type parameters\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Binary function lift
/// Python: def lift2(f: Callable[[T, U], V]) -> ...
#[test]
fn test_depyler_0836_callable_three_type_vars() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Callable

T = TypeVar("T")
U = TypeVar("U")
V = TypeVar("V")

def apply2(f: Callable[[T, U], V], a: T, b: U) -> V:
    return f(a, b)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 4: Generic Container Parameters
// ============================================================================

/// Test: Function with list of type var
/// Python: def safe_head(lst: list[T]) -> Maybe[T]
#[test]
fn test_depyler_0836_list_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")

class Maybe(Generic[T]):
    pass

@dataclass
class Some(Maybe[T]):
    value: T

class Nothing(Maybe[T]):
    pass

def safe_head(lst: list[T]) -> Maybe[T]:
    if lst:
        return Some(lst[0])
    return Nothing()
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn safe_head<T"),
        "safe_head() should have type parameter <T>\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Function with dict of type vars
/// Python: def safe_get(d: dict[str, T], key: str) -> Maybe[T]
#[test]
fn test_depyler_0836_dict_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")

class Maybe(Generic[T]):
    pass

@dataclass
class Some(Maybe[T]):
    value: T

class Nothing(Maybe[T]):
    pass

def safe_get(d: dict[str, T], key: str) -> Maybe[T]:
    if key in d:
        return Some(d[key])
    return Nothing()
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    // Check that T appears as a type parameter (may have lifetimes before it)
    assert!(
        rust_code.contains("safe_get<") && rust_code.contains(", T") || rust_code.contains("<T"),
        "safe_get() should have type parameter T\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 5: Union Types (T | None)
// ============================================================================

/// Test: Union return type should not create invalid type names
/// Python: def filter(...) -> Pipeline[T | None]
/// Expected: fn filter<T>(...) -> Pipeline<Option<T>>
/// Note: This tests T | None -> Option<T> conversion, which is type mapping, not type param extraction
/// Tracked separately - out of scope for DEPYLER-0836
#[test]
#[ignore = "Union type T|None -> Option<T> conversion tracked separately"]
fn test_depyler_0836_union_type_in_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

@dataclass
class Pipeline(Generic[T]):
    value: T

    def filter(self, pred) -> "Pipeline[T | None]":
        if pred(self.value):
            return Pipeline(self.value)
        return Pipeline(None)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Should NOT contain T___None - that's an invalid type name
    assert!(
        !rust_code.contains("T___None"),
        "Should not generate T___None type\n\nGenerated code:\n{}",
        rust_code
    );

    // Should use Option<T> instead
    assert!(
        rust_code.contains("Option<T>") || rust_code.contains("Pipeline<Option<T>>"),
        "Union T | None should become Option<T>\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 6: Real-World Patterns from Corpus
// ============================================================================

/// Test: Either.left pattern from func_either
#[test]
fn test_depyler_0836_either_left_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

L = TypeVar("L")
R = TypeVar("R")

class Either(Generic[L, R]):
    pass

@dataclass
class Left(Either[L, R]):
    value: L

def left(value: L) -> Either[L, R]:
    return Left(value)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (left pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Either.right pattern from func_either
#[test]
fn test_depyler_0836_either_right_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

L = TypeVar("L")
R = TypeVar("R")

class Either(Generic[L, R]):
    pass

@dataclass
class Right(Either[L, R]):
    value: R

def right(value: R) -> Either[L, R]:
    return Right(value)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (right pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Maybe.nothing pattern from func_maybe
#[test]
fn test_depyler_0836_maybe_nothing_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic

T = TypeVar("T")

class Maybe(Generic[T]):
    pass

class Nothing(Maybe[T]):
    pass

def nothing() -> Maybe[T]:
    return Nothing()
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (nothing pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: some constructor pattern
#[test]
fn test_depyler_0836_maybe_some_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")

class Maybe(Generic[T]):
    pass

@dataclass
class Some(Maybe[T]):
    value: T

def some(value: T) -> Maybe[T]:
    return Some(value)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (some pattern)\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 7: Edge Cases
// ============================================================================

/// Test: Type var not in any parameter (zero-arg generic function)
#[test]
fn test_depyler_0836_zero_arg_generic_function() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar

T = TypeVar("T")

def default() -> T:
    return None
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );

    assert!(
        rust_code.contains("fn default<T"),
        "default() should have type parameter\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Type var with same name as concrete type should still work
#[test]
fn test_depyler_0836_type_var_same_name_as_concrete() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic

T = TypeVar("T")

def wrap(x: T) -> list[T]:
    return [x]
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );
}

/// Test: Nested generics in return type
#[test]
fn test_depyler_0836_nested_generic_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar, Generic
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

class Maybe(Generic[T]):
    pass

@dataclass
class Some(Maybe[T]):
    value: T

def sequence(maybes: list[Maybe[T]]) -> Maybe[list[T]]:
    results = []
    for m in maybes:
        results.append(m)
    return Some(results)
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\n\nGenerated code:\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 8: Compilation Verification
// ============================================================================

/// Meta-test: All patterns should not have E0412
#[test]
fn test_depyler_0836_all_patterns_no_e0412() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        (
            "return_only_type_var",
            r#"
from typing import TypeVar, Generic

T = TypeVar("T")

class Box(Generic[T]):
    pass

def empty_box() -> Box[T]:
    return Box()
"#,
        ),
        (
            "extra_type_var_in_return",
            r#"
from typing import TypeVar, Generic

L = TypeVar("L")
R = TypeVar("R")

class Pair(Generic[L, R]):
    pass

def make_pair(left: L) -> Pair[L, R]:
    return Pair()
"#,
        ),
        (
            "identity_function",
            r#"
from typing import TypeVar

T = TypeVar("T")

def id(x: T) -> T:
    return x
"#,
        ),
    ];

    let mut e0412_failures = Vec::new();

    for (name, python_code) in test_cases {
        let result = pipeline.transpile(python_code);
        if let Ok(rust_code) = result {
            if has_e0412_error(&rust_code) {
                e0412_failures.push((name, rust_code));
            }
        } else {
            e0412_failures.push((name, format!("Transpilation failed: {:?}", result.err())));
        }
    }

    assert!(
        e0412_failures.is_empty(),
        "The following test cases have E0412 errors (cannot find type):\n{}",
        e0412_failures
            .iter()
            .map(|(name, code)| format!("=== {} ===\n{}", name, code))
            .collect::<Vec<_>>()
            .join("\n\n")
    );
}
