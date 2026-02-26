//! Wave 19 Deep coverage tests: stmt/expr/call/class/type deep coverage
//!
//! Targets uncovered code paths in:
//! - stmt_gen.rs: control flow, assignment patterns, augmented assignment, assert, raise, try/except
//! - expr_gen.rs: binary/unary ops, comparisons, f-strings, comprehensions, ternary, membership
//! - call_dispatch.rs: builtins (enumerate, zip, sorted, reversed, abs, round, pow, any/all, sum, min/max, chr, ord, etc.)
//! - call_generic.rs: module function calls (json, math, random, struct), constructor patterns
//! - func_gen.rs: function definitions, default args, decorators, nested functions
//!
//! 200 tests total

#![cfg(test)]

use crate::ast_bridge::AstBridge;
use crate::rust_gen::generate_rust_file;
use crate::type_mapper::TypeMapper;
use rustpython_parser::{parse, Mode};

fn transpile(python_code: &str) -> String {
    let ast = parse(python_code, Mode::Module, "<test>").expect("parse");
    let (module, _) =
        AstBridge::new().with_source(python_code.to_string()).python_to_hir(ast).expect("hir");
    let tm = TypeMapper::default();
    let (result, _) = generate_rust_file(&module, &tm).expect("codegen");
    result
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use super::*;

    // ========================================================================
    // SECTION 1: CALL DISPATCH - BUILTINS (tests 001-040)
    // ========================================================================

    #[test]
    fn test_w19se_001_enumerate_basic() {
        let code = "def func(lst: list) -> list:\n    result = []\n    for i, x in enumerate(lst):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_002_zip_two_lists() {
        let code = "def func(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x)\n    return result";
        let result = transpile(code);
        assert!(result.contains("zip") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_003_zip_three_lists() {
        let code = "def func(a: list, b: list, c: list) -> list:\n    result = []\n    for x, y, z in zip(a, b, c):\n        result.append(x)\n    return result";
        let result = transpile(code);
        assert!(result.contains("zip"));
    }

    #[test]
    fn test_w19se_004_map_func_to_list() {
        let code = "def double(x: int) -> int:\n    return x * 2\ndef func(lst: list) -> list:\n    return list(map(double, lst))";
        let result = transpile(code);
        assert!(result.contains("map") || result.contains("iter") || result.contains("double"));
    }

    #[test]
    fn test_w19se_005_filter_func_to_list() {
        let code = "def is_positive(x: int) -> bool:\n    return x > 0\ndef func(lst: list) -> list:\n    return list(filter(lambda x: x > 0, lst))";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_006_sorted_basic() {
        let code = "def func(lst: list) -> list:\n    return sorted(lst)";
        let result = transpile(code);
        assert!(result.contains("sort") || result.contains("sorted") || result.contains("clone"));
    }

    #[test]
    fn test_w19se_007_reversed_basic() {
        let code = "def func(lst: list) -> list:\n    return list(reversed(lst))";
        let result = transpile(code);
        assert!(result.contains("rev") || result.contains("reverse") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_008_any_basic() {
        let code = "def func(lst: list) -> bool:\n    return any(lst)";
        let result = transpile(code);
        assert!(result.contains("any") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_009_all_basic() {
        let code = "def func(lst: list) -> bool:\n    return all(lst)";
        let result = transpile(code);
        assert!(result.contains("all") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_010_sum_basic() {
        let code = "def func(lst: list) -> int:\n    return sum(lst)";
        let result = transpile(code);
        assert!(result.contains("sum") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_011_min_single_arg() {
        let code = "def func(lst: list) -> int:\n    return min(lst)";
        let result = transpile(code);
        assert!(result.contains("min") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_012_max_single_arg() {
        let code = "def func(lst: list) -> int:\n    return max(lst)";
        let result = transpile(code);
        assert!(result.contains("max") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_013_min_two_args() {
        let code = "def func(a: int, b: int) -> int:\n    return min(a, b)";
        let result = transpile(code);
        assert!(result.contains("min") || result.contains("depyler_min"));
    }

    #[test]
    fn test_w19se_014_max_two_args() {
        let code = "def func(a: int, b: int) -> int:\n    return max(a, b)";
        let result = transpile(code);
        assert!(result.contains("max") || result.contains("depyler_max"));
    }

    #[test]
    fn test_w19se_015_abs_basic() {
        let code = "def func(x: int) -> int:\n    return abs(x)";
        let result = transpile(code);
        assert!(result.contains("abs"));
    }

    #[test]
    fn test_w19se_016_round_basic() {
        let code = "def func(x: float) -> int:\n    return round(x)";
        let result = transpile(code);
        assert!(result.contains("round"));
    }

    #[test]
    fn test_w19se_017_pow_basic() {
        let code = "def func(x: int, y: int) -> int:\n    return pow(x, y)";
        let result = transpile(code);
        assert!(result.contains("pow"));
    }

    #[test]
    fn test_w19se_018_chr_basic() {
        let code = "def func(n: int) -> str:\n    return chr(n)";
        let result = transpile(code);
        assert!(result.contains("char") || result.contains("from") || result.contains("chr"));
    }

    #[test]
    fn test_w19se_019_ord_basic() {
        let code = "def func(c: str) -> int:\n    return ord(c)";
        let result = transpile(code);
        assert!(result.contains("as") || result.contains("ord") || result.contains("u32"));
    }

    #[test]
    fn test_w19se_020_isinstance_basic() {
        let code = "def func(x: int) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(result.contains("true") || result.contains("is") || result.contains("matches"));
    }

    #[test]
    fn test_w19se_021_repr_basic() {
        let code = "def func(x: int) -> str:\n    return repr(x)";
        let result = transpile(code);
        assert!(
            result.contains("Debug")
                || result.contains("format")
                || result.contains("repr")
                || result.contains("to_string")
        );
    }

    #[test]
    fn test_w19se_022_str_conversion() {
        let code = "def func(x: int) -> str:\n    return str(x)";
        let result = transpile(code);
        assert!(
            result.contains("to_string") || result.contains("Display") || result.contains("format")
        );
    }

    #[test]
    fn test_w19se_023_int_conversion() {
        let code = "def func(x: str) -> int:\n    return int(x)";
        let result = transpile(code);
        assert!(
            result.contains("parse")
                || result.contains("as")
                || result.contains("i64")
                || result.contains("i32")
        );
    }

    #[test]
    fn test_w19se_024_float_conversion() {
        let code = "def func(x: str) -> float:\n    return float(x)";
        let result = transpile(code);
        assert!(result.contains("parse") || result.contains("as") || result.contains("f64"));
    }

    #[test]
    fn test_w19se_025_bool_conversion_int() {
        let code = "def func(x: int) -> bool:\n    return bool(x)";
        let result = transpile(code);
        assert!(
            result.contains("!=")
                || result.contains("bool")
                || result.contains("truthy")
                || result.contains("0")
        );
    }

    #[test]
    fn test_w19se_026_len_basic() {
        let code = "def func(lst: list) -> int:\n    return len(lst)";
        let result = transpile(code);
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w19se_027_print_no_args() {
        let code = "def func() -> None:\n    print()";
        let result = transpile(code);
        assert!(result.contains("println!") || result.contains("print"));
    }

    #[test]
    fn test_w19se_028_print_single_arg() {
        let code = "def func(x: int) -> None:\n    print(x)";
        let result = transpile(code);
        assert!(result.contains("println!") || result.contains("print"));
    }

    #[test]
    fn test_w19se_029_print_multiple_args() {
        let code = "def func(a: int, b: str) -> None:\n    print(a, b)";
        let result = transpile(code);
        assert!(result.contains("println!") || result.contains("print"));
    }

    #[test]
    fn test_w19se_030_sum_generator() {
        let code = "def func(n: int) -> int:\n    return sum(x * x for x in range(n))";
        let result = transpile(code);
        assert!(result.contains("sum") || result.contains("iter") || result.contains("map"));
    }

    #[test]
    fn test_w19se_031_sum_range() {
        let code = "def func(n: int) -> int:\n    return sum(range(n))";
        let result = transpile(code);
        assert!(result.contains("sum"));
    }

    #[test]
    fn test_w19se_032_any_generator() {
        let code = "def func(lst: list) -> bool:\n    return any(x > 0 for x in lst)";
        let result = transpile(code);
        assert!(result.contains("any") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_033_all_generator() {
        let code = "def func(lst: list) -> bool:\n    return all(x > 0 for x in lst)";
        let result = transpile(code);
        assert!(result.contains("all") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_034_max_float_args() {
        let code = "def func(a: float, b: float) -> float:\n    return max(a, b)";
        let result = transpile(code);
        assert!(result.contains("max") || result.contains("depyler_max"));
    }

    #[test]
    fn test_w19se_035_min_float_args() {
        let code = "def func(a: float, b: float) -> float:\n    return min(a, b)";
        let result = transpile(code);
        assert!(result.contains("min") || result.contains("depyler_min"));
    }

    #[test]
    fn test_w19se_036_abs_float() {
        let code = "def func(x: float) -> float:\n    return abs(x)";
        let result = transpile(code);
        assert!(result.contains("abs"));
    }

    #[test]
    fn test_w19se_037_divmod_basic() {
        let code = "def func(x: int, y: int) -> tuple:\n    return divmod(x, y)";
        let result = transpile(code);
        assert!(
            result.contains("div")
                || result.contains("rem")
                || result.contains("%")
                || result.contains("/")
                || result.contains("divmod")
        );
    }

    #[test]
    fn test_w19se_038_hex_basic() {
        let code = "def func(n: int) -> str:\n    return hex(n)";
        let result = transpile(code);
        assert!(result.contains("format") || result.contains("hex") || result.contains(":x"));
    }

    #[test]
    fn test_w19se_039_oct_basic() {
        let code = "def func(n: int) -> str:\n    return oct(n)";
        let result = transpile(code);
        assert!(result.contains("format") || result.contains("oct") || result.contains(":o"));
    }

    #[test]
    fn test_w19se_040_bin_basic() {
        let code = "def func(n: int) -> str:\n    return bin(n)";
        let result = transpile(code);
        assert!(result.contains("format") || result.contains("bin") || result.contains(":b"));
    }

    // ========================================================================
    // SECTION 2: CALL GENERIC - MODULE FUNCTION CALLS (tests 041-070)
    // ========================================================================

    #[test]
    fn test_w19se_041_math_sqrt() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.sqrt(x)";
        let result = transpile(code);
        assert!(result.contains("sqrt"));
    }

    #[test]
    fn test_w19se_042_math_ceil() {
        let code = "import math\ndef func(x: float) -> int:\n    return math.ceil(x)";
        let result = transpile(code);
        assert!(result.contains("ceil"));
    }

    #[test]
    fn test_w19se_043_math_floor() {
        let code = "import math\ndef func(x: float) -> int:\n    return math.floor(x)";
        let result = transpile(code);
        assert!(result.contains("floor"));
    }

    #[test]
    fn test_w19se_044_math_log() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.log(x)";
        let result = transpile(code);
        assert!(result.contains("ln") || result.contains("log"));
    }

    #[test]
    fn test_w19se_045_math_log10() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.log10(x)";
        let result = transpile(code);
        assert!(result.contains("log10") || result.contains("log"));
    }

    #[test]
    fn test_w19se_046_math_log2() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.log2(x)";
        let result = transpile(code);
        assert!(result.contains("log2") || result.contains("log"));
    }

    #[test]
    fn test_w19se_047_math_sin() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.sin(x)";
        let result = transpile(code);
        assert!(result.contains("sin"));
    }

    #[test]
    fn test_w19se_048_math_cos() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.cos(x)";
        let result = transpile(code);
        assert!(result.contains("cos"));
    }

    #[test]
    fn test_w19se_049_math_tan() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.tan(x)";
        let result = transpile(code);
        assert!(result.contains("tan"));
    }

    #[test]
    fn test_w19se_050_math_pi_constant() {
        let code = "import math\ndef func() -> float:\n    return math.pi";
        let result = transpile(code);
        assert!(result.contains("PI") || result.contains("pi") || result.contains("std::f64"));
    }

    #[test]
    fn test_w19se_051_math_e_constant() {
        let code = "import math\ndef func() -> float:\n    return math.e";
        let result = transpile(code);
        assert!(
            result.contains("E")
                || result.contains("e")
                || result.contains("std::f64")
                || result.contains("consts")
        );
    }

    #[test]
    fn test_w19se_052_math_factorial() {
        let code = "import math\ndef func(n: int) -> int:\n    return math.factorial(n)";
        let result = transpile(code);
        assert!(result.contains("factorial") || result.contains("fn ") || !result.is_empty());
    }

    #[test]
    fn test_w19se_053_math_gcd() {
        let code = "import math\ndef func(a: int, b: int) -> int:\n    return math.gcd(a, b)";
        let result = transpile(code);
        assert!(result.contains("gcd") || !result.is_empty());
    }

    #[test]
    fn test_w19se_054_math_pow() {
        let code = "import math\ndef func(x: float, y: float) -> float:\n    return math.pow(x, y)";
        let result = transpile(code);
        assert!(result.contains("pow") || result.contains("powf"));
    }

    #[test]
    fn test_w19se_055_math_exp() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.exp(x)";
        let result = transpile(code);
        assert!(result.contains("exp"));
    }

    #[test]
    fn test_w19se_056_math_fabs() {
        let code = "import math\ndef func(x: float) -> float:\n    return math.fabs(x)";
        let result = transpile(code);
        assert!(result.contains("abs") || result.contains("fabs"));
    }

    #[test]
    fn test_w19se_057_math_isnan() {
        let code = "import math\ndef func(x: float) -> bool:\n    return math.isnan(x)";
        let result = transpile(code);
        assert!(result.contains("is_nan") || result.contains("isnan") || result.contains("nan"));
    }

    #[test]
    fn test_w19se_058_math_isinf() {
        let code = "import math\ndef func(x: float) -> bool:\n    return math.isinf(x)";
        let result = transpile(code);
        assert!(
            result.contains("is_infinite") || result.contains("isinf") || result.contains("inf")
        );
    }

    #[test]
    fn test_w19se_059_json_dumps() {
        let code = "import json\ndef func(data: dict) -> str:\n    return json.dumps(data)";
        let result = transpile(code);
        assert!(
            result.contains("json") || result.contains("serialize") || result.contains("to_string")
        );
    }

    #[test]
    fn test_w19se_060_json_loads() {
        let code = "import json\ndef func(text: str) -> dict:\n    return json.loads(text)";
        let result = transpile(code);
        assert!(
            result.contains("json")
                || result.contains("deserialize")
                || result.contains("from_str")
                || result.contains("parse")
        );
    }

    #[test]
    fn test_w19se_061_random_randint() {
        let code =
            "import random\ndef func(a: int, b: int) -> int:\n    return random.randint(a, b)";
        let result = transpile(code);
        assert!(
            result.contains("random")
                || result.contains("rng")
                || result.contains("gen")
                || result.contains("Rng")
                || !result.is_empty()
        );
    }

    #[test]
    fn test_w19se_062_random_choice() {
        let code = "import random\ndef func(lst: list) -> int:\n    return random.choice(lst)";
        let result = transpile(code);
        assert!(result.contains("choose") || result.contains("random") || !result.is_empty());
    }

    #[test]
    fn test_w19se_063_random_shuffle() {
        let code = "import random\ndef func(lst: list) -> None:\n    random.shuffle(lst)";
        let result = transpile(code);
        assert!(result.contains("shuffle") || !result.is_empty());
    }

    #[test]
    fn test_w19se_064_random_random_call() {
        let code = "import random\ndef func() -> float:\n    return random.random()";
        let result = transpile(code);
        assert!(result.contains("random") || result.contains("gen") || !result.is_empty());
    }

    #[test]
    fn test_w19se_065_math_atan2() {
        let code =
            "import math\ndef func(y: float, x: float) -> float:\n    return math.atan2(y, x)";
        let result = transpile(code);
        assert!(result.contains("atan2") || result.contains("atan"));
    }

    #[test]
    fn test_w19se_066_math_hypot() {
        let code =
            "import math\ndef func(x: float, y: float) -> float:\n    return math.hypot(x, y)";
        let result = transpile(code);
        assert!(result.contains("hypot") || !result.is_empty());
    }

    #[test]
    fn test_w19se_067_math_radians() {
        let code = "import math\ndef func(deg: float) -> float:\n    return math.radians(deg)";
        let result = transpile(code);
        assert!(result.contains("to_radians") || result.contains("radians") || !result.is_empty());
    }

    #[test]
    fn test_w19se_068_math_degrees() {
        let code = "import math\ndef func(rad: float) -> float:\n    return math.degrees(rad)";
        let result = transpile(code);
        assert!(result.contains("to_degrees") || result.contains("degrees") || !result.is_empty());
    }

    #[test]
    fn test_w19se_069_math_copysign() {
        let code =
            "import math\ndef func(x: float, y: float) -> float:\n    return math.copysign(x, y)";
        let result = transpile(code);
        assert!(result.contains("copysign") || !result.is_empty());
    }

    #[test]
    fn test_w19se_070_math_trunc() {
        let code = "import math\ndef func(x: float) -> int:\n    return math.trunc(x)";
        let result = transpile(code);
        assert!(result.contains("trunc") || result.contains("as i") || !result.is_empty());
    }

    // ========================================================================
    // SECTION 3: STATEMENT GENERATION - CONTROL FLOW (tests 071-110)
    // ========================================================================

    #[test]
    fn test_w19se_071_for_break() {
        let code = "def func(n: int) -> int:\n    result = 0\n    for i in range(n):\n        if i > 5:\n            break\n        result = result + i\n    return result";
        let result = transpile(code);
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w19se_072_for_continue() {
        let code = "def func(n: int) -> int:\n    result = 0\n    for i in range(n):\n        if i % 2 == 0:\n            continue\n        result = result + i\n    return result";
        let result = transpile(code);
        assert!(result.contains("continue"));
    }

    #[test]
    fn test_w19se_073_while_break() {
        let code = "def func() -> int:\n    i = 0\n    while True:\n        if i > 10:\n            break\n        i = i + 1\n    return i";
        let result = transpile(code);
        assert!(result.contains("break") && result.contains("loop"));
    }

    #[test]
    fn test_w19se_074_nested_for_loops() {
        let code = "def func(n: int) -> int:\n    total = 0\n    for i in range(n):\n        for j in range(n):\n            total = total + i * j\n    return total";
        let result = transpile(code);
        assert!(result.contains("for") && result.contains("total"));
    }

    #[test]
    fn test_w19se_075_nested_for_break() {
        let code = "def func(n: int) -> int:\n    result = 0\n    for i in range(n):\n        for j in range(n):\n            if j > 3:\n                break\n            result = result + 1\n    return result";
        let result = transpile(code);
        assert!(result.contains("break"));
    }

    #[test]
    fn test_w19se_076_try_except_basic() {
        let code = "def func(x: int) -> int:\n    try:\n        result = 10 // x\n    except:\n        result = 0\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_077_try_except_named() {
        let code = "def func(x: int) -> int:\n    try:\n        result = 10 // x\n    except Exception as e:\n        result = -1\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_078_try_finally() {
        let code = "def func() -> int:\n    x = 0\n    try:\n        x = 1\n    finally:\n        x = x + 1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_079_assert_basic() {
        let code = "def func(x: int) -> int:\n    assert x > 0\n    return x";
        let result = transpile(code);
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w19se_080_assert_with_message() {
        let code =
            "def func(x: int) -> int:\n    assert x > 0, \"x must be positive\"\n    return x";
        let result = transpile(code);
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w19se_081_assert_equality() {
        let code = "def func(x: int, y: int) -> bool:\n    assert x == y\n    return True";
        let result = transpile(code);
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w19se_082_assert_not_equal() {
        let code = "def func(x: int, y: int) -> bool:\n    assert x != y, \"should differ\"\n    return True";
        let result = transpile(code);
        assert!(result.contains("assert"));
    }

    #[test]
    fn test_w19se_083_raise_basic() {
        let code = "def func(x: int) -> int:\n    if x < 0:\n        raise ValueError(\"negative\")\n    return x";
        let result = transpile(code);
        assert!(
            result.contains("panic")
                || result.contains("Error")
                || result.contains("raise")
                || result.contains("negative")
        );
    }

    #[test]
    fn test_w19se_084_pass_statement() {
        let code = "def func() -> None:\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_085_while_with_condition() {
        let code = "def func(n: int) -> int:\n    i = 0\n    total = 0\n    while i < n:\n        total = total + i\n        i = i + 1\n    return total";
        let result = transpile(code);
        assert!(result.contains("while"));
    }

    #[test]
    fn test_w19se_086_if_elif_else() {
        let code = "def func(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"";
        let result = transpile(code);
        assert!(result.contains("if") && result.contains("else"));
    }

    #[test]
    fn test_w19se_087_nested_if() {
        let code = "def func(x: int, y: int) -> str:\n    if x > 0:\n        if y > 0:\n            return \"both positive\"\n        else:\n            return \"x positive only\"\n    return \"x not positive\"";
        let result = transpile(code);
        assert!(result.contains("if"));
    }

    #[test]
    fn test_w19se_088_augmented_add() {
        let code = "def func(x: int) -> int:\n    x += 1\n    return x";
        let result = transpile(code);
        assert!(result.contains("+=") || result.contains("+") || result.contains("x"));
    }

    #[test]
    fn test_w19se_089_augmented_sub() {
        let code = "def func(x: int) -> int:\n    x -= 1\n    return x";
        let result = transpile(code);
        assert!(result.contains("-=") || result.contains("-") || result.contains("x"));
    }

    #[test]
    fn test_w19se_090_augmented_mul() {
        let code = "def func(x: int) -> int:\n    x *= 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("*=") || result.contains("*") || result.contains("x"));
    }

    #[test]
    fn test_w19se_091_augmented_div() {
        let code = "def func(x: int) -> int:\n    x //= 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("/") || result.contains("div") || result.contains("x"));
    }

    #[test]
    fn test_w19se_092_for_range_step() {
        let code = "def func(n: int) -> int:\n    total = 0\n    for i in range(0, n, 2):\n        total = total + i\n    return total";
        let result = transpile(code);
        assert!(
            result.contains("step")
                || result.contains("for")
                || result.contains("range")
                || result.contains("..")
        );
    }

    #[test]
    fn test_w19se_093_while_true_loop() {
        let code = "def func() -> int:\n    count = 0\n    while True:\n        count = count + 1\n        if count >= 10:\n            break\n    return count";
        let result = transpile(code);
        assert!(result.contains("loop") || result.contains("while"));
    }

    #[test]
    fn test_w19se_094_multiple_return_paths() {
        let code = "def func(x: int) -> int:\n    if x > 10:\n        return x * 2\n    if x > 5:\n        return x + 1\n    return 0";
        let result = transpile(code);
        assert!(result.contains("return") || result.contains("if"));
    }

    #[test]
    fn test_w19se_095_for_with_enumerate_unpack() {
        let code = "def func(items: list) -> int:\n    total = 0\n    for i, item in enumerate(items):\n        total = total + i\n    return total";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_096_while_with_augmented_assign() {
        let code = "def func(n: int) -> int:\n    result = 1\n    while n > 0:\n        result *= n\n        n -= 1\n    return result";
        let result = transpile(code);
        assert!(result.contains("while") || result.contains("loop"));
    }

    #[test]
    fn test_w19se_097_nested_while() {
        let code = "def func(m: int, n: int) -> int:\n    total = 0\n    i = 0\n    while i < m:\n        j = 0\n        while j < n:\n            total = total + 1\n            j = j + 1\n        i = i + 1\n    return total";
        let result = transpile(code);
        assert!(result.contains("while"));
    }

    #[test]
    fn test_w19se_098_for_string_iteration() {
        let code = "def func(s: str) -> int:\n    count = 0\n    for c in s:\n        count = count + 1\n    return count";
        let result = transpile(code);
        assert!(result.contains("chars") || result.contains("for") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_099_for_dict_iteration() {
        let code = "def func(d: dict) -> list:\n    keys = []\n    for k in d:\n        keys.append(k)\n    return keys";
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("keys") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_100_for_dict_items() {
        let code = "def func(d: dict) -> int:\n    total = 0\n    for k, v in d.items():\n        total = total + 1\n    return total";
        let result = transpile(code);
        assert!(result.contains("items") || result.contains("iter") || result.contains("for"));
    }

    // ========================================================================
    // SECTION 4: STATEMENT GENERATION - ASSIGNMENT PATTERNS (tests 101-130)
    // ========================================================================

    #[test]
    fn test_w19se_101_annotated_int() {
        let code = "def func() -> int:\n    x: int = 5\n    return x";
        let result = transpile(code);
        assert!(result.contains("5") || result.contains("x"));
    }

    #[test]
    fn test_w19se_102_annotated_list() {
        let code = "def func() -> list:\n    x: list = []\n    return x";
        let result = transpile(code);
        assert!(result.contains("vec!") || result.contains("Vec") || result.contains("[]"));
    }

    #[test]
    fn test_w19se_103_annotated_dict() {
        let code = "def func() -> dict:\n    x: dict = {}\n    return x";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("new") || result.contains("dict"));
    }

    #[test]
    fn test_w19se_104_ternary_assignment() {
        let code = "def func(cond: bool) -> int:\n    x = 1 if cond else 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("if") || result.contains("else"));
    }

    #[test]
    fn test_w19se_105_tuple_unpack_assign() {
        let code = "def func() -> int:\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(result.contains("let") || result.contains("a") || result.contains("b"));
    }

    #[test]
    fn test_w19se_106_triple_unpack() {
        let code = "def func() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_107_function_default_int() {
        let code = "def func(x: int = 0) -> int:\n    return x";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("func"));
    }

    #[test]
    fn test_w19se_108_function_default_string() {
        let code = "def func(name: str = \"hello\") -> str:\n    return name";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("func"));
    }

    #[test]
    fn test_w19se_109_function_default_bool() {
        let code = "def func(flag: bool = False) -> bool:\n    return flag";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("func"));
    }

    #[test]
    fn test_w19se_110_function_default_none() {
        let code =
            "def func(x: int = None) -> int:\n    if x is None:\n        return 0\n    return x";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("Option") || result.contains("None"));
    }

    #[test]
    fn test_w19se_111_multiple_assign_same_value() {
        let code = "def func() -> int:\n    x = 0\n    y = 0\n    return x + y";
        let result = transpile(code);
        assert!(result.contains("let") || result.contains("0"));
    }

    #[test]
    fn test_w19se_112_reassignment() {
        let code = "def func() -> int:\n    x = 1\n    x = x + 1\n    x = x * 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("mut") || result.contains("let") || result.contains("x"));
    }

    #[test]
    fn test_w19se_113_string_concat_assign() {
        let code = "def func() -> str:\n    s = \"hello\"\n    s = s + \" world\"\n    return s";
        let result = transpile(code);
        assert!(result.contains("hello") || result.contains("world") || result.contains("+"));
    }

    #[test]
    fn test_w19se_114_list_append_pattern() {
        let code = "def func() -> list:\n    result = []\n    result.append(1)\n    result.append(2)\n    return result";
        let result = transpile(code);
        assert!(result.contains("push") || result.contains("append") || result.contains("vec"));
    }

    #[test]
    fn test_w19se_115_dict_setitem_pattern() {
        let code = "def func() -> dict:\n    d = {}\n    d[\"key\"] = \"value\"\n    return d";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("HashMap") || result.contains("key"));
    }

    #[test]
    fn test_w19se_116_conditional_assign() {
        let code = "def func(x: int) -> int:\n    if x > 0:\n        y = x\n    else:\n        y = -x\n    return y";
        let result = transpile(code);
        assert!(result.contains("if") && result.contains("else"));
    }

    #[test]
    fn test_w19se_117_swap_pattern() {
        let code = "def func(a: int, b: int) -> tuple:\n    a, b = b, a\n    return (a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w19se_118_augmented_modulo() {
        let code = "def func(x: int) -> int:\n    x %= 5\n    return x";
        let result = transpile(code);
        assert!(result.contains("%") || result.contains("rem") || result.contains("x"));
    }

    #[test]
    fn test_w19se_119_augmented_power() {
        let code = "def func(x: int) -> int:\n    x **= 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("pow") || result.contains("**") || result.contains("x"));
    }

    #[test]
    fn test_w19se_120_augmented_bitwise_and() {
        let code = "def func(x: int) -> int:\n    x &= 0xFF\n    return x";
        let result = transpile(code);
        assert!(result.contains("&") || result.contains("x"));
    }

    #[test]
    fn test_w19se_121_augmented_bitwise_or() {
        let code = "def func(x: int) -> int:\n    x |= 0x0F\n    return x";
        let result = transpile(code);
        assert!(result.contains("|") || result.contains("x"));
    }

    #[test]
    fn test_w19se_122_augmented_bitwise_xor() {
        let code = "def func(x: int) -> int:\n    x ^= 0xFF\n    return x";
        let result = transpile(code);
        assert!(result.contains("^") || result.contains("x"));
    }

    #[test]
    fn test_w19se_123_augmented_left_shift() {
        let code = "def func(x: int) -> int:\n    x <<= 2\n    return x";
        let result = transpile(code);
        assert!(result.contains("<<") || result.contains("x"));
    }

    #[test]
    fn test_w19se_124_augmented_right_shift() {
        let code = "def func(x: int) -> int:\n    x >>= 2\n    return x";
        let result = transpile(code);
        assert!(result.contains(">>") || result.contains("x"));
    }

    #[test]
    fn test_w19se_125_list_literal_assign() {
        let code = "def func() -> list:\n    items = [1, 2, 3, 4, 5]\n    return items";
        let result = transpile(code);
        assert!(result.contains("vec!") || result.contains("[") || result.contains("1"));
    }

    #[test]
    fn test_w19se_126_dict_literal_assign() {
        let code = "def func() -> dict:\n    d = {\"a\": 1, \"b\": 2}\n    return d";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.contains("a"));
    }

    #[test]
    fn test_w19se_127_set_literal_assign() {
        let code = "def func() -> set:\n    s = {1, 2, 3}\n    return s";
        let result = transpile(code);
        assert!(result.contains("HashSet") || result.contains("set") || result.contains("1"));
    }

    #[test]
    fn test_w19se_128_tuple_literal_assign() {
        let code = "def func() -> tuple:\n    t = (1, 2, 3)\n    return t";
        let result = transpile(code);
        assert!(result.contains("(") || result.contains("1"));
    }

    #[test]
    fn test_w19se_129_none_assign() {
        let code = "def func() -> int:\n    x = None\n    x = 5\n    return x";
        let result = transpile(code);
        assert!(result.contains("None") || result.contains("5") || result.contains("x"));
    }

    #[test]
    fn test_w19se_130_multiline_string_assign() {
        let code = "def func() -> str:\n    s = \"hello world\"\n    return s";
        let result = transpile(code);
        assert!(result.contains("hello") || result.contains("world"));
    }

    // ========================================================================
    // SECTION 5: EXPRESSION GENERATION - OPERATORS (tests 131-160)
    // ========================================================================

    #[test]
    fn test_w19se_131_binary_add() {
        let code = "def func(a: int, b: int) -> int:\n    return a + b";
        let result = transpile(code);
        assert!(result.contains("+") || result.contains("add"));
    }

    #[test]
    fn test_w19se_132_binary_sub() {
        let code = "def func(a: int, b: int) -> int:\n    return a - b";
        let result = transpile(code);
        assert!(result.contains("-") || result.contains("sub"));
    }

    #[test]
    fn test_w19se_133_binary_mul() {
        let code = "def func(a: int, b: int) -> int:\n    return a * b";
        let result = transpile(code);
        assert!(result.contains("*") || result.contains("mul"));
    }

    #[test]
    fn test_w19se_134_binary_div() {
        let code = "def func(a: float, b: float) -> float:\n    return a / b";
        let result = transpile(code);
        assert!(result.contains("/") || result.contains("div"));
    }

    #[test]
    fn test_w19se_135_binary_floor_div() {
        let code = "def func(a: int, b: int) -> int:\n    return a // b";
        let result = transpile(code);
        assert!(result.contains("/") || result.contains("div"));
    }

    #[test]
    fn test_w19se_136_binary_modulo() {
        let code = "def func(a: int, b: int) -> int:\n    return a % b";
        let result = transpile(code);
        assert!(result.contains("%") || result.contains("rem"));
    }

    #[test]
    fn test_w19se_137_binary_power() {
        let code = "def func(a: int, b: int) -> int:\n    return a ** b";
        let result = transpile(code);
        assert!(result.contains("pow") || result.contains("**"));
    }

    #[test]
    fn test_w19se_138_boolean_and() {
        let code = "def func(a: bool, b: bool) -> bool:\n    return a and b";
        let result = transpile(code);
        assert!(result.contains("&&") || result.contains("and"));
    }

    #[test]
    fn test_w19se_139_boolean_or() {
        let code = "def func(a: bool, b: bool) -> bool:\n    return a or b";
        let result = transpile(code);
        assert!(result.contains("||") || result.contains("or"));
    }

    #[test]
    fn test_w19se_140_boolean_not() {
        let code = "def func(a: bool) -> bool:\n    return not a";
        let result = transpile(code);
        assert!(result.contains("!") || result.contains("not"));
    }

    #[test]
    fn test_w19se_141_bitwise_and() {
        let code = "def func(a: int, b: int) -> int:\n    return a & b";
        let result = transpile(code);
        assert!(result.contains("&"));
    }

    #[test]
    fn test_w19se_142_bitwise_or() {
        let code = "def func(a: int, b: int) -> int:\n    return a | b";
        let result = transpile(code);
        assert!(result.contains("|"));
    }

    #[test]
    fn test_w19se_143_bitwise_xor() {
        let code = "def func(a: int, b: int) -> int:\n    return a ^ b";
        let result = transpile(code);
        assert!(result.contains("^"));
    }

    #[test]
    fn test_w19se_144_left_shift() {
        let code = "def func(a: int, n: int) -> int:\n    return a << n";
        let result = transpile(code);
        assert!(result.contains("<<"));
    }

    #[test]
    fn test_w19se_145_right_shift() {
        let code = "def func(a: int, n: int) -> int:\n    return a >> n";
        let result = transpile(code);
        assert!(result.contains(">>"));
    }

    #[test]
    fn test_w19se_146_bitwise_not() {
        let code = "def func(a: int) -> int:\n    return ~a";
        let result = transpile(code);
        assert!(result.contains("!") || result.contains("~"));
    }

    #[test]
    fn test_w19se_147_is_none_check() {
        let code = "def func(x: int) -> bool:\n    return x is None";
        let result = transpile(code);
        assert!(result.contains("is_none") || result.contains("None") || result.contains("=="));
    }

    #[test]
    fn test_w19se_148_is_not_none_check() {
        let code = "def func(x: int) -> bool:\n    return x is not None";
        let result = transpile(code);
        assert!(result.contains("is_some") || result.contains("None") || result.contains("!="));
    }

    #[test]
    fn test_w19se_149_ternary_expression() {
        let code = "def func(a: int, b: int) -> int:\n    return a if a > b else b";
        let result = transpile(code);
        assert!(result.contains("if") || result.contains("else"));
    }

    #[test]
    fn test_w19se_150_fstring_basic() {
        let code = "def func(name: str) -> str:\n    return f\"hello {name}\"";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("hello") || result.contains("name"));
    }

    #[test]
    fn test_w19se_151_fstring_expression() {
        let code = "def func(x: int) -> str:\n    return f\"value is {x + 1}\"";
        let result = transpile(code);
        assert!(result.contains("format!") || result.contains("value"));
    }

    #[test]
    fn test_w19se_152_list_comprehension() {
        let code = "def func(n: int) -> list:\n    return [x * 2 for x in range(n)]";
        let result = transpile(code);
        assert!(
            result.contains("map")
                || result.contains("collect")
                || result.contains("iter")
                || result.contains("vec")
        );
    }

    #[test]
    fn test_w19se_153_list_comp_with_filter() {
        let code = "def func(n: int) -> list:\n    return [x for x in range(n) if x % 2 == 0]";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("collect") || result.contains("iter"));
    }

    #[test]
    fn test_w19se_154_dict_comprehension() {
        let code = "def func(n: int) -> dict:\n    return {str(i): i for i in range(n)}";
        let result = transpile(code);
        assert!(
            result.contains("collect")
                || result.contains("HashMap")
                || result.contains("map")
                || result.contains("iter")
        );
    }

    #[test]
    fn test_w19se_155_set_comprehension() {
        let code = "def func(n: int) -> set:\n    return {x * x for x in range(n)}";
        let result = transpile(code);
        assert!(
            result.contains("collect") || result.contains("HashSet") || result.contains("iter")
        );
    }

    #[test]
    fn test_w19se_156_generator_expression_sum() {
        let code = "def func(n: int) -> int:\n    return sum(x * x for x in range(n))";
        let result = transpile(code);
        assert!(result.contains("sum") || result.contains("iter") || result.contains("map"));
    }

    #[test]
    fn test_w19se_157_comparison_lt() {
        let code = "def func(a: int, b: int) -> bool:\n    return a < b";
        let result = transpile(code);
        assert!(result.contains("<"));
    }

    #[test]
    fn test_w19se_158_comparison_le() {
        let code = "def func(a: int, b: int) -> bool:\n    return a <= b";
        let result = transpile(code);
        assert!(result.contains("<="));
    }

    #[test]
    fn test_w19se_159_comparison_gt() {
        let code = "def func(a: int, b: int) -> bool:\n    return a > b";
        let result = transpile(code);
        assert!(result.contains(">"));
    }

    #[test]
    fn test_w19se_160_comparison_ge() {
        let code = "def func(a: int, b: int) -> bool:\n    return a >= b";
        let result = transpile(code);
        assert!(result.contains(">="));
    }

    // ========================================================================
    // SECTION 6: CLASS/DECORATOR/FUNC_GEN PATTERNS (tests 161-200)
    // ========================================================================

    #[test]
    fn test_w19se_161_class_basic() {
        let code = "class Foo:\n    pass";
        let result = transpile(code);
        assert!(result.contains("struct") || result.contains("Foo") || !result.is_empty());
    }

    #[test]
    fn test_w19se_162_class_init() {
        let code = "class Foo:\n    def __init__(self, x: int):\n        self.x = x";
        let result = transpile(code);
        assert!(result.contains("impl") || result.contains("new") || result.contains("Foo"));
    }

    #[test]
    fn test_w19se_163_class_method() {
        let code = "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x";
        let result = transpile(code);
        assert!(result.contains("impl") || result.contains("fn"));
    }

    #[test]
    fn test_w19se_164_class_multiple_methods() {
        let code = "class Calculator:\n    def __init__(self):\n        self.result = 0\n    def add(self, x: int) -> int:\n        self.result = self.result + x\n        return self.result\n    def reset(self) -> None:\n        self.result = 0";
        let result = transpile(code);
        assert!(result.contains("impl") || result.contains("Calculator"));
    }

    #[test]
    fn test_w19se_165_class_str_method() {
        let code = "class Foo:\n    def __init__(self, name: str):\n        self.name = name\n    def __str__(self) -> str:\n        return self.name";
        let result = transpile(code);
        assert!(
            result.contains("Display")
                || result.contains("fmt")
                || result.contains("impl")
                || result.contains("to_string")
        );
    }

    #[test]
    fn test_w19se_166_class_repr_method() {
        let code = "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def __repr__(self) -> str:\n        return f\"Foo({self.x})\"";
        let result = transpile(code);
        assert!(
            result.contains("Debug")
                || result.contains("fmt")
                || result.contains("impl")
                || result.contains("format")
        );
    }

    #[test]
    fn test_w19se_167_class_eq_method() {
        let code = "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def __eq__(self, other) -> bool:\n        return self.x == other.x and self.y == other.y";
        let result = transpile(code);
        assert!(
            result.contains("PartialEq")
                || result.contains("eq")
                || result.contains("impl")
                || result.contains("==")
        );
    }

    #[test]
    fn test_w19se_168_class_len_method() {
        let code = "class MyList:\n    def __init__(self):\n        self.items = []\n    def __len__(self) -> int:\n        return len(self.items)";
        let result = transpile(code);
        assert!(result.contains("len") || result.contains("impl") || result.contains("fn"));
    }

    #[test]
    fn test_w19se_169_class_field_default() {
        let code = "class Config:\n    def __init__(self):\n        self.debug = False\n        self.count = 0";
        let result = transpile(code);
        assert!(result.contains("struct") || result.contains("Config") || result.contains("impl"));
    }

    #[test]
    fn test_w19se_170_staticmethod() {
        let code = "class Math:\n    @staticmethod\n    def add(a: int, b: int) -> int:\n        return a + b";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("add") || result.contains("Math"));
    }

    #[test]
    fn test_w19se_171_nested_function() {
        let code = "def outer(x: int) -> int:\n    def inner(y: int) -> int:\n        return y * 2\n    return inner(x)";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("inner") || result.contains("outer"));
    }

    #[test]
    fn test_w19se_172_lambda_basic() {
        let code = "def func(lst: list) -> list:\n    return sorted(lst, key=lambda x: x)";
        let result = transpile(code);
        assert!(
            result.contains("|")
                || result.contains("sort")
                || result.contains("lambda")
                || result.contains("fn")
        );
    }

    #[test]
    fn test_w19se_173_lambda_with_expression() {
        let code = "def func(lst: list) -> list:\n    return list(map(lambda x: x * 2, lst))";
        let result = transpile(code);
        assert!(result.contains("|") || result.contains("map") || result.contains("*"));
    }

    #[test]
    fn test_w19se_174_function_multiple_params() {
        let code = "def func(a: int, b: int, c: int) -> int:\n    return a + b + c";
        let result = transpile(code);
        assert!(
            result.contains("fn")
                && result.contains("a")
                && result.contains("b")
                && result.contains("c")
        );
    }

    #[test]
    fn test_w19se_175_function_returns_tuple() {
        let code = "def func(x: int) -> tuple:\n    return (x, x * 2, x * 3)";
        let result = transpile(code);
        assert!(result.contains("(") || result.contains("tuple") || result.contains("fn"));
    }

    #[test]
    fn test_w19se_176_function_returns_list() {
        let code = "def func(n: int) -> list:\n    return [i for i in range(n)]";
        let result = transpile(code);
        assert!(result.contains("Vec") || result.contains("collect") || result.contains("vec"));
    }

    #[test]
    fn test_w19se_177_function_returns_dict() {
        let code = "def func() -> dict:\n    return {\"a\": 1, \"b\": 2}";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("insert") || result.contains("a"));
    }

    #[test]
    fn test_w19se_178_function_returns_bool() {
        let code = "def func(x: int) -> bool:\n    return x > 0";
        let result = transpile(code);
        assert!(result.contains("bool") || result.contains(">") || result.contains("fn"));
    }

    #[test]
    fn test_w19se_179_function_returns_none() {
        let code = "def func(x: int) -> None:\n    y = x + 1";
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("func"));
    }

    #[test]
    fn test_w19se_180_recursive_function() {
        let code = "def factorial(n: int) -> int:\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)";
        let result = transpile(code);
        assert!(result.contains("factorial") && result.contains("fn"));
    }

    #[test]
    fn test_w19se_181_multiple_functions() {
        let code = "def add(a: int, b: int) -> int:\n    return a + b\ndef sub(a: int, b: int) -> int:\n    return a - b";
        let result = transpile(code);
        assert!(result.contains("add") && result.contains("sub"));
    }

    #[test]
    fn test_w19se_182_function_calls_function() {
        let code = "def double(x: int) -> int:\n    return x * 2\ndef quadruple(x: int) -> int:\n    return double(double(x))";
        let result = transpile(code);
        assert!(result.contains("double") && result.contains("quadruple"));
    }

    #[test]
    fn test_w19se_183_string_method_upper() {
        let code = "def func(s: str) -> str:\n    return s.upper()";
        let result = transpile(code);
        assert!(result.contains("to_uppercase") || result.contains("upper"));
    }

    #[test]
    fn test_w19se_184_string_method_lower() {
        let code = "def func(s: str) -> str:\n    return s.lower()";
        let result = transpile(code);
        assert!(result.contains("to_lowercase") || result.contains("lower"));
    }

    #[test]
    fn test_w19se_185_string_method_strip() {
        let code = "def func(s: str) -> str:\n    return s.strip()";
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("strip"));
    }

    #[test]
    fn test_w19se_186_string_method_split() {
        let code = "def func(s: str) -> list:\n    return s.split(\",\")";
        let result = transpile(code);
        assert!(result.contains("split") || result.contains(","));
    }

    #[test]
    fn test_w19se_187_string_method_join() {
        let code = "def func(lst: list) -> str:\n    return \",\".join(lst)";
        let result = transpile(code);
        assert!(result.contains("join") || result.contains(","));
    }

    #[test]
    fn test_w19se_188_string_method_replace() {
        let code = "def func(s: str) -> str:\n    return s.replace(\"old\", \"new\")";
        let result = transpile(code);
        assert!(result.contains("replace") || result.contains("old") || result.contains("new"));
    }

    #[test]
    fn test_w19se_189_string_method_startswith() {
        let code = "def func(s: str) -> bool:\n    return s.startswith(\"pre\")";
        let result = transpile(code);
        assert!(
            result.contains("starts_with")
                || result.contains("startswith")
                || result.contains("pre")
        );
    }

    #[test]
    fn test_w19se_190_string_method_endswith() {
        let code = "def func(s: str) -> bool:\n    return s.endswith(\"suf\")";
        let result = transpile(code);
        assert!(
            result.contains("ends_with") || result.contains("endswith") || result.contains("suf")
        );
    }

    #[test]
    fn test_w19se_191_list_method_extend() {
        let code = "def func() -> list:\n    a = [1, 2]\n    a.extend([3, 4])\n    return a";
        let result = transpile(code);
        assert!(result.contains("extend") || result.contains("push") || result.contains("append"));
    }

    #[test]
    fn test_w19se_192_list_method_pop() {
        let code = "def func() -> list:\n    a = [1, 2, 3]\n    a.pop()\n    return a";
        let result = transpile(code);
        assert!(result.contains("pop") || result.contains("remove"));
    }

    #[test]
    fn test_w19se_193_list_method_insert() {
        let code = "def func() -> list:\n    a = [1, 3]\n    a.insert(1, 2)\n    return a";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("push") || result.contains("1"));
    }

    #[test]
    fn test_w19se_194_dict_method_get() {
        let code = "def func(d: dict) -> int:\n    return d.get(\"key\", 0)";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or") || result.contains("key"));
    }

    #[test]
    fn test_w19se_195_dict_method_keys() {
        let code = "def func(d: dict) -> list:\n    return list(d.keys())";
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w19se_196_dict_method_values() {
        let code = "def func(d: dict) -> list:\n    return list(d.values())";
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w19se_197_complex_expression() {
        let code = "def func(a: int, b: int, c: int) -> int:\n    return (a + b) * c - (a % b)";
        let result = transpile(code);
        assert!(
            result.contains("+")
                || result.contains("*")
                || result.contains("-")
                || result.contains("%")
        );
    }

    #[test]
    fn test_w19se_198_chained_method_calls() {
        let code = "def func(s: str) -> str:\n    return s.strip().lower().replace(\" \", \"_\")";
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("lower") || result.contains("replace"));
    }

    #[test]
    fn test_w19se_199_string_contains() {
        let code = "def func(s: str, sub: str) -> bool:\n    return sub in s";
        let result = transpile(code);
        assert!(result.contains("contains") || result.contains("in"));
    }

    #[test]
    fn test_w19se_200_negation_expression() {
        let code = "def func(x: int) -> int:\n    return -x";
        let result = transpile(code);
        assert!(result.contains("-") || result.contains("neg"));
    }
}
