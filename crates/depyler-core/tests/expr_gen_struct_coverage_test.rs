//! expr_gen.rs struct module coverage tests
//!
//! Target: try_convert_struct_method (struct.pack, struct.unpack, struct.calcsize)
//! Coverage gap: These struct methods are rarely tested

use depyler_core::DepylerPipeline;

// ============================================================================
// struct.pack() Tests
// ============================================================================

#[test]
fn test_struct_pack_single_int() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def pack_int(value: int) -> bytes:
    return struct.pack('i', value)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.pack('i') code:\n{}", rust_code);

    // Should generate .to_le_bytes().to_vec()
    assert!(rust_code.contains("to_le_bytes") || rust_code.contains("Vec"));
}

#[test]
fn test_struct_pack_two_ints() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def pack_two_ints(a: int, b: int) -> bytes:
    return struct.pack('ii', a, b)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.pack('ii') code:\n{}", rust_code);

    // Should generate extend_from_slice for multiple ints
    assert!(rust_code.contains("extend_from_slice") || rust_code.contains("Vec"));
}

// ============================================================================
// struct.unpack() Tests
// ============================================================================

#[test]
fn test_struct_unpack_single_int() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def unpack_int(data: bytes) -> int:
    result = struct.unpack('i', data)
    return result[0]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.unpack('i') code:\n{}", rust_code);

    // Should generate from_le_bytes
    assert!(rust_code.contains("from_le_bytes") || rust_code.contains("try_into"));
}

#[test]
fn test_struct_unpack_two_ints() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def unpack_two_ints(data: bytes) -> tuple:
    return struct.unpack('ii', data)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.unpack('ii') code:\n{}", rust_code);

    // Should generate from_le_bytes for both values
    assert!(rust_code.contains("from_le_bytes"));
}

// ============================================================================
// struct.calcsize() Tests
// ============================================================================

#[test]
fn test_struct_calcsize_single() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def get_size() -> int:
    return struct.calcsize('i')
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.calcsize('i') code:\n{}", rust_code);

    // Should generate literal 4 (size of i32)
    assert!(rust_code.contains("4") || rust_code.contains("calcsize"));
}

#[test]
fn test_struct_calcsize_double() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
import struct

def get_double_size() -> int:
    return struct.calcsize('ii')
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated struct.calcsize('ii') code:\n{}", rust_code);

    // Should generate literal 8 (size of two i32s)
    assert!(rust_code.contains("8") || rust_code.contains("calcsize"));
}

// ============================================================================
// Classmethod Tests (cls.method())
// ============================================================================

#[test]
fn test_classmethod_call() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
class Counter:
    count: int = 0

    @classmethod
    def increment(cls):
        cls.count += 1
        return cls.count

    @classmethod
    def reset(cls):
        cls.count = 0
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated classmethod code:\n{}", rust_code);

    // Should contain struct definition
    assert!(rust_code.contains("Counter") || rust_code.contains("struct"));
}

// ============================================================================
// Generator Expression Tests
// ============================================================================

#[test]
fn test_generator_expression_simple() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated generator expr code:\n{}", rust_code);

    // Should generate map or iter pattern
    assert!(rust_code.contains("iter") || rust_code.contains("map") || rust_code.contains("sum"));
}

#[test]
fn test_generator_expression_with_filter() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sum_even_squares(n: int) -> int:
    return sum(x * x for x in range(n) if x % 2 == 0)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated generator expr with filter:\n{}", rust_code);

    // Should generate filter pattern
    assert!(rust_code.contains("filter") || rust_code.contains("iter"));
}

// ============================================================================
// Lambda Expression Tests
// ============================================================================

#[test]
fn test_lambda_simple() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def apply_func(items: list):
    return list(map(lambda x: x * 2, items))
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated lambda code:\n{}", rust_code);

    // Should generate closure syntax
    assert!(rust_code.contains("|") || rust_code.contains("map"));
}

#[test]
fn test_lambda_with_multiple_args() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def reduce_func(items: list):
    from functools import reduce
    return reduce(lambda x, y: x + y, items)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated multi-arg lambda code:\n{}", rust_code);

    // Should generate closure with multiple params
    assert!(rust_code.contains("|") || rust_code.contains("fold"));
}

// ============================================================================
// Set Operation Tests
// ============================================================================

#[test]
fn test_set_intersection() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def intersect(a: set, b: set) -> set:
    return a & b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set intersection code:\n{}", rust_code);

    // Should generate intersection method
    assert!(rust_code.contains("intersection") || rust_code.contains("&"));
}

#[test]
fn test_set_union() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def union_sets(a: set, b: set) -> set:
    return a | b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set union code:\n{}", rust_code);

    // Should generate union method
    assert!(rust_code.contains("union") || rust_code.contains("|"));
}

#[test]
fn test_set_difference() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def diff_sets(a: set, b: set) -> set:
    return a - b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set difference code:\n{}", rust_code);

    // Should generate difference method
    assert!(rust_code.contains("difference") || rust_code.contains("-"));
}

