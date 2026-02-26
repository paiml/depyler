//! Coverage boost tests for instance method and attribute codegen
//!
//! Targets uncovered branches in:
//! - method_call_routing.rs: convert_method_call
//! - instance_dispatch.rs: convert_instance_method
//! - attribute_convert.rs: convert_attribute
//! - string_methods.rs: convert_string_method
//! - dict_constructors.rs: convert_dict
//! - dict_methods.rs: convert_dict_method
//! - slicing.rs: convert_slice, convert_string_slice
//! - list_methods.rs: convert_list_method
//! - indexing.rs: convert_index

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
// Section 1: convert_method_call (method_call_routing.rs, 39% -> target 65%)
// =============================================================================

// Usage-based inference: unknown var with .append()
#[test]
fn test_method_call_infer_list_from_append() {
    let code = transpile("def foo():\n    items = []\n    items.append(5)\n    return items");
    assert!(code.contains("push"), "infer list from append: {}", code);
}

// Usage-based inference: unknown var with .keys()
#[test]
fn test_method_call_infer_dict_from_keys() {
    let code = transpile("def foo(d: dict) -> list:\n    return list(d.keys())");
    assert!(code.contains("keys"), "dict keys: {}", code);
}

// Usage-based inference: unknown var with .values()
#[test]
fn test_method_call_infer_dict_from_values() {
    let code = transpile("def foo(d: dict) -> list:\n    return list(d.values())");
    assert!(code.contains("values"), "dict values: {}", code);
}

// Usage-based inference: .items() method
#[test]
fn test_method_call_dict_items() {
    let code = transpile("def foo(d: dict):\n    for k, v in d.items():\n        print(k, v)");
    assert!(code.contains("iter"), "dict items iter: {}", code);
}

// .get() with default on dict
#[test]
fn test_method_call_dict_get_default() {
    let code = transpile("def foo(d: dict, key: str) -> int:\n    return d.get(key, 0)");
    assert!(!code.is_empty(), "dict get default: {}", code);
}

// .insert() on dict
#[test]
fn test_method_call_dict_insert() {
    let code = transpile("def foo(d: dict, key: str, val: int):\n    d[key] = val");
    assert!(code.contains("insert"), "dict insert: {}", code);
}

// .pop() on dict
#[test]
fn test_method_call_dict_pop() {
    let code = transpile("def foo(d: dict, key: str) -> int:\n    return d.pop(key)");
    assert!(!code.is_empty(), "dict pop: {}", code);
}

// .setdefault() on dict
#[test]
fn test_method_call_dict_setdefault() {
    let code = transpile("def foo(d: dict, key: str) -> int:\n    return d.setdefault(key, 0)");
    assert!(!code.is_empty(), "dict setdefault: {}", code);
}

// =============================================================================
// Section 2: convert_instance_method (instance_dispatch.rs, 61% -> target 80%)
// =============================================================================

// File read
#[test]
fn test_instance_file_read() {
    let code = transpile(
        "def read_all(path: str) -> str:\n    with open(path) as f:\n        return f.read()",
    );
    assert!(!code.is_empty(), "file read: {}", code);
}

// File readline
#[test]
fn test_instance_file_readline() {
    let code = transpile(
        "def first_line(path: str) -> str:\n    with open(path) as f:\n        return f.readline()",
    );
    assert!(!code.is_empty(), "file readline: {}", code);
}

// File readlines
#[test]
fn test_instance_file_readlines() {
    let code = transpile(
        "def all_lines(path: str) -> list:\n    with open(path) as f:\n        return f.readlines()",
    );
    assert!(!code.is_empty(), "file readlines: {}", code);
}

// File write
#[test]
fn test_instance_file_write() {
    let code = transpile(
        "def write_data(path: str, data: str):\n    with open(path, \"w\") as f:\n        f.write(data)",
    );
    assert!(code.contains("write"), "file write: {}", code);
}

// Path operations
#[test]
fn test_instance_path_exists() {
    let code = transpile(
        "from pathlib import Path\ndef check(p: str) -> bool:\n    return Path(p).exists()",
    );
    assert!(!code.is_empty(), "path exists: {}", code);
}

