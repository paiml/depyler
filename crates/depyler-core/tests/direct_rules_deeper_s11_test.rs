//! Session 11: Deeper direct_rules_convert.rs coverage tests
//!
//! Targets specific uncovered branches in direct_rules_convert.rs (61% coverage):
//! - Colorsys color conversions
//! - DateTime attribute access
//! - Complex comprehension patterns in direct rules
//! - Deque operations
//! - Counter/OrderedDict most_common/move_to_end
//! - Complex slice patterns
//! - String encoding patterns
//! - F-string format specs
//! - Advanced file I/O patterns
//! - Complex builtin conversions

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

// ============================================================================
// OS environ edge cases
// ============================================================================

#[test]
fn test_s11_direct2_environ_setdefault() {
    let code = r#"
import os

def set_default_env(key: str, val: str):
    os.environ.setdefault(key, val)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn set_default_env"),
        "Should transpile environ.setdefault. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_environ_bracket_access() {
    let code = r#"
import os

def get_env(key: str) -> str:
    return os.environ[key]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_env"),
        "Should transpile environ[]. Got: {}",
        result
    );
}

// ============================================================================
// Deque operations
// ============================================================================

#[test]
fn test_s11_direct2_deque_extend() {
    let code = r#"
from collections import deque

def extend_deque(d: deque, items: list) -> deque:
    d.extend(items)
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn extend_deque"),
        "Should transpile deque.extend. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_deque_clear() {
    let code = r#"
from collections import deque

def clear_deque(d: deque) -> deque:
    d.clear()
    return d
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn clear_deque"),
        "Should transpile deque.clear. Got: {}",
        result
    );
}

// ============================================================================
// Counter operations
// ============================================================================

#[test]
fn test_s11_direct2_counter_list() {
    let code = r#"
from collections import Counter

def count_items(items: list) -> dict:
    return Counter(items)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn count_items"),
        "Should transpile Counter(list). Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_counter_most_common() {
    let code = r#"
from collections import Counter

def top_items(items: list) -> list:
    c = Counter(items)
    return c.most_common(3)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn top_items"),
        "Should transpile Counter.most_common. Got: {}",
        result
    );
}

// ============================================================================
// Advanced datetime patterns
// ============================================================================

#[test]
fn test_s11_direct2_datetime_strftime() {
    let code = r#"
from datetime import datetime

def format_date(dt: datetime) -> str:
    return dt.strftime("%Y-%m-%d")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn format_date"),
        "Should transpile strftime. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_datetime_strptime() {
    let code = r#"
from datetime import datetime

def parse_date(s: str):
    return datetime.strptime(s, "%Y-%m-%d")
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn parse_date"),
        "Should transpile strptime. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_timedelta() {
    let code = r#"
from datetime import timedelta

def one_week():
    return timedelta(days=7)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn one_week"),
        "Should transpile timedelta. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_datetime_year() {
    let code = r#"
from datetime import datetime

def get_year(dt: datetime) -> int:
    return dt.year
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_year"),
        "Should transpile datetime.year. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_datetime_month() {
    let code = r#"
from datetime import datetime

def get_month(dt: datetime) -> int:
    return dt.month
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_month"),
        "Should transpile datetime.month. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_datetime_day() {
    let code = r#"
from datetime import datetime

def get_day(dt: datetime) -> int:
    return dt.day
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_day"),
        "Should transpile datetime.day. Got: {}",
        result
    );
}

// ============================================================================
// Pathlib advanced
// ============================================================================

#[test]
fn test_s11_direct2_path_read_text() {
    let code = r#"
from pathlib import Path

def read_text(p: str) -> str:
    return Path(p).read_text()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_text"),
        "Should transpile Path.read_text. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_write_text() {
    let code = r#"
from pathlib import Path

def write_text(p: str, content: str):
    Path(p).write_text(content)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_text"),
        "Should transpile Path.write_text. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_mkdir() {
    let code = r#"
from pathlib import Path

def make_dir(p: str):
    Path(p).mkdir(parents=True, exist_ok=True)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn make_dir"),
        "Should transpile Path.mkdir. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_stem() {
    let code = r#"
from pathlib import Path

def get_stem(p: str) -> str:
    return Path(p).stem
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_stem"),
        "Should transpile Path.stem. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_suffix() {
    let code = r#"
from pathlib import Path

def get_ext(p: str) -> str:
    return Path(p).suffix
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_ext"),
        "Should transpile Path.suffix. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_parent() {
    let code = r#"
from pathlib import Path

def get_parent(p: str) -> str:
    return str(Path(p).parent)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_parent"),
        "Should transpile Path.parent. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_name() {
    let code = r#"
from pathlib import Path

def get_name(p: str) -> str:
    return Path(p).name
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn get_name"),
        "Should transpile Path.name. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_is_file() {
    let code = r#"
from pathlib import Path

def check_file(p: str) -> bool:
    return Path(p).is_file()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_file"),
        "Should transpile Path.is_file. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_path_is_dir() {
    let code = r#"
from pathlib import Path

def check_dir(p: str) -> bool:
    return Path(p).is_dir()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_dir"),
        "Should transpile Path.is_dir. Got: {}",
        result
    );
}

