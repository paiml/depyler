//! Session 12 Batch 26: Regex compiled methods, sys I/O, and match object cold paths
//!
//! Targets expr_gen_instance_methods.rs cold paths:
//! - Compiled regex methods: findall, match, search, group, groups, start, end, span
//! - sys.stdout.write(), sys.stdout.flush()
//! - sys.stdin.readline(), sys.stdin.readlines()
//! - os.path methods in class context
//! - Hasher hexdigest/update in various patterns

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

// ===== Compiled regex object methods =====

#[test]
fn test_s12_compiled_regex_findall() {
    let code = r#"
class RegexMatcher:
    def __init__(self, pattern: str):
        self.pattern = re.compile(pattern)

    def find_all(self, text: str) -> list:
        return self.pattern.findall(text)
"#;
    let result = transpile(code);
    assert!(result.contains("RegexMatcher"), "Got: {}", result);
}

#[test]
fn test_s12_compiled_regex_match() {
    let code = r#"
class RegexMatcher:
    def __init__(self, pattern: str):
        self.pattern = re.compile(pattern)

    def check(self, text: str):
        return self.pattern.match(text)
"#;
    let result = transpile(code);
    assert!(result.contains("RegexMatcher"), "Got: {}", result);
}

#[test]
fn test_s12_compiled_regex_search() {
    let code = r#"
class RegexMatcher:
    def __init__(self, pattern: str):
        self.pattern = re.compile(pattern)

    def search(self, text: str):
        return self.pattern.search(text)
"#;
    let result = transpile(code);
    assert!(result.contains("RegexMatcher"), "Got: {}", result);
}

#[test]
fn test_s12_match_group_default() {
    let code = r#"
def extract_match(text: str, pattern: str) -> str:
    m = re.search(pattern, text)
    if m is not None:
        return m.group()
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn extract_match"), "Got: {}", result);
}

#[test]
fn test_s12_match_group_zero() {
    let code = r#"
def extract_match_zero(text: str, pattern: str) -> str:
    m = re.search(pattern, text)
    if m is not None:
        return m.group(0)
    return ""
"#;
    let result = transpile(code);
    assert!(result.contains("fn extract_match_zero"), "Got: {}", result);
}

#[test]
fn test_s12_match_start_end() {
    let code = r#"
def match_position(text: str, pattern: str) -> tuple:
    m = re.search(pattern, text)
    if m is not None:
        return (m.start(), m.end())
    return (0, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn match_position"), "Got: {}", result);
}

#[test]
fn test_s12_match_span() {
    let code = r#"
def match_span(text: str, pattern: str) -> tuple:
    m = re.search(pattern, text)
    if m is not None:
        return m.span()
    return (0, 0)
"#;
    let result = transpile(code);
    assert!(result.contains("fn match_span"), "Got: {}", result);
}

#[test]
fn test_s12_match_groups() {
    let code = r#"
def match_groups(text: str, pattern: str) -> list:
    m = re.search(pattern, text)
    if m is not None:
        return m.groups()
    return []
"#;
    let result = transpile(code);
    assert!(result.contains("fn match_groups"), "Got: {}", result);
}

// ===== sys.stdout/stderr write and flush =====

#[test]
fn test_s12_stdout_write() {
    let code = r#"
class Logger:
    def log(self, msg: str):
        sys.stdout.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("Logger"), "Got: {}", result);
}

#[test]
fn test_s12_stderr_write() {
    let code = r#"
class ErrorLogger:
    def error(self, msg: str):
        sys.stderr.write(msg)
"#;
    let result = transpile(code);
    assert!(result.contains("ErrorLogger"), "Got: {}", result);
}

#[test]
fn test_s12_stdout_flush() {
    let code = r#"
class Flusher:
    def flush(self):
        sys.stdout.flush()
"#;
    let result = transpile(code);
    assert!(result.contains("Flusher"), "Got: {}", result);
}

// ===== sys.stdin read methods =====

#[test]
fn test_s12_stdin_readline() {
    let code = r#"
class Reader:
    def read_line(self) -> str:
        return sys.stdin.readline()
"#;
    let result = transpile(code);
    assert!(result.contains("Reader"), "Got: {}", result);
}

