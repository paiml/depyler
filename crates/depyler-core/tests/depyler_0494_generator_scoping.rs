//! DEPYLER-0494: Generator Variable Scoping Bug
//!
//! Tests that Python generators with yield transpile to correct Rust state machines
//! with variables accessible across state transitions.
//!
//! BUG: Variables declared in state:0 are not accessible in state:1
//! Expected: Generator variables stored as struct fields, accessed via self

use depyler_core::DepylerPipeline;
use std::io::Write;

#[test]
fn test_generator_variable_scoping() {
    // Simple generator with variables that cross yield boundary
    let python = r#"
def fibonacci_gen():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should contain state struct with a and b fields
    assert!(
        rust.contains("a:") && rust.contains("b:"),
        "BUG: State struct must contain generator variables as fields\nGenerated:\n{}",
        rust
    );

    // Should access via self.a, self.b
    assert!(
        rust.contains("self.a") && rust.contains("self.b"),
        "BUG: Must access generator variables via self\nGenerated:\n{}",
        rust
    );

    // Should NOT declare as local variables in match arm
    // (This would cause scoping issues)
    let has_local_decl = rust.contains("let (mut a, mut b) = (0, 1);");
    let has_match_state = rust.contains("match self.state");

    if has_local_decl && has_match_state {
        // Check if the declaration is INSIDE a match arm (bad)
        // vs in fn fibonacci_gen() body (acceptable)
        let match_idx = rust.find("match self.state").unwrap_or(usize::MAX);
        let decl_idx = rust.find("let (mut a, mut b) = (0, 1);").unwrap_or(0);

        assert!(
            decl_idx < match_idx,
            "BUG: Generator variables declared as match arm locals (wrong scope)\nGenerated:\n{}",
            rust
        );
    }
}

#[test]
fn test_generator_state_struct_fields() {
    // Test that generator state struct has correct fields
    let python = r#"
def counter_gen(start=0):
    count = start
    while True:
        yield count
        count += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have count field in state struct
    assert!(
        rust.contains("count:"),
        "BUG: State struct missing generator variable 'count'\nGenerated:\n{}",
        rust
    );

    // Should initialize count field (not local variable)
    assert!(
        rust.contains("self.count =") || rust.contains("count: "),
        "BUG: count not initialized as struct field\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_multiple_variables() {
    // Test generator with multiple variables crossing yield
    let python = r#"
def multi_gen():
    a, b, c = 1, 2, 3
    while True:
        yield a
        a, b, c = b, c, a + b + c
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // All three variables should be struct fields
    assert!(
        rust.contains("a:") && rust.contains("b:") && rust.contains("c:"),
        "BUG: State struct missing generator variables (a, b, c)\nGenerated:\n{}",
        rust
    );

    // Should access all via self
    assert!(
        rust.contains("self.a") && rust.contains("self.b") && rust.contains("self.c"),
        "BUG: Generator variables not accessed via self\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_with_parameters() {
    // Test that parameters AND generator variables are both handled
    let python = r#"
def range_gen(start, end):
    current = start
    while current < end:
        yield current
        current += 1
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // Should have current as struct field (generator variable)
    assert!(
        rust.contains("current:"),
        "BUG: State struct missing generator variable 'current'\nGenerated:\n{}",
        rust
    );

    // Should also have parameters
    assert!(
        rust.contains("start:") && rust.contains("end:"),
        "BUG: State struct missing parameters\nGenerated:\n{}",
        rust
    );

    // current should be initialized from parameter
    assert!(
        rust.contains("current: start") || rust.contains("self.current = start"),
        "BUG: current not initialized from start parameter\nGenerated:\n{}",
        rust
    );
}

#[test]
#[ignore] // Enable after implementation
fn test_generator_compiles() {
    // CRITICAL: Generated code MUST compile
    let python = r#"
def fib_gen(limit=None):
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

    // Write to temp file and compile
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(rust.as_bytes()).unwrap();
    file.flush().unwrap();

    // Must compile without errors
    let output = std::process::Command::new("rustc")
        .arg("--crate-type=lib")
        .arg("--deny=warnings")
        .arg(file.path())
        .output()
        .expect("Failed to run rustc");

    assert!(
        output.status.success(),
        "BUG: Generated code does not compile\n\nGenerated:\n{}\n\nErrors:\n{}",
        rust,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_fibonacci_generator_specific() {
    // Test the EXACT case from fibonacci.rs that's failing
    let python = r#"
def fibonacci_generator(limit=None):
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

    // Specific assertions for fibonacci case
    assert!(
        rust.contains("a:") && rust.contains("b:") && rust.contains("count:"),
        "BUG: FibonacciGeneratorState struct missing fields a, b, count\nGenerated:\n{}",
        rust
    );

    // Should NOT have the broken pattern
    assert!(
        !rust.contains("let result = a;") || rust.contains("self.a"),
        "BUG: Accessing 'a' without self qualifier (scoping error)\nGenerated:\n{}",
        rust
    );

    // Should access count via self
    assert!(
        rust.contains("self.count"),
        "BUG: count not accessed via self\nGenerated:\n{}",
        rust
    );
}

#[test]
fn test_generator_yield_value_is_correct() {
    // Test that the yielded value is correctly extracted
    let python = r#"
def simple_yield():
    x = 42
    yield x
    x = 99
    yield x
"#;

    let compiler = DepylerPipeline::new();
    let rust = compiler.transpile(python).expect("Transpilation failed");

    println!("Generated Rust code:\n{}", rust);

    // x should be a struct field
    assert!(
        rust.contains("x:"),
        "BUG: State struct missing variable 'x'\nGenerated:\n{}",
        rust
    );

    // Should yield self.x
    assert!(
        rust.contains("self.x") || rust.contains("return Some(x)"),
        "BUG: yield not returning correct value\nGenerated:\n{}",
        rust
    );
}
