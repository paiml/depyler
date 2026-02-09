//! Wave 19 coverage tests: argparse subcommands, generic calls, dict/int class methods,
//! constructor patterns, assignment patterns, and variable type tracking
//!
//! Targets uncovered code paths in:
//! - argparse_transform.rs: subcommand analysis, field access, binary/unary on args
//! - call_generic.rs: os.path.*, math.*, builtin conversions, sorted/reversed/zip/enumerate
//! - collection_constructors.rs: dict.fromkeys, Counter, defaultdict, OrderedDict
//! - stmt_gen.rs codegen_assign_stmt: augmented assign on containers, tuple/star unpack,
//!   walrus operator, type-annotated assignments, method chains
//! - context.rs: variable type tracking for Option, callable, iterator types
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
    #![allow(unused_variables)]
    use super::*;

    // ========================================================================
    // SECTION 1: ARGPARSE SUBCOMMAND ANALYSIS (tests 001-040)
    // ========================================================================

    #[test]
    fn test_wave19_001_argparse_subparsers_basic() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest=\"command\")\n    build_parser = subparsers.add_parser(\"build\")\n    args = parser.parse_args()\n    print(args.command)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_002_argparse_subcommand_with_arg() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest=\"command\")\n    build_parser = subparsers.add_parser(\"build\")\n    build_parser.add_argument(\"--target\", type=str)\n    args = parser.parse_args()\n    if args.command == \"build\":\n        target = args.target";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_003_argparse_multiple_subcommands() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description=\"CLI tool\")\n    subparsers = parser.add_subparsers(dest=\"command\")\n    build_parser = subparsers.add_parser(\"build\")\n    build_parser.add_argument(\"--target\", type=str)\n    test_parser = subparsers.add_parser(\"test\")\n    test_parser.add_argument(\"--verbose\", action=\"store_true\")\n    args = parser.parse_args()\n    print(args.command)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_004_argparse_subcommand_field_access() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest=\"action\")\n    run_parser = subparsers.add_parser(\"run\")\n    run_parser.add_argument(\"--file\", type=str)\n    args = parser.parse_args()\n    if args.action == \"run\":\n        fname = args.file\n        print(fname)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_005_argparse_binary_op_on_fields() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--verbose\", action=\"store_true\")\n    parser.add_argument(\"--debug\", action=\"store_true\")\n    args = parser.parse_args()\n    if args.verbose and args.debug:\n        print(\"verbose debug\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_006_argparse_unary_not_on_field() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--quiet\", action=\"store_true\")\n    args = parser.parse_args()\n    if not args.quiet:\n        print(\"not quiet\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_007_argparse_call_with_field() {
        let code = "import argparse\ndef process(action: str) -> str:\n    return action\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--action\", type=str, default=\"run\")\n    args = parser.parse_args()\n    result = process(args.action)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_008_argparse_method_call_with_field() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--name\", type=str, default=\"world\")\n    args = parser.parse_args()\n    greeting = \"hello {}\".format(args.name)\n    print(greeting)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_009_argparse_add_argument_type_int() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--count\", type=int, default=1)\n    args = parser.parse_args()\n    for i in range(args.count):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_010_argparse_add_argument_choices() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--format\", choices=[\"json\", \"csv\", \"xml\"])\n    args = parser.parse_args()\n    print(args.format)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_011_argparse_add_argument_default_value() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--output\", type=str, default=\"stdout\")\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_012_argparse_mutually_exclusive_group() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    group = parser.add_mutually_exclusive_group()\n    group.add_argument(\"--verbose\", action=\"store_true\")\n    group.add_argument(\"--quiet\", action=\"store_true\")\n    args = parser.parse_args()\n    print(args.verbose)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_013_argparse_parse_args_middle() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--name\", type=str)\n    args = parser.parse_args()\n    name = args.name\n    if name:\n        print(name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_014_argparse_positional_and_optional() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"filename\")\n    parser.add_argument(\"--verbose\", action=\"store_true\")\n    args = parser.parse_args()\n    print(args.filename)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_015_argparse_nargs_plus() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"files\", nargs=\"+\")\n    args = parser.parse_args()\n    for f in args.files:\n        print(f)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_016_argparse_nargs_star() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"items\", nargs=\"*\")\n    args = parser.parse_args()\n    count = len(args.items)\n    print(count)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_017_argparse_nargs_question() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--config\", nargs=\"?\")\n    args = parser.parse_args()\n    print(args.config)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_018_argparse_action_store_false() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--no-color\", action=\"store_false\")\n    args = parser.parse_args()\n    print(args.no_color)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_019_argparse_action_count() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-v\", \"--verbose\", action=\"count\", default=0)\n    args = parser.parse_args()\n    if args.verbose > 1:\n        print(\"very verbose\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_020_argparse_action_append() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--include\", action=\"append\")\n    args = parser.parse_args()\n    if args.include:\n        for inc in args.include:\n            print(inc)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_021_argparse_required_flag() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--host\", type=str, required=True)\n    args = parser.parse_args()\n    print(args.host)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_022_argparse_dest_parameter() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-n\", \"--num-workers\", dest=\"workers\", type=int, default=4)\n    args = parser.parse_args()\n    print(args.workers)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_023_argparse_metavar() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--output\", metavar=\"FILE\", type=str)\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_024_argparse_help_text() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description=\"My tool\")\n    parser.add_argument(\"--verbose\", action=\"store_true\", help=\"Enable verbose mode\")\n    args = parser.parse_args()\n    print(args.verbose)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_025_argparse_subcommand_build_test() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    sub = parser.add_subparsers(dest=\"cmd\")\n    build = sub.add_parser(\"build\")\n    build.add_argument(\"--release\", action=\"store_true\")\n    test = sub.add_parser(\"test\")\n    test.add_argument(\"--filter\", type=str)\n    args = parser.parse_args()\n    if args.cmd == \"build\":\n        print(args.release)\n    elif args.cmd == \"test\":\n        print(args.filter)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_026_argparse_field_in_comparison() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--level\", type=int, default=0)\n    args = parser.parse_args()\n    if args.level > 5:\n        print(\"high\")\n    elif args.level > 0:\n        print(\"low\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_027_argparse_field_in_string_format() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--name\", type=str, default=\"user\")\n    args = parser.parse_args()\n    msg = f\"Hello {args.name}\"\n    print(msg)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_028_argparse_field_as_loop_bound() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--repeat\", type=int, default=3)\n    args = parser.parse_args()\n    for i in range(args.repeat):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_029_argparse_field_or_default() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--port\", type=int)\n    args = parser.parse_args()\n    port = args.port or 8080\n    print(port)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_030_argparse_three_subcommands() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    sub = parser.add_subparsers(dest=\"mode\")\n    sub.add_parser(\"train\")\n    sub.add_parser(\"eval\")\n    sub.add_parser(\"predict\")\n    args = parser.parse_args()\n    print(args.mode)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_031_argparse_subcommand_with_type_int() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    sub = parser.add_subparsers(dest=\"cmd\")\n    scale = sub.add_parser(\"scale\")\n    scale.add_argument(\"--replicas\", type=int, default=1)\n    args = parser.parse_args()\n    total = args.replicas * 2\n    print(total)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_032_argparse_field_concatenation() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--prefix\", type=str, default=\"output\")\n    parser.add_argument(\"--suffix\", type=str, default=\".txt\")\n    args = parser.parse_args()\n    filename = args.prefix + args.suffix\n    print(filename)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_033_argparse_field_in_list() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--tag\", type=str, default=\"latest\")\n    args = parser.parse_args()\n    tags = [args.tag, \"stable\", \"dev\"]\n    print(tags)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_034_argparse_field_in_dict() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--key\", type=str, default=\"name\")\n    parser.add_argument(\"--value\", type=str, default=\"test\")\n    args = parser.parse_args()\n    config = {args.key: args.value}\n    print(config)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_035_argparse_float_argument() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--rate\", type=float, default=0.01)\n    args = parser.parse_args()\n    adjusted = args.rate * 10.0\n    print(adjusted)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_036_argparse_two_positionals() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"source\")\n    parser.add_argument(\"destination\")\n    args = parser.parse_args()\n    print(args.source)\n    print(args.destination)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_037_argparse_short_and_long_flag() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"-o\", \"--output\", type=str, default=\"out.txt\")\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_038_argparse_field_in_return() {
        let code = "import argparse\ndef get_config() -> str:\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--config\", type=str, default=\"default.yaml\")\n    args = parser.parse_args()\n    return args.config";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_039_argparse_field_ternary() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--debug\", action=\"store_true\")\n    args = parser.parse_args()\n    level = \"DEBUG\" if args.debug else \"INFO\"\n    print(level)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_040_argparse_nargs_numeric() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument(\"--coords\", nargs=2, type=float)\n    args = parser.parse_args()\n    print(args.coords)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: GENERIC CALL PATTERNS (tests 041-080)
    // ========================================================================

    #[test]
    fn test_wave19_041_os_path_join_two_args() {
        let code = "import os\ndef make_path(base: str, name: str) -> str:\n    return os.path.join(base, name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_042_os_path_join_three_args() {
        let code = "import os\ndef make_path(base: str, sub: str, name: str) -> str:\n    return os.path.join(base, sub, name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_043_os_path_basename() {
        let code = "import os\ndef get_name(path: str) -> str:\n    return os.path.basename(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_044_os_path_dirname() {
        let code = "import os\ndef get_dir(path: str) -> str:\n    return os.path.dirname(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_045_os_path_splitext() {
        let code = "import os\ndef get_ext(filename: str) -> str:\n    name, ext = os.path.splitext(filename)\n    return ext";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_046_os_path_exists() {
        let code = "import os\ndef check(path: str) -> bool:\n    return os.path.exists(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_047_os_path_isfile() {
        let code = "import os\ndef check_file(path: str) -> bool:\n    return os.path.isfile(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_048_os_path_isdir() {
        let code = "import os\ndef check_dir(path: str) -> bool:\n    return os.path.isdir(path)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_049_isinstance_int() {
        let code = "def check(x: int) -> bool:\n    return isinstance(x, int)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_050_isinstance_str() {
        let code = "def check(x: str) -> bool:\n    return isinstance(x, str)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_051_isinstance_tuple_types() {
        let code = "def check(x) -> bool:\n    return isinstance(x, (int, float))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_052_type_call() {
        let code = "def get_type(x: int) -> str:\n    return str(type(x))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_053_id_call() {
        let code = "def get_id(x: int) -> int:\n    return id(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_054_hash_call() {
        let code = "def get_hash(x: str) -> int:\n    return hash(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_055_len_list() {
        let code = "def size(items: list) -> int:\n    return len(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_056_len_string() {
        let code = "def size(s: str) -> int:\n    return len(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_057_len_dict() {
        let code = "def size(d: dict) -> int:\n    return len(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_058_range_single() {
        let code = "def f() -> list:\n    result = []\n    for i in range(10):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_059_range_two_args() {
        let code = "def f() -> list:\n    result = []\n    for i in range(5, 15):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_060_range_three_args() {
        let code = "def f() -> list:\n    result = []\n    for i in range(0, 100, 10):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_061_enumerate_basic() {
        let code = "def f(items: list) -> list:\n    result = []\n    for i, item in enumerate(items):\n        result.append(i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_062_zip_two_lists() {
        let code = "def f(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_063_map_function() {
        let code = "def double(x: int) -> int:\n    return x * 2\ndef f(items: list) -> list:\n    return list(map(double, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_064_filter_function() {
        let code = "def is_positive(x: int) -> bool:\n    return x > 0\ndef f(items: list) -> list:\n    return list(filter(is_positive, items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_065_sorted_basic() {
        let code = "def f(items: list) -> list:\n    return sorted(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_066_sorted_reverse() {
        let code = "def f(items: list) -> list:\n    return sorted(items, reverse=True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_067_reversed_call() {
        let code = "def f(items: list) -> list:\n    return list(reversed(items))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_068_min_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return min(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_069_max_two_args() {
        let code = "def f(a: int, b: int) -> int:\n    return max(a, b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_070_sum_list() {
        let code = "def f(items: list) -> int:\n    return sum(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_071_abs_int() {
        let code = "def f(x: int) -> int:\n    return abs(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_072_round_no_digits() {
        let code = "def f(x: float) -> int:\n    return round(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_073_round_with_digits() {
        let code = "def f(x: float, n: int) -> float:\n    return round(x, n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_074_int_conversion() {
        let code = "def f(x: float) -> int:\n    return int(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_075_float_conversion() {
        let code = "def f(x: int) -> float:\n    return float(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_076_str_conversion() {
        let code = "def f(x: int) -> str:\n    return str(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_077_bool_conversion() {
        let code = "def f(x: int) -> bool:\n    return bool(x)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_078_chr_call() {
        let code = "def f(n: int) -> str:\n    return chr(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_079_ord_call() {
        let code = "def f(c: str) -> int:\n    return ord(c)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_080_input_with_prompt() {
        let code = "def f() -> str:\n    name = input(\"Enter name: \")\n    return name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: DICT.FROMKEYS AND INT.FROM_BYTES (tests 081-100)
    // ========================================================================

    #[test]
    fn test_wave19_081_dict_fromkeys_one_arg() {
        let code = "def f() -> dict:\n    keys = [\"a\", \"b\", \"c\"]\n    return dict.fromkeys(keys)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_082_dict_fromkeys_two_args() {
        let code = "def f() -> dict:\n    keys = [\"x\", \"y\", \"z\"]\n    return dict.fromkeys(keys, 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_083_dict_fromkeys_string_keys() {
        let code = "def f() -> dict:\n    return dict.fromkeys(\"abc\", True)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_084_dict_fromkeys_none_default() {
        let code = "def f() -> dict:\n    keys = [\"name\", \"age\"]\n    return dict.fromkeys(keys, None)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_085_dict_fromkeys_int_default() {
        let code = "def f() -> dict:\n    return dict.fromkeys([\"a\", \"b\"], 42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_086_dict_fromkeys_empty_list() {
        let code = "def f() -> dict:\n    return dict.fromkeys([], 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_087_dict_fromkeys_range() {
        let code = "def f() -> dict:\n    return dict.fromkeys(range(5), \"init\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_088_int_from_bytes_big() {
        let code = "def f() -> int:\n    data = b\"\\x00\\x01\"\n    return int.from_bytes(data, \"big\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_089_int_from_bytes_little() {
        let code = "def f() -> int:\n    data = b\"\\x01\\x00\"\n    return int.from_bytes(data, \"little\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_090_int_to_bytes() {
        let code = "def f(n: int) -> bytes:\n    return n.to_bytes(4, \"big\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_091_bytes_hex_method() {
        let code = "def f(data: bytes) -> str:\n    return data.hex()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_092_hex_builtin() {
        let code = "def f(n: int) -> str:\n    return hex(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_093_oct_builtin() {
        let code = "def f(n: int) -> str:\n    return oct(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_094_bin_builtin() {
        let code = "def f(n: int) -> str:\n    return bin(n)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_095_list_constructor_from_range() {
        let code = "def f() -> list:\n    return list(range(10))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_096_tuple_constructor() {
        let code = "def f(items: list) -> tuple:\n    return tuple(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_097_set_constructor() {
        let code = "def f(items: list) -> set:\n    return set(items)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_098_dict_constructor_empty() {
        let code = "def f() -> dict:\n    return dict()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_099_int_from_string_base() {
        let code = "def f() -> int:\n    return int(\"ff\", 16)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_100_int_from_string_base_10() {
        let code = "def f() -> int:\n    return int(\"42\", 10)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: CONSTRUCTOR PATTERNS (tests 101-130)
    // ========================================================================

    #[test]
    fn test_wave19_101_counter_empty() {
        let code = "from collections import Counter\ndef f():\n    c = Counter()\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_102_counter_from_list() {
        let code = "from collections import Counter\ndef f(items: list) -> dict:\n    c = Counter(items)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_103_counter_from_string() {
        let code = "from collections import Counter\ndef f(text: str) -> dict:\n    c = Counter(text)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_104_defaultdict_int() {
        let code = "from collections import defaultdict\ndef f() -> dict:\n    d = defaultdict(int)\n    d[\"key\"] += 1\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_105_defaultdict_list() {
        let code = "from collections import defaultdict\ndef f() -> dict:\n    d = defaultdict(list)\n    d[\"key\"].append(1)\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_106_ordereddict_empty() {
        let code = "from collections import OrderedDict\ndef f() -> dict:\n    d = OrderedDict()\n    d[\"a\"] = 1\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_107_namedtuple_basic() {
        let code = "from collections import namedtuple\ndef f():\n    Point = namedtuple(\"Point\", [\"x\", \"y\"])\n    p = Point(1, 2)\n    return p";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_108_custom_class_constructor() {
        let code = "class MyClass:\n    def __init__(self, value: int):\n        self.value = value\ndef f() -> int:\n    obj = MyClass(42)\n    return obj.value";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_109_custom_class_two_args() {
        let code = "class Pair:\n    def __init__(self, first: int, second: int):\n        self.first = first\n        self.second = second\ndef f() -> int:\n    p = Pair(10, 20)\n    return p.first + p.second";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_110_custom_class_default_arg() {
        let code = "class Config:\n    def __init__(self, debug: bool = False):\n        self.debug = debug\ndef f() -> bool:\n    c = Config()\n    return c.debug";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_111_custom_class_string_arg() {
        let code = "class Name:\n    def __init__(self, first: str, last: str):\n        self.first = first\n        self.last = last\ndef f() -> str:\n    n = Name(\"John\", \"Doe\")\n    return n.first";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_112_class_with_method() {
        let code = "class Counter:\n    def __init__(self):\n        self.count = 0\n    def increment(self):\n        self.count += 1\ndef f() -> int:\n    c = Counter()\n    c.increment()\n    return c.count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_113_class_with_property_access() {
        let code = "class Box:\n    def __init__(self, width: int, height: int):\n        self.width = width\n        self.height = height\ndef area(b: Box) -> int:\n    return b.width * b.height";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_114_class_constructor_in_list() {
        let code = "class Item:\n    def __init__(self, name: str):\n        self.name = name\ndef f() -> list:\n    items = [Item(\"a\"), Item(\"b\"), Item(\"c\")]\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_115_class_constructor_in_loop() {
        let code = "class Node:\n    def __init__(self, val: int):\n        self.val = val\ndef f() -> list:\n    nodes = []\n    for i in range(5):\n        nodes.append(Node(i))\n    return nodes";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_116_class_constructor_conditional() {
        let code = "class Error:\n    def __init__(self, msg: str):\n        self.msg = msg\ndef f(ok: bool) -> str:\n    if ok:\n        return \"success\"\n    e = Error(\"failed\")\n    return e.msg";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_117_class_constructor_nested() {
        let code = "class Inner:\n    def __init__(self, x: int):\n        self.x = x\nclass Outer:\n    def __init__(self, inner: Inner):\n        self.inner = inner\ndef f() -> int:\n    obj = Outer(Inner(5))\n    return obj.inner.x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_118_counter_most_common() {
        let code = "from collections import Counter\ndef f(text: str) -> list:\n    c = Counter(text)\n    return c.most_common(3)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_119_deque_constructor() {
        let code = "from collections import deque\ndef f() -> list:\n    d = deque([1, 2, 3])\n    d.append(4)\n    return list(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_120_class_no_args() {
        let code = "class Empty:\n    def __init__(self):\n        self.data = []\ndef f() -> list:\n    e = Empty()\n    return e.data";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_121_class_mixed_types() {
        let code = "class Record:\n    def __init__(self, name: str, age: int, active: bool):\n        self.name = name\n        self.age = age\n        self.active = active\ndef f() -> str:\n    r = Record(\"Alice\", 30, True)\n    return r.name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_122_frozenset_constructor() {
        let code = "def f() -> frozenset:\n    return frozenset([1, 2, 3])";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_123_frozenset_empty() {
        let code = "def f() -> frozenset:\n    return frozenset()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_124_set_from_string() {
        let code = "def f(text: str) -> set:\n    return set(text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_125_list_from_string() {
        let code = "def f(text: str) -> list:\n    return list(text)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_126_dict_from_zip() {
        let code = "def f(keys: list, vals: list) -> dict:\n    return dict(zip(keys, vals))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_127_class_with_list_field() {
        let code = "class Stack:\n    def __init__(self):\n        self.items = []\n    def push(self, item: int):\n        self.items.append(item)\n    def pop(self) -> int:\n        return self.items.pop()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_128_class_with_dict_field() {
        let code = "class Cache:\n    def __init__(self):\n        self.store = {}\n    def get(self, key: str) -> str:\n        return self.store.get(key, \"\")";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_129_class_constructor_returned() {
        let code = "class Token:\n    def __init__(self, kind: str, value: str):\n        self.kind = kind\n        self.value = value\ndef make_token(k: str, v: str) -> Token:\n    return Token(k, v)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_130_class_multiple_constructors() {
        let code = "class Point:\n    def __init__(self, x: float, y: float):\n        self.x = x\n        self.y = y\ndef f() -> float:\n    p1 = Point(0.0, 0.0)\n    p2 = Point(3.0, 4.0)\n    dx = p2.x - p1.x\n    dy = p2.y - p1.y\n    return dx + dy";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: ASSIGNMENT PATTERNS (tests 131-170)
    // ========================================================================

    #[test]
    fn test_wave19_131_augmented_assign_dict_key_plus() {
        let code = "def f() -> dict:\n    d = {\"a\": 1, \"b\": 2}\n    d[\"a\"] += 10\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_132_augmented_assign_dict_key_minus() {
        let code = "def f() -> dict:\n    d = {\"x\": 100}\n    d[\"x\"] -= 1\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_133_augmented_assign_dict_key_mul() {
        let code = "def f() -> dict:\n    d = {\"val\": 5}\n    d[\"val\"] *= 2\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_134_augmented_assign_list_index_plus() {
        let code = "def f() -> list:\n    items = [1, 2, 3]\n    items[0] += 10\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_135_augmented_assign_list_index_minus() {
        let code = "def f() -> list:\n    items = [10, 20, 30]\n    items[1] -= 5\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_136_tuple_unpack_two() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_137_tuple_unpack_three() {
        let code = "def f() -> int:\n    x, y, z = 1, 2, 3\n    return x + y + z";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_138_tuple_unpack_from_function() {
        let code = "def get_pair() -> tuple:\n    return (10, 20)\ndef f() -> int:\n    a, b = get_pair()\n    return a + b";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_139_star_unpack_first_rest() {
        let code = "def f(items: list) -> int:\n    first, *rest = items\n    return first";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave19_140_star_unpack_init_last() {
        let code = "def f(items: list) -> int:\n    *init, last = items\n    return last";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave19_141_walrus_in_if() {
        let code = "def f(items: list) -> int:\n    if (n := len(items)) > 0:\n        return n\n    return 0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_142_walrus_in_while() {
        let code = "def f(text: str) -> int:\n    count = 0\n    i = 0\n    while (ch := text[i:i+1]) != \"\":\n        count += 1\n        i += 1\n    return count";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave19_143_type_annotated_int() {
        let code = "def f() -> int:\n    x: int = 5\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_144_type_annotated_str() {
        let code = "def f() -> str:\n    name: str = \"hello\"\n    return name";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_145_type_annotated_float() {
        let code = "def f() -> float:\n    pi: float = 3.14159\n    return pi";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_146_type_annotated_bool() {
        let code = "def f() -> bool:\n    flag: bool = True\n    return flag";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_147_type_annotated_list() {
        let code = "def f() -> list:\n    items: list = [1, 2, 3]\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_148_conditional_assignment() {
        let code = "def f(x: int) -> str:\n    result = \"positive\" if x > 0 else \"non-positive\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_149_none_assignment() {
        let code = "def f() -> int:\n    result = None\n    result = 42\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_150_method_chain_strip_lower() {
        let code = "def f(text: str) -> str:\n    result = text.strip().lower()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_151_method_chain_upper_replace() {
        let code = "def f(text: str) -> str:\n    result = text.upper().replace(\"A\", \"X\")\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_152_augmented_assign_plus_int() {
        let code = "def f() -> int:\n    total = 0\n    total += 5\n    total += 10\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_153_augmented_assign_minus_int() {
        let code = "def f() -> int:\n    count = 100\n    count -= 25\n    return count";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_154_augmented_assign_mul_int() {
        let code = "def f() -> int:\n    val = 3\n    val *= 7\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_155_augmented_assign_div_float() {
        let code = "def f() -> float:\n    val = 100.0\n    val /= 3.0\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_156_augmented_assign_modulo() {
        let code = "def f() -> int:\n    val = 17\n    val %= 5\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_157_augmented_assign_string_concat() {
        let code = "def f() -> str:\n    msg = \"hello\"\n    msg += \" world\"\n    return msg";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_158_assign_from_dict_get() {
        let code = "def f(d: dict, key: str) -> str:\n    val = d.get(key, \"default\")\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_159_assign_from_list_pop() {
        let code = "def f(items: list) -> int:\n    last = items.pop()\n    return last";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_160_assign_from_string_split() {
        let code = "def f(text: str) -> list:\n    parts = text.split(\",\")\n    return parts";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_161_assign_nested_index() {
        let code = "def f() -> int:\n    matrix = [[1, 2], [3, 4]]\n    val = matrix[0][1]\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_162_assign_from_comprehension() {
        let code = "def f() -> list:\n    squares = [x * x for x in range(10)]\n    return squares";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_163_assign_dict_comprehension() {
        let code = "def f() -> dict:\n    d = {str(i): i for i in range(5)}\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_164_assign_set_comprehension() {
        let code = "def f() -> set:\n    s = {x * 2 for x in range(5)}\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_165_assign_from_ternary_call() {
        let code = "def f(items: list) -> int:\n    result = len(items) if items else 0\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_166_multiple_assign_same_line() {
        let code = "def f() -> int:\n    a = b = 0\n    a += 1\n    return a + b";
        let _result = transpile(code);
    }

    #[test]
    fn test_wave19_167_swap_assignment() {
        let code = "def f() -> int:\n    a = 1\n    b = 2\n    a, b = b, a\n    return a";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_168_assign_bitwise_or() {
        let code = "def f() -> int:\n    flags = 0\n    flags |= 1\n    flags |= 4\n    return flags";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_169_assign_bitwise_and() {
        let code = "def f() -> int:\n    mask = 0xFF\n    mask &= 0x0F\n    return mask";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_170_assign_floor_div() {
        let code = "def f() -> int:\n    val = 17\n    val //= 3\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 6: VARIABLE TYPE TRACKING (tests 171-200)
    // ========================================================================

    #[test]
    fn test_wave19_171_option_from_dict_get() {
        let code = "def f(d: dict) -> str:\n    result = d.get(\"key\")\n    if result is not None:\n        return result\n    return \"\"";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_172_counter_from_len() {
        let code = "def f(text: str) -> int:\n    count = len(text)\n    return count * 2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_173_callable_float_return() {
        let code = "def f(x: int, y: int) -> float:\n    ratio = x / y\n    return ratio";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_174_iterator_variable() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for item in items:\n        total += item\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_175_enumerate_index_value() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for idx, val in enumerate(items):\n        total += idx + val\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_176_zip_paired_iteration() {
        let code = "def f(names: list, ages: list) -> list:\n    result = []\n    for name, age in zip(names, ages):\n        result.append(name)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_177_dict_items_iteration() {
        let code = "def f(d: dict) -> list:\n    result = []\n    for key, value in d.items():\n        result.append(key)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_178_dict_keys_iteration() {
        let code = "def f(d: dict) -> list:\n    result = []\n    for key in d.keys():\n        result.append(key)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_179_dict_values_iteration() {
        let code = "def f(d: dict) -> list:\n    result = []\n    for val in d.values():\n        result.append(val)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_180_string_iteration() {
        let code = "def f(text: str) -> list:\n    chars = []\n    for ch in text:\n        chars.append(ch)\n    return chars";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_181_nested_loop_variables() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(3):\n        for j in range(3):\n            total += i * j\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_182_boolean_variable_tracking() {
        let code = "def f(items: list) -> bool:\n    found = False\n    for item in items:\n        if item > 10:\n            found = True\n            break\n    return found";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_183_accumulator_pattern() {
        let code = "def f(numbers: list) -> float:\n    total = 0.0\n    count = 0\n    for n in numbers:\n        total += n\n        count += 1\n    return total / count if count > 0 else 0.0";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_184_max_tracker_pattern() {
        let code = "def f(items: list) -> int:\n    best = 0\n    for item in items:\n        if item > best:\n            best = item\n    return best";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_185_list_builder_pattern() {
        let code = "def f(n: int) -> list:\n    result = []\n    for i in range(n):\n        result.append(i * i)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_186_dict_builder_pattern() {
        let code = "def f(keys: list) -> dict:\n    result = {}\n    for key in keys:\n        result[key] = len(key)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_187_string_builder_pattern() {
        let code = "def f(parts: list) -> str:\n    result = \"\"\n    for part in parts:\n        result += part\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_188_reassign_different_type() {
        let code = "def f() -> str:\n    val = 42\n    val_str = str(val)\n    return val_str";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_189_variable_from_min() {
        let code = "def f(a: int, b: int) -> int:\n    smallest = min(a, b)\n    return smallest";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_190_variable_from_max() {
        let code = "def f(a: int, b: int) -> int:\n    largest = max(a, b)\n    return largest";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_191_variable_from_abs() {
        let code = "def f(x: int) -> int:\n    magnitude = abs(x)\n    return magnitude";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_192_variable_from_sorted() {
        let code = "def f(items: list) -> list:\n    ordered = sorted(items)\n    return ordered";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_193_variable_from_reversed() {
        let code = "def f(items: list) -> list:\n    rev = list(reversed(items))\n    return rev";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_194_variable_from_sum() {
        let code = "def f(items: list) -> int:\n    total = sum(items)\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_195_chained_comparison() {
        let code = "def f(x: int) -> bool:\n    return 0 < x < 100";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_196_multiple_return_paths() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        result = \"positive\"\n    elif x < 0:\n        result = \"negative\"\n    else:\n        result = \"zero\"\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_197_nested_dict_access() {
        let code = "def f(data: dict) -> int:\n    inner = data[\"key\"]\n    return inner";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_198_list_slice_assign() {
        let code = "def f(items: list) -> list:\n    subset = items[1:3]\n    return subset";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_199_variable_from_string_method() {
        let code = "def f(text: str) -> list:\n    words = text.split(\" \")\n    count = len(words)\n    first = words[0]\n    return words";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_wave19_200_complex_assignment_chain() {
        let code = "def f(items: list) -> int:\n    filtered = [x for x in items if x > 0]\n    total = sum(filtered)\n    count = len(filtered)\n    avg = total / count if count > 0 else 0\n    return avg";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
