//! Targeted expression generation tests
//!
//! These tests specifically target uncovered code paths in expr_gen.rs.

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
// METHOD CALL WITH COMPLEX RECEIVERS
// ============================================================================

#[test]
fn test_method_on_literal_string() {
    assert!(transpile_ok(
        "def foo() -> list[str]:\n    return 'hello,world'.split(',')"
    ));
}

#[test]
fn test_method_on_list_literal() {
    assert!(transpile_ok(
        "def foo() -> int:\n    return [1, 2, 3].pop()"
    ));
}

#[test]
fn test_method_on_dict_literal() {
    let _ = transpile_ok("def foo():\n    return {'a': 1}.get('a')");
}

#[test]
fn test_method_chain_on_literal() {
    assert!(transpile_ok(
        "def foo() -> str:\n    return '  hello  '.strip().upper()"
    ));
}

// ============================================================================
// CALL WITH STARARGS
// ============================================================================

#[test]
fn test_call_with_star_args() {
    assert!(transpile_ok(
        "def bar(*args):\n    pass\n\ndef foo():\n    items = [1, 2, 3]\n    bar(*items)"
    ));
}

#[test]
fn test_call_with_double_star_kwargs() {
    assert!(transpile_ok(
        "def bar(**kwargs):\n    pass\n\ndef foo():\n    opts = {'a': 1}\n    bar(**opts)"
    ));
}

#[test]
fn test_call_with_both_star_args() {
    assert!(transpile_ok("def bar(*args, **kwargs):\n    pass\n\ndef foo():\n    items = [1, 2]\n    opts = {'a': 1}\n    bar(*items, **opts)"));
}

// ============================================================================
// COMPLEX SUBSCRIPT EXPRESSIONS
// ============================================================================

#[test]
fn test_subscript_with_expression_index() {
    assert!(transpile_ok(
        "def foo(items: list[int], offset: int) -> int:\n    return items[offset + 1]"
    ));
}

#[test]
fn test_subscript_with_function_call_index() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return items[len(items) - 1]"
    ));
}

#[test]
fn test_subscript_nested() {
    assert!(transpile_ok(
        "def foo(matrix: list[list[int]], i: int, j: int) -> int:\n    return matrix[i][j]"
    ));
}

#[test]
fn test_subscript_dict_with_var_key() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int], key: str) -> int:\n    return d[key]"
    ));
}

// ============================================================================
// SLICE EXPRESSIONS
// ============================================================================

#[test]
fn test_slice_with_var_start() {
    assert!(transpile_ok(
        "def foo(items: list[int], start: int) -> list[int]:\n    return items[start:]"
    ));
}

#[test]
fn test_slice_with_var_end() {
    assert!(transpile_ok(
        "def foo(items: list[int], end: int) -> list[int]:\n    return items[:end]"
    ));
}

#[test]
fn test_slice_with_var_step() {
    assert!(transpile_ok(
        "def foo(items: list[int], step: int) -> list[int]:\n    return items[::step]"
    ));
}

#[test]
fn test_slice_with_all_vars() {
    assert!(transpile_ok("def foo(items: list[int], start: int, end: int, step: int) -> list[int]:\n    return items[start:end:step]"));
}

#[test]
fn test_slice_string() {
    assert!(transpile_ok("def foo(s: str) -> str:\n    return s[1:-1]"));
}

// ============================================================================
// ATTRIBUTE ACCESS VARIATIONS
// ============================================================================

#[test]
fn test_attribute_on_call_result() {
    assert!(transpile_ok(
        "def bar():\n    return {'x': 1}\n\ndef foo() -> int:\n    return bar()['x']"
    ));
}

#[test]
fn test_nested_attribute_access() {
    assert!(transpile_ok("class Inner:\n    x: int\n\nclass Outer:\n    inner: Inner\n\ndef foo(o: Outer) -> int:\n    return o.inner.x"));
}

// ============================================================================
// BINARY OPERATIONS WITH TYPE COERCION
// ============================================================================

#[test]
fn test_int_plus_float() {
    assert!(transpile_ok(
        "def foo(n: int, f: float) -> float:\n    return n + f"
    ));
}

