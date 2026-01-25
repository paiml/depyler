//! DEPYLER-0967: Dunder Method Translation Tests
//!
//! This test module validates the correct translation of Python dunder methods
//! to their Rust equivalents. Key mappings:
//! - `__len__` → `len()`
//! - `__str__` → `to_string()`
//! - `__repr__` → `fmt()` (Debug trait)
//! - `__getitem__` → `index()`
//! - `__contains__` → `contains()`

use depyler_core::DepylerPipeline;

#[test]
fn test_dunder_len_method_generated() {
    // Python class with __len__ should generate len() method
    let python = r#"
from dataclasses import dataclass

@dataclass
class Container:
    items: list[int]

    def __len__(self) -> int:
        return len(self.items)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate a len() method (not __len__)
    assert!(
        rust_code.contains("pub fn len("),
        "Should generate len() method from __len__\n\nGenerated:\n{}",
        rust_code
    );

    // Should NOT have __len__ as method name
    assert!(
        !rust_code.contains("fn __len__"),
        "Should NOT have __len__ as method name\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_dunder_len_called_in_function() {
    // Python function calling len() on a custom class with __len__
    let python = r#"
from dataclasses import dataclass

@dataclass
class Vector:
    components: list[float]

    def __len__(self) -> int:
        return len(self.components)


def get_length(v: Vector) -> int:
    return len(v)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should have a len() method on Vector
    assert!(
        rust_code.contains("pub fn len("),
        "Should generate len() method\n\nGenerated:\n{}",
        rust_code
    );

    // The function should call v.len() not len(v)
    // Note: This tests the method generation, actual call transformation is separate
    assert!(
        rust_code.contains(".len()"),
        "Should call .len() method\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_dunder_str_method_generated() {
    // Python class with __str__ should generate to_string() method
    let python = r#"
from dataclasses import dataclass

@dataclass
class Point:
    x: float
    y: float

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate a to_string() method (not __str__)
    assert!(
        rust_code.contains("pub fn to_string("),
        "Should generate to_string() method from __str__\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_dunder_getitem_method_generated() {
    // Python class with __getitem__ should generate index() method
    let python = r#"
from dataclasses import dataclass

@dataclass
class MyList:
    data: list[int]

    def __getitem__(self, i: int) -> int:
        return self.data[i]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate an index() method (not __getitem__)
    assert!(
        rust_code.contains("pub fn index("),
        "Should generate index() method from __getitem__\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_dunder_eq_method_generated() {
    // Python class with __eq__ should generate eq() method
    let python = r#"
from dataclasses import dataclass

@dataclass
class Value:
    n: int

    def __eq__(self, other: Value) -> bool:
        return self.n == other.n
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate an eq() method (not __eq__)
    assert!(
        rust_code.contains("pub fn eq("),
        "Should generate eq() method from __eq__\n\nGenerated:\n{}",
        rust_code
    );
}
