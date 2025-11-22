//! DEPYLER-0364: Keyword Arguments End-to-End Transpilation Tests
//!
//! Tests to verify that keyword arguments (kwargs) are correctly transpiled from Python
//! to Rust throughout the entire compilation pipeline (AST→HIR→Rust code generation).
//!
//!
use depyler_core::DepylerPipeline;

#[test]
fn test_kwargs_mixed_positional_and_named() {
    let python = r#"
def greet(name: str, greeting: str = "Hello") -> str:
    return f"{greeting}, {name}!"

def test() -> str:
    return greet("Alice", greeting="Hi")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains(r#"greet("Alice".to_string(), "Hi".to_string())"#));
}

#[test]
fn test_kwargs_all_named_arguments() {
    let python = r#"
def configure(width: int, height: int, title: str) -> dict:
    return {"width": width, "height": height, "title": title}

def test() -> dict:
    return configure(width=800, height=600, title="My App")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains(r#"configure(800, 600, "My App".to_string())"#));
}

#[test]
fn test_kwargs_multiple_named_after_positional() {
    let python = r#"
def calculate(a: int, b: int, operation: str = "add", verbose: bool = False) -> int:
    if operation == "add":
        result = a + b
    else:
        result = a - b
    if verbose:
        print(f"Result: {result}")
    return result

def test() -> int:
    return calculate(10, 20, operation="add", verbose=True)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains(r#"calculate(10, 20, "add", true)"#));
}

#[test]
fn test_kwargs_method_calls_with_named_args() {
    let python = r#"
class MyObject:
    def setup(self, mode: str = "basic", timeout: int = 10, retry: bool = False):
        self.mode = mode
        self.timeout = timeout
        self.retry = retry

def test():
    obj = MyObject()
    obj.setup(mode="advanced", timeout=30, retry=True)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains(r#".setup("advanced".to_string(), 30, true)"#));
}

#[test]
fn test_kwargs_nested_function_calls() {
    let python = r#"
def inner(x: int, y: int) -> int:
    return x + y

def outer(inner_result: int, scale: float = 1.0, offset: int = 0) -> int:
    return inner_result * int(scale) + offset

def test() -> int:
    return outer(inner(x=10, y=20), scale=2.0, offset=inner(x=5, y=5))
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("outer(inner(10, 20), 2.0, inner(5, 5))"));
}

#[test]
fn test_kwargs_only_positional_args() {
    let python = r#"
def add(a: int, b: int) -> int:
    return a + b

def test() -> int:
    return add(5, 3)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("add(5, 3)"));
}

#[test]
fn test_kwargs_with_complex_expressions() {
    let python = r#"
def configure(width: int, height: int, enabled: bool, title: str) -> dict:
    return {"width": width, "height": height, "enabled": enabled, "title": title}

def get_height() -> int:
    return 600

def test() -> dict:
    return configure(
        width=100 + 200,
        height=get_height(),
        enabled=True and not False,
        title="App " + str(42)
    )
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(
        rust_code.contains("configure(")
            && rust_code.contains("100")
            && rust_code.contains("200")
            && rust_code.contains("get_height()")
            && rust_code.contains("true")
    );
}

#[test]
fn test_kwargs_string_literals_converted_properly() {
    let python = r#"
def greet(name: str, greeting: str) -> str:
    return f"{greeting}, {name}!"

def test() -> str:
    return greet(greeting="Hello", name="Alice")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains(r#"greet("Alice".to_string(), "Hello".to_string())"#));
}

#[test]
fn test_kwargs_builtin_functions_with_kwargs() {
    let python = r#"
def test() -> str:
    # Python's open() with kwargs
    with open("data.txt", mode="r", encoding="utf-8") as f:
        return f.read()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    // Note: Python's open() with mode/encoding kwargs gets transpiled to Rust's File::open()
    // which only accepts a path. Mode and encoding are handled differently in Rust.
    // This is a semantic transformation, not a direct kwargs-to-args conversion.
    // We just check that the file path is present.
    assert!(
        rust_code.contains(r#"File::open("data.txt""#),
        "Generated code should contain File::open with the path:\n{}",
        rust_code
    );
}

#[test]
fn test_kwargs_empty_function_call_no_args() {
    let python = r#"
def no_params() -> int:
    return 42

def test() -> int:
    return no_params()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("no_params()"));
}

#[test]
fn test_kwargs_reordered_arguments() {
    let python = r#"
def format_message(name: str, age: int, city: str, country: str) -> str:
    return f"{name} is {age} years old from {city}, {country}"

def test() -> str:
    return format_message(country="USA", name="Alice", city="New York", age=30)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();

    // Check that the arguments appear in the correct order (ignoring whitespace/formatting)
    // Expected order: name="Alice", age=30, city="New York", country="USA"
    let call_start = rust_code
        .find("format_message(")
        .expect("format_message call not found");
    let call_section = &rust_code[call_start..];

    // Find the positions of each argument in the call
    let alice_pos = call_section
        .find(r#""Alice".to_string()"#)
        .expect("Alice not found");
    let age_pos = call_section.find("30").expect("30 not found");
    let newyork_pos = call_section
        .find(r#""New York".to_string()"#)
        .expect("New York not found");
    let usa_pos = call_section
        .find(r#""USA".to_string()"#)
        .expect("USA not found");

    // Verify they appear in the correct order
    assert!(alice_pos < age_pos, "Alice should come before age");
    assert!(age_pos < newyork_pos, "age should come before New York");
    assert!(newyork_pos < usa_pos, "New York should come before USA");
}

#[test]
fn test_kwargs_mixed_positional_and_reordered_named() {
    let python = r#"
def build_url(protocol: str, host: str, port: int, path: str) -> str:
    return f"{protocol}://{host}:{port}/{path}"

def test() -> str:
    return build_url("https", "example.com", path="api", port=443)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);

    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    let rust_code = result.unwrap();

    let call_start = rust_code
        .find("build_url(")
        .expect("build_url call not found");
    let call_section = &rust_code[call_start..];

    // Find the positions of each argument in the call
    let https_pos = call_section
        .find(r#""https".to_string()"#)
        .expect("https not found");
    let example_pos = call_section
        .find(r#""example.com".to_string()"#)
        .expect("example.com not found");
    let port_pos = call_section.find("443").expect("443 not found");
    let api_pos = call_section
        .find(r#""api".to_string()"#)
        .expect("api not found");

    // Verify they appear in the correct order: protocol, host, port, path
    assert!(
        https_pos < example_pos,
        "https should come before example.com"
    );
    assert!(example_pos < port_pos, "example.com should come before 443");
    assert!(
        port_pos < api_pos,
        "443 should come before api (kwargs reordered)"
    );
}