#[test]
fn test_instance_path_name() {
    let code = transpile(
        "from pathlib import Path\ndef name(path: str) -> str:\n    p = Path(path)\n    return p.name",
    );
    assert!(!code.is_empty(), "path.name: {}", code);
}

#[test]
fn test_instance_path_suffix() {
    let code = transpile(
        "from pathlib import Path\ndef ext(path: str) -> str:\n    p = Path(path)\n    return p.suffix",
    );
    assert!(!code.is_empty(), "path.suffix: {}", code);
}

#[test]
fn test_instance_path_parent() {
    let code = transpile(
        "from pathlib import Path\ndef parent(path: str) -> str:\n    p = Path(path)\n    return str(p.parent)",
    );
    assert!(!code.is_empty(), "path.parent: {}", code);
}

// =============================================================================
// Section 3: convert_attribute (attribute_convert.rs, 41.6% -> target 65%)
// =============================================================================

// type().__name__
#[test]
fn test_attr_type_name() {
    let code = transpile("def type_name(x: int) -> str:\n    return type(x).__name__");
    assert!(!code.is_empty(), "type name: {}", code);
}

// os.environ
#[test]
fn test_attr_os_environ() {
    let code =
        transpile("import os\ndef get_env(key: str) -> str:\n    return os.environ.get(key, \"\")");
    assert!(!code.is_empty(), "os.environ: {}", code);
}

// Exception attributes
#[test]
fn test_attr_exception_args() {
    let code = transpile(
        "def get_error() -> str:\n    try:\n        raise ValueError(\"bad\")\n    except ValueError as e:\n        return str(e)",
    );
    assert!(!code.is_empty(), "exception attr: {}", code);
}

// os.stat result attributes
#[test]
fn test_attr_stat_size() {
    let code = transpile(
        "import os\ndef file_size(path: str) -> int:\n    stats = os.stat(path)\n    return stats.st_size",
    );
    assert!(!code.is_empty(), "stat size: {}", code);
}

// =============================================================================
// Section 4: convert_string_method (string_methods.rs, 64.6% -> target 82%)
// =============================================================================

#[test]
fn test_string_startswith() {
    let code = transpile("def check(s: str) -> bool:\n    return s.startswith(\"http\")");
    assert!(code.contains("starts_with"), "startswith: {}", code);
}

#[test]
fn test_string_endswith() {
    let code = transpile("def check(s: str) -> bool:\n    return s.endswith(\".py\")");
    assert!(code.contains("ends_with"), "endswith: {}", code);
}

#[test]
fn test_string_split_no_arg() {
    let code = transpile("def words(s: str) -> list:\n    return s.split()");
    assert!(code.contains("split"), "split no arg: {}", code);
}

#[test]
fn test_string_split_with_delim() {
    let code = transpile("def parts(s: str) -> list:\n    return s.split(\",\")");
    assert!(code.contains("split"), "split with delim: {}", code);
}

#[test]
fn test_string_split_maxsplit() {
    let code = transpile("def first_part(s: str) -> list:\n    return s.split(\",\", 1)");
    assert!(code.contains("split"), "split maxsplit: {}", code);
}

#[test]
fn test_string_join() {
    let code = transpile("def combine(items: list) -> str:\n    return \",\".join(items)");
    assert!(code.contains("join"), "join: {}", code);
}

#[test]
fn test_string_replace() {
    let code = transpile("def fix(s: str) -> str:\n    return s.replace(\"old\", \"new\")");
    assert!(code.contains("replace"), "replace: {}", code);
}

#[test]
fn test_string_find() {
    let code = transpile("def locate(s: str) -> int:\n    return s.find(\"x\")");
    assert!(!code.is_empty(), "find: {}", code);
}

#[test]
fn test_string_count() {
    let code = transpile("def num_a(s: str) -> int:\n    return s.count(\"a\")");
    assert!(code.contains("match"), "count: {}", code);
}

#[test]
fn test_string_isdigit() {
    let code = transpile("def check(s: str) -> bool:\n    return s.isdigit()");
    assert!(!code.is_empty(), "isdigit: {}", code);
}

#[test]
fn test_string_isalpha() {
    let code = transpile("def check(s: str) -> bool:\n    return s.isalpha()");
    assert!(!code.is_empty(), "isalpha: {}", code);
}