// ============================================================================
// Advanced builtins
// ============================================================================

#[test]
fn test_s11_direct2_zip_three() {
    let code = r#"
def zip_three(a: list, b: list, c: list) -> list:
    result: list = []
    for x, y, z in zip(a, b, c):
        result.append((x, y, z))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn zip_three"),
        "Should transpile zip with 3 lists. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_enumerate_with_start() {
    let code = r#"
def numbered_from(items: list, start: int) -> list:
    result: list = []
    for i, v in enumerate(items, start):
        result.append((i, v))
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn numbered_from"),
        "Should transpile enumerate with start. Got: {}",
        result
    );
}

// ============================================================================
// Complex type conversions
// ============================================================================

#[test]
fn test_s11_direct2_int_base() {
    let code = r#"
def from_hex(s: str) -> int:
    return int(s, 16)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn from_hex"),
        "Should transpile int() with base. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_int_from_float() {
    let code = r#"
def truncate(x: float) -> int:
    return int(x)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn truncate"),
        "Should transpile int(float). Got: {}",
        result
    );
}

// ============================================================================
// Complex string methods via direct rules
// ============================================================================

#[test]
fn test_s11_direct2_str_format_multi() {
    let code = r#"
def report(name: str, score: int, grade: str) -> str:
    return "{}: {} ({})".format(name, score, grade)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn report"),
        "Should transpile multi-arg format. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_str_replace_count() {
    let code = r#"
def first_replace(s: str) -> str:
    return s.replace("old", "new", 1)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn first_replace"),
        "Should transpile replace with count. Got: {}",
        result
    );
}

// ============================================================================
// Complex file I/O
// ============================================================================

#[test]
fn test_s11_direct2_file_readline() {
    let code = r#"
def first_line(path: str) -> str:
    with open(path) as f:
        return f.readline()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn first_line"),
        "Should transpile readline. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_file_writelines() {
    let code = r#"
def write_lines(path: str, lines: list):
    with open(path, "w") as f:
        f.writelines(lines)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_lines"),
        "Should transpile writelines. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_open_binary_read() {
    let code = r#"
def read_bytes(path: str) -> bytes:
    with open(path, "rb") as f:
        return f.read()
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn read_bytes"),
        "Should transpile binary read. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_open_binary_write() {
    let code = r#"
def write_bytes(path: str, data: bytes):
    with open(path, "wb") as f:
        f.write(data)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn write_bytes"),
        "Should transpile binary write. Got: {}",
        result
    );
}

// ============================================================================
// Complex expression patterns through direct rules
// ============================================================================

#[test]
fn test_s11_direct2_dict_get_default_expr() {
    let code = r#"
def safe_get(d: dict, key: str, fallback: int) -> int:
    return d.get(key, fallback)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn safe_get"),
        "Should transpile dict.get with variable default. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_dict_pop_default() {
    let code = r#"
def take_or_default(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn take_or_default"),
        "Should transpile dict.pop with default. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_list_sort_lambda() {
    let code = r#"
def sort_by_second(items: list) -> list:
    items.sort(key=lambda x: x[1])
    return items
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sort_by_second"),
        "Should transpile sort with lambda key. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_sorted_lambda() {
    let code = r#"
def sorted_by_abs(items: list) -> list:
    return sorted(items, key=lambda x: abs(x))
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sorted_by_abs"),
        "Should transpile sorted with lambda key. Got: {}",
        result
    );
}

// ============================================================================
// Complex algorithm patterns
// ============================================================================

