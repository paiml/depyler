//! Coverage boost tests for expression codegen functions
//!
//! Targets uncovered branches in:
//! - binary_ops.rs: convert_binary, convert_mul_op, convert_add_op
//! - call_generic.rs: convert_generic_call
//! - call_dispatch.rs: try_convert_stdlib_type_call
//! - stdlib_numpy.rs: try_convert_numpy_call
//! - stdlib_datetime.rs: try_convert_datetime_method
//! - stdlib_misc.rs: try_convert_decimal_method
//! - convert_unary_and_call.rs: convert_call

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// =============================================================================
// Section 1: binary_ops - convert_binary (uncovered branches)
// =============================================================================

#[test]
fn test_binary_float_gt_int() {
    let code = transpile("def check(balance: float) -> bool:\n    return balance > 0");
    assert!(code.contains(">"), "float > int comparison: {}", code);
}

#[test]
fn test_binary_chained_arithmetic() {
    let code = transpile("def add3(x: int, y: int, z: int) -> int:\n    return x + y + z");
    assert!(code.contains("+"), "chained add: {}", code);
}

#[test]
fn test_binary_float_comparison_le() {
    let code = transpile("def check(x: float) -> bool:\n    return x <= 100.0");
    assert!(code.contains("<="), "float <= comparison: {}", code);
}

#[test]
fn test_binary_string_equality() {
    let code = transpile("def check(s: str) -> bool:\n    return s == \"hello\"");
    assert!(code.contains("=="), "string equality: {}", code);
}

#[test]
fn test_binary_and_or_operators() {
    let code = transpile("def check(a: bool, b: bool) -> bool:\n    return a and b or not a");
    assert!(code.contains("&&") || code.contains("||"), "logical ops: {}", code);
}

#[test]
fn test_binary_modulo_op() {
    let code = transpile("def is_even(n: int) -> bool:\n    return n % 2 == 0");
    assert!(code.contains("%"), "modulo: {}", code);
}

#[test]
fn test_binary_floor_division() {
    let code = transpile("def half(n: int) -> int:\n    return n // 2");
    assert!(code.contains("/"), "floor division: {}", code);
}

#[test]
fn test_binary_power_op() {
    let code = transpile("def square(n: int) -> int:\n    return n ** 2");
    assert!(code.contains("pow") || code.contains("powi"), "power: {}", code);
}

#[test]
fn test_binary_in_operator_list() {
    let code = transpile("def has_item(items: list, x: int) -> bool:\n    return x in items");
    assert!(code.contains("contains") || code.contains("iter"), "in operator: {}", code);
}

#[test]
fn test_binary_not_in_operator() {
    let code = transpile("def not_found(items: list, x: int) -> bool:\n    return x not in items");
    assert!(code.contains("contains") || code.contains("!"), "not in operator: {}", code);
}

#[test]
fn test_binary_bitwise_and() {
    let code = transpile("def mask(x: int, m: int) -> int:\n    return x & m");
    assert!(code.contains("&"), "bitwise and: {}", code);
}

#[test]
fn test_binary_bitwise_or() {
    let code = transpile("def combine(x: int, y: int) -> int:\n    return x | y");
    assert!(code.contains("|"), "bitwise or: {}", code);
}

#[test]
fn test_binary_bitwise_xor() {
    let code = transpile("def toggle(x: int, y: int) -> int:\n    return x ^ y");
    assert!(code.contains("^"), "bitwise xor: {}", code);
}

#[test]
fn test_binary_left_shift() {
    let code = transpile("def shift(x: int, n: int) -> int:\n    return x << n");
    assert!(code.contains("<<"), "left shift: {}", code);
}

#[test]
fn test_binary_right_shift() {
    let code = transpile("def shift(x: int, n: int) -> int:\n    return x >> n");
    assert!(code.contains(">>"), "right shift: {}", code);
}

// =============================================================================
// Section 2: binary_ops - convert_mul_op (0% coverage)
// =============================================================================

#[test]
fn test_mul_string_repeat_literal() {
    let code = transpile("def repeat() -> str:\n    return \"x\" * 5");
    assert!(code.contains("repeat") || code.contains("\"x\""), "string repeat: {}", code);
}

#[test]
fn test_mul_string_repeat_variable() {
    let code = transpile("def repeat(width: int) -> str:\n    return \"=\" * width");
    assert!(!code.is_empty(), "string * var: {}", code);
}

#[test]
fn test_mul_list_init() {
    let code = transpile("def zeros(n: int) -> list:\n    return [0] * n");
    assert!(code.contains("vec!") || code.contains("Vec"), "list init: {}", code);
}

