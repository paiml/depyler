//! Coverage Edge Cases Tests
//!
//! These tests specifically target low-coverage code paths in expr_gen.rs,
//! stmt_gen.rs, func_gen.rs, and direct_rules.rs to boost overall coverage.

use depyler_core::DepylerPipeline;

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .unwrap_or_else(|e| format!("ERROR: {}", e))
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ============================================================================
// STRING BASE DETECTION - is_string_base() coverage
// ============================================================================

#[test]
fn test_string_indexing_typed() {
    assert!(transpile_ok("def f(text: str): return text[0]"));
}

#[test]
fn test_string_method_chain_indexing() {
    assert!(transpile_ok("def f(s): return s.upper().strip()[0]"));
}

#[test]
fn test_dict_string_value_indexing() {
    assert!(transpile_ok("def f(d): return d[\"key\"][0]"));
}

#[test]
fn test_optional_string_indexing() {
    assert!(transpile_ok(
        "def f(maybe: str = None):\n    if maybe:\n        return maybe[0]\n    return ''"
    ));
}

#[test]
fn test_string_attribute_indexing() {
    assert!(transpile_ok(
        "class C:\n    name: str\ndef f(c):\n    return c.name[0]"
    ));
}

// ============================================================================
// NUMERIC INDEX DETECTION - is_numeric_index() coverage
// ============================================================================

#[test]
fn test_arithmetic_index_addition() {
    assert!(transpile_ok("def f(arr, i): return arr[i + 1]"));
}

#[test]
fn test_arithmetic_index_subtraction() {
    assert!(transpile_ok("def f(arr, i): return arr[i - 1]"));
}

#[test]
fn test_arithmetic_index_multiplication() {
    assert!(transpile_ok("def f(arr, i): return arr[i * 2]"));
}

#[test]
fn test_idx_prefix_variable() {
    assert!(transpile_ok(
        "def f(arr):\n    idx_start = 0\n    return arr[idx_start]"
    ));
}

#[test]
fn test_idx_suffix_variable() {
    assert!(transpile_ok(
        "def f(arr):\n    loop_idx = 0\n    return arr[loop_idx]"
    ));
}

#[test]
fn test_index_suffix_variable() {
    assert!(transpile_ok(
        "def f(arr):\n    col_index = 1\n    return arr[col_index]"
    ));
}

#[test]
fn test_offset_variable() {
    assert!(transpile_ok(
        "def f(arr):\n    offset = 5\n    return arr[offset]"
    ));
}

#[test]
fn test_pos_variable() {
    assert!(transpile_ok(
        "def f(arr):\n    pos = 3\n    return arr[pos]"
    ));
}

// ============================================================================
// STRING VARIABLE DETECTION - is_string_variable() coverage
// ============================================================================

#[test]
fn test_string_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    error_string = 'failed'\n    return error_string[0]"
    ));
}

#[test]
fn test_word_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    search_word = 'find'\n    return search_word[0]"
    ));
}

#[test]
fn test_text_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    log_text = 'message'\n    return log_text[0]"
    ));
}

#[test]
fn test_key_variable_name() {
    assert!(transpile_ok(
        "def f():\n    key = 'some_key'\n    return key[0]"
    ));
}

#[test]
fn test_msg_variable_name() {
    assert!(transpile_ok(
        "def f():\n    msg = 'hello'\n    return msg[0]"
    ));
}

#[test]
fn test_name_variable_name() {
    assert!(transpile_ok(
        "def f():\n    name = 'test'\n    return name[0]"
    ));
}

// ============================================================================
// TUPLE BASE DETECTION - is_tuple_base() coverage
// ============================================================================

#[test]
fn test_pair_variable() {
    assert!(transpile_ok(
        "def f():\n    pair = (10, 20)\n    return pair[0]"
    ));
}

#[test]
fn test_tuple_variable() {
    assert!(transpile_ok("def f():\n    t = (1, 2, 3)\n    return t[1]"));
}

#[test]
fn test_entry_variable() {
    assert!(transpile_ok(
        "def f():\n    entry = ('key', 'value')\n    return entry[0]"
    ));
}

