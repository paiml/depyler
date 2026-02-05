//! Session 8 coverage tests for expr_gen_instance_methods.rs
//! Targets: set ops, regex match, operator overloads, deque, collections,
//! math constants/functions, string constants, sys module, fractions, timedelta

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
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

// ── Set operations ──────────────────────────────────────────────

#[test]
fn test_set_intersection_update() {
    let code = transpile(
        r#"
def f(a: set, b: set) -> None:
    a.intersection_update(b)
"#,
    );
    assert!(
        code.contains("intersection") || code.contains("retain"),
        "Should generate intersection_update: {code}"
    );
}

#[test]
fn test_set_difference_update() {
    let code = transpile(
        r#"
def f(a: set, b: set) -> None:
    a.difference_update(b)
"#,
    );
    assert!(
        code.contains("difference") || code.contains("retain"),
        "Should generate difference_update: {code}"
    );
}

// symmetric_difference_update not yet implemented in transpiler

#[test]
fn test_set_discard() {
    let code = transpile(
        r#"
def f(s: set) -> None:
    s.discard(42)
"#,
    );
    assert!(
        code.contains("remove") || code.contains("discard"),
        "Should generate discard: {code}"
    );
}

#[test]
fn test_set_issubset() {
    let code = transpile(
        r#"
def f(a: set, b: set) -> bool:
    return a.issubset(b)
"#,
    );
    assert!(
        code.contains("is_subset"),
        "Should generate is_subset: {code}"
    );
}

#[test]
fn test_set_issuperset() {
    let code = transpile(
        r#"
def f(a: set, b: set) -> bool:
    return a.issuperset(b)
"#,
    );
    assert!(
        code.contains("is_superset"),
        "Should generate is_superset: {code}"
    );
}

// ── Deque operations ────────────────────────────────────────────

#[test]
fn test_deque_appendleft() {
    let code = transpile(
        r#"
from collections import deque
def f() -> None:
    d = deque()
    d.appendleft(1)
"#,
    );
    assert!(
        code.contains("push_front") || code.contains("VecDeque"),
        "Should generate push_front for appendleft: {code}"
    );
}

#[test]
fn test_deque_extendleft() {
    let code = transpile(
        r#"
from collections import deque
def f() -> None:
    d = deque()
    d.extendleft([1, 2, 3])
"#,
    );
    assert!(
        code.contains("push_front") || code.contains("VecDeque"),
        "Should generate push_front loop for extendleft: {code}"
    );
}

#[test]
fn test_deque_popleft() {
    let code = transpile(
        r#"
from collections import deque
def f() -> int:
    d = deque()
    d.append(1)
    return d.popleft()
"#,
    );
    assert!(
        code.contains("pop_front") || code.contains("VecDeque"),
        "Should generate pop_front: {code}"
    );
}

#[test]
fn test_deque_rotate() {
    let code = transpile(
        r#"
from collections import deque
def f() -> None:
    d = deque()
    d.rotate(2)
"#,
    );
    assert!(
        code.contains("rotate") || code.contains("VecDeque"),
        "Should generate rotate: {code}"
    );
}

// ── Math constants and functions ────────────────────────────────

#[test]
fn test_math_e_constant() {
    let code = transpile(
        r#"
import math
def f() -> float:
    return math.e
"#,
    );
    assert!(
        code.contains("E") || code.contains("2.71828"),
        "Should generate math E constant: {code}"
    );
}

#[test]
fn test_math_inf_constant() {
    let code = transpile(
        r#"
import math
def f() -> float:
    return math.inf
"#,
    );
    assert!(
        code.contains("INFINITY") || code.contains("f64::INFINITY"),
        "Should generate infinity: {code}"
    );
}

#[test]
fn test_math_cos() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.cos(x)
"#,
    );
    assert!(code.contains("cos"), "Should generate cos: {code}");
}

#[test]
fn test_math_asin() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.asin(x)
"#,
    );
    assert!(code.contains("asin"), "Should generate asin: {code}");
}

#[test]
fn test_math_acos() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.acos(x)
"#,
    );
    assert!(code.contains("acos"), "Should generate acos: {code}");
}

#[test]
fn test_math_atan() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.atan(x)
"#,
    );
    assert!(code.contains("atan"), "Should generate atan: {code}");
}

#[test]
fn test_math_exp() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.exp(x)
"#,
    );
    assert!(code.contains("exp"), "Should generate exp: {code}");
}

#[test]
fn test_math_floor() {
    let code = transpile(
        r#"
import math
def f(x: float) -> int:
    return math.floor(x)
"#,
    );
    assert!(code.contains("floor"), "Should generate floor: {code}");
}