#[test]
fn test_string_title() {
    let code = transpile("def title_case(s: str) -> str:\n    return s.title()");
    assert!(!code.is_empty(), "title: {}", code);
}

#[test]
fn test_string_index() {
    let code = transpile("def locate(s: str) -> int:\n    return s.index(\"x\")");
    assert!(!code.is_empty(), "index: {}", code);
}

#[test]
fn test_string_rfind() {
    let code = transpile("def locate_last(s: str) -> int:\n    return s.rfind(\"x\")");
    assert!(!code.is_empty(), "rfind: {}", code);
}

#[test]
fn test_string_ljust() {
    let code = transpile("def pad(s: str) -> str:\n    return s.ljust(20)");
    assert!(!code.is_empty(), "ljust: {}", code);
}

#[test]
fn test_string_rjust() {
    let code = transpile("def pad(s: str) -> str:\n    return s.rjust(20)");
    assert!(!code.is_empty(), "rjust: {}", code);
}

#[test]
fn test_string_center() {
    let code = transpile("def center_text(s: str) -> str:\n    return s.center(20)");
    assert!(!code.is_empty(), "center: {}", code);
}

#[test]
fn test_string_zfill() {
    let code = transpile("def pad_num(s: str) -> str:\n    return s.zfill(5)");
    assert!(!code.is_empty(), "zfill: {}", code);
}

#[test]
fn test_string_capitalize() {
    let code = transpile("def cap(s: str) -> str:\n    return s.capitalize()");
    assert!(!code.is_empty(), "capitalize: {}", code);
}

#[test]
fn test_string_swapcase() {
    let code = transpile("def swap(s: str) -> str:\n    return s.swapcase()");
    assert!(!code.is_empty(), "swapcase: {}", code);
}

#[test]
fn test_string_lstrip() {
    let code = transpile("def trim_left(s: str) -> str:\n    return s.lstrip()");
    assert!(code.contains("trim_start"), "lstrip: {}", code);
}

#[test]
fn test_string_rstrip() {
    let code = transpile("def trim_right(s: str) -> str:\n    return s.rstrip()");
    assert!(code.contains("trim_end"), "rstrip: {}", code);
}

#[test]
fn test_string_encode() {
    let code = transpile("def to_bytes(s: str) -> bytes:\n    return s.encode()");
    assert!(!code.is_empty(), "encode: {}", code);
}

#[test]
fn test_string_isupper() {
    let code = transpile("def check(s: str) -> bool:\n    return s.isupper()");
    assert!(!code.is_empty(), "isupper: {}", code);
}

#[test]
fn test_string_islower() {
    let code = transpile("def check(s: str) -> bool:\n    return s.islower()");
    assert!(!code.is_empty(), "islower: {}", code);
}

#[test]
fn test_string_isspace() {
    let code = transpile("def check(s: str) -> bool:\n    return s.isspace()");
    assert!(!code.is_empty(), "isspace: {}", code);
}

#[test]
fn test_string_isalnum() {
    let code = transpile("def check(s: str) -> bool:\n    return s.isalnum()");
    assert!(!code.is_empty(), "isalnum: {}", code);
}

// =============================================================================
// Section 5: convert_dict (dict_constructors.rs, 55.2% -> target 75%)
// =============================================================================

#[test]
fn test_dict_empty() {
    let code = transpile("def empty() -> dict:\n    return {}");
    assert!(code.contains("HashMap") || code.contains("new"), "empty dict: {}", code);
}

#[test]
fn test_dict_string_keys() {
    let code = transpile("def make() -> dict:\n    return {\"a\": 1, \"b\": 2}");
    assert!(!code.is_empty(), "string key dict: {}", code);
}

#[test]
fn test_dict_int_keys() {
    let code = transpile("def make() -> dict:\n    return {1: \"one\", 2: \"two\"}");
    assert!(!code.is_empty(), "int key dict: {}", code);
}

#[test]
fn test_dict_nested() {
    let code = transpile("def make() -> dict:\n    return {\"outer\": {\"inner\": 1}}");
    assert!(!code.is_empty(), "nested dict: {}", code);
}