#[test]
fn test_item_variable() {
    assert!(transpile_ok(
        "def f():\n    item = (1, 'a')\n    return item[1]"
    ));
}

#[test]
fn test_enumerate_tuple_indexing() {
    assert!(transpile_ok(
        "def f(items):\n    for t in enumerate(items):\n        print(t[0])"
    ));
}

#[test]
fn test_dict_items_tuple() {
    assert!(transpile_ok(
        "def f(d):\n    for kv in d.items():\n        print(kv[0])"
    ));
}

// ============================================================================
// PATH EXPRESSION DETECTION - is_path_expr() coverage
// ============================================================================

#[test]
fn test_path_prefix_variable() {
    assert!(transpile_ok(
        "def f():\n    path_to_file = 'home'\n    return path_to_file + '/file'"
    ));
}

#[test]
fn test_dir_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    script_dir = '/home/user'\n    return script_dir"
    ));
}

#[test]
fn test_directory_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    output_directory = '/tmp'\n    return output_directory"
    ));
}

#[test]
fn test_folder_suffix_variable() {
    assert!(transpile_ok(
        "def f():\n    data_folder = '/data'\n    return data_folder"
    ));
}

// ============================================================================
// BORROW PATH WITH OPTION CHECK - borrow_path_with_option_check() coverage
// ============================================================================

#[test]
fn test_optional_output_file() {
    assert!(transpile_ok("def f(output_file = None):\n    if output_file:\n        return output_file\n    return ''"));
}

#[test]
fn test_optional_out_file() {
    assert!(transpile_ok(
        "def f(out_file = None):\n    if out_file:\n        return out_file\n    return ''"
    ));
}

#[test]
fn test_optional_file_path() {
    assert!(transpile_ok(
        "def f(file_path = None):\n    if file_path:\n        return file_path\n    return ''"
    ));
}

#[test]
fn test_optional_output_path() {
    assert!(transpile_ok("def f(output_path = None):\n    if output_path:\n        return output_path\n    return ''"));
}

// ============================================================================
// STMT_GEN COVERAGE - Statement generation edge cases
// ============================================================================

#[test]
fn test_for_else_break() {
    assert!(transpile_ok("def f():\n    for i in range(10):\n        if i > 5:\n            break\n    else:\n        print('not found')"));
}

#[test]
fn test_while_else_break() {
    assert!(transpile_ok("def f():\n    x = 0\n    while x < 10:\n        if x == 5:\n            break\n        x += 1\n    else:\n        print('completed')"));
}

#[test]
fn test_try_else() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except:\n        x = 0\n    else:\n        x = 2\n    return x"));
}

#[test]
fn test_nested_try_except() {
    assert!(transpile_ok("def f():\n    try:\n        try:\n            x = 1\n        except:\n            x = 0\n    except:\n        x = -1"));
}

#[test]
fn test_multiple_except_handlers() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except ValueError:\n        x = 0\n    except KeyError:\n        x = -1\n    except:\n        x = -2"));
}

#[test]
fn test_except_tuple_types() {
    assert!(transpile_ok("def f():\n    try:\n        x = 1\n    except (ValueError, KeyError, TypeError):\n        x = 0"));
}

#[test]
fn test_with_nested() {
    assert!(transpile_ok("def f():\n    with open('a') as a:\n        with open('b') as b:\n            return a.read() + b.read()"));
}

#[test]
fn test_with_multiple_items() {
    assert!(transpile_ok(
        "def f():\n    with open('a') as a, open('b') as b, open('c') as c:\n        pass"
    ));
}

#[test]
fn test_match_simple() {
    assert!(transpile_ok("def f(x):\n    match x:\n        case 1:\n            return 'one'\n        case 2:\n            return 'two'\n        case _:\n            return 'other'"));
}

#[test]
fn test_match_tuple_pattern() {
    assert!(transpile_ok("def f(point):\n    match point:\n        case (0, 0):\n            return 'origin'\n        case (x, 0):\n            return 'x-axis'\n        case (0, y):\n            return 'y-axis'\n        case (x, y):\n            return 'general'"));
}

#[test]
fn test_match_or_pattern() {
    assert!(transpile_ok("def f(x):\n    match x:\n        case 1 | 2 | 3:\n            return 'small'\n        case _:\n            return 'large'"));
}

