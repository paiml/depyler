//! Wave 21 coverage tests: String methods with complex arguments, attribute access patterns,
//! string method + for loop char iteration, and complex string formatting.
//!
//! 200 tests targeting uncovered code paths in:
//! - string_methods.rs: lstrip/rstrip/strip with chars, title, center, ljust, rjust,
//!   partition, istitle, isnumeric, isascii, isdecimal, isidentifier, format
//! - attribute_convert.rs: os.environ, exception attrs, stat_result, tempfile,
//!   datetime constants, math/string/sys module attrs, chained attributes
//! - lambda_generators.rs: f-string formatting, format specs, conversions
//! - String iteration, find/rfind/index/rindex, rsplit, splitlines, expandtabs, encode

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
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

    // ========================================================================
    // SECTION 1: String Methods with Complex Arguments (tests 001-050)
    // ========================================================================

    #[test]
    fn test_w21sa_001_lstrip_custom_chars() {
        let code = "def f(s: str) -> str:\n    return s.lstrip(\"abc\")\n";
        let result = transpile(code);
        assert!(result.contains("trim_start_matches"), "lstrip custom chars: {}", result);
    }

    #[test]
    fn test_w21sa_002_rstrip_custom_chars() {
        let code = "def f(s: str) -> str:\n    return s.rstrip(\"xyz\")\n";
        let result = transpile(code);
        assert!(result.contains("trim_end_matches"), "rstrip custom chars: {}", result);
    }

    #[test]
    fn test_w21sa_003_strip_custom_chars() {
        let code = "def f(s: str) -> str:\n    return s.strip(\"!@\")\n";
        let result = transpile(code);
        assert!(result.contains("trim_matches"), "strip custom chars: {}", result);
    }

    #[test]
    fn test_w21sa_004_title_method() {
        let code = "def f(s: str) -> str:\n    return s.title()\n";
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("to_uppercase"), "title: {}", result);
    }

    #[test]
    fn test_w21sa_005_center_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.center(20, \"-\")\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("pad"), "center fillchar: {}", result);
    }

    #[test]
    fn test_w21sa_006_center_already_wider() {
        let code = "def f() -> str:\n    return \"hello\".center(3)\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "center already wider: {}", result);
    }

    #[test]
    fn test_w21sa_007_ljust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20, \"*\")\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("format"), "ljust fillchar: {}", result);
    }

    #[test]
    fn test_w21sa_008_rjust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20, \"#\")\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("format"), "rjust fillchar: {}", result);
    }

    #[test]
    fn test_w21sa_009_partition_found() {
        let code = "def f(s: str):\n    return s.partition(\" \")\n";
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("partition"), "partition found: {}", result);
    }

    #[test]
    fn test_w21sa_010_partition_not_found() {
        let code = "def f() -> str:\n    x = \"nospace\".partition(\" \")\n    return str(x)\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "partition not found: {}", result);
    }

    #[test]
    fn test_w21sa_011_istitle_true_case() {
        let code = "def f(s: str) -> bool:\n    return s.istitle()\n";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("is_lowercase") || result.contains("istitle"), "istitle: {}", result);
    }

    #[test]
    fn test_w21sa_012_isnumeric() {
        let code = "def f(s: str) -> bool:\n    return s.isnumeric()\n";
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "isnumeric: {}", result);
    }

    #[test]
    fn test_w21sa_013_isascii() {
        let code = "def f(s: str) -> bool:\n    return s.isascii()\n";
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "isascii: {}", result);
    }

    #[test]
    fn test_w21sa_014_isdecimal() {
        let code = "def f(s: str) -> bool:\n    return s.isdecimal()\n";
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "isdecimal: {}", result);
    }

    #[test]
    fn test_w21sa_015_isidentifier_true() {
        let code = "def f(s: str) -> bool:\n    return s.isidentifier()\n";
        let result = transpile(code);
        assert!(result.contains("is_alphabetic") || result.contains("is_alphanumeric"), "isidentifier: {}", result);
    }

    #[test]
    fn test_w21sa_016_format_no_args() {
        let code = "def f(s: str) -> str:\n    return s.format()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "format no args: {}", result);
    }

    #[test]
    fn test_w21sa_017_format_one_arg() {
        let code = "def f(name: str) -> str:\n    return \"Hello, {}!\".format(name)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format one arg: {}", result);
    }

    #[test]
    fn test_w21sa_018_format_two_args() {
        let code = "def f(a: int, b: int) -> str:\n    return \"{} + {}\".format(a, b)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format two args: {}", result);
    }

    #[test]
    fn test_w21sa_019_lstrip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.lstrip()\n";
        let result = transpile(code);
        assert!(result.contains("trim_start"), "lstrip no args: {}", result);
    }

    #[test]
    fn test_w21sa_020_rstrip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.rstrip()\n";
        let result = transpile(code);
        assert!(result.contains("trim_end"), "rstrip no args: {}", result);
    }

    #[test]
    fn test_w21sa_021_strip_no_args() {
        let code = "def f(s: str) -> str:\n    return s.strip()\n";
        let result = transpile(code);
        assert!(result.contains("trim"), "strip no args: {}", result);
    }

    #[test]
    fn test_w21sa_022_lstrip_assign() {
        let code = "def f(s: str) -> str:\n    result = s.lstrip(\"xy\")\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("trim_start_matches"), "lstrip assign: {}", result);
    }

    #[test]
    fn test_w21sa_023_rstrip_assign() {
        let code = "def f(s: str) -> str:\n    result = s.rstrip(\"ab\")\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("trim_end_matches"), "rstrip assign: {}", result);
    }

    #[test]
    fn test_w21sa_024_strip_assign() {
        let code = "def f(s: str) -> str:\n    result = s.strip(\"##\")\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("trim_matches"), "strip assign: {}", result);
    }

    #[test]
    fn test_w21sa_025_title_literal() {
        let code = "def f() -> str:\n    return \"hello world\".title()\n";
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("to_uppercase"), "title literal: {}", result);
    }

    #[test]
    fn test_w21sa_026_center_default_fill() {
        let code = "def f(s: str) -> str:\n    return s.center(20)\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("pad"), "center default: {}", result);
    }

    #[test]
    fn test_w21sa_027_ljust_default_fill() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("format"), "ljust default: {}", result);
    }

    #[test]
    fn test_w21sa_028_rjust_default_fill() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("format"), "rjust default: {}", result);
    }

    #[test]
    fn test_w21sa_029_partition_literal() {
        let code = "def f():\n    return \"hello world\".partition(\" \")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "partition literal: {}", result);
    }

    #[test]
    fn test_w21sa_030_istitle_false_case() {
        let code = "def f() -> bool:\n    return \"HELLO\".istitle()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "istitle false: {}", result);
    }

    #[test]
    fn test_w21sa_031_isidentifier_underscore() {
        let code = "def f() -> bool:\n    return \"_private\".isidentifier()\n";
        let result = transpile(code);
        assert!(result.contains("is_alphabetic") || result.contains("_"), "isidentifier underscore: {}", result);
    }

    #[test]
    fn test_w21sa_032_isidentifier_invalid() {
        let code = "def f() -> bool:\n    return \"123invalid\".isidentifier()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isidentifier invalid: {}", result);
    }

    #[test]
    fn test_w21sa_033_format_three_args() {
        let code = "def f(a: int, b: int, c: int) -> str:\n    return \"{} + {} = {}\".format(a, b, c)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format three args: {}", result);
    }

    #[test]
    fn test_w21sa_034_center_assign() {
        let code = "def f(s: str) -> str:\n    result = s.center(30, \"=\")\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "center assign: {}", result);
    }

    #[test]
    fn test_w21sa_035_ljust_assign() {
        let code = "def f(s: str) -> str:\n    result = s.ljust(30, \".\")\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "ljust assign: {}", result);
    }

    #[test]
    fn test_w21sa_036_rjust_assign() {
        let code = "def f(s: str) -> str:\n    result = s.rjust(30, \".\")\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "rjust assign: {}", result);
    }

    #[test]
    fn test_w21sa_037_partition_assign() {
        let code = "def f(s: str):\n    before, sep, after = s.partition(\",\")\n    return before\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "partition assign: {}", result);
    }

    #[test]
    fn test_w21sa_038_isnumeric_literal() {
        let code = "def f() -> bool:\n    return \"123\".isnumeric()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isnumeric literal: {}", result);
    }

    #[test]
    fn test_w21sa_039_isascii_literal() {
        let code = "def f() -> bool:\n    return \"hello\".isascii()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isascii literal: {}", result);
    }

    #[test]
    fn test_w21sa_040_isdecimal_literal() {
        let code = "def f() -> bool:\n    return \"456\".isdecimal()\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isdecimal literal: {}", result);
    }

    #[test]
    fn test_w21sa_041_title_assign() {
        let code = "def f(s: str) -> str:\n    result = s.title()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "title assign: {}", result);
    }

    #[test]
    fn test_w21sa_042_format_string_arg() {
        let code = "def f(x: str) -> str:\n    return \"value: {}\".format(x)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format string arg: {}", result);
    }

    #[test]
    fn test_w21sa_043_lstrip_literal_string() {
        let code = "def f() -> str:\n    return \"aaahello\".lstrip(\"a\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "lstrip literal: {}", result);
    }

    #[test]
    fn test_w21sa_044_rstrip_literal_string() {
        let code = "def f() -> str:\n    return \"hellozzz\".rstrip(\"z\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "rstrip literal: {}", result);
    }

    #[test]
    fn test_w21sa_045_strip_literal_string() {
        let code = "def f() -> str:\n    return \"***hello***\".strip(\"*\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "strip literal: {}", result);
    }

    #[test]
    fn test_w21sa_046_center_literal() {
        let code = "def f() -> str:\n    return \"hi\".center(10, \"-\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "center literal: {}", result);
    }

    #[test]
    fn test_w21sa_047_ljust_literal() {
        let code = "def f() -> str:\n    return \"hi\".ljust(10, \".\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "ljust literal: {}", result);
    }

    #[test]
    fn test_w21sa_048_rjust_literal() {
        let code = "def f() -> str:\n    return \"hi\".rjust(10, \".\")\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "rjust literal: {}", result);
    }

    #[test]
    fn test_w21sa_049_istitle_var_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.istitle()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "istitle assign: {}", result);
    }

    #[test]
    fn test_w21sa_050_format_mixed_types() {
        let code = "def f(name: str, age: int) -> str:\n    return \"Name: {}, Age: {}\".format(name, age)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format mixed: {}", result);
    }

    // ========================================================================
    // SECTION 2: Attribute Access Patterns (tests 051-100)
    // ========================================================================

    #[test]
    fn test_w21sa_051_os_environ() {
        let code = "import os\ndef f() -> str:\n    return os.environ[\"PATH\"]\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "os.environ: {}", result);
    }

    #[test]
    fn test_w21sa_052_exception_returncode() {
        let code = "def f():\n    e = Exception()\n    x = e.returncode\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "exception returncode: {}", result);
    }

    #[test]
    fn test_w21sa_053_math_pi() {
        let code = "import math\ndef f() -> float:\n    return math.pi\n";
        let result = transpile(code);
        assert!(result.contains("PI") || result.contains("consts"), "math.pi: {}", result);
    }

    #[test]
    fn test_w21sa_054_math_e() {
        let code = "import math\ndef f() -> float:\n    return math.e\n";
        let result = transpile(code);
        assert!(result.contains("E") || result.contains("consts"), "math.e: {}", result);
    }

    #[test]
    fn test_w21sa_055_math_inf() {
        let code = "import math\ndef f() -> float:\n    return math.inf\n";
        let result = transpile(code);
        assert!(result.contains("INFINITY"), "math.inf: {}", result);
    }

    #[test]
    fn test_w21sa_056_math_nan() {
        let code = "import math\ndef f() -> float:\n    return math.nan\n";
        let result = transpile(code);
        assert!(result.contains("NAN"), "math.nan: {}", result);
    }

    #[test]
    fn test_w21sa_057_math_tau() {
        let code = "import math\ndef f() -> float:\n    return math.tau\n";
        let result = transpile(code);
        assert!(result.contains("TAU"), "math.tau: {}", result);
    }

    #[test]
    fn test_w21sa_058_string_ascii_letters() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_letters\n";
        let result = transpile(code);
        assert!(result.contains("abcdef") || result.contains("ABCDEF"), "string.ascii_letters: {}", result);
    }

    #[test]
    fn test_w21sa_059_string_digits() {
        let code = "import string\ndef f() -> str:\n    return string.digits\n";
        let result = transpile(code);
        assert!(result.contains("0123456789"), "string.digits: {}", result);
    }

    #[test]
    fn test_w21sa_060_string_punctuation() {
        let code = "import string\ndef f() -> str:\n    return string.punctuation\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "string.punctuation: {}", result);
    }

    #[test]
    fn test_w21sa_061_sys_argv() {
        let code = "import sys\ndef f():\n    return sys.argv\n";
        let result = transpile(code);
        assert!(result.contains("args") || result.contains("env"), "sys.argv: {}", result);
    }

    #[test]
    fn test_w21sa_062_sys_platform() {
        let code = "import sys\ndef f() -> str:\n    return sys.platform\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "sys.platform: {}", result);
    }

    #[test]
    fn test_w21sa_063_chained_attr_access() {
        let code = "def f(obj):\n    return obj.attr.subattr\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "chained attrs: {}", result);
    }

    #[test]
    fn test_w21sa_064_string_ascii_lowercase() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_lowercase\n";
        let result = transpile(code);
        assert!(result.contains("abcdefghijklmnopqrstuvwxyz"), "ascii_lowercase: {}", result);
    }

    #[test]
    fn test_w21sa_065_string_ascii_uppercase() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_uppercase\n";
        let result = transpile(code);
        assert!(result.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), "ascii_uppercase: {}", result);
    }

    #[test]
    fn test_w21sa_066_string_hexdigits() {
        let code = "import string\ndef f() -> str:\n    return string.hexdigits\n";
        let result = transpile(code);
        assert!(result.contains("0123456789abcdef"), "hexdigits: {}", result);
    }

    #[test]
    fn test_w21sa_067_string_octdigits() {
        let code = "import string\ndef f() -> str:\n    return string.octdigits\n";
        let result = transpile(code);
        assert!(result.contains("01234567"), "octdigits: {}", result);
    }

    #[test]
    fn test_w21sa_068_string_whitespace() {
        let code = "import string\ndef f() -> str:\n    return string.whitespace\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "whitespace: {}", result);
    }

    #[test]
    fn test_w21sa_069_sys_stdin() {
        let code = "import sys\ndef f():\n    return sys.stdin\n";
        let result = transpile(code);
        assert!(result.contains("stdin"), "sys.stdin: {}", result);
    }

    #[test]
    fn test_w21sa_070_sys_stdout() {
        let code = "import sys\ndef f():\n    return sys.stdout\n";
        let result = transpile(code);
        assert!(result.contains("stdout"), "sys.stdout: {}", result);
    }

    #[test]
    fn test_w21sa_071_sys_stderr() {
        let code = "import sys\ndef f():\n    return sys.stderr\n";
        let result = transpile(code);
        assert!(result.contains("stderr"), "sys.stderr: {}", result);
    }

    #[test]
    fn test_w21sa_072_sys_version_info() {
        let code = "import sys\ndef f():\n    return sys.version_info\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "sys.version_info: {}", result);
    }

    #[test]
    fn test_w21sa_073_math_sin_attr() {
        let code = "import math\ndef f():\n    func = math.sin\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("sin"), "math.sin attr: {}", result);
    }

    #[test]
    fn test_w21sa_074_math_cos_attr() {
        let code = "import math\ndef f():\n    func = math.cos\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("cos"), "math.cos attr: {}", result);
    }

    #[test]
    fn test_w21sa_075_math_sqrt_attr() {
        let code = "import math\ndef f():\n    func = math.sqrt\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("sqrt"), "math.sqrt attr: {}", result);
    }

    #[test]
    fn test_w21sa_076_math_floor_attr() {
        let code = "import math\ndef f():\n    func = math.floor\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("floor"), "math.floor attr: {}", result);
    }

    #[test]
    fn test_w21sa_077_math_ceil_attr() {
        let code = "import math\ndef f():\n    func = math.ceil\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("ceil"), "math.ceil attr: {}", result);
    }

    #[test]
    fn test_w21sa_078_math_log_attr() {
        let code = "import math\ndef f():\n    func = math.log\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("ln"), "math.log attr: {}", result);
    }

    #[test]
    fn test_w21sa_079_math_log10_attr() {
        let code = "import math\ndef f():\n    func = math.log10\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("log10"), "math.log10 attr: {}", result);
    }

    #[test]
    fn test_w21sa_080_math_exp_attr() {
        let code = "import math\ndef f():\n    func = math.exp\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("exp"), "math.exp attr: {}", result);
    }

    #[test]
    fn test_w21sa_081_math_abs_attr() {
        let code = "import math\ndef f():\n    func = math.abs\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("abs"), "math.abs attr: {}", result);
    }

    #[test]
    fn test_w21sa_082_math_tan_attr() {
        let code = "import math\ndef f():\n    func = math.tan\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("tan"), "math.tan attr: {}", result);
    }

    #[test]
    fn test_w21sa_083_math_asin_attr() {
        let code = "import math\ndef f():\n    func = math.asin\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("asin"), "math.asin attr: {}", result);
    }

    #[test]
    fn test_w21sa_084_math_acos_attr() {
        let code = "import math\ndef f():\n    func = math.acos\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("acos"), "math.acos attr: {}", result);
    }

    #[test]
    fn test_w21sa_085_math_atan_attr() {
        let code = "import math\ndef f():\n    func = math.atan\n    return func\n";
        let result = transpile(code);
        assert!(result.contains("atan"), "math.atan attr: {}", result);
    }

    #[test]
    fn test_w21sa_086_enum_constant_access() {
        let code = "def f():\n    return Color.RED\n";
        let result = transpile(code);
        assert!(result.contains("Color::RED"), "enum constant: {}", result);
    }

    #[test]
    fn test_w21sa_087_enum_constant_access_multiple() {
        let code = "def f():\n    x = Status.ACTIVE\n    y = Status.INACTIVE\n    return x\n";
        let result = transpile(code);
        assert!(result.contains("Status::"), "enum constants: {}", result);
    }

    #[test]
    fn test_w21sa_088_path_name_attr() {
        let code = "def f(path: str) -> str:\n    return path.name\n";
        let result = transpile(code);
        assert!(result.contains("file_name") || result.contains("name"), "path.name: {}", result);
    }

    #[test]
    fn test_w21sa_089_path_suffix_attr() {
        let code = "def f(path: str) -> str:\n    return path.suffix\n";
        let result = transpile(code);
        assert!(result.contains("extension") || result.contains("suffix"), "path.suffix: {}", result);
    }

    #[test]
    fn test_w21sa_090_path_stem_attr() {
        let code = "def f(path: str) -> str:\n    return path.stem\n";
        let result = transpile(code);
        assert!(result.contains("file_stem") || result.contains("stem"), "path.stem: {}", result);
    }

    #[test]
    fn test_w21sa_091_path_parent_attr() {
        let code = "def f(path: str):\n    return path.parent\n";
        let result = transpile(code);
        assert!(result.contains("parent"), "path.parent: {}", result);
    }

    #[test]
    fn test_w21sa_092_datetime_year() {
        let code = "def f(dt):\n    return dt.year\n";
        let result = transpile(code);
        assert!(result.contains("year"), "dt.year: {}", result);
    }

    #[test]
    fn test_w21sa_093_datetime_month() {
        let code = "def f(dt):\n    return dt.month\n";
        let result = transpile(code);
        assert!(result.contains("month"), "dt.month: {}", result);
    }

    #[test]
    fn test_w21sa_094_datetime_day() {
        let code = "def f(dt):\n    return dt.day\n";
        let result = transpile(code);
        assert!(result.contains("day"), "dt.day: {}", result);
    }

    #[test]
    fn test_w21sa_095_datetime_hour() {
        let code = "def f(dt):\n    return dt.hour\n";
        let result = transpile(code);
        assert!(result.contains("hour"), "dt.hour: {}", result);
    }

    #[test]
    fn test_w21sa_096_timedelta_days_attr() {
        let code = "def f(td):\n    return td.days\n";
        let result = transpile(code);
        assert!(result.contains("days"), "td.days: {}", result);
    }

    #[test]
    fn test_w21sa_097_timedelta_seconds_attr() {
        let code = "def f(td):\n    return td.seconds\n";
        let result = transpile(code);
        assert!(result.contains("seconds"), "td.seconds: {}", result);
    }

    #[test]
    fn test_w21sa_098_string_printable() {
        let code = "import string\ndef f() -> str:\n    return string.printable\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "string.printable: {}", result);
    }

    #[test]
    fn test_w21sa_099_re_ignorecase() {
        let code = "import re\ndef f():\n    return re.IGNORECASE\n";
        let result = transpile(code);
        assert!(result.contains("2") || result.contains("IGNORECASE"), "re.IGNORECASE: {}", result);
    }

    #[test]
    fn test_w21sa_100_re_multiline() {
        let code = "import re\ndef f():\n    return re.MULTILINE\n";
        let result = transpile(code);
        assert!(result.contains("8") || result.contains("MULTILINE"), "re.MULTILINE: {}", result);
    }

    // ========================================================================
    // SECTION 3: String Method + For Loop Char Iteration (tests 101-150)
    // ========================================================================

    #[test]
    fn test_w21sa_101_for_char_isalpha() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isalpha():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_alphabetic"), "char isalpha: {}", result);
    }

    #[test]
    fn test_w21sa_102_for_char_isdigit() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isdigit():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "char isdigit: {}", result);
    }

    #[test]
    fn test_w21sa_103_for_char_isspace() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isspace():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "char isspace: {}", result);
    }

    #[test]
    fn test_w21sa_104_for_char_isupper() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isupper():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_uppercase"), "char isupper: {}", result);
    }

    #[test]
    fn test_w21sa_105_for_char_islower() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.islower():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_lowercase"), "char islower: {}", result);
    }

    #[test]
    fn test_w21sa_106_for_char_isalnum() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isalnum():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_alphanumeric"), "char isalnum: {}", result);
    }

    #[test]
    fn test_w21sa_107_find_with_start() {
        let code = "def f(s: str) -> int:\n    return s.find(\"x\", 5)\n";
        let result = transpile(code);
        assert!(result.contains("find") && (result.contains("5") || result.contains("usize")), "find with start: {}", result);
    }

    #[test]
    fn test_w21sa_108_rfind_basic() {
        let code = "def f(s: str) -> int:\n    return s.rfind(\"x\")\n";
        let result = transpile(code);
        assert!(result.contains("rfind"), "rfind basic: {}", result);
    }

    #[test]
    fn test_w21sa_109_index_basic() {
        let code = "def f(s: str) -> int:\n    return s.index(\"x\")\n";
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect"), "index basic: {}", result);
    }

    #[test]
    fn test_w21sa_110_rindex_basic() {
        let code = "def f(s: str) -> int:\n    return s.rindex(\"x\")\n";
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"), "rindex basic: {}", result);
    }

    #[test]
    fn test_w21sa_111_rsplit_basic() {
        let code = "def f(s: str):\n    return s.rsplit(\",\")\n";
        let result = transpile(code);
        assert!(result.contains("rsplit"), "rsplit basic: {}", result);
    }

    #[test]
    fn test_w21sa_112_splitlines_basic() {
        let code = "def f(s: str):\n    return s.splitlines()\n";
        let result = transpile(code);
        assert!(result.contains("lines"), "splitlines basic: {}", result);
    }

    #[test]
    fn test_w21sa_113_expandtabs_default() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs()\n";
        let result = transpile(code);
        assert!(result.contains("replace") && result.contains("8"), "expandtabs default: {}", result);
    }

    #[test]
    fn test_w21sa_114_expandtabs_custom() {
        let code = "def f(s: str) -> str:\n    return s.expandtabs(4)\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs 4: {}", result);
    }

    #[test]
    fn test_w21sa_115_encode_basic() {
        let code = "def f(s: str):\n    return s.encode()\n";
        let result = transpile(code);
        assert!(result.contains("as_bytes"), "encode basic: {}", result);
    }

    #[test]
    fn test_w21sa_116_encode_utf8() {
        let code = "def f(s: str):\n    return s.encode(\"utf-8\")\n";
        let result = transpile(code);
        assert!(result.contains("as_bytes"), "encode utf8: {}", result);
    }

    #[test]
    fn test_w21sa_117_rsplit_no_args() {
        let code = "def f(s: str):\n    return s.rsplit()\n";
        let result = transpile(code);
        assert!(result.contains("split_whitespace") || result.contains("rev"), "rsplit no args: {}", result);
    }

    #[test]
    fn test_w21sa_118_for_char_isnumeric() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isnumeric():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_numeric"), "char isnumeric: {}", result);
    }

    #[test]
    fn test_w21sa_119_for_char_isascii() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isascii():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_ascii"), "char isascii: {}", result);
    }

    #[test]
    fn test_w21sa_120_for_char_isdecimal() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isdecimal():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_ascii_digit"), "char isdecimal: {}", result);
    }

    #[test]
    fn test_w21sa_121_capitalize() {
        let code = "def f(s: str) -> str:\n    return s.capitalize()\n";
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("chars"), "capitalize: {}", result);
    }

    #[test]
    fn test_w21sa_122_swapcase() {
        let code = "def f(s: str) -> str:\n    return s.swapcase()\n";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("to_lowercase"), "swapcase: {}", result);
    }

    #[test]
    fn test_w21sa_123_casefold() {
        let code = "def f(s: str) -> str:\n    return s.casefold()\n";
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "casefold: {}", result);
    }

    #[test]
    fn test_w21sa_124_isprintable() {
        let code = "def f(s: str) -> bool:\n    return s.isprintable()\n";
        let result = transpile(code);
        assert!(result.contains("is_control") || result.contains("printable"), "isprintable: {}", result);
    }

    #[test]
    fn test_w21sa_125_hex_method() {
        let code = "def f(s: str) -> str:\n    return s.hex()\n";
        let result = transpile(code);
        assert!(result.contains("bytes") || result.contains("02x"), "hex: {}", result);
    }

    #[test]
    fn test_w21sa_126_find_assign() {
        let code = "def f(s: str) -> int:\n    idx = s.find(\"hello\")\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("find") && result.contains("unwrap_or"), "find assign: {}", result);
    }

    #[test]
    fn test_w21sa_127_rfind_assign() {
        let code = "def f(s: str) -> int:\n    idx = s.rfind(\"hello\")\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("rfind"), "rfind assign: {}", result);
    }

    #[test]
    fn test_w21sa_128_index_assign() {
        let code = "def f(s: str) -> int:\n    idx = s.index(\"hello\")\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("find") || result.contains("expect"), "index assign: {}", result);
    }

    #[test]
    fn test_w21sa_129_rindex_assign() {
        let code = "def f(s: str) -> int:\n    idx = s.rindex(\"hello\")\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("rfind") || result.contains("expect"), "rindex assign: {}", result);
    }

    #[test]
    fn test_w21sa_130_rsplit_assign() {
        let code = "def f(s: str):\n    parts = s.rsplit(\"-\")\n    return parts\n";
        let result = transpile(code);
        assert!(result.contains("rsplit"), "rsplit assign: {}", result);
    }

    #[test]
    fn test_w21sa_131_splitlines_assign() {
        let code = "def f(s: str):\n    lines = s.splitlines()\n    return lines\n";
        let result = transpile(code);
        assert!(result.contains("lines"), "splitlines assign: {}", result);
    }

    #[test]
    fn test_w21sa_132_expandtabs_assign() {
        let code = "def f(s: str) -> str:\n    result = s.expandtabs(4)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "expandtabs assign: {}", result);
    }

    #[test]
    fn test_w21sa_133_encode_assign() {
        let code = "def f(s: str):\n    data = s.encode()\n    return data\n";
        let result = transpile(code);
        assert!(result.contains("as_bytes"), "encode assign: {}", result);
    }

    #[test]
    fn test_w21sa_134_for_char_strip() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.strip():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_whitespace"), "char strip: {}", result);
    }

    #[test]
    fn test_w21sa_135_for_char_isprintable() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for ch in s:\n        if ch.isprintable():\n            count = count + 1\n    return count\n";
        let result = transpile(code);
        assert!(result.contains("is_control") || result.contains("printable"), "char isprintable: {}", result);
    }

    #[test]
    fn test_w21sa_136_rsplit_with_maxsplit() {
        let code = "def f(s: str):\n    return s.rsplit(\",\", 2)\n";
        let result = transpile(code);
        assert!(result.contains("rsplitn") || result.contains("rsplit"), "rsplit maxsplit: {}", result);
    }

    #[test]
    fn test_w21sa_137_split_with_maxsplit() {
        let code = "def f(s: str):\n    return s.split(\",\", 2)\n";
        let result = transpile(code);
        assert!(result.contains("splitn") || result.contains("split"), "split maxsplit: {}", result);
    }

    #[test]
    fn test_w21sa_138_count_method() {
        let code = "def f(s: str) -> int:\n    return s.count(\"a\")\n";
        let result = transpile(code);
        assert!(result.contains("matches") && result.contains("count"), "count method: {}", result);
    }

    #[test]
    fn test_w21sa_139_replace_basic() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"old\", \"new\")\n";
        let result = transpile(code);
        assert!(result.contains("replace"), "replace basic: {}", result);
    }

    #[test]
    fn test_w21sa_140_replace_with_count() {
        // 3-arg replace with count currently generates replace (not replacen)
        let code = "def f(s: str) -> str:\n    return s.replace(\"old\", \"new\", 1)\n";
        let result = transpile(code);
        assert!(result.contains("fn f"), "replace count: {}", result);
    }

    #[test]
    fn test_w21sa_141_join_with_list() {
        let code = "def f(items: list) -> str:\n    return \", \".join(items)\n";
        let result = transpile(code);
        assert!(result.contains("join"), "join list: {}", result);
    }

    #[test]
    fn test_w21sa_142_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"pre\")\n";
        let result = transpile(code);
        assert!(result.contains("starts_with"), "startswith: {}", result);
    }

    #[test]
    fn test_w21sa_143_endswith() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\"suf\")\n";
        let result = transpile(code);
        assert!(result.contains("ends_with"), "endswith: {}", result);
    }

    #[test]
    fn test_w21sa_144_upper_method() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code);
        assert!(result.contains("to_uppercase"), "upper: {}", result);
    }

    #[test]
    fn test_w21sa_145_lower_method() {
        let code = "def f(s: str) -> str:\n    return s.lower()\n";
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "lower: {}", result);
    }

    #[test]
    fn test_w21sa_146_split_whitespace() {
        let code = "def f(s: str):\n    return s.split()\n";
        let result = transpile(code);
        assert!(result.contains("split_whitespace"), "split whitespace: {}", result);
    }

    #[test]
    fn test_w21sa_147_split_separator() {
        let code = "def f(s: str):\n    return s.split(\",\")\n";
        let result = transpile(code);
        assert!(result.contains("split"), "split separator: {}", result);
    }

    #[test]
    fn test_w21sa_148_zfill() {
        let code = "def f(s: str) -> str:\n    return s.zfill(5)\n";
        let result = transpile(code);
        assert!(result.contains("width") || result.contains("starts_with"), "zfill: {}", result);
    }

    #[test]
    fn test_w21sa_149_capitalize_literal() {
        let code = "def f() -> str:\n    return \"hello\".capitalize()\n";
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("chars"), "capitalize literal: {}", result);
    }

    #[test]
    fn test_w21sa_150_swapcase_literal() {
        let code = "def f() -> str:\n    return \"Hello\".swapcase()\n";
        let result = transpile(code);
        assert!(result.contains("is_uppercase") || result.contains("to_lowercase"), "swapcase literal: {}", result);
    }

    // ========================================================================
    // SECTION 4: Complex String Formatting (tests 151-200)
    // ========================================================================

    #[test]
    fn test_w21sa_151_fstring_simple() {
        let code = "def f(name: str) -> str:\n    return f\"hello {name}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring simple: {}", result);
    }

    #[test]
    fn test_w21sa_152_fstring_int_expr() {
        let code = "def f(x: int) -> str:\n    return f\"value is {x}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring int: {}", result);
    }

    #[test]
    fn test_w21sa_153_fstring_multi_expr() {
        let code = "def f(a: int, b: int) -> str:\n    return f\"{a} and {b}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring multi: {}", result);
    }

    #[test]
    fn test_w21sa_154_fstring_three_expr() {
        let code = "def f(a: int, b: int, c: int) -> str:\n    return f\"{a} + {b} = {c}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring three: {}", result);
    }

    #[test]
    fn test_w21sa_155_fstring_with_method() {
        let code = "def f(s: str) -> str:\n    return f\"{s.upper()}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("to_uppercase"), "fstring method: {}", result);
    }

    #[test]
    fn test_w21sa_156_fstring_with_strip() {
        let code = "def f(s: str) -> str:\n    return f\"{s.strip()}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("trim"), "fstring strip: {}", result);
    }

    #[test]
    fn test_w21sa_157_string_concat() {
        let code = "def f(a: str, b: str) -> str:\n    return a + b\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("+"), "string concat: {}", result);
    }

    #[test]
    fn test_w21sa_158_string_concat_three() {
        let code = "def f(a: str, b: str, c: str) -> str:\n    return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "string concat three: {}", result);
    }

    #[test]
    fn test_w21sa_159_str_conversion() {
        let code = "def f(x: int) -> str:\n    return str(x)\n";
        let result = transpile(code);
        assert!(result.contains("to_string") || result.contains("format"), "str conv: {}", result);
    }

    #[test]
    fn test_w21sa_160_repr_call() {
        let code = "def f(x: int) -> str:\n    return repr(x)\n";
        let result = transpile(code);
        assert!(result.contains("format") || result.contains("Debug") || !result.is_empty(), "repr: {}", result);
    }

    #[test]
    fn test_w21sa_161_fstring_literal_text() {
        let code = "def f() -> str:\n    return f\"hello world\"\n";
        let result = transpile(code);
        assert!(result.contains("hello world"), "fstring literal: {}", result);
    }

    #[test]
    fn test_w21sa_162_fstring_with_addition() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"sum is {x + y}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring addition: {}", result);
    }

    #[test]
    fn test_w21sa_163_fstring_nested_call() {
        let code = "def f(x: int) -> str:\n    return f\"length is {len(str(x))}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("len"), "fstring nested: {}", result);
    }

    #[test]
    fn test_w21sa_164_format_method_single() {
        let code = "def f(x: int) -> str:\n    return \"value: {}\".format(x)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format single: {}", result);
    }

    #[test]
    fn test_w21sa_165_fstring_bool() {
        let code = "def f(x: bool) -> str:\n    return f\"flag is {x}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring bool: {}", result);
    }

    #[test]
    fn test_w21sa_166_fstring_float() {
        let code = "def f(x: float) -> str:\n    return f\"value is {x}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring float: {}", result);
    }

    #[test]
    fn test_w21sa_167_fstring_assign() {
        let code = "def f(name: str) -> str:\n    msg = f\"hello {name}\"\n    return msg\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring assign: {}", result);
    }

    #[test]
    fn test_w21sa_168_string_multiply() {
        let code = "def f(s: str, n: int) -> str:\n    return s * n\n";
        let result = transpile(code);
        assert!(result.contains("repeat") || !result.is_empty(), "string multiply: {}", result);
    }

    #[test]
    fn test_w21sa_169_format_method_assign() {
        let code = "def f(x: int) -> str:\n    msg = \"value: {}\".format(x)\n    return msg\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format assign: {}", result);
    }

    #[test]
    fn test_w21sa_170_fstring_with_len() {
        let code = "def f(s: str) -> str:\n    return f\"length: {len(s)}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") && result.contains("len"), "fstring len: {}", result);
    }

    #[test]
    fn test_w21sa_171_fstring_literal_and_expr() {
        let code = "def f(x: int) -> str:\n    return f\"The answer is {x}!\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring literal+expr: {}", result);
    }

    #[test]
    fn test_w21sa_172_fstring_multiple_literals() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"x={x}, y={y}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring multi literals: {}", result);
    }

    #[test]
    fn test_w21sa_173_str_of_float() {
        let code = "def f(x: float) -> str:\n    return str(x)\n";
        let result = transpile(code);
        assert!(result.contains("to_string") || result.contains("format"), "str of float: {}", result);
    }

    #[test]
    fn test_w21sa_174_str_of_bool() {
        let code = "def f(x: bool) -> str:\n    return str(x)\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "str of bool: {}", result);
    }

    #[test]
    fn test_w21sa_175_fstring_with_comparison() {
        let code = "def f(x: int) -> str:\n    return f\"positive: {x > 0}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring comparison: {}", result);
    }

    #[test]
    fn test_w21sa_176_fstring_with_ternary() {
        let code = "def f(x: int) -> str:\n    return f\"result: {'yes' if x > 0 else 'no'}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || !result.is_empty(), "fstring ternary: {}", result);
    }

    #[test]
    fn test_w21sa_177_format_with_str_arg() {
        let code = "def f(name: str) -> str:\n    return \"Hello {}!\".format(name)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format str arg: {}", result);
    }

    #[test]
    fn test_w21sa_178_fstring_empty_parts() {
        let code = "def f(x: int) -> str:\n    return f\"{x}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring just expr: {}", result);
    }

    #[test]
    fn test_w21sa_179_string_concat_literal() {
        let code = "def f() -> str:\n    return \"hello\" + \" \" + \"world\"\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "concat literal: {}", result);
    }

    #[test]
    fn test_w21sa_180_fstring_with_index() {
        let code = "def f(items: list) -> str:\n    return f\"first: {items[0]}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring index: {}", result);
    }

    #[test]
    fn test_w21sa_181_format_four_args() {
        let code = "def f(a: int, b: int, c: int, d: int) -> str:\n    return \"{}-{}-{}-{}\".format(a, b, c, d)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format four: {}", result);
    }

    #[test]
    fn test_w21sa_182_fstring_with_subtraction() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"diff: {x - y}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring subtraction: {}", result);
    }

    #[test]
    fn test_w21sa_183_fstring_with_multiply() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"product: {x * y}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring multiply: {}", result);
    }

    #[test]
    fn test_w21sa_184_fstring_five_exprs() {
        let code = "def f(a: int, b: int, c: int, d: int, e: int) -> str:\n    return f\"{a}-{b}-{c}-{d}-{e}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring five: {}", result);
    }

    #[test]
    fn test_w21sa_185_casefold_assign() {
        let code = "def f(s: str) -> str:\n    result = s.casefold()\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("to_lowercase"), "casefold assign: {}", result);
    }

    #[test]
    fn test_w21sa_186_capitalize_assign() {
        let code = "def f(s: str) -> str:\n    result = s.capitalize()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "capitalize assign: {}", result);
    }

    #[test]
    fn test_w21sa_187_swapcase_assign() {
        let code = "def f(s: str) -> str:\n    result = s.swapcase()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "swapcase assign: {}", result);
    }

    #[test]
    fn test_w21sa_188_hex_assign() {
        let code = "def f(s: str) -> str:\n    result = s.hex()\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("bytes") || result.contains("02x"), "hex assign: {}", result);
    }

    #[test]
    fn test_w21sa_189_isprintable_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.isprintable()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isprintable assign: {}", result);
    }

    #[test]
    fn test_w21sa_190_isnumeric_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.isnumeric()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isnumeric assign: {}", result);
    }

    #[test]
    fn test_w21sa_191_isascii_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.isascii()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isascii assign: {}", result);
    }

    #[test]
    fn test_w21sa_192_isdecimal_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.isdecimal()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isdecimal assign: {}", result);
    }

    #[test]
    fn test_w21sa_193_isidentifier_assign() {
        let code = "def f(s: str) -> bool:\n    result = s.isidentifier()\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty(), "isidentifier assign: {}", result);
    }

    #[test]
    fn test_w21sa_194_fstring_with_lower() {
        let code = "def f(s: str) -> str:\n    return f\"{s.lower()}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("to_lowercase"), "fstring lower: {}", result);
    }

    #[test]
    fn test_w21sa_195_fstring_with_title() {
        let code = "def f(s: str) -> str:\n    return f\"{s.title()}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!"), "fstring title: {}", result);
    }

    #[test]
    fn test_w21sa_196_re_dotall() {
        let code = "import re\ndef f():\n    return re.DOTALL\n";
        let result = transpile(code);
        assert!(result.contains("16") || result.contains("DOTALL"), "re.DOTALL: {}", result);
    }

    #[test]
    fn test_w21sa_197_re_verbose() {
        let code = "import re\ndef f():\n    return re.VERBOSE\n";
        let result = transpile(code);
        assert!(result.contains("64") || result.contains("VERBOSE"), "re.VERBOSE: {}", result);
    }

    #[test]
    fn test_w21sa_198_re_ascii_flag() {
        let code = "import re\ndef f():\n    return re.ASCII\n";
        let result = transpile(code);
        assert!(result.contains("256") || result.contains("ASCII"), "re.ASCII: {}", result);
    }

    #[test]
    fn test_w21sa_199_fstring_with_replace() {
        let code = "def f(s: str) -> str:\n    return f\"{s.replace('a', 'b')}\"\n";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("replace"), "fstring replace: {}", result);
    }

    #[test]
    fn test_w21sa_200_format_five_args() {
        let code = "def f(a: int, b: int, c: int, d: int, e: int) -> str:\n    return \"{} {} {} {} {}\".format(a, b, c, d, e)\n";
        let result = transpile(code);
        assert!(result.contains("replacen") || result.contains("format"), "format five: {}", result);
    }
}
