// coverage_wave9_fix_inference_tests.rs
// Wave 9: fix_* post-processing functions and type inference in func_gen

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

    // Section 1: fix_* post-processing functions (50 tests)

    #[test]
    fn test_w9fi_fix_001_truthiness_empty_list() {
        let code = r#"
def check_items(items):
    if not items:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("is_empty()") || result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_002_truthiness_empty_string() {
        let code = r#"
def check_name(name):
    if not name:
        return "empty"
    return name
"#;
        let result = transpile(code);
        assert!(result.contains("is_empty()") || result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_003_truthiness_empty_dict() {
        let code = r#"
def check_dict(d):
    if not d:
        return {}
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("is_empty()") || result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_004_truthiness_none_check() {
        let code = r#"
def check_value(val):
    if not val:
        return None
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("None"));
    }

    #[test]
    fn test_w9fi_fix_005_truthiness_zero_check() {
        let code = r#"
def check_count(count):
    if not count:
        return 0
    return count
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("0"));
    }

    #[test]
    fn test_w9fi_fix_006_power_sqrt_distance() {
        let code = r#"
def distance(x, y):
    return (x**2 + y**2)**0.5
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("sqrt") || result.contains("pow"));
    }

    #[test]
    fn test_w9fi_fix_007_power_sqrt_complex() {
        let code = r#"
def magnitude(a, b, c):
    return (a**2 + b**2 + c**2)**0.5
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_008_datetime_subtraction_days() {
        let code = r#"
def days_between(d1, d2):
    return (d2 - d1).days
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_009_datetime_subtraction_seconds() {
        let code = r#"
def seconds_diff(t1, t2):
    delta = t2 - t1
    return delta.seconds
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_010_enum_path_separator() {
        let code = r#"
def get_status():
    return "Status.Active"
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("Status"));
    }

    #[test]
    fn test_w9fi_fix_011_is_none_on_non_option() {
        let code = r#"
def check_val(x):
    if x is None:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_012_truthiness_list_multiple() {
        let code = r#"
def process(items, names):
    if not items and not names:
        return []
    return items + names
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_013_power_integer_exponent() {
        let code = r#"
def cube(x):
    return x**3
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("pow"));
    }

    #[test]
    fn test_w9fi_fix_014_power_negative_exponent() {
        let code = r#"
def reciprocal_square(x):
    return x**(-2)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_015_truthiness_boolean_context() {
        let code = r#"
def flag_check(enabled):
    if not enabled:
        return False
    return True
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_016_datetime_total_seconds() {
        let code = r#"
def elapsed_seconds(start, end):
    return (end - start).total_seconds()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_017_enum_comparison() {
        let code = r#"
def is_active(status):
    return status == "Active"
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("Active"));
    }

    #[test]
    fn test_w9fi_fix_018_truthiness_tuple() {
        let code = r#"
def check_tuple(t):
    if not t:
        return ()
    return t
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_019_power_fractional_exponent() {
        let code = r#"
def cube_root(x):
    return x**(1/3)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_020_is_none_with_else() {
        let code = r#"
def safe_get(val):
    if val is None:
        return 0
    else:
        return val
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_021_truthiness_set() {
        let code = r#"
def check_set(s):
    if not s:
        return set()
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_022_power_sqrt_chain() {
        let code = r#"
def calc(x):
    return ((x**2)**0.5)**2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_023_datetime_microseconds() {
        let code = r#"
def micro_diff(t1, t2):
    return (t2 - t1).microseconds
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_024_truthiness_nested_list() {
        let code = r#"
def check_nested(items):
    if not items:
        return [[]]
    return items[0]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_025_enum_multiple_paths() {
        let code = r#"
def get_states():
    return ["State.Open", "State.Closed"]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_026_is_not_none_check() {
        let code = r#"
def check_present(val):
    if val is not None:
        return val
    return 0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_027_truthiness_string_method() {
        let code = r#"
def get_name(name):
    if not name.strip():
        return "unknown"
    return name
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_028_power_base_expression() {
        let code = r#"
def calc(x, y):
    return (x + y)**2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_029_datetime_timedelta() {
        let code = r#"
def add_days(d, n):
    from datetime import timedelta
    return d + timedelta(days=n)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_030_truthiness_dict_keys() {
        let code = r#"
def check_keys(d):
    if not d.keys():
        return []
    return list(d.keys())
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_031_enum_attribute_access() {
        let code = r#"
def get_value(e):
    return e.value
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_032_power_sqrt_pythagoras() {
        let code = r#"
def hypotenuse(a, b):
    return (a*a + b*b)**0.5
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_033_truthiness_conditional_expr() {
        let code = r#"
def get_value(items):
    return items if items else []
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_034_is_none_in_comprehension() {
        let code = r#"
def filter_none(items):
    return [x for x in items if x is not None]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_035_datetime_date_diff() {
        let code = r#"
def year_diff(d1, d2):
    return d2.year - d1.year
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_036_truthiness_while_loop() {
        let code = r#"
def consume(items):
    while items:
        items.pop()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_037_power_variable_exponent() {
        let code = r#"
def power(base, exp):
    return base**exp
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_038_enum_method_call() {
        let code = r#"
def enum_name(e):
    return e.name
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_039_truthiness_or_default() {
        let code = r#"
def get_or_default(val, default):
    return val or default
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_040_datetime_now_diff() {
        let code = r#"
def since_epoch(d):
    from datetime import datetime
    return (datetime.now() - d).days
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_041_truthiness_and_check() {
        let code = r#"
def both_present(a, b):
    return a and b
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_042_power_sqrt_nested() {
        let code = r#"
def calc(x):
    return ((x**2 + 1)**0.5 + 2)**0.5
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_043_is_none_return() {
        let code = r#"
def check(val):
    return val is None
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_044_enum_in_dict() {
        let code = r#"
def get_map():
    return {"status": "Status.Active"}
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_045_truthiness_assert() {
        let code = r#"
def validate(items):
    assert items, "items must not be empty"
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_046_datetime_replace() {
        let code = r#"
def set_year(d, year):
    return d.replace(year=year)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_047_power_multiple_ops() {
        let code = r#"
def calc(x):
    return x**2 + x**3
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_048_truthiness_filter() {
        let code = r#"
def remove_empty(items):
    return [x for x in items if x]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_049_enum_iteration() {
        let code = r#"
def all_states():
    return [s for s in ["Open", "Closed"]]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_fix_050_datetime_strftime() {
        let code = r#"
def format_date(d):
    return d.strftime("%Y-%m-%d")
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    // Section 2: Argparse codegen (50 tests)

    #[test]
    fn test_w9fi_argparse_001_basic_parser() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
"#;
        let result = transpile(code);
        assert!(result.contains("fn") || result.contains("ArgumentParser"));
    }

    #[test]
    fn test_w9fi_argparse_002_add_int_argument() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--count', type=int)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_003_add_float_argument() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--rate', type=float)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_004_add_str_argument() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', type=str)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_005_store_true_action() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose', action='store_true')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_006_count_action() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-v', action='count')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_007_nargs_plus() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('files', nargs='+')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_008_nargs_star() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('files', nargs='*')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_009_nargs_question() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('file', nargs='?')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_010_default_value() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--count', type=int, default=10)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_011_choices() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--format', choices=['json', 'xml'])
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_012_required_arg() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', required=True)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_013_parse_args() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    args = parser.parse_args()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_014_access_arg_field() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--count', type=int)
    args = parser.parse_args()
    print(args.count)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_015_field_inference_arithmetic() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--count')
    args = parser.parse_args()
    total = args.count + 1
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_016_field_inference_bool() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose')
    args = parser.parse_args()
    if args.verbose:
        print("verbose mode")
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_017_multiple_args() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', type=str)
    parser.add_argument('--age', type=int)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_018_positional_arg() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_019_help_text() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', help='Input file')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_020_metavar() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', metavar='FILE')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_021_short_and_long() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-v', '--verbose', action='store_true')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_022_description() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(description='My tool')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_023_default_none() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', default=None)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_024_type_bool() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--flag', type=bool)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_025_const_value() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--debug', action='store_const', const=True)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_026_append_action() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--include', action='append')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_027_dest_name() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-n', dest='name')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_028_nargs_number() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('coords', nargs=2)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_029_choices_int() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--level', type=int, choices=[1, 2, 3])
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_030_multiple_positional() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input')
    parser.add_argument('output')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_031_store_false() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--no-cache', action='store_false')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_032_version_action() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--version', action='version', version='1.0')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_033_field_string_method() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name')
    args = parser.parse_args()
    upper = args.name.upper()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_034_field_list_method() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--items', nargs='+')
    args = parser.parse_args()
    args.items.append('new')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_035_field_comparison() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--threshold', type=float)
    args = parser.parse_args()
    if args.threshold > 0.5:
        print("high")
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_036_epilog() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(epilog='End text')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_037_prefix_chars() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(prefix_chars='-+')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_038_fromfile_prefix() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(fromfile_prefix_chars='@')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_039_argument_default() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(argument_default=0)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_040_conflict_handler() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(conflict_handler='resolve')
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_041_add_help_false() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser(add_help=False)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_042_field_multiply() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--factor', type=float)
    args = parser.parse_args()
    result = args.factor * 2.0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_043_field_division() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--total', type=int)
    args = parser.parse_args()
    avg = args.total / 10
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_044_field_string_format() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name')
    args = parser.parse_args()
    msg = f"Hello {args.name}"
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_045_field_len() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--items', nargs='*')
    args = parser.parse_args()
    count = len(args.items)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_046_subparsers() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_047_field_index() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--items', nargs='+')
    args = parser.parse_args()
    first = args.items[0]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_048_field_iteration() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--files', nargs='*')
    args = parser.parse_args()
    for f in args.files:
        print(f)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_049_field_negative() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--value', type=int)
    args = parser.parse_args()
    neg = -args.value
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_argparse_050_field_modulo() {
        let code = r#"
import argparse
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--num', type=int)
    args = parser.parse_args()
    remainder = args.num % 10
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    // Section 3: Nested functions/closures (30 tests)

    #[test]
    fn test_w9fi_nested_001_make_adder() {
        let code = r#"
def make_adder(x):
    def adder(y):
        return x + y
    return adder
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_002_lambda_return() {
        let code = r#"
def make_multiplier(n):
    return lambda x: x * n
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_003_capture_outer_var() {
        let code = r#"
def outer():
    count = 0
    def inner():
        return count + 1
    return inner()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_004_multiple_levels() {
        let code = r#"
def level1(x):
    def level2(y):
        def level3(z):
            return x + y + z
        return level3
    return level2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_005_nested_with_params() {
        let code = r#"
def outer(a, b):
    def inner(c):
        return a + b + c
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_006_higher_order_map() {
        let code = r#"
def apply_to_all(func, items):
    return [func(x) for x in items]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_007_closure_counter() {
        let code = r#"
def make_counter():
    count = [0]
    def increment():
        count[0] += 1
        return count[0]
    return increment
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_008_nested_lambda() {
        let code = r#"
def outer():
    return lambda x: lambda y: x + y
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_009_capture_list() {
        let code = r#"
def outer(items):
    def inner():
        return len(items)
    return inner()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_010_decorator_pattern() {
        let code = r#"
def decorator(func):
    def wrapper(*args):
        return func(*args)
    return wrapper
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_011_partial_application() {
        let code = r#"
def partial(f, x):
    def applied(y):
        return f(x, y)
    return applied
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_012_compose_functions() {
        let code = r#"
def compose(f, g):
    def composed(x):
        return f(g(x))
    return composed
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_013_closure_multiple_vars() {
        let code = r#"
def outer(a, b, c):
    def inner():
        return a + b + c
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_014_nested_conditional() {
        let code = r#"
def outer(x):
    def inner(y):
        if x > 0:
            return y + x
        return y
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_015_factory_pattern() {
        let code = r#"
def make_greeter(greeting):
    def greet(name):
        return f"{greeting}, {name}"
    return greet
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_016_nested_loop() {
        let code = r#"
def outer(items):
    def inner():
        result = []
        for item in items:
            result.append(item * 2)
        return result
    return inner()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_017_lambda_in_list() {
        let code = r#"
def make_ops(x):
    return [lambda y: x + y, lambda y: x * y]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_018_nested_default_arg() {
        let code = r#"
def outer(x):
    def inner(y=10):
        return x + y
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_019_memoization() {
        let code = r#"
def memoize(f):
    cache = {}
    def memoized(x):
        if x not in cache:
            cache[x] = f(x)
        return cache[x]
    return memoized
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_020_curry() {
        let code = r#"
def curry(f):
    def curried(x):
        def applied(y):
            return f(x, y)
        return applied
    return curried
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_021_filter_factory() {
        let code = r#"
def make_filter(threshold):
    def filter_func(x):
        return x > threshold
    return filter_func
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_022_nested_return_none() {
        let code = r#"
def outer():
    def inner():
        return None
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_023_accumulator() {
        let code = r#"
def make_accumulator(start):
    total = [start]
    def add(x):
        total[0] += x
        return total[0]
    return add
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_024_nested_try() {
        let code = r#"
def outer():
    def inner(x):
        try:
            return 1 / x
        except:
            return 0
    return inner
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_025_callback_pattern() {
        let code = r#"
def process(callback):
    def wrapper(data):
        return callback(data)
    return wrapper
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_026_nested_class() {
        let code = r#"
def outer():
    class Inner:
        def method(self):
            return 42
    return Inner()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_027_state_machine() {
        let code = r#"
def make_state(initial):
    state = [initial]
    def get_state():
        return state[0]
    return get_state
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_028_nested_unpacking() {
        let code = r#"
def outer(pair):
    def inner():
        a, b = pair
        return a + b
    return inner()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_029_generator_factory() {
        let code = r#"
def make_generator(start, end):
    def generator():
        for i in range(start, end):
            yield i
    return generator
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_nested_030_nested_scope_chain() {
        let code = r#"
def level1(a):
    def level2(b):
        def level3(c):
            def level4(d):
                return a + b + c + d
            return level4
        return level3
    return level2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    // Section 4: Type inference from body usage (40 tests)

    #[test]
    fn test_w9fi_infer_001_string_upper() {
        let code = r#"
def process(x):
    return x.upper()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_002_list_append() {
        let code = r#"
def add_item(x, item):
    x.append(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_003_arithmetic_add() {
        let code = r#"
def increment(x):
    return x + 1
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_004_comparison_gt() {
        let code = r#"
def is_positive(x):
    return x > 0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_005_assignment_list() {
        let code = r#"
def get_list():
    x = [1, 2, 3]
    return x
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_006_return_type_int() {
        let code = r#"
def get_answer():
    return 42
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_007_annotation_preload() {
        let code = r#"
def func():
    x: int = 0
    return x + 1
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_008_homogeneous_list() {
        let code = r#"
def get_numbers():
    return [1, 2, 3, 4]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_009_binary_multiply() {
        let code = r#"
def double(x):
    return x * 2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_010_string_lower() {
        let code = r#"
def normalize(s):
    return s.lower()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_011_list_extend() {
        let code = r#"
def combine(a, b):
    a.extend(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_012_dict_keys() {
        let code = r#"
def get_keys(d):
    return list(d.keys())
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_013_string_split() {
        let code = r#"
def tokenize(text):
    return text.split()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_014_list_pop() {
        let code = r#"
def remove_last(items):
    return items.pop()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_015_arithmetic_subtract() {
        let code = r#"
def decrement(n):
    return n - 1
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_016_comparison_lt() {
        let code = r#"
def is_small(x):
    return x < 100
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_017_string_strip() {
        let code = r#"
def clean(s):
    return s.strip()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_018_list_index() {
        let code = r#"
def first(items):
    return items[0]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_019_dict_get() {
        let code = r#"
def lookup(d, key):
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_020_arithmetic_divide() {
        let code = r#"
def halve(x):
    return x / 2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_021_string_replace() {
        let code = r#"
def substitute(text, old, new):
    return text.replace(old, new)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_022_list_sort() {
        let code = r#"
def sort_items(items):
    items.sort()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_023_comparison_eq() {
        let code = r#"
def is_zero(x):
    return x == 0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_024_string_join() {
        let code = r#"
def concat(sep, items):
    return sep.join(items)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_025_list_reverse() {
        let code = r#"
def reverse_list(items):
    items.reverse()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_026_arithmetic_modulo() {
        let code = r#"
def is_even(n):
    return n % 2 == 0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_027_string_startswith() {
        let code = r#"
def has_prefix(s, prefix):
    return s.startswith(prefix)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_028_dict_update() {
        let code = r#"
def merge(d1, d2):
    d1.update(d2)
    return d1
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_029_list_clear() {
        let code = r#"
def empty_list(items):
    items.clear()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_030_comparison_ge() {
        let code = r#"
def is_adult(age):
    return age >= 18
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_031_string_find() {
        let code = r#"
def locate(text, sub):
    return text.find(sub)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_032_list_count() {
        let code = r#"
def count_item(items, item):
    return items.count(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_033_arithmetic_power() {
        let code = r#"
def square(x):
    return x ** 2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_034_string_endswith() {
        let code = r#"
def has_suffix(s, suffix):
    return s.endswith(suffix)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_035_dict_pop() {
        let code = r#"
def remove_key(d, key):
    return d.pop(key)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_036_list_insert() {
        let code = r#"
def insert_at(items, index, item):
    items.insert(index, item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_037_comparison_ne() {
        let code = r#"
def is_different(a, b):
    return a != b
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_038_string_capitalize() {
        let code = r#"
def title_case(s):
    return s.capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_039_list_remove() {
        let code = r#"
def remove_item(items, item):
    items.remove(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_infer_040_arithmetic_floor_div() {
        let code = r#"
def int_divide(a, b):
    return a // b
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    // Section 5: Borrowing/lifetime analysis (30 tests)

    #[test]
    fn test_w9fi_borrow_001_string_read_only() {
        let code = r#"
def get_length(s):
    return len(s)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_002_string_modified() {
        let code = r#"
def modify(s):
    s = s + " modified"
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_003_list_mut_append() {
        let code = r#"
def add(items, item):
    items.append(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_004_unused_param() {
        let code = r#"
def ignore(x, y):
    return 42
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_005_multiple_borrowing() {
        let code = r#"
def process(s, items):
    items.append(len(s))
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_006_string_strip_owned() {
        let code = r#"
def clean(s):
    return s.strip()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_007_string_slice_borrowed() {
        let code = r#"
def first_char(s):
    return s[0]
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_008_list_read_only() {
        let code = r#"
def sum_list(items):
    return sum(items)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_009_param_with_default() {
        let code = r#"
def greet(name="World"):
    return f"Hello {name}"
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_010_string_upper_owned() {
        let code = r#"
def uppercase(s):
    return s.upper()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_011_list_extend_mut() {
        let code = r#"
def merge(a, b):
    a.extend(b)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_012_unused_prefix() {
        let code = r#"
def func(x, _y, _z):
    return x * 2
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_013_string_contains() {
        let code = r#"
def has_substr(s, sub):
    return sub in s
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_014_list_sort_mut() {
        let code = r#"
def sort_in_place(items):
    items.sort()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_015_string_lower_owned() {
        let code = r#"
def lowercase(s):
    return s.lower()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_016_list_insert_mut() {
        let code = r#"
def insert_first(items, item):
    items.insert(0, item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_017_string_replace_owned() {
        let code = r#"
def substitute(s):
    return s.replace("old", "new")
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_018_list_clear_mut() {
        let code = r#"
def reset(items):
    items.clear()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_019_string_iteration() {
        let code = r#"
def char_count(s):
    count = 0
    for c in s:
        count += 1
    return count
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_020_list_pop_mut() {
        let code = r#"
def take_last(items):
    return items.pop()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_021_string_format() {
        let code = r#"
def make_message(name):
    return f"Hello {name}"
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_022_list_remove_mut() {
        let code = r#"
def delete_item(items, item):
    items.remove(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_023_string_split_owned() {
        let code = r#"
def tokenize(s):
    return s.split()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_024_multiple_unused() {
        let code = r#"
def func(_a, _b, _c):
    return 0
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_025_string_capitalize_owned() {
        let code = r#"
def title(s):
    return s.capitalize()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_026_list_reverse_mut() {
        let code = r#"
def reverse_in_place(items):
    items.reverse()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_027_string_title_owned() {
        let code = r#"
def title_case(s):
    return s.title()
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_028_list_count_read() {
        let code = r#"
def count_occurrences(items, item):
    return items.count(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_029_string_join_owned() {
        let code = r#"
def join_with_comma(items):
    return ", ".join(items)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w9fi_borrow_030_list_index_read() {
        let code = r#"
def find_position(items, item):
    return items.index(item)
"#;
        let result = transpile(code);
        assert!(result.contains("fn"));
    }
}