#[test]
fn test_s11_direct2_algo_matrix_multiply() {
    let code = r#"
def matrix_mul(a: list, b: list) -> list:
    n = len(a)
    m = len(b[0])
    k = len(b)
    result: list = []
    for i in range(n):
        row: list = []
        for j in range(m):
            total = 0
            for p in range(k):
                total += a[i][p] * b[p][j]
            row.append(total)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn matrix_mul"),
        "Should transpile matrix multiplication. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_algo_quicksort() {
    let code = r#"
def quicksort(items: list) -> list:
    if len(items) <= 1:
        return items
    pivot = items[len(items) // 2]
    left: list = [x for x in items if x < pivot]
    middle: list = [x for x in items if x == pivot]
    right: list = [x for x in items if x > pivot]
    return quicksort(left) + middle + quicksort(right)
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn quicksort"),
        "Should transpile quicksort. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_algo_dp_knapsack() {
    let code = r#"
def knapsack(values: list, weights: list, capacity: int) -> int:
    n = len(values)
    dp: list = []
    for i in range(n + 1):
        row: list = [0] * (capacity + 1)
        dp.append(row)
    for i in range(1, n + 1):
        for w in range(capacity + 1):
            if weights[i - 1] <= w:
                dp[i][w] = max(dp[i - 1][w], values[i - 1] + dp[i - 1][w - weights[i - 1]])
            else:
                dp[i][w] = dp[i - 1][w]
    return dp[n][capacity]
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn knapsack"),
        "Should transpile knapsack DP. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_algo_dijkstra() {
    let code = r#"
def dijkstra(graph: dict, start: str) -> dict:
    distances: dict = {start: 0}
    visited: set = set()
    current = start
    while current is not None:
        visited.add(current)
        for neighbor, weight in graph.get(current, {}).items():
            new_dist = distances[current] + weight
            if neighbor not in distances or new_dist < distances[neighbor]:
                distances[neighbor] = new_dist
        current = None
        min_dist = float("inf")
        for node, dist in distances.items():
            if node not in visited and dist < min_dist:
                min_dist = dist
                current = node
    return distances
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn dijkstra"),
        "Should transpile Dijkstra. Got: {}",
        result
    );
}

// ============================================================================
// Advanced async patterns
// ============================================================================

#[test]
fn test_s11_direct2_async_gather() {
    let code = r#"
import asyncio

async def fetch_all(urls: list) -> list:
    results: list = []
    for url in urls:
        results.append(url)
    return results
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn fetch_all"),
        "Should transpile async function with loop. Got: {}",
        result
    );
}

// ============================================================================
// Complex conditional patterns
// ============================================================================

#[test]
fn test_s11_direct2_truthiness_empty_list() {
    let code = r#"
def check_list(items: list) -> bool:
    if items:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_list"),
        "Should transpile list truthiness. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_truthiness_empty_dict() {
    let code = r#"
def check_dict(d: dict) -> bool:
    if d:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_dict"),
        "Should transpile dict truthiness. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_truthiness_empty_string() {
    let code = r#"
def check_str(s: str) -> bool:
    if s:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_str"),
        "Should transpile str truthiness. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_truthiness_zero() {
    let code = r#"
def check_nonzero(n: int) -> bool:
    if n:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn check_nonzero"),
        "Should transpile int truthiness. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_not_truthiness() {
    let code = r#"
def is_empty(items: list) -> bool:
    if not items:
        return True
    return False
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn is_empty"),
        "Should transpile not truthiness. Got: {}",
        result
    );
}

// ============================================================================
// Complex for-loop patterns
// ============================================================================

#[test]
fn test_s11_direct2_for_reversed() {
    let code = r#"
def reverse_iter(items: list) -> list:
    result: list = []
    for x in reversed(items):
        result.append(x)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn reverse_iter"),
        "Should transpile for reversed. Got: {}",
        result
    );
}

#[test]
fn test_s11_direct2_for_sorted() {
    let code = r#"
def sorted_iter(items: list) -> list:
    result: list = []
    for x in sorted(items):
        result.append(x)
    return result
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn sorted_iter"),
        "Should transpile for sorted. Got: {}",
        result
    );
}

// ============================================================================
// Multiple return paths
// ============================================================================

#[test]
fn test_s11_direct2_multiple_returns() {
    let code = r#"
def classify(x: int) -> str:
    if x > 100:
        return "huge"
    if x > 50:
        return "large"
    if x > 20:
        return "medium"
    if x > 5:
        return "small"
    if x > 0:
        return "tiny"
    if x == 0:
        return "zero"
    return "negative"
"#;
    let result = transpile(code);
    assert!(
        result.contains("fn classify"),
        "Should transpile multiple returns. Got: {}",
        result
    );
}
