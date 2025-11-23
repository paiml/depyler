use depyler_core::DepylerPipeline;

/// Helper function to transpile Python code and verify it compiles
fn transpile_and_verify(python: &str, test_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python)?;

    // Write to temp file and verify with rustc
    let temp_file = format!("/tmp/depyler_test_{}.rs", test_name);
    std::fs::write(&temp_file, &rust_code)?;

    // Check compilation (using --crate-type lib for quick validation)
    let output = std::process::Command::new("rustc")
        .args(["--crate-type", "lib", "--edition", "2021", &temp_file])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Compilation failed for {}: {}",
            test_name,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(rust_code)
}

/// Helper function to transpile Python code without verifying compilation
/// Used for code that requires external crates (like rand)
fn transpile_only(python: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pipeline = DepylerPipeline::new();
    let rust_code = pipeline.transpile(python)?;
    Ok(rust_code)
}

// ============================================================================
// Absolute Value Tests
// ============================================================================

#[test]
fn test_abs_variable() {
    let python = r#"
def test_abs(value: int) -> int:
    return abs(value)
"#;

    let rust = transpile_and_verify(python, "abs_variable").unwrap();
    assert!(rust.contains("value.abs()"));
}

#[test]
fn test_abs_negative_literal() {
    let python = r#"
def abs_negative() -> int:
    return abs(-42)
"#;

    let rust = transpile_and_verify(python, "abs_negative").unwrap();
    assert!(rust.contains("(-42 as i32).abs()"));
}

#[test]
fn test_abs_float() {
    let python = r#"
def abs_float(x: float) -> float:
    return abs(x)
"#;

    let rust = transpile_and_verify(python, "abs_float").unwrap();
    assert!(rust.contains("x.abs()"));
}

#[test]
fn test_abs_expression() {
    let python = r#"
def abs_expr(a: int, b: int) -> int:
    return abs(a - b)
"#;

    let rust = transpile_and_verify(python, "abs_expr").unwrap();
    assert!(rust.contains("(a - b).abs()"));
}

// ============================================================================
// Rounding Tests
// ============================================================================

#[test]
fn test_round_to_int() {
    let python = r#"
def round_to_int(x: float) -> int:
    return round(x)
"#;

    let rust = transpile_and_verify(python, "round_to_int").unwrap();
    assert!(rust.contains("x.round() as i32"));
}

#[test]
fn test_round_to_float() {
    let python = r#"
def round_to_float(x: float) -> float:
    return round(x)
"#;

    let rust = transpile_and_verify(python, "round_to_float").unwrap();
    assert!(rust.contains("x.round()"));
}

#[test]
fn test_round_literal() {
    let python = r#"
def round_literal() -> int:
    return round(3.7)
"#;

    let rust = transpile_and_verify(python, "round_literal").unwrap();
    assert!(rust.contains("(3.7 as f64).round() as i32"));
}

// ============================================================================
// Power/Exponentiation Tests
// ============================================================================

#[test]
fn test_pow_literal_exponent() {
    let python = r#"
def pow_literal() -> int:
    return pow(2, 10)
"#;

    let rust = transpile_and_verify(python, "pow_literal").unwrap();
    assert!(rust.contains("2_i32.pow(10 as u32)"));
}

#[test]
fn test_pow_variable_exponent() {
    let python = r#"
def pow_var(base: int, exp: int) -> int:
    return pow(base, exp)
"#;

    let rust = transpile_and_verify(python, "pow_var").unwrap();
    assert!(rust.contains("base.checked_pow(exp as u32)"));
}

#[test]
fn test_power_operator_literal() {
    let python = r#"
def power_op() -> int:
    return 2 ** 8
"#;

    let rust = transpile_and_verify(python, "power_op_lit").unwrap();
    assert!(rust.contains("2_i32.pow(8 as u32)"));
}

#[test]
fn test_power_operator_variable() {
    let python = r#"
def power_op(a: int, b: int) -> int:
    return a ** b
"#;

    let rust = transpile_and_verify(python, "power_op_var").unwrap();
    assert!(rust.contains("a.checked_pow(b as u32)"));
}

#[test]
fn test_pow_float() {
    let python = r#"
def pow_float(base: float, exp: float) -> float:
    return pow(base, exp)
"#;

    let rust = transpile_and_verify(python, "pow_float").unwrap();
    assert!(rust.contains("base.powf(exp)"));
}

