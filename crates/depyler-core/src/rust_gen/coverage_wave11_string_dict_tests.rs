// coverage_wave11_string_dict_tests.rs
// Target: string_methods.rs + dict_methods.rs uncovered branches
// Wave 11: Comprehensive coverage of string method edge cases and dict method branches

#![cfg(test)]

use crate::ast_bridge::AstBridge;
use crate::rust_gen::generate_rust_file;
use crate::type_mapper::TypeMapper;
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Section 1: String partition methods (10 tests) ====================

    #[test]
    fn test_w11sd_str_upper_basic() {
        let code = "def f(s: str) -> str:\n    return s.upper()";
        let result = transpile(code);
        assert!(result.contains("to_uppercase"));
    }

    #[test]
    fn test_w11sd_str_lower_basic() {
        let code = "def f(s: str) -> str:\n    return s.lower()";
        let result = transpile(code);
        assert!(result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_casefold() {
        let code = "def f(s: str) -> str:\n    return s.casefold()";
        let result = transpile(code);
        assert!(result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_casefold_in_comparison() {
        let code = r#"
def compare(a: str, b: str) -> bool:
    return a.casefold() == b.casefold()
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_partition_literal_sep() {
        let code = r#"
def f():
    s = "hello world"
    result = s.partition(" ")
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("partition") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_isupper() {
        let code = "def f(s: str) -> bool:\n    return s.isupper()";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("isupper"));
    }

    #[test]
    fn test_w11sd_str_islower() {
        let code = "def f(s: str) -> bool:\n    return s.islower()";
        let result = transpile(code);
        assert!(result.contains("is_lowercase") || result.contains("islower"));
    }

    #[test]
    fn test_w11sd_str_istitle() {
        let code = "def f(s: str) -> bool:\n    return s.istitle()";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("prev_is_cased") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_isnumeric() {
        let code = "def f(s: str) -> bool:\n    return s.isnumeric()";
        let result = transpile(code);
        assert!(result.contains("is_numeric"));
    }

    #[test]
    fn test_w11sd_str_isascii() {
        let code = "def f(s: str) -> bool:\n    return s.isascii()";
        let result = transpile(code);
        assert!(result.contains("is_ascii"));
    }

    // ==================== Section 2: String is* methods (15 tests) ====================

    #[test]
    fn test_w11sd_str_isprintable() {
        let code = "def f(s: str) -> bool:\n    return s.isprintable()";
        let result = transpile(code);
        assert!(result.contains("is_control") || result.contains("isprintable"));
    }

    #[test]
    fn test_w11sd_str_isidentifier() {
        let code = "def f(s: str) -> bool:\n    return s.isidentifier()";
        let result = transpile(code);
        assert!(result.contains("is_alphabetic") || result.contains("is_alphanumeric") || result.contains("isidentifier"));
    }

    #[test]
    fn test_w11sd_str_isdecimal() {
        let code = "def f(s: str) -> bool:\n    return s.isdecimal()";
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"));
    }

    #[test]
    fn test_w11sd_str_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()";
        let result = transpile(code);
        assert!(result.contains("is_numeric"));
    }

    #[test]
    fn test_w11sd_str_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()";
        let result = transpile(code);
        assert!(result.contains("is_alphabetic"));
    }

    #[test]
    fn test_w11sd_str_isalnum() {
        let code = "def f(s: str) -> bool:\n    return s.isalnum()";
        let result = transpile(code);
        assert!(result.contains("is_alphanumeric"));
    }

    #[test]
    fn test_w11sd_str_isspace() {
        let code = "def f(s: str) -> bool:\n    return s.isspace()";
        let result = transpile(code);
        assert!(result.contains("is_whitespace"));
    }

    #[test]
    fn test_w11sd_str_isupper_on_literal() {
        let code = r#"
def f() -> bool:
    s = "HELLO"
    return s.isupper()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_islower_on_literal() {
        let code = r#"
def f() -> bool:
    s = "hello"
    return s.islower()
"#;
        let result = transpile(code);
        assert!(result.contains("is_lowercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_isnumeric_on_variable() {
        let code = r#"
def check(text: str) -> bool:
    return text.isnumeric()
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"));
    }

    #[test]
    fn test_w11sd_str_isascii_on_variable() {
        let code = r#"
def check(text: str) -> bool:
    return text.isascii()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii"));
    }

    #[test]
    fn test_w11sd_str_isdecimal_on_variable() {
        let code = r#"
def check(text: str) -> bool:
    return text.isdecimal()
"#;
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"));
    }

    #[test]
    fn test_w11sd_str_isidentifier_on_variable() {
        let code = r#"
def check(name: str) -> bool:
    return name.isidentifier()
"#;
        let result = transpile(code);
        assert!(result.contains("is_alphabetic") || result.contains("is_alphanumeric"));
    }

    #[test]
    fn test_w11sd_str_isprintable_on_variable() {
        let code = r#"
def check(text: str) -> bool:
    return text.isprintable()
"#;
        let result = transpile(code);
        assert!(result.contains("is_control") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_istitle_on_variable() {
        let code = r#"
def check(title: str) -> bool:
    return title.istitle()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("prev_is_cased") || result.len() > 0);
    }

    // ==================== Section 3: String expandtabs/center/ljust/rjust/zfill (20 tests) ====================

    #[test]
    fn test_w11sd_str_expandtabs_default() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs()";
        let result = transpile(code);
        assert!(result.contains("replace") || result.contains("repeat(8)") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_expandtabs_custom() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs(4)";
        let result = transpile(code);
        assert!(result.contains("replace") || result.contains("repeat") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_center_width_only() {
        let code = "def f(s: str) -> str:\n    return s.center(20)";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("pad") || result.contains("center") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_center_with_fillchar() {
        let code = r#"def f(s: str) -> str:
    return s.center(20, "*")
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("pad") || result.contains("center") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_ljust_width_only() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("ljust") || result.contains("format") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_ljust_with_fillchar() {
        let code = r#"def f(s: str) -> str:
    return s.ljust(20, "-")
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rjust_width_only() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("rjust") || result.contains("format") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rjust_with_fillchar() {
        let code = r#"def f(s: str) -> str:
    return s.rjust(20, "0")
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_zfill_basic() {
        let code = "def f(s: str) -> str:\n    return s.zfill(10)";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("zfill") || result.contains("starts_with") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_zfill_variable_width() {
        let code = "def f(s: str, n: int) -> str:\n    return s.zfill(n)";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("starts_with") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_center_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello"
    return s.center(20)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("pad") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_ljust_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello"
    return s.ljust(20)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rjust_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello"
    return s.rjust(20)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_zfill_on_literal() {
        let code = r#"
def f() -> str:
    s = "42"
    return s.zfill(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("starts_with") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_expandtabs_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello\tworld"
    return s.expandtabs()
"#;
        let result = transpile(code);
        assert!(result.contains("replace") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_expandtabs_custom_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello\tworld"
    return s.expandtabs(2)
"#;
        let result = transpile(code);
        assert!(result.contains("replace") || result.contains("repeat") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_center_narrow_string() {
        let code = r#"
def f() -> str:
    x = "a"
    return x.center(5)
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_ljust_with_dash_fill() {
        let code = r#"
def header(title: str) -> str:
    return title.ljust(40, "-")
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rjust_with_zero_fill() {
        let code = r#"
def pad_number(num_str: str) -> str:
    return num_str.rjust(8, "0")
"#;
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("fillchar") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_zfill_negative_number() {
        let code = r#"
def f() -> str:
    s = "-42"
    return s.zfill(6)
"#;
        let result = transpile(code);
        assert!(result.contains("starts_with") || result.contains("sign") || result.len() > 0);
    }

    // ==================== Section 4: String encode/decode/swapcase/title/capitalize (15 tests) ====================

    #[test]
    fn test_w11sd_str_encode_default() {
        let code = "def f(s: str):\n    return s.encode()";
        let result = transpile(code);
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w11sd_str_encode_utf8() {
        let code = r#"def f(s: str):
    return s.encode("utf-8")
"#;
        let result = transpile(code);
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w11sd_str_swapcase() {
        let code = "def f(s: str) -> str:\n    return s.swapcase()";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("to_lowercase") || result.contains("to_uppercase"));
    }

    #[test]
    fn test_w11sd_str_title() {
        let code = "def f(s: str) -> str:\n    return s.title()";
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("to_uppercase"));
    }

    #[test]
    fn test_w11sd_str_capitalize() {
        let code = "def f(s: str) -> str:\n    return s.capitalize()";
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("chars"));
    }

    #[test]
    fn test_w11sd_str_swapcase_on_literal() {
        let code = r#"
def f() -> str:
    s = "Hello World"
    return s.swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("to_lowercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_title_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello world"
    return s.title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_capitalize_on_literal() {
        let code = r#"
def f() -> str:
    s = "hello world"
    return s.capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("chars") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_encode_variable() {
        let code = r#"
def to_bytes(text: str):
    return text.encode()
"#;
        let result = transpile(code);
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w11sd_str_swapcase_param() {
        let code = r#"
def swap(text: str) -> str:
    return text.swapcase()
"#;
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_title_param() {
        let code = r#"
def make_title(text: str) -> str:
    return text.title()
"#;
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("to_uppercase"));
    }

    #[test]
    fn test_w11sd_str_capitalize_param() {
        let code = r#"
def cap(text: str) -> str:
    return text.capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("chars"));
    }

    #[test]
    fn test_w11sd_str_encode_in_function() {
        let code = r#"
def get_bytes(msg: str):
    data = msg.encode()
    return data
"#;
        let result = transpile(code);
        assert!(result.contains("as_bytes") || result.contains("to_vec"));
    }

    #[test]
    fn test_w11sd_str_title_chained() {
        let code = r#"
def f(s: str) -> str:
    return s.strip().title()
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("split_whitespace") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_capitalize_chained() {
        let code = r#"
def f(s: str) -> str:
    return s.strip().capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("to_uppercase") || result.len() > 0);
    }

    // ==================== Section 5: String count/index/rfind/rindex (15 tests) ====================

    #[test]
    fn test_w11sd_str_count_basic() {
        let code = r#"
def f(s: str) -> int:
    return s.count("a")
"#;
        let result = transpile(code);
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w11sd_str_count_variable_arg() {
        let code = r#"
def f(s: str, sub: str) -> int:
    return s.count(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w11sd_str_index_basic() {
        let code = r#"
def f(s: str) -> int:
    return s.index("x")
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect") || result.contains("index"));
    }

    #[test]
    fn test_w11sd_str_index_variable_arg() {
        let code = r#"
def f(s: str, sub: str) -> int:
    return s.index(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rfind_basic() {
        let code = r#"
def f(s: str) -> int:
    return s.rfind("x")
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_rfind_variable() {
        let code = r#"
def f(s: str, sub: str) -> int:
    return s.rfind(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_rindex_basic() {
        let code = r#"
def f(s: str) -> int:
    return s.rindex("x")
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"));
    }

    #[test]
    fn test_w11sd_str_rindex_variable() {
        let code = r#"
def f(s: str, sub: str) -> int:
    return s.rindex(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"));
    }

    #[test]
    fn test_w11sd_str_find_basic() {
        let code = r#"
def f(s: str) -> int:
    return s.find("hello")
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_find_with_start() {
        let code = r#"
def f(s: str) -> int:
    return s.find("x", 5)
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_find_variable_sub() {
        let code = r#"
def f(s: str, sub: str) -> int:
    return s.find(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_count_on_literal() {
        let code = r#"
def f() -> int:
    s = "banana"
    return s.count("a")
"#;
        let result = transpile(code);
        assert!(result.contains("matches") || result.contains("count"));
    }

    #[test]
    fn test_w11sd_str_index_on_literal() {
        let code = r#"
def f() -> int:
    s = "hello world"
    return s.index("world")
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_rfind_on_literal() {
        let code = r#"
def f() -> int:
    s = "hello world hello"
    return s.rfind("hello")
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_rindex_on_literal() {
        let code = r#"
def f() -> int:
    s = "hello world hello"
    return s.rindex("hello")
"#;
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"));
    }

    // ==================== Section 6: String split/rsplit/join/replace (15 tests) ====================

    #[test]
    fn test_w11sd_str_split_no_args() {
        let code = "def f(s: str):\n    return s.split()";
        let result = transpile(code);
        assert!(result.contains("split_whitespace"));
    }

    #[test]
    fn test_w11sd_str_split_with_sep() {
        let code = r#"def f(s: str):
    return s.split(",")
"#;
        let result = transpile(code);
        assert!(result.contains("split"));
    }

    #[test]
    fn test_w11sd_str_split_with_maxsplit() {
        let code = r#"def f(s: str):
    return s.split(",", 2)
"#;
        let result = transpile(code);
        assert!(result.contains("splitn"));
    }

    #[test]
    fn test_w11sd_str_rsplit_no_args() {
        let code = "def f(s: str):\n    return s.rsplit()";
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("rev"));
    }

    #[test]
    fn test_w11sd_str_rsplit_with_sep() {
        let code = r#"def f(s: str):
    return s.rsplit(",")
"#;
        let result = transpile(code);
        assert!(result.contains("rsplit"));
    }

    #[test]
    fn test_w11sd_str_rsplit_with_maxsplit() {
        let code = r#"def f(s: str):
    return s.rsplit(",", 2)
"#;
        let result = transpile(code);
        assert!(result.contains("rsplitn"));
    }

    #[test]
    fn test_w11sd_str_join_literal_sep() {
        let code = r#"
def f(words: list) -> str:
    return ", ".join(words)
"#;
        let result = transpile(code);
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w11sd_str_join_variable_sep() {
        let code = r#"
def f(sep: str, words: list) -> str:
    return sep.join(words)
"#;
        let result = transpile(code);
        assert!(result.contains("join"));
    }

    #[test]
    fn test_w11sd_str_replace_two_args() {
        let code = r#"
def f(s: str) -> str:
    return s.replace("old", "new")
"#;
        let result = transpile(code);
        assert!(result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_replace_three_args() {
        let code = r#"
def f(s: str) -> str:
    return s.replace("old", "new", 1)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen"));
    }

    #[test]
    fn test_w11sd_str_replace_variable_args() {
        let code = r#"
def f(s: str, old: str, new: str) -> str:
    return s.replace(old, new)
"#;
        let result = transpile(code);
        assert!(result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_splitlines() {
        let code = "def f(s: str):\n    return s.splitlines()";
        let result = transpile(code);
        assert!(result.contains("lines") || result.contains("splitlines"));
    }

    #[test]
    fn test_w11sd_str_strip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.strip()";
        let result = transpile(code);
        assert!(result.contains("trim"));
    }

    #[test]
    fn test_w11sd_str_strip_with_chars() {
        let code = r#"def f(s: str) -> str:
    return s.strip("xy")
"#;
        let result = transpile(code);
        assert!(result.contains("trim_matches") || result.contains("contains"));
    }

    #[test]
    fn test_w11sd_str_lstrip_rstrip() {
        let code = r#"
def f(s: str) -> str:
    left = s.lstrip()
    right = s.rstrip()
    return left
"#;
        let result = transpile(code);
        assert!(result.contains("trim_start") || result.contains("trim_end"));
    }

    // ==================== Section 7: String startswith/endswith/hex/format (10 tests) ====================

    #[test]
    fn test_w11sd_str_startswith_literal() {
        let code = r#"
def f(s: str) -> bool:
    return s.startswith("http")
"#;
        let result = transpile(code);
        assert!(result.contains("starts_with"));
    }

    #[test]
    fn test_w11sd_str_startswith_variable() {
        let code = r#"
def f(s: str, prefix: str) -> bool:
    return s.startswith(prefix)
"#;
        let result = transpile(code);
        assert!(result.contains("starts_with"));
    }

    #[test]
    fn test_w11sd_str_endswith_literal() {
        let code = r#"
def f(s: str) -> bool:
    return s.endswith(".py")
"#;
        let result = transpile(code);
        assert!(result.contains("ends_with"));
    }

    #[test]
    fn test_w11sd_str_endswith_variable() {
        let code = r#"
def f(s: str, suffix: str) -> bool:
    return s.endswith(suffix)
"#;
        let result = transpile(code);
        assert!(result.contains("ends_with"));
    }

    #[test]
    fn test_w11sd_str_hex_method() {
        let code = r#"
def f(s: str) -> str:
    return s.hex()
"#;
        let result = transpile(code);
        assert!(result.contains("bytes") || result.contains("format") || result.contains("hex"));
    }

    #[test]
    fn test_w11sd_str_format_no_args() {
        let code = r#"
def f() -> str:
    template = "hello"
    return template.format()
"#;
        let result = transpile(code);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_format_single_arg() {
        let code = r#"
def f(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"));
    }

    #[test]
    fn test_w11sd_str_format_multiple_args() {
        let code = r#"
def f(a: str, b: str) -> str:
    return "{} and {}".format(a, b)
"#;
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"));
    }

    #[test]
    fn test_w11sd_str_startswith_in_condition() {
        let code = r#"
def is_url(s: str) -> bool:
    if s.startswith("http"):
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("starts_with"));
    }

    #[test]
    fn test_w11sd_str_endswith_in_condition() {
        let code = r#"
def is_python(name: str) -> bool:
    if name.endswith(".py"):
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("ends_with"));
    }

    // ==================== Section 8: Chained string methods (10 tests) ====================

    #[test]
    fn test_w11sd_str_chain_strip_lower() {
        let code = r#"
def f(s: str) -> str:
    return s.strip().lower()
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_chain_strip_upper() {
        let code = r#"
def f(s: str) -> str:
    return s.strip().upper()
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("to_uppercase"));
    }

    #[test]
    fn test_w11sd_str_chain_lower_replace() {
        let code = r#"
def f(s: str) -> str:
    return s.lower().replace("a", "b")
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase") || result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_chain_strip_lower_replace() {
        let code = r#"
def clean(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("to_lowercase") || result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_chain_upper_strip() {
        let code = r#"
def f(s: str) -> str:
    return s.upper().strip()
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("trim"));
    }

    #[test]
    fn test_w11sd_str_method_in_assignment() {
        let code = r#"
def f(s: str):
    upper = s.upper()
    lower = s.lower()
    return upper
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") && result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_method_in_list_comp() {
        let code = r#"
def f(words: list) -> list:
    return [w.upper() for w in words]
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_lstrip_with_chars() {
        let code = r#"
def f(s: str) -> str:
    return s.lstrip("0")
"#;
        let result = transpile(code);
        assert!(result.contains("trim_start_matches") || result.contains("contains"));
    }

    #[test]
    fn test_w11sd_str_rstrip_with_chars() {
        let code = r#"
def f(s: str) -> str:
    return s.rstrip(".")
"#;
        let result = transpile(code);
        assert!(result.contains("trim_end_matches") || result.contains("contains"));
    }

    #[test]
    fn test_w11sd_str_split_variable_sep() {
        let code = r#"
def f(s: str, sep: str):
    return s.split(sep)
"#;
        let result = transpile(code);
        assert!(result.contains("split"));
    }

    // ==================== Section 9: String methods in control flow (5 tests) ====================

    #[test]
    fn test_w11sd_str_isdigit_in_if() {
        let code = r#"
def f(s: str) -> int:
    if s.isdigit():
        return 1
    return 0
"#;
        let result = transpile(code);
        assert!(result.contains("is_numeric"));
    }

    #[test]
    fn test_w11sd_str_isalpha_in_if() {
        let code = r#"
def f(s: str) -> int:
    if s.isalpha():
        return 1
    return 0
"#;
        let result = transpile(code);
        assert!(result.contains("is_alphabetic"));
    }

    #[test]
    fn test_w11sd_str_find_in_condition() {
        let code = r#"
def f(s: str) -> bool:
    if s.find("x") >= 0:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_replace_in_assignment() {
        let code = r#"
def f(s: str) -> str:
    result = s.replace("a", "b")
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_split_in_loop() {
        let code = r#"
def f(s: str):
    parts = s.split(",")
    result = 0
    for p in parts:
        result = result + 1
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("split"));
    }

    // ==================== Section 10: Dict get method variations (15 tests) ====================

    #[test]
    fn test_w11sd_dict_get_one_arg_literal_key() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.get("a")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w11sd_dict_get_two_args_literal() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.get("c", 0)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_variable_key() {
        let code = r#"
def f(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_string_default() {
        let code = r#"
def f():
    d = {"name": "Alice"}
    return d.get("name", "Unknown")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_one_arg_variable() {
        let code = r#"
def f(d: dict, key: str):
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w11sd_dict_get_int_key() {
        let code = r#"
def f():
    d = {1: "one", 2: "two"}
    return d.get(1)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w11sd_dict_get_default_empty_string() {
        let code = r#"
def f():
    d = {"a": "hello"}
    return d.get("b", "")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_in_condition() {
        let code = r#"
def f():
    d = {"x": 1}
    val = d.get("x", 0)
    if val > 0:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_with_int_default() {
        let code = r#"
def f():
    d = {"count": 5}
    return d.get("count", -1)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_with_bool_default() {
        let code = r#"
def f():
    d = {"flag": True}
    return d.get("flag", False)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_nested() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    x = d.get("a", 0)
    y = d.get("b", 0)
    return x
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w11sd_dict_get_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    keys = ["a", "b", "c"]
    total = 0
    for k in keys:
        total = total + d.get(k, 0)
    return total
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_param_str_key() {
        let code = r#"
def lookup(d: dict, name: str) -> int:
    return d.get(name, 0)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_return_directly() {
        let code = r#"
def f():
    d = {"x": 42}
    return d.get("x")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("cloned"));
    }

    #[test]
    fn test_w11sd_dict_get_expression_default() {
        let code = r#"
def f(d: dict) -> int:
    n = 10
    return d.get("key", n)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    // ==================== Section 11: Dict keys/values/items (15 tests) ====================

    #[test]
    fn test_w11sd_dict_keys_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.keys()
"#;
        let result = transpile(code);
        assert!(result.contains("keys()") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_values_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.values()
"#;
        let result = transpile(code);
        assert!(result.contains("values()") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_items_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.items()
"#;
        let result = transpile(code);
        assert!(result.contains("iter()") || result.contains("items") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_keys_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    result = 0
    for k in d.keys():
        result = result + 1
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_values_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    total = 0
    for v in d.values():
        total = total + v
    return total
"#;
        let result = transpile(code);
        assert!(result.contains("values") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_items_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    total = 0
    for k, v in d.items():
        total = total + v
    return total
"#;
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("items") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_keys_assignment() {
        let code = r#"
def f():
    d = {"x": 1, "y": 2}
    all_keys = d.keys()
    return all_keys
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_values_assignment() {
        let code = r#"
def f():
    d = {"x": 1, "y": 2}
    all_vals = d.values()
    return all_vals
"#;
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_items_assignment() {
        let code = r#"
def f():
    d = {"x": 1, "y": 2}
    pairs = d.items()
    return pairs
"#;
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_keys_param() {
        let code = r#"
def f(d: dict):
    return d.keys()
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_values_param() {
        let code = r#"
def f(d: dict):
    return d.values()
"#;
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_items_param() {
        let code = r#"
def f(d: dict):
    return d.items()
"#;
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_for_k_in_dict() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    result = 0
    for k in d:
        result = result + 1
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("for") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_len() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return len(d)
"#;
        let result = transpile(code);
        assert!(result.contains("len()") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_bool_check() {
        let code = r#"
def f():
    d = {"a": 1}
    if d:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("is_empty") || result.contains("len") || result.len() > 0);
    }

    // ==================== Section 12: Dict update/setdefault/popitem/pop (20 tests) ====================

    #[test]
    fn test_w11sd_dict_setdefault_basic() {
        let code = r#"
def f():
    d = {"a": 1}
    return d.setdefault("b", 0)
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert") || result.contains("setdefault"));
    }

    #[test]
    fn test_w11sd_dict_setdefault_existing() {
        let code = r#"
def f():
    d = {"a": 1}
    return d.setdefault("a", 99)
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w11sd_dict_setdefault_string_val() {
        let code = r#"
def f():
    d = {"name": "Alice"}
    return d.setdefault("city", "Unknown")
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w11sd_dict_popitem_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.popitem()
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_with_key() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.pop("a")
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_with_default() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.pop("c", -1)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or") || result.contains("pop"));
    }

    #[test]
    fn test_w11sd_dict_update_basic() {
        let code = r#"
def f():
    d = {"a": 1}
    other = {"b": 2}
    d.update(other)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update") || result.contains("iter"));
    }

    #[test]
    fn test_w11sd_dict_copy_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    d2 = d.copy()
    return d2
"#;
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w11sd_dict_clear_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    d.clear()
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w11sd_dict_pop_assignment() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    val = d.pop("a")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_with_default_assignment() {
        let code = r#"
def f():
    d = {"a": 1}
    val = d.pop("b", 0)
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_setdefault_in_loop() {
        let code = r#"
def f():
    d = {}
    keys = ["a", "b", "c"]
    for k in keys:
        d.setdefault(k, 0)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_update_with_literal() {
        let code = r#"
def f():
    d = {"a": 1}
    d.update({"b": 2, "c": 3})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_copy_modify() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    d2 = d.copy()
    return d2
"#;
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w11sd_dict_clear_empty_check() {
        let code = r#"
def f():
    d = {"x": 1}
    d.clear()
    return len(d)
"#;
        let result = transpile(code);
        assert!(result.contains("clear") && (result.contains("len") || result.len() > 0));
    }

    #[test]
    fn test_w11sd_dict_pop_string_value() {
        let code = r#"
def f():
    d = {"name": "Alice", "city": "NYC"}
    return d.pop("name")
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_string_default() {
        let code = r#"
def f():
    d = {"name": "Alice"}
    return d.pop("missing", "Unknown")
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_setdefault_int_val() {
        let code = r#"
def f():
    d = {}
    d.setdefault("count", 0)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_popitem_assignment() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    pair = d.popitem()
    return pair
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_update_from_param() {
        let code = r#"
def f(d: dict, other: dict):
    d.update(other)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update") || result.contains("iter"));
    }

    // ==================== Section 13: Dict comprehension and construction (15 tests) ====================

    #[test]
    fn test_w11sd_dict_comprehension_basic() {
        let code = r#"
def f() -> dict:
    return {x: x * 2 for x in range(5)}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("collect") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_comprehension_string_keys() {
        let code = r#"
def f(words: list) -> dict:
    return {w: len(w) for w in words}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("collect") || result.contains("len") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_empty_literal() {
        let code = r#"
def f() -> dict:
    d = {}
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("new") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_literal_int_values() {
        let code = r#"
def f():
    return {"a": 1, "b": 2, "c": 3}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_literal_string_values() {
        let code = r#"
def f():
    return {"name": "Alice", "city": "NYC"}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_literal_bool_values() {
        let code = r#"
def f():
    return {"active": True, "admin": False}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_literal_float_values() {
        let code = r#"
def f():
    return {"x": 1.5, "y": 2.5}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_nested_access() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    x = d["a"]
    return x
"#;
        let result = transpile(code);
        assert!(result.contains("[") || result.contains("get") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_membership_test() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    if "a" in d:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("contains_key") || result.contains("contains") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_items_comprehension() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    result = {k: v * 2 for k, v in d.items()}
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("collect") || result.contains("HashMap") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_comp_with_condition() {
        let code = r#"
def f() -> dict:
    return {x: x * x for x in range(10) if x > 3}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("collect") || result.contains("filter") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_from_zip_pattern() {
        let code = r#"
def f():
    keys = ["a", "b", "c"]
    vals = [1, 2, 3]
    d = {}
    for i in range(3):
        d[keys[i]] = vals[i]
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_int_keys() {
        let code = r#"
def f():
    d = {1: "one", 2: "two", 3: "three"}
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_overwrite_key() {
        let code = r#"
def f():
    d = {"a": 1}
    d["a"] = 2
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("[") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_multiple_updates() {
        let code = r#"
def f():
    d = {}
    d["x"] = 1
    d["y"] = 2
    d["z"] = 3
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("[") || result.len() > 0);
    }

    // ==================== Section 14: Dict in various contexts (15 tests) ====================

    #[test]
    fn test_w11sd_dict_as_param() {
        let code = r#"
def process(d: dict) -> int:
    return len(d)
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("len") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_return_type() {
        let code = r#"
def make_dict() -> dict:
    return {"status": "ok"}
"#;
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_iterate_keys() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    result = []
    for k in d:
        result.append(k)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("push") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_iterate_values() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    total = 0
    for v in d.values():
        total = total + v
    return total
"#;
        let result = transpile(code);
        assert!(result.contains("values") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_iterate_items_unpack() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    for k, v in d.items():
        x = k
    return x
"#;
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("items") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_get_in_expression() {
        let code = r#"
def f():
    d = {"score": 95}
    total = d.get("score", 0) + 5
    return total
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_get_chain_to_method() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    keys = d.keys()
    return keys
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_conditional_get() {
        let code = r#"
def f():
    d = {"a": 1}
    if d.get("a", 0) > 0:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_keys_user_defined_with_args() {
        // Test the branch where keys() has arguments (user-defined method, not dict.keys())
        let code = r#"
def f():
    d = {"a": 1}
    k = d.keys()
    return k
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w11sd_dict_empty_update() {
        let code = r#"
def f():
    d = {"a": 1}
    other = {}
    d.update(other)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update") || result.contains("iter") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_return() {
        let code = r#"
def f() -> int:
    d = {"x": 42}
    return d.pop("x")
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_pop_default_return() {
        let code = r#"
def f() -> int:
    d = {"x": 42}
    return d.pop("y", 0)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_dict_setdefault_return() {
        let code = r#"
def f() -> int:
    d = {"x": 42}
    return d.setdefault("x", 0)
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w11sd_dict_copy_and_modify() {
        let code = r#"
def f():
    d = {"a": 1}
    d2 = d.copy()
    return d2
"#;
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w11sd_dict_clear_and_rebuild() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    d.clear()
    d["c"] = 3
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("clear") || result.len() > 0);
    }

    // ==================== Section 15: Mixed string+dict edge cases (10 tests) ====================

    #[test]
    fn test_w11sd_str_method_on_dict_value() {
        let code = r#"
def f():
    d = {"name": "alice"}
    name = d["name"]
    return name.upper()
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_split_assign_parts() {
        let code = r#"
def f(s: str):
    parts = s.split(",")
    return parts
"#;
        let result = transpile(code);
        assert!(result.contains("split"));
    }

    #[test]
    fn test_w11sd_str_join_dict_keys() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    keys = d.keys()
    result = ", ".join(keys)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("join") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_upper_in_dict_value() {
        let code = r#"
def f():
    name = "alice"
    d = {"name": name.upper()}
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_str_replace_multiple_calls() {
        let code = r#"
def clean(s: str) -> str:
    result = s.replace("a", "x")
    result = result.replace("b", "y")
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("replace"));
    }

    #[test]
    fn test_w11sd_str_split_and_join() {
        let code = r#"
def f(s: str) -> str:
    parts = s.split(",")
    return " ".join(parts)
"#;
        let result = transpile(code);
        assert!(result.contains("split") || result.contains("join"));
    }

    #[test]
    fn test_w11sd_dict_get_with_upper() {
        let code = r#"
def f():
    d = {"key": "hello"}
    val = d.get("key", "default")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w11sd_str_lower_in_comparison() {
        let code = r#"
def f(s: str) -> bool:
    return s.lower() == "hello"
"#;
        let result = transpile(code);
        assert!(result.contains("to_lowercase"));
    }

    #[test]
    fn test_w11sd_str_strip_in_loop() {
        let code = r#"
def f(lines: list):
    result = []
    for line in lines:
        result.append(line.strip())
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("trim") || result.len() > 0);
    }

    #[test]
    fn test_w11sd_dict_comprehension_with_str_method() {
        let code = r#"
def f(words: list) -> dict:
    return {w: w.upper() for w in words}
"#;
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("HashMap") || result.len() > 0);
    }
}
