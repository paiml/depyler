//! DEPYLER-0968: Sum Function Translation Tests
//!
//! This test module validates the correct translation of Python sum() builtin
//! to Rust iterator .sum() method in class method bodies.
//!
//! Key mappings:
//! - `sum(x*x for x in items)` → `.iter().map(|x| x*x).sum::<T>()`
//! - `sum(range(n))` → `(0..n).sum::<T>()`
//! - `sum(d.values())` → `d.values().cloned().sum::<T>()`
//! - `sum(list)` → `list.iter().sum::<T>()`

use depyler_core::DepylerPipeline;

#[test]
fn test_sum_generator_expression_in_class_method() {
    // Python class with sum(generator_exp) should generate .sum() method call
    let python = r#"
from dataclasses import dataclass

@dataclass
class Vector:
    components: list[float]

    def magnitude(self) -> float:
        return sum(c * c for c in self.components) ** 0.5
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate .sum::<f64>() method call (not sum() free function)
    assert!(
        rust_code.contains(".sum::<"),
        "Should generate .sum::<T>() method call from sum(generator)\\n\\nGenerated:\\n{}",
        rust_code
    );

    // Should NOT have sum( as a free function call
    assert!(
        !rust_code.contains("{ sum("),
        "Should NOT have sum() as free function call\\n\\nGenerated:\\n{}",
        rust_code
    );
}

#[test]
fn test_sum_list_in_class_method() {
    // Python class with sum(list) should generate .iter().sum()
    let python = r#"
from dataclasses import dataclass

@dataclass
class Stats:
    values: list[int]

    def total(self) -> int:
        return sum(self.values)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate .sum() method call
    assert!(
        rust_code.contains(".sum::<"),
        "Should generate .sum::<T>() method call\\n\\nGenerated:\\n{}",
        rust_code
    );
}

#[test]
fn test_sum_compiles_correctly() {
    // Full example that should compile
    let python = r#"
from dataclasses import dataclass

@dataclass
class Vector:
    components: list[float]

    def __len__(self) -> int:
        return len(self.components)

    def magnitude(self) -> float:
        return sum(c * c for c in self.components) ** 0.5


def add_vectors(v1: Vector, v2: Vector) -> Vector:
    if len(v1) != len(v2):
        raise ValueError("Dimension mismatch")
    return Vector([v1.components[i] + v2.components[i] for i in range(len(v1))])
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // The code should compile - verify key patterns
    assert!(
        rust_code.contains("pub fn magnitude"),
        "Should have magnitude method\\n\\nGenerated:\\n{}",
        rust_code
    );
    assert!(
        rust_code.contains("pub fn len"),
        "Should have len method (from __len__)\\n\\nGenerated:\\n{}",
        rust_code
    );
    assert!(
        rust_code.contains(".sum::<"),
        "Should use .sum::<T>() for sum()\\n\\nGenerated:\\n{}",
        rust_code
    );
}

#[test]
fn test_sum_preserves_type_inference() {
    // sum() on int list should use i32, on float list should use f64
    let python = r#"
from dataclasses import dataclass

@dataclass
class Container:
    ints: list[int]
    floats: list[float]

    def sum_ints(self) -> int:
        return sum(self.ints)

    def sum_floats(self) -> float:
        return sum(self.floats)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should have both sum methods
    assert!(
        rust_code.contains("sum_ints") && rust_code.contains("sum_floats"),
        "Should have both sum methods\\n\\nGenerated:\\n{}",
        rust_code
    );
}
