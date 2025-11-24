//! DEPYLER-0495: Generator Return Type Incorrect
//!
//! Tests that Python generators with `Iterator[T]` return type transpile to correct
//! Rust `impl Iterator<Item = T>` (not `impl Iterator<Item = Iterator<T>>`).
//!
//! BUG: Iterator[int] transpiles to Iterator<Item = Iterator<i32>> instead of Iterator<Item = i32>
//! Expected: Direct element type mapping, no nested Iterator

use depyler_core::DepylerPipeline;
use std::io::Write;

#[test]
fn test_generator_iterator_int_return_type() {
    // Generator with Iterator[int] return type
    let python = r#"
from typing import Iterator

def count_up(n: int) -> Iterator[int]:
    i = 0
    while i < n:
        yield i
        i += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have correct function signature: impl Iterator<Item = i32>
    // NOT: impl Iterator<Item = Iterator<i32>>
    assert!(
        rust.contains("impl Iterator<Item = i32>") || rust.contains("impl Iterator<Item=i32>"),
        "BUG: Function return type should be 'impl Iterator<Item = i32>'\nGenerated:\n{}",
        rust
    );

    // Should NOT have nested Iterator
    assert!(
        !rust.contains("Iterator<Item = Iterator<i32>>")
        && !rust.contains("Iterator<Item=Iterator<i32>>"),
        "BUG: Return type incorrectly nested Iterator<Item = Iterator<i32>>\nGenerated:\n{}",
        rust
    );

    // Should have correct associated type in impl block
    assert!(
        rust.contains("type Item = i32;"),
        "BUG: Associated type should be 'type Item = i32;' not 'type Item = Iterator<i32>;'\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_iterator_string_return_type() {
    // Generator with Iterator[str] return type
    let python = r#"
from typing import Iterator

def word_gen() -> Iterator[str]:
    words = ["hello", "world"]
    for word in words:
        yield word
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have String, not Iterator<String>
    assert!(
        rust.contains("impl Iterator<Item = String>") || rust.contains("impl Iterator<Item=String>"),
        "BUG: Function return type should be 'impl Iterator<Item = String>'\nGenerated:\n{}",
        rust
    );

    assert!(
        !rust.contains("Iterator<Item = Iterator<String>>"),
        "BUG: Return type incorrectly nested Iterator<Item = Iterator<String>>\nGenerated:\n{}",
        rust
    );

    assert!(
        rust.contains("type Item = String;"),
        "BUG: Associated type should be 'type Item = String;'\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_optional_param_with_iterator_return() {
    // Fibonacci generator with Optional parameter and Iterator return
    let python = r#"
from typing import Iterator, Optional

def fibonacci_generator(limit: Optional[int] = None) -> Iterator[int]:
    a, b = 0, 1
    count = 0
    while limit is None or count < limit:
        yield a
        a, b = b, a + b
        count += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Function signature should be impl Iterator<Item = i32>
    assert!(
        rust.contains("impl Iterator<Item = i32>") || rust.contains("impl Iterator<Item=i32>"),
        "BUG: fibonacci_generator return type should be 'impl Iterator<Item = i32>'\nGenerated:\n{}",
        rust
    );

    // Should NOT be nested
    assert!(
        !rust.contains("Iterator<Item = Iterator<"),
        "BUG: Return type should not nest Iterator\nGenerated:\n{}",
        rust
    );

    // Impl block should have type Item = i32
    assert!(
        rust.contains("type Item = i32;"),
        "BUG: Impl Iterator should have 'type Item = i32;'\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_compilation_no_e0107_error() {
    // Verify generated code compiles without E0107 error
    let python = r#"
from typing import Iterator

def simple_gen(n: int) -> Iterator[int]:
    i = 0
    while i < n:
        yield i
        i += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    // Write to temp file
    let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(rust.as_bytes()).expect("Failed to write");

    // Try to compile with rustc
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--crate-name=test_gen")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("rustc output:\n{}", stderr);

    // Should NOT have E0107 error (trait takes 0 generic arguments)
    assert!(
        !stderr.contains("E0107"),
        "BUG: Generated code has E0107 error (trait takes 0 generic arguments)\nrustc stderr:\n{}",
        stderr
    );

    // Should NOT have E0191 error (associated type Item must be specified)
    assert!(
        !stderr.contains("E0191"),
        "BUG: Generated code has E0191 error (associated type Item must be specified)\nrustc stderr:\n{}",
        stderr
    );
}

#[test]
fn test_generator_iterator_tuple_return_type() {
    // Generator returning tuple elements
    let python = r#"
from typing import Iterator, Tuple

def pair_gen(n: int) -> Iterator[Tuple[int, int]]:
    for i in range(n):
        yield (i, i * 2)
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have impl Iterator<Item = (i32, i32)> or similar tuple type
    // NOT Iterator<Item = Iterator<(i32, i32)>>
    let has_correct_return = rust.contains("impl Iterator<Item = (i32, i32)>")
        || rust.contains("impl Iterator<Item=(i32, i32)>");

    assert!(
        has_correct_return,
        "BUG: Should have tuple return type, not nested Iterator\nGenerated:\n{}",
        rust
    );

    // Definitely should NOT have nested Iterator
    assert!(
        !rust.contains("Iterator<Item = Iterator<"),
        "BUG: Return type incorrectly nested\nGenerated:\n{}",
        rust
    );
}

#[test]
#[ignore = "Complex List[T] element type - deferred"]
fn test_generator_iterator_list_return_type() {
    // Generator returning lists
    let python = r#"
from typing import Iterator, List

def batch_gen(n: int) -> Iterator[List[int]]:
    batch = []
    for i in range(n):
        batch.append(i)
        if len(batch) >= 3:
            yield batch
            batch = []
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have impl Iterator<Item = Vec<i32>>
    // NOT Iterator<Item = Iterator<Vec<i32>>>
    assert!(
        rust.contains("impl Iterator<Item = Vec<i32>>")
        || rust.contains("impl Iterator<Item=Vec<i32>>"),
        "BUG: Should have Vec<i32> element type\nGenerated:\n{}",
        rust
    );

    assert!(
        !rust.contains("Iterator<Item = Iterator<"),
        "BUG: Return type incorrectly nested\nGenerated:\n{}",
        rust
    );
}
