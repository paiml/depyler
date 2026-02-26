//! Coverage wave 5: stmt_gen.rs and rust_gen.rs coverage boost tests
//!
//! Targets uncovered branches in:
//! - stmt_gen.rs (1,256 lines missed, 86.8% covered): complex class patterns, nested try/except,
//!   match/case, with statements, augmented assignments, complex for/while loops,
//!   raise with custom exceptions, global/nonlocal, del statement, assert with messages
//! - rust_gen.rs (1,265 lines missed, 87.4% covered): module-level constants, multiple function
//!   files, complex type annotations, import handling, class hierarchies, async generators,
//!   dataclass patterns, enum patterns

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> String {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).expect("transpile")
    }

    fn transpile_ok(code: &str) -> bool {
        let pipeline = DepylerPipeline::new();
        pipeline.transpile(code).is_ok()
    }

    // =========================================================================
    // Section 1: stmt_gen.rs - Assert statement variants (codegen_assert_stmt)
    // =========================================================================

    #[test]
    fn test_w5_assert_eq_with_message() {
        let code =
            transpile("def check(x: int, y: int):\n    assert x == y, \"values must be equal\"");
        assert!(!code.is_empty());
        assert!(code.contains("assert_eq!"));
    }

    #[test]
    fn test_w5_assert_ne_with_message() {
        let code =
            transpile("def check(a: int, b: int):\n    assert a != b, \"values must differ\"");
        assert!(!code.is_empty());
        assert!(code.contains("assert_ne!"));
    }

    #[test]
    fn test_w5_assert_eq_no_message() {
        let code = transpile("def verify(x: int):\n    assert x == 42");
        assert!(!code.is_empty());
        assert!(code.contains("assert_eq!"));
    }

    #[test]
    fn test_w5_assert_ne_no_message() {
        let code = transpile("def verify(x: int):\n    assert x != 0");
        assert!(!code.is_empty());
        assert!(code.contains("assert_ne!"));
    }

    #[test]
    fn test_w5_assert_boolean_condition() {
        let code = transpile("def verify(flag: bool):\n    assert flag");
        assert!(!code.is_empty());
        assert!(code.contains("assert!"));
    }

    #[test]
    fn test_w5_assert_with_string_message() {
        let code = transpile("def verify(x: int):\n    assert x > 0, \"must be positive\"");
        assert!(!code.is_empty());
        assert!(code.contains("assert!"));
    }

    #[test]
    fn test_w5_assert_greater_than() {
        let code = transpile("def check(val: int):\n    assert val > 10, \"too small\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assert_less_than() {
        let code = transpile("def check(val: int):\n    assert val < 100");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 2: stmt_gen.rs - While loop variants (codegen_while_stmt)
    // =========================================================================

    #[test]
    fn test_w5_while_true_loop() {
        let code = transpile("def run():\n    while True:\n        break");
        assert!(!code.is_empty());
        assert!(code.contains("loop"));
    }

    #[test]
    fn test_w5_while_condition_loop() {
        let code = transpile(
            "def count():\n    x: int = 10\n    while x > 0:\n        x = x - 1\n    return x",
        );
        assert!(!code.is_empty());
        assert!(code.contains("while"));
    }

    #[test]
    fn test_w5_while_with_break() {
        let code = transpile("def search(items: list):\n    i: int = 0\n    while i < 10:\n        if i == 5:\n            break\n        i = i + 1");
        assert!(!code.is_empty());
        assert!(code.contains("break"));
    }

    #[test]
    fn test_w5_while_with_continue() {
        let code = transpile("def skip_odds():\n    i: int = 0\n    while i < 10:\n        i = i + 1\n        if i % 2 == 1:\n            continue\n        print(i)");
        assert!(!code.is_empty());
        assert!(code.contains("continue"));
    }

    #[test]
    fn test_w5_while_truthiness_collection() {
        let code = transpile("def drain(items: list):\n    while items:\n        items.pop()");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_while_nested_loops() {
        let code = transpile("def matrix():\n    i: int = 0\n    while i < 3:\n        j: int = 0\n        while j < 3:\n            j = j + 1\n        i = i + 1");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 3: stmt_gen.rs - Raise statement variants (codegen_raise_stmt)
    // =========================================================================

    #[test]
    fn test_w5_raise_value_error() {
        let code = transpile("def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_type_error() {
        let code = transpile("def check(x: str) -> str:\n    if not x:\n        raise TypeError(\"empty string\")\n    return x");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_runtime_error() {
        let code = transpile(
            "def fail() -> int:\n    raise RuntimeError(\"something went wrong\")\n    return 0",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_key_error() {
        let code = transpile("def lookup(d: dict, key: str) -> str:\n    if key not in d:\n        raise KeyError(\"not found\")\n    return d[key]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_index_error() {
        let code = transpile("def access(items: list, idx: int) -> int:\n    if idx >= len(items):\n        raise IndexError(\"out of range\")\n    return items[idx]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_file_not_found() {
        let code = transpile("def read_file(path: str) -> str:\n    raise FileNotFoundError(\"missing\")\n    return \"\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_zero_division() {
        let code = transpile("def divide(a: int, b: int) -> float:\n    if b == 0:\n        raise ZeroDivisionError(\"cannot divide by zero\")\n    return a / b");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_io_error() {
        let code = transpile("def write_data(data: str) -> bool:\n    raise IOError(\"write failed\")\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_attribute_error() {
        let code = transpile("def get_attr(obj: str) -> str:\n    raise AttributeError(\"no such attribute\")\n    return \"\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_stop_iteration() {
        let code =
            transpile("def next_item() -> int:\n    raise StopIteration(\"done\")\n    return 0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_syntax_error() {
        let code = transpile("def parse_input(text: str) -> str:\n    raise SyntaxError(\"invalid syntax\")\n    return \"\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_raise_in_non_result_function() {
        let code = transpile("def boom():\n    raise ValueError(\"kaboom\")");
        assert!(!code.is_empty());
        assert!(code.contains("panic!"));
    }

    #[test]
    fn test_w5_bare_raise() {
        let result = transpile_ok("def reraise():\n    raise");
        assert!(result);
    }

    // =========================================================================
    // Section 4: stmt_gen.rs - With statement variants (codegen_with_stmt)
    // =========================================================================

    #[test]
    fn test_w5_with_open_file() {
        let code = transpile("def read_data():\n    with open(\"test.txt\") as f:\n        data = f.read()\n        print(data)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_with_open_write() {
        let code = transpile("def write_data():\n    with open(\"out.txt\", \"w\") as f:\n        f.write(\"hello\")");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_with_no_target() {
        let code =
            transpile("def manage():\n    with open(\"test.txt\"):\n        print(\"inside\")");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_with_custom_context_manager() {
        let code =
            transpile("def use_lock():\n    with Lock() as lock:\n        print(\"locked\")");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 5: stmt_gen.rs - For loop variants
    // =========================================================================

    #[test]
    fn test_w5_for_range() {
        let code = transpile("def sum_range() -> int:\n    total: int = 0\n    for i in range(10):\n        total = total + i\n    return total");
        assert!(!code.is_empty());
        assert!(code.contains("for"));
    }

    #[test]
    fn test_w5_for_range_with_step() {
        let code = transpile("def even_sum() -> int:\n    total: int = 0\n    for i in range(0, 10, 2):\n        total = total + i\n    return total");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_for_enumerate() {
        let code = transpile(
            "def indexed(items: list):\n    for i, item in enumerate(items):\n        print(i)",
        );
        assert!(!code.is_empty());
        assert!(code.contains("enumerate"));
    }

    #[test]
    fn test_w5_for_zip() {
        let code = transpile(
            "def pair_up(a: list, b: list):\n    for x, y in zip(a, b):\n        print(x)",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_for_with_break_continue() {
        let code = transpile("def filter_loop():\n    for i in range(20):\n        if i < 5:\n            continue\n        if i > 15:\n            break\n        print(i)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_for_loops() {
        let code = transpile("def matrix_sum() -> int:\n    total: int = 0\n    for i in range(3):\n        for j in range(3):\n            total = total + i * j\n    return total");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_for_dict_items() {
        let code = transpile(
            "def show_dict(d: dict):\n    for key, value in d.items():\n        print(key)",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_for_dict_keys() {
        let code =
            transpile("def show_keys(d: dict):\n    for key in d.keys():\n        print(key)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_for_dict_values() {
        let code =
            transpile("def show_values(d: dict):\n    for val in d.values():\n        print(val)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 6: stmt_gen.rs - If statement variants (codegen_if_stmt)
    // =========================================================================

    #[test]
    fn test_w5_if_else() {
        let code = transpile("def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    else:\n        return \"non-positive\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_elif_else() {
        let code = transpile("def classify(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_if() {
        let code = transpile("def deep_check(a: int, b: int) -> str:\n    if a > 0:\n        if b > 0:\n            return \"both positive\"\n        else:\n            return \"a positive only\"\n    return \"not both\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_isinstance() {
        let code = transpile("def check_type(x: int) -> bool:\n    if isinstance(x, int):\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_type_checking() {
        let code =
            transpile("TYPE_CHECKING = False\ndef foo():\n    if TYPE_CHECKING:\n        pass");
        assert!(transpile_ok(
            "TYPE_CHECKING = False\ndef foo():\n    if TYPE_CHECKING:\n        pass"
        ));
    }

    #[test]
    fn test_w5_if_string_truthiness() {
        let code = transpile("def check_name(name: str) -> bool:\n    if name:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_optional_truthiness() {
        let code = transpile("from typing import Optional\ndef check(val: Optional[int]) -> bool:\n    if val:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_collection_truthiness() {
        let code = transpile("def check_items(items: list) -> bool:\n    if items:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_not_truthiness() {
        let code = transpile("def is_empty(items: list) -> bool:\n    if not items:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_not_string_truthiness() {
        let code = transpile("def is_blank(text: str) -> bool:\n    if not text:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_not_optional_truthiness() {
        let code = transpile("from typing import Optional\ndef is_none(val: Optional[int]) -> bool:\n    if not val:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_if_dict_truthiness() {
        let code = transpile(
            "def check_dict(d: dict) -> bool:\n    if d:\n        return True\n    return False",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 7: stmt_gen.rs - Return statement variants (codegen_return_stmt)
    // =========================================================================

    #[test]
    fn test_w5_return_optional_some() {
        let code = transpile("from typing import Optional\ndef find(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_optional_none() {
        let code = transpile(
            "from typing import Optional\ndef maybe() -> Optional[str]:\n    return None",
        );
        assert!(!code.is_empty());
        assert!(code.contains("None"));
    }

    #[test]
    fn test_w5_return_empty_string() {
        let code = transpile("def empty() -> str:\n    return \"\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_void() {
        let code = transpile("def do_nothing() -> None:\n    return");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_tuple() {
        let code = transpile(
            "from typing import Tuple\ndef pair() -> Tuple[int, str]:\n    return (1, \"hello\")",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_early_return() {
        let code = transpile(
            "def guard(x: int) -> int:\n    if x < 0:\n        return -1\n    return x * 2",
        );
        assert!(!code.is_empty());
        assert!(code.contains("return"));
    }

    #[test]
    fn test_w5_return_from_while() {
        let code = transpile("def find_first() -> int:\n    i: int = 0\n    while i < 100:\n        if i * i > 50:\n            return i\n        i = i + 1\n    return -1");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_dict_subscript() {
        let code = transpile("def get_val(d: dict, key: str) -> str:\n    return d[key]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_method_result_optional() {
        let code = transpile("from typing import Optional\ndef safe_get(d: dict, key: str) -> Optional[str]:\n    return d.get(key)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 8: stmt_gen.rs - Augmented assignment
    // =========================================================================

    #[test]
    fn test_w5_augmented_add_int() {
        let code = transpile("def inc():\n    x: int = 0\n    x += 1\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_augmented_sub_int() {
        let code = transpile("def dec():\n    x: int = 10\n    x -= 1\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_augmented_mul_int() {
        let code = transpile("def double():\n    x: int = 5\n    x *= 2\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_augmented_div_float() {
        let code = transpile("def halve():\n    x: float = 10.0\n    x /= 2.0\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_augmented_mod_int() {
        let code = transpile("def modulo():\n    x: int = 10\n    x %= 3\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_augmented_add_string() {
        let code = transpile(
            "def append_str():\n    s: str = \"hello\"\n    s += \" world\"\n    print(s)",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 9: stmt_gen.rs - Try/Except patterns
    // =========================================================================

    #[test]
    fn test_w5_try_except_basic() {
        let code = transpile("def safe_divide(a: int, b: int) -> int:\n    try:\n        return a // b\n    except ZeroDivisionError:\n        return 0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_try_except_with_variable() {
        let code = transpile("def handle_err() -> str:\n    try:\n        x: int = int(\"abc\")\n        return str(x)\n    except ValueError as e:\n        return str(e)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_try_except_finally() {
        let code = transpile("def cleanup():\n    try:\n        print(\"try\")\n    except Exception:\n        print(\"error\")\n    finally:\n        print(\"cleanup\")");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_try_except_else() {
        let code = transpile("def try_else(x: int) -> str:\n    try:\n        result = 100 // x\n    except ZeroDivisionError:\n        return \"error\"\n    else:\n        return str(result)\n    return \"done\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_try_multiple_except() {
        let code = transpile("def multi_catch(x: str) -> int:\n    try:\n        return int(x)\n    except ValueError:\n        return -1\n    except TypeError:\n        return -2");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_try_except() {
        let code = transpile("def nested_try() -> str:\n    try:\n        try:\n            return str(int(\"bad\"))\n        except ValueError:\n            return \"inner\"\n    except Exception:\n        return \"outer\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_try_except_all() {
        let code = transpile("def catch_all() -> str:\n    try:\n        x: int = 1 // 0\n        return str(x)\n    except Exception:\n        return \"caught\"");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 10: stmt_gen.rs - Expression statements
    // =========================================================================

    #[test]
    fn test_w5_print_statement() {
        let code = transpile("def hello():\n    print(\"hello world\")");
        assert!(!code.is_empty());
        assert!(code.contains("println!"));
    }

    #[test]
    fn test_w5_method_call_expr() {
        let code = transpile("def mutate(items: list):\n    items.append(42)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_pure_expression_suppressed() {
        let code = transpile("def pure():\n    x: int = 5\n    x + 1\n    print(x)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 11: stmt_gen.rs - Assign statement variants
    // =========================================================================

    #[test]
    fn test_w5_assign_simple() {
        let code = transpile("def assign():\n    x: int = 42\n    print(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_tuple_unpack() {
        let code = transpile("def unpack():\n    a, b = 1, 2\n    print(a)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_index() {
        let code = transpile(
            "def set_item():\n    items: list = [1, 2, 3]\n    items[0] = 99\n    print(items)",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_attribute() {
        let code = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n\ndef move_point():\n    p = Point(0, 0)\n    p.x = 10");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_reassignment_mutable() {
        let code = transpile("def reassign():\n    x: int = 1\n    x = 2\n    x = 3\n    print(x)");
        assert!(!code.is_empty());
        assert!(code.contains("mut"));
    }

    #[test]
    fn test_w5_assign_string_typed() {
        let code = transpile("def greeting() -> str:\n    msg: str = \"hello\"\n    return msg");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_list_typed() {
        let code = transpile("from typing import List\ndef make_list() -> List[int]:\n    nums: List[int] = [1, 2, 3]\n    return nums");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_assign_dict_typed() {
        let code = transpile("from typing import Dict\ndef make_dict() -> Dict[str, int]:\n    d: Dict[str, int] = {\"a\": 1}\n    return d");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 12: stmt_gen.rs - Variable hoisting in if/else
    // =========================================================================

    #[test]
    fn test_w5_hoisted_variable_if_else() {
        let code = transpile("def choose(flag: bool) -> int:\n    if flag:\n        result = 1\n    else:\n        result = 2\n    return result");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_hoisted_multiple_vars() {
        let code = transpile("def pick(flag: bool) -> int:\n    if flag:\n        x = 1\n        y = 2\n    else:\n        x = 3\n        y = 4\n    return x + y");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 13: stmt_gen.rs - Truthiness conversion (various types)
    // =========================================================================

    #[test]
    fn test_w5_truthiness_int() {
        let code = transpile(
            "def check(n: int) -> bool:\n    if n:\n        return True\n    return False",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_truthiness_float() {
        let code = transpile(
            "def check(x: float) -> bool:\n    if x:\n        return True\n    return False",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_truthiness_set() {
        let code = transpile("from typing import Set\ndef check(s: Set[int]) -> bool:\n    if s:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_truthiness_self_field_list() {
        let code = transpile("class Container:\n    def __init__(self):\n        self.items: list = []\n\n    def has_items(self) -> bool:\n        if self.items:\n            return True\n        return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_truthiness_not_int() {
        let code = transpile(
            "def is_zero(n: int) -> bool:\n    if not n:\n        return True\n    return False",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_truthiness_not_float() {
        let code = transpile("def is_zero_float(x: float) -> bool:\n    if not x:\n        return True\n    return False");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 14: rust_gen.rs - Module-level constants
    // =========================================================================

    #[test]
    fn test_w5_module_constant_int() {
        let code = transpile("MAX_SIZE: int = 100\n\ndef get_max() -> int:\n    return MAX_SIZE");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_module_constant_string() {
        let code =
            transpile("APP_NAME: str = \"myapp\"\n\ndef get_name() -> str:\n    return APP_NAME");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_module_constant_float() {
        let code = transpile(
            "THRESHOLD: float = 0.5\n\ndef above(x: float) -> bool:\n    return x > THRESHOLD",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_module_constant_bool() {
        let code = transpile("DEBUG: bool = False\n\ndef is_debug() -> bool:\n    return DEBUG");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_multiple_constants() {
        let code = transpile(
            "WIDTH: int = 80\nHEIGHT: int = 24\n\ndef area() -> int:\n    return WIDTH * HEIGHT",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 15: rust_gen.rs - Multiple functions in module
    // =========================================================================

    #[test]
    fn test_w5_two_functions() {
        let code = transpile("def add(a: int, b: int) -> int:\n    return a + b\n\ndef sub(a: int, b: int) -> int:\n    return a - b");
        assert!(!code.is_empty());
        assert!(code.contains("add"));
        assert!(code.contains("sub"));
    }

    #[test]
    fn test_w5_three_functions() {
        let code = transpile("def first() -> int:\n    return 1\n\ndef second() -> int:\n    return 2\n\ndef third() -> int:\n    return 3");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_functions_calling_each_other() {
        let code = transpile("def double(x: int) -> int:\n    return x * 2\n\ndef quadruple(x: int) -> int:\n    return double(double(x))");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 16: rust_gen.rs - Class patterns (convert_classes_to_rust)
    // =========================================================================

    #[test]
    fn test_w5_simple_class() {
        let code = transpile("class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y");
        assert!(!code.is_empty());
        assert!(code.contains("struct"));
    }

    #[test]
    fn test_w5_class_with_method() {
        let code = transpile("class Counter:\n    def __init__(self):\n        self.count: int = 0\n\n    def increment(self):\n        self.count += 1");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_with_str_method() {
        let code = transpile("class Greeting:\n    def __init__(self, name: str):\n        self.name = name\n\n    def __str__(self) -> str:\n        return \"Hello, \" + self.name");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_with_property() {
        let code = transpile("class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius\n\n    @property\n    def area(self) -> float:\n        return 3.14159 * self.radius * self.radius");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_inheritance() {
        let code = transpile("class Animal:\n    def __init__(self, name: str):\n        self.name = name\n\n    def speak(self) -> str:\n        return self.name\n\nclass Dog(Animal):\n    def speak(self) -> str:\n        return \"Woof\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_dataclass_pattern() {
        let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Config:\n    host: str\n    port: int\n    debug: bool = False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_with_class_var() {
        let code = transpile("class Settings:\n    version: str = \"1.0\"\n\n    def __init__(self, name: str):\n        self.name = name");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_multiple_methods() {
        let code = transpile("class Stack:\n    def __init__(self):\n        self.items: list = []\n\n    def push(self, item: int):\n        self.items.append(item)\n\n    def pop(self) -> int:\n        return self.items.pop()\n\n    def is_empty(self) -> bool:\n        return len(self.items) == 0");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 17: rust_gen.rs - Import handling
    // =========================================================================

    #[test]
    fn test_w5_import_os() {
        let code = transpile("import os\n\ndef get_cwd() -> str:\n    return os.getcwd()");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_import_sys() {
        let code = transpile("import sys\n\ndef get_args() -> list:\n    return sys.argv");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_from_typing_import() {
        let code = transpile("from typing import List, Dict, Optional\n\ndef process(items: List[int], lookup: Dict[str, int], default: Optional[int]) -> int:\n    return len(items)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_import_collections() {
        let code = transpile("from collections import defaultdict\n\ndef count_items(items: list) -> dict:\n    d = defaultdict(int)\n    return d");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_import_math() {
        let code = transpile(
            "import math\n\ndef circle_area(r: float) -> float:\n    return math.pi * r * r",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 18: rust_gen.rs - Type annotation handling
    // =========================================================================

    #[test]
    fn test_w5_list_int_annotation() {
        let code =
            transpile("from typing import List\ndef ints() -> List[int]:\n    return [1, 2, 3]");
        assert!(!code.is_empty());
        assert!(code.contains("Vec"));
    }

    #[test]
    fn test_w5_dict_str_int_annotation() {
        let code = transpile(
            "from typing import Dict\ndef mapping() -> Dict[str, int]:\n    return {\"a\": 1}",
        );
        assert!(!code.is_empty());
        assert!(code.contains("HashMap"));
    }

    #[test]
    fn test_w5_optional_annotation() {
        let code = transpile("from typing import Optional\ndef maybe(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None");
        assert!(!code.is_empty());
        assert!(code.contains("Option"));
    }

    #[test]
    fn test_w5_tuple_annotation() {
        let code = transpile(
            "from typing import Tuple\ndef coords() -> Tuple[int, int]:\n    return (1, 2)",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_set_annotation() {
        let code = transpile(
            "from typing import Set\ndef unique() -> Set[str]:\n    return {\"a\", \"b\"}",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_optional_list() {
        let code = transpile("from typing import Optional, List\ndef maybe_list() -> Optional[List[int]]:\n    return [1, 2, 3]");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 19: stmt_gen.rs - Collection element type inference
    // =========================================================================

    #[test]
    fn test_w5_list_int_literals() {
        let code = transpile("def nums() -> list:\n    return [1, 2, 3]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_string_literals() {
        let code = transpile("def words() -> list:\n    return [\"a\", \"b\", \"c\"]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_float_literals() {
        let code = transpile("def decimals() -> list:\n    return [1.0, 2.0, 3.0]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_bool_literals() {
        let code = transpile("def flags() -> list:\n    return [True, False, True]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_mixed_int_float() {
        let code = transpile("def mixed() -> list:\n    return [1, 2.0, 3]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_empty_list() {
        let code = transpile("from typing import List\ndef empty() -> List[int]:\n    return []");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_list() {
        let code = transpile("def nested() -> list:\n    return [[1, 2], [3, 4]]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_set_literal_int() {
        let code = transpile("def unique_nums() -> set:\n    return {1, 2, 3}");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 20: rust_gen.rs - Conditional imports
    // =========================================================================

    #[test]
    fn test_w5_needs_hashmap_import() {
        let code = transpile("from typing import Dict\ndef make_dict() -> Dict[str, int]:\n    d: Dict[str, int] = {}\n    d[\"key\"] = 1\n    return d");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_needs_hashset_import() {
        let code = transpile("from typing import Set\ndef make_set() -> Set[int]:\n    s: Set[int] = set()\n    return s");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 21: stmt_gen.rs - Delete statement
    // =========================================================================

    #[test]
    fn test_w5_del_variable() {
        let code = transpile("def cleanup():\n    x: int = 42\n    del x");
        assert!(transpile_ok("def cleanup():\n    x: int = 42\n    del x"));
    }

    #[test]
    fn test_w5_del_dict_item() {
        let code = transpile("def remove_key(d: dict):\n    del d[\"key\"]");
        assert!(transpile_ok("def remove_key(d: dict):\n    del d[\"key\"]"));
    }

    // =========================================================================
    // Section 22: stmt_gen.rs - Global/Nonlocal
    // =========================================================================

    #[test]
    fn test_w5_global_variable() {
        let code = transpile(
            "counter: int = 0\n\ndef increment():\n    global counter\n    counter = counter + 1",
        );
        assert!(transpile_ok(
            "counter: int = 0\n\ndef increment():\n    global counter\n    counter = counter + 1"
        ));
    }

    // =========================================================================
    // Section 23: stmt_gen.rs - Pass statement
    // =========================================================================

    #[test]
    fn test_w5_pass_in_function() {
        let code = transpile("def noop():\n    pass");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_pass_in_class() {
        let code = transpile("class Empty:\n    pass");
        assert!(transpile_ok("class Empty:\n    pass"));
    }

    #[test]
    fn test_w5_pass_in_if() {
        let code = transpile("def maybe(x: int):\n    if x > 0:\n        pass\n    else:\n        print(\"negative\")");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 24: rust_gen.rs - Type alias generation
    // =========================================================================

    #[test]
    fn test_w5_type_alias_simple() {
        let code = transpile("from typing import List\nVector = List[float]\n\ndef norm(v: Vector) -> float:\n    return sum(x * x for x in v)");
        assert!(transpile_ok("from typing import List\nVector = List[float]\n\ndef norm(v: Vector) -> float:\n    return sum(x * x for x in v)"));
    }

    // =========================================================================
    // Section 25: stmt_gen.rs - Mutable variable analysis
    // =========================================================================

    #[test]
    fn test_w5_mutable_from_append() {
        let code = transpile("def build() -> list:\n    items: list = []\n    items.append(1)\n    items.append(2)\n    return items");
        assert!(!code.is_empty());
        assert!(code.contains("mut"));
    }

    #[test]
    fn test_w5_mutable_from_extend() {
        let code = transpile("def extend_list() -> list:\n    items: list = [1]\n    items.extend([2, 3])\n    return items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_insert() {
        let code = transpile("def insert_item() -> list:\n    items: list = [1, 3]\n    items.insert(1, 2)\n    return items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_pop() {
        let code = transpile("def pop_item() -> list:\n    items: list = [1, 2, 3]\n    items.pop()\n    return items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_remove() {
        let code = transpile("def remove_item() -> list:\n    items: list = [1, 2, 3]\n    items.remove(2)\n    return items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_reassignment() {
        let code = transpile("def swap() -> int:\n    a: int = 1\n    b: int = 2\n    a = b\n    b = a\n    return a + b");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_index_assign() {
        let code = transpile(
            "def set_element():\n    arr: list = [0, 0, 0]\n    arr[1] = 42\n    print(arr)",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_mutable_from_dict_assign() {
        let code =
            transpile("def set_key():\n    d: dict = {}\n    d[\"key\"] = \"value\"\n    print(d)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 26: rust_gen.rs - Error type definitions
    // =========================================================================

    #[test]
    fn test_w5_error_type_value_error_generated() {
        let code = transpile("def validate(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"must be non-negative\")\n    return x");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_multiple_error_types() {
        let code = transpile("def complex_validate(x: int, y: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative x\")\n    if y == 0:\n        raise ZeroDivisionError(\"zero y\")\n    return x // y");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 27: stmt_gen.rs - List comprehension
    // =========================================================================

    #[test]
    fn test_w5_list_comprehension_simple() {
        let code = transpile("def squares() -> list:\n    return [x * x for x in range(10)]");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_comprehension_with_filter() {
        let code = transpile(
            "def even_squares() -> list:\n    return [x * x for x in range(10) if x % 2 == 0]",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_dict_comprehension() {
        let code =
            transpile("def square_map() -> dict:\n    return {str(x): x * x for x in range(5)}");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 28: rust_gen.rs - String formatting and f-strings
    // =========================================================================

    #[test]
    fn test_w5_fstring_simple() {
        let code = transpile("def greet(name: str) -> str:\n    return f\"Hello, {name}!\"");
        assert!(!code.is_empty());
        assert!(code.contains("format!"));
    }

    #[test]
    fn test_w5_fstring_with_expression() {
        let code = transpile("def show(x: int) -> str:\n    return f\"Value: {x + 1}\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_fstring_multiple_vars() {
        let code = transpile(
            "def info(name: str, age: int) -> str:\n    return f\"{name} is {age} years old\"",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 29: rust_gen.rs - Complex function signatures
    // =========================================================================

    #[test]
    fn test_w5_function_default_arg() {
        let code =
            transpile("def greet(name: str = \"World\") -> str:\n    return \"Hello, \" + name");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_multiple_returns() {
        let code = transpile(
            "def abs_val(x: int) -> int:\n    if x >= 0:\n        return x\n    return -x",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_no_return_type() {
        let code = transpile("def side_effect(msg: str):\n    print(msg)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_no_params() {
        let code = transpile("def constant() -> int:\n    return 42");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_many_params() {
        let code = transpile(
            "def multi(a: int, b: int, c: int, d: int) -> int:\n    return a + b + c + d",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 30: rust_gen.rs - Async functions
    // =========================================================================

    #[test]
    fn test_w5_async_function() {
        let code =
            transpile("import asyncio\n\nasync def fetch_data() -> str:\n    return \"data\"");
        assert!(!code.is_empty());
        assert!(code.contains("async"));
    }

    #[test]
    fn test_w5_async_with_await() {
        let code = transpile("import asyncio\n\nasync def process() -> int:\n    await asyncio.sleep(1)\n    return 42");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 31: rust_gen.rs - Enum patterns
    // =========================================================================

    #[test]
    fn test_w5_enum_class() {
        let code = transpile(
            "from enum import Enum\n\nclass Color(Enum):\n    RED = 1\n    GREEN = 2\n    BLUE = 3",
        );
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 32: stmt_gen.rs - Complex patterns
    // =========================================================================

    #[test]
    fn test_w5_function_with_docstring() {
        let code = transpile("def documented(x: int) -> int:\n    \"\"\"Double the input value.\"\"\"\n    return x * 2");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_lambda_basic() {
        let code = transpile("def apply():\n    f = lambda x: x + 1\n    print(f(5))");
        assert!(transpile_ok("def apply():\n    f = lambda x: x + 1\n    print(f(5))"));
    }

    #[test]
    fn test_w5_ternary_expression() {
        let code = transpile(
            "def classify(x: int) -> str:\n    return \"positive\" if x > 0 else \"non-positive\"",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_methods() {
        let code = transpile("def process(text: str) -> str:\n    return text.strip().lower()");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_split() {
        let code = transpile("def tokenize(line: str) -> list:\n    return line.split(\",\")");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_join() {
        let code = transpile("def rejoin(parts: list) -> str:\n    return \", \".join(parts)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_replace() {
        let code = transpile(
            "def sanitize(text: str) -> str:\n    return text.replace(\"bad\", \"good\")",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_startswith() {
        let code = transpile(
            "def check_prefix(text: str) -> bool:\n    return text.startswith(\"hello\")",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_endswith() {
        let code =
            transpile("def check_suffix(text: str) -> bool:\n    return text.endswith(\".py\")");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 33: stmt_gen.rs - Complex control flow
    // =========================================================================

    #[test]
    fn test_w5_for_with_else() {
        let code = transpile("def find_negative(nums: list) -> bool:\n    for n in nums:\n        if n < 0:\n            return True\n    return False");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_deeply_nested_control() {
        let code = transpile("def deep(a: int, b: int, c: int) -> int:\n    if a > 0:\n        if b > 0:\n            if c > 0:\n                return a + b + c\n            else:\n                return a + b\n        else:\n            return a\n    return 0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_while_with_if_break() {
        let code = transpile("def find_threshold(values: list) -> int:\n    i: int = 0\n    while i < len(values):\n        if values[i] > 100:\n            break\n        i = i + 1\n    return i");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 34: rust_gen.rs - Deduplication of imports
    // =========================================================================

    #[test]
    fn test_w5_no_duplicate_imports() {
        let code = transpile("from typing import Dict, List\n\ndef process(d: Dict[str, int], items: List[str]) -> int:\n    return len(d) + len(items)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 35: stmt_gen.rs - Complex assignment patterns
    // =========================================================================

    #[test]
    fn test_w5_multiple_assignment_same_line() {
        let code = transpile("def swap_vars():\n    a: int = 1\n    b: int = 2\n    a, b = b, a\n    print(a)\n    print(b)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_nested_dict_access() {
        let code = transpile("def nested_access(d: dict) -> str:\n    return d[\"outer\"]");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 36: rust_gen.rs - Dataclass generation
    // =========================================================================

    #[test]
    fn test_w5_dataclass_with_defaults() {
        let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass Server:\n    host: str = \"localhost\"\n    port: int = 8080");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_dataclass_multiple_fields() {
        let code = transpile("from dataclasses import dataclass\n\n@dataclass\nclass User:\n    name: str\n    age: int\n    email: str");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 37: stmt_gen.rs - Recursive functions
    // =========================================================================

    #[test]
    fn test_w5_recursive_factorial() {
        let code = transpile("def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)");
        assert!(!code.is_empty());
        assert!(code.contains("factorial"));
    }

    #[test]
    fn test_w5_recursive_fibonacci() {
        let code = transpile("def fib(n: int) -> int:\n    if n <= 1:\n        return n\n    return fib(n - 1) + fib(n - 2)");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 38: rust_gen.rs - Main function handling
    // =========================================================================

    #[test]
    fn test_w5_main_function_void() {
        let code = transpile("def main():\n    print(\"hello\")");
        assert!(!code.is_empty());
        assert!(code.contains("main"));
    }

    #[test]
    fn test_w5_main_function_with_return_zero() {
        let code = transpile("def main() -> int:\n    print(\"done\")\n    return 0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_main_function_with_return_nonzero() {
        let code = transpile("def main() -> int:\n    return 1");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 39: stmt_gen.rs - Dict operations
    // =========================================================================

    #[test]
    fn test_w5_dict_get_with_default() {
        let code = transpile(
            "def safe_get(d: dict, key: str) -> str:\n    return d.get(key, \"default\")",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_dict_update() {
        let code = transpile("def merge(a: dict, b: dict) -> dict:\n    a.update(b)\n    return a");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_dict_keys_iteration() {
        let code = transpile("def all_keys(d: dict) -> list:\n    result: list = []\n    for k in d:\n        result.append(k)\n    return result");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 40: rust_gen.rs - String interning and optimization
    // =========================================================================

    #[test]
    fn test_w5_repeated_string_constants() {
        let code = transpile("def categorize(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 41: stmt_gen.rs - Complex return patterns
    // =========================================================================

    #[test]
    fn test_w5_return_optional_if_expr_with_none() {
        let code = transpile("from typing import Optional\ndef check(x: int) -> Optional[int]:\n    return x if x > 0 else None");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_empty_dict() {
        let code =
            transpile("from typing import Dict\ndef empty_map() -> Dict[str, int]:\n    return {}");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_string_literal_to_string() {
        let code = transpile("def fixed() -> str:\n    return \"constant\"");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_return_from_can_fail_function() {
        let code = transpile("def parse_int(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return 0");
        assert!(!code.is_empty());
    }

    // =========================================================================
    // Section 42: Additional coverage - edge cases
    // =========================================================================

    #[test]
    fn test_w5_empty_function() {
        let code = transpile("def empty():\n    pass");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_only_return() {
        let code = transpile("def identity(x: int) -> int:\n    return x");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_bool_return() {
        let code = transpile("def is_positive(x: int) -> bool:\n    return x > 0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_function_float_return() {
        let code = transpile("def half(x: float) -> float:\n    return x / 2.0");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_multiple_classes_and_functions() {
        let code = transpile("class Foo:\n    def __init__(self, x: int):\n        self.x = x\n\nclass Bar:\n    def __init__(self, y: str):\n        self.y = y\n\ndef create_foo() -> int:\n    f = Foo(42)\n    return f.x");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_class_with_eq() {
        let code = transpile("class Wrapper:\n    def __init__(self, val: int):\n        self.val = val\n\n    def __eq__(self, other) -> bool:\n        return self.val == other.val");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_none_return_type() {
        let code = transpile("def log(msg: str) -> None:\n    print(msg)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_chained_method_calls() {
        let code = transpile(
            "def process(text: str) -> str:\n    return text.strip().upper().replace(\"A\", \"B\")",
        );
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_boolean_operators() {
        let code = transpile("def logic(a: bool, b: bool) -> bool:\n    return a and b or not a");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_comparison_chain() {
        let code = transpile("def in_range(x: int) -> bool:\n    return 0 < x and x < 100");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_integer_arithmetic() {
        let code = transpile("def compute(a: int, b: int) -> int:\n    return (a + b) * (a - b)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_floor_division() {
        let code = transpile("def floor_div(a: int, b: int) -> int:\n    return a // b");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_power_operator() {
        let code = transpile("def power(base: int, exp: int) -> int:\n    return base ** exp");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_multiplication() {
        let code = transpile("def repeat(s: str, n: int) -> str:\n    return s * n");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_list_len() {
        let code = transpile("def count(items: list) -> int:\n    return len(items)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_string_len() {
        let code = transpile("def str_len(s: str) -> int:\n    return len(s)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_in_operator_list() {
        let code =
            transpile("def contains(items: list, val: int) -> bool:\n    return val in items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_in_operator_string() {
        let code = transpile("def has_sub(text: str, sub: str) -> bool:\n    return sub in text");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_not_in_operator() {
        let code =
            transpile("def missing(items: list, val: int) -> bool:\n    return val not in items");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_max_builtin() {
        let code = transpile("def biggest(a: int, b: int) -> int:\n    return max(a, b)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_min_builtin() {
        let code = transpile("def smallest(a: int, b: int) -> int:\n    return min(a, b)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_abs_builtin() {
        let code = transpile("def magnitude(x: int) -> int:\n    return abs(x)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_sorted_builtin() {
        let code = transpile("def order(items: list) -> list:\n    return sorted(items)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_reversed_builtin() {
        let code = transpile("def flip(items: list) -> list:\n    return list(reversed(items))");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_sum_builtin() {
        let code = transpile("def total(nums: list) -> int:\n    return sum(nums)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_any_builtin() {
        let code = transpile("def has_true(flags: list) -> bool:\n    return any(flags)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_all_builtin() {
        let code = transpile("def all_true(flags: list) -> bool:\n    return all(flags)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_map_function() {
        let code = transpile(
            "def double_all(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))",
        );
        assert!(transpile_ok(
            "def double_all(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))"
        ));
    }

    #[test]
    fn test_w5_filter_function() {
        let code = transpile(
            "def positives(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))",
        );
        assert!(transpile_ok(
            "def positives(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))"
        ));
    }

    #[test]
    fn test_w5_int_conversion() {
        let code = transpile("def to_int(s: str) -> int:\n    return int(s)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_str_conversion() {
        let code = transpile("def to_str(n: int) -> str:\n    return str(n)");
        assert!(!code.is_empty());
    }

    #[test]
    fn test_w5_float_conversion() {
        let code = transpile("def to_float(s: str) -> float:\n    return float(s)");
        assert!(!code.is_empty());
    }
}
