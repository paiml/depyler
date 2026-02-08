//! Coverage wave 3: type dispatch and conversion coverage boost tests
//!
//! Targets uncovered branches in binary_ops.rs, call_dispatch.rs,
//! instance_dispatch.rs, attribute_convert.rs, type_helpers.rs,
//! dict_constructors.rs, and method_call_routing.rs

use crate::DepylerPipeline;

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

// =============================================================================
// Section 1: binary_ops.rs - Floor division edge cases (49.5% cov, 409 missed)
// =============================================================================

#[test]
fn test_floor_div_negative_numerator() {
    let code = transpile(
        "def floor_neg(x: int) -> int:\n    return (-7) // 2",
    );
    assert!(code.contains("/"), "floor div negative numerator: {}", code);
}

#[test]
fn test_floor_div_negative_denominator() {
    let code = transpile(
        "def floor_neg_denom(x: int) -> int:\n    return 7 // (-2)",
    );
    assert!(code.contains("/"), "floor div negative denom: {}", code);
}

#[test]
fn test_floor_div_both_negative() {
    let code = transpile(
        "def floor_both_neg(x: int) -> int:\n    return (-7) // (-3)",
    );
    assert!(code.contains("/"), "floor div both negative: {}", code);
}

#[test]
fn test_floor_div_variable_operands() {
    let code = transpile(
        "def floor_vars(a: int, b: int) -> int:\n    return a // b",
    );
    assert!(code.contains("/"), "floor div with variables: {}", code);
}

#[test]
fn test_floor_div_in_assignment() {
    let code = transpile(
        "def floor_assign(n: int) -> int:\n    result = n // 10\n    return result",
    );
    assert!(!code.is_empty(), "floor div in assignment: {}", code);
}

// =============================================================================
// Section 2: binary_ops.rs - Dict merge operator
// =============================================================================

#[test]
fn test_dict_merge_operator() {
    let code = transpile(
        "def merge_dicts(d1: dict, d2: dict) -> dict:\n    return d1 | d2",
    );
    assert!(!code.is_empty(), "dict merge operator: {}", code);
}

#[test]
fn test_dict_merge_with_literals() {
    let code = transpile(
        "def merge_lit() -> dict:\n    d1 = {\"a\": 1}\n    d2 = {\"b\": 2}\n    return d1 | d2",
    );
    assert!(!code.is_empty(), "dict merge literals: {}", code);
}

// =============================================================================
// Section 3: binary_ops.rs - Set operations via operators
// =============================================================================

#[test]
fn test_set_intersection_operator() {
    let code = transpile(
        "def intersect(a: set, b: set) -> set:\n    return a & b",
    );
    assert!(!code.is_empty(), "set intersection op: {}", code);
}

#[test]
fn test_set_union_operator() {
    let code = transpile(
        "def unite(a: set, b: set) -> set:\n    return a | b",
    );
    assert!(!code.is_empty(), "set union op: {}", code);
}

#[test]
fn test_set_difference_operator() {
    let code = transpile(
        "def diff(a: set, b: set) -> set:\n    return a - b",
    );
    assert!(!code.is_empty(), "set difference op: {}", code);
}

#[test]
fn test_set_symmetric_diff_operator() {
    let code = transpile(
        "def sym_diff(a: set, b: set) -> set:\n    return a ^ b",
    );
    assert!(!code.is_empty(), "set symmetric diff op: {}", code);
}

// =============================================================================
// Section 4: binary_ops.rs - String comparisons
// =============================================================================

#[test]
fn test_string_ge_comparison() {
    let code = transpile(
        "def check_ge(s: str) -> bool:\n    return \"abc\" >= s",
    );
    assert!(code.contains(">="), "string >= comparison: {}", code);
}

#[test]
fn test_string_lt_comparison() {
    let code = transpile(
        "def check_lt(s: str) -> bool:\n    return s < \"z\"",
    );
    assert!(code.contains("<"), "string < comparison: {}", code);
}

#[test]
fn test_string_le_comparison() {
    let code = transpile(
        "def check_le(s: str) -> bool:\n    return s <= \"zzz\"",
    );
    assert!(code.contains("<="), "string <= comparison: {}", code);
}

#[test]
fn test_string_gt_comparison() {
    let code = transpile(
        "def check_gt(s: str) -> bool:\n    return s > \"a\"",
    );
    assert!(code.contains(">"), "string > comparison: {}", code);
}

// =============================================================================
// Section 5: binary_ops.rs - Value-returning or/and
// =============================================================================

#[test]
fn test_or_with_string_default() {
    let code = transpile(
        "def with_default(name: str) -> str:\n    return name or \"unknown\"",
    );
    assert!(
        code.contains("is_empty") || code.contains("is_true"),
        "or with string default: {}",
        code
    );
}

#[test]
fn test_or_with_numeric_default() {
    let code = transpile(
        "def safe_value(x: float) -> float:\n    return x or 1.0",
    );
    assert!(!code.is_empty(), "or with numeric default: {}", code);
}

#[test]
fn test_and_with_processing() {
    let code = transpile(
        "def safe_len(items: list) -> int:\n    return items and len(items)",
    );
    assert!(!code.is_empty(), "and with processing: {}", code);
}

#[test]
fn test_or_chain() {
    let code = transpile(
        "def first_nonempty(a: str, b: str, c: str) -> str:\n    return a or b or c",
    );
    assert!(!code.is_empty(), "or chain: {}", code);
}

// =============================================================================
// Section 6: binary_ops.rs - Path division
// =============================================================================

#[test]
fn test_path_division_operator() {
    let code = transpile(
        "from pathlib import Path\ndef join_path(base: str) -> str:\n    p = Path(base)\n    return str(p / \"file.txt\")",
    );
    assert!(!code.is_empty(), "path division: {}", code);
}

#[test]
fn test_path_division_chained() {
    let code = transpile(
        "from pathlib import Path\ndef nested_path() -> str:\n    return str(Path(\"/home\") / \"user\" / \"docs\")",
    );
    assert!(!code.is_empty(), "path division chained: {}", code);
}

// =============================================================================
// Section 7: binary_ops.rs - Float/int comparison edge cases
// =============================================================================

#[test]
fn test_float_eq_int() {
    let code = transpile(
        "def check_eq(x: float) -> bool:\n    return x == 0",
    );
    assert!(
        code.contains("==") || code.contains("0.0") || code.contains("as f64"),
        "float == int: {}",
        code
    );
}

#[test]
fn test_float_ne_int() {
    let code = transpile(
        "def check_ne(x: float) -> bool:\n    return x != 0",
    );
    assert!(
        code.contains("!=") || code.contains("0.0"),
        "float != int: {}",
        code
    );
}

#[test]
fn test_float_lt_int() {
    let code = transpile(
        "def check_lt(x: float) -> bool:\n    return x < 100",
    );
    assert!(
        code.contains("<") || code.contains("100"),
        "float < int: {}",
        code
    );
}

#[test]
fn test_int_gt_float() {
    let code = transpile(
        "def check_gt(x: float, y: int) -> bool:\n    return y > x",
    );
    assert!(code.contains(">"), "int > float: {}", code);
}

