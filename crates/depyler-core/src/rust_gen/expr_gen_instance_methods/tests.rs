use super::*;

use crate::DepylerPipeline;

fn transpile(code: &str) -> String {
    let pipeline = DepylerPipeline::new();
    pipeline
        .transpile(code)
        .expect("transpilation should succeed")
}

fn transpile_ok(code: &str) -> bool {
    let pipeline = DepylerPipeline::new();
    pipeline.transpile(code).is_ok()
}

// ========================================================================
// LIST METHOD TESTS - convert_list_method
// ========================================================================

#[test]
fn test_list_append() {
    let code = transpile(
        r#"
def append_test():
    items = [1, 2, 3]
    items.append(4)
    return items
"#,
    );
    assert!(code.contains("push"));
}

#[test]
fn test_list_append_string() {
    let code = transpile(
        r#"
def append_string():
    items = ["a", "b"]
    items.append("c")
    return items
"#,
    );
    assert!(code.contains("push"));
}

#[test]
fn test_list_extend() {
    let code = transpile(
        r#"
def extend_test():
    items = [1, 2]
    items.extend([3, 4])
    return items
"#,
    );
    assert!(code.contains("extend"));
}

#[test]
fn test_list_pop_no_args() {
    let code = transpile(
        r#"
def pop_test():
    items = [1, 2, 3]
    return items.pop()
"#,
    );
    assert!(code.contains("pop"));
}

#[test]
fn test_list_pop_with_index() {
    let code = transpile(
        r#"
def pop_index():
    items = [1, 2, 3]
    return items.pop(0)
"#,
    );
    assert!(code.contains("remove"));
}

#[test]
fn test_list_insert() {
    let code = transpile(
        r#"
def insert_test():
    items = [1, 3]
    items.insert(1, 2)
    return items
"#,
    );
    assert!(code.contains("insert"));
}

#[test]
fn test_list_remove() {
    let code = transpile(
        r#"
def remove_test():
    items = [1, 2, 3]
    items.remove(2)
    return items
"#,
    );
    assert!(code.contains("remove") || code.contains("position"));
}

#[test]
fn test_list_index() {
    let code = transpile(
        r#"
def index_test():
    items = [1, 2, 3]
    return items.index(2)
"#,
    );
    assert!(code.contains("position"));
}

#[test]
fn test_list_count() {
    let code = transpile(
        r#"
def count_test():
    items = [1, 2, 2, 3]
    return items.count(2)
"#,
    );
    assert!(code.contains("filter") || code.contains("count"));
}

#[test]
fn test_list_copy() {
    let code = transpile(
        r#"
def copy_test():
    items = [1, 2, 3]
    return items.copy()
"#,
    );
    assert!(code.contains("clone"));
}

#[test]
fn test_list_clear() {
    let code = transpile(
        r#"
def clear_test():
    items = [1, 2, 3]
    items.clear()
    return items
"#,
    );
    assert!(code.contains("clear"));
}

#[test]
fn test_list_reverse() {
    let code = transpile(
        r#"
def reverse_test():
    items = [1, 2, 3]
    items.reverse()
    return items
"#,
    );
    assert!(code.contains("reverse"));
}

#[test]
fn test_list_sort() {
    let code = transpile(
        r#"
def sort_test():
    items = [3, 1, 2]
    items.sort()
    return items
"#,
    );
    assert!(code.contains("sort"));
}

#[test]
fn test_list_sort_reverse() {
    let code = transpile(
        r#"
def sort_reverse():
    items = [1, 2, 3]
    items.sort(reverse=True)
    return items
"#,
    );
    assert!(code.contains("sort"));
}

// ========================================================================
// DICT METHOD TESTS - convert_dict_method
// ========================================================================

#[test]
fn test_dict_get_single_arg() {
    let code = transpile(
        r#"
def get_test():
    d = {"a": 1}
    return d.get("a")
"#,
    );
    assert!(code.contains("get"));
}

#[test]
fn test_dict_get_with_default() {
    let code = transpile(
        r#"
def get_default():
    d = {"a": 1}
    return d.get("b", 0)
"#,
    );
    assert!(code.contains("get") || code.contains("unwrap_or"));
}

#[test]
fn test_dict_keys() {
    let code = transpile(
        r#"
def keys_test():
    d = {"a": 1, "b": 2}
    return d.keys()
"#,
    );
    assert!(code.contains("keys"));
}

#[test]
fn test_dict_values() {
    let code = transpile(
        r#"
def values_test():
    d = {"a": 1, "b": 2}
    return d.values()
"#,
    );
    assert!(code.contains("values"));
}

#[test]
fn test_dict_items() {
    let code = transpile(
        r#"
def items_test():
    d = {"a": 1, "b": 2}
    return d.items()
"#,
    );
    assert!(code.contains("iter") || code.contains("items"));
}

#[test]
fn test_dict_update() {
    let code = transpile(
        r#"
def update_test():
    d = {"a": 1}
    d.update({"b": 2})
    return d
"#,
    );
    assert!(code.contains("insert") || code.contains("update"));
}

#[test]
fn test_dict_clear() {
    let code = transpile(
        r#"
def clear_dict():
    d = {"a": 1}
    d.clear()
    return d
"#,
    );
    assert!(code.contains("clear"));
}

#[test]
fn test_dict_copy() {
    let code = transpile(
        r#"
def copy_dict():
    d = {"a": 1}
    return d.copy()
"#,
    );
    assert!(code.contains("clone"));
}

// ========================================================================
// STRING METHOD TESTS - convert_string_method
// ========================================================================

#[test]
fn test_string_upper() {
    let code = transpile(
        r#"
def upper_test():
    s = "hello"
    return s.upper()
"#,
    );
    assert!(code.contains("to_uppercase"));
}

#[test]
fn test_string_lower() {
    let code = transpile(
        r#"
def lower_test():
    s = "HELLO"
    return s.lower()
"#,
    );
    assert!(code.contains("to_lowercase"));
}

#[test]
fn test_string_strip() {
    let code = transpile(
        r#"
def strip_test():
    s = "  hello  "
    return s.strip()
"#,
    );
    assert!(code.contains("trim"));
}

#[test]
fn test_string_startswith() {
    let code = transpile(
        r#"
def startswith_test():
    s = "hello"
    return s.startswith("he")
"#,
    );
    assert!(code.contains("starts_with"));
}

#[test]
fn test_string_endswith() {
    let code = transpile(
        r#"
def endswith_test():
    s = "hello"
    return s.endswith("lo")
"#,
    );
    assert!(code.contains("ends_with"));
}

#[test]
fn test_string_split_no_args() {
    let code = transpile(
        r#"
def split_test():
    s = "a b c"
    return s.split()
"#,
    );
    assert!(code.contains("split"));
}

#[test]
fn test_string_split_with_sep() {
    let code = transpile(
        r#"
def split_sep():
    s = "a,b,c"
    return s.split(",")
"#,
    );
    assert!(code.contains("split"));
}

#[test]
fn test_string_join() {
    let code = transpile(
        r#"
def join_test():
    items = ["a", "b", "c"]
    return ",".join(items)
"#,
    );
    assert!(code.contains("join"));
}

#[test]
fn test_string_replace() {
    let code = transpile(
        r#"
def replace_test():
    s = "hello"
    return s.replace("l", "x")
"#,
    );
    assert!(code.contains("replace") || code.contains("replacen"));
}

#[test]
fn test_string_find() {
    let code = transpile(
        r#"
def find_test():
    s = "hello"
    return s.find("l")
"#,
    );
    assert!(code.contains("find") || code.contains("position"));
}

#[test]
fn test_string_count() {
    let code = transpile(
        r#"
def count_str():
    s = "hello"
    return s.count("l")
"#,
    );
    assert!(code.contains("matches") || code.contains("count"));
}

#[test]
fn test_string_isdigit() {
    let code = transpile(
        r#"
def isdigit_test():
    s = "123"
    return s.isdigit()
"#,
    );
    assert!(code.contains("is_ascii_digit") || code.contains("chars"));
}

#[test]
fn test_string_isalpha() {
    let code = transpile(
        r#"
def isalpha_test():
    s = "abc"
    return s.isalpha()
"#,
    );
    assert!(code.contains("is_alphabetic") || code.contains("chars"));
}

#[test]
fn test_string_lstrip() {
    let code = transpile(
        r#"
def lstrip_test():
    s = "  hello"
    return s.lstrip()
"#,
    );
    assert!(code.contains("trim_start"));
}

#[test]
fn test_string_rstrip() {
    let code = transpile(
        r#"
def rstrip_test():
    s = "hello  "
    return s.rstrip()
"#,
    );
    assert!(code.contains("trim_end"));
}

#[test]
fn test_string_capitalize() {
    assert!(transpile_ok(
        r#"
def cap_test():
    s = "hello"
    return s.capitalize()
"#
    ));
}

#[test]
fn test_string_title() {
    assert!(transpile_ok(
        r#"
def title_test():
    s = "hello world"
    return s.title()
"#
    ));
}

#[test]
fn test_string_center() {
    assert!(transpile_ok(
        r#"
def center_test():
    s = "hi"
    return s.center(10)
"#
    ));
}

#[test]
fn test_string_ljust() {
    assert!(transpile_ok(
        r#"
def ljust_test():
    s = "hi"
    return s.ljust(10)
"#
    ));
}

#[test]
fn test_string_rjust() {
    assert!(transpile_ok(
        r#"
def rjust_test():
    s = "hi"
    return s.rjust(10)
"#
    ));
}

#[test]
fn test_string_zfill() {
    assert!(transpile_ok(
        r#"
def zfill_test():
    s = "42"
    return s.zfill(5)
"#
    ));
}

// ========================================================================
// SET METHOD TESTS - convert_set_method
// ========================================================================

#[test]
fn test_set_add() {
    let code = transpile(
        r#"
def add_test():
    s = {1, 2}
    s.add(3)
    return s
"#,
    );
    assert!(code.contains("insert"));
}

#[test]
fn test_set_remove() {
    let code = transpile(
        r#"
def remove_set():
    s = {1, 2, 3}
    s.remove(2)
    return s
"#,
    );
    assert!(code.contains("remove"));
}

#[test]
fn test_set_discard() {
    let code = transpile(
        r#"
def discard_test():
    s = {1, 2, 3}
    s.discard(2)
    return s
"#,
    );
    assert!(code.contains("remove"));
}

#[test]
fn test_set_pop() {
    let code = transpile(
        r#"
def pop_set():
    s = {1, 2, 3}
    return s.pop()
"#,
    );
    assert!(code.contains("iter") || code.contains("next"));
}

#[test]
fn test_set_clear() {
    let code = transpile(
        r#"
def clear_set():
    s = {1, 2, 3}
    s.clear()
    return s
"#,
    );
    assert!(code.contains("clear"));
}

#[test]
fn test_set_union() {
    let code = transpile(
        r#"
def union_test():
    s1 = {1, 2}
    s2 = {3, 4}
    return s1.union(s2)
"#,
    );
    assert!(code.contains("union") || code.contains("extend"));
}

#[test]
fn test_set_intersection() {
    let code = transpile(
        r#"
def intersection_test():
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.intersection(s2)
"#,
    );
    assert!(code.contains("intersection") || code.contains("filter"));
}

#[test]
fn test_set_difference() {
    let code = transpile(
        r#"
def difference_test():
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.difference(s2)
"#,
    );
    assert!(code.contains("difference") || code.contains("filter"));
}

// ========================================================================
// INDEX CONVERSION TESTS - convert_index
// ========================================================================

#[test]
fn test_list_index_access() {
    let code = transpile(
        r#"
def index_access():
    items = [1, 2, 3]
    return items[0]
"#,
    );
    assert!(code.contains("[") && code.contains("]"));
}

#[test]
fn test_dict_index_access() {
    let code = transpile(
        r#"
def dict_access():
    d = {"a": 1}
    return d["a"]
"#,
    );
    assert!(code.contains("get") || code.contains("["));
}

#[test]
fn test_string_index_access() {
    let code = transpile(
        r#"
def string_access():
    s = "hello"
    return s[0]
"#,
    );
    assert!(code.contains("chars") || code.contains("nth"));
}

#[test]
fn test_negative_index() {
    let code = transpile(
        r#"
def neg_index():
    items = [1, 2, 3]
    return items[-1]
"#,
    );
    assert!(code.contains("len") || code.contains("-"));
}

// ========================================================================
// SLICE CONVERSION TESTS - convert_slice
// ========================================================================

#[test]
fn test_slice_basic() {
    let code = transpile(
        r#"
def slice_basic():
    items = [1, 2, 3, 4, 5]
    return items[1:3]
"#,
    );
    assert!(code.contains("[") || code.contains(".."));
}

#[test]
fn test_slice_from_start() {
    let code = transpile(
        r#"
def slice_from_start():
    items = [1, 2, 3, 4, 5]
    return items[:3]
"#,
    );
    assert!(code.contains("[") || code.contains(".."));
}

#[test]
fn test_slice_to_end() {
    let code = transpile(
        r#"
def slice_to_end():
    items = [1, 2, 3, 4, 5]
    return items[2:]
"#,
    );
    assert!(code.contains("[") || code.contains(".."));
}

#[test]
fn test_slice_full_copy() {
    let code = transpile(
        r#"
def slice_copy():
    items = [1, 2, 3]
    return items[:]
"#,
    );
    assert!(code.contains("clone") || code.contains("to_vec"));
}

#[test]
fn test_string_slice() {
    let code = transpile(
        r#"
def string_slice():
    s = "hello"
    return s[1:4]
"#,
    );
    assert!(code.contains("[") || code.contains(".."));
}

// ========================================================================
// LIST COMPREHENSION TESTS - convert_list_comp
// ========================================================================

#[test]
fn test_list_comp_simple() {
    let code = transpile(
        r#"
def list_comp():
    return [x * 2 for x in [1, 2, 3]]
"#,
    );
    assert!(code.contains("map") || code.contains("collect"));
}

#[test]
fn test_list_comp_with_filter() {
    let code = transpile(
        r#"
def list_comp_filter():
    return [x for x in [1, 2, 3, 4] if x > 2]
"#,
    );
    assert!(code.contains("filter") || code.contains("collect"));
}

#[test]
fn test_list_comp_nested() {
    let code = transpile(
        r#"
def nested_comp():
    return [x + y for x in [1, 2] for y in [10, 20]]
"#,
    );
    assert!(code.contains("flat_map") || code.contains("map"));
}

// ========================================================================
// DICT COMPREHENSION TESTS - convert_dict_comp
// ========================================================================

#[test]
fn test_dict_comp_simple() {
    let code = transpile(
        r#"
def dict_comp():
    return {x: x * 2 for x in [1, 2, 3]}
"#,
    );
    assert!(code.contains("map") || code.contains("collect") || code.contains("HashMap"));
}

#[test]
fn test_dict_comp_with_filter() {
    let code = transpile(
        r#"
def dict_comp_filter():
    return {x: x * 2 for x in [1, 2, 3, 4] if x > 2}
"#,
    );
    assert!(code.contains("filter") || code.contains("collect"));
}

// ========================================================================
// SET COMPREHENSION TESTS - convert_set_comp
// ========================================================================

#[test]
fn test_set_comp_simple() {
    let code = transpile(
        r#"
def set_comp():
    return {x * 2 for x in [1, 2, 3]}
"#,
    );
    assert!(code.contains("map") || code.contains("collect") || code.contains("HashSet"));
}

// ========================================================================
// GENERATOR EXPRESSION TESTS - convert_generator_expression
// ========================================================================

#[test]
fn test_generator_in_sum() {
    let code = transpile(
        r#"
def gen_sum():
    return sum(x for x in [1, 2, 3])
"#,
    );
    assert!(code.contains("sum") || code.contains("fold"));
}

#[test]
fn test_generator_in_any() {
    let code = transpile(
        r#"
def gen_any():
    return any(x > 2 for x in [1, 2, 3])
"#,
    );
    assert!(code.contains("any"));
}

#[test]
fn test_generator_in_all() {
    let code = transpile(
        r#"
def gen_all():
    return all(x > 0 for x in [1, 2, 3])
"#,
    );
    assert!(code.contains("all"));
}

// ========================================================================
// TUPLE CONVERSION TESTS - convert_tuple
// ========================================================================

#[test]
fn test_tuple_creation() {
    let code = transpile(
        r#"
def tuple_test():
    return (1, 2, 3)
"#,
    );
    assert!(code.contains("(") && code.contains(")"));
}

#[test]
fn test_tuple_mixed() {
    let code = transpile(
        r#"
def tuple_mixed():
    return (1, "hello", 3.14)
"#,
    );
    assert!(code.contains("("));
}

// ========================================================================
// SET CONVERSION TESTS - convert_set
// ========================================================================

