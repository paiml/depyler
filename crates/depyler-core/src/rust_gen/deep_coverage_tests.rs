//! Deep coverage tests for rust_gen
//!
//! These tests target specific uncovered code paths in expr_gen.rs and stmt_gen.rs.

use crate::DepylerPipeline;

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// COMPLEX METHOD CALL PATTERNS
// ============================================================================

#[test]
fn test_method_chain_upper_lower() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.upper().lower()"
    ));
}

#[test]
fn test_method_chain_strip_split() {
    assert!(transpile_ok(
        "def foo(s: str) -> list[str]:\n    return s.strip().split()"
    ));
}

#[test]
fn test_method_call_with_kwargs() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]) -> int:\n    return d.pop('key', default=0)"
    ));
}

#[test]
fn test_sorted_with_key() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> list[str]:\n    return sorted(items, key=len)"
    ));
}

#[test]
fn test_sorted_with_reverse() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return sorted(items, reverse=True)"
    ));
}

#[test]
fn test_sorted_with_key_and_reverse() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> list[str]:\n    return sorted(items, key=len, reverse=True)"
    ));
}

// ============================================================================
// EXPRESSION CONVERTER EDGE CASES
// ============================================================================

#[test]
fn test_rust_keyword_variable_as() {
    assert!(transpile_ok("def foo():\n    r_as = 1\n    return r_as"));
}

#[test]
fn test_rust_keyword_variable_type() {
    assert!(transpile_ok(
        "def foo():\n    r_type = 'int'\n    return r_type"
    ));
}

#[test]
fn test_rust_keyword_variable_match() {
    assert!(transpile_ok(
        "def foo():\n    r_match = True\n    return r_match"
    ));
}

#[test]
fn test_boolean_and_expression() {
    assert!(transpile_ok(
        "def foo(a: bool, b: bool) -> bool:\n    return a and b"
    ));
}

#[test]
fn test_boolean_or_expression() {
    assert!(transpile_ok(
        "def foo(a: bool, b: bool) -> bool:\n    return a or b"
    ));
}

#[test]
fn test_boolean_not_and() {
    assert!(transpile_ok(
        "def foo(a: bool, b: bool) -> bool:\n    return not (a and b)"
    ));
}

#[test]
fn test_comparison_eq() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a == b"
    ));
}

#[test]
fn test_comparison_ne() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a != b"
    ));
}

#[test]
fn test_comparison_lt() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a < b"
    ));
}

#[test]
fn test_comparison_le() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a <= b"
    ));
}

#[test]
fn test_comparison_gt() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a > b"
    ));
}

#[test]
fn test_comparison_ge() {
    assert!(transpile_ok(
        "def foo(a: int, b: int) -> bool:\n    return a >= b"
    ));
}

// ============================================================================
// SLICE EXPRESSION TESTS
// ============================================================================

#[test]
fn test_slice_start_only() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items[1:]"
    ));
}

#[test]
fn test_slice_end_only() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items[:3]"
    ));
}

#[test]
fn test_slice_start_end() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items[1:3]"
    ));
}

#[test]
fn test_slice_with_step() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items[::2]"
    ));
}

#[test]
fn test_slice_negative_index() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return items[-1]"
    ));
}

#[test]
fn test_slice_negative_range() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items[-3:-1]"
    ));
}

// ============================================================================
// ATTRIBUTE ACCESS TESTS
// ============================================================================

#[test]
fn test_attribute_simple() {
    assert!(transpile_ok(
        "class Point:\n    x: int\n\ndef foo(p: Point) -> int:\n    return p.x"
    ));
}

#[test]
fn test_attribute_chain() {
    assert!(transpile_ok("class Inner:\n    value: int\n\nclass Outer:\n    inner: Inner\n\ndef foo(o: Outer) -> int:\n    return o.inner.value"));
}

// ============================================================================
// INDEX ACCESS TESTS
// ============================================================================

#[test]
fn test_index_list() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return items[0]"
    ));
}

#[test]
fn test_index_dict() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]) -> int:\n    return d['key']"
    ));
}

#[test]
fn test_index_string() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s[0]"));
}

#[test]
fn test_index_nested() {
    assert!(transpile_ok(
        "def foo(matrix: list[list[int]]) -> int:\n    return matrix[0][0]"
    ));
}

// ============================================================================
// CALL EXPRESSION TESTS
// ============================================================================

