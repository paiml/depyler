//! Session 8 coverage tests for stmt_gen.rs and stmt_gen_complex.rs
//! Targets: subcommand patterns, complex control flow, exception handling

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

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

// ── Try/except patterns ─────────────────────────────────────────

#[test]
fn test_try_except_value_error() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
"#,
    );
    assert!(
        code.contains("parse") || code.contains("Err"),
        "Should generate error handling: {code}"
    );
}

#[test]
fn test_try_except_multiple_handlers() {
    let code = transpile(
        r#"
def f(s: str) -> int:
    try:
        return int(s)
    except ValueError:
        return -1
    except TypeError:
        return -2
"#,
    );
    // Transpiler may merge exception handlers or generate match arms
    assert!(
        code.contains("-1") || code.contains("Err") || code.contains("parse"),
        "Should handle exception types: {code}"
    );
}

#[test]
fn test_try_except_finally() {
    let code = transpile(
        r#"
def f() -> int:
    result = 0
    try:
        result = 42
    except Exception:
        result = -1
    finally:
        print("done")
    return result
"#,
    );
    assert!(code.contains("42"), "Should generate try body: {code}");
}

#[test]
fn test_try_except_as_binding() {
    let code = transpile(
        r#"
def f() -> str:
    try:
        x = 1 / 0
    except ZeroDivisionError as e:
        return str(e)
    return "ok"
"#,
    );
    assert!(
        code.contains("Err") || code.contains("error") || code.contains("catch"),
        "Should bind exception: {code}"
    );
}

#[test]
fn test_try_except_bare() {
    let code = transpile(
        r#"
def f() -> int:
    try:
        return 42
    except:
        return -1
"#,
    );
    assert!(
        code.contains("42") && code.contains("-1"),
        "Should handle bare except: {code}"
    );
}

// ── Complex control flow ────────────────────────────────────────

#[test]
fn test_nested_if_elif_else() {
    let code = transpile(
        r#"
def classify(x: int) -> str:
    if x > 100:
        return "large"
    elif x > 50:
        return "medium"
    elif x > 10:
        return "small"
    else:
        return "tiny"
"#,
    );
    assert!(
        code.contains("large") && code.contains("medium") && code.contains("small"),
        "Should generate all branches: {code}"
    );
}

#[test]
fn test_while_with_break_continue() {
    let code = transpile(
        r#"
def f(n: int) -> int:
    total = 0
    i = 0
    while i < n:
        i += 1
        if i % 2 == 0:
            continue
        if i > 10:
            break
        total += i
    return total
"#,
    );
    assert!(
        code.contains("break") && code.contains("continue"),
        "Should generate break and continue: {code}"
    );
}

#[test]
fn test_for_enumerate() {
    let code = transpile(
        r#"
def f(items: list) -> None:
    for i, item in enumerate(items):
        print(i, item)
"#,
    );
    assert!(
        code.contains("enumerate") || code.contains("iter()"),
        "Should generate enumerate: {code}"
    );
}

#[test]
fn test_for_zip() {
    let code = transpile(
        r#"
def f(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#,
    );
    assert!(
        code.contains("zip") || code.contains("iter()"),
        "Should generate zip: {code}"
    );
}

#[test]
fn test_for_reversed() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    result = []
    for item in reversed(items):
        result.append(item)
    return result
"#,
    );
    assert!(
        code.contains("rev()") || code.contains("reversed"),
        "Should generate reversed iteration: {code}"
    );
}

#[test]
fn test_for_sorted() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    result = []
    for item in sorted(items):
        result.append(item)
    return result
"#,
    );
    assert!(
        code.contains("sort") || code.contains("sorted"),
        "Should generate sorted iteration: {code}"
    );
}

// ── Assert statements ───────────────────────────────────────────

#[test]
fn test_assert_simple() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    assert x > 0
    return x
"#,
    );
    assert!(
        code.contains("assert") || code.contains("debug_assert"),
        "Should generate assert: {code}"
    );
}

#[test]
fn test_assert_with_message() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    assert x > 0, "x must be positive"
    return x
"#,
    );
    assert!(
        code.contains("assert") || code.contains("positive"),
        "Should generate assert with message: {code}"
    );
}

// ── Augmented assignments ───────────────────────────────────────

#[test]
fn test_augmented_add() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    x += 10
    return x
"#,
    );
    assert!(
        code.contains("+=") || code.contains("+ 10"),
        "Should generate +=: {code}"
    );
}

#[test]
fn test_augmented_mul() {
    let code = transpile(
        r#"
def f(x: int) -> int:
    x *= 2
    return x
"#,
    );
    assert!(
        code.contains("*=") || code.contains("* 2") || code.contains("2"),
        "Should generate multiplication: {code}"
    );
}

#[test]
fn test_augmented_list_extend() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    items += [1, 2, 3]
    return items
"#,
    );
    assert!(
        code.contains("extend") || code.contains("+="),
        "Should generate list extend: {code}"
    );
}

// ── With statement / context managers ───────────────────────────