// ============================================================================
// FUNC_GEN COVERAGE - Function generation edge cases
// ============================================================================

#[test]
fn test_function_with_docstring() {
    assert!(transpile_ok(
        "def f():\n    '''This is a docstring.'''\n    return 1"
    ));
}

#[test]
fn test_function_with_multiline_docstring() {
    assert!(transpile_ok(
        "def f():\n    '''\n    Multi-line\n    docstring\n    '''\n    return 1"
    ));
}

#[test]
fn test_async_function() {
    assert!(transpile_ok("async def f():\n    return 1"));
}

#[test]
fn test_async_with() {
    assert!(transpile_ok(
        "async def f():\n    async with open('file') as f:\n        return await f.read()"
    ));
}

#[test]
fn test_async_for() {
    assert!(transpile_ok(
        "async def f(items):\n    async for item in items:\n        print(item)"
    ));
}

#[test]
fn test_generator_function() {
    assert!(transpile_ok(
        "def f():\n    yield 1\n    yield 2\n    yield 3"
    ));
}

#[test]
fn test_generator_with_return() {
    assert!(transpile_ok(
        "def f():\n    yield 1\n    yield 2\n    return 'done'"
    ));
}

#[test]
fn test_generator_yield_from() {
    assert!(transpile_ok("def f(items):\n    yield from items"));
}

#[test]
fn test_function_multiple_decorators() {
    assert!(transpile_ok(
        "@staticmethod\n@cache\ndef f():\n    return 1"
    ));
}

#[test]
fn test_function_decorator_with_args() {
    assert!(transpile_ok("@decorator(arg=1)\ndef f():\n    return 1"));
}

#[test]
fn test_nested_function() {
    assert!(transpile_ok(
        "def outer():\n    def inner():\n        return 1\n    return inner()"
    ));
}

#[test]
fn test_closure() {
    assert!(transpile_ok(
        "def outer(x):\n    def inner():\n        return x + 1\n    return inner"
    ));
}

// ============================================================================
// DIRECT_RULES COVERAGE - Class/struct conversion edge cases
// ============================================================================

#[test]
fn test_class_with_class_var() {
    assert!(transpile_ok(
        "class Counter:\n    count = 0\n    def increment(self):\n        Counter.count += 1"
    ));
}

#[test]
fn test_class_with_property() {
    assert!(transpile_ok("class C:\n    def __init__(self):\n        self._x = 0\n    @property\n    def x(self):\n        return self._x\n    @x.setter\n    def x(self, value):\n        self._x = value"));
}

#[test]
fn test_class_with_slots() {
    assert!(transpile_ok("class C:\n    __slots__ = ['x', 'y']\n    def __init__(self, x, y):\n        self.x = x\n        self.y = y"));
}

#[test]
fn test_class_multiple_inheritance() {
    assert!(transpile_ok(
        "class A:\n    pass\nclass B:\n    pass\nclass C(A, B):\n    pass"
    ));
}

#[test]
fn test_class_with_all_dunder() {
    assert!(transpile_ok("class C:\n    def __init__(self, x):\n        self.x = x\n    def __str__(self):\n        return str(self.x)\n    def __repr__(self):\n        return f'C({self.x})'\n    def __eq__(self, other):\n        return self.x == other.x\n    def __hash__(self):\n        return hash(self.x)\n    def __len__(self):\n        return 1"));
}

#[test]
fn test_class_arithmetic_dunder() {
    assert!(transpile_ok("class Vec:\n    def __init__(self, x):\n        self.x = x\n    def __add__(self, other):\n        return Vec(self.x + other.x)\n    def __sub__(self, other):\n        return Vec(self.x - other.x)\n    def __mul__(self, other):\n        return Vec(self.x * other.x)"));
}

#[test]
fn test_class_comparison_dunder() {
    assert!(transpile_ok("class C:\n    def __init__(self, x):\n        self.x = x\n    def __lt__(self, other):\n        return self.x < other.x\n    def __le__(self, other):\n        return self.x <= other.x\n    def __gt__(self, other):\n        return self.x > other.x\n    def __ge__(self, other):\n        return self.x >= other.x"));
}