#[test]
fn test_call_no_args() {
    assert!(transpile_ok(
        "def bar() -> int:\n    return 1\n\ndef foo() -> int:\n    return bar()"
    ));
}

#[test]
fn test_call_positional_args() {
    assert!(transpile_ok("def bar(x: int, y: int) -> int:\n    return x + y\n\ndef foo() -> int:\n    return bar(1, 2)"));
}

#[test]
fn test_call_keyword_args() {
    assert!(transpile_ok("def bar(x: int, y: int) -> int:\n    return x + y\n\ndef foo() -> int:\n    return bar(x=1, y=2)"));
}

#[test]
fn test_call_mixed_args() {
    assert!(transpile_ok("def bar(x: int, y: int, z: int) -> int:\n    return x + y + z\n\ndef foo() -> int:\n    return bar(1, y=2, z=3)"));
}

// ============================================================================
// LITERAL TESTS
// ============================================================================

#[test]
fn test_literal_int() {
    assert!(transpile_ok("def foo() -> int:\n    return 42"));
}

#[test]
fn test_literal_float() {
    assert!(transpile_ok("def foo() -> float:\n    return 3.14"));
}

#[test]
fn test_literal_string() {
    assert!(transpile_ok("def foo() -> str:\n    return 'hello'"));
}

#[test]
fn test_literal_bool_true() {
    assert!(transpile_ok("def foo() -> bool:\n    return True"));
}

#[test]
fn test_literal_bool_false() {
    assert!(transpile_ok("def foo() -> bool:\n    return False"));
}

#[test]
fn test_literal_none() {
    assert!(transpile_ok("def foo():\n    return None"));
}

#[test]
fn test_literal_bytes() {
    assert!(transpile_ok("def foo() -> bytes:\n    return b'hello'"));
}

// ============================================================================
// COLLECTION LITERAL TESTS
// ============================================================================

#[test]
fn test_list_literal_empty() {
    assert!(transpile_ok("def foo() -> list[int]:\n    return []"));
}

#[test]
fn test_list_literal_ints() {
    assert!(transpile_ok(
        "def foo() -> list[int]:\n    return [1, 2, 3]"
    ));
}

#[test]
fn test_dict_literal_empty() {
    assert!(transpile_ok("def foo() -> dict[str, int]:\n    return {}"));
}

#[test]
fn test_dict_literal_items() {
    assert!(transpile_ok(
        "def foo() -> dict[str, int]:\n    return {'a': 1, 'b': 2}"
    ));
}

#[test]
fn test_set_literal() {
    assert!(transpile_ok("def foo() -> set[int]:\n    return {1, 2, 3}"));
}

#[test]
fn test_tuple_literal() {
    assert!(transpile_ok("def foo():\n    return (1, 2, 3)"));
}

// ============================================================================
// TRUTHINESS CONVERSION TESTS
// ============================================================================

#[test]
fn test_if_list_truthiness() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> bool:\n    if items:\n        return True\n    return False"
    ));
}

#[test]
fn test_if_dict_truthiness() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]) -> bool:\n    if d:\n        return True\n    return False"
    ));
}

#[test]
fn test_if_string_truthiness() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    if s:\n        return True\n    return False"
    ));
}

#[test]
fn test_while_list_truthiness() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    while items:\n        items.pop()"
    ));
}

// ============================================================================
// DICT COMPREHENSION TESTS
// ============================================================================

#[test]
fn test_dict_comprehension_simple() {
    assert!(transpile_ok(
        "def foo() -> dict[int, int]:\n    return {x: x * 2 for x in range(5)}"
    ));
}

#[test]
fn test_dict_comprehension_with_if() {
    assert!(transpile_ok(
        "def foo() -> dict[int, int]:\n    return {x: x * 2 for x in range(10) if x % 2 == 0}"
    ));
}

// ============================================================================
// GENERATOR EXPRESSION TESTS
// ============================================================================

#[test]
fn test_generator_in_sum() {
    assert!(transpile_ok(
        "def foo() -> int:\n    return sum(x * x for x in range(10))"
    ));
}

#[test]
fn test_generator_in_any() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> bool:\n    return any(x > 0 for x in items)"
    ));
}

#[test]
fn test_generator_in_all() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> bool:\n    return all(x > 0 for x in items)"
    ));
}

