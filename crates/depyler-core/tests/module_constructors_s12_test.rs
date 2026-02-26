//! Session 12 Batch 24: Module constructor and stdlib conversion cold paths
//!
//! Targets direct_rules_convert.rs cold paths:
//! - Collections module: deque, Counter, OrderedDict, defaultdict
//! - Queue module: Queue, LifoQueue, PriorityQueue
//! - Asyncio module: Event, Lock, Semaphore, sleep
//! - Time module: time.time(), time.sleep(), time.ctime()
//! - JSON module: json.loads(), json.dumps()
//! - Base64 module: b64encode, b64decode, urlsafe variants
//! - Colorsys module: rgb_to_hsv, hsv_to_rgb
//! - int.from_bytes class method
//! - dict.fromkeys class method

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

// ===== Collections module constructors =====

#[test]
fn test_s12_collections_deque_empty() {
    let code = r#"
class Queue:
    def __init__(self):
        self.items = collections.deque()
"#;
    let result = transpile(code);
    assert!(result.contains("Queue"), "Got: {}", result);
}

#[test]
fn test_s12_collections_deque_from_list() {
    let code = r#"
class Queue:
    def __init__(self, items: list):
        self.items = collections.deque(items)
"#;
    let result = transpile(code);
    assert!(result.contains("Queue"), "Got: {}", result);
}

#[test]
fn test_s12_collections_counter() {
    let code = r#"
class WordCounter:
    def __init__(self, words: list):
        self.counts = collections.Counter(words)
"#;
    let result = transpile(code);
    assert!(result.contains("WordCounter"), "Got: {}", result);
}

#[test]
fn test_s12_collections_ordered_dict() {
    let code = r#"
class OrderedRegistry:
    def __init__(self):
        self.data = collections.OrderedDict()
"#;
    let result = transpile(code);
    assert!(result.contains("OrderedRegistry"), "Got: {}", result);
}

#[test]
fn test_s12_collections_defaultdict() {
    let code = r#"
class GroupedData:
    def __init__(self):
        self.groups = collections.defaultdict(list)
"#;
    let result = transpile(code);
    assert!(result.contains("GroupedData"), "Got: {}", result);
}

// ===== Queue module constructors =====

#[test]
fn test_s12_queue_constructor() {
    let code = r#"
class TaskQueue:
    def __init__(self):
        self.q = queue.Queue()
"#;
    let result = transpile(code);
    assert!(result.contains("TaskQueue"), "Got: {}", result);
}

#[test]
fn test_s12_queue_lifo() {
    let code = r#"
class StackQueue:
    def __init__(self):
        self.q = queue.LifoQueue()
"#;
    let result = transpile(code);
    assert!(result.contains("StackQueue"), "Got: {}", result);
}

#[test]
fn test_s12_queue_priority() {
    let code = r#"
class PQueue:
    def __init__(self):
        self.q = queue.PriorityQueue()
"#;
    let result = transpile(code);
    assert!(result.contains("PQueue"), "Got: {}", result);
}

// ===== Asyncio module =====