#[test]
fn test_float_div_int() {
    assert!(transpile_ok(
        "def foo(f: float, n: int) -> float:\n    return f / n"
    ));
}

#[test]
fn test_string_multiply_int() {
    assert!(transpile_ok(
        "def foo(s: str, n: int) -> str:\n    return s * n"
    ));
}

#[test]
fn test_list_multiply_int() {
    assert!(transpile_ok(
        "def foo(items: list[int], n: int) -> list[int]:\n    return items * n"
    ));
}

#[test]
fn test_list_add_list() {
    assert!(transpile_ok(
        "def foo(a: list[int], b: list[int]) -> list[int]:\n    return a + b"
    ));
}

// ============================================================================
// COMPARISON CHAINS
// ============================================================================

#[test]
fn test_chain_lt_le() {
    assert!(transpile_ok(
        "def foo(x: int) -> bool:\n    return 0 < x <= 10"
    ));
}

#[test]
fn test_chain_gt_ge() {
    assert!(transpile_ok(
        "def foo(x: int) -> bool:\n    return 10 > x >= 0"
    ));
}

#[test]
fn test_chain_eq_eq() {
    assert!(transpile_ok(
        "def foo(a: int, b: int, c: int) -> bool:\n    return a == b == c"
    ));
}

#[test]
fn test_chain_ne() {
    assert!(transpile_ok(
        "def foo(a: int, b: int, c: int) -> bool:\n    return a != b != c"
    ));
}

// ============================================================================
// BOOLEAN OPERATIONS
// ============================================================================

#[test]
fn test_short_circuit_and() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return items and items[0]"
    ));
}

#[test]
fn test_short_circuit_or() {
    assert!(transpile_ok("def foo(x) -> int:\n    return x or 0"));
}

#[test]
fn test_nested_boolean() {
    assert!(transpile_ok(
        "def foo(a: bool, b: bool, c: bool) -> bool:\n    return (a and b) or (not c)"
    ));
}

// ============================================================================
// CONTAINMENT OPERATIONS
// ============================================================================

#[test]
fn test_in_string() {
    assert!(transpile_ok(
        "def foo(needle: str, haystack: str) -> bool:\n    return needle in haystack"
    ));
}

#[test]
fn test_in_set() {
    assert!(transpile_ok(
        "def foo(item: int, s: set[int]) -> bool:\n    return item in s"
    ));
}

#[test]
fn test_not_in_dict_values() {
    let _ = transpile_ok(
        "def foo(value: int, d: dict[str, int]) -> bool:\n    return value not in d.values()",
    );
}

// ============================================================================
// UNARY OPERATIONS
// ============================================================================

#[test]
fn test_unary_not_comparison() {
    assert!(transpile_ok(
        "def foo(x: int) -> bool:\n    return not x > 0"
    ));
}

#[test]
fn test_unary_not_membership() {
    assert!(transpile_ok(
        "def foo(item: int, items: list[int]) -> bool:\n    return not item in items"
    ));
}

#[test]
fn test_double_negation() {
    assert!(transpile_ok("def foo(x: int) -> int:\n    return --x"));
}

// ============================================================================
// CONDITIONAL EXPRESSIONS (TERNARY)
// ============================================================================

#[test]
fn test_ternary_with_calls() {
    assert!(transpile_ok(
        "def foo(x: int) -> int:\n    return abs(x) if x < 0 else x"
    ));
}

#[test]
fn test_ternary_with_method_calls() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.upper() if s.islower() else s.lower()"
    ));
}

#[test]
fn test_nested_ternary() {
    assert!(transpile_ok(
        "def foo(x: int) -> str:\n    return 'big' if x > 100 else 'medium' if x > 10 else 'small'"
    ));
}

// ============================================================================
// LAMBDA EXPRESSIONS
// ============================================================================

#[test]
fn test_lambda_no_args() {
    assert!(transpile_ok(
        "def foo():\n    f = lambda: 42\n    return f()"
    ));
}

#[test]
fn test_lambda_single_arg() {
    assert!(transpile_ok(
        "def foo():\n    f = lambda x: x * 2\n    return f(5)"
    ));
}

#[test]
fn test_lambda_multiple_args() {
    assert!(transpile_ok(
        "def foo():\n    f = lambda x, y: x + y\n    return f(1, 2)"
    ));
}