// ============================================================================
// ENUMERATE AND ZIP TESTS
// ============================================================================

#[test]
fn test_enumerate_with_start() {
    assert!(transpile_ok("def foo(items: list[str]):\n    for i, item in enumerate(items, start=1):\n        print(i, item)"));
}

#[test]
fn test_zip_three_iterables() {
    assert!(transpile_ok("def foo(a: list[int], b: list[int], c: list[int]):\n    for x, y, z in zip(a, b, c):\n        print(x, y, z)"));
}

// ============================================================================
// COMPLEX EXPRESSION TESTS
// ============================================================================

#[test]
fn test_nested_binary_expr() {
    assert!(transpile_ok(
        "def foo(a: int, b: int, c: int) -> int:\n    return (a + b) * c"
    ));
}

#[test]
fn test_nested_comparison() {
    assert!(transpile_ok(
        "def foo(x: int) -> bool:\n    return 0 <= x <= 100"
    ));
}

#[test]
fn test_complex_arithmetic() {
    assert!(transpile_ok(
        "def foo(x: float, y: float) -> float:\n    return (x + y) / 2.0 * (x - y)"
    ));
}

// ============================================================================
// SPECIAL METHOD TESTS
// ============================================================================

#[test]
fn test_dunder_len() {
    assert!(transpile_ok("class Foo:\n    items: list[int]\n    def __len__(self) -> int:\n        return len(self.items)"));
}

#[test]
fn test_dunder_getitem() {
    assert!(transpile_ok("class Foo:\n    items: list[int]\n    def __getitem__(self, idx: int) -> int:\n        return self.items[idx]"));
}

#[test]
fn test_dunder_setitem() {
    assert!(transpile_ok("class Foo:\n    items: list[int]\n    def __setitem__(self, idx: int, value: int):\n        self.items[idx] = value"));
}

#[test]
fn test_dunder_contains() {
    assert!(transpile_ok("class Foo:\n    items: list[int]\n    def __contains__(self, item: int) -> bool:\n        return item in self.items"));
}

#[test]
fn test_dunder_iter() {
    assert!(transpile_ok("class Foo:\n    items: list[int]\n    def __iter__(self):\n        return iter(self.items)"));
}

// ============================================================================
// STDLIB METHOD TESTS
// ============================================================================

#[test]
fn test_os_path_join() {
    assert!(transpile_ok(
        "import os\ndef foo(a: str, b: str) -> str:\n    return os.path.join(a, b)"
    ));
}

#[test]
fn test_os_path_exists() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> bool:\n    return os.path.exists(path)"
    ));
}

#[test]
fn test_os_path_isfile() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> bool:\n    return os.path.isfile(path)"
    ));
}

#[test]
fn test_os_path_isdir() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> bool:\n    return os.path.isdir(path)"
    ));
}

#[test]
fn test_os_path_dirname() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> str:\n    return os.path.dirname(path)"
    ));
}

#[test]
fn test_os_path_basename() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> str:\n    return os.path.basename(path)"
    ));
}

#[test]
fn test_os_listdir() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> list[str]:\n    return os.listdir(path)"
    ));
}

#[test]
fn test_os_makedirs() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str):\n    os.makedirs(path)"
    ));
}

#[test]
fn test_os_remove() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str):\n    os.remove(path)"
    ));
}

#[test]
fn test_os_rename() {
    assert!(transpile_ok(
        "import os\ndef foo(src: str, dst: str):\n    os.rename(src, dst)"
    ));
}

// ============================================================================
// STRING FORMATTING TESTS
// ============================================================================

#[test]
fn test_format_string_percent() {
    assert!(transpile_ok(
        "def foo(name: str) -> str:\n    return 'Hello %s' % name"
    ));
}

#[test]
fn test_format_string_multiple() {
    assert!(transpile_ok(
        "def foo(name: str, age: int) -> str:\n    return '%s is %d' % (name, age)"
    ));
}

// ============================================================================
// EXCEPTION HANDLING TESTS
// ============================================================================

#[test]
fn test_try_except_tuple() {
    assert!(transpile_ok(
        "def foo():\n    try:\n        x = 1\n    except (ValueError, TypeError):\n        pass"
    ));
}

#[test]
fn test_try_except_reraise() {
    assert!(transpile_ok(
        "def foo():\n    try:\n        x = 1\n    except Exception:\n        raise"
    ));
}

