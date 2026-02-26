//! Session 12 Batch 7: Deep coverage tests for direct_rules_convert.rs cold paths
//!
//! Targets the following uncovered sections:
//! - re module: finditer, subn, escape, compile, split (with both literal and variable patterns)
//! - colorsys module: rgb_to_hsv, hsv_to_rgb, rgb_to_hls, hls_to_rgb
//! - hashlib: blake2b, blake2s, new() factory
//! - string methods: isdigit, isalpha, isalnum, __contains__/contains
//! - dict-like contains detection heuristics
//! - Semaphore/Mutex acquire/release methods
//! - copy() method on list/dict
//! - datetime module: datetime, date, time, timedelta, now
//! - collections module: deque, Counter, OrderedDict, defaultdict
//! - asyncio module: Event, Lock, Semaphore, Queue, sleep, run
//! - json module: loads, dumps
//! - os module: getcwd, getenv, listdir
//! - threading module: Lock, Event, Thread
//! - queue module: Queue, LifoQueue
//! - fnmatch module

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

// ===== re module: finditer =====

#[test]
fn test_s12_re_finditer_literals() {
    let code = r#"
import re

def find_all_matches(text: str) -> list:
    matches = re.finditer("[0-9]+", text)
    return list(matches)
"#;
    let result = transpile(code);
    assert!(result.contains("find_all_matches"), "Got: {}", result);
}

#[test]
fn test_s12_re_finditer_variable_pattern() {
    let code = r#"
import re

def find_pattern(pattern: str, text: str) -> list:
    matches = re.finditer(pattern, text)
    return list(matches)
"#;
    let result = transpile(code);
    assert!(result.contains("find_pattern"), "Got: {}", result);
}

// ===== re module: subn =====

#[test]
fn test_s12_re_subn_literals() {
    let code = r#"
import re

def replace_count(text: str) -> tuple:
    result = re.subn("[0-9]", "X", text)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("replace_count"), "Got: {}", result);
}

#[test]
fn test_s12_re_subn_variable_args() {
    let code = r#"
import re

def replace_dynamic(pattern: str, repl: str, text: str) -> tuple:
    result = re.subn(pattern, repl, text)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("replace_dynamic"), "Got: {}", result);
}

// ===== re module: escape =====

#[test]
fn test_s12_re_escape_literal() {
    let code = r#"
import re

def escape_special(text: str) -> str:
    return re.escape("hello.world")
"#;
    let result = transpile(code);
    assert!(result.contains("escape_special"), "Got: {}", result);
}

#[test]
fn test_s12_re_escape_variable() {
    let code = r#"
import re

def escape_var(pattern: str) -> str:
    return re.escape(pattern)
"#;
    let result = transpile(code);
    assert!(result.contains("escape_var"), "Got: {}", result);
}

// ===== re module: compile =====

#[test]
fn test_s12_re_compile_literal() {
    let code = r#"
import re

def compile_pattern() -> str:
    pattern = re.compile("[a-z]+")
    return str(pattern)
"#;
    let result = transpile(code);
    assert!(result.contains("compile_pattern"), "Got: {}", result);
}

#[test]
fn test_s12_re_compile_variable() {
    let code = r#"
import re

def compile_dynamic(pat: str) -> str:
    compiled = re.compile(pat)
    return str(compiled)
"#;
    let result = transpile(code);
    assert!(result.contains("compile_dynamic"), "Got: {}", result);
}

// ===== re module: split =====

#[test]
fn test_s12_re_split_literal() {
    let code = r#"
import re

def split_text(text: str) -> list:
    return re.split("[,;]", text)
"#;
    let result = transpile(code);
    assert!(result.contains("split_text"), "Got: {}", result);
}

#[test]
fn test_s12_re_split_variable() {
    let code = r#"
import re

def split_dynamic(pattern: str, text: str) -> list:
    return re.split(pattern, text)
"#;
    let result = transpile(code);
    assert!(result.contains("split_dynamic"), "Got: {}", result);
}

// ===== re module: sub with literals and variables =====

#[test]
fn test_s12_re_sub_all_literals() {
    let code = r#"
import re

def clean_text() -> str:
    return re.sub("[0-9]+", "", "abc123def456")
"#;
    let result = transpile(code);
    assert!(result.contains("clean_text"), "Got: {}", result);
}

#[test]
fn test_s12_re_sub_variable_args() {
    let code = r#"
import re

def replace_pattern(pattern: str, repl: str, text: str) -> str:
    return re.sub(pattern, repl, text)
"#;
    let result = transpile(code);
    assert!(result.contains("replace_pattern"), "Got: {}", result);
}

// ===== colorsys module =====