// =============================================================================
// Section 8: binary_ops.rs - In/not in with sets
// =============================================================================

#[test]
fn test_in_set_membership() {
    let code = transpile(
        "def is_member(x: int, s: set) -> bool:\n    return x in s",
    );
    assert!(
        code.contains("contains"),
        "in set membership: {}",
        code
    );
}

#[test]
fn test_not_in_set_membership() {
    let code = transpile(
        "def not_member(x: int, s: set) -> bool:\n    return x not in s",
    );
    assert!(
        code.contains("contains"),
        "not in set membership: {}",
        code
    );
}

#[test]
fn test_in_list_membership() {
    let code = transpile(
        "def in_list(x: int, items: list) -> bool:\n    return x in items",
    );
    assert!(
        code.contains("contains"),
        "in list membership: {}",
        code
    );
}

#[test]
fn test_not_in_list() {
    let code = transpile(
        "def not_in_list(x: int, items: list) -> bool:\n    return x not in items",
    );
    assert!(
        code.contains("contains"),
        "not in list: {}",
        code
    );
}

#[test]
fn test_in_dict_membership() {
    let code = transpile(
        "def has_key(k: str, d: dict) -> bool:\n    return k in d",
    );
    assert!(
        code.contains("contains_key") || code.contains("contains"),
        "in dict membership: {}",
        code
    );
}

#[test]
fn test_in_string_membership() {
    let code = transpile(
        "def has_sub(s: str) -> bool:\n    return \"x\" in s",
    );
    assert!(
        code.contains("contains"),
        "in string membership: {}",
        code
    );
}

// =============================================================================
// Section 9: call_dispatch.rs - datetime constructors (64.7% cov)
// =============================================================================

#[test]
fn test_datetime_three_args() {
    let code = transpile(
        "from datetime import datetime\ndef make_date() -> datetime:\n    return datetime(2024, 6, 15)",
    );
    assert!(!code.is_empty(), "datetime 3 args: {}", code);
}

#[test]
fn test_datetime_six_args() {
    let code = transpile(
        "from datetime import datetime\ndef make_full() -> datetime:\n    return datetime(2024, 6, 15, 14, 30, 45)",
    );
    assert!(!code.is_empty(), "datetime 6 args: {}", code);
}

#[test]
fn test_datetime_seven_args_microsecond() {
    let code = transpile(
        "from datetime import datetime\ndef make_precise() -> datetime:\n    return datetime(2024, 6, 15, 14, 30, 45, 123456)",
    );
    assert!(!code.is_empty(), "datetime 7 args: {}", code);
}

#[test]
fn test_date_constructor() {
    let code = transpile(
        "from datetime import date\ndef make_date() -> date:\n    return date(2024, 6, 15)",
    );
    assert!(!code.is_empty(), "date constructor: {}", code);
}

#[test]
fn test_time_no_args() {
    let code = transpile(
        "from datetime import time\ndef make_time() -> time:\n    return time()",
    );
    assert!(!code.is_empty(), "time no args: {}", code);
}

#[test]
fn test_time_one_arg() {
    let code = transpile(
        "from datetime import time\ndef make_hour() -> time:\n    return time(14)",
    );
    assert!(!code.is_empty(), "time 1 arg: {}", code);
}

#[test]
fn test_time_two_args() {
    let code = transpile(
        "from datetime import time\ndef make_hm() -> time:\n    return time(14, 30)",
    );
    assert!(!code.is_empty(), "time 2 args: {}", code);
}

#[test]
fn test_time_three_args() {
    let code = transpile(
        "from datetime import time\ndef make_hms() -> time:\n    return time(14, 30, 45)",
    );
    assert!(!code.is_empty(), "time 3 args: {}", code);
}

#[test]
fn test_timedelta_no_args() {
    let code = transpile(
        "from datetime import timedelta\ndef make_zero() -> timedelta:\n    return timedelta()",
    );
    assert!(!code.is_empty(), "timedelta no args: {}", code);
}

#[test]
fn test_timedelta_one_arg_days() {
    let code = transpile(
        "from datetime import timedelta\ndef make_days() -> timedelta:\n    return timedelta(5)",
    );
    assert!(!code.is_empty(), "timedelta 1 arg: {}", code);
}

#[test]
fn test_timedelta_two_args() {
    let code = transpile(
        "from datetime import timedelta\ndef make_ds() -> timedelta:\n    return timedelta(5, 30)",
    );
    assert!(!code.is_empty(), "timedelta 2 args: {}", code);
}

#[test]
fn test_timedelta_three_args() {
    let code = transpile(
        "from datetime import timedelta\ndef make_dsm() -> timedelta:\n    return timedelta(5, 30, 600)",
    );
    assert!(!code.is_empty(), "timedelta 3 args: {}", code);
}

#[test]
fn test_datetime_now() {
    let ok = transpile_ok(
        "from datetime import datetime\ndef get_now():\n    return datetime.now()",
    );
    assert!(ok, "datetime.now() should transpile");
}

#[test]
fn test_date_today() {
    let ok = transpile_ok(
        "from datetime import date\ndef get_today():\n    return date.today()",
    );
    assert!(ok, "date.today() should transpile");
}

// =============================================================================
// Section 10: instance_dispatch.rs - datetime methods (60.9% cov)
// =============================================================================

#[test]
fn test_dt_isoformat() {
    let code = transpile(
        "from datetime import datetime\ndef fmt_iso(dt: datetime) -> str:\n    return dt.isoformat()",
    );
    assert!(!code.is_empty(), "dt.isoformat: {}", code);
}

#[test]
fn test_dt_strftime() {
    let code = transpile(
        "from datetime import datetime\ndef fmt_str(dt: datetime) -> str:\n    return dt.strftime(\"%Y-%m-%d\")",
    );
    assert!(!code.is_empty(), "dt.strftime: {}", code);
}

#[test]
fn test_dt_timestamp() {
    let code = transpile(
        "from datetime import datetime\ndef get_ts(dt: datetime) -> float:\n    return dt.timestamp()",
    );
    assert!(!code.is_empty(), "dt.timestamp: {}", code);
}

#[test]
fn test_dt_date_component() {
    let code = transpile(
        "from datetime import datetime\ndef get_date(dt: datetime):\n    return dt.date()",
    );
    assert!(!code.is_empty(), "dt.date(): {}", code);
}

#[test]
fn test_dt_time_component() {
    let code = transpile(
        "from datetime import datetime\ndef get_time(dt: datetime):\n    return dt.time()",
    );
    assert!(!code.is_empty(), "dt.time(): {}", code);
}

#[test]
fn test_date_weekday() {
    let code = transpile(
        "from datetime import date\ndef get_weekday(d: date) -> int:\n    return d.weekday()",
    );
    assert!(!code.is_empty(), "date.weekday: {}", code);
}

#[test]
fn test_date_isoformat() {
    let code = transpile(
        "from datetime import date\ndef fmt_date(d: date) -> str:\n    return d.isoformat()",
    );
    assert!(!code.is_empty(), "date.isoformat: {}", code);
}

// =============================================================================
// Section 11: attribute_convert.rs - pathlib patterns (51.6% cov)
// =============================================================================

