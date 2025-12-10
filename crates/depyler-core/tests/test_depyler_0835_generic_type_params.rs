// DEPYLER-0835: Generic Type Parameter Scope Extraction
// RED PHASE TESTS - These tests should FAIL until implementation is complete
//
// Problem: When a class inherits from a parameterized generic base like `Iter[tuple[int, T]]`
// but doesn't explicitly declare `Generic[T]`, the type parameter T is not extracted.
// This causes E0412 "cannot find type `T` in this scope" in the generated Rust code.
//
// Example E0412 Error Pattern:
// ```python
// class EnumerateIter(Iter[tuple[int, T]]):
//     source: Iter[T]  # T used here but not declared
// ```
// Generates:
// ```rust
// pub struct EnumerateIter {  // Missing <T>
//     pub source: Iter<T>,    // E0412: cannot find type T
// }
// ```
//
// Solution: Extract type parameters from:
// 1. Explicit `Generic[T, U]` base class (current behavior)
// 2. Type variables used in parameterized base classes like `Iter[tuple[int, T]]`
// 3. Type variables used in field type annotations

use depyler_core::DepylerPipeline;

/// Helper to check for E0412 specifically
fn has_e0412_error(rust_code: &str) -> bool {
    use std::io::Write;
    use std::process::Command;

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let rust_file = temp_dir.path().join("test_generic.rs");
    let mut file = std::fs::File::create(&rust_file).expect("Failed to create file");
    writeln!(file, "{}", rust_code).expect("Failed to write file");
    drop(file);

    let output = Command::new("rustc")
        .args(["--crate-type", "lib", "--emit=metadata", "-o"])
        .arg(temp_dir.path().join("test_generic.rmeta"))
        .arg(&rust_file)
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.contains("E0412") || stderr.contains("cannot find type")
}

// ============================================================================
// SECTION 1: Type Parameters from Parameterized Base Class
// ============================================================================

/// Test: Class inherits from parameterized generic, T used in fields
/// Python: class EnumerateIter(Iter[tuple[int, T]]) with source: Iter[T]
/// Expected: struct EnumerateIter<T> with source: Iter<T>
#[test]
fn test_depyler_0835_parameterized_base_type_var_in_field() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Base(Generic[T]):
    def get(self) -> T:
        ...