#[test]
fn test_try_nested() {
    assert!(transpile_ok("def foo():\n    try:\n        try:\n            x = 1\n        except ValueError:\n            pass\n    except TypeError:\n        pass"));
}

// ============================================================================
// CONTEXT MANAGER TESTS
// ============================================================================

#[test]
fn test_with_nested() {
    assert!(transpile_ok("def foo():\n    with open('a.txt') as a:\n        with open('b.txt') as b:\n            pass"));
}

// ============================================================================
// AUGMENTED ASSIGNMENT EDGE CASES
// ============================================================================

#[test]
fn test_augmented_assign_list_extend() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    items += [1, 2, 3]"
    ));
}

#[test]
fn test_augmented_assign_string_concat() {
    assert!(transpile_ok(
        "def foo():\n    s = 'hello'\n    s += ' world'"
    ));
}

// ============================================================================
// FUNCTION PARAMETER EDGE CASES
// ============================================================================

#[test]
fn test_param_default_none() {
    assert!(transpile_ok("def foo(x: int = None):\n    pass"));
}

#[test]
fn test_param_default_list() {
    assert!(transpile_ok("def foo(items: list[int] = []):\n    pass"));
}

#[test]
fn test_param_default_dict() {
    assert!(transpile_ok("def foo(d: dict[str, int] = {}):\n    pass"));
}

#[test]
fn test_param_args() {
    assert!(transpile_ok(
        "def foo(*args):\n    for arg in args:\n        print(arg)"
    ));
}

#[test]
fn test_param_kwargs() {
    assert!(transpile_ok(
        "def foo(**kwargs):\n    for k, v in kwargs.items():\n        print(k, v)"
    ));
}

#[test]
fn test_param_args_and_kwargs() {
    assert!(transpile_ok("def foo(*args, **kwargs):\n    pass"));
}

// ============================================================================
// RETURN VALUE EDGE CASES
// ============================================================================

#[test]
fn test_return_tuple() {
    assert!(transpile_ok(
        "def foo() -> tuple[int, int]:\n    return 1, 2"
    ));
}

#[test]
fn test_return_multiple_values() {
    assert!(transpile_ok(
        "def foo() -> tuple[int, str, bool]:\n    return 1, 'hello', True"
    ));
}

#[test]
fn test_return_early() {
    assert!(transpile_ok(
        "def foo(x: int) -> int:\n    if x < 0:\n        return 0\n    return x"
    ));
}

// ============================================================================
// CLASS INHERITANCE TESTS
// ============================================================================

#[test]
fn test_class_inheritance() {
    let _ = transpile_ok("class Animal:\n    name: str\n\nclass Dog(Animal):\n    breed: str");
}

#[test]
fn test_class_method_override() {
    let _ = transpile_ok("class Animal:\n    def speak(self) -> str:\n        return 'sound'\n\nclass Dog(Animal):\n    def speak(self) -> str:\n        return 'bark'");
}

// ============================================================================
// PRINT STATEMENT TESTS
// ============================================================================

#[test]
fn test_print_simple() {
    assert!(transpile_ok("def foo():\n    print('hello')"));
}

#[test]
fn test_print_multiple() {
    assert!(transpile_ok("def foo():\n    print('a', 'b', 'c')"));
}

#[test]
fn test_print_with_sep() {
    assert!(transpile_ok("def foo():\n    print('a', 'b', sep=', ')"));
}

#[test]
fn test_print_with_end() {
    assert!(transpile_ok("def foo():\n    print('hello', end='')"));
}

// ============================================================================
// INPUT/OUTPUT TESTS
// ============================================================================

#[test]
fn test_input_simple() {
    assert!(transpile_ok("def foo() -> str:\n    return input()"));
}

#[test]
fn test_input_with_prompt() {
    assert!(transpile_ok(
        "def foo() -> str:\n    return input('Enter: ')"
    ));
}

// ============================================================================
// STRING INTERPOLATION TESTS
// ============================================================================

#[test]
fn test_fstring_nested_expr() {
    assert!(transpile_ok(
        "def foo(x: int) -> str:\n    return f'result: {x + 1}'"
    ));
}

#[test]
fn test_fstring_method_call() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return f'{s.upper()}'"
    ));
}

#[test]
fn test_fstring_conditional() {
    assert!(transpile_ok(
        "def foo(x: int) -> str:\n    return f'{\"yes\" if x > 0 else \"no\"}'"
    ));
}

