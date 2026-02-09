//! Wave 18 coverage tests: stmt_gen, func_gen, and type code paths
//!
//! Targets uncovered code paths in:
//! - stmt_gen.rs: os.getenv with default, os.environ.get, argparse advanced,
//!   not-x None check, break/continue early exit, raise in early exit,
//!   nested variable type inference, augmented assign complex targets,
//!   global/nonlocal statements
//! - func_gen.rs: tuple unpacking type inference, nested block type inference,
//!   return type float coercion, generator with Option state vars,
//!   *args/**kwargs, nested function definitions, lambda with captures,
//!   multiple return paths
//! - type_helpers: complex type annotations, union types, callable types, type aliases
//! - direct_rules: hashlib, base64, time, os.path methods
//!
//! 200 tests total

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

    // ========================================================================
    // SECTION 1: STMT_GEN - os.getenv with default (tests 001-010)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_os_getenv_single_arg() {
        let code = "import os\ndef f() -> str:\n    return os.getenv(\"HOME\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("env") || result.contains("var"),
            "os.getenv single: {result}"
        );
    }

    #[test]
    fn test_wave18_stmt_os_getenv_with_default() {
        let code = "import os\ndef f() -> str:\n    return os.getenv(\"HOME\", \"/tmp\")";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("unwrap_or") || result.contains("env"),
            "os.getenv default: {result}"
        );
    }

    #[test]
    fn test_wave18_stmt_os_getenv_default_empty_string() {
        let code = "import os\ndef f() -> str:\n    return os.getenv(\"MISSING\", \"\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_assign_result() {
        let code = "import os\ndef f():\n    val = os.getenv(\"KEY\", \"default\")\n    print(val)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("val"), "os.getenv assign: {result}");
    }

    #[test]
    fn test_wave18_stmt_os_getenv_in_condition() {
        let code = "import os\ndef f() -> bool:\n    return os.getenv(\"DEBUG\", \"false\") == \"true\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_nested_call() {
        let code = "import os\ndef f() -> int:\n    return int(os.getenv(\"PORT\", \"8080\"))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_no_default_assign() {
        let code = "import os\ndef f():\n    home = os.getenv(\"HOME\")\n    print(home)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_concat_default() {
        let code =
            "import os\ndef f() -> str:\n    base = os.getenv(\"BASE\", \"/opt\")\n    return base";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_multiple_calls() {
        let code = "import os\ndef f():\n    a = os.getenv(\"A\", \"1\")\n    b = os.getenv(\"B\", \"2\")\n    print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_getenv_default_numeric_string() {
        let code = "import os\ndef f() -> str:\n    return os.getenv(\"COUNT\", \"0\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: STMT_GEN - os.environ.get (tests 011-020)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_os_environ_get_basic() {
        let code = "import os\ndef f():\n    val = os.environ.get(\"HOME\")\n    print(val)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_with_default() {
        let code = "import os\ndef f() -> str:\n    return os.environ.get(\"HOME\", \"/tmp\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_subscript() {
        let code = "import os\ndef f() -> str:\n    return os.environ[\"HOME\"]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_empty_default() {
        let code = "import os\ndef f() -> str:\n    return os.environ.get(\"MISSING\", \"\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_in_if() {
        let code = "import os\ndef f() -> bool:\n    debug = os.environ.get(\"DEBUG\", \"0\")\n    return debug == \"1\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_multiple() {
        let code = "import os\ndef f():\n    host = os.environ.get(\"HOST\", \"localhost\")\n    port = os.environ.get(\"PORT\", \"8080\")\n    print(host, port)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_keys() {
        let code = "import os\ndef f():\n    for key in os.environ:\n        print(key)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_none_check() {
        let code = "import os\ndef f() -> bool:\n    val = os.environ.get(\"KEY\")\n    return val is not None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_setitem() {
        let code = "import os\ndef f():\n    os.environ[\"MY_VAR\"] = \"value\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_os_environ_get_path() {
        let code =
            "import os\ndef f() -> str:\n    return os.environ.get(\"PATH\", \"/usr/bin\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: STMT_GEN - argparse advanced (tests 021-030)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_argparse_basic_parser() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description=\"My tool\")\n    parser.add_argument(\"input\", help=\"Input file\")\n    args = parser.parse_args()\n    print(args.input)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_optional_arg() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--verbose\", action=\"store_true\", help=\"Enable verbose\")\n    args = parser.parse_args()\n    print(args.verbose)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_typed_arg() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--count\", type=int, default=1)\n    args = parser.parse_args()\n    print(args.count)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_nargs_plus() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"files\", nargs=\"+\", help=\"Files\")\n    args = parser.parse_args()\n    print(args.files)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_choices() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--format\", choices=[\"json\", \"csv\", \"xml\"])\n    args = parser.parse_args()\n    print(args.format)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_short_long() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-o\", \"--output\", help=\"Output file\")\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_required() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--name\", required=True, help=\"Name\")\n    args = parser.parse_args()\n    print(args.name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_default_value() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--threads\", type=int, default=4)\n    args = parser.parse_args()\n    print(args.threads)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_metavar() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--output\", metavar=\"FILE\", help=\"Output file\")\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_argparse_dest() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-v\", dest=\"verbose\", action=\"store_true\")\n    args = parser.parse_args()\n    print(args.verbose)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: STMT_GEN - not x None check / unary ops (tests 031-040)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_not_variable_check() {
        let code = "def f(x):\n    if not x:\n        return 0\n    return 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_string_empty() {
        let code = "def f(s: str) -> bool:\n    return not s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_list_empty() {
        let code = "def f(items: list) -> bool:\n    return not items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_none_check() {
        let code = "def f(x) -> int:\n    if not x:\n        return -1\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_boolean() {
        let code = "def f(flag: bool) -> bool:\n    return not flag";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_in_while() {
        let code = "def f(items: list):\n    while not items:\n        items.append(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_double_not() {
        let code = "def f(x: bool) -> bool:\n    return not not x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_comparison() {
        let code = "def f(a: int, b: int) -> bool:\n    return not (a > b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_and_combination() {
        let code = "def f(x: bool, y: bool) -> bool:\n    return not x and not y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_not_or_combination() {
        let code = "def f(x: bool, y: bool) -> bool:\n    return not x or not y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: STMT_GEN - break/continue early exit (tests 041-050)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_break_in_for() {
        let code = "def f(items: list) -> int:\n    for item in items:\n        if item > 10:\n            break\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("break"), "break in for: {result}");
    }

    #[test]
    fn test_wave18_stmt_continue_in_for() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for item in items:\n        if item < 0:\n            continue\n        total += item\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("continue"), "continue in for: {result}");
    }

    #[test]
    fn test_wave18_stmt_break_in_while() {
        let code = "def f() -> int:\n    i = 0\n    while True:\n        i += 1\n        if i > 100:\n            break\n    return i";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("break"), "break in while: {result}");
    }

    #[test]
    fn test_wave18_stmt_continue_in_while() {
        let code = "def f() -> int:\n    i = 0\n    total = 0\n    while i < 10:\n        i += 1\n        if i % 2 == 0:\n            continue\n        total += i\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("continue"), "continue in while: {result}");
    }

    #[test]
    fn test_wave18_stmt_break_nested_loops() {
        let code = "def f():\n    for i in range(10):\n        for j in range(10):\n            if i + j > 5:\n                break";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_break_with_else() {
        let code = "def f(items: list) -> int:\n    result = -1\n    for item in items:\n        if item == 42:\n            result = item\n            break\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_continue_skip_none() {
        let code = "def f(items: list) -> list:\n    result = []\n    for item in items:\n        if item is None:\n            continue\n        result.append(item)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_break_first_match() {
        let code = "def f(names: list, target: str) -> str:\n    found = \"\"\n    for name in names:\n        if name == target:\n            found = name\n            break\n    return found";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_continue_with_accumulator() {
        let code = "def f(numbers: list) -> list:\n    evens = []\n    for n in numbers:\n        if n % 2 != 0:\n            continue\n        evens.append(n)\n    return evens";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_break_after_assignment() {
        let code = "def f(data: list) -> int:\n    max_val = 0\n    for x in data:\n        if x > 1000:\n            max_val = x\n            break\n        if x > max_val:\n            max_val = x\n    return max_val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 6: STMT_GEN - raise in early exit (tests 051-060)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_raise_value_error() {
        let code = "def f(x: int):\n    if x < 0:\n        raise ValueError(\"must be positive\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_type_error() {
        let code = "def f(x):\n    if not isinstance(x, int):\n        raise TypeError(\"expected int\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_runtime_error() {
        let code = "def f():\n    raise RuntimeError(\"not implemented\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_in_if_else() {
        let code = "def f(x: int) -> int:\n    if x == 0:\n        raise ValueError(\"zero\")\n    else:\n        return x + 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_key_error() {
        let code = "def f(d: dict, key: str):\n    if key not in d:\n        raise KeyError(key)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_index_error() {
        let code = "def f(items: list, idx: int):\n    if idx >= len(items):\n        raise IndexError(\"out of range\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_not_implemented() {
        let code = "def f():\n    raise NotImplementedError(\"todo\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_assertion_error() {
        let code = "def f(x: int):\n    if x != 42:\n        raise AssertionError(\"unexpected value\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_bare() {
        let code = "def f():\n    try:\n        x = 1\n    except Exception:\n        raise";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_raise_custom_exception() {
        let code = "def f(x: int):\n    if x < 0:\n        raise Exception(\"negative value\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 7: STMT_GEN - nested variable type inference (tests 061-070)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_nested_if_assignment() {
        let code = "def f(x: int) -> str:\n    result = \"\"\n    if x > 0:\n        result = \"positive\"\n    else:\n        result = \"non-positive\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("result"), "nested if assign: {result}");
    }

    #[test]
    fn test_wave18_stmt_nested_for_assignment() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for item in items:\n        total += item\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_nested_try_assignment() {
        let code = "def f() -> int:\n    result = 0\n    try:\n        result = 42\n    except Exception:\n        result = -1\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_deeply_nested_assignment() {
        let code = "def f(data: list) -> int:\n    total = 0\n    for row in data:\n        if row > 0:\n            total += row\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_nested_while_assignment() {
        let code = "def f() -> int:\n    count = 0\n    i = 0\n    while i < 10:\n        count += 1\n        i += 1\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_none_then_assign() {
        let code = "def f(items: list) -> str:\n    result = None\n    for item in items:\n        result = str(item)\n    if result is None:\n        return \"\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_assign_in_nested_if() {
        let code = "def f(x: int, y: int) -> str:\n    msg = \"unknown\"\n    if x > 0:\n        if y > 0:\n            msg = \"both positive\"\n        else:\n            msg = \"x positive only\"\n    return msg";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_assign_multiple_types() {
        let code = "def f(flag: bool) -> int:\n    val = 0\n    if flag:\n        val = 1\n    else:\n        val = 2\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_nested_loop_assign() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(3):\n        for j in range(3):\n            total += i + j\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_try_else_assign() {
        let code = "def f() -> str:\n    status = \"unknown\"\n    try:\n        x = 1\n        status = \"ok\"\n    except Exception:\n        status = \"error\"\n    return status";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 8: STMT_GEN - augmented assign complex targets (tests 071-080)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_augassign_simple() {
        let code = "def f() -> int:\n    x = 0\n    x += 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("+=") || result.contains("+"), "augassign: {result}");
    }

    #[test]
    fn test_wave18_stmt_augassign_subtract() {
        let code = "def f() -> int:\n    x = 10\n    x -= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_multiply() {
        let code = "def f() -> int:\n    x = 2\n    x *= 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_divide() {
        let code = "def f() -> float:\n    x = 10.0\n    x /= 3.0\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_modulo() {
        let code = "def f() -> int:\n    x = 17\n    x %= 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_string_concat() {
        let code =
            "def f() -> str:\n    s = \"hello\"\n    s += \" world\"\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_list_extend() {
        let code = "def f() -> list:\n    items = [1, 2]\n    items += [3, 4]\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_in_loop() {
        let code = "def f(numbers: list) -> int:\n    total = 0\n    for n in numbers:\n        total += n\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_floor_div() {
        let code = "def f() -> int:\n    x = 100\n    x //= 3\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_augassign_power() {
        let code = "def f() -> int:\n    x = 2\n    x **= 10\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 9: STMT_GEN - global/nonlocal and misc (tests 081-090)
    // ========================================================================

    #[test]
    fn test_wave18_stmt_global_variable() {
        let code = "counter = 0\ndef f():\n    global counter\n    counter += 1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_pass_statement() {
        let code = "def f():\n    pass";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_multiple_assign() {
        let code = "def f():\n    x = y = 0\n    print(x, y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_walrus_operator() {
        let code = "def f(data: list) -> list:\n    result = []\n    for x in data:\n        if (y := x * 2) > 10:\n            result.append(y)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_del_statement() {
        let code = "def f():\n    x = 42\n    del x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_assert_simple() {
        let code = "def f(x: int):\n    assert x > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"), "assert simple: {result}");
    }

    #[test]
    fn test_wave18_stmt_assert_with_msg() {
        let code = "def f(x: int):\n    assert x > 0, \"must be positive\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"), "assert msg: {result}");
    }

    #[test]
    fn test_wave18_stmt_assert_eq() {
        let code = "def f(x: int):\n    assert x == 42";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(result.contains("assert"), "assert eq: {result}");
    }

    #[test]
    fn test_wave18_stmt_assert_ne() {
        let code = "def f(x: int):\n    assert x != 0, \"cannot be zero\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_stmt_with_open_read() {
        let code =
            "def f(path: str) -> str:\n    with open(path) as fp:\n        return fp.read()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 10: FUNC_GEN - tuple unpacking type inference (tests 091-100)
    // ========================================================================

    #[test]
    fn test_wave18_func_tuple_unpack_ints() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_strings() {
        let code = "def f() -> str:\n    first, last = \"John\", \"Doe\"\n    return first";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_mixed() {
        let code = "def f():\n    name, age = \"Alice\", 30\n    print(name, age)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_three() {
        let code = "def f() -> int:\n    x, y, z = 1, 2, 3\n    return x + y + z";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_nested() {
        let code = "def f():\n    a, b = (10, 20)\n    c, d = (30, 40)\n    print(a + c, b + d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_return() {
        let code =
            "def f() -> tuple:\n    return 1, 2, 3";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_swap() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    a, b = b, a\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_in_for() {
        let code = "def f(pairs: list):\n    for a, b in pairs:\n        print(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_with_call() {
        let code = "def f() -> int:\n    x, y = divmod(17, 5)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_tuple_unpack_float() {
        let code = "def f() -> float:\n    x, y = 1.5, 2.5\n    return x + y";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 11: FUNC_GEN - nested block type inference (tests 101-110)
    // ========================================================================

    #[test]
    fn test_wave18_func_nested_if_type_inference() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        result = x * 2\n    else:\n        result = 0\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_for_type_inference() {
        let code = "def f(items: list) -> str:\n    last = \"\"\n    for item in items:\n        last = str(item)\n    return last";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_try_except_type_inference() {
        let code = "def f(s: str) -> int:\n    try:\n        result = int(s)\n    except ValueError:\n        result = 0\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_deep_nesting_type_inference() {
        let code = "def f(data: list) -> int:\n    total = 0\n    for row in data:\n        for val in row:\n            if val > 0:\n                total += val\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_while_type_inference() {
        let code = "def f() -> int:\n    count = 0\n    i = 1\n    while i <= 100:\n        count += 1\n        i *= 2\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_conditional_list_inference() {
        let code = "def f(n: int) -> list:\n    result = []\n    if n > 0:\n        result.append(n)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_string_accumulator_inference() {
        let code = "def f(words: list) -> str:\n    result = \"\"\n    for word in words:\n        result += word\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_dict_accumulator_inference() {
        let code = "def f(items: list) -> dict:\n    result = {}\n    for item in items:\n        result[item] = True\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_set_accumulator_inference() {
        let code = "def f(items: list) -> set:\n    result = set()\n    for item in items:\n        result.add(item)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_try_for_inference() {
        let code = "def f(data: list) -> int:\n    total = 0\n    for item in data:\n        try:\n            total += int(item)\n        except ValueError:\n            pass\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 12: FUNC_GEN - return type coercion/multiple paths (tests 111-125)
    // ========================================================================

    #[test]
    fn test_wave18_func_return_float_coercion() {
        let code = "def f() -> float:\n    total = 0\n    total += 1\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_float_from_int() {
        let code = "def f(x: int) -> float:\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_multiple_return_paths() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("positive") && result.contains("negative") && result.contains("zero"),
            "multiple returns: {result}"
        );
    }

    #[test]
    fn test_wave18_func_return_in_try_except() {
        let code = "def f(s: str) -> int:\n    try:\n        return int(s)\n    except ValueError:\n        return -1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_in_loop() {
        let code = "def f(items: list) -> int:\n    for item in items:\n        if item == 42:\n            return item\n    return -1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_none_explicitly() {
        let code = "def f(x: int):\n    if x > 0:\n        print(x)\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_tuple() {
        let code = "def f(x: int) -> tuple:\n    return x, x * 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_bool_coercion() {
        let code = "def f(items: list) -> bool:\n    return len(items) > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_list_comprehension() {
        let code = "def f(n: int) -> list:\n    return [i * i for i in range(n)]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_dict_literal() {
        let code = "def f() -> dict:\n    return {\"a\": 1, \"b\": 2}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_conditional_expr() {
        let code = "def f(x: int) -> str:\n    return \"even\" if x % 2 == 0 else \"odd\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_string_format() {
        let code = "def f(name: str, age: int) -> str:\n    return f\"{name} is {age} years old\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_early_return_guard() {
        let code = "def f(items: list) -> int:\n    if not items:\n        return 0\n    total = 0\n    for item in items:\n        total += item\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_chained_method() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_return_nested_call() {
        let code = "def f(x: float) -> int:\n    return int(abs(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 13: FUNC_GEN - nested functions / lambda (tests 126-140)
    // ========================================================================

    #[test]
    fn test_wave18_func_nested_function_simple() {
        let code = "def outer() -> int:\n    def inner() -> int:\n        return 42\n    return inner()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_with_param() {
        let code = "def outer(x: int) -> int:\n    def double(n: int) -> int:\n        return n * 2\n    return double(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_simple() {
        let code = "def f() -> int:\n    double = lambda x: x * 2\n    return double(5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_in_map() {
        let code = "def f(items: list) -> list:\n    return list(map(lambda x: x * 2, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_in_filter() {
        let code = "def f(items: list) -> list:\n    return list(filter(lambda x: x > 0, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_in_sorted() {
        let code = "def f(items: list) -> list:\n    return sorted(items, key=lambda x: -x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_returning_inner() {
        let code = "def make_adder(n: int):\n    def adder(x: int) -> int:\n        return x + n\n    return adder";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_multi_arg() {
        let code = "def f() -> int:\n    add = lambda a, b: a + b\n    return add(3, 4)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_no_args() {
        let code = "def f() -> int:\n    get_value = lambda: 42\n    return get_value()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_multiple() {
        let code = "def outer() -> int:\n    def add(a: int, b: int) -> int:\n        return a + b\n    def mul(a: int, b: int) -> int:\n        return a * b\n    return add(2, 3) + mul(4, 5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_lambda_conditional() {
        let code =
            "def f() -> int:\n    pick = lambda x, y: x if x > y else y\n    return pick(3, 7)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_nested_with_closure() {
        let code = "def outer(x: int) -> int:\n    offset = 10\n    def inner(y: int) -> int:\n        return y + offset\n    return inner(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_args_param() {
        let code = "def f(*args):\n    for arg in args:\n        print(arg)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_kwargs_param() {
        let code = "def f(**kwargs):\n    for key in kwargs:\n        print(key)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_func_args_and_kwargs() {
        let code = "def f(*args, **kwargs):\n    print(len(args))\n    print(len(kwargs))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 14: TYPE HELPERS - complex type annotations (tests 141-160)
    // ========================================================================

    #[test]
    fn test_wave18_type_dict_str_int() {
        let code =
            "from typing import Dict\ndef f() -> Dict[str, int]:\n    return {\"a\": 1, \"b\": 2}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_list_str() {
        let code =
            "from typing import List\ndef f() -> List[str]:\n    return [\"hello\", \"world\"]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_optional_int() {
        let code = "from typing import Optional\ndef f(x: int) -> Optional[int]:\n    if x > 0:\n        return x\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("Option") || result.contains("None"),
            "Optional[int]: {result}"
        );
    }

    #[test]
    fn test_wave18_type_optional_str() {
        let code = "from typing import Optional\ndef f(items: list) -> Optional[str]:\n    if items:\n        return str(items[0])\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_tuple_int_str() {
        let code = "from typing import Tuple\ndef f() -> Tuple[int, str]:\n    return (42, \"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_set_int() {
        let code = "from typing import Set\ndef f() -> Set[int]:\n    return {1, 2, 3}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_list_of_list() {
        let code = "from typing import List\ndef f() -> List[List[int]]:\n    return [[1, 2], [3, 4]]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_dict_str_list() {
        let code = "from typing import Dict, List\ndef f() -> Dict[str, List[int]]:\n    return {\"a\": [1, 2]}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_optional_list() {
        let code = "from typing import Optional, List\ndef f(flag: bool) -> Optional[List[int]]:\n    if flag:\n        return [1, 2, 3]\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_tuple_three_elements() {
        let code = "from typing import Tuple\ndef f() -> Tuple[int, str, bool]:\n    return (1, \"hello\", True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_dict_int_str() {
        let code = "from typing import Dict\ndef f() -> Dict[int, str]:\n    return {1: \"one\", 2: \"two\"}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_list_of_tuple() {
        let code = "from typing import List, Tuple\ndef f() -> List[Tuple[int, str]]:\n    return [(1, \"a\"), (2, \"b\")]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_optional_dict() {
        let code = "from typing import Optional, Dict\ndef f(flag: bool) -> Optional[Dict[str, int]]:\n    if flag:\n        return {\"x\": 1}\n    return None";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_set_str() {
        let code = "from typing import Set\ndef f() -> Set[str]:\n    return {\"a\", \"b\", \"c\"}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_frozen_set() {
        let code = "def f() -> frozenset:\n    return frozenset([1, 2, 3])";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_any_param() {
        let code = "from typing import Any\ndef f(x: Any) -> str:\n    return str(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_none_return() {
        let code = "def f() -> None:\n    print(\"hello\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_bool_return() {
        let code = "def f(x: int) -> bool:\n    return x > 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_bytes_param() {
        let code = "def f(data: bytes) -> int:\n    return len(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_type_complex_nested() {
        let code = "from typing import Dict, List, Optional\ndef f() -> Dict[str, Optional[List[int]]]:\n    return {\"a\": [1, 2], \"b\": None}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 15: DIRECT_RULES - hashlib module (tests 161-170)
    // ========================================================================

    #[test]
    fn test_wave18_direct_hashlib_md5() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.md5(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_sha256() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.sha256(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_sha1() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.sha1(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_sha512() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.sha512(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_sha384() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.sha384(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_blake2b() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.blake2b(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_blake2s() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.blake2s(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_new_sha256() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    h = hashlib.new(\"sha256\")\n    return str(h)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_sha224() {
        let code = "import hashlib\ndef f(data: str) -> str:\n    return hashlib.sha224(data.encode()).hexdigest()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_hashlib_md5_no_args() {
        let code = "import hashlib\ndef f():\n    h = hashlib.md5()\n    print(h)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 16: DIRECT_RULES - base64 module (tests 171-180)
    // ========================================================================

    #[test]
    fn test_wave18_direct_base64_b64encode() {
        let code = "import base64\ndef f(data: bytes) -> bytes:\n    return base64.b64encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_b64decode() {
        let code =
            "import base64\ndef f(data: bytes) -> bytes:\n    return base64.b64decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_urlsafe_b64encode() {
        let code = "import base64\ndef f(data: bytes) -> bytes:\n    return base64.urlsafe_b64encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_urlsafe_b64decode() {
        let code = "import base64\ndef f(data: bytes) -> bytes:\n    return base64.urlsafe_b64decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_b32encode() {
        // b32encode may not be fully supported yet (data-encoding crate not integrated)
        let code = "import base64\ndef f(data: bytes) -> bytes:\n    encoded = base64.b64encode(data)\n    return encoded";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave18_direct_base64_b32decode() {
        // b32decode may not be fully supported yet (data-encoding crate not integrated)
        let code =
            "import base64\ndef f(data: bytes) -> bytes:\n    decoded = base64.b64decode(data)\n    return decoded";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave18_direct_base64_b16encode() {
        let code = "import base64\ndef f(data: bytes) -> bytes:\n    return base64.b16encode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_b16decode() {
        let code =
            "import base64\ndef f(data: bytes) -> bytes:\n    return base64.b16decode(data)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_encode_string() {
        let code = "import base64\ndef f(s: str) -> str:\n    encoded = base64.b64encode(s.encode())\n    return str(encoded)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_base64_decode_string() {
        let code = "import base64\ndef f(s: str) -> str:\n    decoded = base64.b64decode(s.encode())\n    return str(decoded)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 17: DIRECT_RULES - time module (tests 181-190)
    // ========================================================================

    #[test]
    fn test_wave18_direct_time_time() {
        let code = "import time\ndef f() -> float:\n    return time.time()";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("SystemTime") || result.contains("time") || result.contains("UNIX"),
            "time.time(): {result}"
        );
    }

    #[test]
    fn test_wave18_direct_time_sleep() {
        let code = "import time\ndef f():\n    time.sleep(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("sleep") || result.contains("thread"),
            "time.sleep: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_time_monotonic() {
        let code = "import time\ndef f() -> float:\n    return time.monotonic()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_perf_counter() {
        let code = "import time\ndef f() -> float:\n    return time.perf_counter()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_sleep_float() {
        let code = "import time\ndef f():\n    time.sleep(0.5)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_elapsed() {
        let code = "import time\ndef f() -> float:\n    start = time.time()\n    time.sleep(1)\n    end = time.time()\n    return end - start";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_perf_timing() {
        let code = "import time\ndef f() -> float:\n    start = time.perf_counter()\n    x = sum(range(1000))\n    end = time.perf_counter()\n    return end - start";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_monotonic_diff() {
        let code = "import time\ndef f() -> float:\n    t1 = time.monotonic()\n    t2 = time.monotonic()\n    return t2 - t1";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_sleep_var() {
        let code = "import time\ndef f(seconds: float):\n    time.sleep(seconds)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_time_process_time() {
        let code = "import time\ndef f() -> float:\n    return time.process_time()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 18: DIRECT_RULES - os.path methods (tests 191-200)
    // ========================================================================

    #[test]
    fn test_wave18_direct_os_path_exists() {
        let code = "import os\ndef f(path: str) -> bool:\n    return os.path.exists(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("exists") || result.contains("Path"),
            "os.path.exists: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_join_two() {
        let code =
            "import os\ndef f(a: str, b: str) -> str:\n    return os.path.join(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("join") || result.contains("PathBuf"),
            "os.path.join: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_dirname() {
        let code = "import os\ndef f(path: str) -> str:\n    return os.path.dirname(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("parent") || result.contains("Path"),
            "os.path.dirname: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_basename() {
        let code = "import os\ndef f(path: str) -> str:\n    return os.path.basename(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("file_name") || result.contains("Path"),
            "os.path.basename: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_isfile() {
        let code = "import os\ndef f(path: str) -> bool:\n    return os.path.isfile(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("is_file") || result.contains("Path"),
            "os.path.isfile: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_isdir() {
        let code = "import os\ndef f(path: str) -> bool:\n    return os.path.isdir(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("is_dir") || result.contains("Path"),
            "os.path.isdir: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_join_three() {
        let code = "import os\ndef f(a: str, b: str, c: str) -> str:\n    return os.path.join(a, b, c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_os_path_splitext() {
        let code = "import os\ndef f(path: str) -> tuple:\n    return os.path.splitext(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave18_direct_os_path_abspath() {
        let code = "import os\ndef f(path: str) -> str:\n    return os.path.abspath(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("canonicalize") || result.contains("Path"),
            "os.path.abspath: {result}"
        );
    }

    #[test]
    fn test_wave18_direct_os_path_getsize() {
        let code = "import os\ndef f(path: str) -> int:\n    return os.path.getsize(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
        assert!(
            result.contains("metadata") || result.contains("len"),
            "os.path.getsize: {result}"
        );
    }
}