#[test]
fn test_mul_list_init_literal() {
    let code = transpile("def zeros() -> list:\n    return [0] * 10");
    assert!(code.contains("vec!") || code.contains("0"), "list init literal: {}", code);
}

#[test]
fn test_mul_int_multiply() {
    let code = transpile("def double(x: int) -> int:\n    return x * 2");
    assert!(code.contains("*"), "int multiply: {}", code);
}

#[test]
fn test_mul_float_multiply() {
    let code = transpile("def scale(x: float, factor: float) -> float:\n    return x * factor");
    assert!(code.contains("*"), "float multiply: {}", code);
}

// =============================================================================
// Section 3: binary_ops - convert_add_op (0% coverage)
// =============================================================================

#[test]
fn test_add_list_concat() {
    let code =
        transpile("def merge(a: list, b: list) -> list:\n    result = a + b\n    return result");
    assert!(!code.is_empty(), "list concat: {}", code);
}

#[test]
fn test_add_string_concat() {
    let code = transpile("def greet(name: str) -> str:\n    return \"Hello, \" + name");
    assert!(
        code.contains("format!") || code.contains("+") || code.contains("push_str"),
        "string concat: {}",
        code
    );
}

#[test]
fn test_add_int_addition() {
    let code = transpile("def add(a: int, b: int) -> int:\n    return a + b");
    assert!(code.contains("+"), "int add: {}", code);
}

#[test]
fn test_add_float_addition() {
    let code = transpile("def add(a: float, b: float) -> float:\n    return a + b");
    assert!(code.contains("+"), "float add: {}", code);
}

#[test]
fn test_add_string_concat_multiple() {
    let code = transpile("def build(a: str, b: str, c: str) -> str:\n    return a + b + c");
    assert!(!code.is_empty(), "multi string concat: {}", code);
}

// =============================================================================
// Section 4: call_dispatch - try_convert_stdlib_type_call
// =============================================================================

#[test]
fn test_call_datetime_constructor() {
    let code = transpile(
        "from datetime import datetime\ndef make() -> datetime:\n    return datetime(2024, 1, 15)",
    );
    assert!(!code.is_empty(), "datetime constructor: {}", code);
}

#[test]
fn test_call_date_constructor() {
    let code =
        transpile("from datetime import date\ndef make() -> date:\n    return date(2024, 1, 15)");
    assert!(!code.is_empty(), "date constructor: {}", code);
}

#[test]
fn test_call_time_constructor() {
    let code = transpile("from datetime import time\ndef make():\n    return time(14, 30, 45)");
    assert!(!code.is_empty(), "time constructor: {}", code);
}

#[test]
fn test_call_timedelta_constructor() {
    let code =
        transpile("from datetime import timedelta\ndef make():\n    return timedelta(days=1)");
    assert!(!code.is_empty(), "timedelta constructor: {}", code);
}

#[test]
fn test_call_path_constructor() {
    let code = transpile(
        "from pathlib import Path\ndef make() -> str:\n    p = Path(\"/tmp\")\n    return str(p)",
    );
    assert!(code.contains("Path") || code.contains("PathBuf"), "Path constructor: {}", code);
}

// =============================================================================
// Section 5: stdlib_datetime - try_convert_datetime_method
// =============================================================================

#[test]
fn test_datetime_now() {
    let code = transpile("from datetime import datetime\ndef now():\n    return datetime.now()");
    assert!(!code.is_empty(), "datetime.now(): {}", code);
}

#[test]
fn test_datetime_utcnow() {
    let code = transpile("from datetime import datetime\ndef utc():\n    return datetime.utcnow()");
    assert!(!code.is_empty(), "datetime.utcnow(): {}", code);
}

#[test]
fn test_datetime_today() {
    let code =
        transpile("from datetime import datetime\ndef today():\n    return datetime.today()");
    assert!(!code.is_empty(), "datetime.today(): {}", code);
}

#[test]
fn test_datetime_strptime() {
    let code = transpile(
        "from datetime import datetime\ndef parse(s: str):\n    return datetime.strptime(s, \"%Y-%m-%d\")",
    );
    assert!(!code.is_empty(), "strptime: {}", code);
}

#[test]
fn test_datetime_fromisoformat() {
    let code = transpile(
        "from datetime import datetime\ndef parse(s: str):\n    return datetime.fromisoformat(s)",
    );
    assert!(!code.is_empty(), "fromisoformat: {}", code);
}