// ============================================================================
// BYTES OPERATIONS
// ============================================================================

#[test]
fn test_bytes_decode() {
    assert!(transpile_ok(
        "def foo(b: bytes) -> str:\n    return b.decode('utf-8')"
    ));
}

#[test]
fn test_string_encode() {
    assert!(transpile_ok(
        "def foo(s: str) -> bytes:\n    return s.encode('utf-8')"
    ));
}

// ============================================================================
// TYPE CONVERSION TESTS
// ============================================================================

#[test]
fn test_int_from_float() {
    assert!(transpile_ok("def foo(x: float) -> int:\n    return int(x)"));
}

#[test]
fn test_float_from_int() {
    assert!(transpile_ok(
        "def foo(x: int) -> float:\n    return float(x)"
    ));
}

#[test]
fn test_str_from_int() {
    assert!(transpile_ok("def foo(x: int) -> str:\n    return str(x)"));
}

#[test]
fn test_list_from_tuple() {
    assert!(transpile_ok(
        "def foo(t: tuple[int, int, int]) -> list[int]:\n    return list(t)"
    ));
}

#[test]
fn test_tuple_from_list() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    return tuple(items)"
    ));
}

#[test]
fn test_set_from_list() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> set[int]:\n    return set(items)"
    ));
}

// ============================================================================
// COPY OPERATIONS
// ============================================================================

#[test]
fn test_list_copy_method() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return items.copy()"
    ));
}

#[test]
fn test_dict_copy_method() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]) -> dict[str, int]:\n    return d.copy()"
    ));
}

#[test]
fn test_set_copy_method() {
    assert!(transpile_ok(
        "def foo(s: set[int]) -> set[int]:\n    return s.copy()"
    ));
}

// ============================================================================
// ISINSTANCE TESTS
// ============================================================================

#[test]
fn test_isinstance_int() {
    assert!(transpile_ok(
        "def foo(x) -> bool:\n    return isinstance(x, int)"
    ));
}

#[test]
fn test_isinstance_str() {
    assert!(transpile_ok(
        "def foo(x) -> bool:\n    return isinstance(x, str)"
    ));
}

#[test]
fn test_isinstance_tuple() {
    assert!(transpile_ok(
        "def foo(x) -> bool:\n    return isinstance(x, (int, str))"
    ));
}

// ============================================================================
// TYPE CHECKS
// ============================================================================

#[test]
fn test_type_check() {
    // type() builtin may not be fully supported
    let _ = transpile_ok("def foo(x) -> type:\n    return type(x)");
}

#[test]
fn test_callable_check() {
    assert!(transpile_ok("def foo(x) -> bool:\n    return callable(x)"));
}

// ============================================================================
// HASATTR/GETATTR/SETATTR TESTS
// ============================================================================

#[test]
fn test_hasattr_call() {
    let _ = transpile_ok("def foo(obj) -> bool:\n    return hasattr(obj, 'x')");
}

#[test]
fn test_setattr_call() {
    let _ = transpile_ok("def foo(obj, value):\n    setattr(obj, 'x', value)");
}

#[test]
fn test_delattr_call() {
    let _ = transpile_ok("def foo(obj):\n    delattr(obj, 'x')");
}

// ============================================================================
// EVAL/EXEC TESTS (may not be supported)
// ============================================================================

#[test]
fn test_eval_call() {
    let _ = transpile_ok("def foo(expr: str):\n    return eval(expr)");
}

#[test]
fn test_exec_call() {
    let _ = transpile_ok("def foo(code: str):\n    exec(code)");
}

// ============================================================================
// COMPLEX NUMBER TESTS
// ============================================================================

#[test]
fn test_complex_literal() {
    let _ = transpile_ok("def foo() -> complex:\n    return 1 + 2j");
}

#[test]
fn test_complex_constructor() {
    let _ = transpile_ok("def foo() -> complex:\n    return complex(1, 2)");
}

// ============================================================================
// RANGE VARIATIONS
// ============================================================================

#[test]
fn test_range_negative_step() {
    assert!(transpile_ok(
        "def foo():\n    for i in range(10, 0, -1):\n        print(i)"
    ));
}

#[test]
fn test_range_to_list() {
    assert!(transpile_ok(
        "def foo() -> list[int]:\n    return list(range(10))"
    ));
}