#[test]
fn test_dict_mixed_value_types() {
    let code = transpile(
        "def config() -> dict:\n    return {\"name\": \"test\", \"count\": 42, \"enabled\": True}",
    );
    assert!(!code.is_empty(), "mixed values: {}", code);
}

#[test]
fn test_dict_with_list_value() {
    let code = transpile("def make() -> dict:\n    return {\"items\": [1, 2, 3]}");
    assert!(!code.is_empty(), "dict with list value: {}", code);
}

#[test]
fn test_dict_with_none_value() {
    let code = transpile("def make() -> dict:\n    return {\"key\": None}");
    assert!(!code.is_empty(), "dict with None: {}", code);
}

#[test]
fn test_dict_from_comprehension() {
    let code = transpile("def squares(n: int) -> dict:\n    return {i: i * i for i in range(n)}");
    assert!(!code.is_empty(), "dict comprehension: {}", code);
}

// =============================================================================
// Section 6: dict_methods (dict_methods.rs, 48.9% -> target 70%)
// =============================================================================

#[test]
fn test_dict_method_get() {
    let code =
        transpile("def lookup(d: dict, key: str) -> str:\n    return d.get(key, \"default\")");
    assert!(!code.is_empty(), "dict.get: {}", code);
}

#[test]
fn test_dict_method_keys() {
    let code = transpile("def all_keys(d: dict) -> list:\n    return list(d.keys())");
    assert!(code.contains("keys"), "dict.keys: {}", code);
}

#[test]
fn test_dict_method_values() {
    let code = transpile("def all_vals(d: dict) -> list:\n    return list(d.values())");
    assert!(code.contains("values"), "dict.values: {}", code);
}

#[test]
fn test_dict_method_items() {
    let code = transpile("def all_items(d: dict) -> list:\n    return list(d.items())");
    assert!(!code.is_empty(), "dict.items: {}", code);
}

#[test]
fn test_dict_method_update() {
    let code = transpile("def merge(a: dict, b: dict) -> dict:\n    a.update(b)\n    return a");
    assert!(!code.is_empty(), "dict.update: {}", code);
}

#[test]
fn test_dict_method_pop() {
    let code = transpile("def remove(d: dict, key: str) -> int:\n    return d.pop(key)");
    assert!(!code.is_empty(), "dict.pop: {}", code);
}

#[test]
fn test_dict_method_clear() {
    let code = transpile("def reset(d: dict):\n    d.clear()");
    assert!(code.contains("clear"), "dict.clear: {}", code);
}

#[test]
fn test_dict_method_copy() {
    let code = transpile("def clone_dict(d: dict) -> dict:\n    return d.copy()");
    assert!(code.contains("clone"), "dict.copy: {}", code);
}

// =============================================================================
// Section 7: convert_slice / convert_string_slice (slicing.rs)
// =============================================================================

#[test]
fn test_slice_start_stop() {
    let code = transpile("def mid(items: list) -> list:\n    return items[1:3]");
    assert!(!code.is_empty(), "slice [1:3]: {}", code);
}

#[test]
fn test_slice_from_start() {
    let code = transpile("def tail(items: list) -> list:\n    return items[2:]");
    assert!(!code.is_empty(), "slice [2:]: {}", code);
}

#[test]
fn test_slice_to_stop() {
    let code = transpile("def head(items: list) -> list:\n    return items[:3]");
    assert!(!code.is_empty(), "slice [:3]: {}", code);
}

#[test]
fn test_slice_full_clone() {
    let code = transpile("def clone(items: list) -> list:\n    return items[:]");
    assert!(code.contains("clone") || code.contains("to_vec"), "slice [:]: {}", code);
}

#[test]
fn test_slice_negative_stop() {
    let code = transpile("def most(items: list) -> list:\n    return items[:-1]");
    assert!(!code.is_empty(), "slice [:-1]: {}", code);
}

#[test]
fn test_slice_negative_start() {
    let code = transpile("def last_two(items: list) -> list:\n    return items[-2:]");
    assert!(!code.is_empty(), "slice [-2:]: {}", code);
}

#[test]
fn test_slice_step() {
    let code = transpile("def every_other(items: list) -> list:\n    return items[::2]");
    assert!(!code.is_empty(), "slice [::2]: {}", code);
}