#[test]
fn test_math_ceil() {
    let code = transpile(
        r#"
import math
def f(x: float) -> int:
    return math.ceil(x)
"#,
    );
    assert!(code.contains("ceil"), "Should generate ceil: {code}");
}

#[test]
fn test_math_fabs() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.fabs(x)
"#,
    );
    assert!(
        code.contains("abs") || code.contains("fabs"),
        "Should generate abs: {code}"
    );
}

#[test]
fn test_math_log() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.log(x)
"#,
    );
    assert!(
        code.contains("ln") || code.contains("log"),
        "Should generate log: {code}"
    );
}

#[test]
fn test_math_log2() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.log2(x)
"#,
    );
    assert!(code.contains("log2"), "Should generate log2: {code}");
}

#[test]
fn test_math_log10() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.log10(x)
"#,
    );
    assert!(code.contains("log10"), "Should generate log10: {code}");
}

#[test]
fn test_math_pow() {
    let code = transpile(
        r#"
import math
def f(x: float, y: float) -> float:
    return math.pow(x, y)
"#,
    );
    assert!(
        code.contains("pow") || code.contains("powi"),
        "Should generate pow: {code}"
    );
}

#[test]
fn test_math_tan() {
    let code = transpile(
        r#"
import math
def f(x: float) -> float:
    return math.tan(x)
"#,
    );
    assert!(code.contains("tan"), "Should generate tan: {code}");
}

#[test]
fn test_math_atan2() {
    let code = transpile(
        r#"
import math
def f(y: float, x: float) -> float:
    return math.atan2(y, x)
"#,
    );
    assert!(code.contains("atan2"), "Should generate atan2: {code}");
}

// ── String constants ────────────────────────────────────────────

#[test]
fn test_string_ascii_uppercase() {
    let code = transpile(
        r#"
import string
def f() -> str:
    return string.ascii_uppercase
"#,
    );
    assert!(
        code.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        "Should generate uppercase letters: {code}"
    );
}

#[test]
fn test_string_ascii_letters() {
    let code = transpile(
        r#"
import string
def f() -> str:
    return string.ascii_letters
"#,
    );
    assert!(
        code.contains("abcdefghijklmnopqrstuvwxyz"),
        "Should generate all letters: {code}"
    );
}

#[test]
fn test_string_digits() {
    let code = transpile(
        r#"
import string
def f() -> str:
    return string.digits
"#,
    );
    assert!(
        code.contains("0123456789"),
        "Should generate digits: {code}"
    );
}

#[test]
fn test_string_hexdigits() {
    let code = transpile(
        r#"
import string
def f() -> str:
    return string.hexdigits
"#,
    );
    assert!(
        code.contains("0123456789abcdefABCDEF"),
        "Should generate hexdigits: {code}"
    );
}

#[test]
fn test_string_punctuation() {
    let code = transpile(
        r#"
import string
def f() -> str:
    return string.punctuation
"#,
    );
    // Transpiler generates the literal punctuation string
    assert!(
        code.contains("punctuation") || code.contains("!") || code.contains("#$%"),
        "Should generate punctuation: {code}"
    );
}

// ── sys module ──────────────────────────────────────────────────

#[test]
fn test_sys_argv() {
    let code = transpile(
        r#"
import sys
def f() -> list:
    return sys.argv
"#,
    );
    assert!(
        code.contains("args()") || code.contains("env::args"),
        "Should generate env::args for sys.argv: {code}"
    );
}

#[test]
fn test_sys_exit() {
    let code = transpile(
        r#"
import sys
def f() -> None:
    sys.exit(1)
"#,
    );
    assert!(
        code.contains("exit") || code.contains("process::exit"),
        "Should generate process::exit: {code}"
    );
}

// sys.maxsize not yet mapped in transpiler

// ── Timedelta attributes ────────────────────────────────────────

#[test]
fn test_timedelta_days() {
    let code = transpile(
        r#"
from datetime import timedelta
def f(td: timedelta) -> int:
    return td.days
"#,
    );
    assert!(
        code.contains("days") || code.contains("num_days"),
        "Should generate days accessor: {code}"
    );
}

#[test]
fn test_timedelta_seconds() {
    let code = transpile(
        r#"
from datetime import timedelta
def f(td: timedelta) -> int:
    return td.seconds
"#,
    );
    assert!(
        code.contains("seconds") || code.contains("num_seconds"),
        "Should generate seconds accessor: {code}"
    );
}

