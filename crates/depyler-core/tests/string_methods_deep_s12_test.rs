//! Session 12 Batch 23: Deep coverage for string method conversions
//!
//! Targets expr_gen_instance_methods.rs cold paths:
//! - Character iteration methods (isupper/islower/isprintable etc. on char vars)
//! - istitle, isnumeric, isascii, isdecimal, isidentifier
//! - hex() string method
//! - format() with 0, 1, and multiple args
//! - casefold()
//! - String method chains

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

// ===== Character iteration string methods =====

#[test]
fn test_s12_char_iter_isupper() {
    let code = r#"
def count_upper(text: str) -> int:
    count = 0
    for c in text:
        if c.isupper():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_upper"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_islower() {
    let code = r#"
def count_lower(text: str) -> int:
    count = 0
    for c in text:
        if c.islower():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_lower"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isdigit() {
    let code = r#"
def count_digits(text: str) -> int:
    count = 0
    for c in text:
        if c.isdigit():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_digits"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isalpha() {
    let code = r#"
def count_alpha(text: str) -> int:
    count = 0
    for c in text:
        if c.isalpha():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_alpha"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isalnum() {
    let code = r#"
def count_alnum(text: str) -> int:
    count = 0
    for c in text:
        if c.isalnum():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_alnum"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isspace() {
    let code = r#"
def count_spaces(text: str) -> int:
    count = 0
    for c in text:
        if c.isspace():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_spaces"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isprintable() {
    let code = r#"
def all_printable(text: str) -> bool:
    for c in text:
        if not c.isprintable():
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_printable"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isnumeric() {
    let code = r#"
def count_numeric(text: str) -> int:
    count = 0
    for c in text:
        if c.isnumeric():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_numeric"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isascii() {
    let code = r#"
def all_ascii(text: str) -> bool:
    for c in text:
        if not c.isascii():
            return False
    return True
"#;
    let result = transpile(code);
    assert!(result.contains("fn all_ascii"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_isdecimal() {
    let code = r#"
def count_decimal(text: str) -> int:
    count = 0
    for c in text:
        if c.isdecimal():
            count += 1
    return count
"#;
    let result = transpile(code);
    assert!(result.contains("fn count_decimal"), "Got: {}", result);
}

// ===== String-level is* methods (non-char-iter) =====

#[test]
fn test_s12_string_istitle() {
    let code = r#"
def check_title(text: str) -> bool:
    return text.istitle()
"#;
    let result = transpile(code);
    assert!(result.contains("fn check_title"), "Got: {}", result);
}

#[test]
fn test_s12_string_isidentifier() {
    let code = r#"
def is_valid_name(text: str) -> bool:
    return text.isidentifier()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_valid_name"), "Got: {}", result);
}

#[test]
fn test_s12_string_isnumeric_whole() {
    let code = r#"
def is_all_numeric(text: str) -> bool:
    return text.isnumeric()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_all_numeric"), "Got: {}", result);
}

#[test]
fn test_s12_string_isascii_whole() {
    let code = r#"
def is_ascii_text(text: str) -> bool:
    return text.isascii()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_ascii_text"), "Got: {}", result);
}

#[test]
fn test_s12_string_isdecimal_whole() {
    let code = r#"
def is_decimal_string(text: str) -> bool:
    return text.isdecimal()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_decimal_string"), "Got: {}", result);
}

#[test]
fn test_s12_string_isprintable_whole() {
    let code = r#"
def is_printable(text: str) -> bool:
    return text.isprintable()
"#;
    let result = transpile(code);
    assert!(result.contains("fn is_printable"), "Got: {}", result);
}

// ===== casefold =====

#[test]
fn test_s12_string_casefold() {
    let code = r#"
def normalize(text: str) -> str:
    return text.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("fn normalize"), "Got: {}", result);
}

// ===== hex() method =====

#[test]
fn test_s12_string_hex() {
    let code = r#"
def to_hex(text: str) -> str:
    return text.hex()
"#;
    let result = transpile(code);
    assert!(result.contains("fn to_hex"), "Got: {}", result);
}

// ===== format() with various arg counts =====

#[test]
fn test_s12_format_no_args() {
    let code = r#"
def get_template() -> str:
    template = "Hello World"
    return template.format()
"#;
    let result = transpile(code);
    assert!(result.contains("fn get_template"), "Got: {}", result);
}

#[test]
fn test_s12_format_one_arg() {
    let code = r#"
def greet(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
    let result = transpile(code);
    assert!(result.contains("fn greet"), "Got: {}", result);
}

#[test]
fn test_s12_format_multiple_args() {
    let code = r#"
def describe(name: str, age: int) -> str:
    return "{} is {} years old".format(name, age)
"#;
    let result = transpile(code);
    assert!(result.contains("fn describe"), "Got: {}", result);
}

// ===== Complex char iteration patterns =====

#[test]
fn test_s12_char_iter_with_ord() {
    let code = r#"
def sum_char_codes(text: str) -> int:
    total = 0
    for c in text:
        total += ord(c)
    return total
"#;
    let result = transpile(code);
    assert!(result.contains("fn sum_char_codes"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_with_upper() {
    let code = r#"
def transform_chars(text: str) -> str:
    result = ""
    for c in text:
        if c.islower():
            result += c.upper()
        else:
            result += c.lower()
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn transform_chars"), "Got: {}", result);
}

#[test]
fn test_s12_char_iter_multiple_checks() {
    let code = r#"
def categorize_chars(text: str) -> dict:
    upper = 0
    lower = 0
    digit = 0
    other = 0
    for c in text:
        if c.isupper():
            upper += 1
        elif c.islower():
            lower += 1
        elif c.isdigit():
            digit += 1
        else:
            other += 1
    return {"upper": upper, "lower": lower, "digit": digit, "other": other}
"#;
    let result = transpile(code);
    assert!(result.contains("fn categorize_chars"), "Got: {}", result);
}

// ===== String methods on class attributes =====

#[test]
fn test_s12_class_string_methods() {
    let code = r#"
class StringProcessor:
    def __init__(self, text: str):
        self.text = text

    def is_valid_identifier(self) -> bool:
        return self.text.isidentifier()

    def is_title_case(self) -> bool:
        return self.text.istitle()

    def to_hex(self) -> str:
        return self.text.hex()

    def normalize(self) -> str:
        return self.text.casefold()
"#;
    let result = transpile(code);
    assert!(result.contains("StringProcessor"), "Got: {}", result);
}

// ===== Set operations (union, intersection, difference) =====

#[test]
fn test_s12_set_bitwise_union() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a | b
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sets"), "Got: {}", result);
}

#[test]
fn test_s12_set_bitwise_intersection() {
    let code = r#"
def common_elements(a: set, b: set) -> set:
    return a & b
"#;
    let result = transpile(code);
    assert!(result.contains("fn common_elements"), "Got: {}", result);
}

#[test]
fn test_s12_set_bitwise_difference() {
    let code = r#"
def only_in_first(a: set, b: set) -> set:
    return a - b
"#;
    let result = transpile(code);
    assert!(result.contains("fn only_in_first"), "Got: {}", result);
}

#[test]
fn test_s12_set_bitwise_symmetric_diff() {
    let code = r#"
def exclusive_elements(a: set, b: set) -> set:
    return a ^ b
"#;
    let result = transpile(code);
    assert!(result.contains("fn exclusive_elements"), "Got: {}", result);
}

// ===== Complex real-world pattern =====

#[test]
fn test_s12_password_validator() {
    let code = r#"
def validate_password(password: str) -> bool:
    if len(password) < 8:
        return False
    has_upper = False
    has_lower = False
    has_digit = False
    for c in password:
        if c.isupper():
            has_upper = True
        if c.islower():
            has_lower = True
        if c.isdigit():
            has_digit = True
    return has_upper and has_lower and has_digit
"#;
    let result = transpile(code);
    assert!(result.contains("fn validate_password"), "Got: {}", result);
}

#[test]
fn test_s12_sanitize_input() {
    let code = r#"
def sanitize(text: str) -> str:
    result = ""
    for c in text:
        if c.isalnum() or c.isspace():
            result += c
    return result.strip()
"#;
    let result = transpile(code);
    assert!(result.contains("fn sanitize"), "Got: {}", result);
}