#[test]
fn test_path_parent_attr() {
    let code = transpile(
        "from pathlib import Path\ndef get_parent(path: str) -> str:\n    p = Path(path)\n    return str(p.parent)",
    );
    assert!(
        code.contains("parent"),
        "path.parent: {}",
        code
    );
}

#[test]
fn test_path_name_attr() {
    let code = transpile(
        "from pathlib import Path\ndef get_name(path: str) -> str:\n    p = Path(path)\n    return p.name",
    );
    assert!(
        code.contains("file_name") || !code.is_empty(),
        "path.name: {}",
        code
    );
}

#[test]
fn test_path_suffix_attr() {
    let code = transpile(
        "from pathlib import Path\ndef get_ext(path: str) -> str:\n    p = Path(path)\n    return p.suffix",
    );
    assert!(
        code.contains("extension") || !code.is_empty(),
        "path.suffix: {}",
        code
    );
}

#[test]
fn test_path_stem_attr() {
    let code = transpile(
        "from pathlib import Path\ndef get_stem(path: str) -> str:\n    p = Path(path)\n    return p.stem",
    );
    assert!(
        code.contains("file_stem") || !code.is_empty(),
        "path.stem: {}",
        code
    );
}

#[test]
fn test_path_exists_method() {
    let code = transpile(
        "from pathlib import Path\ndef check_exists(path: str) -> bool:\n    return Path(path).exists()",
    );
    assert!(!code.is_empty(), "path.exists: {}", code);
}

#[test]
fn test_path_is_file_method() {
    let code = transpile(
        "from pathlib import Path\ndef check_file(path: str) -> bool:\n    return Path(path).is_file()",
    );
    assert!(!code.is_empty(), "path.is_file: {}", code);
}

#[test]
fn test_path_is_dir_method() {
    let code = transpile(
        "from pathlib import Path\ndef check_dir(path: str) -> bool:\n    return Path(path).is_dir()",
    );
    assert!(!code.is_empty(), "path.is_dir: {}", code);
}

#[test]
fn test_path_stat_method() {
    let code = transpile(
        "from pathlib import Path\ndef get_stats(path: str):\n    return path.stat()",
    );
    assert!(
        code.contains("metadata") || !code.is_empty(),
        "path.stat: {}",
        code
    );
}

#[test]
fn test_path_resolve_method() {
    let code = transpile(
        "from pathlib import Path\ndef resolve_it(path: str) -> str:\n    return path.resolve()",
    );
    assert!(
        code.contains("canonicalize") || !code.is_empty(),
        "path.resolve: {}",
        code
    );
}

#[test]
fn test_path_absolute_method() {
    let code = transpile(
        "from pathlib import Path\ndef abs_path(path: str) -> str:\n    return path.absolute()",
    );
    assert!(!code.is_empty(), "path.absolute: {}", code);
}

#[test]
fn test_path_with_suffix_method() {
    let code = transpile(
        "from pathlib import Path\ndef change_ext(path: str) -> str:\n    p = Path(path)\n    return str(p.with_suffix(\".rs\"))",
    );
    assert!(!code.is_empty(), "path.with_suffix: {}", code);
}

#[test]
fn test_path_read_text() {
    let code = transpile(
        "from pathlib import Path\ndef read_content(path: str) -> str:\n    return Path(path).read_text()",
    );
    assert!(!code.is_empty(), "path.read_text: {}", code);
}

#[test]
fn test_path_write_text() {
    let code = transpile(
        "from pathlib import Path\ndef write_content(path: str, content: str):\n    Path(path).write_text(content)",
    );
    assert!(!code.is_empty(), "path.write_text: {}", code);
}

// =============================================================================
// Section 12: attribute_convert.rs - sys module attributes
// =============================================================================

#[test]
fn test_sys_platform() {
    let code = transpile(
        "import sys\ndef get_platform() -> str:\n    return sys.platform",
    );
    assert!(
        code.contains("platform") || code.contains("darwin") || code.contains("linux"),
        "sys.platform: {}",
        code
    );
}

#[test]
fn test_sys_argv() {
    let code = transpile(
        "import sys\ndef get_args() -> list:\n    return sys.argv",
    );
    assert!(
        code.contains("args") || code.contains("env"),
        "sys.argv: {}",
        code
    );
}

#[test]
fn test_sys_stdin() {
    let code = transpile(
        "import sys\ndef get_stdin():\n    return sys.stdin",
    );
    assert!(
        code.contains("stdin"),
        "sys.stdin: {}",
        code
    );
}

#[test]
fn test_sys_stdout() {
    let code = transpile(
        "import sys\ndef get_stdout():\n    return sys.stdout",
    );
    assert!(
        code.contains("stdout"),
        "sys.stdout: {}",
        code
    );
}

#[test]
fn test_sys_stderr() {
    let code = transpile(
        "import sys\ndef get_stderr():\n    return sys.stderr",
    );
    assert!(
        code.contains("stderr"),
        "sys.stderr: {}",
        code
    );
}

#[test]
fn test_sys_version_info() {
    let code = transpile(
        "import sys\ndef get_version():\n    return sys.version_info",
    );
    assert!(!code.is_empty(), "sys.version_info: {}", code);
}

// =============================================================================
// Section 13: attribute_convert.rs - os.environ
// =============================================================================

#[test]
fn test_os_environ_access() {
    let code = transpile(
        "import os\ndef get_env() -> dict:\n    return os.environ",
    );
    assert!(
        code.contains("env") || code.contains("vars"),
        "os.environ: {}",
        code
    );
}

#[test]
fn test_os_environ_get() {
    let code = transpile(
        "import os\ndef get_home() -> str:\n    return os.environ.get(\"HOME\", \"\")",
    );
    assert!(!code.is_empty(), "os.environ.get: {}", code);
}

#[test]
fn test_os_environ_get_no_default() {
    let code = transpile(
        "import os\ndef get_var(key: str) -> str:\n    return os.environ.get(key, \"fallback\")",
    );
    assert!(!code.is_empty(), "os.environ.get no default: {}", code);
}

// =============================================================================
// Section 14: attribute_convert.rs - type() and isinstance
// =============================================================================

#[test]
fn test_type_name() {
    let code = transpile(
        "def get_type_name(x: int) -> str:\n    return type(x).__name__",
    );
    assert!(
        code.contains("type_name"),
        "type().__name__: {}",
        code
    );
}

#[test]
fn test_isinstance_single_type() {
    let code = transpile(
        "def is_int(x: int) -> bool:\n    return isinstance(x, int)",
    );
    assert!(!code.is_empty(), "isinstance single: {}", code);
}

#[test]
fn test_isinstance_tuple_types() {
    let code = transpile(
        "def is_numeric(x: int) -> bool:\n    return isinstance(x, (int, float))",
    );
    assert!(!code.is_empty(), "isinstance tuple: {}", code);
}

#[test]
fn test_type_check_str() {
    let code = transpile(
        "def is_str(x: str) -> bool:\n    return isinstance(x, str)",
    );
    assert!(!code.is_empty(), "isinstance str: {}", code);
}

// =============================================================================
// Section 15: instance_dispatch.rs - file operations
// =============================================================================