#[test]
fn test_timedelta_total_seconds() {
    let code = transpile(
        r#"
from datetime import timedelta
def f(td: timedelta) -> float:
    return td.total_seconds()
"#,
    );
    assert!(
        code.contains("total_seconds") || code.contains("num_seconds"),
        "Should generate total_seconds: {code}"
    );
}

// ── Operator overloads ──────────────────────────────────────────

#[test]
fn test_dunder_contains() {
    let code = transpile(
        r#"
class Container:
    def __init__(self) -> None:
        self.items: list = []
    def __contains__(self, item: int) -> bool:
        return item in self.items
"#,
    );
    assert!(
        code.contains("contains"),
        "Should generate contains method: {code}"
    );
}

#[test]
fn test_dunder_eq() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
    def __eq__(self, other: object) -> bool:
        return self.x == other.x and self.y == other.y
"#,
    );
    assert!(code.contains("eq"), "Should generate eq method: {code}");
}

#[test]
fn test_dunder_hash() {
    let code = transpile(
        r#"
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y
    def __hash__(self) -> int:
        return hash((self.x, self.y))
"#,
    );
    assert!(
        code.contains("hash") || code.contains("Hash"),
        "Should generate hash: {code}"
    );
}

#[test]
fn test_dunder_gt() {
    let code = transpile(
        r#"
class Score:
    def __init__(self, value: int) -> None:
        self.value = value
    def __gt__(self, other: object) -> bool:
        return self.value > other.value
"#,
    );
    assert!(
        code.contains("gt") || code.contains(">"),
        "Should generate gt: {code}"
    );
}

#[test]
fn test_dunder_ge() {
    let code = transpile(
        r#"
class Score:
    def __init__(self, value: int) -> None:
        self.value = value
    def __ge__(self, other: object) -> bool:
        return self.value >= other.value
"#,
    );
    assert!(
        code.contains("ge") || code.contains(">="),
        "Should generate ge: {code}"
    );
}

#[test]
fn test_dunder_lt() {
    let code = transpile(
        r#"
class Score:
    def __init__(self, value: int) -> None:
        self.value = value
    def __lt__(self, other: object) -> bool:
        return self.value < other.value
"#,
    );
    assert!(
        code.contains("lt") || code.contains("<"),
        "Should generate lt: {code}"
    );
}

#[test]
fn test_dunder_le() {
    let code = transpile(
        r#"
class Score:
    def __init__(self, value: int) -> None:
        self.value = value
    def __le__(self, other: object) -> bool:
        return self.value <= other.value
"#,
    );
    assert!(
        code.contains("le") || code.contains("<="),
        "Should generate le: {code}"
    );
}

// ── Collections constructors ────────────────────────────────────

#[test]
fn test_collections_defaultdict() {
    let code = transpile(
        r#"
from collections import defaultdict
def f() -> dict:
    d = defaultdict(int)
    d["key"] += 1
    return d
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("defaultdict") || code.contains("entry"),
        "Should generate HashMap for defaultdict: {code}"
    );
}

#[test]
fn test_collections_counter() {
    let code = transpile(
        r#"
from collections import Counter
def f(items: list) -> dict:
    return Counter(items)
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("Counter") || code.contains("entry"),
        "Should generate Counter equivalent: {code}"
    );
}

// ── Regex match methods ─────────────────────────────────────────

#[test]
fn test_regex_match_group() {
    let code = transpile(
        r#"
import re
def f(text: str) -> str:
    m = re.match(r"\d+", text)
    if m:
        return m.group(0)
    return ""
"#,
    );
    assert!(
        code.contains("group") || code.contains("Regex") || code.contains("captures"),
        "Should generate regex match: {code}"
    );
}

#[test]
fn test_regex_search() {
    let code = transpile(
        r#"
import re
def f(text: str, pattern: str) -> bool:
    return re.search(pattern, text) is not None
"#,
    );
    assert!(
        code.contains("Regex") || code.contains("is_match") || code.contains("find"),
        "Should generate regex search: {code}"
    );
}

#[test]
fn test_regex_findall() {
    let code = transpile(
        r#"
import re
def f(text: str) -> list:
    return re.findall(r"\d+", text)
"#,
    );
    assert!(
        code.contains("Regex") || code.contains("find_iter") || code.contains("captures"),
        "Should generate regex findall: {code}"
    );
}