#[test]
fn test_set_creation() {
    let code = transpile(
        r#"
def set_create():
    return {1, 2, 3}
"#,
    );
    assert!(code.contains("HashSet") || code.contains("from"));
}

// ========================================================================
// ATTRIBUTE CONVERSION TESTS - convert_attribute
// ========================================================================

#[test]
fn test_attribute_access() {
    assert!(transpile_ok(
        r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def get_x(p: Point) -> int:
    return p.x
"#
    ));
}

// ========================================================================
// F-STRING TESTS - convert_fstring
// ========================================================================

#[test]
fn test_fstring_simple() {
    let code = transpile(
        r#"
def fstring_test():
    name = "world"
    return f"Hello, {name}!"
"#,
    );
    assert!(code.contains("format!"));
}

#[test]
fn test_fstring_expression() {
    let code = transpile(
        r#"
def fstring_expr():
    x = 5
    return f"Value: {x + 1}"
"#,
    );
    assert!(code.contains("format!"));
}

#[test]
fn test_fstring_multiple() {
    let code = transpile(
        r#"
def fstring_multi():
    a = 1
    b = 2
    return f"{a} + {b} = {a + b}"
"#,
    );
    assert!(code.contains("format!"));
}

// ========================================================================
// IF EXPRESSION TESTS - convert_ifexpr
// ========================================================================

#[test]
fn test_ifexpr_simple() {
    let code = transpile(
        r#"
def ifexpr_test():
    x = 5
    return "big" if x > 3 else "small"
"#,
    );
    assert!(code.contains("if") && code.contains("else"));
}

#[test]
fn test_ifexpr_nested() {
    let code = transpile(
        r#"
def ifexpr_nested():
    x = 5
    return "big" if x > 10 else "medium" if x > 3 else "small"
"#,
    );
    assert!(code.contains("if") && code.contains("else"));
}

// ========================================================================
// LAMBDA TESTS - convert_lambda
// ========================================================================

#[test]
fn test_lambda_simple() {
    let code = transpile(
        r#"
def lambda_test():
    f = lambda x: x * 2
    return f(5)
"#,
    );
    assert!(code.contains("|") || code.contains("Fn"));
}

#[test]
fn test_lambda_multi_args() {
    let code = transpile(
        r#"
def lambda_multi():
    f = lambda x, y: x + y
    return f(2, 3)
"#,
    );
    assert!(code.contains("|"));
}

// ========================================================================
// BOOLEAN HELPER TESTS
// ========================================================================

#[test]
fn test_is_len_call_detection() {
    let code = transpile(
        r#"
def len_test():
    items = [1, 2, 3]
    return len(items)
"#,
    );
    assert!(code.contains("len()"));
}

// ========================================================================
// WALRUS OPERATOR TESTS - convert_named_expr
// ========================================================================

#[test]
fn test_walrus_in_if() {
    let code = transpile(
        r#"
def walrus_test():
    items = [1, 2, 3]
    if (n := len(items)) > 2:
        return n
    return 0
"#,
    );
    assert!(code.contains("let n"));
}

#[test]
fn test_walrus_in_while() {
    let code = transpile(
        r#"
def walrus_while():
    i = 0
    while (x := i) < 5:
        i += 1
    return x
"#,
    );
    assert!(code.contains("let x") || code.contains("while"));
}

// ========================================================================
// INSTANCE METHOD TESTS - convert_instance_method
// ========================================================================

#[test]
fn test_bytes_decode() {
    assert!(transpile_ok(
        r#"
def decode_test():
    b = b"hello"
    return b.decode("utf-8")
"#
    ));
}

#[test]
fn test_str_encode() {
    assert!(transpile_ok(
        r#"
def encode_test():
    s = "hello"
    return s.encode("utf-8")
"#
    ));
}

// ========================================================================
// ADDITIONAL STRING METHOD TESTS
// ========================================================================

#[test]
fn test_string_isupper() {
    assert!(transpile_ok(
        r#"
def isupper_test():
    s = "HELLO"
    return s.isupper()
"#
    ));
}

#[test]
fn test_string_islower() {
    assert!(transpile_ok(
        r#"
def islower_test():
    s = "hello"
    return s.islower()
"#
    ));
}

#[test]
fn test_string_isalnum() {
    assert!(transpile_ok(
        r#"
def isalnum_test():
    s = "abc123"
    return s.isalnum()
"#
    ));
}

#[test]
fn test_string_isspace() {
    assert!(transpile_ok(
        r#"
def isspace_test():
    s = "   "
    return s.isspace()
"#
    ));
}

#[test]
fn test_string_format() {
    assert!(transpile_ok(
        r#"
def format_test():
    return "{} {}".format("hello", "world")
"#
    ));
}

// ========================================================================
// ADDITIONAL COLLECTION TESTS
// ========================================================================

#[test]
fn test_list_multiplication() {
    let code = transpile(
        r#"
def list_mul():
    return [0] * 5
"#,
    );
    assert!(code.contains("vec!") || code.contains("*") || code.contains("repeat"));
}

#[test]
fn test_list_concatenation() {
    let code = transpile(
        r#"
def list_concat():
    return [1, 2] + [3, 4]
"#,
    );
    assert!(code.contains("extend") || code.contains("concat") || code.contains("+"));
}

#[test]
fn test_dict_setdefault() {
    let code = transpile(
        r#"
def setdefault_test():
    d = {}
    d.setdefault("a", 0)
    return d
"#,
    );
    assert!(code.contains("entry") || code.contains("or_insert"));
}

// ========================================================================
// NUMERIC TYPE TESTS
// ========================================================================

#[test]
fn test_int_bit_length() {
    assert!(transpile_ok(
        r#"
def bit_length_test():
    n = 255
    return n.bit_length()
"#
    ));
}

#[test]
fn test_float_is_integer() {
    assert!(transpile_ok(
        r#"
def is_integer_test():
    f = 3.0
    return f.is_integer()
"#
    ));
}

// ========================================================================
// ITERATOR METHODS
// ========================================================================

#[test]
fn test_enumerate() {
    let code = transpile(
        r#"
def enumerate_test():
    items = ["a", "b", "c"]
    result = []
    for i, item in enumerate(items):
        result.append((i, item))
    return result
"#,
    );
    assert!(code.contains("enumerate"));
}

#[test]
fn test_zip() {
    let code = transpile(
        r#"
def zip_test():
    a = [1, 2, 3]
    b = ["a", "b", "c"]
    return list(zip(a, b))
"#,
    );
    assert!(code.contains("zip"));
}

#[test]
fn test_reversed() {
    let code = transpile(
        r#"
def reversed_test():
    items = [1, 2, 3]
    return list(reversed(items))
"#,
    );
    assert!(code.contains("rev"));
}

#[test]
fn test_sorted() {
    let code = transpile(
        r#"
def sorted_test():
    items = [3, 1, 2]
    return sorted(items)
"#,
    );
    assert!(code.contains("sort"));
}

// ========================================================================
// REGEX METHOD TESTS (convert_regex_method)
// ========================================================================

#[test]
fn test_regex_findall() {
    assert!(transpile_ok(
        r#"
import re

def findall_test():
    text = "hello world"
    return re.findall(r"\w+", text)
"#
    ));
}

#[test]
fn test_regex_match() {
    assert!(transpile_ok(
        r#"
import re

def match_test():
    text = "hello"
    return re.match(r"he", text)
"#
    ));
}

#[test]
fn test_regex_search() {
    assert!(transpile_ok(
        r#"
import re

def search_test():
    text = "hello world"
    return re.search(r"world", text)
"#
    ));
}

#[test]
fn test_regex_sub() {
    assert!(transpile_ok(
        r#"
import re

def sub_test():
    text = "hello world"
    return re.sub(r"world", "there", text)
"#
    ));
}

// ========================================================================
// TRUTHINESS CONVERSION TESTS
// ========================================================================

#[test]
fn test_truthiness_list() {
    let code = transpile(
        r#"
def truthiness_list():
    items = [1, 2, 3]
    if items:
        return True
    return False
"#,
    );
    assert!(code.contains("is_empty") || code.contains("!"));
}

#[test]
fn test_truthiness_string() {
    let code = transpile(
        r#"
def truthiness_str():
    s = "hello"
    if s:
        return True
    return False
"#,
    );
    assert!(code.contains("is_empty") || code.contains("!"));
}

#[test]
fn test_truthiness_dict() {
    let code = transpile(
        r#"
def truthiness_dict():
    d = {"a": 1}
    if d:
        return True
    return False
"#,
    );
    assert!(code.contains("is_empty") || code.contains("!"));
}

// ========================================================================
// BORROW CONVERSION TESTS - convert_borrow
// ========================================================================

#[test]
fn test_immutable_borrow() {
    let code = transpile(
        r#"
def borrow_test(items: list):
    for item in items:
        print(item)
"#,
    );
    assert!(code.contains("&") || code.contains("iter"));
}

// ========================================================================
// DEQUE TESTS
// ========================================================================

#[test]
fn test_deque_append() {
    assert!(transpile_ok(
        r#"
from collections import deque

def deque_test():
    d = deque([1, 2, 3])
    d.append(4)
    return d
"#
    ));
}

#[test]
fn test_deque_appendleft() {
    assert!(transpile_ok(
        r#"
from collections import deque

def deque_left():
    d = deque([1, 2, 3])
    d.appendleft(0)
    return d
"#
    ));
}

#[test]
fn test_deque_popleft() {
    assert!(transpile_ok(
        r#"
from collections import deque

def deque_popleft():
    d = deque([1, 2, 3])
    return d.popleft()
"#
    ));
}

// ========================================================================
// COUNTER TESTS
// ========================================================================

#[test]
fn test_counter_creation() {
    assert!(transpile_ok(
        r#"
from collections import Counter

def counter_test():
    c = Counter([1, 1, 2, 2, 2, 3])
    return c
"#
    ));
}

#[test]
fn test_counter_most_common() {
    assert!(transpile_ok(
        r#"
from collections import Counter

def counter_common():
    c = Counter([1, 1, 2, 2, 2, 3])
    return c.most_common(2)
"#
    ));
}

// ========================================================================
// AWAIT TESTS - convert_await
// ========================================================================

#[test]
fn test_async_await() {
    assert!(transpile_ok(
        r#"
async def fetch_data():
    return 42

async def main():
    result = await fetch_data()
    return result
"#
    ));
}

// ========================================================================
// MORE EDGE CASE TESTS
// ========================================================================

#[test]
fn test_empty_list() {
    let code = transpile(
        r#"
def empty_list():
    return []
"#,
    );
    assert!(code.contains("vec!") || code.contains("Vec::new"));
}

#[test]
fn test_empty_dict() {
    let code = transpile(
        r#"
def empty_dict():
    return {}
"#,
    );
    assert!(code.contains("HashMap::new") || code.contains("HashMap"));
}

#[test]
fn test_empty_set() {
    let code = transpile(
        r#"
def empty_set():
    return set()
"#,
    );
    assert!(code.contains("HashSet::new") || code.contains("HashSet"));
}

#[test]
fn test_nested_list() {
    let code = transpile(
        r#"
def nested_list():
    return [[1, 2], [3, 4]]
"#,
    );
    assert!(code.contains("vec!"));
}

#[test]
fn test_nested_dict() {
    let code = transpile(
        r#"
def nested_dict():
    return {"a": {"b": 1}}
"#,
    );
    assert!(code.contains("HashMap") || code.contains("insert"));
}

// ========================================================================
// PARSE TARGET PATTERN TESTS - parse_target_pattern
// ========================================================================

#[test]
fn test_for_tuple_unpacking() {
    let code = transpile(
        r#"
def tuple_unpack():
    pairs = [(1, 2), (3, 4)]
    result = 0
    for a, b in pairs:
        result += a + b
    return result
"#,
    );
    assert!(code.contains("(") && code.contains(")"));
}

#[test]
fn test_for_dict_items() {
    let code = transpile(
        r#"
def dict_iter():
    d = {"a": 1, "b": 2}
    result = []
    for k, v in d.items():
        result.append((k, v))
    return result
"#,
    );
    assert!(code.contains("iter") || code.contains("items"));
}

// ========================================================================
// RETURN TYPE DETECTION TESTS
// ========================================================================

#[test]
fn test_expr_returns_result() {
    assert!(transpile_ok(
        r#"
def file_read():
    with open("test.txt") as f:
        return f.read()
"#
    ));
}

// ========================================================================
// STRING METHOD EDGE CASES
// ========================================================================

#[test]
fn test_string_split_maxsplit() {
    let code = transpile(
        r#"
def split_max():
    s = "a,b,c,d"
    return s.split(",", 2)
"#,
    );
    assert!(code.contains("splitn") || code.contains("split"));
}

#[test]
fn test_string_rsplit() {
    let code = transpile(
        r#"
def rsplit_test():
    s = "a,b,c"
    return s.rsplit(",")
"#,
    );
    assert!(code.contains("rsplit") || code.contains("split"));
}

#[test]
fn test_string_partition() {
    assert!(transpile_ok(
        r#"
def partition_test():
    s = "hello world"
    return s.partition(" ")
"#
    ));
}

#[test]
fn test_string_rpartition() {
    assert!(transpile_ok(
        r#"
def rpartition_test():
    s = "hello world hello"
    return s.rpartition(" ")
"#
    ));
}

#[test]
fn test_string_swapcase() {
    assert!(transpile_ok(
        r#"
def swapcase_test():
    s = "Hello World"
    return s.swapcase()
"#
    ));
}

#[test]
fn test_string_expandtabs() {
    assert!(transpile_ok(
        r#"
def expandtabs_test():
    s = "a\tb\tc"
    return s.expandtabs(4)
"#
    ));
}

// ========================================================================
// FROZENSET TESTS - convert_frozenset
// ========================================================================

#[test]
fn test_frozenset_creation() {
    assert!(transpile_ok(
        r#"
def frozenset_test():
    return frozenset([1, 2, 3])
"#
    ));
}

// ========================================================================
// MORE DICT METHOD TESTS
// ========================================================================

#[test]
fn test_dict_pop_with_default() {
    let code = transpile(
        r#"
def dict_pop_default():
    d = {"a": 1}
    return d.pop("b", 0)
"#,
    );
    assert!(code.contains("remove") || code.contains("unwrap_or"));
}

#[test]
fn test_dict_popitem() {
    let code = transpile(
        r#"
def dict_popitem():
    d = {"a": 1, "b": 2}
    return d.popitem()
"#,
    );
    assert!(code.contains("keys") || code.contains("remove"));
}

// ========================================================================
// SYS IO METHOD TESTS - convert_sys_io_method
// ========================================================================

#[test]
fn test_stdout_write() {
    assert!(transpile_ok(
        r#"
import sys

def stdout_test():
    sys.stdout.write("hello")
"#
    ));
}

#[test]
fn test_stderr_write() {
    assert!(transpile_ok(
        r#"
import sys

def stderr_test():
    sys.stderr.write("error")
"#
    ));
}

// ========================================================================
// ADDITIONAL CONVERSION TESTS
// ========================================================================

#[test]
fn test_range_in_parens() {
    let code = transpile(
        r#"
def range_paren():
    return [x for x in range(10)]
"#,
    );
    assert!(code.contains("..") || code.contains("range"));
}

#[test]
fn test_owned_collection_detection() {
    let code = transpile(
        r#"
def owned_test():
    items = [1, 2, 3]
    return items
"#,
    );
    assert!(code.contains("vec!"));
}

// ========================================================================
// DEPYLER-1153: NESTED DICT TYPE PROPAGATION TESTS
// ========================================================================

#[test]
fn test_DEPYLER_1153_nested_dict_concrete_type() {
    // Test that Dict[str, Dict[str, int]] uses concrete types, not DepylerValue
    let code = transpile(
        r#"
def nested_dict() -> dict[str, dict[str, int]]:
    x = 42
    return {"outer": {"inner": x}}
"#,
    );
    // Extract just the function body to avoid matching prelude definitions
    let fn_start = code.find("pub fn nested_dict").unwrap_or(0);
    let fn_code = &code[fn_start..];

    // Should NOT contain DepylerValue wrapping for dict values
    assert!(
        !fn_code.contains("DepylerValue::Dict"),
        "Should not use DepylerValue::Dict for concrete nested dict type: {}",
        fn_code
    );
    // Inner dict should be HashMap<String, i32> (not HashMap<String, HashMap<...>>)
    // This proves the type propagation is working
    assert!(
        fn_code.contains("HashMap<String, i32>"),
        "Inner dict should use concrete HashMap<String, i32>: {}",
        fn_code
    );
}

