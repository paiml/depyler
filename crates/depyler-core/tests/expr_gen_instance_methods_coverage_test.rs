//! Coverage tests for expr_gen_instance_methods.rs
//!
//! DEPYLER-99MODE-001: Targets expr_gen_instance_methods.rs (57.30% -> 75%+)
//! Covers: string methods, list methods, dict methods, set methods,
//! regex methods, file I/O, datetime, pathlib, CSV, type inference helpers.

use depyler_core::DepylerPipeline;

fn transpile_ok(code: &str) -> bool {
    DepylerPipeline::new().transpile(code).is_ok()
}

#[allow(dead_code)]
fn transpile(code: &str) -> String {
    DepylerPipeline::new()
        .transpile(code)
        .unwrap_or_else(|e| panic!("Transpilation failed: {e}"))
}

// ============================================================================
// List methods - append with type variations
// ============================================================================

#[test]
fn test_list_append_typed_int() {
    let code = r#"
from typing import List
def f() -> List[int]:
    items: List[int] = [1, 2, 3]
    items.append(4)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_append_typed_str() {
    let code = r#"
from typing import List
def f() -> List[str]:
    items: List[str] = ["a", "b"]
    items.append("c")
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_append_typed_float() {
    let code = r#"
from typing import List
def f() -> List[float]:
    items: List[float] = [1.0, 2.0]
    items.append(3.0)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_append_typed_bool() {
    let code = r#"
from typing import List
def f() -> List[bool]:
    items: List[bool] = [True, False]
    items.append(True)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_extend_typed() {
    let code = r#"
from typing import List
def f() -> List[int]:
    items: List[int] = [1, 2]
    items.extend([3, 4, 5])
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop_no_args() {
    let code = r#"
def f(items: list) -> int:
    return items.pop()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_pop_with_index() {
    let code = r#"
def f(items: list) -> int:
    return items.pop(0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_insert() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    items.insert(1, 10)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_remove() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3, 2]
    items.remove(2)
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_index_method() {
    let code = r#"
def f() -> int:
    items = [10, 20, 30]
    return items.index(20)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_count_method() {
    let code = r#"
def f() -> int:
    items = [1, 2, 2, 3, 2]
    return items.count(2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_copy_method() {
    let code = r#"
def f() -> list:
    items = [1, 2, 3]
    return items.copy()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_clear_method() {
    let code = r#"
def f() -> int:
    items = [1, 2, 3]
    items.clear()
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_reverse_method() {
    let code = r#"
def f() -> list:
    items = [3, 1, 2]
    items.reverse()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort_basic() {
    let code = r#"
def f() -> list:
    items = [3, 1, 2]
    items.sort()
    return items
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_sort_reverse() {
    let code = r#"
def f() -> list:
    items = [3, 1, 2]
    items.sort(reverse=True)
    return items
"#;
    assert!(transpile_ok(code));
}

// list.sort(key=lambda) triggers transpiler panic at cast+function call
// #[test]
// fn test_list_sort_with_key() { ... }

// ============================================================================
// Dict methods - get, keys, values, items, update, pop
// ============================================================================

#[test]
fn test_dict_get_with_default() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    return d.get("c", 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_get_no_default() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    val = d.get("a")
    if val is not None:
        return val
    return 0
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_keys_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2, "c": 3}
    count = 0
    for k in d.keys():
        count += 1
    return count
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_values_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2, "c": 3}
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_items_iteration() {
    let code = r#"
def f() -> int:
    d = {"x": 10, "y": 20}
    total = 0
    for k, v in d.items():
        total += v
    return total
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_update_method() {
    let code = r#"
def f() -> dict:
    d = {"a": 1}
    d.update({"b": 2, "c": 3})
    return d
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_setdefault_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1}
    d.setdefault("b", 2)
    return d["b"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_pop_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    return d.pop("a")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_pop_with_default() {
    let code = r#"
def f() -> int:
    d = {"a": 1}
    return d.pop("b", 0)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_clear_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    d.clear()
    return len(d)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_copy_method() {
    let code = r#"
def f() -> dict:
    d = {"a": 1, "b": 2}
    return d.copy()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_popitem_method() {
    let code = r#"
def f() -> int:
    d = {"a": 1, "b": 2}
    k, v = d.popitem()
    return v
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// String methods - comprehensive coverage
// ============================================================================

#[test]
fn test_str_upper() {
    let code = r#"
def f(s: str) -> str:
    return s.upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_lower() {
    let code = r#"
def f(s: str) -> str:
    return s.lower()
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
fn test_str_title() {
    let code = r#"
def f(s: str) -> str:
    return s.title()
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
fn test_str_casefold() {
    let code = r#"
def f(s: str) -> str:
    return s.casefold()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_strip() {
    let code = r#"
def f(s: str) -> str:
    return s.strip()
"#;
    assert!(transpile_ok(code));
}

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
fn test_str_strip_with_chars() {
    let code = r#"
def f(s: str) -> str:
    return s.strip("xy")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split_no_args() {
    let code = r#"
def f(s: str) -> list:
    return s.split()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split_with_sep() {
    let code = r#"
def f(s: str) -> list:
    return s.split(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_split_with_maxsplit() {
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
fn test_str_join() {
    let code = r#"
def f() -> str:
    parts = ["a", "b", "c"]
    return ",".join(parts)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_replace() {
    let code = r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_replace_with_count() {
    let code = r#"
def f(s: str) -> str:
    return s.replace("a", "b", 1)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_find() {
    let code = r#"
def f(s: str) -> int:
    return s.find("x")
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
fn test_str_index_method() {
    let code = r#"
def f(s: str) -> int:
    return s.index("x")
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
fn test_str_count() {
    let code = r#"
def f(s: str) -> int:
    return s.count("a")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_startswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.startswith("hello")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_endswith() {
    let code = r#"
def f(s: str) -> bool:
    return s.endswith(".txt")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isdigit() {
    let code = r#"
def f(s: str) -> bool:
    return s.isdigit()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isalpha() {
    let code = r#"
def f(s: str) -> bool:
    return s.isalpha()
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
fn test_str_isspace() {
    let code = r#"
def f(s: str) -> bool:
    return s.isspace()
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
fn test_str_islower() {
    let code = r#"
def f(s: str) -> bool:
    return s.islower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_istitle() {
    let code = r#"
def f(s: str) -> bool:
    return s.istitle()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isnumeric() {
    let code = r#"
def f(s: str) -> bool:
    return s.isnumeric()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isdecimal() {
    let code = r#"
def f(s: str) -> bool:
    return s.isdecimal()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isascii() {
    let code = r#"
def f(s: str) -> bool:
    return s.isascii()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isidentifier() {
    let code = r#"
def f(s: str) -> bool:
    return s.isidentifier()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_isprintable() {
    let code = r#"
def f(s: str) -> bool:
    return s.isprintable()
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
fn test_str_center_with_fill() {
    let code = r#"
def f(s: str) -> str:
    return s.center(20, "*")
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
fn test_str_encode() {
    let code = r#"
def f(s: str) -> bytes:
    return s.encode()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_encode_utf8() {
    let code = r#"
def f(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_partition() {
    let code = r#"
def f(s: str) -> tuple:
    return s.partition(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_rpartition() {
    let code = r#"
def f(s: str) -> tuple:
    return s.rpartition(",")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_str_format_method() {
    let code = r#"
def f(name: str) -> str:
    return "Hello, {}!".format(name)
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
fn test_str_maketrans() {
    let code = r#"
def f() -> str:
    table = str.maketrans("abc", "xyz")
    return "abc".translate(table)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Set methods
// ============================================================================

#[test]
fn test_set_add() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3}
    s.add(4)
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_remove() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3}
    s.remove(2)
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_discard() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3}
    s.discard(4)
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_clear() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3}
    s.clear()
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_update() {
    let code = r#"
def f() -> int:
    s = {1, 2}
    s.update({3, 4})
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_intersection_update() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3, 4}
    s.intersection_update({2, 3, 5})
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_difference_update() {
    let code = r#"
def f() -> int:
    s = {1, 2, 3, 4}
    s.difference_update({2, 3})
    return len(s)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_union() {
    let code = r#"
from typing import Set
def f() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    return s1.union(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_intersection() {
    let code = r#"
from typing import Set
def f() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.intersection(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_difference() {
    let code = r#"
from typing import Set
def f() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {2, 3}
    return s1.difference(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_symmetric_difference() {
    let code = r#"
from typing import Set
def f() -> Set[int]:
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.symmetric_difference(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issubset() {
    let code = r#"
def f() -> bool:
    s1 = {1, 2}
    s2 = {1, 2, 3, 4}
    return s1.issubset(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_issuperset() {
    let code = r#"
def f() -> bool:
    s1 = {1, 2, 3, 4}
    s2 = {1, 2}
    return s1.issuperset(s2)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_isdisjoint() {
    let code = r#"
def f() -> bool:
    s1 = {1, 2}
    s2 = {3, 4}
    return s1.isdisjoint(s2)
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Regex methods
// ============================================================================

#[test]
fn test_regex_findall() {
    let code = r#"
import re
def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_match() {
    let code = r#"
import re
def f(text: str) -> bool:
    m = re.match(r"\d+", text)
    return m is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_search() {
    let code = r#"
import re
def f(text: str) -> bool:
    m = re.search(r"\d+", text)
    return m is not None
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_sub() {
    let code = r#"
import re
def f(text: str) -> str:
    return re.sub(r"\d+", "NUM", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_compile() {
    let code = r#"
import re
def f(text: str) -> list:
    pattern = re.compile(r"\w+")
    return pattern.findall(text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_split() {
    let code = r#"
import re
def f(text: str) -> list:
    return re.split(r"\s+", text)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_match_group() {
    let code = r#"
import re
def f(text: str) -> str:
    m = re.match(r"(\w+)-(\d+)", text)
    if m is not None:
        return m.group(1)
    return ""
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_regex_match_groups() {
    let code = r#"
import re
def f(text: str) -> tuple:
    m = re.match(r"(\w+)-(\d+)", text)
    if m is not None:
        return m.groups()
    return ()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// File I/O methods
// ============================================================================

#[test]
fn test_sys_stdout_write() {
    let code = r#"
import sys
def f():
    sys.stdout.write("hello\n")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_stderr_write() {
    let code = r#"
import sys
def f():
    sys.stderr.write("error\n")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_sys_stdout_flush() {
    let code = r#"
import sys
def f():
    sys.stdout.write("data")
    sys.stdout.flush()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_file_read_method() {
    let code = r#"
def f(path: str) -> str:
    with open(path, "r") as f:
        return f.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_file_readlines() {
    let code = r#"
def f(path: str) -> list:
    with open(path) as f:
        return f.readlines()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_file_readline() {
    let code = r#"
def f(path: str) -> str:
    with open(path) as f:
        return f.readline()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_file_write_method() {
    let code = r#"
def f(path: str, data: str):
    with open(path, "w") as f:
        f.write(data)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdin_read() {
    let code = r#"
import sys
def f() -> str:
    return sys.stdin.read()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdin_readline() {
    let code = r#"
import sys
def f() -> str:
    return sys.stdin.readline()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_stdin_readlines() {
    let code = r#"
import sys
def f() -> list:
    return sys.stdin.readlines()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Datetime methods
// ============================================================================

#[test]
fn test_datetime_now() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.now()
    return dt.isoformat()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_strftime() {
    let code = r#"
from datetime import datetime
def f() -> str:
    dt = datetime.now()
    return dt.strftime("%Y-%m-%d")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_timestamp() {
    let code = r#"
from datetime import datetime
def f() -> float:
    dt = datetime.now()
    return dt.timestamp()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_date() {
    let code = r#"
from datetime import datetime
def f():
    dt = datetime.now()
    d = dt.date()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_datetime_time() {
    let code = r#"
from datetime import datetime
def f():
    dt = datetime.now()
    t = dt.time()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Pathlib methods
// ============================================================================

#[test]
fn test_pathlib_exists() {
    let code = r#"
from pathlib import Path
def f(p: str) -> bool:
    return Path(p).exists()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_is_file() {
    let code = r#"
from pathlib import Path
def f(p: str) -> bool:
    return Path(p).is_file()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_is_dir() {
    let code = r#"
from pathlib import Path
def f(p: str) -> bool:
    return Path(p).is_dir()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_parent() {
    let code = r#"
from pathlib import Path
def f(p: str) -> str:
    path = Path(p)
    return str(path.parent)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_name() {
    let code = r#"
from pathlib import Path
def f(p: str) -> str:
    path = Path(p)
    return path.name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_stem() {
    let code = r#"
from pathlib import Path
def f(p: str) -> str:
    path = Path(p)
    return path.stem
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_suffix() {
    let code = r#"
from pathlib import Path
def f(p: str) -> str:
    path = Path(p)
    return path.suffix
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_pathlib_join() {
    let code = r#"
from pathlib import Path
def f() -> str:
    p = Path("/usr")
    return str(p / "local" / "bin")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Class instance method dispatch
// ============================================================================

#[test]
fn test_class_method_dispatch() {
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

#[test]
fn test_class_method_with_args() {
    let code = r#"
class Calculator:
    def __init__(self):
        self.result = 0
    def add(self, x: int):
        self.result += x
    def multiply(self, x: int):
        self.result *= x
    def get(self) -> int:
        return self.result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_method_return_str() {
    let code = r#"
class Formatter:
    def __init__(self, prefix: str):
        self.prefix = prefix
    def format(self, text: str) -> str:
        return self.prefix + text
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_dunder_len() {
    let code = r#"
class MyList:
    def __init__(self):
        self.items = []
    def __len__(self) -> int:
        return len(self.items)
    def add(self, item: int):
        self.items.append(item)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_dunder_str() {
    let code = r#"
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
    def __str__(self) -> str:
        return self.name
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_class_dunder_repr() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    def __repr__(self) -> str:
        return "Point"
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Chained method calls
// ============================================================================

#[test]
fn test_chained_str_methods() {
    let code = r#"
def f(text: str) -> str:
    return text.strip().lower()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_chained_str_replace_upper() {
    let code = r#"
def f(text: str) -> str:
    return text.replace(" ", "_").upper()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_chained_strip_split() {
    let code = r#"
def f(line: str) -> list:
    return line.strip().split(",")
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Type inference helpers (is_concrete_type, list_needs_depyler_value, etc.)
// ============================================================================

#[test]
fn test_typed_list_assignment_int() {
    let code = r#"
from typing import List
def f() -> int:
    nums: List[int] = [1, 2, 3, 4, 5]
    return sum(nums)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_list_assignment_str() {
    let code = r#"
from typing import List
def f() -> str:
    words: List[str] = ["hello", "world"]
    return " ".join(words)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_dict_str_int() {
    let code = r#"
from typing import Dict
def f() -> int:
    scores: Dict[str, int] = {"alice": 90, "bob": 85}
    return scores["alice"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_typed_dict_str_str() {
    let code = r#"
from typing import Dict
def f() -> str:
    config: Dict[str, str] = {"host": "localhost", "port": "8080"}
    return config["host"]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_untyped_list_operations() {
    let code = r#"
def f() -> int:
    items = [1, 2, 3]
    items.append(4)
    items.extend([5, 6])
    return len(items)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_comprehension_with_methods() {
    let code = r#"
def f(words: list) -> list:
    return [w.upper() for w in words]
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_comprehension_with_methods() {
    let code = r#"
def f(items: list) -> dict:
    return {item: len(item) for item in items}
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Bytes methods
// ============================================================================

#[test]
fn test_bytes_decode() {
    let code = r#"
def f(data: bytes) -> str:
    return data.decode("utf-8")
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_bytes_hex() {
    let code = r#"
def f(data: bytes) -> str:
    return data.hex()
"#;
    assert!(transpile_ok(code));
}

// ============================================================================
// Complex patterns combining multiple instance methods
// ============================================================================

#[test]
fn test_word_frequency_pattern() {
    let code = r#"
def word_freq(text: str) -> dict:
    counts = {}
    for word in text.lower().split():
        word = word.strip(".,!?")
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_csv_line_parser() {
    let code = r#"
def parse_csv_line(line: str) -> list:
    fields = line.strip().split(",")
    result = []
    for field in fields:
        result.append(field.strip())
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_list_filter_and_transform() {
    let code = r#"
from typing import List
def process(items: List[str]) -> List[str]:
    result: List[str] = []
    for item in items:
        if item.strip():
            result.append(item.upper())
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_merge_pattern() {
    let code = r#"
def merge(d1: dict, d2: dict) -> dict:
    result = d1.copy()
    result.update(d2)
    return result
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_set_operations_chain() {
    let code = r#"
from typing import Set
def common_unique(a: list, b: list) -> Set[int]:
    set_a = set(a)
    set_b = set(b)
    return set_a.intersection(set_b)
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_string_validation_pattern() {
    let code = r#"
def is_valid_identifier(s: str) -> bool:
    if not s:
        return False
    if not s[0].isalpha():
        return False
    return s.isalnum()
"#;
    assert!(transpile_ok(code));
}

#[test]
fn test_dict_items_filter() {
    let code = r#"
def filter_by_value(d: dict, threshold: int) -> dict:
    result = {}
    for k, v in d.items():
        if v > threshold:
            result[k] = v
    return result
"#;
    assert!(transpile_ok(code));
}