#[test]
fn test_s12_stdin_readlines() {
    let code = r#"
class BatchReader:
    def read_all(self) -> list:
        return sys.stdin.readlines()
"#;
    let result = transpile(code);
    assert!(result.contains("BatchReader"), "Got: {}", result);
}

#[test]
fn test_s12_stdin_read() {
    let code = r#"
class FullReader:
    def read_all(self) -> str:
        return sys.stdin.read()
"#;
    let result = transpile(code);
    assert!(result.contains("FullReader"), "Got: {}", result);
}

// ===== os.path methods in class context =====

#[test]
fn test_s12_os_path_join_in_method() {
    let code = r#"
class PathHelper:
    def __init__(self, base: str):
        self.base = base

    def join(self, name: str) -> str:
        return os.path.join(self.base, name)
"#;
    let result = transpile(code);
    assert!(result.contains("PathHelper"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_exists_in_method() {
    let code = r#"
class FileChecker:
    def exists(self, path: str) -> bool:
        return os.path.exists(path)
"#;
    let result = transpile(code);
    assert!(result.contains("FileChecker"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_basename_in_method() {
    let code = r#"
class PathParser:
    def basename(self, path: str) -> str:
        return os.path.basename(path)
"#;
    let result = transpile(code);
    assert!(result.contains("PathParser"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_dirname_in_method() {
    let code = r#"
class PathParser:
    def dirname(self, path: str) -> str:
        return os.path.dirname(path)
"#;
    let result = transpile(code);
    assert!(result.contains("PathParser"), "Got: {}", result);
}

#[test]
fn test_s12_os_path_expanduser_in_method() {
    let code = r#"
class PathExpander:
    def expand(self, path: str) -> str:
        return os.path.expanduser(path)
"#;
    let result = transpile(code);
    assert!(result.contains("PathExpander"), "Got: {}", result);
}

// ===== Complex regex patterns =====

#[test]
fn test_s12_regex_search_with_string_literals() {
    let code = r#"
def find_email(text: str) -> bool:
    m = re.search("[a-zA-Z0-9]+@[a-zA-Z]+\\.[a-zA-Z]+", text)
    return m is not None
"#;
    let result = transpile(code);
    assert!(result.contains("fn find_email"), "Got: {}", result);
}

#[test]
fn test_s12_regex_finditer() {
    let code = r#"
def count_words(text: str) -> int:
    matches = re.finditer("[a-zA-Z]+", text)
    count = 0
    for m in matches:
        count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_words"), "Got: {}", result);
}

#[test]
fn test_s12_regex_subn() {
    let code = r#"
def replace_and_count(text: str) -> tuple:
    return re.subn("[0-9]+", "NUM", text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn replace_and_count"), "Got: {}", result);
}

#[test]
fn test_s12_regex_escape() {
    let code = r#"
def escape_pattern(text: str) -> str:
    return re.escape(text)
"#;
    let result = transpile(code);
    assert!(result.contains("fn escape_pattern"), "Got: {}", result);
}

// ===== Complex class combining IO and regex =====

#[test]
fn test_s12_log_analyzer() {
    let code = r##"
class LogAnalyzer:
    def __init__(self, pattern: str):
        self.pattern = re.compile(pattern)
        self.matches = []

    def analyze(self, text: str) -> int:
        results = self.pattern.findall(text)
        for r in results:
            self.matches.append(r)
        count = len(results)
        return count

    def get_count(self) -> int:
        return len(self.matches)
"##;
    let result = transpile(code);
    assert!(result.contains("LogAnalyzer"), "Got: {}", result);
}

// ===== File I/O patterns =====

#[test]
fn test_s12_file_reader_class() {
    let code = r##"
class FileReader:
    def __init__(self, path: str):
        self.path = path

    def read(self) -> str:
        f = open(self.path, "r")
        return ""

    def write(self, data: str):
        f = open(self.path, "w")
"##;
    let result = transpile(code);
    assert!(result.contains("FileReader"), "Got: {}", result);
}

// ===== parse_args stub =====

#[test]
fn test_s12_parse_args_stub() {
    let code = r#"
class CLI:
    def run(self):
        args = self.parser.parse_args()
"#;
    let result = transpile(code);
    assert!(result.contains("CLI"), "Got: {}", result);
}