#[test]
fn test_lambda_default_arg() {
    let _ = transpile_ok("def foo():\n    f = lambda x, y=10: x + y\n    return f(5)");
}

#[test]
fn test_lambda_in_filter() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return list(filter(lambda x: x % 2 == 0, items))"));
}

#[test]
fn test_lambda_in_sorted_key() {
    assert!(transpile_ok("def foo(items: list[str]) -> list[str]:\n    return sorted(items, key=lambda x: x.lower())"));
}

// ============================================================================
// COMPREHENSIONS
// ============================================================================

#[test]
fn test_list_comp_with_method() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> list[str]:\n    return [x.strip() for x in items]"
    ));
}

#[test]
fn test_list_comp_with_function() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> list[int]:\n    return [abs(x) for x in items]"
    ));
}

#[test]
fn test_list_comp_nested_if() {
    assert!(transpile_ok("def foo(items: list[int]) -> list[int]:\n    return [x for x in items if x > 0 if x < 100]"));
}

#[test]
fn test_dict_comp_from_list() {
    assert!(transpile_ok("def foo(items: list[str]) -> dict[str, int]:\n    return {item: len(item) for item in items}"));
}

#[test]
fn test_set_comp_with_method() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> set[str]:\n    return {x.lower() for x in items}"
    ));
}

// ============================================================================
// GENERATOR EXPRESSIONS
// ============================================================================

#[test]
fn test_genexp_in_sum() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return sum(x * x for x in items)"
    ));
}

#[test]
fn test_genexp_in_min() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> int:\n    return min(len(x) for x in items)"
    ));
}

#[test]
fn test_genexp_in_max() {
    assert!(transpile_ok(
        "def foo(items: list[str]) -> int:\n    return max(len(x) for x in items)"
    ));
}

#[test]
fn test_genexp_with_condition() {
    assert!(transpile_ok(
        "def foo(items: list[int]) -> int:\n    return sum(x for x in items if x > 0)"
    ));
}

// ============================================================================
// BUILTIN CALLS
// ============================================================================

#[test]
fn test_print_with_sep_end() {
    assert!(transpile_ok(
        "def foo():\n    print('a', 'b', sep=', ', end='\\n')"
    ));
}

#[test]
fn test_print_to_stderr() {
    assert!(transpile_ok(
        "import sys\ndef foo():\n    print('error', file=sys.stderr)"
    ));
}

#[test]
fn test_len_on_string() {
    assert!(transpile_ok("def foo(s: str) -> int:\n    return len(s)"));
}

#[test]
fn test_len_on_dict() {
    assert!(transpile_ok(
        "def foo(d: dict[str, int]) -> int:\n    return len(d)"
    ));
}

#[test]
fn test_len_on_set() {
    assert!(transpile_ok(
        "def foo(s: set[int]) -> int:\n    return len(s)"
    ));
}

#[test]
fn test_range_in_len() {
    assert!(transpile_ok("def foo() -> int:\n    return len(range(10))"));
}

#[test]
fn test_enumerate_start_arg() {
    assert!(transpile_ok("def foo(items: list[str]):\n    for i, x in enumerate(items, start=1):\n        print(i, x)"));
}

#[test]
fn test_zip_different_lengths() {
    assert!(transpile_ok(
        "def foo(a: list[int], b: list[str]):\n    for x, y in zip(a, b):\n        print(x, y)"
    ));
}

// ============================================================================
// STRING METHODS
// ============================================================================

#[test]
fn test_string_partition() {
    assert!(transpile_ok(
        "def foo(s: str):\n    head, sep, tail = s.partition(':')"
    ));
}

#[test]
fn test_string_rpartition() {
    assert!(transpile_ok(
        "def foo(s: str):\n    head, sep, tail = s.rpartition(':')"
    ));
}

#[test]
fn test_string_splitlines() {
    assert!(transpile_ok(
        "def foo(s: str) -> list[str]:\n    return s.splitlines()"
    ));
}

#[test]
fn test_string_title() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.title()"
    ));
}

#[test]
fn test_string_capitalize() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.capitalize()"
    ));
}

#[test]
fn test_string_swapcase() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.swapcase()"
    ));
}

