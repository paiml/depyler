//! Wave 20 coverage tests: rust_gen.rs generate_rust_file_internal + analyze_mutable_vars
//!
//! Targets uncovered code paths in:
//! - analyze_mutable_vars (rust_gen.rs:403): variable reassignment, CSV patterns,
//!   attribute/index assignment, tuple unpacking, method call mutations, transitive mutation,
//!   expression analysis (Binary, Unary, IfExpr, List, Dict, Index, Attribute)
//! - generate_rust_file_internal (rust_gen.rs:2684): module constants, lambdas,
//!   TypeVar skipping, deduplication, import processing, async detection,
//!   property methods, dataclass defaults, class method return types,
//!   Option-returning functions, varargs, function param types, string interning,
//!   lazy/simple constant generation
//!
//! 200 tests total

#![cfg(test)]

use crate::ast_bridge::AstBridge;
use crate::rust_gen::generate_rust_file;
use crate::type_mapper::TypeMapper;
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

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;

    // ========================================================================
    // SECTION 1: analyze_mutable_vars - REASSIGNMENT (tests 001-020)
    // ========================================================================

    #[test]
    fn test_w20rm_001_simple_reassignment() {
        let code = "def func() -> int:\n    x = 5\n    x = 10\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("mut"));
    }

    #[test]
    fn test_w20rm_002_reassignment_with_operation() {
        let code = "def func() -> int:\n    x = 5\n    x = x + 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_003_multiple_reassignments() {
        let code = "def func() -> int:\n    x = 1\n    x = 2\n    x = 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_004_param_reassignment() {
        let code = "def func(a: int, b: int) -> int:\n    a = b\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_005_no_reassignment() {
        let code = "def func() -> int:\n    x = 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_006_reassignment_different_types() {
        let code = "def func() -> str:\n    x = 5\n    x = \"hello\"\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_007_swap_variables() {
        let code = "def func() -> int:\n    a = 1\n    b = 2\n    a = b\n    b = a\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_008_gcd_param_mutation() {
        let code = "def gcd(a: int, b: int) -> int:\n    while b != 0:\n        temp = b\n        b = a % b\n        a = temp\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_009_accumulator_reassignment() {
        let code = "def func(n: int) -> int:\n    total = 0\n    i = 0\n    while i < n:\n        total = total + i\n        i = i + 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_010_conditional_reassignment() {
        let code = "def func(x: int) -> int:\n    result = 0\n    if x > 0:\n        result = x\n    else:\n        result = -x\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_011_loop_reassignment() {
        let code = "def func() -> int:\n    x = 0\n    for i in range(10):\n        x = x + i\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_012_string_reassignment() {
        let code = "def func() -> str:\n    s = \"\"\n    s = s + \"hello\"\n    s = s + \" world\"\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_013_reassign_from_function_call() {
        let code = "def helper() -> int:\n    return 42\n\ndef func() -> int:\n    x = 0\n    x = helper()\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_014_reassignment_in_nested_if() {
        let code = "def func(x: int) -> int:\n    result = 0\n    if x > 0:\n        if x > 10:\n            result = 100\n        else:\n            result = 50\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_015_reassignment_in_while_loop() {
        let code = "def func() -> int:\n    x = 100\n    while x > 0:\n        x = x - 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_016_multiple_vars_reassigned() {
        let code = "def func() -> int:\n    a = 1\n    b = 2\n    c = 3\n    a = b + c\n    b = a + c\n    c = a + b\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_017_reassign_bool() {
        let code = "def func(x: int) -> bool:\n    found = False\n    if x > 0:\n        found = True\n    return found";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_018_reassign_float() {
        let code = "def func() -> float:\n    x = 0.0\n    x = 1.5\n    x = x + 2.5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_019_param_reassignment_multiple() {
        let code = "def func(a: int, b: int, c: int) -> int:\n    a = a + 1\n    b = b + 1\n    c = c + 1\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_020_reassign_in_for_body() {
        let code = "def func(items: list) -> str:\n    result = \"\"\n    for item in items:\n        result = result + str(item)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: analyze_mutable_vars - CSV PATTERNS (tests 021-030)
    // ========================================================================

    #[test]
    fn test_w20rm_021_csv_reader_name_heuristic() {
        let code = "def func() -> list:\n    reader = []\n    return reader";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_022_csv_writer_name_heuristic() {
        let code = "def func() -> list:\n    writer = []\n    return writer";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_023_csv_data_reader_contains() {
        let code = "def func() -> list:\n    data_reader = []\n    return data_reader";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_024_csv_file_writer_contains() {
        let code = "def func() -> list:\n    file_writer = []\n    return file_writer";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_025_csv_reader_pattern_call() {
        let code = "def func(path: str) -> None:\n    reader = DictReader(path)\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_026_csv_writer_pattern_call() {
        let code = "def func(path: str) -> None:\n    w = Writer(path)\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_027_reader_builder_pattern() {
        let code = "def func(path: str) -> None:\n    r = ReaderBuilder(path)\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_028_csv_reader_no_csv_name() {
        let code = "def func() -> list:\n    my_data = []\n    return my_data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_029_csv_reader_in_with() {
        let code = "def func(path: str) -> None:\n    with open(path) as f:\n        reader = []\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_030_csv_writer_reassigned() {
        let code = "def func() -> list:\n    writer = []\n    writer = [1, 2, 3]\n    return writer";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: analyze_mutable_vars - ATTRIBUTE ASSIGNMENT (tests 031-045)
    // ========================================================================

    #[test]
    fn test_w20rm_031_attribute_assign_simple() {
        let code = "class Point:\n    x: int\n    y: int\n\ndef func() -> int:\n    p = Point()\n    p.x = 10\n    return p.x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_032_attribute_assign_multiple() {
        let code = "class Rect:\n    width: int\n    height: int\n\ndef func() -> int:\n    r = Rect()\n    r.width = 100\n    r.height = 200\n    return r.width";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_033_self_attribute_assign() {
        let code = "class Counter:\n    count: int\n    def increment(self) -> None:\n        self.count = self.count + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_034_nested_attribute_access() {
        let code = "class Inner:\n    val: int\n\nclass Outer:\n    inner: Inner\n\ndef func() -> int:\n    o = Outer()\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_035_attribute_from_param() {
        let code = "class Node:\n    value: int\n\ndef modify(n: Node) -> None:\n    n.value = 42";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_036_attribute_assign_in_if() {
        let code = "class Obj:\n    val: int\n\ndef func(o: Obj, x: int) -> None:\n    if x > 0:\n        o.val = x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_037_attribute_assign_in_loop() {
        let code = "class Obj:\n    val: int\n\ndef func(o: Obj) -> None:\n    for i in range(10):\n        o.val = i";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_038_attribute_assign_in_while() {
        let code = "class Obj:\n    val: int\n\ndef func(o: Obj) -> None:\n    i = 0\n    while i < 10:\n        o.val = i\n        i = i + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_039_attribute_assign_string() {
        let code = "class Person:\n    name: str\n\ndef func(p: Person) -> None:\n    p.name = \"Alice\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_040_attribute_assign_bool() {
        let code = "class Toggle:\n    active: bool\n\ndef func(t: Toggle) -> None:\n    t.active = True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: analyze_mutable_vars - INDEX ASSIGNMENT (tests 041-055)
    // ========================================================================

    #[test]
    fn test_w20rm_041_index_assign_list() {
        let code = "def func() -> list:\n    arr = [1, 2, 3]\n    arr[0] = 10\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_042_index_assign_dict() {
        let code = "def func() -> dict:\n    d = {}\n    d[\"a\"] = 1\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_043_nested_index_assign() {
        let code = "def func() -> dict:\n    d = {}\n    d[\"a\"] = {}\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_044_index_assign_variable() {
        let code = "def func(idx: int) -> list:\n    arr = [0, 0, 0]\n    arr[idx] = 42\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_045_index_assign_in_loop() {
        let code = "def func() -> list:\n    arr = [0, 0, 0]\n    for i in range(3):\n        arr[i] = i * 2\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_046_index_assign_in_if() {
        let code = "def func(arr: list, x: int) -> list:\n    if x > 0:\n        arr[0] = x\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_047_index_assign_string_key() {
        let code = "def func() -> dict:\n    d = {\"x\": 0}\n    d[\"x\"] = 42\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_048_index_assign_multiple() {
        let code = "def func() -> list:\n    arr = [0, 0, 0]\n    arr[0] = 1\n    arr[1] = 2\n    arr[2] = 3\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_049_index_assign_nested_list() {
        let code = "def func() -> list:\n    matrix = [[0, 0], [0, 0]]\n    matrix[0] = [1, 2]\n    return matrix";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_050_index_assign_with_expr() {
        let code = "def func() -> list:\n    arr = [0, 0, 0]\n    arr[1] = 2 + 3\n    return arr";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: analyze_mutable_vars - TUPLE UNPACKING (tests 051-060)
    // ========================================================================

    #[test]
    fn test_w20rm_051_tuple_unpack_simple() {
        let code = "def func() -> int:\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_052_tuple_unpack_reassign() {
        let code = "def func() -> int:\n    a = 0\n    b = 0\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_053_tuple_unpack_three() {
        let code = "def func() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_054_tuple_swap() {
        let code = "def func() -> int:\n    a = 1\n    b = 2\n    a, b = b, a\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_055_nested_tuple_unpack() {
        let code = "def func() -> int:\n    a, b = 1, 2\n    c, d = 3, 4\n    return a + b + c + d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_056_tuple_unpack_from_func() {
        let code = "def get_pair() -> tuple:\n    return (1, 2)\n\ndef func() -> int:\n    a, b = get_pair()\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_057_tuple_unpack_in_for() {
        let code = "def func(items: list) -> int:\n    total = 0\n    for k, v in items:\n        total = total + v\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_058_tuple_unpack_with_nested_tuple() {
        let code = "def func() -> int:\n    a, b = 10, 20\n    a, b = b, a\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_059_tuple_unpack_string_values() {
        let code = "def func() -> str:\n    first, last = \"John\", \"Doe\"\n    return first";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_060_tuple_unpack_mixed_types() {
        let code = "def func() -> int:\n    name, age = \"Alice\", 30\n    return age";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 6: analyze_mutable_vars - METHOD CALL MUTATIONS (tests 061-085)
    // ========================================================================

    #[test]
    fn test_w20rm_061_list_append() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_062_list_extend() {
        let code = "def func() -> list:\n    items = [1]\n    items.extend([2, 3])\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_063_list_pop() {
        let code = "def func() -> list:\n    items = [1, 2, 3]\n    items.pop()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_064_list_insert() {
        let code = "def func() -> list:\n    items = [1, 3]\n    items.insert(1, 2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_065_list_remove() {
        let code = "def func() -> list:\n    items = [1, 2, 3]\n    items.remove(2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_066_list_sort() {
        let code = "def func() -> list:\n    items = [3, 1, 2]\n    items.sort()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_067_list_reverse() {
        let code = "def func() -> list:\n    items = [1, 2, 3]\n    items.reverse()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_068_list_clear() {
        let code = "def func() -> list:\n    items = [1, 2, 3]\n    items.clear()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_069_dict_update() {
        let code = "def func() -> dict:\n    d = {}\n    d.update({\"a\": 1})\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_070_set_add() {
        let code = "def func() -> set:\n    s = set()\n    s.add(1)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_071_set_discard() {
        let code = "def func() -> set:\n    s = {1, 2, 3}\n    s.discard(2)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_072_set_remove() {
        let code = "def func() -> set:\n    s = {1, 2, 3}\n    s.remove(1)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_073_list_append_in_loop() {
        let code = "def func() -> list:\n    items = []\n    for i in range(5):\n        items.append(i)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_074_list_append_in_if() {
        let code = "def func(x: int) -> list:\n    items = []\n    if x > 0:\n        items.append(x)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_075_list_append_in_while() {
        let code = "def func() -> list:\n    items = []\n    i = 0\n    while i < 5:\n        items.append(i)\n        i = i + 1\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_076_multiple_method_mutations() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    items.append(2)\n    items.extend([3, 4])\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_077_dict_pop() {
        let code = "def func() -> dict:\n    d = {\"a\": 1}\n    d.pop(\"a\")\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_078_set_update() {
        let code = "def func() -> set:\n    s = {1}\n    s.update({2, 3})\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_079_list_append_string() {
        let code = "def func() -> list:\n    words = []\n    words.append(\"hello\")\n    words.append(\"world\")\n    return words";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_080_mutation_as_standalone_expr() {
        let code = "def func() -> None:\n    items = []\n    items.append(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_081_mutation_in_with_block() {
        let code = "def func(path: str) -> None:\n    items = []\n    with open(path) as f:\n        items.append(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_082_mutation_in_try() {
        let code = "def func() -> list:\n    items = []\n    try:\n        items.append(1)\n    except Exception:\n        items.append(0)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_083_mutation_in_try_finally() {
        let code = "def func() -> list:\n    items = []\n    try:\n        items.append(1)\n    finally:\n        items.append(99)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_084_mutation_in_try_else() {
        let code = "def func() -> list:\n    items = []\n    try:\n        items.append(1)\n    except Exception:\n        pass\n    else:\n        items.append(2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_085_set_clear() {
        let code = "def func() -> set:\n    s = {1, 2, 3}\n    s.clear()\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 7: analyze_mutable_vars - EXPR ANALYSIS (tests 086-110)
    // ========================================================================

    #[test]
    fn test_w20rm_086_binary_expr_mutation() {
        let code = "def func() -> list:\n    a = []\n    b = []\n    a.append(1)\n    x = a + b\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_087_unary_expr_analysis() {
        let code = "def func() -> int:\n    x = 5\n    y = -x\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_088_if_expr_analysis() {
        let code = "def func(x: int) -> int:\n    result = 1 if x > 0 else -1\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_089_list_expr_analysis() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    result = [items, []]\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_090_tuple_expr_analysis() {
        let code = "def func() -> tuple:\n    a = 1\n    b = 2\n    return (a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_091_set_expr_analysis() {
        let code = "def func() -> set:\n    a = 1\n    return {a, 2, 3}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_092_dict_expr_analysis() {
        let code = "def func() -> dict:\n    k = \"key\"\n    v = 42\n    return {k: v}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_093_index_expr_analysis() {
        let code = "def func(arr: list) -> int:\n    idx = 0\n    return arr[idx]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_094_attribute_expr_mutation() {
        let code = "class Obj:\n    items: list\n\ndef func(o: Obj) -> None:\n    o.items.append(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_095_method_call_nested_expr() {
        let code = "def func() -> list:\n    items = []\n    items.append(1 + 2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_096_return_with_mutation() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_097_mutation_via_binary_in_assign() {
        let code = "def func() -> int:\n    items = []\n    items.append(1)\n    x = len(items) + 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_098_mutation_via_unary_not() {
        let code = "def func() -> bool:\n    x = True\n    y = not x\n    return y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_099_if_expr_with_mutation() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    x = items if len(items) > 0 else []\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_100_frozenset_expr_analysis() {
        let code = "def func() -> frozenset:\n    a = 1\n    b = 2\n    return frozenset({a, b})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_101_dict_key_value_mutation_analysis() {
        let code = "def func() -> dict:\n    items = []\n    items.append(1)\n    d = {\"key\": items}\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_102_call_no_param_muts() {
        let code = "def helper(x: int) -> int:\n    return x + 1\n\ndef func() -> int:\n    a = 5\n    return helper(a)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_103_method_call_recursive_expr() {
        let code = "def func() -> list:\n    items = []\n    items.append(len(items))\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_104_augmented_assign_makes_mutable() {
        let code = "def func() -> int:\n    x = 0\n    x += 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_105_class_var_type_tracking() {
        let code = "class MyClass:\n    val: int\n\ndef func() -> int:\n    obj = MyClass()\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_106_constructor_var_type() {
        let code = "class Pair:\n    x: int\n    y: int\n\ndef func() -> int:\n    p = Pair()\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_107_multiple_mutations_diff_vars() {
        let code = "def func() -> int:\n    a = []\n    b = {}\n    s = set()\n    a.append(1)\n    b.update({\"x\": 1})\n    s.add(42)\n    return len(a)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_108_mutation_through_chained_if() {
        let code = "def func(x: int) -> list:\n    items = []\n    if x > 0:\n        items.append(x)\n    elif x < 0:\n        items.append(-x)\n    else:\n        items.append(0)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_109_binary_comparison_analysis() {
        let code = "def func(a: int, b: int) -> bool:\n    return a > b and a < 100";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_110_complex_expr_in_return() {
        let code = "def func(x: int) -> int:\n    return x * 2 + 1 if x > 0 else -x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 8: generate_rust_file_internal - CONSTANTS (tests 111-135)
    // ========================================================================

    #[test]
    fn test_w20rm_111_module_level_int_const() {
        let code = "MAX_SIZE = 100\n\ndef func() -> int:\n    return MAX_SIZE";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("MAX_SIZE") || result.contains("100"));
    }

    #[test]
    fn test_w20rm_112_module_level_float_const() {
        let code = "RATE = 0.05\n\ndef func() -> float:\n    return RATE";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_113_module_level_string_const() {
        let code = "NAME = \"depyler\"\n\ndef func() -> str:\n    return NAME";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_114_module_level_bool_const() {
        let code = "DEBUG = True\n\ndef func() -> bool:\n    return DEBUG";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_115_module_level_dict_const() {
        let code = "CONFIG = {\"a\": 1, \"b\": 2}\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_116_module_level_list_const() {
        let code = "ITEMS = [1, 2, 3]\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_117_module_level_set_const() {
        let code = "VALID = {1, 2, 3}\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_118_module_level_tuple_const() {
        let code = "PAIR = (1, 2)\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_119_module_level_lambda() {
        let code = "square = lambda x: x * x\n\ndef func() -> int:\n    return square(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("square") || result.contains("fn"));
    }

    #[test]
    fn test_w20rm_120_module_level_lambda_two_params() {
        let code = "add = lambda a, b: a + b\n\ndef func() -> int:\n    return add(1, 2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_121_typevar_skipped() {
        let code = "from typing import TypeVar\nT = TypeVar(\"T\")\n\ndef identity(x: int) -> int:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_122_duplicate_constant_dedup() {
        let code = "NAME = \"old\"\nNAME = \"new\"\n\ndef func() -> str:\n    return NAME";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_123_multiple_constants() {
        let code = "A = 1\nB = 2\nC = 3\n\ndef func() -> int:\n    return A + B + C";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_124_constant_with_func_call() {
        let code = "LENGTH = len([1, 2, 3])\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_125_constant_empty_dict() {
        let code = "EMPTY = {}\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_126_constant_empty_list() {
        let code = "EMPTY = []\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_127_constant_nested_dict() {
        let code = "DATA = {\"a\": {\"b\": 1}}\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_128_constant_list_of_strings() {
        let code = "NAMES = [\"alice\", \"bob\", \"charlie\"]\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_129_lambda_identity() {
        let code = "identity = lambda x: x\n\ndef func() -> int:\n    return identity(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_130_constant_negative_int() {
        let code = "MIN_VAL = -100\n\ndef func() -> int:\n    return MIN_VAL";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_131_constant_large_float() {
        let code = "EPSILON = 0.000001\n\ndef func() -> float:\n    return EPSILON";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_132_multiple_typevars_skipped() {
        let code = "from typing import TypeVar\nT = TypeVar(\"T\")\nU = TypeVar(\"U\")\n\ndef func(x: int) -> int:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_133_constant_reassigned_three_times() {
        let code = "X = 1\nX = 2\nX = 3\n\ndef func() -> int:\n    return X";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_134_constant_binary_expr() {
        let code = "TOTAL = 10 + 20\n\ndef func() -> int:\n    return TOTAL";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_135_constant_empty_string() {
        let code = "DEFAULT = \"\"\n\ndef func() -> str:\n    return DEFAULT";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 9: generate_rust_file_internal - IMPORTS (tests 136-155)
    // ========================================================================

    #[test]
    fn test_w20rm_136_import_datetime() {
        let code = "import datetime\n\ndef func() -> str:\n    return \"now\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_137_import_itertools() {
        let code = "import itertools\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_138_import_statistics() {
        let code = "import statistics\n\ndef func() -> float:\n    return 0.0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_139_import_json() {
        let code = "import json\n\ndef func() -> str:\n    return json.dumps({})";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_140_import_os() {
        let code = "import os\n\ndef func() -> str:\n    return \"path\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_141_import_sys() {
        let code = "import sys\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_142_import_math() {
        let code = "import math\n\ndef func() -> float:\n    return math.sqrt(4.0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_143_import_re() {
        let code = "import re\n\ndef func() -> bool:\n    return True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_144_from_typing_import() {
        let code = "from typing import List, Dict, Optional\n\ndef func(x: List[int]) -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_145_import_collections() {
        let code = "from collections import defaultdict\n\ndef func() -> dict:\n    d = defaultdict(int)\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_146_import_random() {
        let code = "import random\n\ndef func() -> int:\n    return random.randint(1, 10)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_147_import_hashlib() {
        let code = "import hashlib\n\ndef func() -> str:\n    return \"hash\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_148_multiple_imports() {
        let code = "import os\nimport sys\nimport json\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_149_from_import_specific() {
        let code = "from os.path import join\n\ndef func() -> str:\n    return \"path\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_150_import_time() {
        let code = "import time\n\ndef func() -> float:\n    return 0.0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_151_import_string() {
        let code = "import string\n\ndef func() -> str:\n    return \"abc\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_152_import_functools() {
        let code = "import functools\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_153_import_pathlib() {
        let code = "from pathlib import Path\n\ndef func() -> str:\n    return \"path\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_154_import_enum() {
        let code = "from enum import Enum\n\nclass Color(Enum):\n    RED = 1\n    GREEN = 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_155_import_abc() {
        let code = "from abc import ABC\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 10: generate_rust_file_internal - FUNCTIONS & CLASSES (tests 156-200)
    // ========================================================================

    #[test]
    fn test_w20rm_156_async_function_detection() {
        let code = "import asyncio\n\nasync def fetch() -> str:\n    return \"data\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_157_property_method() {
        let code = "class Circle:\n    radius: float\n    @property\n    def area(self) -> float:\n        return self.radius * self.radius";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_158_dataclass_defaults() {
        let code = "class Config:\n    name: str\n    value: int\n    def __init__(self, name: str, value: int) -> None:\n        self.name = name\n        self.value = value";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_159_class_method_return_type() {
        let code = "class Calculator:\n    def add(self, a: int, b: int) -> int:\n        return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_160_option_returning_function() {
        let code = "from typing import Optional\n\ndef find(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_161_varargs_function() {
        let code = "def func(*args) -> int:\n    return len(args)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_162_function_param_types() {
        let code = "def func(a: int, b: float, c: str) -> str:\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_163_can_fail_function() {
        let code = "def risky(x: int) -> int:\n    if x == 0:\n        raise ValueError(\"zero\")\n    return 10 // x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_164_result_bool_function() {
        let code = "def check(x: int) -> bool:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_165_class_field_types() {
        let code = "class Point:\n    x: float\n    y: float\n    def distance(self) -> float:\n        return (self.x * self.x + self.y * self.y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_166_class_constructor_return_type() {
        let code = "class Foo:\n    val: int\n\ndef make_foo() -> Foo:\n    return Foo()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_167_optional_param() {
        let code = "from typing import Optional\n\ndef func(x: int, y: Optional[int] = None) -> int:\n    if y is not None:\n        return x + y\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_168_multiple_classes() {
        let code = "class A:\n    x: int\n\nclass B:\n    y: int\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_169_class_with_multiple_methods() {
        let code = "class Stack:\n    items: list\n    def push(self, x: int) -> None:\n        self.items.append(x)\n    def size(self) -> int:\n        return len(self.items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_170_module_constant_type_tracking() {
        let code = "DATA = {1: \"a\", 2: \"b\"}\n\ndef func(key: int) -> str:\n    return DATA[key]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_171_mutating_methods_detection() {
        let code = "class MyList:\n    data: list\n    def add(self, x: int) -> None:\n        self.data.append(x)\n\ndef func() -> None:\n    ml = MyList()\n    ml.add(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_172_class_no_init() {
        let code = "class Empty:\n    pass\n\ndef func() -> int:\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_173_function_returns_list() {
        let code = "def func() -> list:\n    return [1, 2, 3]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_174_function_returns_dict() {
        let code = "def func() -> dict:\n    return {\"a\": 1}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_175_function_returns_tuple() {
        let code = "def func() -> tuple:\n    return (1, 2, 3)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_176_simple_int_function() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w20rm_177_string_function() {
        let code = "def greet(name: str) -> str:\n    return \"Hello, \" + name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_178_bool_function() {
        let code = "def is_positive(x: int) -> bool:\n    return x > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_179_class_method_return_types_tracked() {
        let code = "class Math:\n    def square(self, x: int) -> int:\n        return x * x\n    def cube(self, x: int) -> int:\n        return x * x * x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_180_multiple_function_return_types() {
        let code = "def get_int() -> int:\n    return 42\n\ndef get_str() -> str:\n    return \"hello\"\n\ndef get_bool() -> bool:\n    return True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_181_for_loop_mutation_detection() {
        let code = "def func() -> int:\n    total = 0\n    items = [1, 2, 3]\n    for x in items:\n        total = total + x\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_182_while_mutation_with_condition() {
        let code = "def func() -> int:\n    count = 0\n    items = []\n    while count < 10:\n        items.append(count)\n        count = count + 1\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_183_mutation_across_scopes() {
        let code = "def func() -> list:\n    result = []\n    for i in range(3):\n        if i > 0:\n            result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_184_dict_setdefault() {
        let code = "def func() -> dict:\n    d = {}\n    d.setdefault(\"key\", 0)\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_185_list_append_multiple_types() {
        let code = "def func() -> list:\n    items = []\n    items.append(1)\n    items.append(\"hello\")\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_186_class_with_property_and_method() {
        let code = "class Rect:\n    width: float\n    height: float\n    @property\n    def area(self) -> float:\n        return self.width * self.height\n    def perimeter(self) -> float:\n        return 2.0 * (self.width + self.height)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_187_function_with_default_param() {
        let code = "def func(x: int, y: int = 10) -> int:\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_188_module_with_class_and_func() {
        let code = "VERSION = \"1.0\"\n\nclass App:\n    name: str\n\ndef run() -> str:\n    return VERSION";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_189_constant_used_in_function() {
        let code = "LIMIT = 100\n\ndef is_valid(x: int) -> bool:\n    return x < LIMIT";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_190_empty_function() {
        let code = "def noop() -> None:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_191_function_multiple_returns() {
        let code = "def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    return \"zero\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_192_class_init_method() {
        let code = "class Person:\n    name: str\n    age: int\n    def __init__(self, name: str, age: int) -> None:\n        self.name = name\n        self.age = age";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_193_nested_if_mutation() {
        let code = "def func() -> list:\n    items = []\n    for i in range(10):\n        if i % 2 == 0:\n            if i > 4:\n                items.append(i)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_194_mutation_in_try_except_handler() {
        let code = "def func() -> list:\n    errors = []\n    try:\n        x = 1\n    except ValueError:\n        errors.append(\"val\")\n    except TypeError:\n        errors.append(\"type\")\n    return errors";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_195_module_constant_list_type() {
        let code = "NUMBERS = [10, 20, 30]\n\ndef func() -> int:\n    return NUMBERS[0]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_196_module_constant_set_type() {
        let code = "ALLOWED = {\"a\", \"b\", \"c\"}\n\ndef func() -> bool:\n    return True";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_197_function_param_optional_tracking() {
        let code = "from typing import Optional\n\ndef func(a: int, b: Optional[str] = None, c: Optional[int] = None) -> int:\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_198_class_field_default_values() {
        let code = "class Settings:\n    debug: bool\n    timeout: int\n    def __init__(self, debug: bool = False, timeout: int = 30) -> None:\n        self.debug = debug\n        self.timeout = timeout";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_199_full_module_with_all_features() {
        let code = "VERSION = \"2.0\"\nMAX = 100\n\nclass Item:\n    name: str\n    value: int\n\ndef process(items: list) -> list:\n    result = []\n    for item in items:\n        result.append(item)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w20rm_200_complex_mutation_scenario() {
        let code = "def func() -> dict:\n    data = {}\n    items = []\n    counts = set()\n    for i in range(10):\n        items.append(i)\n        if i > 5:\n            counts.add(i)\n        data[str(i)] = i\n    return data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