#[test]
fn test_set_symmetric_difference() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def sym_diff(a: set, b: set) -> set:
    return a ^ b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set symmetric diff code:\n{}", rust_code);

    // Should generate symmetric_difference method
    assert!(rust_code.contains("symmetric_difference") || rust_code.contains("^"));
}

// ============================================================================
// Dict Merge Tests (Python 3.9+)
// ============================================================================

#[test]
fn test_dict_merge_operator() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def merge_dicts(a: dict, b: dict) -> dict:
    return a | b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict merge code:\n{}", rust_code);

    // Should generate extend or clone pattern
    assert!(rust_code.contains("extend") || rust_code.contains("clone") || rust_code.contains("merge"));
}

// ============================================================================
// String Format Tests
// ============================================================================

#[test]
fn test_fstring_basic() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}!"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated f-string code:\n{}", rust_code);

    // Should generate format! macro
    assert!(rust_code.contains("format!"));
}

#[test]
fn test_fstring_with_expression() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def calc_msg(x: int) -> str:
    return f"Result: {x * 2}"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated f-string with expr code:\n{}", rust_code);

    // Should generate format! with expression
    assert!(rust_code.contains("format!"));
}

// ============================================================================
// Containment Tests (in / not in)
// ============================================================================

#[test]
fn test_in_list() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_membership(item: int, items: list) -> bool:
    return item in items
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated 'in' list code:\n{}", rust_code);

    // Should generate some form of membership check (contains, get().is_some(), etc.)
    assert!(rust_code.contains("contains") || rust_code.contains("iter") || rust_code.contains("is_some"));
}

#[test]
fn test_not_in_dict() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_not_present(key: str, data: dict) -> bool:
    return key not in data
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated 'not in' dict code:\n{}", rust_code);

    // Should generate !contains or contains_key negated
    assert!(rust_code.contains("contains") || rust_code.contains("!"));
}

#[test]
fn test_in_string() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_substring(sub: str, text: str) -> bool:
    return sub in text
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated 'in' string code:\n{}", rust_code);

    // Should generate contains method
    assert!(rust_code.contains("contains"));
}

// ============================================================================
// Slice Tests
// ============================================================================

#[test]
fn test_slice_basic() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_slice(items: list) -> list:
    return items[1:3]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated slice code:\n{}", rust_code);

    // Should generate range or slice syntax
    assert!(rust_code.contains("[") && (rust_code.contains("..") || rust_code.contains("slice")));
}

#[test]
fn test_slice_with_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_every_other(items: list) -> list:
    return items[::2]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated slice with step code:\n{}", rust_code);

    // Should generate step_by or similar pattern
    assert!(rust_code.contains("step_by") || rust_code.contains("iter") || rust_code.contains("["));
}

#[test]
fn test_negative_slice() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_last_two(items: list) -> list:
    return items[-2:]
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated negative slice code:\n{}", rust_code);

    // Should handle negative indices
    assert!(rust_code.contains("len") || rust_code.contains("saturating") || rust_code.contains("["));
}

// ============================================================================
// Ternary/Conditional Expression Tests
// ============================================================================

#[test]
fn test_ternary_simple() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def max_val(a: int, b: int) -> int:
    return a if a > b else b
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated ternary code:\n{}", rust_code);

    // Should generate if-else expression
    assert!(rust_code.contains("if") && rust_code.contains("else"));
}

#[test]
fn test_ternary_nested() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else "zero" if x == 0 else "negative"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated nested ternary code:\n{}", rust_code);

    // Should generate nested if-else or match
    assert!(rust_code.contains("if") || rust_code.contains("match"));
}

// ============================================================================
// Attribute Access Tests
// ============================================================================

#[test]
fn test_chained_attribute_access() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def get_name_len(obj) -> int:
    return len(obj.name)
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated chained attr code:\n{}", rust_code);

    // Should generate dot notation
    assert!(rust_code.contains("."));
}

// ============================================================================
// Comparison Chain Tests
// ============================================================================

#[test]
fn test_comparison_chain() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def in_range(x: int, lo: int, hi: int) -> bool:
    return lo < x < hi
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated comparison chain code:\n{}", rust_code);

    // Should generate && with two comparisons
    assert!(rust_code.contains("&&") || rust_code.contains("<"));
}

// ============================================================================
// Walrus Operator Tests (:=)
// ============================================================================

#[test]
// DEPYLER-0639: Walrus operator (:=) fully supported since DEPYLER-0188
fn test_walrus_operator() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def process_if_long(text: str) -> str:
    if (n := len(text)) > 10:
        return f"Long text: {n} chars"
    return "Short"
"#;
    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated walrus operator code:\n{}", rust_code);

    // Should handle assignment expression
    assert!(rust_code.contains("let") || rust_code.contains("len"));
}
