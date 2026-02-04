//! Coverage tests for expr_gen.rs and expr_gen_instance_methods.rs
//!
//! DEPYLER-99MODE-001: Targets uncovered paths in expression codegen
//! Focus: stdlib modules, less-common methods, type inference edge cases,
//! error paths, and complex expression combinations.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

// ============================================================================
// String methods: less-common
// ============================================================================

#[test]
fn test_str_lstrip() {
    let code = r#"
def f(s: str) -> str:
    return s.lstrip()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rstrip() {
    let code = r#"
def f(s: str) -> str:
    return s.rstrip()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_title() {
    let code = r#"
def f(s: str) -> str:
    return s.title()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_capitalize() {
    let code = r#"
def f(s: str) -> str:
    return s.capitalize()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_center() {
    let code = r#"
def f(s: str) -> str:
    return s.center(20)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_ljust() {
    let code = r#"
def f(s: str) -> str:
    return s.ljust(20)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rjust() {
    let code = r#"
def f(s: str) -> str:
    return s.rjust(20)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_zfill() {
    let code = r#"
def f(s: str) -> str:
    return s.zfill(10)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_swapcase() {
    let code = r#"
def f(s: str) -> str:
    return s.swapcase()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_islower() {
    let code = r#"
def f(s: str) -> bool:
    return s.islower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isupper() {
    let code = r#"
def f(s: str) -> bool:
    return s.isupper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isspace() {
    let code = r#"
def f(s: str) -> bool:
    return s.isspace()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isalnum() {
    let code = r#"
def f(s: str) -> bool:
    return s.isalnum()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rfind() {
    let code = r#"
def f(s: str) -> int:
    return s.rfind("x")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rindex() {
    let code = r#"
def f(s: str) -> int:
    return s.rindex("x")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_encode() {
    let code = r#"
def f(s: str) -> bytes:
    return s.encode()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split_maxsplit() {
    let code = r#"
def f(s: str) -> list:
    return s.split(",", 2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rsplit() {
    let code = r#"
def f(s: str) -> list:
    return s.rsplit(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_splitlines() {
    let code = r#"
def f(s: str) -> list:
    return s.splitlines()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_expandtabs() {
    let code = r#"
def f(s: str) -> str:
    return s.expandtabs(4)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_format() {
    let code = r#"
def f(name: str) -> str:
    return "Hello {}".format(name)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_maketrans() {
    let code = r#"
def f() -> dict:
    return str.maketrans("abc", "xyz")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// List methods: less-common
// ============================================================================

#[test]
fn test_list_count() {
    let code = r#"
def f(items: list) -> int:
    return items.count(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_copy() {
    let code = r#"
def f(items: list) -> list:
    return items.copy()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop_index() {
    let code = r#"
def f(items: list) -> int:
    return items.pop(0)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Dict methods: less-common
// ============================================================================

#[test]
fn test_dict_pop() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.pop(key)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_pop_default() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_setdefault() {
    let code = r#"
def f(d: dict, key: str) -> int:
    return d.setdefault(key, 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_update() {
    let code = r#"
def f(d: dict):
    d.update({"a": 1})
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_clear() {
    let code = r#"
def f(d: dict):
    d.clear()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_copy() {
    let code = r#"
def f(d: dict) -> dict:
    return d.copy()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Set methods
// ============================================================================

#[test]
fn test_set_add() {
    let code = r#"
def f() -> set:
    s = {1, 2, 3}
    s.add(4)
    return s
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_remove() {
    let code = r#"
def f(s: set):
    s.remove(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_discard() {
    let code = r#"
def f(s: set):
    s.discard(1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_union() {
    let code = r#"
def f(a: set, b: set) -> set:
    return a.union(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_intersection() {
    let code = r#"
def f(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_difference() {
    let code = r#"
def f(a: set, b: set) -> set:
    return a.difference(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_symmetric_difference() {
    let code = r#"
def f(a: set, b: set) -> set:
    return a.symmetric_difference(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issubset() {
    let code = r#"
def f(a: set, b: set) -> bool:
    return a.issubset(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issuperset() {
    let code = r#"
def f(a: set, b: set) -> bool:
    return a.issuperset(b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_clear() {
    let code = r#"
def f(s: set):
    s.clear()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_pop() {
    let code = r#"
def f(s: set) -> int:
    return s.pop()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Math module
// ============================================================================

#[test]
fn test_math_sqrt() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.sqrt(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_ceil() {
    let code = r#"
import math
def f(x: float) -> int:
    return math.ceil(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_floor() {
    let code = r#"
import math
def f(x: float) -> int:
    return math.floor(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.log(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log2() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.log2(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_log10() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.log10(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_sin() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.sin(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_cos() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.cos(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_tan() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.tan(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pow() {
    let code = r#"
import math
def f(x: float, y: float) -> float:
    return math.pow(x, y)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_fabs() {
    let code = r#"
import math
def f(x: float) -> float:
    return math.fabs(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_pi() {
    let code = r#"
import math
def f() -> float:
    return math.pi
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_e() {
    let code = r#"
import math
def f() -> float:
    return math.e
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_math_inf() {
    let code = r#"
import math
def f() -> float:
    return math.inf
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// os/sys module
// ============================================================================

#[test]
fn test_os_path_join() {
    let code = r#"
import os
def f(a: str, b: str) -> str:
    return os.path.join(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_os_path_exists() {
    let code = r#"
import os
def f(p: str) -> bool:
    return os.path.exists(p)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_argv() {
    let code = r#"
import sys
def f() -> list:
    return sys.argv
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_exit() {
    let code = r#"
import sys
def f(code: int):
    sys.exit(code)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// json module
// ============================================================================

#[test]
fn test_json_dumps() {
    let code = r#"
import json
def f(data: dict) -> str:
    return json.dumps(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_json_loads() {
    let code = r#"
import json
def f(s: str) -> dict:
    return json.loads(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// collections module
// ============================================================================

#[test]
fn test_collections_counter() {
    let code = r#"
from collections import Counter
def f(items: list) -> dict:
    return Counter(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_collections_defaultdict() {
    let code = r#"
from collections import defaultdict
def f() -> dict:
    d = defaultdict(int)
    return d
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Builtin functions: less-common
// ============================================================================

#[test]
fn test_builtin_isinstance() {
    let code = r#"
def f(x: int) -> bool:
    return isinstance(x, int)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_type() {
    let code = r#"
def f(x: int) -> str:
    return type(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_round() {
    let code = r#"
def f(x: float) -> int:
    return round(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_hex() {
    let code = r#"
def f(x: int) -> str:
    return hex(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_oct() {
    let code = r#"
def f(x: int) -> str:
    return oct(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_bin() {
    let code = r#"
def f(x: int) -> str:
    return bin(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_divmod() {
    let code = r#"
def f(a: int, b: int) -> tuple:
    return divmod(a, b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_map() {
    let code = r#"
def f(items: list) -> list:
    return list(map(str, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_filter() {
    let code = r#"
def f(items: list) -> list:
    return list(filter(None, items))
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_hash() {
    let code = r#"
def f(s: str) -> int:
    return hash(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_id() {
    let code = r#"
def f(x: int) -> int:
    return id(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_input() {
    let code = r#"
def f() -> str:
    return input("prompt: ")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_repr() {
    let code = r#"
def f(x: int) -> str:
    return repr(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_bool_conversion() {
    let code = r#"
def f(x: int) -> bool:
    return bool(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_list_constructor() {
    let code = r#"
def f(s: str) -> list:
    return list(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_dict_constructor() {
    let code = r#"
def f() -> dict:
    return dict()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_set_constructor() {
    let code = r#"
def f(items: list) -> set:
    return set(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_tuple_constructor() {
    let code = r#"
def f(items: list) -> tuple:
    return tuple(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_builtin_frozenset() {
    let code = r#"
def f(items: list) -> frozenset:
    return frozenset(items)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex expressions
// ============================================================================

#[test]
fn test_chained_comparisons() {
    let code = r#"
def f(x: int) -> bool:
    return 0 < x < 100
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_ternary() {
    let code = r#"
def f(x: int) -> str:
    return "positive" if x > 0 else ("negative" if x < 0 else "zero")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_list_comprehension() {
    let code = r#"
def f(items: list) -> list:
    return [x * 2 for x in items if x > 0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_comprehension_with_filter() {
    let code = r#"
def f(items: list) -> dict:
    return {str(x): x for x in items if x > 0}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_comprehension_with_filter() {
    let code = r#"
def f(items: list) -> set:
    return {x * 2 for x in items if x > 0}
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_nested_function_calls() {
    let code = r#"
def f(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_complex_binary_expression() {
    let code = r#"
def f(a: int, b: int, c: int) -> int:
    return (a + b) * c - (a % b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_boolean_expression_chain() {
    let code = r#"
def f(a: bool, b: bool, c: bool) -> bool:
    return a and b or not c
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_mixed_type_comparison() {
    let code = r#"
def f(x: int) -> bool:
    return x >= 0 and x <= 100
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Lambda expressions
// ============================================================================

#[test]
fn test_lambda_in_sorted_key() {
    let code = r#"
def f(items: list) -> list:
    return sorted(items, key=lambda x: -x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_lambda_multiarg() {
    let code = r#"
def f() -> int:
    add = lambda a, b: a + b
    return add(1, 2)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// F-string edge cases
// ============================================================================

#[test]
fn test_fstring_int_interpolation() {
    let code = r#"
def f(x: int) -> str:
    return f"value: {x}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_expression_interpolation() {
    let code = r#"
def f(x: int) -> str:
    return f"double: {x * 2}"
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_fstring_method_call() {
    let code = r#"
def f(s: str) -> str:
    return f"upper: {s.upper()}"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type conversion edge cases
// ============================================================================

#[test]
fn test_int_from_float() {
    let code = r#"
def f(x: float) -> int:
    return int(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_float_from_int() {
    let code = r#"
def f(x: int) -> float:
    return float(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_from_int() {
    let code = r#"
def f(x: int) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_from_float() {
    let code = r#"
def f(x: float) -> str:
    return str(x)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Index/slice edge cases
// ============================================================================

#[test]
fn test_negative_index() {
    let code = r#"
def f(items: list) -> int:
    return items[-2]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_index() {
    let code = r#"
def f(s: str) -> str:
    return s[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_with_step() {
    let code = r#"
def f(items: list) -> list:
    return items[::2]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_slice_negative_step() {
    let code = r#"
def f(items: list) -> list:
    return items[::-1]
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Await/async
// ============================================================================

#[test]
fn test_async_function() {
    let code = r#"
async def f(x: int) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Yield
// ============================================================================

#[test]
fn test_yield_expression() {
    let code = r#"
def f(n: int):
    for i in range(n):
        yield i
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_yield_no_value() {
    let code = r#"
def f():
    yield
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Named expression (walrus)
// ============================================================================

#[test]
fn test_walrus_in_while() {
    let code = r#"
def f(items: list) -> int:
    count = 0
    while (n := len(items)) > 0:
        count += n
        items.pop()
    return count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Multiple return values
// ============================================================================

#[test]
fn test_return_multiple_values() {
    let code = r#"
def f(x: int) -> tuple:
    return x, x + 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Global/nonlocal (should handle gracefully)
// ============================================================================

#[test]
fn test_global_declaration() {
    let code = r#"
counter = 0
def f():
    global counter
    counter += 1
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Default parameter values
// ============================================================================

#[test]
fn test_default_int_param() {
    let code = r#"
def f(x: int = 0) -> int:
    return x + 1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_str_param() {
    let code = r#"
def f(name: str = "world") -> str:
    return "Hello " + name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_none_param() {
    let code = r#"
from typing import Optional
def f(x: Optional[int] = None) -> int:
    if x is None:
        return 0
    return x
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_default_bool_param() {
    let code = r#"
def f(flag: bool = False) -> bool:
    return not flag
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type annotations
// ============================================================================

#[test]
fn test_typed_list() {
    let code = r#"
from typing import List
def f(items: List[int]) -> int:
    return sum(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_dict() {
    let code = r#"
from typing import Dict
def f(d: Dict[str, int]) -> int:
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_optional() {
    let code = r#"
from typing import Optional
def f(x: Optional[int]) -> int:
    if x is not None:
        return x
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_tuple() {
    let code = r#"
from typing import Tuple
def f(t: Tuple[int, str]) -> int:
    return t[0]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_set() {
    let code = r#"
from typing import Set
def f(s: Set[int]) -> int:
    return len(s)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Attribute access patterns
// ============================================================================

#[test]
fn test_self_attribute_in_method() {
    let code = r#"
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1

    def get_count(self) -> int:
        return self.count
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String formatting patterns
// ============================================================================

#[test]
fn test_percent_format() {
    let code = r#"
def f(name: str) -> str:
    return "Hello %s" % name
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex real-world patterns
// ============================================================================

#[test]
fn test_fibonacci() {
    let code = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_binary_search() {
    let code = r#"
def binary_search(items: list, target: int) -> int:
    low = 0
    high = len(items) - 1
    while low <= high:
        mid = (low + high) // 2
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_matrix_multiply() {
    let code = r#"
def dot_product(a: list, b: list) -> int:
    total = 0
    for i in range(len(a)):
        total += a[i] * b[i]
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_word_count() {
    let code = r#"
def word_count(text: str) -> dict:
    counts = {}
    words = text.split()
    for word in words:
        w = word.lower()
        if w in counts:
            counts[w] += 1
        else:
            counts[w] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_flatten_list() {
    let code = r#"
def flatten(items: list) -> list:
    result = []
    for item in items:
        if isinstance(item, list):
            result.extend(flatten(item))
        else:
            result.append(item)
    return result
"#;
    assert!(transpile_ok(code));
}