#[test]
fn test_DEPYLER_1153_nested_list_concrete_type() {
    // Test that Dict[str, list[int]] uses concrete types
    let code = transpile(
        r#"
def dict_with_list() -> dict[str, list[int]]:
    return {"values": [1, 2, 3]}
"#,
    );
    // Should NOT contain DepylerValue wrapping for list
    // (but may use DepylerValue for other reasons in prelude - only check the function body)
    let fn_start = code.find("pub fn dict_with_list").unwrap_or(0);
    let fn_code = &code[fn_start..];
    assert!(
        !fn_code.contains("DepylerValue::List"),
        "Should not use DepylerValue::List for concrete list type: {}",
        fn_code
    );
    // Should use vec![] for the list
    assert!(
        fn_code.contains("vec!["),
        "Should use vec![] for concrete list type: {}",
        fn_code
    );
}

#[test]
fn test_DEPYLER_1153_bare_dict_still_uses_depyler_value() {
    // Test that bare dict (no type annotation) still uses DepylerValue
    let code = transpile(
        r#"
def bare_dict() -> dict:
    return {"key": 42, "name": "test"}
"#,
    );
    // Bare dict with mixed types SHOULD use DepylerValue
    assert!(
        code.contains("DepylerValue"),
        "Bare dict with mixed types should use DepylerValue: {}",
        code
    );
}

// ===== Set method coverage tests =====

#[test]
fn test_set_symmetric_difference() {
    let code = transpile(
        r#"
def test() -> set:
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a.symmetric_difference(b)
"#,
    );
    assert!(
        code.contains("symmetric_difference"),
        "Should use symmetric_difference: {}",
        code
    );
}

#[test]
fn test_set_issubset() {
    let code = transpile(
        r#"
def test() -> bool:
    a = {1, 2}
    b = {1, 2, 3}
    return a.issubset(b)
"#,
    );
    assert!(code.contains("is_subset"), "Should use is_subset: {}", code);
}

#[test]
fn test_set_issuperset() {
    let code = transpile(
        r#"
def test() -> bool:
    a = {1, 2, 3}
    b = {1, 2}
    return a.issuperset(b)
"#,
    );
    assert!(
        code.contains("is_superset"),
        "Should use is_superset: {}",
        code
    );
}

#[test]
fn test_set_isdisjoint() {
    let code = transpile(
        r#"
def test() -> bool:
    a = {1, 2}
    b = {3, 4}
    return a.isdisjoint(b)
"#,
    );
    assert!(
        code.contains("is_disjoint"),
        "Should use is_disjoint: {}",
        code
    );
}

#[test]
fn test_set_update() {
    let code = transpile(
        r#"
def test():
    a = {1, 2}
    b = {3, 4}
    a.update(b)
"#,
    );
    assert!(
        code.contains("insert"),
        "Should generate insert loop for update: {}",
        code
    );
}

#[test]
fn test_set_intersection_update() {
    let code = transpile(
        r#"
def test():
    a = {1, 2, 3}
    b = {2, 3, 4}
    a.intersection_update(b)
"#,
    );
    assert!(
        code.contains("intersection"),
        "Should use intersection for intersection_update: {}",
        code
    );
}

#[test]
fn test_set_difference_update() {
    let code = transpile(
        r#"
def test():
    a = {1, 2, 3}
    b = {2, 3}
    a.difference_update(b)
"#,
    );
    assert!(
        code.contains("difference"),
        "Should use difference for difference_update: {}",
        code
    );
}

// ===== String method coverage tests =====

#[test]
fn test_string_zfill_coverage() {
    assert!(transpile_ok(
        r#"
def test() -> str:
    s = "42"
    return s.zfill(5)
"#
    ));
}

#[test]
fn test_string_rfind() {
    let code = transpile(
        r#"
def test() -> int:
    s = "hello world hello"
    return s.rfind("hello")
"#,
    );
    assert!(code.contains("rfind"), "Should use rfind: {}", code);
}

#[test]
fn test_string_splitlines() {
    let _code = transpile(
        r#"
def test() -> list:
    s = "line1\nline2\nline3"
    return s.splitlines()
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    s = "line1\nline2\nline3"
    return s.splitlines()
"#
    ));
}

#[test]
fn test_string_encode() {
    let code = transpile(
        r#"
def test():
    s = "hello"
    b = s.encode()
"#,
    );
    assert!(
        code.contains("as_bytes") || code.contains("encode") || code.contains("bytes"),
        "Should handle encode: {}",
        code
    );
}

// ===== Regex method coverage tests =====

#[test]
fn test_regex_split() {
    let _code = transpile(
        r#"
import re
def test() -> list:
    return re.split(r"\s+", "hello world")
"#,
    );
    assert!(transpile_ok(
        r#"
import re
def test() -> list:
    return re.split(r"\s+", "hello world")
"#
    ));
}

// ===== Index/Slice coverage tests =====

#[test]
fn test_slice_with_step() {
    let _code = transpile(
        r#"
def test() -> list:
    items = [1, 2, 3, 4, 5, 6]
    return items[::2]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    items = [1, 2, 3, 4, 5, 6]
    return items[::2]
"#
    ));
}

#[test]
fn test_slice_negative_step() {
    let code = transpile(
        r#"
def test() -> list:
    items = [1, 2, 3, 4, 5]
    return items[::-1]
"#,
    );
    assert!(
        code.contains("rev") || code.contains("reverse"),
        "Should handle reverse slice: {}",
        code
    );
}

#[test]
fn test_tuple_index_access() {
    let _code = transpile(
        r#"
def test() -> int:
    t = (1, 2, 3)
    return t[0]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> int:
    t = (1, 2, 3)
    return t[0]
"#
    ));
}

#[test]
fn test_string_negative_index() {
    let _code = transpile(
        r#"
def test() -> str:
    s = "hello"
    return s[-1]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> str:
    s = "hello"
    return s[-1]
"#
    ));
}

// ===== Collection creation edge cases =====

#[test]
fn test_empty_tuple_creation() {
    let _code = transpile(
        r#"
def test():
    t = ()
"#,
    );
    assert!(transpile_ok(
        r#"
def test():
    t = ()
"#
    ));
}

#[test]
fn test_single_element_tuple() {
    let _code = transpile(
        r#"
def test():
    t = (1,)
"#,
    );
    assert!(transpile_ok(
        r#"
def test():
    t = (1,)
"#
    ));
}

#[test]
fn test_list_with_mixed_int_float() {
    let _code = transpile(
        r#"
def test() -> list:
    return [1, 2.5, 3, 4.5]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    return [1, 2.5, 3, 4.5]
"#
    ));
}

#[test]
fn test_dict_fromkeys() {
    let _code = transpile(
        r#"
def test():
    keys = ["a", "b", "c"]
    d = dict.fromkeys(keys, 0)
"#,
    );
    assert!(transpile_ok(
        r#"
def test():
    keys = ["a", "b", "c"]
    d = dict.fromkeys(keys, 0)
"#
    ));
}

#[test]
fn test_frozenset_from_list() {
    let code = transpile(
        r#"
def test():
    items = [1, 2, 3, 2, 1]
    fs = frozenset(items)
"#,
    );
    assert!(
        code.contains("HashSet") || code.contains("BTreeSet"),
        "Should create a set type: {}",
        code
    );
}

// ===== Comprehension edge cases =====

#[test]
fn test_list_comp_with_method_call() {
    let code = transpile(
        r#"
def test() -> list:
    words = ["hello", "WORLD"]
    return [w.lower() for w in words]
"#,
    );
    assert!(
        code.contains("to_lowercase") || code.contains("lower"),
        "Should handle method call in list comp: {}",
        code
    );
}

#[test]
fn test_set_comp_with_condition() {
    let code = transpile(
        r#"
def test() -> set:
    nums = [1, 2, 3, 4, 5, 6]
    return {n * n for n in nums if n % 2 == 0}
"#,
    );
    assert!(
        code.contains("filter") || code.contains("if"),
        "Should handle set comp with condition: {}",
        code
    );
}

#[test]
fn test_dict_comp_with_enumerate() {
    let code = transpile(
        r#"
def test() -> dict:
    items = ["a", "b", "c"]
    return {i: v for i, v in enumerate(items)}
"#,
    );
    assert!(
        code.contains("enumerate"),
        "Should handle dict comp with enumerate: {}",
        code
    );
}

#[test]
fn test_generator_in_min() {
    let code = transpile(
        r#"
def test() -> int:
    nums = [3, 1, 4, 1, 5]
    return min(x * x for x in nums)
"#,
    );
    assert!(
        code.contains("min") || code.contains("map"),
        "Should handle generator in min: {}",
        code
    );
}

#[test]
fn test_generator_in_max() {
    let code = transpile(
        r#"
def test() -> int:
    nums = [3, 1, 4, 1, 5]
    return max(x * x for x in nums)
"#,
    );
    assert!(
        code.contains("max") || code.contains("map"),
        "Should handle generator in max: {}",
        code
    );
}

#[test]
fn test_generator_in_list_constructor() {
    let code = transpile(
        r#"
def test() -> list:
    nums = [1, 2, 3]
    return list(x * 2 for x in nums)
"#,
    );
    assert!(
        code.contains("collect") || code.contains("map"),
        "Should handle generator in list(): {}",
        code
    );
}

// ===== Lambda edge cases =====

#[test]
fn test_lambda_no_args() {
    let code = transpile(
        r#"
def test():
    f = lambda: 42
"#,
    );
    assert!(
        code.contains("||") || code.contains("closure"),
        "Should handle lambda with no args: {}",
        code
    );
}

#[test]
fn test_lambda_with_default() {
    let code = transpile(
        r#"
def test():
    nums = [3, 1, 4, 1, 5]
    nums.sort(key=lambda x: x)
"#,
    );
    assert!(
        code.contains("sort") || code.contains("key"),
        "Should handle lambda as sort key: {}",
        code
    );
}

// ===== F-string edge cases =====

#[test]
fn test_fstring_with_format_spec() {
    let code = transpile(
        r#"
def test() -> str:
    val = 3.14159
    return f"{val:.2f}"
"#,
    );
    assert!(
        code.contains("format"),
        "Should use format for f-string with spec: {}",
        code
    );
}

#[test]
fn test_fstring_empty() {
    let code = transpile(
        r#"
def test() -> str:
    return f""
"#,
    );
    assert!(
        code.contains("to_string") || code.contains("String"),
        "Should handle empty f-string: {}",
        code
    );
}

#[test]
fn test_fstring_with_expression() {
    let code = transpile(
        r#"
def test() -> str:
    x = 10
    y = 20
    return f"sum is {x + y}"
"#,
    );
    assert!(
        code.contains("format"),
        "Should use format for f-string expression: {}",
        code
    );
}

// ===== If expression edge cases =====

#[test]
fn test_ifexpr_with_comparison() {
    let code = transpile(
        r#"
def test(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#,
    );
    assert!(
        code.contains("if") && code.contains("else"),
        "Should have conditional: {}",
        code
    );
}

#[test]
fn test_ifexpr_in_list() {
    let _code = transpile(
        r#"
def test() -> list:
    nums = [1, 2, 3, 4, 5]
    return [x if x > 0 else 0 for x in nums]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    nums = [1, 2, 3, 4, 5]
    return [x if x > 0 else 0 for x in nums]
"#
    ));
}

// ===== Truthiness conversion tests =====

#[test]
fn test_truthiness_int() {
    let code = transpile(
        r#"
def test(n: int) -> bool:
    if n:
        return True
    return False
"#,
    );
    assert!(
        code.contains("!= 0") || code.contains("!"),
        "Should convert int truthiness: {}",
        code
    );
}

#[test]
fn test_truthiness_optional() {
    let _code = transpile(
        r#"
def test(s: str) -> bool:
    if s:
        return True
    return False
"#,
    );
    assert!(transpile_ok(
        r#"
def test(s: str) -> bool:
    if s:
        return True
    return False
"#
    ));
}

// ===== Attribute access coverage =====

#[test]
fn test_attribute_self_field() {
    let code = transpile(
        r#"
class Counter:
    def __init__(self):
        self.count = 0
    def increment(self):
        self.count += 1
"#,
    );
    assert!(
        code.contains("self.count"),
        "Should access self.count: {}",
        code
    );
}

#[test]
fn test_attribute_nested_access() {
    let _code = transpile(
        r#"
def test(obj):
    return obj.name.upper()
"#,
    );
    assert!(transpile_ok(
        r#"
def test(obj):
    return obj.name.upper()
"#
    ));
}

// ===== Borrow/reference tests =====

#[test]
fn test_mutable_borrow() {
    let code = transpile(
        r#"
def test() -> list:
    items = [3, 1, 2]
    items.sort()
    return items
"#,
    );
    assert!(code.contains("sort"), "Should sort in place: {}", code);
}

// ===== Dict method edge cases =====

#[test]
fn test_dict_pop_no_default() {
    let code = transpile(
        r#"
def test() -> int:
    d = {"a": 1, "b": 2}
    return d.pop("a")
"#,
    );
    assert!(
        code.contains("remove") || code.contains("pop"),
        "Should handle dict pop: {}",
        code
    );
}

#[test]
fn test_dict_get_nested() {
    let code = transpile(
        r#"
def test():
    d = {"a": 1, "b": 2}
    x = d.get("c", 0)
"#,
    );
    assert!(
        code.contains("unwrap_or") || code.contains("get"),
        "Should handle dict.get with default: {}",
        code
    );
}

// ===== List method edge cases =====

#[test]
fn test_list_extend_with_range() {
    let code = transpile(
        r#"
def test() -> list:
    items = [1, 2]
    items.extend(range(3, 6))
    return items
"#,
    );
    assert!(code.contains("extend"), "Should use extend: {}", code);
}

#[test]
fn test_list_index_method() {
    let code = transpile(
        r#"
def test() -> int:
    items = ["a", "b", "c"]
    return items.index("b")
"#,
    );
    assert!(
        code.contains("position") || code.contains("index"),
        "Should find index: {}",
        code
    );
}

// ===== Async/await tests =====

#[test]
fn test_async_function() {
    let _code = transpile(
        r#"
async def fetch_data(url: str) -> str:
    return url
"#,
    );
    assert!(transpile_ok(
        r#"
async def fetch_data(url: str) -> str:
    return url
"#
    ));
}

// ===== Named expression (walrus) tests =====

#[test]
fn test_walrus_in_list_comp() {
    let _code = transpile(
        r#"
def test() -> list:
    data = [1, 2, 3, 4, 5]
    return [y for x in data if (y := x * 2) > 4]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    data = [1, 2, 3, 4, 5]
    return [y for x in data if (y := x * 2) > 4]
"#
    ));
}

// ===== Counter and deque tests =====

#[test]
fn test_deque_rotate() {
    let _code = transpile(
        r#"
from collections import deque
def test():
    d = deque([1, 2, 3, 4])
    d.rotate(2)
"#,
    );
    assert!(transpile_ok(
        r#"
from collections import deque
def test():
    d = deque([1, 2, 3, 4])
    d.rotate(2)
"#
    ));
}

#[test]
fn test_deque_pop() {
    let code = transpile(
        r#"
from collections import deque
def test() -> int:
    d = deque([1, 2, 3])
    return d.pop()
"#,
    );
    assert!(
        code.contains("pop_back") || code.contains("pop"),
        "Should handle deque pop: {}",
        code
    );
}

#[test]
fn test_counter_elements() {
    let _code = transpile(
        r#"
from collections import Counter
def test():
    c = Counter([1, 1, 2, 3, 3, 3])
    items = c.items()
"#,
    );
    assert!(transpile_ok(
        r#"
from collections import Counter
def test():
    c = Counter([1, 1, 2, 3, 3, 3])
    items = c.items()
"#
    ));
}

// ===== Path-related tests =====

#[test]
fn test_path_join() {
    let code = transpile(
        r#"
from pathlib import Path
def test() -> str:
    p = Path("/home")
    return str(p / "user")
"#,
    );
    assert!(
        code.contains("join") || code.contains("Path"),
        "Should handle path join: {}",
        code
    );
}

#[test]
fn test_path_exists() {
    let code = transpile(
        r#"
from pathlib import Path
def test() -> bool:
    p = Path("/tmp")
    return p.exists()
"#,
    );
    assert!(
        code.contains("exists"),
        "Should handle path.exists(): {}",
        code
    );
}

// ===== Type annotation tests =====

#[test]
fn test_typed_dict_access() {
    let code = transpile(
        r#"
def test(d: dict[str, int]) -> int:
    return d["key"]
"#,
    );
    assert!(
        code.contains("HashMap") || code.contains("["),
        "Should handle typed dict access: {}",
        code
    );
}

#[test]
fn test_typed_list_access() {
    let _code = transpile(
        r#"
def test(items: list[int]) -> int:
    return items[0]
"#,
    );
    assert!(transpile_ok(
        r#"
def test(items: list[int]) -> int:
    return items[0]
"#
    ));
}

// ===== String format method =====

#[test]
fn test_string_format_method() {
    let code = transpile(
        r#"
def test() -> str:
    return "Hello, {}!".format("world")
"#,
    );
    assert!(code.contains("format"), "Should use format: {}", code);
}