#[test]
fn test_datetime_fromtimestamp() {
    let code = transpile(
        "from datetime import datetime\ndef from_ts(ts: float):\n    return datetime.fromtimestamp(ts)",
    );
    assert!(!code.is_empty(), "fromtimestamp: {}", code);
}

#[test]
fn test_date_weekday() {
    let code =
        transpile("from datetime import date\ndef day(d: date) -> int:\n    return d.weekday()");
    assert!(!code.is_empty(), "weekday: {}", code);
}

#[test]
fn test_date_isoweekday() {
    let code =
        transpile("from datetime import date\ndef day(d: date) -> int:\n    return d.isoweekday()");
    assert!(!code.is_empty(), "isoweekday: {}", code);
}

// =============================================================================
// Section 6: stdlib_misc - try_convert_decimal_method (0% coverage)
// =============================================================================

#[test]
fn test_decimal_sqrt() {
    let code = transpile("from decimal import Decimal\ndef root(d: Decimal):\n    return d.sqrt()");
    assert!(!code.is_empty(), "decimal sqrt: {}", code);
}

#[test]
fn test_decimal_quantize() {
    let code = transpile(
        "from decimal import Decimal\ndef round_it(d: Decimal):\n    return d.quantize(Decimal(\"0.01\"))",
    );
    assert!(!code.is_empty(), "decimal quantize: {}", code);
}

#[test]
fn test_decimal_is_nan() {
    let code = transpile(
        "from decimal import Decimal\ndef check(d: Decimal) -> bool:\n    return d.is_nan()",
    );
    assert!(!code.is_empty(), "decimal is_nan: {}", code);
}

#[test]
fn test_decimal_is_zero() {
    let code = transpile(
        "from decimal import Decimal\ndef check(d: Decimal) -> bool:\n    return d.is_zero()",
    );
    assert!(!code.is_empty(), "decimal is_zero: {}", code);
}

#[test]
fn test_decimal_is_finite() {
    let code = transpile(
        "from decimal import Decimal\ndef check(d: Decimal) -> bool:\n    return d.is_finite()",
    );
    assert!(!code.is_empty(), "decimal is_finite: {}", code);
}

#[test]
fn test_decimal_compare() {
    let code = transpile(
        "from decimal import Decimal\ndef cmp(a: Decimal, b: Decimal) -> int:\n    return a.compare(b)",
    );
    assert!(!code.is_empty(), "decimal compare: {}", code);
}

// =============================================================================
// Section 7: call_generic - convert_generic_call
// =============================================================================

#[test]
fn test_call_math_isqrt() {
    let code = transpile("from math import isqrt\ndef root(n: int) -> int:\n    return isqrt(n)");
    assert!(!code.is_empty(), "math isqrt: {}", code);
}

#[test]
fn test_call_os_path_join() {
    let code = transpile(
        "import os\ndef join_path(a: str, b: str) -> str:\n    return os.path.join(a, b)",
    );
    assert!(!code.is_empty(), "os.path.join: {}", code);
}

#[test]
fn test_call_os_path_exists() {
    let code = transpile("import os\ndef check(p: str) -> bool:\n    return os.path.exists(p)");
    assert!(!code.is_empty(), "os.path.exists: {}", code);
}

#[test]
fn test_call_json_loads() {
    let code = transpile("import json\ndef parse(s: str) -> dict:\n    return json.loads(s)");
    assert!(!code.is_empty(), "json.loads: {}", code);
}

#[test]
fn test_call_json_dumps() {
    let code = transpile("import json\ndef dump(d: dict) -> str:\n    return json.dumps(d)");
    assert!(!code.is_empty(), "json.dumps: {}", code);
}

// =============================================================================
// Section 8: convert_call - map/filter/isinstance/statistics
// =============================================================================

#[test]
fn test_call_map_int() {
    let code = transpile("def convert(items: list) -> list:\n    return list(map(int, items))");
    assert!(!code.is_empty(), "map(int, ...): {}", code);
}

#[test]
fn test_call_map_str() {
    let code = transpile("def convert(items: list) -> list:\n    return list(map(str, items))");
    assert!(!code.is_empty(), "map(str, ...): {}", code);
}

#[test]
fn test_call_filter_lambda() {
    let code = transpile(
        "def evens(items: list) -> list:\n    return list(filter(lambda x: x % 2 == 0, items))",
    );
    assert!(!code.is_empty(), "filter(lambda, ...): {}", code);
}

#[test]
fn test_call_isinstance() {
    let code = transpile("def check(x: int) -> bool:\n    return isinstance(x, int)");
    assert!(!code.is_empty(), "isinstance: {}", code);
}

