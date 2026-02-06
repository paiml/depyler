//! Session 12 Batch 92: Direct rules convert deep cold paths 8
//!
//! Targets direct_rules_convert.rs which has the worst coverage (57.62% line).
//! Focuses on: complex assignment targets, starred expressions,
//! augmented assignment on subscripts, and attribute assignment patterns.

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

// ===== Attribute assignment =====

#[test]
fn test_s12_b92_self_attr_assign() {
    let code = r#"
class Point:
    def __init__(self, x: float, y: float):
        self.x = x
        self.y = y

    def move_by(self, dx: float, dy: float):
        self.x += dx
        self.y += dy

    def reset(self):
        self.x = 0.0
        self.y = 0.0
"#;
    let result = transpile(code);
    assert!(result.contains("Point"), "Got: {}", result);
}

#[test]
fn test_s12_b92_self_list_attr() {
    let code = r#"
class TaskList:
    def __init__(self):
        self.tasks = []
        self.done = []

    def add(self, task: str):
        self.tasks.append(task)

    def complete(self, idx: int):
        task = self.tasks.pop(idx)
        self.done.append(task)
"#;
    let result = transpile(code);
    assert!(result.contains("TaskList"), "Got: {}", result);
}

#[test]
fn test_s12_b92_self_dict_attr() {
    let code = r#"
class Config:
    def __init__(self):
        self.settings = {}

    def set(self, key: str, value: str):
        self.settings[key] = value

    def get(self, key: str) -> str:
        return self.settings.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

// ===== Complex assignment patterns =====

#[test]
fn test_s12_b92_swap_values() {
    let code = r#"
def sort_pair(a: int, b: int) -> tuple:
    if a > b:
        a, b = b, a
    return (a, b)
"#;
    let result = transpile(code);
    assert!(result.contains("fn sort_pair"), "Got: {}", result);
}

#[test]
fn test_s12_b92_multi_assign_from_split() {
    let code = r#"
def parse_name(full_name: str) -> tuple:
    parts = full_name.split(" ", 1)
    first = parts[0]
    last = parts[1] if len(parts) > 1 else ""
    return (first, last)
"#;
    let result = transpile(code);
    assert!(result.contains("fn parse_name"), "Got: {}", result);
}

#[test]
fn test_s12_b92_augmented_on_subscript() {
    let code = r#"
def accumulate(counts: dict, items: list):
    for item in items:
        if item not in counts:
            counts[item] = 0
        counts[item] += 1
"#;
    let result = transpile(code);
    assert!(result.contains("fn accumulate"), "Got: {}", result);
}

#[test]
fn test_s12_b92_augmented_on_list_index() {
    let code = r#"
def scale_vector(vec: list, factor: float):
    for i in range(len(vec)):
        vec[i] *= factor
"#;
    let result = transpile(code);
    assert!(result.contains("fn scale_vector"), "Got: {}", result);
}

#[test]
fn test_s12_b92_nested_dict_assign() {
    let code = r##"
def build_tree(pairs: list) -> dict:
    tree = {}
    for parent, child in pairs:
        if parent not in tree:
            tree[parent] = []
        tree[parent].append(child)
    return tree
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_tree"), "Got: {}", result);
}

// ===== Delete statement patterns =====

#[test]
fn test_s12_b92_del_from_dict() {
    let code = r#"
def remove_keys(d: dict, keys: list) -> dict:
    for key in keys:
        if key in d:
            del d[key]
    return d
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_keys"), "Got: {}", result);
}

#[test]
fn test_s12_b92_del_from_list() {
    let code = r#"
def remove_first(items: list, target: int) -> list:
    for i in range(len(items)):
        if items[i] == target:
            del items[i]
            break
    return items
"#;
    let result = transpile(code);
    assert!(result.contains("fn remove_first"), "Got: {}", result);
}

// ===== Complex expression in assignment =====

#[test]
fn test_s12_b92_conditional_assign() {
    let code = r#"
def pick(a: int, b: int, use_first: bool) -> int:
    result = a if use_first else b
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn pick"), "Got: {}", result);
}

#[test]
fn test_s12_b92_comprehension_assign() {
    let code = r#"
def process(items: list) -> list:
    evens = [x for x in items if x % 2 == 0]
    doubles = [x * 2 for x in evens]
    return doubles
"#;
    let result = transpile(code);
    assert!(result.contains("fn process"), "Got: {}", result);
}

#[test]
fn test_s12_b92_dict_comp_assign() {
    let code = r#"
def index_items(items: list) -> dict:
    lookup = {item: i for i, item in enumerate(items)}
    return lookup
"#;
    let result = transpile(code);
    assert!(result.contains("fn index_items"), "Got: {}", result);
}

#[test]
fn test_s12_b92_set_comp_assign() {
    let code = r#"
def unique_lengths(words: list) -> set:
    lengths = {len(w) for w in words}
    return lengths
"#;
    let result = transpile(code);
    assert!(result.contains("fn unique_lengths"), "Got: {}", result);
}

// ===== Complex class assignment patterns =====

#[test]
fn test_s12_b92_class_with_init_logic() {
    let code = r#"
class Matrix:
    def __init__(self, rows: int, cols: int):
        self.rows = rows
        self.cols = cols
        self.data = []
        for i in range(rows):
            row = [0] * cols
            self.data.append(row)

    def set(self, r: int, c: int, val: int):
        self.data[r][c] = val

    def get(self, r: int, c: int) -> int:
        return self.data[r][c]

    def fill(self, val: int):
        for i in range(self.rows):
            for j in range(self.cols):
                self.data[i][j] = val
"#;
    let result = transpile(code);
    assert!(result.contains("Matrix"), "Got: {}", result);
}

#[test]
fn test_s12_b92_multiple_attr_types() {
    let code = r##"
class UserProfile:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.tags = []
        self.metadata = {}
        self.active = True

    def add_tag(self, tag: str):
        if tag not in self.tags:
            self.tags.append(tag)

    def set_meta(self, key: str, value: str):
        self.metadata[key] = value

    def deactivate(self):
        self.active = False
"##;
    let result = transpile(code);
    assert!(result.contains("UserProfile"), "Got: {}", result);
}

// ===== Augmented assignment operators =====

#[test]
fn test_s12_b92_aug_floor_div() {
    let code = r#"
def halve(n: int) -> int:
    n //= 2
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn halve"), "Got: {}", result);
}

#[test]
fn test_s12_b92_aug_modulo() {
    let code = r#"
def wrap(n: int, limit: int) -> int:
    n %= limit
    return n
"#;
    let result = transpile(code);
    assert!(result.contains("fn wrap"), "Got: {}", result);
}

#[test]
fn test_s12_b92_aug_power() {
    let code = r#"
def square_in_place(x: float) -> float:
    x **= 2
    return x
"#;
    let result = transpile(code);
    assert!(result.contains("fn square_in_place"), "Got: {}", result);
}

#[test]
fn test_s12_b92_aug_bitwise() {
    let code = r#"
def set_flag(flags: int, bit: int) -> int:
    flags |= (1 << bit)
    return flags
"#;
    let result = transpile(code);
    assert!(result.contains("fn set_flag"), "Got: {}", result);
}