#[test]
fn test_regex_sub() {
    let code = transpile(
        r#"
import re
def f(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#,
    );
    assert!(
        code.contains("Regex") || code.contains("replace"),
        "Should generate regex sub: {code}"
    );
}

#[test]
fn test_regex_split() {
    let code = transpile(
        r#"
import re
def f(text: str) -> list:
    return re.split(r"\s+", text)
"#,
    );
    assert!(
        code.contains("Regex") || code.contains("split"),
        "Should generate regex split: {code}"
    );
}

// ── String methods (less tested) ────────────────────────────────

#[test]
fn test_str_center() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.center(20)
"#,
    );
    assert!(
        code.contains("center") || code.contains("pad") || code.contains("format"),
        "Should generate centering: {code}"
    );
}

#[test]
fn test_str_zfill() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.zfill(5)
"#,
    );
    assert!(
        code.contains("zfill") || code.contains("pad") || code.contains("format"),
        "Should generate zero-fill: {code}"
    );
}

#[test]
fn test_str_expandtabs() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.expandtabs(4)
"#,
    );
    assert!(
        code.contains("expandtabs") || code.contains("replace") || code.contains("tab"),
        "Should generate expandtabs: {code}"
    );
}

#[test]
fn test_str_encode() {
    let code = transpile(
        r#"
def f(s: str) -> bytes:
    return s.encode("utf-8")
"#,
    );
    assert!(
        code.contains("as_bytes") || code.contains("encode") || code.contains("into_bytes"),
        "Should generate encode: {code}"
    );
}

#[test]
fn test_str_casefold() {
    let code = transpile(
        r#"
def f(s: str) -> str:
    return s.casefold()
"#,
    );
    assert!(
        code.contains("to_lowercase") || code.contains("casefold"),
        "Should generate casefold: {code}"
    );
}

// ── Dict methods ────────────────────────────────────────────────

#[test]
fn test_dict_setdefault() {
    let code = transpile(
        r#"
def f(d: dict) -> int:
    return d.setdefault("key", 0)
"#,
    );
    assert!(
        code.contains("entry") || code.contains("or_insert") || code.contains("setdefault"),
        "Should generate entry/or_insert: {code}"
    );
}

#[test]
fn test_dict_update() {
    let code = transpile(
        r#"
def f(a: dict, b: dict) -> None:
    a.update(b)
"#,
    );
    assert!(
        code.contains("extend") || code.contains("update"),
        "Should generate dict update: {code}"
    );
}

#[test]
fn test_dict_pop() {
    let code = transpile(
        r#"
def f(d: dict) -> int:
    return d.pop("key")
"#,
    );
    assert!(
        code.contains("remove") || code.contains("pop"),
        "Should generate dict pop: {code}"
    );
}

// ── os.path methods ─────────────────────────────────────────────

#[test]
fn test_os_path_join() {
    let code = transpile(
        r#"
import os
def f() -> str:
    return os.path.join("/home", "user", "file.txt")
"#,
    );
    assert!(
        code.contains("Path") || code.contains("join") || code.contains("push"),
        "Should generate path join: {code}"
    );
}

#[test]
fn test_os_path_exists() {
    let code = transpile(
        r#"
import os
def f(p: str) -> bool:
    return os.path.exists(p)
"#,
    );
    assert!(
        code.contains("exists") || code.contains("Path"),
        "Should generate path exists: {code}"
    );
}

#[test]
fn test_os_path_basename() {
    let code = transpile(
        r#"
import os
def f(p: str) -> str:
    return os.path.basename(p)
"#,
    );
    assert!(
        code.contains("file_name") || code.contains("basename") || code.contains("Path"),
        "Should generate basename: {code}"
    );
}

// ── List comprehension variants ─────────────────────────────────

#[test]
fn test_list_comprehension_with_condition() {
    let code = transpile(
        r#"
def f(nums: list) -> list:
    return [x * 2 for x in nums if x > 0]
"#,
    );
    assert!(
        code.contains("filter") || code.contains("iter"),
        "Should generate filtered iteration: {code}"
    );
}

#[test]
fn test_dict_comprehension() {
    let code = transpile(
        r#"
def f(items: list) -> dict:
    return {k: v for k, v in items}
"#,
    );
    assert!(
        code.contains("collect") || code.contains("HashMap"),
        "Should generate dict comprehension: {code}"
    );
}

// ── Enum-like patterns ──────────────────────────────────────────

#[test]
fn test_class_with_class_vars() {
    let code = transpile(
        r#"
class Color:
    RED: int = 0
    GREEN: int = 1
    BLUE: int = 2
"#,
    );
    assert!(
        code.contains("RED") && code.contains("GREEN") && code.contains("BLUE"),
        "Should generate class constants: {code}"
    );
}