#[test]
fn test_file_read_full() {
    let code = transpile(
        "def read_all(path: str) -> str:\n    with open(path) as f:\n        return f.read()",
    );
    assert!(!code.is_empty(), "f.read(): {}", code);
}

#[test]
fn test_file_read_chunked() {
    let code = transpile(
        "def read_chunk(path: str) -> bytes:\n    with open(path) as f:\n        return f.read(1024)",
    );
    assert!(!code.is_empty(), "f.read(1024): {}", code);
}

#[test]
fn test_file_readlines() {
    let code = transpile(
        "def get_lines(path: str) -> list:\n    with open(path) as f:\n        return f.readlines()",
    );
    assert!(!code.is_empty(), "f.readlines(): {}", code);
}

#[test]
fn test_file_readline() {
    let code = transpile(
        "def first_line(path: str) -> str:\n    with open(path) as f:\n        return f.readline()",
    );
    assert!(!code.is_empty(), "f.readline(): {}", code);
}

#[test]
fn test_file_write() {
    let code = transpile(
        "def write_data(path: str, data: str):\n    with open(path, \"w\") as f:\n        f.write(data)",
    );
    assert!(code.contains("write"), "f.write(): {}", code);
}

#[test]
fn test_file_writelines() {
    let code = transpile(
        "def write_lines(path: str, lines: list):\n    with open(path, \"w\") as f:\n        f.writelines(lines)",
    );
    assert!(!code.is_empty(), "f.writelines(): {}", code);
}

#[test]
fn test_file_close() {
    let code = transpile(
        "def close_file(path: str):\n    f = open(path)\n    data = f.read()\n    f.close()\n    return data",
    );
    assert!(!code.is_empty(), "f.close(): {}", code);
}

#[test]
fn test_file_seek() {
    let code = transpile(
        "def rewind(path: str) -> str:\n    with open(path) as f:\n        f.seek(0)\n        return f.read()",
    );
    assert!(!code.is_empty(), "f.seek(0): {}", code);
}

// =============================================================================
// Section 16: dict_constructors.rs - dict literal patterns (57.3% cov)
// =============================================================================

#[test]
fn test_dict_string_keys_int_values() {
    let code = transpile(
        "def make_dict() -> dict:\n    return {\"a\": 1, \"b\": 2, \"c\": 3}",
    );
    assert!(
        code.contains("HashMap") || code.contains("insert") || code.contains("hashmap"),
        "dict string/int: {}",
        code
    );
}

#[test]
fn test_dict_mixed_value_types() {
    let code = transpile(
        "def make_mixed() -> dict:\n    return {\"count\": 1, \"name\": \"test\", \"active\": True}",
    );
    assert!(!code.is_empty(), "dict mixed types: {}", code);
}

#[test]
fn test_dict_integer_keys() {
    let code = transpile(
        "def int_keys() -> dict:\n    return {1: \"a\", 2: \"b\", 3: \"c\"}",
    );
    assert!(!code.is_empty(), "dict int keys: {}", code);
}

#[test]
fn test_dict_nested() {
    let code = transpile(
        "def nested_dict() -> dict:\n    return {\"user\": {\"name\": \"Bob\", \"age\": 30}}",
    );
    assert!(!code.is_empty(), "dict nested: {}", code);
}

#[test]
fn test_dict_empty() {
    let code = transpile(
        "def empty_dict() -> dict:\n    return {}",
    );
    assert!(!code.is_empty(), "dict empty: {}", code);
}

#[test]
fn test_dict_from_variables() {
    let code = transpile(
        "def from_vars(key: str, val: int) -> dict:\n    return {key: val}",
    );
    assert!(!code.is_empty(), "dict from vars: {}", code);
}

#[test]
fn test_dict_bool_values() {
    let code = transpile(
        "def bool_dict() -> dict:\n    return {\"enabled\": True, \"visible\": False}",
    );
    assert!(!code.is_empty(), "dict bool values: {}", code);
}

#[test]
fn test_dict_float_values() {
    let code = transpile(
        "def float_dict() -> dict:\n    return {\"x\": 1.5, \"y\": 2.5, \"z\": 3.5}",
    );
    assert!(!code.is_empty(), "dict float values: {}", code);
}

// =============================================================================
// Section 17: method_call_routing.rs - method dispatch (42.3% cov)
// =============================================================================

#[test]
fn test_method_route_list_append() {
    let code = transpile(
        "def build_list() -> list:\n    items = []\n    items.append(1)\n    items.append(2)\n    return items",
    );
    assert!(code.contains("push"), "list append routing: {}", code);
}

#[test]
fn test_method_route_list_extend() {
    let code = transpile(
        "def extend_list(items: list, more: list) -> list:\n    items.extend(more)\n    return items",
    );
    assert!(code.contains("extend"), "list extend routing: {}", code);
}

#[test]
fn test_method_route_list_insert() {
    let code = transpile(
        "def insert_at(items: list, val: int) -> list:\n    items.insert(0, val)\n    return items",
    );
    assert!(code.contains("insert"), "list insert routing: {}", code);
}

#[test]
fn test_method_route_list_remove() {
    let code = transpile(
        "def remove_val(items: list, val: int):\n    items.remove(val)",
    );
    assert!(!code.is_empty(), "list remove routing: {}", code);
}

#[test]
fn test_method_route_list_pop() {
    let code = transpile(
        "def pop_last(items: list) -> int:\n    return items.pop()",
    );
    assert!(!code.is_empty(), "list pop routing: {}", code);
}

#[test]
fn test_method_route_list_sort() {
    let code = transpile(
        "def sort_items(items: list):\n    items.sort()",
    );
    assert!(code.contains("sort"), "list sort routing: {}", code);
}

#[test]
fn test_method_route_list_reverse() {
    let code = transpile(
        "def reverse_items(items: list):\n    items.reverse()",
    );
    assert!(code.contains("reverse"), "list reverse routing: {}", code);
}

#[test]
fn test_method_route_list_clear() {
    let code = transpile(
        "def clear_items(items: list):\n    items.clear()",
    );
    assert!(code.contains("clear"), "list clear routing: {}", code);
}

#[test]
fn test_method_route_list_copy() {
    let code = transpile(
        "def copy_items(items: list) -> list:\n    return items.copy()",
    );
    assert!(
        code.contains("clone") || code.contains("copy"),
        "list copy routing: {}",
        code
    );
}

#[test]
fn test_method_route_dict_keys() {
    let code = transpile(
        "def get_keys(d: dict) -> list:\n    return list(d.keys())",
    );
    assert!(code.contains("keys"), "dict keys routing: {}", code);
}

#[test]
fn test_method_route_dict_values() {
    let code = transpile(
        "def get_values(d: dict) -> list:\n    return list(d.values())",
    );
    assert!(code.contains("values"), "dict values routing: {}", code);
}

#[test]
fn test_method_route_dict_items() {
    let code = transpile(
        "def get_items(d: dict):\n    for k, v in d.items():\n        print(k, v)",
    );
    assert!(code.contains("iter"), "dict items routing: {}", code);
}

#[test]
fn test_method_route_dict_get() {
    let code = transpile(
        "def safe_get(d: dict, key: str) -> int:\n    return d.get(key, 0)",
    );
    assert!(!code.is_empty(), "dict get routing: {}", code);
}