#[test]
fn test_call_statistics_median() {
    let code = transpile(
        "from statistics import median\ndef mid(data: list) -> float:\n    return median(data)",
    );
    assert!(!code.is_empty(), "statistics.median: {}", code);
}

#[test]
fn test_call_statistics_stdev() {
    let code = transpile(
        "from statistics import stdev\ndef spread(data: list) -> float:\n    return stdev(data)",
    );
    assert!(!code.is_empty(), "statistics.stdev: {}", code);
}

#[test]
fn test_call_bisect_left() {
    let code = transpile(
        "import bisect\ndef find(items: list, x: int) -> int:\n    return bisect.bisect_left(items, x)",
    );
    assert!(!code.is_empty(), "bisect_left: {}", code);
}

#[test]
fn test_call_heapq_heappush() {
    let code =
        transpile("import heapq\ndef push(heap: list, val: int):\n    heapq.heappush(heap, val)");
    assert!(!code.is_empty(), "heappush: {}", code);
}

#[test]
fn test_call_heapq_heappop() {
    let code =
        transpile("import heapq\ndef pop(heap: list) -> int:\n    return heapq.heappop(heap)");
    assert!(!code.is_empty(), "heappop: {}", code);
}

// =============================================================================
// Section 9: stdlib_numpy - try_convert_numpy_call (7% coverage)
// =============================================================================

#[test]
fn test_numpy_sqrt() {
    let code = transpile("import numpy as np\ndef root(x: float) -> float:\n    return np.sqrt(x)");
    assert!(code.contains("sqrt"), "np.sqrt: {}", code);
}

#[test]
fn test_numpy_array() {
    let code = transpile("import numpy as np\ndef make():\n    return np.array([1.0, 2.0, 3.0])");
    assert!(!code.is_empty(), "np.array: {}", code);
}

#[test]
fn test_numpy_zeros() {
    let code = transpile("import numpy as np\ndef make(n: int):\n    return np.zeros(n)");
    assert!(!code.is_empty(), "np.zeros: {}", code);
}

#[test]
fn test_numpy_ones() {
    let code = transpile("import numpy as np\ndef make(n: int):\n    return np.ones(n)");
    assert!(!code.is_empty(), "np.ones: {}", code);
}

#[test]
fn test_numpy_min() {
    let code =
        transpile("import numpy as np\ndef minimum(arr: list) -> float:\n    return np.min(arr)");
    assert!(!code.is_empty(), "np.min: {}", code);
}

#[test]
fn test_numpy_max() {
    let code =
        transpile("import numpy as np\ndef maximum(arr: list) -> float:\n    return np.max(arr)");
    assert!(!code.is_empty(), "np.max: {}", code);
}

#[test]
fn test_numpy_mean() {
    let code =
        transpile("import numpy as np\ndef average(arr: list) -> float:\n    return np.mean(arr)");
    assert!(!code.is_empty(), "np.mean: {}", code);
}

#[test]
fn test_numpy_std() {
    let code =
        transpile("import numpy as np\ndef spread(arr: list) -> float:\n    return np.std(arr)");
    assert!(!code.is_empty(), "np.std: {}", code);
}

#[test]
fn test_numpy_sum() {
    let code =
        transpile("import numpy as np\ndef total(arr: list) -> float:\n    return np.sum(arr)");
    assert!(!code.is_empty(), "np.sum: {}", code);
}

#[test]
fn test_numpy_abs() {
    let code =
        transpile("import numpy as np\ndef magnitude(x: float) -> float:\n    return np.abs(x)");
    assert!(!code.is_empty(), "np.abs: {}", code);
}

#[test]
fn test_numpy_clip() {
    let code = transpile(
        "import numpy as np\ndef bound(x: float) -> float:\n    return np.clip(x, 0.0, 1.0)",
    );
    assert!(!code.is_empty(), "np.clip: {}", code);
}

#[test]
fn test_numpy_argmax() {
    let code =
        transpile("import numpy as np\ndef peak(arr: list) -> int:\n    return np.argmax(arr)");
    assert!(!code.is_empty(), "np.argmax: {}", code);
}

#[test]
fn test_numpy_dot() {
    let code = transpile(
        "import numpy as np\ndef product(a: list, b: list) -> float:\n    return np.dot(a, b)",
    );
    assert!(!code.is_empty(), "np.dot: {}", code);
}

#[test]
fn test_numpy_linspace() {
    let code = transpile("import numpy as np\ndef span():\n    return np.linspace(0.0, 1.0, 10)");
    assert!(!code.is_empty(), "np.linspace: {}", code);
}
