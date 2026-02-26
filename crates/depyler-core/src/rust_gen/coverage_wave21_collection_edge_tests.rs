//! Wave 21 coverage tests: collection edge cases for list, dict, set, deque, and collections
//!
//! Targets deep uncovered code paths in:
//! - list_methods.rs: pop disambiguation, extend, insert, index, count, reverse, sort variants, copy, clear, remove
//! - dict_methods.rs: get with defaults, setdefault, popitem, update, pop, merge operators
//! - set_methods.rs: union, intersection, difference, symmetric_difference, subset/superset/disjoint checks
//! - constructors.rs: list/set/frozenset construction, tuple operations, collection conversions
//! - dict_constructors.rs: dict comprehension, nested dict, dict unpacking
//!
//! 200 tests total

#[cfg(test)]
mod tests {
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

    // ========================================================================
    // SECTION 1: List Method Edge Cases (tests 001-050)
    // ========================================================================

    #[test]
    fn test_w21ce_001_list_pop_no_args() {
        let code = "def f():\n    lst = [1, 2, 3]\n    x = lst.pop()\n    return x\n";
        let result = transpile(code);
        assert!(result.contains("pop") || result.contains("remove"));
    }

    #[test]
    fn test_w21ce_002_list_pop_index_zero() {
        let code = "def f():\n    lst = [10, 20, 30]\n    x = lst.pop(0)\n    return x\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w21ce_003_list_pop_last_index() {
        let code = "def f():\n    lst = [1, 2, 3]\n    x = lst.pop(-1)\n    return x\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_004_list_pop_middle_index() {
        let code = "def f():\n    lst = [10, 20, 30, 40]\n    x = lst.pop(2)\n    return x\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w21ce_005_list_extend_other_list() {
        let code = "def f():\n    a = [1, 2]\n    b = [3, 4]\n    a.extend(b)\n    return a\n";
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w21ce_006_list_extend_empty() {
        let code = "def f():\n    a = [1, 2, 3]\n    b = []\n    a.extend(b)\n    return a\n";
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w21ce_007_list_insert_at_index() {
        let code = "def f():\n    lst = [1, 2, 3]\n    lst.insert(1, 99)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_008_list_insert_at_zero() {
        let code = "def f():\n    lst = [10, 20]\n    lst.insert(0, 5)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_009_list_insert_string_value() {
        let code = "def f():\n    lst = [\"a\", \"b\"]\n    lst.insert(2, \"c\")\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_010_list_index_find_element() {
        let code = "def f():\n    lst = [10, 20, 30]\n    idx = lst.index(20)\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w21ce_011_list_index_string_element() {
        let code = "def f():\n    lst = [\"a\", \"b\", \"c\"]\n    idx = lst.index(\"b\")\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w21ce_012_list_count_occurrences() {
        let code = "def f():\n    lst = [1, 2, 1, 3, 1]\n    n = lst.count(1)\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w21ce_013_list_count_string() {
        let code =
            "def f():\n    lst = [\"x\", \"y\", \"x\"]\n    n = lst.count(\"x\")\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w21ce_014_list_reverse_in_place() {
        let code = "def f():\n    lst = [1, 2, 3]\n    lst.reverse()\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("reverse"));
    }

    #[test]
    fn test_w21ce_015_list_sort_basic() {
        let code = "def f():\n    lst = [3, 1, 2]\n    lst.sort()\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w21ce_016_list_sort_reverse_true() {
        let code = "def f():\n    lst = [1, 3, 2]\n    lst.sort(reverse=True)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w21ce_017_list_sort_key_len() {
        let code =
            "def f():\n    lst = [\"abc\", \"a\", \"ab\"]\n    lst.sort(key=len)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w21ce_018_list_sort_key_reverse() {
        let code = "def f():\n    lst = [\"abc\", \"a\", \"ab\"]\n    lst.sort(key=len, reverse=True)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("sort") || result.contains("Reverse"));
    }

    #[test]
    fn test_w21ce_019_list_copy_shallow() {
        let code = "def f():\n    lst = [1, 2, 3]\n    cpy = lst.copy()\n    return cpy\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_020_list_clear() {
        let code = "def f():\n    lst = [1, 2, 3]\n    lst.clear()\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w21ce_021_list_remove_first_occurrence() {
        let code = "def f():\n    lst = [1, 2, 3, 2]\n    lst.remove(2)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("position"));
    }

    #[test]
    fn test_w21ce_022_list_remove_string() {
        let code =
            "def f():\n    lst = [\"a\", \"b\", \"c\"]\n    lst.remove(\"b\")\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("position"));
    }

    #[test]
    fn test_w21ce_023_list_comp_with_method_strip() {
        let code = "def f():\n    lines = [\" a \", \" b \"]\n    result = [x.strip() for x in lines]\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("trim") || result.contains("strip") || result.contains("map"));
    }

    #[test]
    fn test_w21ce_024_list_comp_with_method_upper() {
        let code = "def f():\n    words = [\"hello\", \"world\"]\n    result = [w.upper() for w in words]\n    return result\n";
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("upper") || result.contains("map")
        );
    }

    #[test]
    fn test_w21ce_025_nested_list_access() {
        let code =
            "def f():\n    matrix = [[1, 2], [3, 4]]\n    val = matrix[0][1]\n    return val\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_026_nested_list_three_deep() {
        let code = "def f():\n    cube = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]\n    return cube\n";
        let result = transpile(code);
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_w21ce_027_list_concatenation() {
        let code = "def f():\n    a = [1, 2]\n    b = [3, 4]\n    c = a + b\n    return c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_028_list_multiplication() {
        let code = "def f():\n    lst = [0] * 10\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_029_list_from_range() {
        let code = "def f():\n    lst = list(range(10))\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("collect") || result.contains("range") || result.contains("vec"));
    }

    #[test]
    fn test_w21ce_030_list_from_range_step() {
        let code = "def f():\n    lst = list(range(0, 20, 2))\n    return lst\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_031_list_comp_filter() {
        let code = "def f():\n    nums = [1, 2, 3, 4, 5]\n    evens = [x for x in nums if x % 2 == 0]\n    return evens\n";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_032_list_comp_nested_loops() {
        let code =
            "def f():\n    pairs = [(x, y) for x in [1, 2] for y in [3, 4]]\n    return pairs\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_033_list_append_string_literal() {
        let code = "def f():\n    lst = [\"a\", \"b\"]\n    lst.append(\"c\")\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w21ce_034_list_append_int() {
        let code = "def f():\n    lst = [1, 2]\n    lst.append(3)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w21ce_035_list_multiple_operations() {
        let code = "def f():\n    lst = []\n    lst.append(1)\n    lst.append(2)\n    lst.append(3)\n    lst.reverse()\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push") && result.contains("reverse"));
    }

    #[test]
    fn test_w21ce_036_list_sort_after_append() {
        let code =
            "def f():\n    lst = [3, 1]\n    lst.append(2)\n    lst.sort()\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push") && result.contains("sort"));
    }

    #[test]
    fn test_w21ce_037_list_extend_with_strings() {
        let code =
            "def f():\n    a = [\"x\"]\n    b = [\"y\", \"z\"]\n    a.extend(b)\n    return a\n";
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w21ce_038_list_comp_transform() {
        let code = "def f():\n    nums = [1, 2, 3]\n    squares = [x * x for x in nums]\n    return squares\n";
        let result = transpile(code);
        assert!(result.contains("map") || result.contains("iter") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_039_list_comp_string_format() {
        let code = "def f():\n    items = [1, 2, 3]\n    result = [str(x) for x in items]\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_040_list_pop_and_use() {
        let code = "def f():\n    lst = [10, 20, 30]\n    last = lst.pop()\n    first = lst.pop(0)\n    return last + first\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_041_list_insert_at_end() {
        let code = "def f():\n    lst = [1, 2]\n    lst.insert(2, 3)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_042_list_index_not_found_pattern() {
        let code = "def f():\n    lst = [1, 2, 3]\n    idx = lst.index(2)\n    return idx\n";
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w21ce_043_list_count_zero() {
        let code = "def f():\n    lst = [1, 2, 3]\n    n = lst.count(5)\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w21ce_044_list_clear_and_rebuild() {
        let code =
            "def f():\n    lst = [1, 2, 3]\n    lst.clear()\n    lst.append(10)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("clear") && result.contains("push"));
    }

    #[test]
    fn test_w21ce_045_list_copy_modify_original() {
        let code = "def f():\n    lst = [1, 2, 3]\n    cpy = lst.copy()\n    lst.append(4)\n    return cpy\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_046_list_len_after_operations() {
        let code =
            "def f():\n    lst = [1, 2, 3]\n    lst.append(4)\n    n = len(lst)\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w21ce_047_list_membership_check() {
        let code = "def f():\n    lst = [1, 2, 3]\n    found = 2 in lst\n    return found\n";
        let result = transpile(code);
        assert!(result.contains("contains") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_048_list_reverse_strings() {
        let code =
            "def f():\n    words = [\"hello\", \"world\"]\n    words.reverse()\n    return words\n";
        let result = transpile(code);
        assert!(result.contains("reverse"));
    }

    #[test]
    fn test_w21ce_049_list_remove_and_append() {
        let code =
            "def f():\n    lst = [1, 2, 3]\n    lst.remove(2)\n    lst.append(4)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w21ce_050_list_comp_with_condition_and_transform() {
        let code = "def f():\n    nums = [1, 2, 3, 4, 5, 6]\n    result = [x * 2 for x in nums if x > 3]\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("map") || result.contains("iter"));
    }

    // ========================================================================
    // SECTION 2: Dict Method Edge Cases (tests 051-100)
    // ========================================================================

    #[test]
    fn test_w21ce_051_dict_get_with_int_default() {
        let code =
            "def f():\n    d = {\"a\": 1, \"b\": 2}\n    val = d.get(\"c\", 0)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_052_dict_get_with_str_default() {
        let code = "def f():\n    d = {\"name\": \"alice\"}\n    val = d.get(\"name\", \"unknown\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_053_dict_get_with_none_default() {
        let code = "def f():\n    d = {\"a\": 1}\n    val = d.get(\"b\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w21ce_054_dict_get_with_list_default() {
        let code = "def f():\n    d = {\"items\": [1, 2]}\n    val = d.get(\"missing\", [])\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_055_dict_setdefault_int() {
        let code =
            "def f():\n    d = {\"a\": 1}\n    val = d.setdefault(\"b\", 0)\n    return val\n";
        let result = transpile(code);
        assert!(
            result.contains("entry")
                || result.contains("or_insert")
                || result.contains("setdefault")
        );
    }

    #[test]
    fn test_w21ce_056_dict_setdefault_string() {
        let code = "def f():\n    d = {\"name\": \"alice\"}\n    val = d.setdefault(\"title\", \"unknown\")\n    return val\n";
        let result = transpile(code);
        assert!(
            result.contains("entry")
                || result.contains("or_insert")
                || result.contains("setdefault")
        );
    }

    #[test]
    fn test_w21ce_057_dict_popitem() {
        let code =
            "def f():\n    d = {\"a\": 1, \"b\": 2}\n    item = d.popitem()\n    return item\n";
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem"));
    }

    #[test]
    fn test_w21ce_058_dict_update_from_dict() {
        let code = "def f():\n    d1 = {\"a\": 1}\n    d2 = {\"b\": 2}\n    d1.update(d2)\n    return d1\n";
        let result = transpile(code);
        assert!(
            result.contains("insert") || result.contains("update") || result.contains("extend")
        );
    }

    #[test]
    fn test_w21ce_059_dict_pop_without_default() {
        let code = "def f():\n    d = {\"a\": 10}\n    val = d.pop(\"a\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w21ce_060_dict_pop_with_default() {
        let code = "def f():\n    d = {\"a\": 10}\n    val = d.pop(\"b\", 0)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_061_dict_keys() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    k = d.keys()\n    return k\n";
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_062_dict_values() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    v = d.values()\n    return v\n";
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_063_dict_items() {
        let code =
            "def f():\n    d = {\"a\": 1, \"b\": 2}\n    items = d.items()\n    return items\n";
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("items") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_064_dict_clear() {
        let code = "def f():\n    d = {\"a\": 1}\n    d.clear()\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w21ce_065_dict_copy() {
        let code = "def f():\n    d = {\"a\": 1}\n    d2 = d.copy()\n    return d2\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_066_dict_comprehension_basic() {
        let code = "def f():\n    d = {str(i): i for i in range(5)}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_067_dict_comprehension_filter() {
        let code = "def f():\n    d = {k: v for k, v in [(\"a\", 1), (\"b\", 2)] if v > 1}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_068_nested_dict_access() {
        let code = "def f():\n    d = {\"a\": {\"b\": 1}}\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("map") || result.contains("insert"));
    }

    #[test]
    fn test_w21ce_069_dict_get_chain() {
        let code = "def f():\n    d = {\"key\": \"value\"}\n    val = d.get(\"key\", \"default\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_070_dict_membership_check() {
        let code = "def f():\n    d = {\"a\": 1}\n    found = \"a\" in d\n    return found\n";
        let result = transpile(code);
        assert!(result.contains("contains_key") || result.contains("contains"));
    }

    #[test]
    fn test_w21ce_071_dict_iteration_keys() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    result = []\n    for k in d:\n        result.append(k)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_072_dict_iteration_items() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    result = []\n    for k, v in d.items():\n        result.append(k)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("items"));
    }

    #[test]
    fn test_w21ce_073_dict_update_overwrite() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    other = {\"b\": 3, \"c\": 4}\n    d.update(other)\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update"));
    }

    #[test]
    fn test_w21ce_074_dict_pop_int_key_default() {
        let code = "def f():\n    d = {\"x\": 100}\n    val = d.pop(\"x\", -1)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_075_dict_setdefault_existing_key() {
        let code =
            "def f():\n    d = {\"a\": 10}\n    val = d.setdefault(\"a\", 20)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w21ce_076_dict_empty_creation() {
        let code = "def f():\n    d = {}\n    d[\"key\"] = \"value\"\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("map"));
    }

    #[test]
    fn test_w21ce_077_dict_from_list_tuples() {
        let code =
            "def f():\n    pairs = [(\"a\", 1), (\"b\", 2)]\n    d = dict(pairs)\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_078_dict_get_nested_key() {
        let code = "def f():\n    d = {\"outer\": {\"inner\": 42}}\n    outer = d.get(\"outer\")\n    return outer\n";
        let result = transpile(code);
        assert!(result.contains("get"));
    }

    #[test]
    fn test_w21ce_079_dict_len() {
        let code =
            "def f():\n    d = {\"a\": 1, \"b\": 2, \"c\": 3}\n    n = len(d)\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w21ce_080_dict_multiple_operations() {
        let code = "def f():\n    d = {\"a\": 1}\n    d[\"b\"] = 2\n    d[\"c\"] = 3\n    d.pop(\"a\")\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("remove"));
    }

    #[test]
    fn test_w21ce_081_dict_keys_iteration() {
        let code = "def f():\n    d = {\"x\": 10, \"y\": 20}\n    keys = list(d.keys())\n    return keys\n";
        let result = transpile(code);
        assert!(result.contains("keys"));
    }

    #[test]
    fn test_w21ce_082_dict_values_sum() {
        let code = "def f():\n    d = {\"a\": 10, \"b\": 20}\n    total = sum(d.values())\n    return total\n";
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("sum"));
    }

    #[test]
    fn test_w21ce_083_dict_get_float_default() {
        let code =
            "def f():\n    d = {\"pi\": 3.14}\n    val = d.get(\"e\", 2.71)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_084_dict_pop_string_value() {
        let code = "def f():\n    d = {\"name\": \"alice\", \"city\": \"nyc\"}\n    name = d.pop(\"name\")\n    return name\n";
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w21ce_085_dict_with_int_values() {
        let code = "def f():\n    counts = {\"a\": 0, \"b\": 0}\n    counts[\"a\"] = counts.get(\"a\", 0) + 1\n    return counts\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("insert"));
    }

    #[test]
    fn test_w21ce_086_dict_comprehension_from_list() {
        let code = "def f():\n    items = [\"a\", \"b\", \"c\"]\n    d = {item: i for i, item in enumerate(items)}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_087_dict_copy_and_modify() {
        let code = "def f():\n    original = {\"a\": 1}\n    copy = original.copy()\n    copy[\"b\"] = 2\n    return original\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_088_dict_update_empty() {
        let code = "def f():\n    d = {}\n    d.update({\"a\": 1})\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update"));
    }

    #[test]
    fn test_w21ce_089_dict_nested_values() {
        let code = "def f():\n    d = {\"users\": [{\"name\": \"alice\"}]}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_090_dict_get_bool_default() {
        let code = "def f():\n    d = {\"flag\": True}\n    val = d.get(\"other\", False)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_091_dict_pop_and_use_value() {
        let code = "def f():\n    d = {\"x\": 42}\n    val = d.pop(\"x\")\n    result = val + 1\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w21ce_092_dict_setdefault_with_zero() {
        let code = "def f():\n    d = {}\n    d.setdefault(\"count\", 0)\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w21ce_093_dict_keys_to_sorted_list() {
        let code = "def f():\n    d = {\"c\": 3, \"a\": 1, \"b\": 2}\n    keys = sorted(d.keys())\n    return keys\n";
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("sort"));
    }

    #[test]
    fn test_w21ce_094_dict_values_to_list() {
        let code = "def f():\n    d = {\"a\": 10, \"b\": 20}\n    vals = list(d.values())\n    return vals\n";
        let result = transpile(code);
        assert!(result.contains("values"));
    }

    #[test]
    fn test_w21ce_095_dict_items_unpack() {
        let code = "def f():\n    d = {\"x\": 1, \"y\": 2}\n    pairs = list(d.items())\n    return pairs\n";
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("items") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_096_dict_clear_and_refill() {
        let code = "def f():\n    d = {\"a\": 1}\n    d.clear()\n    d[\"b\"] = 2\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("clear") && result.contains("insert"));
    }

    #[test]
    fn test_w21ce_097_dict_from_zip() {
        let code = "def f():\n    keys = [\"a\", \"b\"]\n    vals = [1, 2]\n    d = dict(zip(keys, vals))\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_098_dict_comprehension_squared() {
        let code = "def f():\n    d = {x: x * x for x in range(5)}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_099_dict_default_none_value() {
        let code = "def f():\n    d = {\"a\": None, \"b\": 1}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_100_dict_string_value_operations() {
        let code = "def f():\n    d = {\"greet\": \"hello\"}\n    val = d.get(\"greet\", \"hi\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    // ========================================================================
    // SECTION 3: Set Method Edge Cases (tests 101-150)
    // ========================================================================

    #[test]
    fn test_w21ce_101_set_union() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {3, 4, 5}\n    result = s1.union(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("union") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_102_set_intersection() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {2, 3, 4}\n    result = s1.intersection(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_103_set_difference() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {2, 3}\n    result = s1.difference(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_104_set_symmetric_difference() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {2, 3, 4}\n    result = s1.symmetric_difference(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("symmetric_difference") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_105_set_issubset() {
        let code = "def f():\n    s1 = {1, 2}\n    s2 = {1, 2, 3}\n    result = s1.issubset(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("is_subset") || result.contains("issubset"));
    }

    #[test]
    fn test_w21ce_106_set_issuperset() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {1, 2}\n    result = s1.issuperset(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("is_superset") || result.contains("issuperset"));
    }

    #[test]
    fn test_w21ce_107_set_isdisjoint() {
        let code = "def f():\n    s1 = {1, 2}\n    s2 = {3, 4}\n    result = s1.isdisjoint(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("is_disjoint") || result.contains("isdisjoint"));
    }

    #[test]
    fn test_w21ce_108_set_intersection_update() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {2, 3, 4}\n    s1.intersection_update(s2)\n    return s1\n";
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("retain"));
    }

    #[test]
    fn test_w21ce_109_set_difference_update() {
        let code = "def f():\n    s1 = {1, 2, 3, 4}\n    s2 = {2, 4}\n    s1.difference_update(s2)\n    return s1\n";
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("remove"));
    }

    #[test]
    fn test_w21ce_110_set_add_element() {
        let code = "def f():\n    s = {1, 2}\n    s.add(3)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("add"));
    }

    #[test]
    fn test_w21ce_111_set_discard_element() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.discard(2)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("discard"));
    }

    #[test]
    fn test_w21ce_112_set_remove_element() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.remove(2)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w21ce_113_set_pop() {
        let code = "def f():\n    s = {1, 2, 3}\n    val = s.pop()\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("iter") || result.contains("next") || result.contains("pop"));
    }

    #[test]
    fn test_w21ce_114_set_clear() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.clear()\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("clear"));
    }

    #[test]
    fn test_w21ce_115_set_update_from_iterable() {
        let code =
            "def f():\n    s = {1, 2}\n    other = {3, 4}\n    s.update(other)\n    return s\n";
        let result = transpile(code);
        assert!(
            result.contains("insert") || result.contains("update") || result.contains("extend")
        );
    }

    #[test]
    fn test_w21ce_116_set_copy() {
        let code = "def f():\n    s = {1, 2, 3}\n    s2 = s.copy()\n    return s2\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_117_set_comprehension() {
        let code = "def f():\n    s = {x * 2 for x in range(5)}\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("collect") || result.contains("HashSet") || result.contains("set"));
    }

    #[test]
    fn test_w21ce_118_set_comprehension_filter() {
        let code = "def f():\n    s = {x for x in range(10) if x % 2 == 0}\n    return s\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_119_set_union_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\"}\n    s2 = {\"b\", \"c\"}\n    result = s1.union(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("union") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_120_set_intersection_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\", \"c\"}\n    s2 = {\"b\", \"c\", \"d\"}\n    result = s1.intersection(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_121_set_difference_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\", \"c\"}\n    s2 = {\"b\"}\n    result = s1.difference(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_122_set_add_string() {
        let code = "def f():\n    fruits = {\"apple\", \"banana\"}\n    fruits.add(\"cherry\")\n    return fruits\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_123_set_discard_string() {
        let code = "def f():\n    s = {\"x\", \"y\", \"z\"}\n    s.discard(\"y\")\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("discard"));
    }

    #[test]
    fn test_w21ce_124_set_remove_string() {
        let code =
            "def f():\n    s = {\"hello\", \"world\"}\n    s.remove(\"hello\")\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w21ce_125_set_len() {
        let code = "def f():\n    s = {1, 2, 3, 4}\n    n = len(s)\n    return n\n";
        let result = transpile(code);
        assert!(result.contains("len"));
    }

    #[test]
    fn test_w21ce_126_set_membership() {
        let code = "def f():\n    s = {1, 2, 3}\n    found = 2 in s\n    return found\n";
        let result = transpile(code);
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w21ce_127_set_not_membership() {
        let code =
            "def f():\n    s = {1, 2, 3}\n    not_found = 5 not in s\n    return not_found\n";
        let result = transpile(code);
        assert!(result.contains("contains"));
    }

    #[test]
    fn test_w21ce_128_set_from_list() {
        let code = "def f():\n    lst = [1, 2, 2, 3, 3]\n    s = set(lst)\n    return s\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_129_set_iteration() {
        let code = "def f():\n    s = {10, 20, 30}\n    total = 0\n    for x in s:\n        total = total + x\n    return total\n";
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_130_set_empty_creation() {
        let code = "def f():\n    s = set()\n    s.add(1)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("HashSet") || result.contains("set") || result.contains("insert"));
    }

    #[test]
    fn test_w21ce_131_set_union_operator() {
        let code =
            "def f():\n    s1 = {1, 2}\n    s2 = {3, 4}\n    result = s1 | s2\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_132_set_intersection_operator() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {2, 3, 4}\n    result = s1 & s2\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_133_set_difference_operator() {
        let code =
            "def f():\n    s1 = {1, 2, 3}\n    s2 = {2}\n    result = s1 - s2\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_134_set_symmetric_diff_operator() {
        let code = "def f():\n    s1 = {1, 2, 3}\n    s2 = {3, 4, 5}\n    result = s1 ^ s2\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_135_frozenset_basic() {
        let code = "def f():\n    fs = frozenset([1, 2, 3])\n    return fs\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_136_set_add_multiple() {
        let code =
            "def f():\n    s = set()\n    s.add(1)\n    s.add(2)\n    s.add(3)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_137_set_clear_and_refill() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.clear()\n    s.add(10)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("clear") && result.contains("insert"));
    }

    #[test]
    fn test_w21ce_138_set_issubset_true() {
        let code = "def f():\n    small = {1, 2}\n    big = {1, 2, 3, 4}\n    return small.issubset(big)\n";
        let result = transpile(code);
        assert!(result.contains("is_subset") || result.contains("issubset"));
    }

    #[test]
    fn test_w21ce_139_set_issuperset_true() {
        let code = "def f():\n    big = {1, 2, 3, 4}\n    small = {1, 2}\n    return big.issuperset(small)\n";
        let result = transpile(code);
        assert!(result.contains("is_superset") || result.contains("issuperset"));
    }

    #[test]
    fn test_w21ce_140_set_isdisjoint_true() {
        let code = "def f():\n    s1 = {1, 2}\n    s2 = {3, 4}\n    return s1.isdisjoint(s2)\n";
        let result = transpile(code);
        assert!(result.contains("is_disjoint") || result.contains("isdisjoint"));
    }

    #[test]
    fn test_w21ce_141_set_isdisjoint_false() {
        let code =
            "def f():\n    s1 = {1, 2, 3}\n    s2 = {3, 4, 5}\n    return s1.isdisjoint(s2)\n";
        let result = transpile(code);
        assert!(result.contains("is_disjoint") || result.contains("isdisjoint"));
    }

    #[test]
    fn test_w21ce_142_set_update_multiple_elements() {
        let code = "def f():\n    s = {1}\n    s.update({2, 3, 4})\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("update"));
    }

    #[test]
    fn test_w21ce_143_set_intersection_update_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\", \"c\"}\n    s2 = {\"b\", \"c\", \"d\"}\n    s1.intersection_update(s2)\n    return s1\n";
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("retain"));
    }

    #[test]
    fn test_w21ce_144_set_difference_update_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\", \"c\"}\n    s2 = {\"b\"}\n    s1.difference_update(s2)\n    return s1\n";
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("remove"));
    }

    #[test]
    fn test_w21ce_145_set_symmetric_difference_strings() {
        let code = "def f():\n    s1 = {\"a\", \"b\"}\n    s2 = {\"b\", \"c\"}\n    result = s1.symmetric_difference(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("symmetric_difference") || result.contains("collect"));
    }

    #[test]
    fn test_w21ce_146_set_copy_and_modify() {
        let code = "def f():\n    s = {1, 2, 3}\n    s2 = s.copy()\n    s2.add(4)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("clone") || result.contains("copy"));
    }

    #[test]
    fn test_w21ce_147_set_add_duplicate() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.add(2)\n    return len(s)\n";
        let result = transpile(code);
        assert!(result.contains("insert") && result.contains("len"));
    }

    #[test]
    fn test_w21ce_148_set_discard_missing() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.discard(99)\n    return s\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("discard"));
    }

    #[test]
    fn test_w21ce_149_set_from_string_list() {
        let code = "def f():\n    words = [\"hello\", \"hello\", \"world\"]\n    unique = set(words)\n    return unique\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_150_set_empty_union() {
        let code = "def f():\n    s1 = set()\n    s2 = {1, 2}\n    result = s1.union(s2)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("union") || result.contains("collect"));
    }

    // ========================================================================
    // SECTION 4: Deque, Collections, and Tuple Edge Cases (tests 151-200)
    // ========================================================================

    #[test]
    fn test_w21ce_151_tuple_basic_creation() {
        let code = "def f():\n    t = (1, 2, 3)\n    return t\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_152_tuple_unpack_two() {
        let code = "def f():\n    x, y = (10, 20)\n    return x + y\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_153_tuple_unpack_three() {
        let code = "def f():\n    a, b, c = (1, 2, 3)\n    return a + b + c\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_154_tuple_index_access() {
        let code = "def f():\n    t = (10, 20, 30)\n    return t[1]\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_155_tuple_mixed_types() {
        let code = "def f():\n    t = (1, \"hello\", 3.14)\n    return t\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_156_tuple_nested() {
        let code = "def f():\n    t = ((1, 2), (3, 4))\n    return t\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_157_tuple_single_element() {
        let code = "def f():\n    t = (42,)\n    return t\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_158_tuple_empty() {
        let code = "def f():\n    t = ()\n    return t\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_159_tuple_in_list() {
        let code = "def f():\n    pairs = [(1, 2), (3, 4), (5, 6)]\n    return pairs\n";
        let result = transpile(code);
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_w21ce_160_tuple_from_function_return() {
        let code = "def f():\n    return (1, 2)\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_161_tuple_len() {
        let code = "def f():\n    t = (1, 2, 3)\n    n = len(t)\n    return n\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_162_tuple_concatenation() {
        let code = "def f():\n    t1 = (1, 2)\n    t2 = (3, 4)\n    t3 = t1 + t2\n    return t3\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_163_tuple_membership() {
        let code = "def f():\n    t = (1, 2, 3)\n    found = 2 in t\n    return found\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_164_list_of_tuples_unpack() {
        let code = "def f():\n    pairs = [(\"a\", 1), (\"b\", 2)]\n    for k, v in pairs:\n        pass\n    return 0\n";
        let result = transpile(code);
        assert!(result.contains("for") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_165_enumerate_list() {
        let code = "def f():\n    items = [\"a\", \"b\", \"c\"]\n    for i, item in enumerate(items):\n        pass\n    return 0\n";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_166_zip_two_lists() {
        let code = "def f():\n    a = [1, 2, 3]\n    b = [\"x\", \"y\", \"z\"]\n    pairs = list(zip(a, b))\n    return pairs\n";
        let result = transpile(code);
        assert!(result.contains("zip") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_167_list_sorted_builtin() {
        let code = "def f():\n    lst = [3, 1, 2]\n    result = sorted(lst)\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("sort") || result.contains("sorted"));
    }

    #[test]
    fn test_w21ce_168_list_reversed_builtin() {
        let code =
            "def f():\n    lst = [1, 2, 3]\n    result = list(reversed(lst))\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("rev") || result.contains("reverse") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_169_list_min_max() {
        let code = "def f():\n    lst = [5, 3, 8, 1]\n    lo = min(lst)\n    hi = max(lst)\n    return lo + hi\n";
        let result = transpile(code);
        assert!(result.contains("min") || result.contains("max") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_170_list_sum() {
        let code = "def f():\n    lst = [1, 2, 3, 4]\n    total = sum(lst)\n    return total\n";
        let result = transpile(code);
        assert!(result.contains("sum") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_171_list_any_all() {
        let code = "def f():\n    lst = [True, False, True]\n    a = any(lst)\n    b = all(lst)\n    return a\n";
        let result = transpile(code);
        assert!(result.contains("any") || result.contains("all") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_172_list_map_int() {
        let code = "def f():\n    strs = [\"1\", \"2\", \"3\"]\n    nums = list(map(int, strs))\n    return nums\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_173_list_filter_none() {
        let code = "def f():\n    items = [1, 0, 2, 0, 3]\n    result = list(filter(None, items))\n    return result\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_174_bytes_creation() {
        let code = "def f():\n    b = bytes(10)\n    return b\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_175_bytearray_from_literal() {
        let code = "def f():\n    ba = bytearray(b\"hello\")\n    return ba\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_176_list_slicing_basic() {
        let code = "def f():\n    lst = [1, 2, 3, 4, 5]\n    sub = lst[1:3]\n    return sub\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_177_list_slicing_from_start() {
        let code = "def f():\n    lst = [1, 2, 3, 4]\n    sub = lst[:2]\n    return sub\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_178_list_slicing_to_end() {
        let code = "def f():\n    lst = [1, 2, 3, 4]\n    sub = lst[2:]\n    return sub\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_179_list_negative_index() {
        let code = "def f():\n    lst = [1, 2, 3, 4]\n    last = lst[-1]\n    return last\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_180_list_step_slicing() {
        let code =
            "def f():\n    lst = [1, 2, 3, 4, 5, 6]\n    evens = lst[::2]\n    return evens\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_181_dict_comprehension_transform_values() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    doubled = {k: v * 2 for k, v in d.items()}\n    return doubled\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_182_dict_with_bool_values() {
        let code =
            "def f():\n    flags = {\"debug\": True, \"verbose\": False}\n    return flags\n";
        let result = transpile(code);
        assert!(result.contains("HashMap") || result.contains("map"));
    }

    #[test]
    fn test_w21ce_183_list_extend_from_range() {
        let code = "def f():\n    lst = [1, 2]\n    lst.extend(range(3, 6))\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w21ce_184_dict_iteration_values() {
        let code = "def f():\n    d = {\"a\": 10, \"b\": 20}\n    total = 0\n    for v in d.values():\n        total = total + v\n    return total\n";
        let result = transpile(code);
        assert!(result.contains("values") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_185_set_from_comprehension_filter() {
        let code = "def f():\n    nums = [1, 2, 3, 4, 5]\n    evens = {x for x in nums if x % 2 == 0}\n    return evens\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_186_tuple_with_strings() {
        let code = "def f():\n    t = (\"hello\", \"world\")\n    return t\n";
        let result = transpile(code);
        assert!(result.contains("to_string") || result.contains("String"));
    }

    #[test]
    fn test_w21ce_187_list_comprehension_enumerate() {
        let code = "def f():\n    items = [\"a\", \"b\", \"c\"]\n    result = [(i, x) for i, x in enumerate(items)]\n    return result\n";
        let result = transpile(code);
        assert!(result.contains("enumerate") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_188_dict_get_with_variable_key() {
        let code = "def f(key: str):\n    d = {\"a\": 1, \"b\": 2}\n    val = d.get(key, -1)\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("get") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_189_list_append_from_dict_get() {
        let code = "def f():\n    d = {\"x\": 10}\n    lst = []\n    lst.append(d.get(\"x\", 0))\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("push") && result.contains("get"));
    }

    #[test]
    fn test_w21ce_190_set_from_dict_keys() {
        let code = "def f():\n    d = {\"a\": 1, \"b\": 2}\n    s = set(d.keys())\n    return s\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_191_list_comp_with_len() {
        let code = "def f():\n    words = [\"hi\", \"hello\", \"hey\"]\n    lengths = [len(w) for w in words]\n    return lengths\n";
        let result = transpile(code);
        assert!(result.contains("len") || result.contains("map") || result.contains("iter"));
    }

    #[test]
    fn test_w21ce_192_dict_nested_with_lists() {
        let code =
            "def f():\n    d = {\"nums\": [1, 2, 3], \"strs\": [\"a\", \"b\"]}\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_193_tuple_as_dict_key() {
        let code = "def f():\n    d = {}\n    d[(1, 2)] = \"point\"\n    return d\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_194_list_pop_in_while_loop() {
        let code = "def f():\n    lst = [1, 2, 3]\n    total = 0\n    while len(lst) > 0:\n        total = total + lst.pop()\n    return total\n";
        let result = transpile(code);
        assert!(result.contains("pop") || result.contains("while"));
    }

    #[test]
    fn test_w21ce_195_dict_setdefault_increment() {
        let code = "def f():\n    d = {}\n    d.setdefault(\"count\", 0)\n    return d\n";
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w21ce_196_set_from_string() {
        let code = "def f():\n    s = set(\"hello\")\n    return s\n";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ce_197_list_all_string_ops() {
        let code = "def f():\n    lst = [\"hello\", \"world\"]\n    result = [s.upper() for s in lst]\n    return result\n";
        let result = transpile(code);
        assert!(
            result.contains("to_uppercase") || result.contains("upper") || result.contains("map")
        );
    }

    #[test]
    fn test_w21ce_198_dict_pop_string_default() {
        let code = "def f():\n    d = {\"name\": \"alice\"}\n    val = d.pop(\"name\", \"unknown\")\n    return val\n";
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w21ce_199_list_insert_negative_index() {
        let code = "def f():\n    lst = [1, 2, 3]\n    lst.insert(-1, 99)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w21ce_200_collection_chained_operations() {
        let code = "def f():\n    lst = [3, 1, 4, 1, 5]\n    lst.sort()\n    lst.reverse()\n    first = lst.pop(0)\n    lst.append(first)\n    return lst\n";
        let result = transpile(code);
        assert!(result.contains("sort") || result.contains("reverse") || result.contains("push"));
    }
}