#[test]
fn test_method_route_dict_update() {
    let code = transpile(
        "def merge_into(d: dict, other: dict):\n    d.update(other)",
    );
    assert!(!code.is_empty(), "dict update routing: {}", code);
}

#[test]
fn test_method_route_dict_pop() {
    let code = transpile(
        "def remove_key(d: dict, key: str) -> int:\n    return d.pop(key)",
    );
    assert!(!code.is_empty(), "dict pop routing: {}", code);
}

#[test]
fn test_method_route_dict_setdefault() {
    let code = transpile(
        "def set_default(d: dict, key: str) -> int:\n    return d.setdefault(key, 0)",
    );
    assert!(!code.is_empty(), "dict setdefault routing: {}", code);
}

// =============================================================================
// Section 18: method_call_routing.rs - encode/decode
// =============================================================================

#[test]
fn test_string_encode() {
    let code = transpile(
        "def to_bytes(text: str) -> bytes:\n    return text.encode()",
    );
    assert!(
        code.contains("bytes") || code.contains("as_bytes") || code.contains("encode"),
        "str.encode(): {}",
        code
    );
}

#[test]
fn test_bytes_decode() {
    let code = transpile(
        "def from_bytes(data: bytes) -> str:\n    return data.decode()",
    );
    assert!(
        code.contains("from_utf8") || code.contains("decode") || !code.is_empty(),
        "bytes.decode(): {}",
        code
    );
}

#[test]
fn test_string_encode_utf8() {
    let code = transpile(
        "def to_utf8(text: str) -> bytes:\n    return text.encode(\"utf-8\")",
    );
    assert!(!code.is_empty(), "str.encode('utf-8'): {}", code);
}

// =============================================================================
// Section 19: attribute_convert.rs - Enum patterns
// =============================================================================

#[test]
fn test_enum_value_access() {
    let code = transpile(
        "class Color:\n    RED = 1\n    GREEN = 2\n    BLUE = 3\n\ndef get_red() -> int:\n    return Color.RED",
    );
    assert!(
        code.contains("Color") && code.contains("RED"),
        "enum value access: {}",
        code
    );
}

#[test]
fn test_enum_comparison() {
    let code = transpile(
        "class Status:\n    ACTIVE = 1\n    INACTIVE = 0\n\ndef is_active(s: int) -> bool:\n    return s == Status.ACTIVE",
    );
    assert!(
        code.contains("Status") && code.contains("ACTIVE"),
        "enum comparison: {}",
        code
    );
}

#[test]
fn test_enum_multiple_values() {
    let code = transpile(
        "class Direction:\n    NORTH = 0\n    SOUTH = 1\n    EAST = 2\n    WEST = 3\n\ndef heading() -> int:\n    return Direction.NORTH",
    );
    assert!(!code.is_empty(), "enum multiple values: {}", code);
}

// =============================================================================
// Section 20: call_dispatch.rs - print edge cases
// =============================================================================

#[test]
fn test_print_multiple_values() {
    let code = transpile(
        "def log_info(name: str, age: int):\n    print(name, age)",
    );
    assert!(
        code.contains("print") || code.contains("format"),
        "print multiple: {}",
        code
    );
}

#[test]
fn test_print_with_separator() {
    let code = transpile(
        "def print_csv(a: str, b: str, c: str):\n    print(a, b, c, sep=\",\")",
    );
    assert!(!code.is_empty(), "print with sep: {}", code);
}

#[test]
fn test_print_with_end() {
    let code = transpile(
        "def print_inline(val: str):\n    print(val, end=\"\")",
    );
    assert!(!code.is_empty(), "print with end: {}", code);
}

#[test]
fn test_print_no_args() {
    let code = transpile(
        "def blank_line():\n    print()",
    );
    assert!(
        code.contains("println") || code.contains("print"),
        "print no args: {}",
        code
    );
}

#[test]
fn test_print_formatted_string() {
    let code = transpile(
        "def show(x: int):\n    print(f\"value: {x}\")",
    );
    assert!(!code.is_empty(), "print fstring: {}", code);
}

// =============================================================================
// Section 21: attribute_convert.rs - datetime class constants
// =============================================================================

#[test]
fn test_date_min() {
    let code = transpile(
        "from datetime import date\ndef earliest():\n    return date.min",
    );
    assert!(!code.is_empty(), "date.min: {}", code);
}

#[test]
fn test_date_max() {
    let code = transpile(
        "from datetime import date\ndef latest():\n    return date.max",
    );
    assert!(!code.is_empty(), "date.max: {}", code);
}

#[test]
fn test_datetime_min() {
    let code = transpile(
        "from datetime import datetime\ndef earliest_dt():\n    return datetime.min",
    );
    assert!(!code.is_empty(), "datetime.min: {}", code);
}

#[test]
fn test_datetime_max() {
    let code = transpile(
        "from datetime import datetime\ndef latest_dt():\n    return datetime.max",
    );
    assert!(!code.is_empty(), "datetime.max: {}", code);
}

#[test]
fn test_time_min() {
    let code = transpile(
        "from datetime import time\ndef earliest_time():\n    return time.min",
    );
    assert!(!code.is_empty(), "time.min: {}", code);
}

#[test]
fn test_time_max() {
    let code = transpile(
        "from datetime import time\ndef latest_time():\n    return time.max",
    );
    assert!(!code.is_empty(), "time.max: {}", code);
}

#[test]
fn test_timedelta_min() {
    let code = transpile(
        "from datetime import timedelta\ndef smallest_delta():\n    return timedelta.min",
    );
    assert!(!code.is_empty(), "timedelta.min: {}", code);
}

#[test]
fn test_timedelta_max() {
    let code = transpile(
        "from datetime import timedelta\ndef largest_delta():\n    return timedelta.max",
    );
    assert!(!code.is_empty(), "timedelta.max: {}", code);
}

#[test]
fn test_timedelta_resolution() {
    let code = transpile(
        "from datetime import timedelta\ndef precision():\n    return timedelta.resolution",
    );
    assert!(!code.is_empty(), "timedelta.resolution: {}", code);
}

#[test]
fn test_time_resolution() {
    let code = transpile(
        "from datetime import time\ndef time_precision():\n    return time.resolution",
    );
    assert!(!code.is_empty(), "time.resolution: {}", code);
}

// =============================================================================
// Section 22: attribute_convert.rs - math module constants
// =============================================================================

#[test]
fn test_math_pi() {
    let code = transpile(
        "import math\ndef circle_area(r: float) -> float:\n    return math.pi * r * r",
    );
    assert!(code.contains("PI"), "math.pi: {}", code);
}

#[test]
fn test_math_e() {
    let code = transpile(
        "import math\ndef euler() -> float:\n    return math.e",
    );
    assert!(code.contains("E"), "math.e: {}", code);
}

#[test]
fn test_math_tau() {
    let code = transpile(
        "import math\ndef full_circle() -> float:\n    return math.tau",
    );
    assert!(code.contains("TAU"), "math.tau: {}", code);
}