#[test]
fn test_s12_asyncio_event() {
    let code = r#"
class Waiter:
    def __init__(self):
        self.event = asyncio.Event()
"#;
    let result = transpile(code);
    assert!(result.contains("Waiter"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_lock() {
    let code = r#"
class SafeCounter:
    def __init__(self):
        self.lock = asyncio.Lock()
"#;
    let result = transpile(code);
    assert!(result.contains("SafeCounter"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_semaphore() {
    let code = r#"
class RateLimiter:
    def __init__(self, n: int):
        self.sem = asyncio.Semaphore(n)
"#;
    let result = transpile(code);
    assert!(result.contains("RateLimiter"), "Got: {}", result);
}

#[test]
fn test_s12_asyncio_sleep() {
    let code = r#"
async def delay(seconds: float):
    await asyncio.sleep(seconds)
"#;
    let result = transpile(code);
    assert!(result.contains("fn delay"), "Got: {}", result);
}

// ===== Time module =====

#[test]
fn test_s12_time_time() {
    let code = r#"
class Timer:
    def __init__(self):
        self.start = time.time()
"#;
    let result = transpile(code);
    assert!(result.contains("Timer"), "Got: {}", result);
}

#[test]
fn test_s12_time_sleep() {
    let code = r#"
def wait(seconds: float):
    time.sleep(seconds)
"#;
    let result = transpile(code);
    assert!(result.contains("fn wait"), "Got: {}", result);
}

// ===== JSON module =====

#[test]
fn test_s12_json_loads() {
    let code = r#"
class Parser:
    def parse(self, text: str) -> dict:
        return json.loads(text)
"#;
    let result = transpile(code);
    assert!(result.contains("Parser"), "Got: {}", result);
}

#[test]
fn test_s12_json_dumps() {
    let code = r#"
class Serializer:
    def serialize(self, data: dict) -> str:
        return json.dumps(data)
"#;
    let result = transpile(code);
    assert!(result.contains("Serializer"), "Got: {}", result);
}

// ===== Base64 module =====

#[test]
fn test_s12_base64_encode() {
    let code = r#"
class Encoder:
    def encode(self, data: str) -> str:
        return base64.b64encode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("Encoder"), "Got: {}", result);
}

#[test]
fn test_s12_base64_decode() {
    let code = r#"
class Decoder:
    def decode(self, data: str) -> str:
        return base64.b64decode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("Decoder"), "Got: {}", result);
}

#[test]
fn test_s12_base64_urlsafe_encode() {
    let code = r#"
class URLEncoder:
    def encode(self, data: str) -> str:
        return base64.urlsafe_b64encode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("URLEncoder"), "Got: {}", result);
}

#[test]
fn test_s12_base64_urlsafe_decode() {
    let code = r#"
class URLDecoder:
    def decode(self, data: str) -> str:
        return base64.urlsafe_b64decode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("URLDecoder"), "Got: {}", result);
}

#[test]
fn test_s12_base64_b32encode() {
    let code = r#"
class B32Encoder:
    def encode(self, data: str) -> str:
        return base64.b32encode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("B32Encoder"), "Got: {}", result);
}

#[test]
fn test_s12_base64_b16encode() {
    let code = r#"
class HexEncoder:
    def encode(self, data: str) -> str:
        return base64.b16encode(data)
"#;
    let result = transpile(code);
    assert!(result.contains("HexEncoder"), "Got: {}", result);
}

// ===== Colorsys module =====

#[test]
fn test_s12_colorsys_rgb_to_hsv() {
    let code = r#"
class ColorConverter:
    def to_hsv(self, r: float, g: float, b: float) -> tuple:
        return colorsys.rgb_to_hsv(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("ColorConverter"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hsv_to_rgb() {
    let code = r#"
class ColorConverter:
    def to_rgb(self, h: float, s: float, v: float) -> tuple:
        return colorsys.hsv_to_rgb(h, s, v)
"#;
    let result = transpile(code);
    assert!(result.contains("ColorConverter"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_rgb_to_hls() {
    let code = r#"
class ColorConverter:
    def to_hls(self, r: float, g: float, b: float) -> tuple:
        return colorsys.rgb_to_hls(r, g, b)
"#;
    let result = transpile(code);
    assert!(result.contains("ColorConverter"), "Got: {}", result);
}

#[test]
fn test_s12_colorsys_hls_to_rgb() {
    let code = r#"
class ColorConverter:
    def from_hls(self, h: float, l: float, s: float) -> tuple:
        return colorsys.hls_to_rgb(h, l, s)
"#;
    let result = transpile(code);
    assert!(result.contains("ColorConverter"), "Got: {}", result);
}

// ===== int.from_bytes class method =====

#[test]
fn test_s12_int_from_bytes_big_endian() {
    let code = r#"
class ByteReader:
    def read_int(self, data: bytes) -> int:
        return int.from_bytes(data, "big")
"#;
    let result = transpile(code);
    assert!(result.contains("ByteReader"), "Got: {}", result);
}

#[test]
fn test_s12_int_from_bytes_little_endian() {
    let code = r#"
class ByteReader:
    def read_int_le(self, data: bytes) -> int:
        return int.from_bytes(data, "little")
"#;
    let result = transpile(code);
    assert!(result.contains("ByteReader"), "Got: {}", result);
}

// ===== dict.fromkeys class method =====

#[test]
fn test_s12_dict_fromkeys_with_default() {
    let code = r#"
class Initializer:
    def init_dict(self, keys: list) -> dict:
        return dict.fromkeys(keys, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("Initializer"), "Got: {}", result);
}

#[test]
fn test_s12_dict_fromkeys_no_default() {
    let code = r#"
class Initializer:
    def init_dict_none(self, keys: list) -> dict:
        return dict.fromkeys(keys)
"#;
    let result = transpile(code);
    assert!(result.contains("Initializer"), "Got: {}", result);
}

// ===== Threading module =====

#[test]
fn test_s12_threading_lock() {
    let code = r#"
class ThreadSafe:
    def __init__(self):
        self.lock = threading.Lock()
"#;
    let result = transpile(code);
    assert!(result.contains("ThreadSafe"), "Got: {}", result);
}

#[test]
fn test_s12_threading_semaphore() {
    let code = r#"
class Throttle:
    def __init__(self, n: int):
        self.sem = threading.Semaphore(n)
"#;
    let result = transpile(code);
    assert!(result.contains("Throttle"), "Got: {}", result);
}

// ===== Sys module methods =====

#[test]
fn test_s12_sys_exit_in_method() {
    let code = r#"
class App:
    def quit(self, code: int):
        sys.exit(code)
"#;
    let result = transpile(code);
    assert!(result.contains("App"), "Got: {}", result);
}

#[test]
fn test_s12_sys_platform() {
    let code = r#"
class Platform:
    def get_os(self) -> str:
        return sys.platform
"#;
    let result = transpile(code);
    assert!(result.contains("Platform"), "Got: {}", result);
}

#[test]
fn test_s12_sys_version() {
    let code = r#"
class Info:
    def get_version(self) -> str:
        return sys.version
"#;
    let result = transpile(code);
    assert!(result.contains("Info"), "Got: {}", result);
}

#[test]
fn test_s12_sys_getsizeof() {
    let code = r#"
class MemoryTracker:
    def size(self, obj) -> int:
        return sys.getsizeof(obj)
"#;
    let result = transpile(code);
    assert!(result.contains("MemoryTracker"), "Got: {}", result);
}

#[test]
fn test_s12_sys_stdin_stdout() {
    let code = r#"
class IOHandler:
    def get_stdin(self):
        return sys.stdin

    def get_stdout(self):
        return sys.stdout

    def get_stderr(self):
        return sys.stderr
"#;
    let result = transpile(code);
    assert!(result.contains("IOHandler"), "Got: {}", result);
}

// ===== OS environ methods =====

#[test]
fn test_s12_os_environ_get() {
    let code = r#"
class Config:
    def get_env(self, key: str) -> str:
        return os.environ.get(key, "")
"#;
    let result = transpile(code);
    assert!(result.contains("Config"), "Got: {}", result);
}

// ===== Regex methods in class context =====

#[test]
fn test_s12_re_search_in_method() {
    let code = r#"
class Matcher:
    def find(self, pattern: str, text: str):
        return re.search(pattern, text)
"#;
    let result = transpile(code);
    assert!(result.contains("Matcher"), "Got: {}", result);
}

#[test]
fn test_s12_re_findall_in_method() {
    let code = r#"
class Extractor:
    def extract(self, pattern: str, text: str) -> list:
        return re.findall(pattern, text)
"#;
    let result = transpile(code);
    assert!(result.contains("Extractor"), "Got: {}", result);
}

#[test]
fn test_s12_re_sub_in_method() {
    let code = r#"
class Replacer:
    def replace(self, pattern: str, repl: str, text: str) -> str:
        return re.sub(pattern, repl, text)
"#;
    let result = transpile(code);
    assert!(result.contains("Replacer"), "Got: {}", result);
}

#[test]
fn test_s12_re_split_in_method() {
    let code = r#"
class Splitter:
    def split(self, pattern: str, text: str) -> list:
        return re.split(pattern, text)
"#;
    let result = transpile(code);
    assert!(result.contains("Splitter"), "Got: {}", result);
}

#[test]
fn test_s12_re_match_in_method() {
    let code = r#"
class Validator:
    def validate(self, pattern: str, text: str) -> bool:
        m = re.match(pattern, text)
        return m is not None
"#;
    let result = transpile(code);
    assert!(result.contains("Validator"), "Got: {}", result);
}

#[test]
fn test_s12_re_fullmatch_in_method() {
    let code = r#"
class StrictValidator:
    def validate(self, pattern: str, text: str) -> bool:
        m = re.fullmatch(pattern, text)
        return m is not None
"#;
    let result = transpile(code);
    assert!(result.contains("StrictValidator"), "Got: {}", result);
}

#[test]
fn test_s12_re_compile() {
    let code = r#"
class RegexCache:
    def __init__(self, pattern: str):
        self.pattern = re.compile(pattern)
"#;
    let result = transpile(code);
    assert!(result.contains("RegexCache"), "Got: {}", result);
}