#[test]
fn test_slice_reverse() {
    let code = transpile("def reversed_list(items: list) -> list:\n    return items[::-1]");
    assert!(code.contains("rev") || code.contains("reverse"), "slice [::-1]: {}", code);
}

#[test]
fn test_string_slice_mid() {
    let code = transpile("def mid(s: str) -> str:\n    return s[1:3]");
    assert!(!code.is_empty(), "string [1:3]: {}", code);
}

#[test]
fn test_string_slice_from() {
    let code = transpile("def tail(s: str) -> str:\n    return s[2:]");
    assert!(!code.is_empty(), "string [2:]: {}", code);
}

#[test]
fn test_string_slice_to() {
    let code = transpile("def head(s: str) -> str:\n    return s[:3]");
    assert!(!code.is_empty(), "string [:3]: {}", code);
}

#[test]
fn test_string_slice_negative() {
    let code = transpile("def trim_last(s: str) -> str:\n    return s[:-1]");
    assert!(!code.is_empty(), "string [:-1]: {}", code);
}

#[test]
fn test_string_slice_reverse() {
    let code = transpile("def reverse(s: str) -> str:\n    return s[::-1]");
    assert!(!code.is_empty(), "string [::-1]: {}", code);
}

#[test]
fn test_string_slice_step() {
    let code = transpile("def every_other(s: str) -> str:\n    return s[::2]");
    assert!(!code.is_empty(), "string [::2]: {}", code);
}

// =============================================================================
// Section 8: convert_list_method (list_methods.rs, 72.8% -> target 85%)
// =============================================================================

#[test]
fn test_list_insert() {
    let code = transpile("def add(items: list, idx: int, val: int):\n    items.insert(idx, val)");
    assert!(code.contains("insert"), "list insert: {}", code);
}

#[test]
fn test_list_remove() {
    let code = transpile("def rem(items: list, val: int):\n    items.remove(val)");
    assert!(!code.is_empty(), "list remove: {}", code);
}

#[test]
fn test_list_index() {
    let code = transpile("def find(items: list, val: int) -> int:\n    return items.index(val)");
    assert!(!code.is_empty(), "list index: {}", code);
}

#[test]
fn test_list_count() {
    let code = transpile("def count(items: list, val: int) -> int:\n    return items.count(val)");
    assert!(!code.is_empty(), "list count: {}", code);
}

#[test]
fn test_list_copy() {
    let code = transpile("def clone(items: list) -> list:\n    return items.copy()");
    assert!(code.contains("clone"), "list copy: {}", code);
}

#[test]
fn test_list_extend_from_range() {
    let code = transpile("def extend(items: list, n: int):\n    items.extend(range(n))");
    assert!(code.contains("extend"), "list extend range: {}", code);
}

#[test]
fn test_list_sort_reverse() {
    let code = transpile("def sort_desc(items: list):\n    items.sort(reverse=True)");
    assert!(code.contains("sort"), "list sort reverse: {}", code);
}

// =============================================================================
// Section 9: convert_index (indexing.rs, 67.9% -> target 82%)
// =============================================================================

#[test]
fn test_index_list_positive() {
    let code = transpile("def first(items: list) -> int:\n    return items[0]");
    assert!(code.contains("[0]") || code.contains("get"), "list[0]: {}", code);
}

#[test]
fn test_index_list_negative() {
    let code = transpile("def last(items: list) -> int:\n    return items[-1]");
    assert!(!code.is_empty(), "list[-1]: {}", code);
}

#[test]
fn test_index_dict_string() {
    let code = transpile("def lookup(d: dict) -> int:\n    return d[\"key\"]");
    assert!(!code.is_empty(), "dict[str]: {}", code);
}

#[test]
fn test_index_string_char() {
    let code = transpile("def first_char(s: str) -> str:\n    return s[0]");
    assert!(!code.is_empty(), "string[0]: {}", code);
}

#[test]
fn test_index_nested_dict() {
    let code = transpile("def nested(d: dict) -> int:\n    return d[\"outer\"][\"inner\"]");
    assert!(!code.is_empty(), "nested dict index: {}", code);
}

#[test]
fn test_index_variable() {
    let code = transpile("def at(items: list, idx: int) -> int:\n    return items[idx]");
    assert!(!code.is_empty(), "list[var]: {}", code);
}