// ============================================================================
// Min/Max Tests - Two Arguments
// ============================================================================

#[test]
fn test_max_two_ints() {
    let python = r#"
def max_two(a: int, b: int) -> int:
    return max(a, b)
"#;

    let rust = transpile_and_verify(python, "max_two_ints").unwrap();
    assert!(rust.contains("std::cmp::max(a, b)"));
}

#[test]
fn test_min_two_ints() {
    let python = r#"
def min_two(a: int, b: int) -> int:
    return min(a, b)
"#;

    let rust = transpile_and_verify(python, "min_two_ints").unwrap();
    assert!(rust.contains("std::cmp::min(a, b)"));
}

#[test]
fn test_max_two_floats() {
    let python = r#"
def max_two_f(a: float, b: float) -> float:
    return max(a, b)
"#;

    let rust = transpile_and_verify(python, "max_two_floats").unwrap();
    assert!(rust.contains("a.max(b)"));
}

#[test]
fn test_min_two_floats() {
    let python = r#"
def min_two_f(a: float, b: float) -> float:
    return min(a, b)
"#;

    let rust = transpile_and_verify(python, "min_two_floats").unwrap();
    assert!(rust.contains("a.min(b)"));
}

// ============================================================================
// Min/Max Tests - Multiple Arguments
// ============================================================================

#[test]
fn test_max_three_ints() {
    let python = r#"
def max_three() -> int:
    return max(10, 20, 5)
"#;

    let rust = transpile_and_verify(python, "max_three_ints").unwrap();
    assert!(rust.contains("std::cmp::max(std::cmp::max(10, 20), 5)"));
}

#[test]
fn test_min_three_floats() {
    let python = r#"
def min_three() -> float:
    return min(1.5, 2.7, 0.8)
"#;

    let rust = transpile_and_verify(python, "min_three_floats").unwrap();
    assert!(rust.contains(".min(2.7).min(0.8)")); // Check the chain, allow cast on first literal
}

#[test]
fn test_max_four_values() {
    let python = r#"
def max_four(a: int, b: int, c: int, d: int) -> int:
    return max(a, b, c, d)
"#;

    let rust = transpile_and_verify(python, "max_four").unwrap();
    assert!(rust.contains("std::cmp::max(std::cmp::max(std::cmp::max(a, b), c), d)"));
}

// ============================================================================
// Min/Max Tests - Collections
// ============================================================================

#[test]
fn test_max_list() {
    let python = r#"
def max_list(items: list[int]) -> int:
    return max(items)
"#;

    let rust = transpile_and_verify(python, "max_list").unwrap();
    assert!(rust.contains("items.iter().max().unwrap()"));
}

#[test]
fn test_min_list() {
    let python = r#"
def min_list(items: list[int]) -> int:
    return min(items)
"#;

    let rust = transpile_and_verify(python, "min_list").unwrap();
    assert!(rust.contains("items.iter().min().unwrap()"));
}

#[test]
fn test_max_float_list() {
    let python = r#"
def max_floats(items: list[float]) -> float:
    return max(items)
"#;

    let rust = transpile_and_verify(python, "max_floats").unwrap();
    assert!(rust.contains("items.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))"));
}

#[test]
fn test_min_float_list() {
    let python = r#"
def min_floats(items: list[float]) -> float:
    return min(items)
"#;

    let rust = transpile_and_verify(python, "min_floats").unwrap();
    assert!(rust.contains("items.iter().fold(f64::INFINITY, |a, &b| a.min(b))"));
}

// ============================================================================
// First/Last Element Tests
// ============================================================================

#[test]
fn test_first_element() {
    let python = r#"
def first(items: list[int]) -> int:
    return items[0]
"#;

    let rust = transpile_and_verify(python, "first").unwrap();
    assert!(rust.contains("*items.first().unwrap()"));
}

#[test]
fn test_last_element() {
    let python = r#"
def last(items: list[int]) -> int:
    return items[-1]
"#;

    let rust = transpile_and_verify(python, "last").unwrap();
    assert!(rust.contains("*items.last().unwrap()"));
}

#[test]
fn test_negative_index_second_last() {
    let python = r#"
def second_last(items: list[int]) -> int:
    return items[-2]
"#;

    let rust = transpile_and_verify(python, "second_last").unwrap();
    assert!(rust.contains("items[items.len() - 2")); // Allow with or without usize suffix
}