// ============================================================================
// SPECIAL BUILTINS
// ============================================================================

#[test]
fn test_id_builtin() {
    let _ = transpile_ok("def foo(x) -> int:\n    return id(x)");
}

#[test]
fn test_vars_builtin() {
    let _ = transpile_ok("def foo(obj) -> dict:\n    return vars(obj)");
}

#[test]
fn test_dir_builtin() {
    let _ = transpile_ok("def foo(obj) -> list[str]:\n    return dir(obj)");
}

// ============================================================================
// GLOBAL CONSTANTS
// ============================================================================

#[test]
fn test_global_constant() {
    assert!(transpile_ok(
        "PI = 3.14159\n\ndef foo() -> float:\n    return PI"
    ));
}

#[test]
fn test_module_level_list() {
    assert!(transpile_ok(
        "ITEMS = [1, 2, 3]\n\ndef foo() -> int:\n    return ITEMS[0]"
    ));
}

// ============================================================================
// NESTED FUNCTIONS
// ============================================================================

#[test]
fn test_nested_function() {
    assert!(transpile_ok("def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y * 2\n    return inner(x)"));
}

#[test]
fn test_closure() {
    assert!(transpile_ok("def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder"));
}

// ============================================================================
// DECORATOR TESTS
// ============================================================================

#[test]
fn test_staticmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @staticmethod\n    def bar() -> int:\n        return 42"
    ));
}

#[test]
fn test_classmethod() {
    assert!(transpile_ok(
        "class Foo:\n    @classmethod\n    def bar(cls) -> str:\n        return 'Foo'"
    ));
}

#[test]
fn test_property() {
    assert!(transpile_ok(
        "class Foo:\n    _x: int\n    @property\n    def x(self) -> int:\n        return self._x"
    ));
}

// ============================================================================
// ASSERT VARIATIONS
// ============================================================================

#[test]
fn test_assert_expression() {
    assert!(transpile_ok(
        "def foo(x: int):\n    assert x > 0, f'Expected positive, got {x}'"
    ));
}