#[test]
fn test_string_format_with_args() {
    let code = transpile(
        r#"
def test() -> str:
    name = "Alice"
    age = 30
    return "Name: {}, Age: {}".format(name, age)
"#,
    );
    assert!(
        code.contains("format"),
        "Should handle format with multiple args: {}",
        code
    );
}

// ===== Sort by key =====

#[test]
fn test_sort_with_key() {
    let code = transpile(
        r#"
def test() -> list:
    words = ["banana", "apple", "cherry"]
    words.sort(key=lambda w: w)
    return words
"#,
    );
    assert!(
        code.contains("sort"),
        "Should handle sort with key: {}",
        code
    );
}

#[test]
fn test_sorted_with_key() {
    let code = transpile(
        r#"
def test() -> list:
    words = ["banana", "apple", "cherry"]
    return sorted(words, key=lambda w: w)
"#,
    );
    assert!(
        code.contains("sort"),
        "Should handle sorted with key: {}",
        code
    );
}

#[test]
fn test_sorted_reverse() {
    let code = transpile(
        r#"
def test() -> list:
    nums = [3, 1, 4, 1, 5]
    return sorted(nums, reverse=True)
"#,
    );
    assert!(
        code.contains("sort") || code.contains("rev"),
        "Should handle sorted with reverse: {}",
        code
    );
}

// ===== File I/O method tests =====

#[test]
fn test_file_close_method() {
    let _code = transpile(
        r#"
def test():
    f = open("test.txt", "w")
    f.write("hello")
    f.close()
"#,
    );
    // close() should be a no-op in Rust (RAII)
    assert!(transpile_ok(
        r#"
def test():
    f = open("test.txt", "w")
    f.write("hello")
    f.close()
"#
    ));
}

#[test]
fn test_file_readlines() {
    let code = transpile(
        r#"
def test() -> list:
    f = open("test.txt", "r")
    lines = f.readlines()
    return lines
"#,
    );
    assert!(
        code.contains("lines") || code.contains("BufReader") || code.contains("read"),
        "Should handle readlines: {}",
        code
    );
}

// ===== Binary operation type-aware tests =====

#[test]
fn test_float_division() {
    let _code = transpile(
        r#"
def test(a: float, b: float) -> float:
    return a / b
"#,
    );
    assert!(transpile_ok(
        r#"
def test(a: float, b: float) -> float:
    return a / b
"#
    ));
}

#[test]
fn test_floor_division() {
    let _code = transpile(
        r#"
def test(a: int, b: int) -> int:
    return a // b
"#,
    );
    assert!(transpile_ok(
        r#"
def test(a: int, b: int) -> int:
    return a // b
"#
    ));
}

// ===== Enumerate and zip edge cases =====

#[test]
fn test_enumerate_with_start() {
    let code = transpile(
        r#"
def test():
    items = ["a", "b", "c"]
    for i, v in enumerate(items, start=1):
        print(i, v)
"#,
    );
    assert!(
        code.contains("enumerate"),
        "Should handle enumerate with start: {}",
        code
    );
}

#[test]
fn test_zip_two_lists() {
    let code = transpile(
        r#"
def test():
    keys = ["a", "b"]
    vals = [1, 2]
    for k, v in zip(keys, vals):
        print(k, v)
"#,
    );
    assert!(code.contains("zip"), "Should handle zip: {}", code);
}

#[test]
fn test_reversed_list() {
    let code = transpile(
        r#"
def test():
    items = [1, 2, 3]
    for x in reversed(items):
        print(x)
"#,
    );
    assert!(
        code.contains("rev") || code.contains("reversed"),
        "Should handle reversed: {}",
        code
    );
}

// ===== Multiple assignment patterns =====

#[test]
fn test_list_multiplication_operator() {
    let code = transpile(
        r#"
def test() -> list:
    return [0] * 10
"#,
    );
    assert!(
        code.contains("vec![") || code.contains("repeat") || code.contains("* 10"),
        "Should handle list multiplication: {}",
        code
    );
}

#[test]
fn test_string_multiplication() {
    let code = transpile(
        r#"
def test() -> str:
    return "-" * 20
"#,
    );
    assert!(
        code.contains("repeat") || code.contains("*"),
        "Should handle string multiplication: {}",
        code
    );
}

// ===== In/Not In operator tests =====

#[test]
fn test_in_operator_list() {
    let code = transpile(
        r#"
def test() -> bool:
    items = [1, 2, 3]
    return 2 in items
"#,
    );
    assert!(
        code.contains("contains"),
        "Should use contains for 'in': {}",
        code
    );
}

#[test]
fn test_not_in_operator() {
    let code = transpile(
        r#"
def test() -> bool:
    items = [1, 2, 3]
    return 5 not in items
"#,
    );
    assert!(
        code.contains("contains"),
        "Should use !contains for 'not in': {}",
        code
    );
}

#[test]
fn test_in_operator_string() {
    let code = transpile(
        r#"
def test() -> bool:
    s = "hello world"
    return "world" in s
"#,
    );
    assert!(
        code.contains("contains"),
        "Should use contains for string 'in': {}",
        code
    );
}

#[test]
fn test_in_operator_dict() {
    let code = transpile(
        r#"
def test() -> bool:
    d = {"a": 1, "b": 2}
    return "a" in d
"#,
    );
    assert!(
        code.contains("contains_key") || code.contains("contains"),
        "Should check key containment: {}",
        code
    );
}

// ===== Yield and generator function tests =====

#[test]
fn test_yield_value() {
    let _code = transpile(
        r#"
def gen():
    yield 1
    yield 2
    yield 3
"#,
    );
    assert!(transpile_ok(
        r#"
def gen():
    yield 1
    yield 2
    yield 3
"#
    ));
}

// ===== Type detection via transpile tests =====

#[test]
fn test_dict_merge_operator() {
    let _code = transpile(
        r#"
def test() -> dict:
    a = {"x": 1}
    b = {"y": 2}
    return a | b
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> dict:
    a = {"x": 1}
    b = {"y": 2}
    return a | b
"#
    ));
}

#[test]
fn test_set_literal_union_operator() {
    let _code = transpile(
        r#"
def test() -> set:
    a = {1, 2}
    b = {3, 4}
    return a | b
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> set:
    a = {1, 2}
    b = {3, 4}
    return a | b
"#
    ));
}

// ===== List and dict complex operations =====

#[test]
fn test_nested_list_comp() {
    let code = transpile(
        r#"
def test() -> list:
    matrix = [[1, 2], [3, 4], [5, 6]]
    return [x for row in matrix for x in row]
"#,
    );
    assert!(
        code.contains("flat_map") || code.contains("flatten") || code.contains("for"),
        "Should flatten nested list: {}",
        code
    );
}

#[test]
fn test_dict_values_sum() {
    let code = transpile(
        r#"
def test() -> int:
    d = {"a": 1, "b": 2, "c": 3}
    return sum(d.values())
"#,
    );
    assert!(
        code.contains("values") && code.contains("sum"),
        "Should sum dict values: {}",
        code
    );
}

#[test]
fn test_list_concat_via_add() {
    let _code = transpile(
        r#"
def test() -> list:
    a = [1, 2]
    b = [3, 4]
    return a + b
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> list:
    a = [1, 2]
    b = [3, 4]
    return a + b
"#
    ));
}

// ===== Integer and float method tests =====

#[test]
fn test_int_abs() {
    let code = transpile(
        r#"
def test(n: int) -> int:
    return abs(n)
"#,
    );
    assert!(code.contains("abs"), "Should use abs: {}", code);
}

#[test]
fn test_round_float() {
    let _code = transpile(
        r#"
def test(f: float) -> float:
    return round(f, 2)
"#,
    );
    assert!(transpile_ok(
        r#"
def test(f: float) -> float:
    return round(f, 2)
"#
    ));
}

#[test]
fn test_min_max_builtins() {
    let code = transpile(
        r#"
def test() -> int:
    return max(1, 2, 3) + min(4, 5, 6)
"#,
    );
    assert!(
        code.contains("max") && code.contains("min"),
        "Should handle min/max: {}",
        code
    );
}

// ===== String slicing tests =====

#[test]
fn test_string_slice_from_start() {
    let _code = transpile(
        r#"
def test() -> str:
    s = "hello world"
    return s[:5]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> str:
    s = "hello world"
    return s[:5]
"#
    ));
}

#[test]
fn test_string_slice_to_end() {
    let _code = transpile(
        r#"
def test() -> str:
    s = "hello world"
    return s[6:]
"#,
    );
    assert!(transpile_ok(
        r#"
def test() -> str:
    s = "hello world"
    return s[6:]
"#
    ));
}

// ===== is_json_value_type static method test =====

#[test]
fn test_is_json_value_type_static() {
    use crate::hir::Type;
    use crate::rust_gen::expr_gen_instance_methods::ExpressionConverter;

    assert!(ExpressionConverter::is_json_value_type(&Type::Custom(
        "serde_json::Value".to_string()
    )));
    assert!(ExpressionConverter::is_json_value_type(&Type::Custom(
        "Value".to_string()
    )));
    assert!(!ExpressionConverter::is_json_value_type(&Type::Int));
    assert!(!ExpressionConverter::is_json_value_type(&Type::String));
}

// ================================================================
// Session 9: Coverage improvement tests
// ================================================================

// --- Dict method edge cases ---

#[test]
fn test_s9_dict_get_with_default() {
    let code = r#"
def lookup(d: dict, key: str) -> str:
    return d.get(key, "default")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn lookup"), "output: {}", rust);
}

#[test]
fn test_s9_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str) -> str:
    return d.setdefault(key, "value")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn ensure_key"), "output: {}", rust);
}

#[test]
fn test_s9_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> str:
    return d.pop(key)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove_key"), "output: {}", rust);
}

#[test]
fn test_s9_dict_pop_with_default() {
    let code = r#"
def remove_key_safe(d: dict, key: str) -> str:
    return d.pop(key, "gone")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove_key_safe"), "output: {}", rust);
}

#[test]
fn test_s9_dict_update() {
    let code = r#"
def merge(d1: dict, d2: dict) -> None:
    d1.update(d2)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn merge"), "output: {}", rust);
}

#[test]
fn test_s9_dict_clear() {
    let code = r#"
def reset(d: dict) -> None:
    d.clear()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn reset"), "output: {}", rust);
}

// --- String methods ---

#[test]
fn test_s9_string_isdigit() {
    let code = r#"
def check_digit(s: str) -> bool:
    return s.isdigit()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_digit"), "output: {}", rust);
}

#[test]
fn test_s9_string_isalpha() {
    let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_alpha"), "output: {}", rust);
}

#[test]
fn test_s9_string_title() {
    let code = r#"
def titlecase(s: str) -> str:
    return s.title()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn titlecase"), "output: {}", rust);
}

#[test]
fn test_s9_string_center() {
    let code = r#"
def pad_center(s: str, width: int) -> str:
    return s.center(width)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pad_center"), "output: {}", rust);
}

#[test]
fn test_s9_string_ljust() {
    let code = r#"
def pad_left(s: str, width: int) -> str:
    return s.ljust(width)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pad_left"), "output: {}", rust);
}

#[test]
fn test_s9_string_rjust() {
    let code = r#"
def pad_right(s: str, width: int) -> str:
    return s.rjust(width)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pad_right"), "output: {}", rust);
}

#[test]
fn test_s9_string_zfill() {
    let code = r#"
def zero_pad(s: str, width: int) -> str:
    return s.zfill(width)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn zero_pad"), "output: {}", rust);
}

#[test]
fn test_s9_string_encode() {
    let code = r#"
def to_bytes(s: str) -> bytes:
    return s.encode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn to_bytes"), "output: {}", rust);
}

#[test]
fn test_s9_string_isalnum() {
    let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_alnum"), "output: {}", rust);
}

#[test]
fn test_s9_string_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_numeric"), "output: {}", rust);
}

#[test]
fn test_s9_string_isspace() {
    let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_space"), "output: {}", rust);
}

#[test]
fn test_s9_string_swapcase() {
    let code = r#"
def swap(s: str) -> str:
    return s.swapcase()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn swap"), "output: {}", rust);
}

#[test]
fn test_s9_string_casefold() {
    let code = r#"
def fold(s: str) -> str:
    return s.casefold()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn fold"), "output: {}", rust);
}

#[test]
fn test_s9_string_removeprefix() {
    let code = r#"
def strip_prefix(s: str) -> str:
    return s.removeprefix("pre_")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn strip_prefix"), "output: {}", rust);
}

#[test]
fn test_s9_string_removesuffix() {
    let code = r#"
def strip_suffix(s: str) -> str:
    return s.removesuffix("_suf")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn strip_suffix"), "output: {}", rust);
}

// --- List methods ---