#[test]
fn test_negative_index_third_last() {
    let python = r#"
def third_last(items: list[float]) -> float:
    return items[-3]
"#;

    let rust = transpile_and_verify(python, "third_last").unwrap();
    assert!(rust.contains("items[items.len() - 3")); // Allow with or without usize suffix
}

// ============================================================================
// Random Tests
// ============================================================================

#[test]
fn test_randint() {
    let python = r#"
import random

def rand_int(start: int, end: int) -> int:
    return random.randint(start, end)
"#;

    let rust = transpile_only(python).unwrap();
    assert!(rust.contains("rand::thread_rng().gen_range(start..=end)"));
}

#[test]
fn test_randint_literal() {
    let python = r#"
import random

def rand_int() -> int:
    return random.randint(1, 100)
"#;

    let rust = transpile_only(python).unwrap();
    assert!(rust.contains("rand::thread_rng().gen_range(1..=100)"));
}

#[test]
fn test_uniform() {
    let python = r#"
import random

def rand_float(a: float, b: float) -> float:
    return random.uniform(a, b)
"#;

    let rust = transpile_only(python).unwrap();
    assert!(rust.contains("rand::thread_rng().gen_range((a as f64)..=(b as f64))"));
}

#[test]
fn test_random_choice() {
    let python = r#"
import random

def rand_choice(items: list[int]) -> int:
    return random.choice(items)
"#;

    let rust = transpile_only(python).unwrap();
    assert!(rust.contains("*items.choose(&mut rand::thread_rng()).unwrap()"));
}

#[test]
fn test_random_random() {
    let python = r#"
import random

def rand() -> float:
    return random.random()
"#;

    let rust = transpile_only(python).unwrap();
    assert!(rust.contains("rand::random::<f64>()"));
}

// ============================================================================
// Combined Math Operations
// ============================================================================

#[test]
fn test_abs_and_max() {
    let python = r#"
def abs_max(x: int, y: int) -> int:
    return max(abs(x), abs(y))
"#;

    let rust = transpile_and_verify(python, "abs_max").unwrap();
    assert!(rust.contains("std::cmp::max(x.abs(), y.abs())"));
}

#[test]
fn test_nested_round_abs() {
    let python = r#"
def nested(x: float) -> int:
    return abs(round(x))
"#;

    let rust = transpile_and_verify(python, "nested").unwrap();
    assert!(rust.contains("(x.round() as i32).abs()"));
}

#[test]
fn test_power_sum() {
    let python = r#"
def pythagorean(a: int, b: int) -> int:
    return a ** 2 + b ** 2
"#;

    let rust = transpile_and_verify(python, "power_sum").unwrap();
    assert!(rust.contains("a.pow(2 as u32) + b.pow(2 as u32)"));
}

#[test]
fn test_clamp_pattern() {
    let python = r#"
def clamp(x: int, lo: int, hi: int) -> int:
    return max(lo, min(x, hi))
"#;

    let rust = transpile_and_verify(python, "clamp").unwrap();
    assert!(rust.contains("std::cmp::max(lo, std::cmp::min(x, hi))"));
}

#[test]
fn test_floor_division() {
    let python = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;

    let rust = transpile_and_verify(python, "floor_div").unwrap();
    assert!(rust.contains("a / b"));
}

#[test]
fn test_float_division() {
    let python = r#"
def float_div(a: float, b: float) -> float:
    return a / b
"#;

    let rust = transpile_and_verify(python, "float_div").unwrap();
    assert!(rust.contains("a / b"));
}

#[test]
fn test_modulo() {
    let python = r#"
def mod_op(a: int, b: int) -> int:
    return a % b
"#;

    let rust = transpile_and_verify(python, "modulo").unwrap();
    assert!(rust.contains("a % b"));
}

#[test]
fn test_complex_expression() {
    let python = r#"
def complex_expr(a: int, b: int) -> int:
    return abs(a - b) ** 2 + max(a, b)
"#;

    let rust = transpile_and_verify(python, "complex_expr").unwrap();
    assert!(rust.contains("(a - b).abs().pow(2 as u32) + std::cmp::max(a, b)"));
}

#[test]
fn test_arithmetic_precedence() {
    let python = r#"
def precedence(a: int, b: int, c: int) -> int:
    return a + b * c
"#;

    let rust = transpile_and_verify(python, "precedence").unwrap();
    assert!(rust.contains("a + b * c"));
}