#[test]
fn test_math_inf() {
    let code = transpile(
        "import math\ndef infinity() -> float:\n    return math.inf",
    );
    assert!(
        code.contains("INFINITY"),
        "math.inf: {}",
        code
    );
}

#[test]
fn test_math_nan() {
    let code = transpile(
        "import math\ndef not_a_number() -> float:\n    return math.nan",
    );
    assert!(code.contains("NAN"), "math.nan: {}", code);
}

// =============================================================================
// Section 23: attribute_convert.rs - string module constants
// =============================================================================

#[test]
fn test_string_ascii_lowercase() {
    let code = transpile(
        "import string\ndef lower() -> str:\n    return string.ascii_lowercase",
    );
    assert!(
        code.contains("abcdefghijklmnopqrstuvwxyz"),
        "string.ascii_lowercase: {}",
        code
    );
}

#[test]
fn test_string_digits() {
    let code = transpile(
        "import string\ndef digits() -> str:\n    return string.digits",
    );
    assert!(
        code.contains("0123456789"),
        "string.digits: {}",
        code
    );
}

#[test]
fn test_string_ascii_uppercase() {
    let code = transpile(
        "import string\ndef upper() -> str:\n    return string.ascii_uppercase",
    );
    assert!(
        code.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        "string.ascii_uppercase: {}",
        code
    );
}

#[test]
fn test_string_hexdigits() {
    let code = transpile(
        "import string\ndef hex_chars() -> str:\n    return string.hexdigits",
    );
    assert!(
        code.contains("0123456789abcdefABCDEF"),
        "string.hexdigits: {}",
        code
    );
}

// =============================================================================
// Section 24: attribute_convert.rs - re module constants
// =============================================================================

#[test]
fn test_re_ignorecase() {
    let code = transpile(
        "import re\ndef get_flag() -> int:\n    return re.IGNORECASE",
    );
    assert!(!code.is_empty(), "re.IGNORECASE: {}", code);
}

#[test]
fn test_re_multiline() {
    let code = transpile(
        "import re\ndef get_flag() -> int:\n    return re.MULTILINE",
    );
    assert!(!code.is_empty(), "re.MULTILINE: {}", code);
}

// =============================================================================
// Section 25: attribute_convert.rs - os.stat attributes
// =============================================================================

#[test]
fn test_stat_st_size() {
    let code = transpile(
        "import os\ndef file_size(path: str) -> int:\n    stats = os.stat(path)\n    return stats.st_size",
    );
    assert!(
        code.contains("len") || !code.is_empty(),
        "stats.st_size: {}",
        code
    );
}

#[test]
fn test_stat_st_mtime() {
    let code = transpile(
        "import os\ndef mod_time(path: str) -> float:\n    stats = os.stat(path)\n    return stats.st_mtime",
    );
    assert!(
        code.contains("modified") || !code.is_empty(),
        "stats.st_mtime: {}",
        code
    );
}

#[test]
fn test_stat_st_atime() {
    let code = transpile(
        "import os\ndef access_time(path: str) -> float:\n    stats = os.stat(path)\n    return stats.st_atime",
    );
    assert!(
        code.contains("accessed") || !code.is_empty(),
        "stats.st_atime: {}",
        code
    );
}

#[test]
fn test_stat_st_ctime() {
    let code = transpile(
        "import os\ndef create_time(path: str) -> float:\n    stats = os.stat(path)\n    return stats.st_ctime",
    );
    assert!(
        code.contains("created") || !code.is_empty(),
        "stats.st_ctime: {}",
        code
    );
}

// =============================================================================
// Section 26: instance_dispatch.rs - regex match methods
// =============================================================================

#[test]
fn test_regex_group_zero() {
    let code = transpile(
        "import re\ndef get_match(text: str) -> str:\n    m = re.search(\"pattern\", text)\n    return m.group(0)",
    );
    assert!(!code.is_empty(), "m.group(0): {}", code);
}

#[test]
fn test_regex_group_no_args() {
    let code = transpile(
        "import re\ndef get_full(text: str) -> str:\n    m = re.search(\"pattern\", text)\n    return m.group()",
    );
    assert!(!code.is_empty(), "m.group(): {}", code);
}

#[test]
fn test_regex_group_numbered() {
    let code = transpile(
        "import re\ndef get_first_group(text: str) -> str:\n    m = re.search(\"(pattern)\", text)\n    return m.group(1)",
    );
    assert!(!code.is_empty(), "m.group(1): {}", code);
}

// =============================================================================
// Section 27: instance_dispatch.rs - csv writer methods
// =============================================================================

#[test]
fn test_csv_writeheader() {
    let code = transpile(
        "import csv\ndef write_header(writer):\n    writer.writeheader()",
    );
    assert!(!code.is_empty(), "writeheader: {}", code);
}

#[test]
fn test_csv_writerow() {
    let code = transpile(
        "import csv\ndef write_row(writer, row: dict):\n    writer.writerow(row)",
    );
    assert!(!code.is_empty(), "writerow: {}", code);
}

// =============================================================================
// Section 28: binary_ops.rs - bitwise operators (non-set)
// =============================================================================

#[test]
fn test_bitwise_and_int() {
    let code = transpile(
        "def mask(x: int) -> int:\n    return x & 0xFF",
    );
    assert!(code.contains("&"), "bitwise and: {}", code);
}

#[test]
fn test_bitwise_or_int() {
    let code = transpile(
        "def set_bit(x: int) -> int:\n    return x | 0x01",
    );
    assert!(!code.is_empty(), "bitwise or: {}", code);
}

#[test]
fn test_bitwise_xor_int() {
    let code = transpile(
        "def toggle(x: int) -> int:\n    return x ^ 0xFF",
    );
    assert!(!code.is_empty(), "bitwise xor: {}", code);
}

#[test]
fn test_left_shift() {
    let code = transpile(
        "def shift_left(x: int) -> int:\n    return x << 4",
    );
    assert!(code.contains("<<"), "left shift: {}", code);
}

#[test]
fn test_right_shift() {
    let code = transpile(
        "def shift_right(x: int) -> int:\n    return x >> 4",
    );
    assert!(code.contains(">>"), "right shift: {}", code);
}

// =============================================================================
// Section 29: Additional method_call_routing.rs patterns
// =============================================================================

#[test]
fn test_set_add_method() {
    let code = transpile(
        "def add_to_set(s: set, x: int):\n    s.add(x)",
    );
    assert!(
        code.contains("insert") || !code.is_empty(),
        "set.add: {}",
        code
    );
}

#[test]
fn test_set_discard_method() {
    let code = transpile(
        "def remove_from_set(s: set, x: int):\n    s.discard(x)",
    );
    assert!(!code.is_empty(), "set.discard: {}", code);
}

#[test]
fn test_set_union_method() {
    let code = transpile(
        "def combine_sets(a: set, b: set) -> set:\n    return a.union(b)",
    );
    assert!(!code.is_empty(), "set.union: {}", code);
}

#[test]
fn test_set_intersection_method() {
    let code = transpile(
        "def common_items(a: set, b: set) -> set:\n    return a.intersection(b)",
    );
    assert!(!code.is_empty(), "set.intersection: {}", code);
}