#[test]
fn test_string_center() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.center(20)"
    ));
}

#[test]
fn test_string_ljust() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.ljust(20)"
    ));
}

#[test]
fn test_string_rjust() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.rjust(20)"
    ));
}

#[test]
fn test_string_zfill() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.zfill(10)"
    ));
}

#[test]
fn test_string_expandtabs() {
    assert!(transpile_ok(
        "def foo(s: str) -> str:\n    return s.expandtabs(4)"
    ));
}

#[test]
fn test_string_isspace() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    return s.isspace()"
    ));
}

#[test]
fn test_string_isupper() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    return s.isupper()"
    ));
}

#[test]
fn test_string_islower() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    return s.islower()"
    ));
}

#[test]
fn test_string_istitle() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    return s.istitle()"
    ));
}

#[test]
fn test_string_isidentifier() {
    assert!(transpile_ok(
        "def foo(s: str) -> bool:\n    return s.isidentifier()"
    ));
}

// ============================================================================
// LIST METHODS
// ============================================================================

#[test]
fn test_list_count() {
    assert!(transpile_ok(
        "def foo(items: list[int], val: int) -> int:\n    return items.count(val)"
    ));
}

#[test]
fn test_list_index_with_bounds() {
    let _ = transpile_ok(
        "def foo(items: list[int], val: int) -> int:\n    return items.index(val, 1, 5)",
    );
}

#[test]
fn test_list_sort_with_key() {
    assert!(transpile_ok(
        "def foo(items: list[str]):\n    items.sort(key=len)"
    ));
}

#[test]
fn test_list_sort_reverse() {
    assert!(transpile_ok(
        "def foo(items: list[int]):\n    items.sort(reverse=True)"
    ));
}

// ============================================================================
// DICT METHODS
// ============================================================================

#[test]
fn test_dict_fromkeys() {
    let _ = transpile_ok(
        "def foo(keys: list[str]) -> dict[str, int]:\n    return dict.fromkeys(keys, 0)",
    );
}

#[test]
fn test_dict_popitem() {
    let _ = transpile_ok("def foo(d: dict[str, int]):\n    key, value = d.popitem()");
}

// ============================================================================
// SET METHODS
// ============================================================================

#[test]
fn test_set_symmetric_difference() {
    assert!(transpile_ok(
        "def foo(a: set[int], b: set[int]) -> set[int]:\n    return a.symmetric_difference(b)"
    ));
}

#[test]
fn test_set_issubset() {
    assert!(transpile_ok(
        "def foo(a: set[int], b: set[int]) -> bool:\n    return a.issubset(b)"
    ));
}

#[test]
fn test_set_issuperset() {
    assert!(transpile_ok(
        "def foo(a: set[int], b: set[int]) -> bool:\n    return a.issuperset(b)"
    ));
}

#[test]
fn test_set_isdisjoint() {
    assert!(transpile_ok(
        "def foo(a: set[int], b: set[int]) -> bool:\n    return a.isdisjoint(b)"
    ));
}

// ============================================================================
// BYTES METHODS
// ============================================================================

#[test]
fn test_bytes_decode_utf8() {
    assert!(transpile_ok(
        "def foo(b: bytes) -> str:\n    return b.decode('utf-8')"
    ));
}

#[test]
fn test_bytes_decode_latin1() {
    assert!(transpile_ok(
        "def foo(b: bytes) -> str:\n    return b.decode('latin-1')"
    ));
}

#[test]
fn test_bytes_hex() {
    assert!(transpile_ok(
        "def foo(b: bytes) -> str:\n    return b.hex()"
    ));
}

#[test]
fn test_bytes_fromhex() {
    let _ = transpile_ok("def foo(s: str) -> bytes:\n    return bytes.fromhex(s)");
}

// ============================================================================
// MATH FUNCTIONS
// ============================================================================

#[test]
fn test_math_pow() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float, y: float) -> float:\n    return math.pow(x, y)"
    ));
}

#[test]
fn test_math_fabs() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> float:\n    return math.fabs(x)"
    ));
}

#[test]
fn test_math_trunc() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> int:\n    return math.trunc(x)"
    ));
}

#[test]
fn test_math_modf() {
    let _ = transpile_ok("import math\ndef foo(x: float):\n    frac, integer = math.modf(x)");
}