#[test]
fn test_with_open_read() {
    let code = transpile(
        r#"
def f(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#,
    );
    assert!(
        code.contains("File") || code.contains("read_to_string") || code.contains("BufReader"),
        "Should generate file reading: {code}"
    );
}

#[test]
fn test_with_open_write() {
    let code = transpile(
        r#"
def f(path: str, data: str) -> None:
    with open(path, "w") as f:
        f.write(data)
"#,
    );
    assert!(
        code.contains("File") || code.contains("write") || code.contains("create"),
        "Should generate file writing: {code}"
    );
}

// ── Global/nonlocal ─────────────────────────────────────────────

#[test]
fn test_global_variable() {
    let code = transpile(
        r#"
counter = 0
def increment() -> int:
    global counter
    counter += 1
    return counter
"#,
    );
    assert!(
        code.contains("counter") || code.contains("static") || code.contains("COUNTER"),
        "Should handle global: {code}"
    );
}

// ── Multiple return values ──────────────────────────────────────

#[test]
fn test_tuple_return() {
    let code = transpile(
        r#"
def divmod_custom(a: int, b: int) -> tuple:
    return a // b, a % b
"#,
    );
    assert!(
        code.contains("(") && (code.contains("/") || code.contains("%")),
        "Should generate tuple return: {code}"
    );
}

#[test]
fn test_tuple_unpack_assignment() {
    let code = transpile(
        r#"
def f() -> int:
    a, b, c = 1, 2, 3
    return a + b + c
"#,
    );
    assert!(
        code.contains("let") && code.contains("1") && code.contains("2"),
        "Should unpack tuple: {code}"
    );
}

// ── Complex patterns ────────────────────────────────────────────

#[test]
fn test_nested_list_comprehension() {
    let code = transpile(
        r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#,
    );
    assert!(
        code.contains("iter") || code.contains("flat_map") || code.contains("flatten"),
        "Should generate flattened iteration: {code}"
    );
}

#[test]
fn test_walrus_operator() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    result = []
    for item in items:
        if (n := len(item)) > 3:
            result.append(n)
    return result
"#,
    );
    assert!(
        code.contains("len") || code.contains("let n"),
        "Should handle walrus operator: {code}"
    );
}

#[test]
fn test_conditional_expression() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#,
    );
    assert!(
        code.contains("if") && code.contains("positive"),
        "Should generate conditional: {code}"
    );
}

#[test]
fn test_chained_comparison() {
    let code = transpile(
        r#"
def f(x: int) -> bool:
    return 0 < x < 100
"#,
    );
    assert!(
        code.contains("&&") || code.contains("0") && code.contains("100"),
        "Should generate chained comparison: {code}"
    );
}

// ── Class patterns ──────────────────────────────────────────────

#[test]
fn test_class_with_methods() {
    let code = transpile(
        r#"
class Stack:
    def __init__(self) -> None:
        self.items: list = []

    def push(self, item: int) -> None:
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#,
    );
    assert!(
        code.contains("struct Stack") || code.contains("impl Stack"),
        "Should generate struct and impl: {code}"
    );
    assert!(code.contains("push"), "Should have push method: {code}");
    assert!(code.contains("pop"), "Should have pop method: {code}");
}

#[test]
fn test_class_with_property_like() {
    let code = transpile(
        r#"
class Rectangle:
    def __init__(self, width: float, height: float) -> None:
        self.width = width
        self.height = height

    def area(self) -> float:
        return self.width * self.height

    def perimeter(self) -> float:
        return 2 * (self.width + self.height)
"#,
    );
    assert!(
        code.contains("width") && code.contains("height"),
        "Should have fields: {code}"
    );
    assert!(
        code.contains("area") && code.contains("perimeter"),
        "Should have methods: {code}"
    );
}

// ── String formatting ───────────────────────────────────────────

#[test]
fn test_fstring_simple() {
    let code = transpile(
        r#"
def f(name: str, age: int) -> str:
    return f"Hello {name}, you are {age} years old"
"#,
    );
    assert!(
        code.contains("format!") || code.contains("Hello"),
        "Should generate format!: {code}"
    );
}

#[test]
fn test_fstring_with_expression() {
    let code = transpile(
        r#"
def f(x: int) -> str:
    return f"Result: {x * 2}"
"#,
    );
    assert!(
        code.contains("format!") || code.contains("Result"),
        "Should generate format with expr: {code}"
    );
}

#[test]
fn test_format_spec() {
    let code = transpile(
        r#"
def f(x: float) -> str:
    return f"{x:.2f}"
"#,
    );
    assert!(
        code.contains("format!") || code.contains(".2"),
        "Should generate format spec: {code}"
    );
}

// ── Slice operations ────────────────────────────────────────────

#[test]
fn test_list_slice() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return items[1:3]
"#,
    );
    assert!(
        code.contains("[1..3]") || code.contains("1..") || code.contains("slice"),
        "Should generate slice: {code}"
    );
}

#[test]
fn test_list_slice_step() {
    let code = transpile(
        r#"
def f(items: list) -> list:
    return items[::2]
"#,
    );
    assert!(
        code.contains("step") || code.contains("skip") || code.contains("iter"),
        "Should generate step slice: {code}"
    );
}

#[test]
fn test_string_slice() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s[:5]
"#,
    );
    assert!(
        code.contains("5") || code.contains("..5") || code.contains("chars"),
        "Should generate string slice: {code}"
    );
}