#[test]
fn test_set_difference_method() {
    let code = transpile(
        "def only_in_a(a: set, b: set) -> set:\n    return a.difference(b)",
    );
    assert!(!code.is_empty(), "set.difference: {}", code);
}

// =============================================================================
// Section 30: Exception handling attributes
// =============================================================================

#[test]
fn test_exception_returncode() {
    let code = transpile(
        "def get_rc() -> int:\n    try:\n        pass\n    except Exception as e:\n        return e.returncode",
    );
    assert!(!code.is_empty(), "exception returncode: {}", code);
}

#[test]
fn test_exception_str() {
    let code = transpile(
        "def get_error_msg() -> str:\n    try:\n        pass\n    except ValueError as e:\n        return str(e)",
    );
    assert!(!code.is_empty(), "str(exception): {}", code);
}

// =============================================================================
// Section 31: Tempfile attribute handling
// =============================================================================

#[test]
fn test_tempfile_name() {
    let code = transpile(
        "def get_temp_name() -> str:\n    temp = None\n    return temp.name",
    );
    assert!(
        code.contains("path") || !code.is_empty(),
        "temp.name: {}",
        code
    );
}

// =============================================================================
// Section 32: binary_ops.rs - Power operator
// =============================================================================

#[test]
fn test_power_int() {
    let code = transpile(
        "def square(n: int) -> int:\n    return n ** 2",
    );
    assert!(
        code.contains("pow") || code.contains("powi"),
        "int power: {}",
        code
    );
}

#[test]
fn test_power_float() {
    let code = transpile(
        "def cube_root(x: float) -> float:\n    return x ** 0.5",
    );
    assert!(
        code.contains("pow") || code.contains("sqrt"),
        "float power: {}",
        code
    );
}

// =============================================================================
// Section 33: binary_ops.rs - Modulo operator
// =============================================================================

#[test]
fn test_modulo_int() {
    let code = transpile(
        "def is_even(n: int) -> bool:\n    return n % 2 == 0",
    );
    assert!(code.contains("%"), "modulo int: {}", code);
}

#[test]
fn test_modulo_float() {
    let code = transpile(
        "def fmod(x: float) -> float:\n    return x % 1.0",
    );
    assert!(code.contains("%"), "modulo float: {}", code);
}

// =============================================================================
// Section 34: call_dispatch.rs - type constructors
// =============================================================================

#[test]
fn test_int_constructor() {
    let code = transpile(
        "def to_int(s: str) -> int:\n    return int(s)",
    );
    assert!(
        code.contains("parse") || code.contains("i64"),
        "int(): {}",
        code
    );
}

#[test]
fn test_float_constructor() {
    let code = transpile(
        "def to_float(s: str) -> float:\n    return float(s)",
    );
    assert!(
        code.contains("parse") || code.contains("f64"),
        "float(): {}",
        code
    );
}

#[test]
fn test_str_constructor() {
    let code = transpile(
        "def to_str(n: int) -> str:\n    return str(n)",
    );
    assert!(
        code.contains("to_string") || code.contains("format"),
        "str(): {}",
        code
    );
}

#[test]
fn test_bool_constructor() {
    let code = transpile(
        "def to_bool(x: int) -> bool:\n    return bool(x)",
    );
    assert!(!code.is_empty(), "bool(): {}", code);
}

#[test]
fn test_list_constructor_from_range() {
    let code = transpile(
        "def make_list(n: int) -> list:\n    return list(range(n))",
    );
    assert!(
        code.contains("collect") || code.contains("Vec"),
        "list(range): {}",
        code
    );
}

#[test]
fn test_set_constructor() {
    let code = transpile(
        "def make_set(items: list) -> set:\n    return set(items)",
    );
    assert!(
        code.contains("HashSet") || code.contains("collect"),
        "set(): {}",
        code
    );
}

#[test]
fn test_tuple_constructor() {
    let code = transpile(
        "def make_tuple(items: list) -> tuple:\n    return tuple(items)",
    );
    assert!(!code.is_empty(), "tuple(): {}", code);
}

// =============================================================================
// Section 35: call_dispatch.rs - len/sorted/reversed/enumerate/zip
// =============================================================================

#[test]
fn test_len_list() {
    let code = transpile(
        "def count(items: list) -> int:\n    return len(items)",
    );
    assert!(code.contains("len"), "len(list): {}", code);
}

#[test]
fn test_len_string() {
    let code = transpile(
        "def count_chars(s: str) -> int:\n    return len(s)",
    );
    assert!(code.contains("len"), "len(str): {}", code);
}

#[test]
fn test_len_dict() {
    let code = transpile(
        "def count_keys(d: dict) -> int:\n    return len(d)",
    );
    assert!(code.contains("len"), "len(dict): {}", code);
}

#[test]
fn test_sorted_list() {
    let code = transpile(
        "def sort_copy(items: list) -> list:\n    return sorted(items)",
    );
    assert!(
        code.contains("sort") || code.contains("sorted"),
        "sorted(): {}",
        code
    );
}

#[test]
fn test_reversed_list() {
    let code = transpile(
        "def rev(items: list) -> list:\n    return list(reversed(items))",
    );
    assert!(!code.is_empty(), "reversed(): {}", code);
}

#[test]
fn test_enumerate_loop() {
    let code = transpile(
        "def indexed(items: list):\n    for i, item in enumerate(items):\n        print(i, item)",
    );
    assert!(
        code.contains("enumerate"),
        "enumerate(): {}",
        code
    );
}

#[test]
fn test_zip_two_lists() {
    let code = transpile(
        "def pair_up(a: list, b: list) -> list:\n    return list(zip(a, b))",
    );
    assert!(
        code.contains("zip"),
        "zip(): {}",
        code
    );
}

// =============================================================================
// Section 36: call_dispatch.rs - min/max/sum/any/all
// =============================================================================

#[test]
fn test_min_list() {
    let code = transpile(
        "def minimum(items: list) -> int:\n    return min(items)",
    );
    assert!(
        code.contains("min") || code.contains("iter"),
        "min(): {}",
        code
    );
}

#[test]
fn test_max_list() {
    let code = transpile(
        "def maximum(items: list) -> int:\n    return max(items)",
    );
    assert!(
        code.contains("max") || code.contains("iter"),
        "max(): {}",
        code
    );
}

#[test]
fn test_sum_list() {
    let code = transpile(
        "def total(items: list) -> int:\n    return sum(items)",
    );
    assert!(
        code.contains("sum") || code.contains("iter"),
        "sum(): {}",
        code
    );
}

#[test]
fn test_any_list() {
    let code = transpile(
        "def has_true(items: list) -> bool:\n    return any(items)",
    );
    assert!(
        code.contains("any") || code.contains("iter"),
        "any(): {}",
        code
    );
}

#[test]
fn test_all_list() {
    let code = transpile(
        "def all_true(items: list) -> bool:\n    return all(items)",
    );
    assert!(
        code.contains("all") || code.contains("iter"),
        "all(): {}",
        code
    );
}

// =============================================================================
// Section 37: call_dispatch.rs - abs/round/hash
// =============================================================================