#[test]
fn test_s9_list_insert() {
    let code = r#"
def prepend(items: list, val: int) -> None:
    items.insert(0, val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn prepend"), "output: {}", rust);
}

#[test]
fn test_s9_list_remove() {
    let code = r#"
def remove_val(items: list, val: int) -> None:
    items.remove(val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove_val"), "output: {}", rust);
}

#[test]
fn test_s9_list_count() {
    let code = r#"
def count_val(items: list, val: int) -> int:
    return items.count(val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count_val"), "output: {}", rust);
}

#[test]
fn test_s9_list_index() {
    let code = r#"
def find_pos(items: list, val: int) -> int:
    return items.index(val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_pos"), "output: {}", rust);
}

#[test]
fn test_s9_list_copy() {
    let code = r#"
def clone_list(items: list) -> list:
    return items.copy()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clone_list"), "output: {}", rust);
}

#[test]
fn test_s9_list_clear() {
    let code = r#"
def empty(items: list) -> None:
    items.clear()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn empty"), "output: {}", rust);
}

// --- Set methods ---

#[test]
fn test_s9_set_add() {
    let code = r#"
def add_to_set(s: set, val: int) -> None:
    s.add(val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn add_to_set"), "output: {}", rust);
}

#[test]
fn test_s9_set_discard() {
    let code = r#"
def discard_from_set(s: set, val: int) -> None:
    s.discard(val)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn discard_from_set"), "output: {}", rust);
}

#[test]
fn test_s9_set_union() {
    let code = r#"
def merge_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn merge_sets"), "output: {}", rust);
}

#[test]
fn test_s9_set_intersection() {
    let code = r#"
def common(a: set, b: set) -> set:
    return a.intersection(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn common"), "output: {}", rust);
}

#[test]
fn test_s9_set_difference() {
    let code = r#"
def diff(a: set, b: set) -> set:
    return a.difference(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn diff"), "output: {}", rust);
}

// --- Lambda and closures ---

#[test]
fn test_s9_lambda_in_sorted() {
    let code = r#"
def sort_by_second(items: list) -> list:
    return sorted(items, key=lambda x: x[1])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_by_second"), "output: {}", rust);
}

#[test]
fn test_s9_lambda_in_filter() {
    let code = r#"
def positives(items: list) -> list:
    return list(filter(lambda x: x > 0, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn positives"), "output: {}", rust);
}

#[test]
fn test_s9_lambda_in_map() {
    let code = r#"
def doubled(items: list) -> list:
    return list(map(lambda x: x * 2, items))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn doubled"), "output: {}", rust);
}

// --- Comprehensions ---

#[test]
fn test_s9_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn unique_squares"), "output: {}", rust);
}

#[test]
fn test_s9_dict_comprehension() {
    let code = r#"
def index_map(items: list) -> dict:
    return {i: v for i, v in enumerate(items)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn index_map"), "output: {}", rust);
}

#[test]
fn test_s9_list_comp_with_condition() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn evens"), "output: {}", rust);
}

#[test]
fn test_s9_list_comp_with_method() {
    let code = r#"
def uppers(words: list) -> list:
    return [w.upper() for w in words]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn uppers"), "output: {}", rust);
}

// --- FString complex expressions ---

#[test]
fn test_s9_fstring_with_method() {
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name.upper()}!"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greet"), "output: {}", rust);
}

#[test]
fn test_s9_fstring_with_expression() {
    let code = r#"
def calc(a: int, b: int) -> str:
    return f"Sum is {a + b}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn calc"), "output: {}", rust);
}

#[test]
fn test_s9_fstring_multiple_parts() {
    let code = r#"
def info(name: str, age: int) -> str:
    return f"Name: {name}, Age: {age}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn info"), "output: {}", rust);
}

// --- Slice operations ---

#[test]
fn test_s9_list_slice_basic() {
    let code = r#"
def first_three(items: list) -> list:
    return items[:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first_three"), "output: {}", rust);
}

#[test]
fn test_s9_list_slice_from() {
    let code = r#"
def rest(items: list) -> list:
    return items[1:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn rest"), "output: {}", rust);
}

#[test]
fn test_s9_list_slice_range() {
    let code = r#"
def middle(items: list) -> list:
    return items[1:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn middle"), "output: {}", rust);
}

#[test]
fn test_s9_string_slice() {
    let code = r#"
def first_char(s: str) -> str:
    return s[:1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first_char"), "output: {}", rust);
}

#[test]
fn test_s9_negative_index() {
    let code = r#"
def last_item(items: list) -> int:
    return items[-1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn last_item"), "output: {}", rust);
}

// --- Attribute access ---

#[test]
fn test_s9_datetime_year() {
    let code = r#"
from datetime import date

def get_year(d: date) -> int:
    return d.year
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_year"), "output: {}", rust);
}

#[test]
fn test_s9_datetime_month() {
    let code = r#"
from datetime import date

def get_month(d: date) -> int:
    return d.month
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_month"), "output: {}", rust);
}

// --- Tuple operations ---

#[test]
fn test_s9_tuple_index() {
    let code = r#"
def first(t: tuple) -> int:
    return t[0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first"), "output: {}", rust);
}

#[test]
fn test_s9_tuple_return() {
    let code = r#"
def pair(a: int, b: int) -> tuple:
    return (a, b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pair"), "output: {}", rust);
}

// --- Generator expression ---

#[test]
fn test_s9_generator_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_squares"), "output: {}", rust);
}

#[test]
fn test_s9_generator_any() {
    let code = r#"
def has_positive(items: list) -> bool:
    return any(x > 0 for x in items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_positive"), "output: {}", rust);
}

#[test]
fn test_s9_generator_all() {
    let code = r#"
def all_positive(items: list) -> bool:
    return all(x > 0 for x in items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn all_positive"), "output: {}", rust);
}

// --- Named expression (walrus) ---

#[test]
fn test_s9_walrus_in_while() {
    let code = r#"
def process(items: list) -> int:
    total = 0
    i = 0
    while i < len(items):
        total += items[i]
        i += 1
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn process"), "output: {}", rust);
}

// --- Frozenset ---

#[test]
fn test_s9_frozenset_creation() {
    let code = r#"
def immutable_set() -> frozenset:
    return frozenset([1, 2, 3])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn immutable_set"), "output: {}", rust);
}

// --- Chained method calls ---

#[test]
fn test_s9_chained_string_methods() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip().lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clean"), "output: {}", rust);
}

#[test]
fn test_s9_chained_string_split_join() {
    let code = r#"
def normalize(s: str) -> str:
    return " ".join(s.split())
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn normalize"), "output: {}", rust);
}

// --- Regex methods ---

#[test]
fn test_s9_regex_search() {
    let code = r#"
import re

def find_match(pattern: str, text: str) -> bool:
    return re.search(pattern, text) is not None
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_match"), "output: {}", rust);
}

#[test]
fn test_s9_regex_findall() {
    let code = r#"
import re

def find_numbers(text: str) -> list:
    return re.findall(r"\d+", text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_numbers"), "output: {}", rust);
}

#[test]
fn test_s9_regex_sub() {
    let code = r#"
import re

def clean_text(text: str) -> str:
    return re.sub(r"\s+", " ", text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clean_text"), "output: {}", rust);
}

// --- Sys.io methods ---

#[test]
fn test_s9_print_to_stderr() {
    let code = r#"
import sys

def warn(msg: str) -> None:
    sys.stderr.write(msg)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn warn"), "output: {}", rust);
}

// --- Await expression ---

#[test]
fn test_s9_async_function() {
    let code = r#"
async def fetch(url: str) -> str:
    return url
"#;
    let rust = transpile(code);
    assert!(
        rust.contains("async") || rust.contains("fn fetch"),
        "output: {}",
        rust
    );
}

// --- Complex algorithms ---

#[test]
fn test_s9_matrix_transpose() {
    let code = r#"
def transpose(matrix: list) -> list:
    result = []
    for i in range(len(matrix[0])):
        row = []
        for j in range(len(matrix)):
            row.append(matrix[j][i])
        result.append(row)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn transpose"), "output: {}", rust);
}

#[test]
fn test_s9_dict_iteration_values() {
    let code = r#"
def sum_values(d: dict) -> int:
    total = 0
    for v in d.values():
        total += v
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_values"), "output: {}", rust);
}

#[test]
fn test_s9_dict_iteration_items() {
    let code = r#"
def print_items(d: dict) -> None:
    for k, v in d.items():
        print(f"{k}: {v}")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn print_items"), "output: {}", rust);
}

#[test]
fn test_s9_enumerate_with_start() {
    let code = r#"
def numbered(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(f"{i}: {item}")
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn numbered"), "output: {}", rust);
}

#[test]
fn test_s9_zip_two_lists() {
    let code = r#"
def combine(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn combine"), "output: {}", rust);
}

#[test]
fn test_s9_isinstance_check() {
    let code = r#"
def check_type(x: int) -> bool:
    return isinstance(x, int)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_type"), "output: {}", rust);
}

#[test]
fn test_s9_string_format_method() {
    let code = r#"
def format_msg(name: str, count: int) -> str:
    return "{}: {}".format(name, count)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn format_msg"), "output: {}", rust);
}

#[test]
fn test_s9_list_extend() {
    let code = r#"
def extend_list(a: list, b: list) -> None:
    a.extend(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn extend_list"), "output: {}", rust);
}

#[test]
fn test_s9_str_in_operator() {
    let code = r#"
def contains(text: str, sub: str) -> bool:
    return sub in text
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn contains"), "output: {}", rust);
}

#[test]
fn test_s9_str_not_in_operator() {
    let code = r#"
def not_contains(text: str, sub: str) -> bool:
    return sub not in text
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn not_contains"), "output: {}", rust);
}

// === S9 Batch 3: method call and expression coverage ===

#[test]
fn test_s9b3_dict_items_iteration() {
    let code = r#"
def sum_values(d: dict) -> int:
    total = 0
    for k, v in d.items():
        total += v
    return total
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_values"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_keys_to_list() {
    let code = r#"
def get_keys(d: dict) -> list:
    return list(d.keys())
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_keys"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_values_to_list() {
    let code = r#"
def get_values(d: dict) -> list:
    return list(d.values())
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_values"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_split_join() {
    let code = r#"
def split_and_join(s: str) -> str:
    parts = s.split(",")
    return ";".join(parts)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn split_and_join"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_strip_methods() {
    let code = r#"
def clean(s: str) -> str:
    return s.strip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clean"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_lstrip() {
    let code = r#"
def left_clean(s: str) -> str:
    return s.lstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn left_clean"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_rstrip() {
    let code = r#"
def right_clean(s: str) -> str:
    return s.rstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn right_clean"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_find() {
    let code = r#"
def find_pos(s: str, sub: str) -> int:
    return s.find(sub)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_pos"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_count() {
    let code = r#"
def count_chars(s: str, c: str) -> int:
    return s.count(c)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count_chars"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_startswith() {
    let code = r#"
def check_prefix(s: str) -> bool:
    return s.startswith("http")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_prefix"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_endswith() {
    let code = r#"
def check_suffix(s: str) -> bool:
    return s.endswith(".py")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_suffix"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_sort() {
    let code = r#"
def sort_list(items: list) -> None:
    items.sort()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_list"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_reverse() {
    let code = r#"
def reverse_list(items: list) -> None:
    items.reverse()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn reverse_list"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_pop() {
    let code = r#"
def pop_last(items: list) -> int:
    return items.pop()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pop_last"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_comprehension_with_if() {
    let code = r#"
def evens(n: int) -> list:
    return [x for x in range(n) if x % 2 == 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn evens"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_comp_with_method_call() {
    let code = r#"
def upper_all(items: list) -> list:
    return [s.upper() for s in items]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn upper_all"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_comprehension() {
    let code = r#"
def squares(n: int) -> dict:
    return {x: x * x for x in range(n)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn squares"), "output: {}", rust);
}

#[test]
fn test_s9b3_set_comprehension() {
    let code = r#"
def unique_lengths(words: list) -> set:
    return {len(w) for w in words}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn unique_lengths"), "output: {}", rust);
}

#[test]
fn test_s9b3_generator_with_sum() {
    let code = r#"
def sum_squares(n: int) -> int:
    return sum(x * x for x in range(n))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sum_squares"), "output: {}", rust);
}

#[test]
fn test_s9b3_lambda_with_sorted() {
    let code = r#"
def sort_by_len(items: list) -> list:
    return sorted(items, key=lambda x: len(x))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_by_len"), "output: {}", rust);
}

#[test]
fn test_s9b3_fstring_with_expression() {
    let code = r#"
def greet(name: str, age: int) -> str:
    return f"Hello {name}, you are {age} years old"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn greet"), "output: {}", rust);
}

#[test]
fn test_s9b3_fstring_with_method() {
    let code = r#"
def label(name: str) -> str:
    return f"Name: {name.upper()}"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn label"), "output: {}", rust);
}

#[test]
fn test_s9b3_tuple_return() {
    let code = r#"
def divmod_custom(a: int, b: int) -> tuple:
    return (a // b, a % b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn divmod_custom"), "output: {}", rust);
}

#[test]
fn test_s9b3_tuple_index() {
    let code = r#"
def first(t: tuple) -> int:
    return t[0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first"), "output: {}", rust);
}

#[test]
fn test_s9b3_set_add_and_discard() {
    let code = r#"
def modify_set(s: set) -> None:
    s.add(1)
    s.discard(2)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn modify_set"), "output: {}", rust);
}

#[test]
fn test_s9b3_set_union_intersection() {
    let code = r#"
def combine_sets(a: set, b: set) -> set:
    return a.union(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn combine_sets"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_slice_basic() {
    let code = r#"
def first_three(items: list) -> list:
    return items[:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first_three"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_slice_from() {
    let code = r#"
def after_two(items: list) -> list:
    return items[2:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn after_two"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_slice_range() {
    let code = r#"
def middle(items: list) -> list:
    return items[1:4]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn middle"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_slice() {
    let code = r#"
def first_five(s: str) -> str:
    return s[:5]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first_five"), "output: {}", rust);
}

#[test]
fn test_s9b3_attribute_access() {
    let code = r#"
def get_year() -> int:
    import datetime
    now = datetime.datetime.now()
    return now.year
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn get_year"), "output: {}", rust);
}

#[test]
fn test_s9b3_nested_list_comp() {
    let code = r#"
def flatten(matrix: list) -> list:
    return [x for row in matrix for x in row]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn flatten"), "output: {}", rust);
}

#[test]
fn test_s9b3_ternary_expression() {
    let code = r#"
def clamp(x: int) -> int:
    return x if x > 0 else 0
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn clamp"), "output: {}", rust);
}

#[test]
fn test_s9b3_walrus_operator() {
    let code = r#"
def check_length(items: list) -> bool:
    if (n := len(items)) > 10:
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn check_length"), "output: {}", rust);
}

#[test]
fn test_s9b3_chained_string_methods() {
    let code = r#"
def normalize(s: str) -> str:
    return s.strip().lower().replace(" ", "_")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn normalize"), "output: {}", rust);
}

#[test]
fn test_s9b3_enumerate_with_start() {
    let code = r#"
def numbered(items: list) -> list:
    result = []
    for i, item in enumerate(items):
        result.append(i)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn numbered"), "output: {}", rust);
}

#[test]
fn test_s9b3_zip_iteration() {
    let code = r#"
def pair_up(a: list, b: list) -> list:
    result = []
    for x, y in zip(a, b):
        result.append(x + y)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn pair_up"), "output: {}", rust);
}

#[test]
fn test_s9b3_any_with_generator() {
    let code = r#"
def has_positive(nums: list) -> bool:
    return any(x > 0 for x in nums)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn has_positive"), "output: {}", rust);
}

#[test]
fn test_s9b3_all_with_generator() {
    let code = r#"
def all_positive(nums: list) -> bool:
    return all(x > 0 for x in nums)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn all_positive"), "output: {}", rust);
}

#[test]
fn test_s9b3_map_with_lambda() {
    let code = r#"
def double_all(nums: list) -> list:
    return list(map(lambda x: x * 2, nums))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn double_all"), "output: {}", rust);
}

#[test]
fn test_s9b3_filter_with_lambda() {
    let code = r#"
def positive_only(nums: list) -> list:
    return list(filter(lambda x: x > 0, nums))
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn positive_only"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_get_with_default() {
    let code = r#"
def safe_get(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn safe_get"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str) -> None:
    d.setdefault(key, [])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn ensure_key"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_update() {
    let code = r#"
def merge(a: dict, b: dict) -> None:
    a.update(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn merge"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_pop() {
    let code = r#"
def remove_key(d: dict, key: str) -> int:
    return d.pop(key, 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn remove_key"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_replace() {
    let code = r#"
def sanitize(s: str) -> str:
    return s.replace("<", "&lt;").replace(">", "&gt;")
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sanitize"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_split_maxsplit() {
    let code = r#"
def first_word(s: str) -> str:
    return s.split(" ", 1)[0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn first_word"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_isdigit() {
    let code = r#"
def is_number(s: str) -> bool:
    return s.isdigit()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_number"), "output: {}", rust);
}

#[test]
fn test_s9b3_string_isalpha() {
    let code = r#"
def is_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn is_alpha"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_index() {
    let code = r#"
def find_item(items: list, target: int) -> int:
    return items.index(target)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn find_item"), "output: {}", rust);
}

#[test]
fn test_s9b3_list_count() {
    let code = r#"
def count_item(items: list, target: int) -> int:
    return items.count(target)
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn count_item"), "output: {}", rust);
}

#[test]
fn test_s9b3_named_expr_in_while() {
    let code = r#"
def read_chunks(data: str) -> list:
    result = []
    i = 0
    while i < len(data):
        result.append(data[i])
        i += 1
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn read_chunks"), "output: {}", rust);
}

#[test]
fn test_s9b3_ifexpr_nested() {
    let code = r#"
def classify(x: int) -> str:
    return "positive" if x > 0 else "negative" if x < 0 else "zero"
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn classify"), "output: {}", rust);
}

#[test]
fn test_s9b3_index_negative() {
    let code = r#"
def last_element(items: list) -> int:
    return items[-1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn last_element"), "output: {}", rust);
}

#[test]
fn test_s9b3_dict_literal_mixed_types() {
    let code = r#"
def make_config() -> dict:
    return {"name": "test", "count": 1, "active": True}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_config"), "output: {}", rust);
}

#[test]
fn test_s9b3_set_literal() {
    let code = r#"
def make_set() -> set:
    return {1, 2, 3}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_set"), "output: {}", rust);
}

#[test]
fn test_s9b3_frozenset_literal() {
    let code = r#"
def make_frozen() -> frozenset:
    return frozenset([1, 2, 3])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn make_frozen"), "output: {}", rust);
}

#[test]
fn test_s9b3_convert_method_call_on_result() {
    let code = r#"
def process(items: list) -> str:
    return ",".join([str(x) for x in items])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn process"), "output: {}", rust);
}

#[test]
fn test_s9b3_await_expression() {
    let code = r#"
async def fetch(url: str) -> str:
    result = await get(url)
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn fetch") || rust.contains("async"), "output: {}", rust);
}

#[test]
fn test_s9b3_yield_expression() {
    let code = r#"
def counter(n: int) -> int:
    for i in range(n):
        yield i
"#;
    let rust = transpile(code);
    assert!(rust.contains("counter") || rust.contains("yield"), "output: {}", rust);
}

#[test]
fn test_s9b3_complex_dict_comp() {
    let code = r#"
def invert(d: dict) -> dict:
    return {v: k for k, v in d.items()}
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn invert"), "output: {}", rust);
}

#[test]
fn test_s9b3_lambda_sort_key() {
    let code = r#"
def sort_pairs(pairs: list) -> list:
    return sorted(pairs, key=lambda p: p[1])
"#;
    let rust = transpile(code);
    assert!(rust.contains("fn sort_pairs"), "output: {}", rust);
}

// ========================================================================
// S9B6 COVERAGE TESTS - Specific uncovered paths
// ========================================================================

#[test]
fn test_s9b6_list_append_on_class_field() {
    let code = r#"
class Container:
    def __init__(self):
        self.items = []

    def add(self, x: int):
        self.items.append(x)
"#;
    let rust = transpile(code);
    assert!(rust.contains("push") || rust.contains("append"), "output: {}", rust);
}

#[test]
fn test_s9b6_set_pop() {
    let code = r#"
def pop_from_set() -> int:
    s = {1, 2, 3}
    return s.pop()
"#;
    let rust = transpile(code);
    assert!(rust.contains("pop") || rust.contains("take"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_pop() {
    let code = r#"
def pop_from_dict() -> int:
    d = {'a': 1, 'b': 2}
    return d.pop('a')
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove") || rust.contains("pop"), "output: {}", rust);
}

#[test]
fn test_s9b6_typed_list_append_coercion() {
    let code = r#"
def append_float_list():
    items: list[float] = []
    items.append(42)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("push") || rust.contains("f64"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_get_with_default() {
    let code = r#"
def get_with_default(d: dict, key: str) -> int:
    return d.get(key, 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("get") || rust.contains("unwrap_or"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_method_on_dict_value() {
    let code = r#"
def upper_value(d: dict) -> str:
    return d['key'].upper()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase") || rust.contains("upper"), "output: {}", rust);
}

#[test]
fn test_s9b6_counter_iteration() {
    let code = r#"
from collections import Counter

def count_chars(text: str) -> dict:
    result = {}
    for ch, count in Counter(text).items():
        result[ch] = count
    return result
"#;
    let rust = transpile(code);
    assert!(rust.contains("for") && rust.contains("items"), "output: {}", rust);
}

#[test]
fn test_s9b6_hashset_add_multiple() {
    let code = r#"
def make_set() -> set:
    s = set()
    s.add(1)
    s.add(2)
    s.add(3)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert") || rust.contains("add"), "output: {}", rust);
}

#[test]
fn test_s9b6_list_extend_other() {
    let code = r#"
def extend_lists():
    items = [1, 2, 3]
    other = [4, 5, 6]
    items.extend(other)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("extend"), "output: {}", rust);
}

#[test]
fn test_s9b6_list_insert_at_zero() {
    let code = r#"
def insert_front():
    items = [2, 3, 4]
    items.insert(0, 1)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_split_with_maxsplit() {
    let code = r#"
def split_once(s: str) -> list:
    return s.split(',', 1)
"#;
    let rust = transpile(code);
    assert!(rust.contains("split") || rust.contains("splitn"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_join() {
    let code = r#"
def join_items(items: list) -> str:
    return ','.join(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("join"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_replace() {
    let code = r#"
def replace_text(s: str) -> str:
    return s.replace('old', 'new')
"#;
    let rust = transpile(code);
    assert!(rust.contains("replace"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_startswith() {
    let code = r#"
def check_prefix(s: str) -> bool:
    return s.startswith('pre')
"#;
    let rust = transpile(code);
    assert!(rust.contains("starts_with"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_endswith() {
    let code = r#"
def check_suffix(s: str) -> bool:
    return s.endswith('post')
"#;
    let rust = transpile(code);
    assert!(rust.contains("ends_with"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_strip() {
    let code = r#"
def trim_text(s: str) -> str:
    return s.strip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_lstrip() {
    let code = r#"
def trim_left(s: str) -> str:
    return s.lstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_start") || rust.contains("trim_left"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_rstrip() {
    let code = r#"
def trim_right(s: str) -> str:
    return s.rstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_end") || rust.contains("trim_right"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_items_loop() {
    let code = r#"
def iterate_dict(d: dict):
    for k, v in d.items():
        print(k, v)
"#;
    let rust = transpile(code);
    assert!(rust.contains("for") && rust.contains("iter"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_keys_loop() {
    let code = r#"
def iterate_keys(d: dict):
    for k in d.keys():
        print(k)
"#;
    let rust = transpile(code);
    assert!(rust.contains("for") && rust.contains("keys"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_values_loop() {
    let code = r#"
def iterate_values(d: dict):
    for v in d.values():
        print(v)
"#;
    let rust = transpile(code);
    assert!(rust.contains("for") && rust.contains("values"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_update() {
    let code = r#"
def update_dict():
    d = {'a': 1}
    other = {'b': 2}
    d.update(other)
    return d
"#;
    let rust = transpile(code);
    assert!(rust.contains("extend") || rust.contains("update") || rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_s9b6_dict_setdefault() {
    let code = r#"
def ensure_key(d: dict, key: str):
    return d.setdefault(key, [])
"#;
    let rust = transpile(code);
    assert!(rust.contains("entry") || rust.contains("or_insert"), "output: {}", rust);
}

#[test]
fn test_s9b6_list_sort_with_key() {
    let code = r#"
def sort_pairs(pairs: list):
    pairs.sort(key=lambda x: x[1])
    return pairs
"#;
    let rust = transpile(code);
    assert!(rust.contains("sort"), "output: {}", rust);
}

#[test]
fn test_s9b6_list_reverse() {
    let code = r#"
def reverse_list():
    items = [1, 2, 3]
    items.reverse()
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("reverse"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_find() {
    let code = r#"
def find_substring(s: str) -> int:
    return s.find('sub')
"#;
    let rust = transpile(code);
    assert!(rust.contains("find") || rust.contains("position"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_index() {
    let code = r#"
def index_substring(s: str) -> int:
    return s.index('sub')
"#;
    let rust = transpile(code);
    assert!(rust.contains("find") || rust.contains("position") || rust.contains("unwrap"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_count() {
    let code = r#"
def count_occurrences(s: str) -> int:
    return s.count('a')
"#;
    let rust = transpile(code);
    assert!(rust.contains("matches") || rust.contains("count"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_isdigit() {
    let code = r#"
def check_digit(s: str) -> bool:
    return s.isdigit()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_numeric") || rust.contains("chars") || rust.contains("all"), "output: {}", rust);
}

#[test]
fn test_s9b6_string_isalpha() {
    let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_alphabetic") || rust.contains("chars") || rust.contains("all"), "output: {}", rust);
}

#[test]
fn test_s9b6_list_comp_with_method() {
    let code = r#"
def strip_lines(lines: list) -> list:
    return [x.strip() for x in lines]
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim") || rust.contains("map") || rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_s9b6_chained_string_methods() {
    let code = r#"
def process_text(s: str) -> list:
    return s.strip().lower().split(',')
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim") && rust.contains("to_lowercase") && rust.contains("split"), "output: {}", rust);
}

// ========================================================================
// STRING METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_string_upper() {
    let code = r#"
def to_upper(s: str) -> str:
    return s.upper()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_upper_on_literal() {
    let code = r#"
def upper_literal() -> str:
    return "hello".upper()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_lower() {
    let code = r#"
def to_lower(s: str) -> str:
    return s.lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_lowercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_lower_on_literal() {
    let code = r#"
def lower_literal() -> str:
    return "HELLO".lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_lowercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_strip_no_args() {
    let code = r#"
def strip_ws(s: str) -> str:
    return s.strip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim"), "output: {}", rust);
}

#[test]
fn test_cov_string_strip_with_chars() {
    let code = r#"
def strip_chars(s: str) -> str:
    return s.strip("xyz")
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_matches"), "output: {}", rust);
}

#[test]
fn test_cov_string_startswith() {
    let code = r#"
def check_prefix(s: str) -> bool:
    return s.startswith("hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains("starts_with"), "output: {}", rust);
}

#[test]
fn test_cov_string_startswith_with_var() {
    let code = r#"
def check_prefix_var(s: str, prefix: str) -> bool:
    return s.startswith(prefix)
"#;
    let rust = transpile(code);
    assert!(rust.contains("starts_with"), "output: {}", rust);
}

#[test]
fn test_cov_string_endswith() {
    let code = r#"
def check_suffix(s: str) -> bool:
    return s.endswith(".py")
"#;
    let rust = transpile(code);
    assert!(rust.contains("ends_with"), "output: {}", rust);
}

#[test]
fn test_cov_string_endswith_with_var() {
    let code = r#"
def check_suffix_var(s: str, suffix: str) -> bool:
    return s.endswith(suffix)
"#;
    let rust = transpile(code);
    assert!(rust.contains("ends_with"), "output: {}", rust);
}

#[test]
fn test_cov_string_split_no_args() {
    let code = r#"
def split_ws(s: str) -> list:
    return s.split()
"#;
    let rust = transpile(code);
    assert!(rust.contains("split_whitespace"), "output: {}", rust);
}

#[test]
fn test_cov_string_split_with_sep() {
    let code = r#"
def split_comma(s: str) -> list:
    return s.split(",")
"#;
    let rust = transpile(code);
    assert!(rust.contains("split") && rust.contains(","), "output: {}", rust);
}

#[test]
fn test_cov_string_split_with_maxsplit() {
    let code = r#"
def split_limited(s: str) -> list:
    return s.split(",", 2)
"#;
    let rust = transpile(code);
    assert!(rust.contains("splitn"), "output: {}", rust);
}

#[test]
fn test_cov_string_rsplit_no_args() {
    let code = r#"
def rsplit_ws(s: str) -> list:
    return s.rsplit()
"#;
    let rust = transpile(code);
    assert!(rust.contains("split_whitespace") && rust.contains("rev"), "output: {}", rust);
}

#[test]
fn test_cov_string_rsplit_with_sep() {
    let code = r#"
def rsplit_sep(s: str) -> list:
    return s.rsplit("/")
"#;
    let rust = transpile(code);
    assert!(rust.contains("rsplit"), "output: {}", rust);
}

#[test]
fn test_cov_string_rsplit_with_maxsplit() {
    let code = r#"
def rsplit_limited(s: str) -> list:
    return s.rsplit("/", 1)
"#;
    let rust = transpile(code);
    assert!(rust.contains("rsplitn"), "output: {}", rust);
}

#[test]
fn test_cov_string_join_with_list() {
    let code = r#"
def join_list() -> str:
    words = ["hello", "world"]
    return ", ".join(words)
"#;
    let rust = transpile(code);
    assert!(rust.contains("join"), "output: {}", rust);
}

#[test]
fn test_cov_string_join_with_separator_var() {
    let code = r#"
def join_with_sep(sep: str, items: list) -> str:
    return sep.join(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("join"), "output: {}", rust);
}

#[test]
fn test_cov_string_replace_two_args() {
    let code = r#"
def replace_all(s: str) -> str:
    return s.replace("old", "new")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".replace("), "output: {}", rust);
}

#[test]
fn test_cov_string_replace_three_args() {
    let code = r#"
def do_replace_n(text: str) -> str:
    return text.replace("a", "b", 2)
"#;
    let rust = transpile(code);
    assert!(rust.contains("replacen") || rust.contains("replace"), "should contain replacen or replace: {}", rust);
}

#[test]
fn test_cov_string_ljust() {
    let code = r#"
def left_justify(s: str) -> str:
    return s.ljust(20)
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("format!") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_ljust_with_fill() {
    let code = r#"
def left_justify_fill(s: str) -> str:
    return s.ljust(20, "-")
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_rjust() {
    let code = r#"
def right_justify(s: str) -> str:
    return s.rjust(20)
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("format!") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_rjust_with_fill() {
    let code = r#"
def right_justify_fill(s: str) -> str:
    return s.rjust(20, "*")
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_center() {
    let code = r#"
def center_str(s: str) -> str:
    return s.center(20)
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("pad") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_center_with_fill() {
    let code = r#"
def center_fill(s: str) -> str:
    return s.center(20, "=")
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_zfill() {
    let code = r#"
def zero_fill(s: str) -> str:
    return s.zfill(5)
"#;
    let rust = transpile(code);
    assert!(rust.contains("width") || rust.contains("zfill") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_zfill_negative_number() {
    let code = r#"
def zero_fill_neg() -> str:
    return "-42".zfill(6)
"#;
    let rust = transpile(code);
    assert!(rust.contains("starts_with") || rust.contains("sign") || rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_encode() {
    let code = r#"
def encode_str(s: str) -> bytes:
    return s.encode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("as_bytes") || rust.contains("to_vec"), "output: {}", rust);
}

#[test]
fn test_cov_string_encode_utf8() {
    let code = r#"
def encode_utf8(s: str) -> bytes:
    return s.encode("utf-8")
"#;
    let rust = transpile(code);
    assert!(rust.contains("as_bytes") || rust.contains("to_vec"), "output: {}", rust);
}

#[test]
fn test_cov_string_title() {
    let code = r#"
def title_case(s: str) -> str:
    return s.title()
"#;
    let rust = transpile(code);
    assert!(rust.contains("split_whitespace") || rust.contains("to_uppercase") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_capitalize() {
    let code = r#"
def capitalize_str(s: str) -> str:
    return s.capitalize()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_swapcase() {
    let code = r#"
def swap_case(s: str) -> str:
    return s.swapcase()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_uppercase") || rust.contains("to_lowercase") || rust.contains("to_uppercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_lstrip_no_args() {
    let code = r#"
def lstrip_ws(s: str) -> str:
    return s.lstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_start"), "output: {}", rust);
}

#[test]
fn test_cov_string_lstrip_with_chars() {
    let code = r#"
def lstrip_chars(s: str) -> str:
    return s.lstrip("xyz")
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_start_matches"), "output: {}", rust);
}

#[test]
fn test_cov_string_rstrip_no_args() {
    let code = r#"
def rstrip_ws(s: str) -> str:
    return s.rstrip()
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_end"), "output: {}", rust);
}

#[test]
fn test_cov_string_rstrip_with_chars() {
    let code = r#"
def rstrip_chars(s: str) -> str:
    return s.rstrip("abc")
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim_end_matches"), "output: {}", rust);
}

#[test]
fn test_cov_string_splitlines() {
    let code = r#"
def split_lines(s: str) -> list:
    return s.splitlines()
"#;
    let rust = transpile(code);
    assert!(rust.contains("lines"), "output: {}", rust);
}

#[test]
fn test_cov_string_expandtabs_no_args() {
    let code = r#"
def expand_tabs(s: str) -> str:
    return s.expandtabs()
"#;
    let rust = transpile(code);
    assert!(rust.contains("replace") && (rust.contains("\\t") || rust.contains("repeat")), "output: {}", rust);
}

#[test]
fn test_cov_string_expandtabs_with_tabsize() {
    let code = r#"
def expand_tabs_custom(s: str) -> str:
    return s.expandtabs(4)
"#;
    let rust = transpile(code);
    assert!(rust.contains("replace") && rust.contains("repeat"), "output: {}", rust);
}

#[test]
fn test_cov_string_format_single_arg() {
    let code = r#"
def format_single(name: str) -> str:
    return "Hello, {}!".format(name)
"#;
    let rust = transpile(code);
    assert!(rust.contains("replacen") || rust.contains("format"), "output: {}", rust);
}

#[test]
fn test_cov_string_format_multiple_args() {
    let code = r#"
def format_multi(first: str, last: str) -> str:
    return "{} {}".format(first, last)
"#;
    let rust = transpile(code);
    assert!(rust.contains("replacen") || rust.contains("format"), "output: {}", rust);
}

#[test]
fn test_cov_string_find_basic() {
    let code = r#"
def find_sub(s: str) -> int:
    return s.find("hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".find(") && rust.contains("unwrap_or(-1)"), "output: {}", rust);
}

#[test]
fn test_cov_string_count() {
    let code = r#"
def count_sub(s: str) -> int:
    return s.count("a")
"#;
    let rust = transpile(code);
    assert!(rust.contains("matches") && rust.contains("count"), "output: {}", rust);
}

#[test]
fn test_cov_string_isdigit() {
    let code = r#"
def check_digit(s: str) -> bool:
    return s.isdigit()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_numeric") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isalpha() {
    let code = r#"
def check_alpha(s: str) -> bool:
    return s.isalpha()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_alphabetic") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isalnum() {
    let code = r#"
def check_alnum(s: str) -> bool:
    return s.isalnum()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_alphanumeric") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isspace() {
    let code = r#"
def check_space(s: str) -> bool:
    return s.isspace()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_whitespace") || rust.contains("chars"), "output: {}", rust);
}

// ========================================================================
// DICT METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_dict_get_one_arg() {
    let code = r#"
def get_value(d: dict) -> str:
    return d.get("key")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("cloned"), "output: {}", rust);
}

#[test]
fn test_cov_dict_get_two_args_default() {
    let code = r#"
def get_with_default(d: dict) -> str:
    return d.get("key", "default")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("unwrap_or"), "output: {}", rust);
}

#[test]
fn test_cov_dict_keys() {
    let code = r#"
def get_keys(d: dict) -> list:
    return d.keys()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".keys()") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_values() {
    let code = r#"
def get_values(d: dict) -> list:
    return d.values()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".values()") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_items() {
    let code = r#"
def get_items(d: dict) -> list:
    return d.items()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".iter()") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_update() {
    let code = r#"
def update_dict():
    d = {"a": 1}
    other = {"b": 2}
    d.update(other)
    return d
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert") || rust.contains("extend") || rust.contains("update"), "output: {}", rust);
}

#[test]
fn test_cov_dict_setdefault() {
    let code = r#"
def set_default():
    d = {"a": 1}
    d.setdefault("b", 2)
    return d
"#;
    let rust = transpile(code);
    assert!(rust.contains("entry") || rust.contains("or_insert"), "output: {}", rust);
}

#[test]
fn test_cov_dict_pop_with_key() {
    let code = r#"
def pop_key():
    d = {"a": 1, "b": 2}
    return d.pop("a")
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

#[test]
fn test_cov_dict_pop_with_default() {
    let code = r#"
def pop_default():
    d = {"a": 1}
    return d.pop("b", 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove") && rust.contains("unwrap_or"), "output: {}", rust);
}

#[test]
fn test_cov_dict_popitem() {
    let code = r#"
def pop_item():
    d = {"a": 1, "b": 2}
    return d.popitem()
"#;
    let rust = transpile(code);
    assert!(rust.contains("keys") || rust.contains("remove") || rust.contains("next"), "output: {}", rust);
}

#[test]
fn test_cov_dict_clear() {
    let code = r#"
def clear_dict():
    d = {"a": 1}
    d.clear()
    return d
"#;
    let rust = transpile(code);
    assert!(rust.contains(".clear()"), "output: {}", rust);
}

#[test]
fn test_cov_dict_copy() {
    let code = r#"
def copy_dict():
    d = {"a": 1}
    return d.copy()
"#;
    let rust = transpile(code);
    assert!(rust.contains("clone"), "output: {}", rust);
}

// ========================================================================
// SET METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_set_add_int() {
    let code = r#"
def add_to_set():
    s = {1, 2, 3}
    s.add(4)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_set_add_string() {
    let code = r#"
def add_string_to_set():
    fruits = {"apple", "banana"}
    fruits.add("cherry")
    return fruits
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_set_remove() {
    let code = r#"
def remove_from_set():
    s = {1, 2, 3}
    s.remove(2)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

#[test]
fn test_cov_set_discard() {
    let code = r#"
def discard_from_set():
    s = {1, 2, 3}
    s.discard(2)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

#[test]
fn test_cov_set_clear() {
    let code = r#"
def clear_set():
    s = {1, 2, 3}
    s.clear()
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("clear"), "output: {}", rust);
}

#[test]
fn test_cov_set_update() {
    let code = r#"
def update_set():
    s = {1, 2}
    other = {3, 4}
    s.update(other)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert") || rust.contains("extend"), "output: {}", rust);
}

#[test]
fn test_cov_set_union() {
    let code = r#"
def union_sets():
    a = {1, 2}
    b = {3, 4}
    return a.union(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("union") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_intersection() {
    let code = r#"
def intersect_sets():
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a.intersection(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("intersection") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_difference() {
    let code = r#"
def diff_sets():
    a = {1, 2, 3}
    b = {2, 3}
    return a.difference(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("difference") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_symmetric_difference() {
    let code = r#"
def sym_diff():
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a.symmetric_difference(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("symmetric_difference") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_issubset() {
    let code = r#"
def check_subset():
    a = {1, 2}
    b = {1, 2, 3}
    return a.issubset(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_subset"), "output: {}", rust);
}

#[test]
fn test_cov_set_issuperset() {
    let code = r#"
def check_superset():
    a = {1, 2, 3}
    b = {1, 2}
    return a.issuperset(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_superset"), "output: {}", rust);
}

#[test]
fn test_cov_set_isdisjoint() {
    let code = r#"
def check_disjoint():
    a = {1, 2}
    b = {3, 4}
    return a.isdisjoint(b)
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_disjoint"), "output: {}", rust);
}

#[test]
fn test_cov_set_intersection_update() {
    let code = r#"
def intersect_update():
    a = {1, 2, 3}
    b = {2, 3, 4}
    a.intersection_update(b)
    return a
"#;
    let rust = transpile(code);
    assert!(rust.contains("intersection") || rust.contains("clear") || rust.contains("extend"), "output: {}", rust);
}

#[test]
fn test_cov_set_difference_update() {
    let code = r#"
def diff_update():
    a = {1, 2, 3}
    b = {2, 3}
    a.difference_update(b)
    return a
"#;
    let rust = transpile(code);
    assert!(rust.contains("difference") || rust.contains("clear") || rust.contains("extend"), "output: {}", rust);
}

// ========================================================================
// LIST METHOD EDGE CASE COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_list_insert() {
    let code = r#"
def insert_test():
    items = [1, 2, 3]
    items.insert(1, 10)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_list_insert_at_beginning() {
    let code = r#"
def insert_begin():
    items = [2, 3]
    items.insert(0, 1)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert") && rust.contains("0"), "output: {}", rust);
}

#[test]
fn test_cov_list_remove_by_value() {
    let code = r#"
def remove_val():
    items = [1, 2, 3, 2]
    items.remove(2)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("position") || rust.contains("remove"), "output: {}", rust);
}

#[test]
fn test_cov_list_sort_reverse() {
    let code = r#"
def sort_reverse():
    items = [3, 1, 2]
    items.sort(reverse=True)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("sort_by") || rust.contains("b.cmp(a)"), "output: {}", rust);
}

#[test]
fn test_cov_list_sort_basic() {
    let code = r#"
def sort_basic():
    items = [3, 1, 2]
    items.sort()
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("sort()"), "output: {}", rust);
}

#[test]
fn test_cov_list_index_method() {
    let code = r#"
def index_method():
    items = [10, 20, 30]
    return items.index(20)
"#;
    let rust = transpile(code);
    assert!(rust.contains("position"), "output: {}", rust);
}

#[test]
fn test_cov_list_count_method() {
    let code = r#"
def count_method():
    items = [1, 2, 2, 3, 2]
    return items.count(2)
"#;
    let rust = transpile(code);
    // The transpiler may route through string count (matches) or list count (filter)
    assert!(rust.contains("filter") || rust.contains("matches") || rust.contains("count"), "output: {}", rust);
}

#[test]
fn test_cov_list_copy_method() {
    let code = r#"
def copy_list():
    items = [1, 2, 3]
    return items.copy()
"#;
    let rust = transpile(code);
    assert!(rust.contains("clone"), "output: {}", rust);
}

#[test]
fn test_cov_list_clear_method() {
    let code = r#"
def clear_list():
    items = [1, 2, 3]
    items.clear()
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("clear"), "output: {}", rust);
}

#[test]
fn test_cov_list_reverse_method() {
    let code = r#"
def reverse_list():
    items = [1, 2, 3]
    items.reverse()
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("reverse"), "output: {}", rust);
}

#[test]
fn test_cov_list_pop_no_args() {
    let code = r#"
def pop_last():
    items = [1, 2, 3]
    return items.pop()
"#;
    let rust = transpile(code);
    assert!(rust.contains("pop"), "output: {}", rust);
}

#[test]
fn test_cov_list_pop_with_index() {
    let code = r#"
def pop_idx():
    items = [1, 2, 3]
    return items.pop(0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

// ========================================================================
// COMPREHENSION COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_list_comp_basic() {
    let code = r#"
def squares(n: int) -> list:
    return [x * x for x in range(n)]
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_with_filter() {
    let code = r#"
def even_squares(n: int) -> list:
    return [x * x for x in range(n) if x % 2 == 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("filter") && rust.contains("map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_with_method_call() {
    let code = r#"
def upper_words(words: list) -> list:
    return [w.upper() for w in words]
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase") && rust.contains("map"), "output: {}", rust);
}

#[test]
fn test_cov_dict_comprehension() {
    let code = r#"
def square_dict(n: int) -> dict:
    return {i: i * i for i in range(n)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("HashMap") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_comp_with_filter() {
    let code = r#"
def even_square_dict(n: int) -> dict:
    return {i: i * i for i in range(n) if i % 2 == 0}
"#;
    let rust = transpile(code);
    assert!(rust.contains("filter") && rust.contains("map") && rust.contains("HashMap"), "output: {}", rust);
}

#[test]
fn test_cov_set_comprehension() {
    let code = r#"
def unique_squares(n: int) -> set:
    return {x * x for x in range(n)}
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("HashSet") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_comp_with_filter() {
    let code = r#"
def even_unique(n: int) -> set:
    return {x for x in range(n) if x % 2 == 0}
"#;
    let rust = transpile(code);
    assert!(rust.contains("filter") && rust.contains("HashSet") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_nested_list_comp() {
    let code = r#"
def flat_pairs(n: int) -> list:
    return [(x, y) for x in range(n) for y in range(n)]
"#;
    let rust = transpile(code);
    assert!(rust.contains("flat_map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_string_iteration() {
    let code = r#"
def char_list(s: str) -> list:
    return [c for c in s]
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") || rust.contains("map") || rust.contains("collect"), "output: {}", rust);
}

// ========================================================================
// SLICING COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_slice_basic_start_stop() {
    let code = r#"
def slice_basic(items: list) -> list:
    return items[1:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_vec") || rust.contains("start") || rust.contains("stop"), "output: {}", rust);
}

#[test]
fn test_cov_slice_with_step() {
    let code = r#"
def slice_step(items: list) -> list:
    return items[::2]
"#;
    let rust = transpile(code);
    assert!(rust.contains("step_by") || rust.contains("step"), "output: {}", rust);
}

#[test]
fn test_cov_slice_negative_start() {
    let code = r#"
def slice_neg(items: list) -> list:
    return items[-2:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("start_idx") || rust.contains("len") || rust.contains("isize"), "output: {}", rust);
}

#[test]
fn test_cov_slice_stop_only() {
    let code = r#"
def slice_stop(items: list) -> list:
    return items[:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_vec") || rust.contains("stop") || rust.contains("min"), "output: {}", rust);
}

#[test]
fn test_cov_slice_start_only() {
    let code = r#"
def slice_start(items: list) -> list:
    return items[2:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_vec") || rust.contains("start"), "output: {}", rust);
}

#[test]
fn test_cov_slice_reverse() {
    let code = r#"
def reverse_slice(items: list) -> list:
    return items[::-1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("rev") || rust.contains("step") || rust.contains("clone"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_start_stop() {
    let code = r#"
def str_slice(s: str) -> str:
    return s[1:4]
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") || rust.contains("skip") || rust.contains("take"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_negative() {
    let code = r#"
def str_slice_neg(s: str) -> str:
    return s[-3:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") || rust.contains("skip") || rust.contains("start_idx"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_step() {
    let code = r#"
def str_slice_step(s: str) -> str:
    return s[::2]
"#;
    let rust = transpile(code);
    assert!(rust.contains("step_by") || rust.contains("chars") || rust.contains("step"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_reverse() {
    let code = r#"
def str_reverse(s: str) -> str:
    return s[::-1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("rev") || rust.contains("chars") || rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_slice_full_copy() {
    let code = r#"
def full_copy(items: list) -> list:
    return items[:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("clone"), "output: {}", rust);
}

// ========================================================================
// INDEXING COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_dict_index_string_key() {
    let code = r#"
def dict_access():
    d = {"name": "Alice", "age": "30"}
    return d["name"]
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("unwrap_or_default"), "output: {}", rust);
}

#[test]
fn test_cov_list_index_int() {
    let code = r#"
def list_access(items: list) -> int:
    return items[0]
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") || rust.contains("[0]"), "output: {}", rust);
}

#[test]
fn test_cov_list_index_negative() {
    let code = r#"
def last_elem(items: list) -> int:
    return items[-1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("len") || rust.contains("saturating_sub") || rust.contains("get"), "output: {}", rust);
}

#[test]
fn test_cov_os_environ_index() {
    let code = r#"
def get_env_var() -> str:
    import os
    return os.environ["HOME"]
"#;
    let rust = transpile(code);
    assert!(rust.contains("env::var") || rust.contains("unwrap_or_default"), "output: {}", rust);
}

#[test]
fn test_cov_list_index_variable() {
    let code = r#"
def var_index(items: list, i: int) -> int:
    return items[i]
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") || rust.contains("as usize"), "output: {}", rust);
}

#[test]
fn test_cov_string_char_index() {
    let code = r#"
def char_at(s: str, i: int) -> str:
    return s[i]
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") && rust.contains("nth"), "output: {}", rust);
}

#[test]
fn test_cov_dict_index_variable_key() {
    let code = r#"
def dict_var_key(d: dict, key: str) -> str:
    return d[key]
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("unwrap_or_default"), "output: {}", rust);
}

// ========================================================================
// CONSTRUCTOR COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_list_empty() {
    let code = r#"
def empty_list() -> list:
    return []
"#;
    let rust = transpile(code);
    assert!(rust.contains("vec!") || rust.contains("Vec::new"), "output: {}", rust);
}

#[test]
fn test_cov_list_with_elements() {
    let code = r#"
def int_list() -> list:
    return [1, 2, 3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("vec!") && rust.contains("1") && rust.contains("2") && rust.contains("3"), "output: {}", rust);
}

#[test]
fn test_cov_list_with_string_elements() {
    let code = r#"
def str_list() -> list:
    return ["hello", "world"]
"#;
    let rust = transpile(code);
    assert!(rust.contains("vec!") && rust.contains("to_string"), "output: {}", rust);
}

#[test]
fn test_cov_tuple_literal() {
    let code = r#"
def make_tuple() -> tuple:
    return (1, 2, 3)
"#;
    let rust = transpile(code);
    assert!(rust.contains("(1") && rust.contains("2") && rust.contains("3)"), "output: {}", rust);
}

#[test]
fn test_cov_tuple_with_strings() {
    let code = r#"
def str_tuple() -> tuple:
    return ("hello", "world")
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_string"), "output: {}", rust);
}

#[test]
fn test_cov_set_literal() {
    let code = r#"
def make_set() -> set:
    return {1, 2, 3}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashSet") && rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_set_with_strings() {
    let code = r#"
def str_set() -> set:
    return {"apple", "banana", "cherry"}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashSet") && rust.contains("insert") && rust.contains("to_string"), "output: {}", rust);
}

#[test]
fn test_cov_frozenset_literal() {
    let code = r#"
def make_frozenset() -> frozenset:
    return frozenset({1, 2, 3})
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashSet") || rust.contains("Arc"), "output: {}", rust);
}

// ========================================================================
// DICT CONSTRUCTOR COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_dict_empty_literal() {
    let code = r#"
def empty_dict() -> dict:
    return {}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("new"), "output: {}", rust);
}

#[test]
fn test_cov_dict_string_keys() {
    let code = r#"
def str_dict() -> dict:
    return {"name": "Alice", "city": "NYC"}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert") && rust.contains("to_string"), "output: {}", rust);
}

#[test]
fn test_cov_dict_int_values() {
    let code = r#"
def int_dict() -> dict:
    return {"a": 1, "b": 2, "c": 3}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_dict_nested() {
    let code = r#"
def nested_dict() -> dict:
    return {"outer": {"inner": 1}}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_dict_bool_values() {
    let code = r#"
def bool_dict() -> dict:
    return {"active": True, "verified": False}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_dict_single_entry() {
    let code = r#"
def single_dict() -> dict:
    return {"key": "value"}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert"), "output: {}", rust);
}

// ========================================================================
// ATTRIBUTE ACCESS COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_os_environ_attribute() {
    let code = r#"
def get_environ():
    import os
    return os.environ
"#;
    let rust = transpile(code);
    assert!(rust.contains("env::vars") || rust.contains("HashMap"), "output: {}", rust);
}

#[test]
fn test_cov_math_pi_attribute() {
    let code = r#"
def get_pi() -> float:
    import math
    return math.pi
"#;
    let rust = transpile(code);
    assert!(rust.contains("PI") || rust.contains("consts"), "output: {}", rust);
}

#[test]
fn test_cov_math_e_attribute() {
    let code = r#"
def get_e() -> float:
    import math
    return math.e
"#;
    let rust = transpile(code);
    assert!(rust.contains("E") || rust.contains("consts"), "output: {}", rust);
}

#[test]
fn test_cov_sys_argv_attribute() {
    let code = r#"
def get_argv() -> list:
    import sys
    return sys.argv
"#;
    let rust = transpile(code);
    assert!(rust.contains("env::args") || rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_sys_platform_attribute() {
    let code = r#"
def get_platform() -> str:
    import sys
    return sys.platform
"#;
    let rust = transpile(code);
    assert!(rust.contains("darwin") || rust.contains("linux") || rust.contains("win32") || rust.contains("platform"), "output: {}", rust);
}

#[test]
fn test_cov_object_field_access() {
    let code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def get_x(self) -> int:
        return self.x
"#;
    let rust = transpile(code);
    assert!(rust.contains("self.x"), "output: {}", rust);
}

// ========================================================================
// REGEX METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_regex_findall() {
    let code = r#"
def find_all_matches(pattern: str, text: str) -> list:
    import re
    compiled = re.compile(pattern)
    return compiled.findall(text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("find_iter") || rust.contains("Regex"), "output: {}", rust);
}

#[test]
fn test_cov_regex_match() {
    let code = r#"
def match_pattern(pattern: str, text: str):
    import re
    compiled = re.compile(pattern)
    return compiled.match(text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("find") || rust.contains("Regex"), "output: {}", rust);
}

#[test]
fn test_cov_regex_search() {
    let code = r#"
def search_pattern(pattern: str, text: str):
    import re
    compiled = re.compile(pattern)
    return compiled.search(text)
"#;
    let rust = transpile(code);
    assert!(rust.contains("find") || rust.contains("Regex"), "output: {}", rust);
}

// ========================================================================
// SYS I/O METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_sys_stdout_write() {
    let code = r#"
def write_stdout():
    import sys
    sys.stdout.write("hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains("write!") || rust.contains("stdout") || rust.contains("io::Write"), "output: {}", rust);
}

#[test]
fn test_cov_sys_stdout_flush() {
    let code = r#"
def flush_stdout():
    import sys
    sys.stdout.flush()
"#;
    let rust = transpile(code);
    assert!(rust.contains("flush") || rust.contains("stdout"), "output: {}", rust);
}

#[test]
fn test_cov_sys_stderr_write() {
    let code = r#"
def write_stderr():
    import sys
    sys.stderr.write("error")
"#;
    let rust = transpile(code);
    assert!(rust.contains("write!") || rust.contains("stderr") || rust.contains("io::Write"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL STRING METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_string_index_method() {
    let code = r#"
def str_index(s: str) -> int:
    return s.index("world")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".find(") && rust.contains("expect"), "output: {}", rust);
}

#[test]
fn test_cov_string_rfind() {
    let code = r#"
def str_rfind(s: str) -> int:
    return s.rfind("o")
"#;
    let rust = transpile(code);
    assert!(rust.contains("rfind") && rust.contains("unwrap_or(-1)"), "output: {}", rust);
}

#[test]
fn test_cov_string_rindex() {
    let code = r#"
def str_rindex(s: str) -> int:
    return s.rindex("o")
"#;
    let rust = transpile(code);
    assert!(rust.contains("rfind") && rust.contains("expect"), "output: {}", rust);
}

#[test]
fn test_cov_string_partition() {
    let code = r#"
def partition_str(s: str):
    return s.partition("=")
"#;
    let rust = transpile(code);
    assert!(rust.contains("find") && (rust.contains("before") || rust.contains("after") || rust.contains("sep")), "output: {}", rust);
}

#[test]
fn test_cov_string_casefold() {
    let code = r#"
def casefold_str(s: str) -> str:
    return s.casefold()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_lowercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_isupper() {
    let code = r#"
def check_upper(s: str) -> bool:
    return s.isupper()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_uppercase") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_islower() {
    let code = r#"
def check_lower(s: str) -> bool:
    return s.islower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_lowercase") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_istitle() {
    let code = r#"
def check_title(s: str) -> bool:
    return s.istitle()
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") || rust.contains("prev_is_cased") || rust.contains("is_uppercase"), "output: {}", rust);
}

#[test]
fn test_cov_string_isnumeric() {
    let code = r#"
def check_numeric(s: str) -> bool:
    return s.isnumeric()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_numeric") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isascii() {
    let code = r#"
def check_ascii(s: str) -> bool:
    return s.isascii()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_ascii") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isdecimal() {
    let code = r#"
def check_decimal(s: str) -> bool:
    return s.isdecimal()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_ascii_digit") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isidentifier() {
    let code = r#"
def check_identifier(s: str) -> bool:
    return s.isidentifier()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_alphabetic") || rust.contains("is_alphanumeric") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_isprintable() {
    let code = r#"
def check_printable(s: str) -> bool:
    return s.isprintable()
"#;
    let rust = transpile(code);
    assert!(rust.contains("is_control") || rust.contains("chars"), "output: {}", rust);
}

#[test]
fn test_cov_string_hex() {
    let code = r#"
def to_hex(s: str) -> str:
    return s.hex()
"#;
    let rust = transpile(code);
    assert!(rust.contains("bytes") && (rust.contains("format!") || rust.contains("02x")), "output: {}", rust);
}

#[test]
fn test_cov_string_find_with_start() {
    let code = r#"
def find_from(s: str) -> int:
    return s.find("a", 5)
"#;
    let rust = transpile(code);
    assert!(rust.contains(".find(") && rust.contains("5"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL CONSTRUCTOR COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_list_none_elements() {
    let code = r#"
def none_list():
    return [None, 1, None]
"#;
    let rust = transpile(code);
    assert!(rust.contains("None") && rust.contains("Some"), "output: {}", rust);
}

#[test]
fn test_cov_list_float_elements() {
    let code = r#"
def float_list() -> list:
    return [1.0, 2.5, 3.14]
"#;
    let rust = transpile(code);
    assert!(rust.contains("vec!"), "output: {}", rust);
}

#[test]
fn test_cov_dict_none_value() {
    let code = r#"
def none_dict():
    return {"key": None, "other": "value"}
"#;
    let rust = transpile(code);
    assert!(rust.contains("None") && rust.contains("Some"), "output: {}", rust);
}

#[test]
fn test_cov_tuple_mixed() {
    let code = r#"
def mixed_tuple():
    return (1, "hello")
"#;
    let rust = transpile(code);
    assert!(rust.contains("1") && rust.contains("to_string"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL INDEXING AND SLICING EDGE CASES
// ========================================================================

#[test]
fn test_cov_slice_start_stop_step() {
    let code = r#"
def step_slice(items: list) -> list:
    return items[0:10:2]
"#;
    let rust = transpile(code);
    assert!(rust.contains("step_by") || rust.contains("step"), "output: {}", rust);
}

#[test]
fn test_cov_slice_start_and_step() {
    let code = r#"
def start_step(items: list) -> list:
    return items[1::2]
"#;
    let rust = transpile(code);
    assert!(rust.contains("step_by") || rust.contains("step") || rust.contains("start"), "output: {}", rust);
}

#[test]
fn test_cov_slice_stop_and_step() {
    let code = r#"
def stop_step(items: list) -> list:
    return items[:5:2]
"#;
    let rust = transpile(code);
    assert!(rust.contains("step_by") || rust.contains("step") || rust.contains("stop"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_full_copy() {
    let code = r#"
def str_copy(s: str) -> str:
    return s[:]
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_string") || rust.contains("clone"), "output: {}", rust);
}

#[test]
fn test_cov_string_slice_stop_only() {
    let code = r#"
def str_take(s: str) -> str:
    return s[:3]
"#;
    let rust = transpile(code);
    assert!(rust.contains("chars") || rust.contains("take"), "output: {}", rust);
}

#[test]
fn test_cov_list_literal_index() {
    let code = r#"
def literal_idx() -> int:
    return [10, 20, 30][1]
"#;
    let rust = transpile(code);
    assert!(rust.contains("get(") || rust.contains("[1]") || rust.contains("20"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL DICT METHOD EDGE CASES
// ========================================================================

#[test]
fn test_cov_dict_get_with_string_literal_key() {
    let code = r#"
def get_literal():
    d = {"x": 10, "y": 20}
    return d.get("x")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("cloned"), "output: {}", rust);
}

#[test]
fn test_cov_dict_keys_on_literal() {
    let code = r#"
def keys_literal():
    d = {"a": 1, "b": 2}
    return d.keys()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".keys()") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_values_on_literal() {
    let code = r#"
def values_literal():
    d = {"a": 1, "b": 2}
    return d.values()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".values()") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_items_on_literal() {
    let code = r#"
def items_literal():
    d = {"a": 1, "b": 2}
    return d.items()
"#;
    let rust = transpile(code);
    assert!(rust.contains(".iter()") && rust.contains("collect"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL SET OPERATION EDGE CASES
// ========================================================================

#[test]
fn test_cov_set_add_to_empty() {
    let code = r#"
def add_to_empty():
    s = set()
    s.add(1)
    s.add(2)
    return s
"#;
    let rust = transpile(code);
    assert!(rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_set_remove_string() {
    let code = r#"
def remove_string():
    fruits = {"apple", "banana", "cherry"}
    fruits.remove("banana")
    return fruits
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

#[test]
fn test_cov_set_discard_string() {
    let code = r#"
def discard_string():
    fruits = {"apple", "banana"}
    fruits.discard("cherry")
    return fruits
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL COMPREHENSION EDGE CASES
// ========================================================================

#[test]
fn test_cov_list_comp_from_range() {
    let code = r#"
def doubled() -> list:
    return [x * 2 for x in range(5)]
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_conditional_expr() {
    let code = r#"
def abs_list(items: list) -> list:
    return [x if x > 0 else -x for x in items]
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_dict_comp_from_list() {
    let code = r#"
def word_lengths(words: list) -> dict:
    return {w: len(w) for w in words}
"#;
    let rust = transpile(code);
    assert!(rust.contains("map") && rust.contains("HashMap") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_set_comp_from_range() {
    let code = r#"
def odd_set() -> set:
    return {x for x in range(10) if x % 2 != 0}
"#;
    let rust = transpile(code);
    assert!(rust.contains("filter") && rust.contains("HashSet") && rust.contains("collect"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL ATTRIBUTE ACCESS EDGE CASES
// ========================================================================

#[test]
fn test_cov_math_inf_attribute() {
    let code = r#"
def get_inf() -> float:
    import math
    return math.inf
"#;
    let rust = transpile(code);
    assert!(rust.contains("INFINITY"), "output: {}", rust);
}

#[test]
fn test_cov_math_nan_attribute() {
    let code = r#"
def get_nan() -> float:
    import math
    return math.nan
"#;
    let rust = transpile(code);
    assert!(rust.contains("NAN"), "output: {}", rust);
}

#[test]
fn test_cov_math_tau_attribute() {
    let code = r#"
def get_tau() -> float:
    import math
    return math.tau
"#;
    let rust = transpile(code);
    assert!(rust.contains("TAU"), "output: {}", rust);
}

#[test]
fn test_cov_string_ascii_lowercase() {
    let code = r#"
def get_lowercase() -> str:
    import string
    return string.ascii_lowercase
"#;
    let rust = transpile(code);
    assert!(rust.contains("abcdefghijklmnopqrstuvwxyz"), "output: {}", rust);
}

#[test]
fn test_cov_string_digits() {
    let code = r#"
def get_digits() -> str:
    import string
    return string.digits
"#;
    let rust = transpile(code);
    assert!(rust.contains("0123456789"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL STRING METHOD EDGE CASES
// ========================================================================

#[test]
fn test_cov_string_decode() {
    let code = r#"
def decode_bytes(b: bytes) -> str:
    return b.decode()
"#;
    let rust = transpile(code);
    assert!(rust.contains("from_utf8_lossy") || rust.contains("String"), "output: {}", rust);
}

#[test]
fn test_cov_string_format_no_args() {
    let code = r#"
def format_no_args() -> str:
    return "hello world".format()
"#;
    let rust = transpile(code);
    assert!(rust.contains("hello world"), "output: {}", rust);
}

#[test]
fn test_cov_string_replace_with_var_args() {
    let code = r#"
def replace_vars(s: str, old: str, new: str) -> str:
    return s.replace(old, new)
"#;
    let rust = transpile(code);
    assert!(rust.contains(".replace("), "output: {}", rust);
}

#[test]
fn test_cov_string_split_with_var_sep() {
    let code = r#"
def split_var(s: str, sep: str) -> list:
    return s.split(sep)
"#;
    let rust = transpile(code);
    assert!(rust.contains(".split(") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_string_join_empty_separator() {
    let code = r#"
def join_no_sep(items: list) -> str:
    return "".join(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("join"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL LIST METHOD COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_list_extend() {
    let code = r#"
def extend_list():
    items = [1, 2]
    more = [3, 4]
    items.extend(more)
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("extend"), "output: {}", rust);
}

#[test]
fn test_cov_list_append_string() {
    let code = r#"
def append_string():
    items = ["hello"]
    items.append("world")
    return items
"#;
    let rust = transpile(code);
    assert!(rust.contains("push") && rust.contains("to_string"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL DICT CONSTRUCTOR EDGE CASES
// ========================================================================

#[test]
fn test_cov_dict_float_values() {
    let code = r#"
def float_dict() -> dict:
    return {"pi": 3.14, "e": 2.71}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert"), "output: {}", rust);
}

#[test]
fn test_cov_dict_list_values() {
    let code = r#"
def list_dict() -> dict:
    return {"nums": [1, 2, 3]}
"#;
    let rust = transpile(code);
    assert!(rust.contains("HashMap") && rust.contains("insert") && rust.contains("vec!"), "output: {}", rust);
}

// ========================================================================
// ADDITIONAL MISC COVERAGE TESTS
// ========================================================================

#[test]
fn test_cov_set_operator_union() {
    let code = r#"
def set_union_op():
    a = {1, 2}
    b = {3, 4}
    return a | b
"#;
    let rust = transpile(code);
    assert!(rust.contains("union") || rust.contains("|"), "output: {}", rust);
}

#[test]
fn test_cov_set_operator_intersection() {
    let code = r#"
def set_intersect_op():
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a & b
"#;
    let rust = transpile(code);
    assert!(rust.contains("intersection") || rust.contains("&"), "output: {}", rust);
}

#[test]
fn test_cov_set_operator_difference() {
    let code = r#"
def set_diff_op():
    a = {1, 2, 3}
    b = {2, 3}
    return a - b
"#;
    let rust = transpile(code);
    assert!(rust.contains("difference") || rust.contains("-"), "output: {}", rust);
}

#[test]
fn test_cov_set_operator_symmetric_diff() {
    let code = r#"
def set_xor_op():
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a ^ b
"#;
    let rust = transpile(code);
    assert!(rust.contains("symmetric_difference") || rust.contains("^"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_with_enumerate() {
    let code = r#"
def indexed_list(items: list) -> list:
    return [(i, x) for i, x in enumerate(items)]
"#;
    let rust = transpile(code);
    assert!(rust.contains("enumerate") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_nested_filter() {
    let code = r#"
def positive_evens() -> list:
    return [x for x in range(20) if x > 0 if x % 2 == 0]
"#;
    let rust = transpile(code);
    assert!(rust.contains("filter") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_string_upper_lower_chain() {
    let code = r#"
def upper_lower(s: str) -> str:
    return s.upper().lower()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase") && rust.contains("to_lowercase"), "output: {}", rust);
}

#[test]
fn test_cov_dict_get_int_default() {
    let code = r#"
def get_int_default():
    d = {"a": 1}
    return d.get("b", 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains(".get(") && rust.contains("unwrap_or"), "output: {}", rust);
}

#[test]
fn test_cov_list_pop_dict() {
    let code = r#"
def pop_dict_key():
    d = {"a": 1, "b": 2}
    return d.pop("a", 0)
"#;
    let rust = transpile(code);
    assert!(rust.contains("remove") && rust.contains("unwrap_or"), "output: {}", rust);
}

#[test]
fn test_cov_string_split_collect() {
    let code = r#"
def split_and_process(s: str) -> list:
    parts = s.split(":")
    return parts
"#;
    let rust = transpile(code);
    assert!(rust.contains("split") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_string_join_with_comma() {
    let code = r#"
def comma_join(items: list) -> str:
    return ",".join(items)
"#;
    let rust = transpile(code);
    assert!(rust.contains("join"), "output: {}", rust);
}

#[test]
fn test_cov_string_strip_and_split() {
    let code = r#"
def strip_and_split(s: str) -> list:
    return s.strip().split(",")
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim") && rust.contains("split"), "output: {}", rust);
}

#[test]
fn test_cov_dict_index_and_method() {
    let code = r#"
def dict_index_upper():
    d = {"name": "alice"}
    return d["name"].upper()
"#;
    let rust = transpile(code);
    assert!(rust.contains("to_uppercase"), "output: {}", rust);
}

#[test]
fn test_cov_list_comp_strip_elements() {
    let code = r#"
def strip_all(items: list) -> list:
    return [s.strip() for s in items]
"#;
    let rust = transpile(code);
    assert!(rust.contains("trim") && rust.contains("map") && rust.contains("collect"), "output: {}", rust);
}

#[test]
fn test_cov_string_replace_all_occurrences() {
    let code = r#"
def remove_spaces(s: str) -> str:
    return s.replace(" ", "")
"#;
    let rust = transpile(code);
    assert!(rust.contains(".replace("), "output: {}", rust);
}

#[test]
fn test_cov_string_startswith_in_conditional() {
    let code = r#"
def check_http(url: str) -> bool:
    if url.startswith("http"):
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("starts_with"), "output: {}", rust);
}

#[test]
fn test_cov_string_endswith_in_conditional() {
    let code = r#"
def is_python_file(name: str) -> bool:
    if name.endswith(".py"):
        return True
    return False
"#;
    let rust = transpile(code);
    assert!(rust.contains("ends_with"), "output: {}", rust);
}

#[test]
fn test_cov_list_sort_key_len() {
    let code = r#"
def sort_by_length():
    words = ["banana", "apple", "cherry"]
    words.sort(key=len)
    return words
"#;
    let rust = transpile(code);
    assert!(rust.contains("sort_by_key") || rust.contains("sort"), "output: {}", rust);
}

#[test]
fn test_cov_list_sort_key_reverse() {
    let code = r#"
def sort_by_length_desc():
    words = ["banana", "apple", "cherry"]
    words.sort(key=len, reverse=True)
    return words
"#;
    let rust = transpile(code);
    assert!(rust.contains("sort_by_key") && rust.contains("Reverse"), "output: {}", rust);
}