#[test]
fn test_s12_colorsys_rgb_to_hsv() {
    let code = r#"
import colorsys

def convert_rgb_hsv(r: float, g: float, b: float) -> tuple:
    return colorsys.rgb_to_hsv(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("convert_rgb_hsv"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hsv_to_rgb() {
    let code = r#"
import colorsys

def convert_hsv_rgb(h: float, s: float, v: float) -> tuple:
    return colorsys.hsv_to_rgb(h, s, v)
"#;
    let result = transpile(code);
    assert!(result.contains("convert_hsv_rgb"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_rgb_to_hls() {
    let code = r#"
import colorsys

def convert_rgb_hls(r: float, g: float, b: float) -> tuple:
    return colorsys.rgb_to_hls(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("convert_rgb_hls"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hls_to_rgb() {
    let code = r#"
import colorsys

def convert_hls_rgb(h: float, l: float, s: float) -> tuple:
    return colorsys.hls_to_rgb(h, l, s)
"#;
    let result = transpile(code);
    assert!(result.contains("convert_hls_rgb"), "Got: {}", result);
}

// ===== String check methods =====

#[test]
fn test_s12_str_isdigit() {
    let code = r#"
def check_digits(s: str) -> bool:
    return s.isdigit()
"#;
    let result = transpile(code);
    assert!(result.contains("check_digits"), "Got: {}", result);
    assert!(result.contains("is_ascii_digit") || result.contains("isdigit"), "Got: {}", result);
}

#[test]
fn test_s12_str_isalpha() {
    let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let result = transpile(code);
    assert!(result.contains("check_alpha"), "Got: {}", result);
    assert!(result.contains("is_alphabetic") || result.contains("isalpha"), "Got: {}", result);
}

#[test]
fn test_s12_str_isalnum() {
    let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let result = transpile(code);
    assert!(result.contains("check_alnum"), "Got: {}", result);
    assert!(result.contains("is_alphanumeric") || result.contains("isalnum"), "Got: {}", result);
}

// ===== Dict-like contains heuristics =====

#[test]
fn test_s12_dict_contains_by_name() {
    let code = r#"
def lookup_config(config: dict, key: str) -> bool:
    return key in config
"#;
    let result = transpile(code);
    assert!(result.contains("lookup_config"), "Got: {}", result);
}

#[test]
fn test_s12_dict_contains_settings() {
    let code = r#"
def check_settings(settings: dict, name: str) -> bool:
    return name in settings
"#;
    let result = transpile(code);
    assert!(result.contains("check_settings"), "Got: {}", result);
}

#[test]
fn test_s12_string_contains() {
    let code = r#"
def has_substring(text: str, sub: str) -> bool:
    return sub in text
"#;
    let result = transpile(code);
    assert!(result.contains("has_substring"), "Got: {}", result);
}

// ===== acquire/release (Mutex/Semaphore) =====

#[test]
fn test_s12_mutex_acquire_release() {
    let code = r#"
def use_lock(lock: object) -> bool:
    acquired = lock.acquire()
    lock.release()
    return acquired
"#;
    let result = transpile(code);
    assert!(result.contains("use_lock"), "Got: {}", result);
}

// ===== copy method =====

#[test]
fn test_s12_list_copy() {
    let code = r#"
def duplicate_list(items: list) -> list:
    return items.copy()
"#;
    let result = transpile(code);
    assert!(result.contains("duplicate_list"), "Got: {}", result);
    assert!(result.contains("clone"), "Expected .clone(), got: {}", result);
}

// ===== datetime module =====

#[test]
fn test_s12_datetime_datetime() {
    let code = r#"
import datetime

def get_now():
    return datetime.datetime(2024, 1, 1)
"#;
    let result = transpile(code);
    assert!(result.contains("get_now"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_date() {
    let code = r#"
import datetime

def get_date():
    return datetime.date(2024, 6, 15)
"#;
    let result = transpile(code);
    assert!(result.contains("get_date"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_time() {
    let code = r#"
import datetime

def get_time():
    return datetime.time(12, 30, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("get_time"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_timedelta_with_arg() {
    let code = r#"
import datetime

def get_delta():
    return datetime.timedelta(7)
"#;
    let result = transpile(code);
    assert!(result.contains("get_delta"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_timedelta_no_arg() {
    let code = r#"
import datetime

def get_zero_delta():
    return datetime.timedelta()
"#;
    let result = transpile(code);
    assert!(result.contains("get_zero_delta"), "Got: {}", result);
}

#[test]
fn test_s12_datetime_now() {
    let code = r#"
import datetime

def current_time():
    return datetime.now()
"#;
    let result = transpile(code);
    assert!(result.contains("current_time"), "Got: {}", result);
}

// ===== collections module =====

#[test]
fn test_s12_collections_deque_empty() {
    let code = r#"
import collections

def make_deque():
    return collections.deque()
"#;
    let result = transpile(code);
    assert!(result.contains("make_deque"), "Got: {}", result);
}

#[test]
fn test_s12_collections_deque_with_arg() {
    let code = r#"
import collections

def make_deque_from(items: list):
    return collections.deque(items)
"#;
    let result = transpile(code);
    assert!(result.contains("make_deque_from"), "Got: {}", result);
}

#[test]
fn test_s12_collections_counter_empty() {
    let code = r#"
import collections

def make_counter():
    return collections.Counter()
"#;
    let result = transpile(code);
    assert!(result.contains("make_counter"), "Got: {}", result);
}

#[test]
fn test_s12_collections_counter_with_arg() {
    let code = r#"
import collections

def count_items(items: list):
    return collections.Counter(items)
"#;
    let result = transpile(code);
    assert!(result.contains("count_items"), "Got: {}", result);
}

#[test]
fn test_s12_collections_ordered_dict_empty() {
    let code = r#"
import collections

def make_ordered():
    return collections.OrderedDict()
"#;
    let result = transpile(code);
    assert!(result.contains("make_ordered"), "Got: {}", result);
}

#[test]
fn test_s12_collections_defaultdict() {
    let code = r#"
import collections

def make_default():
    return collections.defaultdict(int)
"#;
    let result = transpile(code);
    assert!(result.contains("make_default"), "Got: {}", result);
}

// ===== asyncio module =====

#[test]
fn test_s12_asyncio_event() {
    let code = r#"
import asyncio

async def make_event():
    event = asyncio.Event()
    return event
"#;
    let result = transpile(code);
    assert!(result.contains("make_event"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_lock() {
    let code = r#"
import asyncio

async def make_lock():
    lock = asyncio.Lock()
    return lock
"#;
    let result = transpile(code);
    assert!(result.contains("make_lock"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_semaphore() {
    let code = r#"
import asyncio

async def make_sem():
    sem = asyncio.Semaphore(5)
    return sem
"#;
    let result = transpile(code);
    assert!(result.contains("make_sem"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_queue() {
    let code = r#"
import asyncio

async def make_queue():
    q = asyncio.Queue()
    return q
"#;
    let result = transpile(code);
    assert!(result.contains("make_queue"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_sleep_with_arg() {
    let code = r#"
import asyncio

async def wait_a_bit():
    asyncio.sleep(1.5)
"#;
    let result = transpile(code);
    assert!(result.contains("wait_a_bit"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_sleep_no_arg() {
    let code = r#"
import asyncio

async def yield_control():
    asyncio.sleep()
"#;
    let result = transpile(code);
    assert!(result.contains("yield_control"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_run() {
    let code = r#"
import asyncio

async def main():
    return 42

def start():
    return asyncio.run(main())
"#;
    let result = transpile(code);
    assert!(result.contains("start") || result.contains("main"), "Got: {}", result);
}

// ===== json module =====

#[test]
fn test_s12_json_loads() {
    let code = r#"
import json

def parse_json(data: str) -> dict:
    return json.loads(data)
"#;
    let result = transpile(code);
    assert!(result.contains("parse_json"), "Got: {}", result);
}

#[test]
fn test_s12_json_dumps() {
    let code = r#"
import json

def to_json(data: dict) -> str:
    return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("to_json"), "Got: {}", result);
}

// ===== os module =====

#[test]
fn test_s12_os_getcwd() {
    let code = r#"
import os

def current_dir() -> str:
    return os.getcwd()
"#;
    let result = transpile(code);
    assert!(result.contains("current_dir"), "Got: {}", result);
}

#[test]
fn test_s12_os_getenv() {
    let code = r#"
import os

def get_home() -> str:
    return os.getenv("HOME")
"#;
    let result = transpile(code);
    assert!(result.contains("get_home"), "Got: {}", result);
}

#[test]
fn test_s12_os_listdir_with_arg() {
    let code = r#"
import os

def list_files(path: str) -> list:
    return os.listdir(path)
"#;
    let result = transpile(code);
    assert!(result.contains("list_files"), "Got: {}", result);
}

#[test]
fn test_s12_os_listdir_no_arg() {
    let code = r#"
import os

def list_current() -> list:
    return os.listdir()
"#;
    let result = transpile(code);
    assert!(result.contains("list_current"), "Got: {}", result);
}

// ===== threading module =====

#[test]
fn test_s12_threading_lock() {
    let code = r#"
import threading

def make_lock():
    return threading.Lock()
"#;
    let result = transpile(code);
    assert!(result.contains("make_lock"), "Got: {}", result);
}

#[test]
fn test_s12_threading_event() {
    let code = r#"
import threading

def make_event():
    return threading.Event()
"#;
    let result = transpile(code);
    assert!(result.contains("make_event"), "Got: {}", result);
}

#[test]
fn test_s12_threading_thread() {
    let code = r#"
import threading

def spawn_thread():
    return threading.Thread()
"#;
    let result = transpile(code);
    assert!(result.contains("spawn_thread"), "Got: {}", result);
}

// ===== queue module =====

#[test]
fn test_s12_queue_queue() {
    let code = r#"
import queue

def make_queue():
    return queue.Queue()
"#;
    let result = transpile(code);
    assert!(result.contains("make_queue"), "Got: {}", result);
}

// ===== fnmatch module =====

#[test]
fn test_s12_fnmatch() {
    let code = r#"
import fnmatch

def matches(name: str, pattern: str) -> bool:
    return fnmatch.fnmatch(name, pattern)
"#;
    let result = transpile(code);
    assert!(result.contains("matches"), "Got: {}", result);
}

// ===== hashlib: blake2b, blake2s, new =====

#[test]
fn test_s12_hashlib_blake2b() {
    let code = r#"
import hashlib

def hash_blake2b(data: str) -> str:
    h = hashlib.blake2b(data.encode())
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("hash_blake2b"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_blake2s() {
    let code = r#"
import hashlib

def hash_blake2s(data: str) -> str:
    h = hashlib.blake2s(data.encode())
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("hash_blake2s"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_new() {
    let code = r#"
import hashlib

def hash_dynamic(algo: str, data: str) -> str:
    h = hashlib.new(algo, data.encode())
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("hash_dynamic"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_new_static_sha256() {
    let code = r#"
import hashlib

def hash_with_new() -> str:
    h = hashlib.new("sha256")
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("hash_with_new"), "Got: {}", result);
    // Verify it transpiles successfully - the dispatch happens internally
    assert!(result.contains("fn hash_with_new"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_blake2b_no_args() {
    let code = r#"
import hashlib

def empty_blake2b() -> str:
    h = hashlib.blake2b()
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("empty_blake2b"), "Got: {}", result);
}

#[test]
fn test_s12_hashlib_blake2s_no_args() {
    let code = r#"
import hashlib

def empty_blake2s() -> str:
    h = hashlib.blake2s()
    return h.hexdigest()
"#;
    let result = transpile(code);
    assert!(result.contains("empty_blake2s"), "Got: {}", result);
}

// ===== Unary positive operator =====

#[test]
fn test_s12_unary_positive() {
    let code = r#"
def identity(x: int) -> int:
    return +x
"#;
    let result = transpile(code);
    assert!(result.contains("identity"), "Got: {}", result);
}

// ===== Floor division edge cases =====

#[test]
fn test_s12_floor_div_float() {
    let code = r#"
def floor_divide(a: float, b: float) -> float:
    return a // b
"#;
    let result = transpile(code);
    assert!(result.contains("floor_divide"), "Got: {}", result);
}

// ===== String slicing with negative indices =====

#[test]
fn test_s12_string_negative_slice() {
    let code = r#"
def last_three(s: str) -> str:
    return s[-3:]
"#;
    let result = transpile(code);
    assert!(result.contains("last_three"), "Got: {}", result);
}

#[test]
fn test_s12_string_slice_both_ends() {
    let code = r#"
def middle(s: str) -> str:
    return s[1:-1]
"#;
    let result = transpile(code);
    assert!(result.contains("middle"), "Got: {}", result);
}

// ===== Complex method chains =====

#[test]
fn test_s12_str_rfind() {
    let code = r#"
def find_last(text: str, sub: str) -> int:
    return text.rfind(sub)
"#;
    let result = transpile(code);
    assert!(result.contains("find_last"), "Got: {}", result);
}

// ===== contains_key direct call =====

#[test]
fn test_s12_dict_get_method() {
    let code = r#"
def safe_get(data: dict, key: str) -> int:
    return data.get(key, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("safe_get"), "Got: {}", result);
}

// ===== Multiple patterns for coverage lift =====

#[test]
fn test_s12_re_findall_variable() {
    let code = r#"
import re

def find_all(pattern: str, text: str) -> list:
    return re.findall(pattern, text)
"#;
    let result = transpile(code);
    assert!(result.contains("find_all"), "Got: {}", result);
}

#[test]
fn test_s12_re_match_variable() {
    let code = r#"
import re

def try_match(pattern: str, text: str) -> bool:
    return re.match(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("try_match"), "Got: {}", result);
}

#[test]
fn test_s12_re_search_variable() {
    let code = r#"
import re

def search_text(pattern: str, text: str) -> bool:
    return re.search(pattern, text) is not None
"#;
    let result = transpile(code);
    assert!(result.contains("search_text"), "Got: {}", result);
}