@dataclass
class Wrapper(Base[tuple[int, T]]):
    inner: Base[T]
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Generated code should NOT have E0412
    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error: cannot find type 'T'\\n\\nGenerated code:\\n{}",
        rust_code
    );

    // Struct should have type parameter
    assert!(
        rust_code.contains("struct Wrapper<T") || rust_code.contains("struct Wrapper <T"),
        "Wrapper struct should have type parameter <T>\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: Multiple type vars in parameterized base
/// Python: class Mapper(Transform[T, U]) with input: T, output: U
#[test]
fn test_depyler_0835_parameterized_base_multiple_type_vars() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

class Transform(Generic[T, U]):
    pass

@dataclass
class Mapper(Transform[T, U]):
    input_val: T
    output_val: U
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 2: Type Parameters from Field Annotations Only
// ============================================================================

/// Test: Type var used in field but not in any base class
/// Python: class Container with item: T (no Generic base)
#[test]
fn test_depyler_0835_type_var_in_field_only() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar
from dataclasses import dataclass

T = TypeVar("T")

@dataclass
class Container:
    item: T
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Should have T as type param derived from field usage
    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error for field-only type var\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: Multiple type vars used in fields only
#[test]
fn test_depyler_0835_multiple_type_vars_in_fields() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import TypeVar
from dataclasses import dataclass

K = TypeVar("K")
V = TypeVar("V")

@dataclass
class Pair:
    key: K
    value: V
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 3: Complex Nested Type Parameters
// ============================================================================

/// Test: Type var nested in tuple in base class
/// Python: class EnumerateIter(Iter[tuple[int, T]])
#[test]
fn test_depyler_0835_type_var_nested_in_tuple() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Iter(Generic[T]):
    pass

@dataclass
class EnumerateIter(Iter[tuple[int, T]]):
    source: Iter[T]
    index: int = 0
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error for nested tuple type var\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: Type var in nested generic (list[T])
#[test]
fn test_depyler_0835_type_var_in_nested_generic() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Base(Generic[T]):
    pass

@dataclass
class ListWrapper(Base[list[T]]):
    items: list[T]
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error for nested generic type var\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 4: Method Return Types with Type Parameters
// ============================================================================

/// Test: Type var in method return type
#[test]
fn test_depyler_0835_type_var_in_method_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar

T = TypeVar("T")

class Container(Generic[T]):
    def get(self) -> T:
        ...

    def set(self, value: T) -> None:
        ...
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error in method return type\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: Method with type var in both param and return
#[test]
fn test_depyler_0835_type_var_method_param_and_return() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar

T = TypeVar("T")
U = TypeVar("U")

class Mapper(Generic[T]):
    def map(self, f: "Callable[[T], U]") -> U:
        ...
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // Method-level type params (U) should be on method, class-level (T) on struct
    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 5: Real-World Patterns from Corpus
// ============================================================================

/// Test: EnumerateIter pattern from example_generic_iterator
/// This is the exact pattern causing E0412 in the corpus
#[test]
fn test_depyler_0835_enumerate_iter_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Iter(Generic[T]):
    def next(self) -> tuple[T, bool]:
        ...

@dataclass
class EnumerateIter(Iter[tuple[int, T]]):
    source: Iter[T]
    index: int = 0

    def next(self) -> tuple[tuple[int, T], bool]:
        item, ok = self.source.next()
        if not ok:
            return (0, None), False
        result = (self.index, item)
        self.index += 1
        return result, True
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (EnumerateIter pattern)\\n\\nGenerated code:\\n{}",
        rust_code
    );

    // EnumerateIter should have <T>
    assert!(
        rust_code.contains("EnumerateIter<T") || rust_code.contains("EnumerateIter <T"),
        "EnumerateIter should have type parameter\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: ZipIter pattern - explicit Generic[T, U] alongside parameterized base
#[test]
fn test_depyler_0835_zip_iter_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")
U = TypeVar("U")

class Iter(Generic[T]):
    pass

@dataclass
class ZipIter(Iter[tuple[T, U]], Generic[T, U]):
    first: Iter[T]
    second: Iter[U]
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (ZipIter pattern)\\n\\nGenerated code:\\n{}",
        rust_code
    );

    // ZipIter should have <T, U>
    assert!(
        rust_code.contains("ZipIter<T") && rust_code.contains("U"),
        "ZipIter should have both type parameters\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: FilterIter pattern - inherits Iter[T] without explicit Generic[T]
#[test]
fn test_depyler_0835_filter_iter_pattern() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar, Callable
from dataclasses import dataclass

T = TypeVar("T")

class Iter(Generic[T]):
    pass

@dataclass
class FilterIter(Iter[T]):
    source: Iter[T]
    predicate: Callable[[T], bool]
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error (FilterIter pattern)\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 6: Edge Cases
// ============================================================================

/// Test: Concrete type in base but type var in field
/// Python: class Wrapper(Base[str]) with extra: T
#[test]
fn test_depyler_0835_concrete_base_type_var_field() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Base(Generic[T]):
    pass

@dataclass
class Wrapper(Base[str]):
    extra: T
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    // T should still be extracted from field usage
    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

/// Test: Same type var used multiple times in complex type
#[test]
fn test_depyler_0835_repeated_type_var() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Base(Generic[T]):
    pass

@dataclass
class Duplicate(Base[tuple[T, T]]):
    left: T
    right: T
"#;

    let transpiled = pipeline.transpile(python_code);
    assert!(transpiled.is_ok(), "Transpilation should succeed");
    let rust_code = transpiled.unwrap();

    assert!(
        !has_e0412_error(&rust_code),
        "Generated code has E0412 error\\n\\nGenerated code:\\n{}",
        rust_code
    );

    // Should only have one <T>, not <T, T>
    let count = rust_code.matches("<T:").count() + rust_code.matches("<T>").count();
    assert!(
        count > 0,
        "Should have type parameter T declared\\n\\nGenerated code:\\n{}",
        rust_code
    );
}

// ============================================================================
// SECTION 7: Compilation Verification
// ============================================================================

/// Meta-test: Ensure all generated code does NOT have E0412 errors
/// Note: We check for E0412 specifically, not full compilation
/// (other errors like E0369/E0277 are tracked in separate tickets)
#[test]
fn test_depyler_0835_all_patterns_no_e0412() {
    let pipeline = DepylerPipeline::new();

    let test_cases = vec![
        (
            "parameterized_base",
            r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Base(Generic[T]):
    pass

@dataclass
class Child(Base[list[T]]):
    items: list[T]
"#,
        ),
        (
            "field_only_type_var",
            r#"
from typing import TypeVar
from dataclasses import dataclass

T = TypeVar("T")

@dataclass
class Box:
    value: T
"#,
        ),
        (
            "nested_tuple",
            r#"
from typing import Generic, TypeVar
from dataclasses import dataclass

T = TypeVar("T")

class Iter(Generic[T]):
    pass

@dataclass
class EnumIter(Iter[tuple[int, T]]):
    inner: Iter[T]
"#,
        ),
    ];

    let mut e0412_failures = Vec::new();

    for (name, python_code) in test_cases {
        let result = pipeline.transpile(python_code);
        if let Ok(rust_code) = result {
            // Only fail if we have E0412 errors specifically
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