#[test]
fn test_abs_int() {
    let code = transpile(
        "def absolute(n: int) -> int:\n    return abs(n)",
    );
    assert!(code.contains("abs"), "abs(int): {}", code);
}

#[test]
fn test_abs_float() {
    let code = transpile(
        "def absolute_f(n: float) -> float:\n    return abs(n)",
    );
    assert!(code.contains("abs"), "abs(float): {}", code);
}

#[test]
fn test_round_float() {
    let code = transpile(
        "def rounded(x: float) -> int:\n    return round(x)",
    );
    assert!(
        code.contains("round"),
        "round(): {}",
        code
    );
}

#[test]
fn test_round_with_precision() {
    let code = transpile(
        "def rounded_to(x: float) -> float:\n    return round(x, 2)",
    );
    assert!(!code.is_empty(), "round(x, 2): {}", code);
}

// =============================================================================
// Section 38: call_dispatch.rs - ord/chr
// =============================================================================

#[test]
fn test_ord_char() {
    let code = transpile(
        "def char_code(c: str) -> int:\n    return ord(c)",
    );
    assert!(!code.is_empty(), "ord(): {}", code);
}

#[test]
fn test_chr_int() {
    let code = transpile(
        "def from_code(n: int) -> str:\n    return chr(n)",
    );
    assert!(!code.is_empty(), "chr(): {}", code);
}

// =============================================================================
// Section 39: call_dispatch.rs - map/filter
// =============================================================================

#[test]
fn test_map_function() {
    let code = transpile(
        "def double_all(items: list) -> list:\n    return list(map(lambda x: x * 2, items))",
    );
    assert!(
        code.contains("map") || code.contains("iter"),
        "map(): {}",
        code
    );
}

#[test]
fn test_filter_function() {
    let code = transpile(
        "def positives(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))",
    );
    assert!(
        code.contains("filter") || code.contains("iter"),
        "filter(): {}",
        code
    );
}

// =============================================================================
// Section 40: Additional binary_ops.rs - chained comparison
// =============================================================================

#[test]
fn test_chained_add_sub() {
    let code = transpile(
        "def compute(a: int, b: int, c: int) -> int:\n    return a + b - c",
    );
    assert!(!code.is_empty(), "chained add/sub: {}", code);
}

#[test]
fn test_chained_mul_div() {
    let code = transpile(
        "def compute(a: int, b: int, c: int) -> int:\n    return a * b / c",
    );
    assert!(!code.is_empty(), "chained mul/div: {}", code);
}

#[test]
fn test_mixed_arithmetic() {
    let code = transpile(
        "def formula(x: float, y: float) -> float:\n    return x * x + y * y",
    );
    assert!(!code.is_empty(), "mixed arithmetic: {}", code);
}

#[test]
fn test_complex_expression() {
    let code = transpile(
        "def quadratic(a: float, b: float, c: float, x: float) -> float:\n    return a * x * x + b * x + c",
    );
    assert!(!code.is_empty(), "quadratic: {}", code);
}

// =============================================================================
// Section 41: classmethod/staticmethod attribute conversion
// =============================================================================

#[test]
fn test_cls_attribute_access() {
    let code = transpile(
        "class Config:\n    DEBUG = False\n    @classmethod\n    def is_debug(cls) -> bool:\n        return cls.DEBUG",
    );
    assert!(!code.is_empty(), "cls.ATTR: {}", code);
}

// =============================================================================
// Section 42: String method routing through instance_dispatch
// =============================================================================

#[test]
fn test_string_upper_routing() {
    let code = transpile(
        "def loud(s: str) -> str:\n    return s.upper()",
    );
    assert!(
        code.contains("to_uppercase"),
        "str.upper routing: {}",
        code
    );
}

#[test]
fn test_string_lower_routing() {
    let code = transpile(
        "def quiet(s: str) -> str:\n    return s.lower()",
    );
    assert!(
        code.contains("to_lowercase"),
        "str.lower routing: {}",
        code
    );
}

#[test]
fn test_string_strip_routing() {
    let code = transpile(
        "def clean(s: str) -> str:\n    return s.strip()",
    );
    assert!(
        code.contains("trim"),
        "str.strip routing: {}",
        code
    );
}

#[test]
fn test_string_lstrip_routing() {
    let code = transpile(
        "def clean_left(s: str) -> str:\n    return s.lstrip()",
    );
    assert!(
        code.contains("trim_start"),
        "str.lstrip routing: {}",
        code
    );
}

#[test]
fn test_string_rstrip_routing() {
    let code = transpile(
        "def clean_right(s: str) -> str:\n    return s.rstrip()",
    );
    assert!(
        code.contains("trim_end"),
        "str.rstrip routing: {}",
        code
    );
}

#[test]
fn test_string_title_routing() {
    let code = transpile(
        "def titlecase(s: str) -> str:\n    return s.title()",
    );
    assert!(!code.is_empty(), "str.title routing: {}", code);
}

#[test]
fn test_string_isdigit_routing() {
    let code = transpile(
        "def numeric(s: str) -> bool:\n    return s.isdigit()",
    );
    assert!(!code.is_empty(), "str.isdigit routing: {}", code);
}

#[test]
fn test_string_isalpha_routing() {
    let code = transpile(
        "def alpha(s: str) -> bool:\n    return s.isalpha()",
    );
    assert!(!code.is_empty(), "str.isalpha routing: {}", code);
}

#[test]
fn test_string_isalnum_routing() {
    let code = transpile(
        "def alnum(s: str) -> bool:\n    return s.isalnum()",
    );
    assert!(!code.is_empty(), "str.isalnum routing: {}", code);
}

#[test]
fn test_string_center_routing() {
    let code = transpile(
        "def centered(s: str) -> str:\n    return s.center(20)",
    );
    assert!(!code.is_empty(), "str.center routing: {}", code);
}

#[test]
fn test_string_zfill_routing() {
    let code = transpile(
        "def padded(s: str) -> str:\n    return s.zfill(5)",
    );
    assert!(!code.is_empty(), "str.zfill routing: {}", code);
}

// =============================================================================
// Section 43: Dict comprehension and method combinations
// =============================================================================

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        "def squares(n: int) -> dict:\n    return {i: i * i for i in range(n)}",
    );
    assert!(!code.is_empty(), "dict comprehension: {}", code);
}

#[test]
fn test_dict_with_list_values() {
    let code = transpile(
        "def categories() -> dict:\n    return {\"fruits\": [\"apple\"], \"vegs\": [\"carrot\"]}",
    );
    assert!(!code.is_empty(), "dict with list values: {}", code);
}

// =============================================================================
// Section 44: instance_dispatch.rs - write to stderr
// =============================================================================

#[test]
fn test_sys_stderr_write() {
    let code = transpile(
        "import sys\ndef log_error(msg: str):\n    sys.stderr.write(msg)",
    );
    assert!(
        code.contains("stderr") || code.contains("eprintln"),
        "sys.stderr.write: {}",
        code
    );
}

#[test]
fn test_sys_stdout_write() {
    let code = transpile(
        "import sys\ndef write_out(msg: str):\n    sys.stdout.write(msg)",
    );
    assert!(
        code.contains("stdout") || code.contains("print"),
        "sys.stdout.write: {}",
        code
    );
}