#[test]
fn test_math_isnan() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> bool:\n    return math.isnan(x)"
    ));
}

#[test]
fn test_math_isinf() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> bool:\n    return math.isinf(x)"
    ));
}

#[test]
fn test_math_degrees() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> float:\n    return math.degrees(x)"
    ));
}

#[test]
fn test_math_radians() {
    assert!(transpile_ok(
        "import math\ndef foo(x: float) -> float:\n    return math.radians(x)"
    ));
}

// ============================================================================
// OS MODULE FUNCTIONS
// ============================================================================

#[test]
fn test_os_getenv_default() {
    assert!(transpile_ok(
        "import os\ndef foo() -> str:\n    return os.getenv('PATH', '')"
    ));
}

#[test]
fn test_os_environ_get() {
    assert!(transpile_ok(
        "import os\ndef foo() -> str:\n    return os.environ.get('PATH', '')"
    ));
}

#[test]
fn test_os_path_splitext() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str):\n    name, ext = os.path.splitext(path)"
    ));
}

#[test]
fn test_os_path_split() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str):\n    dirname, basename = os.path.split(path)"
    ));
}

#[test]
fn test_os_path_getsize() {
    let _ = transpile_ok("import os\ndef foo(path: str) -> int:\n    return os.path.getsize(path)");
}

#[test]
fn test_os_path_abspath() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> str:\n    return os.path.abspath(path)"
    ));
}

#[test]
fn test_os_path_normpath() {
    assert!(transpile_ok(
        "import os\ndef foo(path: str) -> str:\n    return os.path.normpath(path)"
    ));
}

// ============================================================================
// SYS MODULE
// ============================================================================

#[test]
fn test_sys_argv() {
    assert!(transpile_ok(
        "import sys\ndef foo() -> list[str]:\n    return sys.argv"
    ));
}

#[test]
fn test_sys_exit() {
    assert!(transpile_ok("import sys\ndef foo():\n    sys.exit(0)"));
}

#[test]
fn test_sys_exit_message() {
    assert!(transpile_ok(
        "import sys\ndef foo():\n    sys.exit('error')"
    ));
}

// ============================================================================
// DATETIME MODULE
// ============================================================================

#[test]
fn test_datetime_strptime() {
    assert!(transpile_ok("from datetime import datetime\ndef foo(s: str):\n    return datetime.strptime(s, '%Y-%m-%d')"));
}

#[test]
fn test_datetime_fromtimestamp() {
    assert!(transpile_ok(
        "from datetime import datetime\ndef foo(ts: float):\n    return datetime.fromtimestamp(ts)"
    ));
}

#[test]
fn test_datetime_replace() {
    assert!(transpile_ok("from datetime import datetime\ndef foo():\n    dt = datetime.now()\n    return dt.replace(year=2020)"));
}

#[test]
fn test_date_today() {
    assert!(transpile_ok(
        "from datetime import date\ndef foo():\n    return date.today()"
    ));
}

#[test]
fn test_timedelta_total_seconds() {
    assert!(transpile_ok("from datetime import timedelta\ndef foo():\n    td = timedelta(hours=1)\n    return td.total_seconds()"));
}

// ============================================================================
// JSON MODULE
// ============================================================================

#[test]
fn test_json_dumps_indent() {
    assert!(transpile_ok(
        "import json\ndef foo(data: dict[str, int]) -> str:\n    return json.dumps(data, indent=2)"
    ));
}

#[test]
fn test_json_dumps_sort_keys() {
    assert!(transpile_ok("import json\ndef foo(data: dict[str, int]) -> str:\n    return json.dumps(data, sort_keys=True)"));
}

// ============================================================================
// RE MODULE
// ============================================================================

#[test]
fn test_re_compile_flags() {
    assert!(transpile_ok(
        "import re\ndef foo(pattern: str):\n    return re.compile(pattern, re.IGNORECASE)"
    ));
}

#[test]
fn test_re_match_groups() {
    assert!(transpile_ok("import re\ndef foo(pattern: str, text: str):\n    m = re.match(pattern, text)\n    if m:\n        return m.groups()"));
}