#[test]
fn test_class_container_dunder() {
    assert!(transpile_ok("class Container:\n    def __init__(self):\n        self.items = []\n    def __getitem__(self, key):\n        return self.items[key]\n    def __setitem__(self, key, value):\n        self.items[key] = value\n    def __delitem__(self, key):\n        del self.items[key]\n    def __contains__(self, item):\n        return item in self.items\n    def __iter__(self):\n        return iter(self.items)"));
}

#[test]
fn test_dataclass_with_defaults() {
    assert!(transpile_ok("from dataclasses import dataclass\n@dataclass\nclass Point:\n    x: int = 0\n    y: int = 0\n    name: str = 'origin'"));
}

#[test]
fn test_dataclass_with_field() {
    assert!(transpile_ok("from dataclasses import dataclass, field\n@dataclass\nclass Config:\n    items: list = field(default_factory=list)"));
}

// ============================================================================
// COMPLEX EXPRESSION EDGE CASES
// ============================================================================

#[test]
fn test_deeply_nested_calls() {
    assert!(transpile_ok("def f(x): return str(int(float(str(x))))"));
}

#[test]
fn test_deeply_nested_subscripts() {
    assert!(transpile_ok("def f(m): return m[0][1][2][0]"));
}

#[test]
fn test_complex_slice_chain() {
    assert!(transpile_ok("def f(lst): return lst[1:10:2][::2][::-1]"));
}

#[test]
fn test_attribute_subscript_mixed() {
    assert!(transpile_ok("def f(obj): return obj.items[0].name[1]"));
}

#[test]
fn test_call_with_starred_args() {
    assert!(transpile_ok("def f(*args): return sum(*args)"));
}

#[test]
fn test_call_with_double_starred_kwargs() {
    assert!(transpile_ok("def f(**kwargs): return dict(**kwargs)"));
}

#[test]
fn test_call_with_mixed_args() {
    assert!(transpile_ok(
        "def f(a, *args, b=1, **kwargs): return a + sum(args) + b"
    ));
}

#[test]
fn test_complex_walrus() {
    assert!(transpile_ok("def f(data):\n    if (result := process(data)) and (n := len(result)) > 0:\n        return result[:n//2]"));
}

#[test]
fn test_nested_comprehensions() {
    assert!(transpile_ok(
        "def f(): return {k: [x*2 for x in v if x > 0] for k, v in data.items() if v}"
    ));
}

#[test]
fn test_comprehension_with_walrus() {
    assert!(transpile_ok(
        "def f(items): return [y for x in items if (y := x * 2) > 10]"
    ));
}

// ============================================================================
// ERROR PATH COVERAGE
// ============================================================================

#[test]
fn test_empty_function() {
    assert!(transpile_ok("def f(): pass"));
}

#[test]
fn test_only_docstring() {
    assert!(transpile_ok("def f():\n    '''Only a docstring.'''"));
}

#[test]
fn test_recursive_function() {
    assert!(transpile_ok(
        "def fib(n):\n    if n <= 1:\n        return n\n    return fib(n-1) + fib(n-2)"
    ));
}

#[test]
fn test_mutually_recursive() {
    assert!(transpile_ok("def even(n):\n    if n == 0:\n        return True\n    return odd(n - 1)\ndef odd(n):\n    if n == 0:\n        return False\n    return even(n - 1)"));
}

#[test]
fn test_very_long_parameter_list() {
    assert!(transpile_ok(
        "def f(a, b, c, d, e, f, g, h, i, j): return a+b+c+d+e+f+g+h+i+j"
    ));
}

#[test]
fn test_very_long_expression() {
    assert!(transpile_ok(
        "def f(x): return x + x + x + x + x + x + x + x + x + x + x + x + x + x + x"
    ));
}

#[test]
fn test_deeply_nested_if() {
    assert!(transpile_ok("def f(a, b, c, d):\n    if a:\n        if b:\n            if c:\n                if d:\n                    return 1\n    return 0"));
}

#[test]
fn test_deeply_nested_loops() {
    assert!(transpile_ok("def f():\n    for a in range(2):\n        for b in range(2):\n            for c in range(2):\n                for d in range(2):\n                    print(a, b, c, d)"));
}