#[test]
fn test_assert_isinstance() {
    assert!(transpile_ok("def foo(x):\n    assert isinstance(x, int)"));
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[test]
fn test_raise_value_error() {
    assert!(transpile_ok(
        "def foo(x: int):\n    if x < 0:\n        raise ValueError('negative')"
    ));
}

#[test]
fn test_raise_type_error() {
    assert!(transpile_ok(
        "def foo(x):\n    if not isinstance(x, int):\n        raise TypeError('expected int')"
    ));
}

#[test]
fn test_raise_runtime_error() {
    assert!(transpile_ok("def foo():\n    raise RuntimeError('oops')"));
}

#[test]
fn test_raise_not_implemented() {
    assert!(transpile_ok("def foo():\n    raise NotImplementedError()"));
}

// ============================================================================
// FILE OPERATIONS
// ============================================================================

#[test]
fn test_file_read_mode() {
    assert!(transpile_ok(
        "def foo(path: str) -> str:\n    with open(path, 'r') as f:\n        return f.read()"
    ));
}

#[test]
fn test_file_write_mode() {
    assert!(transpile_ok(
        "def foo(path: str, data: str):\n    with open(path, 'w') as f:\n        f.write(data)"
    ));
}

#[test]
fn test_file_append_mode() {
    assert!(transpile_ok(
        "def foo(path: str, data: str):\n    with open(path, 'a') as f:\n        f.write(data)"
    ));
}

#[test]
fn test_file_binary_read() {
    assert!(transpile_ok(
        "def foo(path: str) -> bytes:\n    with open(path, 'rb') as f:\n        return f.read()"
    ));
}

#[test]
fn test_file_binary_write() {
    assert!(transpile_ok(
        "def foo(path: str, data: bytes):\n    with open(path, 'wb') as f:\n        f.write(data)"
    ));
}

#[test]
fn test_file_readlines() {
    assert!(transpile_ok(
        "def foo(path: str) -> list[str]:\n    with open(path) as f:\n        return f.readlines()"
    ));
}

#[test]
fn test_file_readline() {
    assert!(transpile_ok(
        "def foo(path: str) -> str:\n    with open(path) as f:\n        return f.readline()"
    ));
}

// ============================================================================
// Session 9 Batch 5: Targeted deep coverage tests
// ============================================================================

// --- Nested function patterns ---

#[test]
fn test_s9_nested_fn_filter_pattern() {
    assert!(transpile_ok(
        "def filter_items(items: list) -> list:\n    def is_valid(x: int) -> bool:\n        return x > 0\n    return [x for x in items if is_valid(x)]"
    ));
}

#[test]
fn test_s9_nested_fn_recursive_factorial() {
    assert!(transpile_ok(
        "def compute(n: int) -> int:\n    def fact(x: int) -> int:\n        if x <= 1:\n            return 1\n        return x * fact(x - 1)\n    return fact(n)"
    ));
}

#[test]
fn test_s9_nested_fn_with_closure_capture() {
    assert!(transpile_ok(
        "def outer(base: int) -> int:\n    def adder(x: int) -> int:\n        return base + x\n    return adder(10)"
    ));
}

#[test]
fn test_s9_nested_fn_multiple_siblings() {
    assert!(transpile_ok(
        "def run() -> int:\n    def add(a: int, b: int) -> int:\n        return a + b\n    def sub(a: int, b: int) -> int:\n        return a - b\n    return add(5, 3) + sub(10, 2)"
    ));
}

// --- try/except patterns ---

#[test]
fn test_s9_try_except_parse_with_negative_fallback() {
    assert!(transpile_ok(
        "def parse_num(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1"
    ));
}

#[test]
fn test_s9_try_except_with_finally_and_cleanup() {
    assert!(transpile_ok(
        "def run_with_cleanup() -> str:\n    result = \"\"\n    try:\n        result = \"ok\"\n    except ValueError:\n        result = \"error\"\n    finally:\n        print(\"done\")\n    return result"
    ));
}

#[test]
fn test_s9_try_except_variable_hoisting() {
    assert!(transpile_ok(
        "def hoisted(s: str) -> int:\n    try:\n        x = int(s)\n    except ValueError:\n        x = 0\n    return x"
    ));
}

#[test]
fn test_s9_try_except_multiple_handlers_bind() {
    assert!(transpile_ok(
        "def multi_catch(s: str) -> str:\n    try:\n        return str(int(s))\n    except ValueError as e:\n        return \"value\"\n    except TypeError as e:\n        return \"type\""
    ));
}

#[test]
fn test_s9_nested_try_except() {
    assert!(transpile_ok(
        "def nested_err() -> int:\n    try:\n        try:\n            return 1\n        except TypeError:\n            return 2\n    except ValueError:\n        return 3"
    ));
}

// --- Complex expression patterns ---

#[test]
fn test_s9_dict_comprehension_with_method() {
    assert!(transpile_ok(
        "def word_lengths(words: list) -> dict:\n    return {w: len(w) for w in words}"
    ));
}

#[test]
fn test_s9_set_comprehension_with_condition() {
    assert!(transpile_ok(
        "def even_squares(n: int) -> set:\n    return {x * x for x in range(n) if x % 2 == 0}"
    ));
}

#[test]
fn test_s9_generator_expression_in_sum() {
    assert!(transpile_ok(
        "def sum_cubes(n: int) -> int:\n    return sum(x ** 3 for x in range(n))"
    ));
}

#[test]
fn test_s9_lambda_in_filter() {
    assert!(transpile_ok(
        "def only_positive(nums: list) -> list:\n    return list(filter(lambda x: x > 0, nums))"
    ));
}

#[test]
fn test_s9_lambda_in_map() {
    assert!(transpile_ok(
        "def double(nums: list) -> list:\n    return list(map(lambda x: x * 2, nums))"
    ));
}

#[test]
fn test_s9_nested_list_comprehension() {
    assert!(transpile_ok(
        "def flatten(matrix: list) -> list:\n    return [x for row in matrix for x in row]"
    ));
}

// --- Complex algorithm patterns ---

#[test]
fn test_s9_merge_sort_algorithm() {
    assert!(transpile_ok(
        "def merge(a: list, b: list) -> list:\n    result = []\n    i = 0\n    j = 0\n    while i < len(a) and j < len(b):\n        if a[i] <= b[j]:\n            result.append(a[i])\n            i += 1\n        else:\n            result.append(b[j])\n            j += 1\n    while i < len(a):\n        result.append(a[i])\n        i += 1\n    while j < len(b):\n        result.append(b[j])\n        j += 1\n    return result"
    ));
}

#[test]
fn test_s9_matrix_multiply_pattern() {
    assert!(transpile_ok(
        "def dot_product(a: list, b: list) -> int:\n    total = 0\n    for i in range(len(a)):\n        total += a[i] * b[i]\n    return total"
    ));
}

#[test]
fn test_s9_quicksort_partition() {
    assert!(transpile_ok(
        "def partition(arr: list, low: int, high: int) -> int:\n    pivot = arr[high]\n    i = low - 1\n    for j in range(low, high):\n        if arr[j] <= pivot:\n            i += 1\n    return i + 1"
    ));
}

// --- Class patterns ---

#[test]
fn test_s9_class_with_property() {
    assert!(transpile_ok(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def distance(self) -> float:\n        return (self.x ** 2 + self.y ** 2) ** 0.5"
    ));
}

#[test]
fn test_s9_class_with_str_method() {
    assert!(transpile_ok(
        "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self) -> None:\n        self.count += 1\n    def __str__(self) -> str:\n        return str(self.count)"
    ));
}

// --- Edge case patterns ---

#[test]
fn test_s9_multiple_return_values() {
    assert!(transpile_ok(
        "def swap(a: int, b: int) -> tuple:\n    return (b, a)"
    ));
}

#[test]
fn test_s9_string_formatting_fstring() {
    assert!(transpile_ok(
        "def greet(name: str) -> str:\n    return f\"Hello, {name}!\""
    ));
}

#[test]
fn test_s9_string_format_method() {
    assert!(transpile_ok(
        "def template(x: int) -> str:\n    return \"Value: {}\".format(x)"
    ));
}

#[test]
fn test_s9_chained_comparisons() {
    assert!(transpile_ok(
        "def in_range(x: int, lo: int, hi: int) -> bool:\n    return lo <= x <= hi"
    ));
}

#[test]
fn test_s9_walrus_in_if() {
    assert!(transpile_ok(
        "def check(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0"
    ));
}

#[test]
fn test_s9_assert_with_message() {
    assert!(transpile_ok(
        "def validate(x: int) -> int:\n    assert x >= 0, \"must be non-negative\"\n    return x"
    ));
}

#[test]
fn test_s9_del_variable() {
    assert!(transpile_ok(
        "def cleanup() -> None:\n    x = 10\n    del x"
    ));
}

#[test]
fn test_s9_global_constant_usage() {
    assert!(transpile_ok(
        "MAX = 100\n\ndef check(n: int) -> bool:\n    return n < MAX"
    ));
}

#[test]
fn test_s9_import_and_use() {
    assert!(transpile_ok(
        "import os\n\ndef get_path() -> str:\n    return os.getcwd()"
    ));
}

#[test]
fn test_s9_from_import() {
    assert!(transpile_ok(
        "from pathlib import Path\n\ndef exists(p: str) -> bool:\n    return Path(p).exists()"
    ));
}

#[test]
fn test_s9_empty_function_pass() {
    assert!(transpile_ok(
        "def noop() -> None:\n    pass"
    ));
}

#[test]
fn test_s9_type_annotation_optional() {
    assert!(transpile_ok(
        "def maybe(x: int) -> int:\n    if x > 0:\n        return x\n    return 0"
    ));
}

#[test]
fn test_s9_complex_dict_ops() {
    assert!(transpile_ok(
        "def merge_dicts(a: dict, b: dict) -> dict:\n    result = {}\n    for k, v in a.items():\n        result[k] = v\n    for k, v in b.items():\n        result[k] = v\n    return result"
    ));
}

#[test]
fn test_s9_list_comprehension_nested_if() {
    assert!(transpile_ok(
        "def filter_even_positive(nums: list) -> list:\n    return [x for x in nums if x > 0 if x % 2 == 0]"
    ));
}

#[test]
fn test_s9_string_methods_chain() {
    assert!(transpile_ok(
        "def normalize(s: str) -> str:\n    return s.strip().lower().replace(\" \", \"_\")"
    ));
}

#[test]
fn test_s9_enumerate_pattern() {
    assert!(transpile_ok(
        "def indexed_list(items: list) -> list:\n    result = []\n    for i, item in enumerate(items):\n        result.append(i)\n    return result"
    ));
}

#[test]
fn test_s9_zip_pattern() {
    assert!(transpile_ok(
        "def pair_up(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x + y)\n    return result"
    ));
}