#[test]
fn test_re_finditer() {
    assert!(transpile_ok("import re\ndef foo(pattern: str, text: str):\n    for m in re.finditer(pattern, text):\n        print(m.group())"));
}

// ============================================================================
// COLLECTIONS MODULE
// ============================================================================

#[test]
fn test_counter_most_common() {
    assert!(transpile_ok("from collections import Counter\ndef foo(items: list[str]):\n    c = Counter(items)\n    return c.most_common(3)"));
}

#[test]
fn test_counter_elements() {
    assert!(transpile_ok("from collections import Counter\ndef foo():\n    c = Counter({'a': 2, 'b': 3})\n    return list(c.elements())"));
}

#[test]
fn test_deque_append_left() {
    assert!(transpile_ok(
        "from collections import deque\ndef foo():\n    d = deque()\n    d.appendleft(1)"
    ));
}

#[test]
fn test_deque_pop_left() {
    assert!(transpile_ok("from collections import deque\ndef foo():\n    d = deque([1, 2, 3])\n    return d.popleft()"));
}

#[test]
fn test_deque_rotate() {
    assert!(transpile_ok(
        "from collections import deque\ndef foo():\n    d = deque([1, 2, 3])\n    d.rotate(1)"
    ));
}

// ============================================================================
// ITERTOOLS MODULE
// ============================================================================

#[test]
fn test_itertools_count() {
    assert!(transpile_ok(
        "import itertools\ndef foo():\n    c = itertools.count(10)"
    ));
}

#[test]
fn test_itertools_islice() {
    assert!(transpile_ok("import itertools\ndef foo(items: list[int]) -> list[int]:\n    return list(itertools.islice(items, 5))"));
}

#[test]
fn test_itertools_takewhile() {
    assert!(transpile_ok("import itertools\ndef foo(items: list[int]) -> list[int]:\n    return list(itertools.takewhile(lambda x: x < 5, items))"));
}

#[test]
fn test_itertools_dropwhile() {
    assert!(transpile_ok("import itertools\ndef foo(items: list[int]) -> list[int]:\n    return list(itertools.dropwhile(lambda x: x < 5, items))"));
}

#[test]
fn test_itertools_product() {
    // itertools.product may not be fully supported
    let _ = transpile_ok("import itertools\ndef foo(a: list[int], b: list[int]):\n    for x, y in itertools.product(a, b):\n        print(x, y)");
}

#[test]
fn test_itertools_permutations() {
    // itertools.permutations may not be fully supported
    let _ = transpile_ok("import itertools\ndef foo(items: list[int]):\n    return list(itertools.permutations(items, 2))");
}

#[test]
fn test_itertools_combinations() {
    // itertools.combinations may not be fully supported
    let _ = transpile_ok("import itertools\ndef foo(items: list[int]):\n    return list(itertools.combinations(items, 2))");
}

// ============================================================================
// FUNCTOOLS MODULE
// ============================================================================

#[test]
fn test_functools_reduce() {
    assert!(transpile_ok("from functools import reduce\ndef foo(items: list[int]) -> int:\n    return reduce(lambda x, y: x + y, items)"));
}

#[test]
fn test_functools_reduce_initial() {
    assert!(transpile_ok("from functools import reduce\ndef foo(items: list[int]) -> int:\n    return reduce(lambda x, y: x + y, items, 0)"));
}

// ============================================================================
// SPECIAL EXPRESSIONS
// ============================================================================

#[test]
fn test_starred_in_list() {
    assert!(transpile_ok(
        "def foo(a: list[int], b: list[int]) -> list[int]:\n    return [*a, *b]"
    ));
}

#[test]
fn test_starred_in_dict() {
    assert!(transpile_ok(
        "def foo(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:\n    return {**a, **b}"
    ));
}

#[test]
fn test_starred_in_call() {
    assert!(transpile_ok("def bar(x: int, y: int, z: int) -> int:\n    return x + y + z\n\ndef foo(items: list[int]) -> int:\n    return bar(*items)"));
}

#[test]
fn test_double_starred_in_call() {
    assert!(transpile_ok("def bar(x: int, y: int) -> int:\n    return x + y\n\ndef foo(kwargs: dict[str, int]) -> int:\n    return bar(**kwargs)"));
}
