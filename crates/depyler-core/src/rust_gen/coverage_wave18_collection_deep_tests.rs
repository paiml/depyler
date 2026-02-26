//! Wave 18: Deep coverage tests for dict_methods.rs, list_methods.rs, and set_methods.rs
//!
//! 200 tests targeting uncovered code paths in collection method handlers:
//! - dict_methods.rs: pop(key, default), update, popitem, copy, setdefault, get variants
//! - list_methods.rs: sort(key/reverse), extend, insert, count, index, reverse, copy
//! - set_methods.rs: union, intersection, difference, symmetric_difference,
//!   issubset, issuperset, isdisjoint, intersection_update, difference_update
//!
//! Status: 200/200 tests

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
    // DICT METHODS (70 tests: test_w18cd_dict_001 through test_w18cd_dict_070)
    // ========================================================================

    // --- dict.pop(key, default) (2-arg) ---

    #[test]
    fn test_w18cd_dict_001_pop_key_default_str() {
        let code = r#"
def f():
    d = {"a": "x", "b": "y"}
    val = d.pop("a", "fallback")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop"));
    }

    #[test]
    fn test_w18cd_dict_002_pop_key_default_int() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    val = d.pop("a", 0)
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_003_pop_key_only() {
        let code = r#"
def f():
    d = {"a": 10}
    val = d.pop("a")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w18cd_dict_004_pop_key_default_empty_string() {
        let code = r#"
def f():
    d = {"name": "alice"}
    val = d.pop("name", "")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_005_pop_variable_key() {
        let code = r#"
def f(key: str):
    d = {"a": 1, "b": 2}
    return d.pop(key, -1)
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w18cd_dict_006_pop_return_value() {
        let code = r#"
def f():
    d = {"x": 42}
    result = d.pop("x", 0)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_007_pop_missing_key_with_default() {
        let code = r#"
def f():
    d = {"a": 1}
    return d.pop("missing", 99)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_008_pop_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2, "c": 3}
    keys = ["a", "b"]
    for k in keys:
        d.pop(k, 0)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- dict.update(other) ---

    #[test]
    fn test_w18cd_dict_009_update_dict_var() {
        let code = r#"
def f():
    d1 = {"a": 1}
    d2 = {"b": 2}
    d1.update(d2)
    return d1
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("extend") || result.contains("iter"));
    }

    #[test]
    fn test_w18cd_dict_010_update_dict_literal() {
        let code = r#"
def f():
    d = {"a": 1}
    d.update({"b": 2, "c": 3})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    #[test]
    fn test_w18cd_dict_011_update_overwrite_existing() {
        let code = r#"
def f():
    d = {"a": 1}
    d.update({"a": 99})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    #[test]
    fn test_w18cd_dict_012_update_string_values() {
        let code = r#"
def f():
    d = {"name": "alice"}
    d.update({"name": "bob", "age": "30"})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    #[test]
    fn test_w18cd_dict_013_update_empty_dict() {
        let code = r#"
def f():
    d = {}
    d.update({"a": 1, "b": 2})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    #[test]
    fn test_w18cd_dict_014_update_in_function() {
        let code = r#"
def merge(base, extra):
    base.update(extra)
    return base
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter") || result.contains("extend"));
    }

    // --- dict.popitem() ---

    #[test]
    fn test_w18cd_dict_015_popitem_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    item = d.popitem()
    return item
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem"));
    }

    #[test]
    fn test_w18cd_dict_016_popitem_assign() {
        let code = r#"
def f():
    d = {"x": 10}
    pair = d.popitem()
    return pair
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove"));
    }

    #[test]
    fn test_w18cd_dict_017_popitem_in_loop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2, "c": 3}
    items = []
    while len(d) > 0:
        items.append(d.popitem())
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove") || result.contains("popitem"));
    }

    // --- dict.copy() ---

    #[test]
    fn test_w18cd_dict_018_copy_basic() {
        let code = r#"
def f():
    d1 = {"a": 1, "b": 2}
    d2 = d1.copy()
    return d2
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w18cd_dict_019_copy_string_dict() {
        let code = r#"
def f():
    d = {"name": "alice", "city": "nyc"}
    backup = d.copy()
    return backup
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w18cd_dict_020_copy_then_modify() {
        let code = r#"
def f():
    original = {"a": 1}
    copied = original.copy()
    copied["b"] = 2
    return copied
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    // --- dict.setdefault(key, default) ---

    #[test]
    fn test_w18cd_dict_021_setdefault_basic() {
        let code = r#"
def f():
    d = {"a": 1}
    val = d.setdefault("a", 99)
    return val
"#;
        let result = transpile(code);
        assert!(
            result.contains("entry")
                || result.contains("or_insert")
                || result.contains("setdefault")
        );
    }

    #[test]
    fn test_w18cd_dict_022_setdefault_new_key() {
        let code = r#"
def f():
    d = {"a": 1}
    val = d.setdefault("b", 42)
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w18cd_dict_023_setdefault_string_values() {
        let code = r#"
def f():
    d = {"name": "alice"}
    val = d.setdefault("city", "unknown")
    return val
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    #[test]
    fn test_w18cd_dict_024_setdefault_in_loop() {
        let code = r#"
def f():
    d = {}
    keys = ["a", "b", "c"]
    for k in keys:
        d.setdefault(k, 0)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    // --- dict.get() various forms ---

    #[test]
    fn test_w18cd_dict_025_get_single_arg() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.get("a")
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_026_get_with_default() {
        let code = r#"
def f():
    d = {"a": 1}
    return d.get("b", 0)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_027_get_string_default() {
        let code = r#"
def f():
    d = {"name": "alice"}
    return d.get("city", "unknown")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_028_get_variable_key() {
        let code = r#"
def f(key: str):
    d = {"a": 1, "b": 2}
    return d.get(key)
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_029_get_variable_key_with_default() {
        let code = r#"
def f(key: str):
    d = {"a": 1, "b": 2}
    return d.get(key, -1)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_030_get_chained() {
        let code = r#"
def f():
    d = {"a": 1}
    x = d.get("a")
    return x
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    // --- dict.keys(), values(), items() ---

    #[test]
    fn test_w18cd_dict_031_keys_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.keys()
"#;
        let result = transpile(code);
        assert!(result.contains("keys()"));
    }

    #[test]
    fn test_w18cd_dict_032_values_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.values()
"#;
        let result = transpile(code);
        assert!(result.contains("values()"));
    }

    #[test]
    fn test_w18cd_dict_033_items_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    return d.items()
"#;
        let result = transpile(code);
        assert!(result.contains("iter()") || result.contains("items"));
    }

    #[test]
    fn test_w18cd_dict_034_keys_iterate() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    result = []
    for k in d.keys():
        result.append(k)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("keys()"));
    }

    #[test]
    fn test_w18cd_dict_035_values_iterate() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    total = 0
    for v in d.values():
        total = total + v
    return total
"#;
        let result = transpile(code);
        assert!(result.contains("values()"));
    }

    #[test]
    fn test_w18cd_dict_036_items_iterate() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    pairs = []
    for k, v in d.items():
        pairs.append(k)
    return pairs
"#;
        let result = transpile(code);
        assert!(result.contains("iter()") || result.contains("items"));
    }

    // --- dict.clear() ---

    #[test]
    fn test_w18cd_dict_037_clear_basic() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    d.clear()
    return d
"#;
        let result = transpile(code);
        assert!(result.contains(".clear()"));
    }

    #[test]
    fn test_w18cd_dict_038_clear_empty_dict() {
        let code = r#"
def f():
    d = {}
    d.clear()
    return d
"#;
        let result = transpile(code);
        assert!(result.contains(".clear()"));
    }

    // --- dict pop with variable defaults ---

    #[test]
    fn test_w18cd_dict_039_pop_int_default_zero() {
        let code = r#"
def f():
    d = {"x": 100}
    return d.pop("y", 0)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_040_pop_in_conditional() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    if "a" in d:
        return d.pop("a", -1)
    return -1
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- dict.get with None check patterns ---

    #[test]
    fn test_w18cd_dict_041_get_none_check() {
        let code = r#"
def f():
    d = {"a": 1}
    val = d.get("b")
    if val is None:
        return -1
    return val
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_042_get_or_default_pattern() {
        let code = r#"
def f(config):
    timeout = config.get("timeout", 30)
    return timeout
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    // --- dict update multiple times ---

    #[test]
    fn test_w18cd_dict_043_update_chain() {
        let code = r#"
def f():
    d = {}
    d.update({"a": 1})
    d.update({"b": 2})
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    // --- dict copy and modify ---

    #[test]
    fn test_w18cd_dict_044_copy_independent() {
        let code = r#"
def f():
    a = {"x": 1}
    b = a.copy()
    b["y"] = 2
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    // --- dict setdefault accumulation pattern ---

    #[test]
    fn test_w18cd_dict_045_setdefault_accumulate() {
        let code = r#"
def f():
    groups = {}
    items = [("a", 1), ("b", 2), ("a", 3)]
    for key, val in items:
        groups.setdefault(key, val)
    return groups
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    // --- dict.get with literal keys ---

    #[test]
    fn test_w18cd_dict_046_get_literal_string_key() {
        let code = r#"
def f():
    d = {"hello": "world"}
    return d.get("hello")
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_047_get_literal_default_string() {
        let code = r#"
def f():
    d = {"a": "first"}
    return d.get("missing", "none")
"#;
        let result = transpile(code);
        assert!(result.contains("unwrap_or"));
    }

    // --- dict pop single arg ---

    #[test]
    fn test_w18cd_dict_048_pop_single_arg_expect() {
        let code = r#"
def f():
    d = {"key": "value"}
    return d.pop("key")
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- dict operations in function params ---

    #[test]
    fn test_w18cd_dict_049_get_param_dict() {
        let code = r#"
def lookup(data, key: str):
    return data.get(key, "default")
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_050_update_param_dict() {
        let code = r#"
def extend_config(config, overrides):
    config.update(overrides)
    return config
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter") || result.contains("extend"));
    }

    // --- dict get with int default ---

    #[test]
    fn test_w18cd_dict_051_get_int_default() {
        let code = r#"
def count_word(counts, word: str) -> int:
    return counts.get(word, 0)
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("unwrap_or"));
    }

    // --- dict popitem return tuple ---

    #[test]
    fn test_w18cd_dict_052_popitem_tuple_unpack() {
        let code = r#"
def f():
    d = {"a": 1}
    pair = d.popitem()
    return pair
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove"));
    }

    // --- dict copy of string dict ---

    #[test]
    fn test_w18cd_dict_053_copy_large_dict() {
        let code = r#"
def f():
    d = {"a": "1", "b": "2", "c": "3", "d": "4"}
    return d.copy()
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    // --- dict.setdefault chained ---

    #[test]
    fn test_w18cd_dict_054_setdefault_return_value() {
        let code = r#"
def f():
    d = {}
    result = d.setdefault("key", "value")
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    // --- more dict get patterns ---

    #[test]
    fn test_w18cd_dict_055_get_in_expression() {
        let code = r#"
def f():
    d = {"a": 10}
    x = d.get("a", 0) + 5
    return x
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_056_get_comparison() {
        let code = r#"
def f():
    d = {"level": "high"}
    if d.get("level", "low") == "high":
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    #[test]
    fn test_w18cd_dict_057_multiple_gets() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    x = d.get("a")
    y = d.get("b")
    return x
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    // --- dict update with string dict ---

    #[test]
    fn test_w18cd_dict_058_update_string_dict() {
        let code = r#"
def f():
    headers = {"content-type": "text/html"}
    headers.update({"accept": "application/json"})
    return headers
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    // --- dict operations combined ---

    #[test]
    fn test_w18cd_dict_059_get_then_pop() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2}
    val = d.get("a")
    d.pop("a", 0)
    return val
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("remove"));
    }

    #[test]
    fn test_w18cd_dict_060_copy_then_update() {
        let code = r#"
def f():
    base = {"a": 1}
    new = base.copy()
    new.update({"b": 2})
    return new
"#;
        let result = transpile(code);
        assert!(result.contains("clone") && (result.contains("insert") || result.contains("iter")));
    }

    // --- dict clear then repopulate ---

    #[test]
    fn test_w18cd_dict_061_clear_repopulate() {
        let code = r#"
def f():
    d = {"old": 1}
    d.clear()
    d["new"] = 2
    return d
"#;
        let result = transpile(code);
        assert!(result.contains(".clear()"));
    }

    // --- dict pop with bool default ---

    #[test]
    fn test_w18cd_dict_062_pop_bool_default() {
        let code = r#"
def f():
    flags = {"debug": True}
    return flags.pop("verbose", False)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    // --- dict setdefault with complex default ---

    #[test]
    fn test_w18cd_dict_063_setdefault_int_zero() {
        let code = r#"
def f():
    counts = {}
    counts.setdefault("a", 0)
    return counts
"#;
        let result = transpile(code);
        assert!(result.contains("entry") || result.contains("or_insert"));
    }

    // --- dict pop edge cases ---

    #[test]
    fn test_w18cd_dict_064_pop_float_default() {
        let code = r#"
def f():
    scores = {"math": 95.0}
    return scores.pop("english", 0.0)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_065_pop_multiple_times() {
        let code = r#"
def f():
    d = {"a": 1, "b": 2, "c": 3}
    x = d.pop("a", 0)
    y = d.pop("b", 0)
    return x + y
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- dict get with expression result ---

    #[test]
    fn test_w18cd_dict_066_get_default_used_in_calc() {
        let code = r#"
def f():
    prices = {"apple": 3}
    total = prices.get("apple", 0) + prices.get("banana", 0)
    return total
"#;
        let result = transpile(code);
        assert!(result.contains(".get("));
    }

    // --- dict operations with param dicts ---

    #[test]
    fn test_w18cd_dict_067_popitem_to_var() {
        let code = r#"
def f():
    d = {"only": "one"}
    key_val = d.popitem()
    return key_val
"#;
        let result = transpile(code);
        assert!(result.contains("keys") || result.contains("remove"));
    }

    #[test]
    fn test_w18cd_dict_068_copy_empty() {
        let code = r#"
def f():
    d = {}
    e = d.copy()
    return e
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w18cd_dict_069_get_nested_default() {
        let code = r#"
def f():
    d = {"a": 1}
    val = d.get("b", 100)
    return val * 2
"#;
        let result = transpile(code);
        assert!(result.contains(".get(") && result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_dict_070_update_from_computed() {
        let code = r#"
def f():
    d = {"x": 1}
    extra = {"y": 2}
    d.update(extra)
    return d
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("iter"));
    }

    // ========================================================================
    // LIST METHODS (70 tests: test_w18cd_list_071 through test_w18cd_list_140)
    // ========================================================================

    // --- list.sort(reverse=True) ---

    #[test]
    fn test_w18cd_list_071_sort_basic() {
        let code = r#"
def f():
    nums = [3, 1, 2]
    nums.sort()
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains(".sort()") || result.contains("sort"));
    }

    #[test]
    fn test_w18cd_list_072_sort_reverse_true() {
        let code = r#"
def f():
    nums = [3, 1, 2]
    nums.sort(reverse=True)
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("sort_by") || result.contains("sort") || result.contains("cmp"));
    }

    #[test]
    fn test_w18cd_list_073_sort_reverse_false() {
        let code = r#"
def f():
    nums = [3, 1, 2]
    nums.sort(reverse=False)
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("sort"));
    }

    #[test]
    fn test_w18cd_list_074_sort_key_len() {
        let code = r#"
def f():
    words = ["cc", "a", "bbb"]
    words.sort(key=len)
    return words
"#;
        let result = transpile(code);
        assert!(result.contains("sort_by_key") || result.contains("sort"));
    }

    #[test]
    fn test_w18cd_list_075_sort_key_and_reverse() {
        let code = r#"
def f():
    words = ["cc", "a", "bbb"]
    words.sort(key=len, reverse=True)
    return words
"#;
        let result = transpile(code);
        assert!(
            result.contains("sort_by_key") || result.contains("Reverse") || result.contains("sort")
        );
    }

    // --- list.extend ---

    #[test]
    fn test_w18cd_list_076_extend_list() {
        let code = r#"
def f():
    a = [1, 2]
    b = [3, 4]
    a.extend(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w18cd_list_077_extend_empty() {
        let code = r#"
def f():
    a = [1, 2]
    b = []
    a.extend(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w18cd_list_078_extend_strings() {
        let code = r#"
def f():
    a = ["hello"]
    b = ["world"]
    a.extend(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w18cd_list_079_extend_param() {
        let code = r#"
def f(extra):
    base = [1, 2, 3]
    base.extend(extra)
    return base
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    // --- list.insert ---

    #[test]
    fn test_w18cd_list_080_insert_at_zero() {
        let code = r#"
def f():
    items = [2, 3]
    items.insert(0, 1)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert") && (result.contains("usize") || result.contains("0")));
    }

    #[test]
    fn test_w18cd_list_081_insert_at_end() {
        let code = r#"
def f():
    items = [1, 2]
    items.insert(2, 3)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w18cd_list_082_insert_string() {
        let code = r#"
def f():
    words = ["b", "c"]
    words.insert(0, "a")
    return words
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w18cd_list_083_insert_middle() {
        let code = r#"
def f():
    items = [1, 3]
    items.insert(1, 2)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- list.count ---

    #[test]
    fn test_w18cd_list_084_count_int() {
        let code = r#"
def f():
    nums = [1, 2, 2, 3, 2]
    return nums.count(2)
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w18cd_list_085_count_string() {
        let code = r#"
def f():
    words = ["hello", "world", "hello"]
    return words.count("hello")
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w18cd_list_086_count_zero() {
        let code = r#"
def f():
    nums = [1, 2, 3]
    return nums.count(42)
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    #[test]
    fn test_w18cd_list_087_count_in_comparison() {
        let code = r#"
def f():
    items = [1, 1, 2]
    if items.count(1) > 1:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    // --- list.index ---

    #[test]
    fn test_w18cd_list_088_index_int() {
        let code = r#"
def f():
    nums = [10, 20, 30]
    return nums.index(20)
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w18cd_list_089_index_string() {
        let code = r#"
def f():
    words = ["a", "b", "c"]
    return words.index("b")
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w18cd_list_090_index_first_occurrence() {
        let code = r#"
def f():
    nums = [1, 2, 3, 2]
    return nums.index(2)
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    #[test]
    fn test_w18cd_list_091_index_in_expression() {
        let code = r#"
def f():
    letters = ["x", "y", "z"]
    pos = letters.index("y")
    return pos
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    // --- list.reverse ---

    #[test]
    fn test_w18cd_list_092_reverse_basic() {
        let code = r#"
def f():
    nums = [1, 2, 3]
    nums.reverse()
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains(".reverse()"));
    }

    #[test]
    fn test_w18cd_list_093_reverse_strings() {
        let code = r#"
def f():
    words = ["c", "b", "a"]
    words.reverse()
    return words
"#;
        let result = transpile(code);
        assert!(result.contains(".reverse()"));
    }

    #[test]
    fn test_w18cd_list_094_reverse_single_element() {
        let code = r#"
def f():
    items = [42]
    items.reverse()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains(".reverse()"));
    }

    // --- list.copy ---

    #[test]
    fn test_w18cd_list_095_copy_basic() {
        let code = r#"
def f():
    a = [1, 2, 3]
    b = a.copy()
    return b
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w18cd_list_096_copy_strings() {
        let code = r#"
def f():
    words = ["hello", "world"]
    backup = words.copy()
    return backup
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w18cd_list_097_copy_then_modify() {
        let code = r#"
def f():
    original = [1, 2, 3]
    copied = original.copy()
    copied.append(4)
    return copied
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    // --- list.clear ---

    #[test]
    fn test_w18cd_list_098_clear_basic() {
        let code = r#"
def f():
    items = [1, 2, 3]
    items.clear()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains(".clear()"));
    }

    // --- list.pop ---

    #[test]
    fn test_w18cd_list_099_pop_no_args() {
        let code = r#"
def f():
    items = [1, 2, 3]
    last = items.pop()
    return last
"#;
        let result = transpile(code);
        assert!(result.contains(".pop()") || result.contains("unwrap_or"));
    }

    #[test]
    fn test_w18cd_list_100_pop_index() {
        let code = r#"
def f():
    items = [1, 2, 3]
    first = items.pop(0)
    return first
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop"));
    }

    // --- list.remove ---

    #[test]
    fn test_w18cd_list_101_remove_int() {
        let code = r#"
def f():
    nums = [1, 2, 3]
    nums.remove(2)
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("remove"));
    }

    #[test]
    fn test_w18cd_list_102_remove_string() {
        let code = r#"
def f():
    words = ["a", "b", "c"]
    words.remove("b")
    return words
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("remove"));
    }

    // --- list.append edge cases ---

    #[test]
    fn test_w18cd_list_103_append_int() {
        let code = r#"
def f():
    nums = [1, 2]
    nums.append(3)
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w18cd_list_104_append_string_literal() {
        let code = r#"
def f():
    words = ["a"]
    words.append("b")
    return words
"#;
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    // --- list sort with simple sort call ---

    #[test]
    fn test_w18cd_list_105_sort_integers() {
        let code = r#"
def f():
    nums = [5, 3, 8, 1]
    nums.sort()
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains(".sort()") || result.contains("sort"));
    }

    #[test]
    fn test_w18cd_list_106_sort_strings() {
        let code = r#"
def f():
    words = ["banana", "apple", "cherry"]
    words.sort()
    return words
"#;
        let result = transpile(code);
        assert!(result.contains(".sort()") || result.contains("sort"));
    }

    // --- list operations combined ---

    #[test]
    fn test_w18cd_list_107_extend_then_sort() {
        let code = r#"
def f():
    a = [3, 1]
    b = [2, 4]
    a.extend(b)
    a.sort()
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend") && result.contains("sort"));
    }

    #[test]
    fn test_w18cd_list_108_insert_then_count() {
        let code = r#"
def f():
    nums = [1, 2, 3]
    nums.insert(0, 1)
    return nums.count(1)
"#;
        let result = transpile(code);
        assert!(
            result.contains("insert") && (result.contains("filter") || result.contains("count"))
        );
    }

    #[test]
    fn test_w18cd_list_109_reverse_then_index() {
        let code = r#"
def f():
    nums = [10, 20, 30]
    nums.reverse()
    return nums.index(30)
"#;
        let result = transpile(code);
        assert!(
            result.contains("reverse") && (result.contains("position") || result.contains("index"))
        );
    }

    #[test]
    fn test_w18cd_list_110_copy_then_clear() {
        let code = r#"
def f():
    items = [1, 2, 3]
    backup = items.copy()
    items.clear()
    return backup
"#;
        let result = transpile(code);
        assert!(result.contains("clone") && result.contains("clear"));
    }

    // --- list extend with different iterables ---

    #[test]
    fn test_w18cd_list_111_extend_range() {
        let code = r#"
def f():
    items = [0]
    extra = [1, 2, 3]
    items.extend(extra)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    #[test]
    fn test_w18cd_list_112_extend_multiple() {
        let code = r#"
def f():
    result = []
    a = [1]
    b = [2]
    result.extend(a)
    result.extend(b)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    // --- list.insert at various positions ---

    #[test]
    fn test_w18cd_list_113_insert_negative() {
        let code = r#"
def f():
    items = [1, 2, 3]
    items.insert(-1, 99)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- list.count edge cases ---

    #[test]
    fn test_w18cd_list_114_count_bool() {
        let code = r#"
def f():
    flags = [True, False, True, True]
    return flags.count(True)
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    // --- list.index with var ---

    #[test]
    fn test_w18cd_list_115_index_variable() {
        let code = r#"
def f(target: int):
    nums = [10, 20, 30, 40]
    return nums.index(target)
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    // --- list sort reverse with strings ---

    #[test]
    fn test_w18cd_list_116_sort_reverse_strings() {
        let code = r#"
def f():
    words = ["a", "c", "b"]
    words.sort(reverse=True)
    return words
"#;
        let result = transpile(code);
        assert!(result.contains("sort_by") || result.contains("sort") || result.contains("cmp"));
    }

    // --- list operations in loops ---

    #[test]
    fn test_w18cd_list_117_append_in_loop() {
        let code = r#"
def f():
    result = []
    for i in range(5):
        result.append(i)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("push"));
    }

    #[test]
    fn test_w18cd_list_118_insert_in_loop() {
        let code = r#"
def f():
    items = []
    for i in range(3):
        items.insert(0, i)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- list pop return value ---

    #[test]
    fn test_w18cd_list_119_pop_and_use() {
        let code = r#"
def f():
    stack = [1, 2, 3]
    top = stack.pop()
    return top
"#;
        let result = transpile(code);
        assert!(result.contains(".pop()") || result.contains("unwrap_or"));
    }

    // --- list remove then count ---

    #[test]
    fn test_w18cd_list_120_remove_then_count() {
        let code = r#"
def f():
    items = [1, 2, 2, 3]
    items.remove(2)
    return items.count(2)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("position"));
    }

    // --- list reverse twice ---

    #[test]
    fn test_w18cd_list_121_reverse_twice() {
        let code = r#"
def f():
    items = [1, 2, 3]
    items.reverse()
    items.reverse()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("reverse"));
    }

    // --- list copy empty ---

    #[test]
    fn test_w18cd_list_122_copy_empty() {
        let code = r#"
def f():
    items = []
    copy = items.copy()
    return copy
"#;
        let result = transpile(code);
        assert!(result.contains("clone"));
    }

    // --- list pop with index var ---

    #[test]
    fn test_w18cd_list_123_pop_index_var() {
        let code = r#"
def f(idx: int):
    items = [10, 20, 30]
    return items.pop(idx)
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("pop"));
    }

    // --- list extend then reverse ---

    #[test]
    fn test_w18cd_list_124_extend_reverse() {
        let code = r#"
def f():
    a = [1, 2]
    b = [3, 4]
    a.extend(b)
    a.reverse()
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend") && result.contains("reverse"));
    }

    // --- list complex chained operations ---

    #[test]
    fn test_w18cd_list_125_sort_then_reverse() {
        let code = r#"
def f():
    nums = [3, 1, 4, 1, 5]
    nums.sort()
    nums.reverse()
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("sort") && result.contains("reverse"));
    }

    #[test]
    fn test_w18cd_list_126_clear_then_extend() {
        let code = r#"
def f():
    items = [1, 2, 3]
    items.clear()
    new_items = [4, 5]
    items.extend(new_items)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("clear") && result.contains("extend"));
    }

    // --- list.count for comparison ---

    #[test]
    fn test_w18cd_list_127_count_equals_zero() {
        let code = r#"
def f():
    items = [1, 2, 3]
    if items.count(99) == 0:
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    // --- list.index used as slice bound ---

    #[test]
    fn test_w18cd_list_128_index_used_in_slice() {
        let code = r#"
def f():
    nums = [10, 20, 30, 40]
    idx = nums.index(30)
    return idx
"#;
        let result = transpile(code);
        assert!(result.contains("position") || result.contains("index"));
    }

    // --- list.insert at beginning multiple ---

    #[test]
    fn test_w18cd_list_129_insert_beginning_multiple() {
        let code = r#"
def f():
    items = []
    items.insert(0, "c")
    items.insert(0, "b")
    items.insert(0, "a")
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- list sort key only ---

    #[test]
    fn test_w18cd_list_130_sort_key_abs() {
        let code = r#"
def f():
    nums = [-3, 1, -2]
    nums.sort(key=abs)
    return nums
"#;
        let result = transpile(code);
        assert!(result.contains("sort_by_key") || result.contains("sort"));
    }

    // --- list extend from param ---

    #[test]
    fn test_w18cd_list_131_extend_from_function() {
        let code = r#"
def combine(a, b):
    result = []
    result.extend(a)
    result.extend(b)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("extend"));
    }

    // --- list.remove from end ---

    #[test]
    fn test_w18cd_list_132_remove_last() {
        let code = r#"
def f():
    items = [1, 2, 3]
    items.remove(3)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("remove") || result.contains("position"));
    }

    // --- list count and index together ---

    #[test]
    fn test_w18cd_list_133_count_and_index() {
        let code = r#"
def f():
    nums = [1, 2, 3, 2, 1]
    c = nums.count(2)
    i = nums.index(2)
    return c + i
"#;
        let result = transpile(code);
        assert!(
            (result.contains("filter") || result.contains("count"))
                && (result.contains("position") || result.contains("index"))
        );
    }

    // --- list.pop multiple ---

    #[test]
    fn test_w18cd_list_134_pop_until_empty() {
        let code = r#"
def f():
    items = [1, 2]
    a = items.pop()
    b = items.pop()
    return a + b
"#;
        let result = transpile(code);
        assert!(result.contains(".pop()") || result.contains("unwrap_or"));
    }

    // --- list.reverse in function ---

    #[test]
    fn test_w18cd_list_135_reverse_param() {
        let code = r#"
def reverse_list(items):
    items.reverse()
    return items
"#;
        let result = transpile(code);
        assert!(result.contains(".reverse()"));
    }

    // --- list.copy then extend ---

    #[test]
    fn test_w18cd_list_136_copy_extend() {
        let code = r#"
def f():
    base = [1, 2]
    extended = base.copy()
    extra = [3, 4]
    extended.extend(extra)
    return extended
"#;
        let result = transpile(code);
        assert!(result.contains("clone") && result.contains("extend"));
    }

    // --- list sort returns None (in-place) ---

    #[test]
    fn test_w18cd_list_137_sort_inplace() {
        let code = r#"
def f():
    items = [5, 2, 8]
    items.sort()
    return items[0]
"#;
        let result = transpile(code);
        assert!(result.contains("sort"));
    }

    // --- list.insert float index ---

    #[test]
    fn test_w18cd_list_138_insert_large_index() {
        let code = r#"
def f():
    items = [1]
    items.insert(100, 2)
    return items
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- list.count with negative ---

    #[test]
    fn test_w18cd_list_139_count_negative() {
        let code = r#"
def f():
    nums = [-1, 0, 1, -1]
    return nums.count(-1)
"#;
        let result = transpile(code);
        assert!(result.contains("filter") || result.contains("count"));
    }

    // --- list.extend then sort ---

    #[test]
    fn test_w18cd_list_140_extend_sort_reverse() {
        let code = r#"
def f():
    a = [5, 3]
    b = [1, 4]
    a.extend(b)
    a.sort(reverse=True)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("extend") && result.contains("sort"));
    }

    // ========================================================================
    // SET METHODS (60 tests: test_w18cd_set_141 through test_w18cd_set_200)
    // ========================================================================

    // --- set.union ---

    #[test]
    fn test_w18cd_set_141_union_basic() {
        let code = r#"
def f():
    s1 = {1, 2, 3}
    s2 = {3, 4, 5}
    return s1.union(s2)
"#;
        let result = transpile(code);
        assert!(result.contains("union") || result.contains("HashSet"));
    }

    #[test]
    fn test_w18cd_set_142_union_strings() {
        let code = r#"
def f():
    a = {"hello", "world"}
    b = {"world", "foo"}
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(result.contains("union") || result.contains("HashSet"));
    }

    #[test]
    fn test_w18cd_set_143_union_disjoint() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w18cd_set_144_union_same() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2, 3}
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(result.contains("union"));
    }

    // --- set.intersection ---

    #[test]
    fn test_w18cd_set_145_intersection_basic() {
        let code = r#"
def f():
    s1 = {1, 2, 3}
    s2 = {2, 3, 4}
    return s1.intersection(s2)
"#;
        let result = transpile(code);
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w18cd_set_146_intersection_strings() {
        let code = r#"
def f():
    a = {"a", "b", "c"}
    b = {"b", "c", "d"}
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w18cd_set_147_intersection_empty_result() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w18cd_set_148_intersection_complete_overlap() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2, 3}
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(result.contains("intersection"));
    }

    // --- set.difference ---

    #[test]
    fn test_w18cd_set_149_difference_basic() {
        let code = r#"
def f():
    s1 = {1, 2, 3, 4}
    s2 = {3, 4, 5}
    return s1.difference(s2)
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w18cd_set_150_difference_strings() {
        let code = r#"
def f():
    a = {"x", "y", "z"}
    b = {"y"}
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w18cd_set_151_difference_no_overlap() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    #[test]
    fn test_w18cd_set_152_difference_complete_removal() {
        let code = r#"
def f():
    a = {1, 2}
    b = {1, 2, 3}
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    // --- set.symmetric_difference ---

    #[test]
    fn test_w18cd_set_153_symmetric_difference_basic() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {2, 3, 4}
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w18cd_set_154_symmetric_difference_strings() {
        let code = r#"
def f():
    a = {"a", "b"}
    b = {"b", "c"}
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w18cd_set_155_symmetric_difference_disjoint() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("symmetric_difference"));
    }

    #[test]
    fn test_w18cd_set_156_symmetric_difference_identical() {
        let code = r#"
def f():
    a = {1, 2}
    b = {1, 2}
    return a.symmetric_difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("symmetric_difference"));
    }

    // --- set.issubset ---

    #[test]
    fn test_w18cd_set_157_issubset_true() {
        let code = r#"
def f():
    a = {1, 2}
    b = {1, 2, 3}
    return a.issubset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_subset"));
    }

    #[test]
    fn test_w18cd_set_158_issubset_false() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2}
    return a.issubset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_subset"));
    }

    #[test]
    fn test_w18cd_set_159_issubset_equal() {
        let code = r#"
def f():
    a = {1, 2}
    b = {1, 2}
    return a.issubset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_subset"));
    }

    #[test]
    fn test_w18cd_set_160_issubset_empty() {
        let code = r#"
def f():
    a = set()
    b = {1, 2, 3}
    return a.issubset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_subset"));
    }

    // --- set.issuperset ---

    #[test]
    fn test_w18cd_set_161_issuperset_true() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2}
    return a.issuperset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_superset"));
    }

    #[test]
    fn test_w18cd_set_162_issuperset_false() {
        let code = r#"
def f():
    a = {1}
    b = {1, 2}
    return a.issuperset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_superset"));
    }

    #[test]
    fn test_w18cd_set_163_issuperset_equal() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2, 3}
    return a.issuperset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_superset"));
    }

    #[test]
    fn test_w18cd_set_164_issuperset_strings() {
        let code = r#"
def f():
    a = {"x", "y", "z"}
    b = {"x", "y"}
    return a.issuperset(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_superset"));
    }

    // --- set.isdisjoint ---

    #[test]
    fn test_w18cd_set_165_isdisjoint_true() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_disjoint"));
    }

    #[test]
    fn test_w18cd_set_166_isdisjoint_false() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {3, 4, 5}
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_disjoint"));
    }

    #[test]
    fn test_w18cd_set_167_isdisjoint_strings() {
        let code = r#"
def f():
    a = {"a", "b"}
    b = {"c", "d"}
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_disjoint"));
    }

    #[test]
    fn test_w18cd_set_168_isdisjoint_overlapping_strings() {
        let code = r#"
def f():
    a = {"hello", "world"}
    b = {"world", "foo"}
    return a.isdisjoint(b)
"#;
        let result = transpile(code);
        assert!(result.contains("is_disjoint"));
    }

    // --- set.intersection_update ---

    #[test]
    fn test_w18cd_set_169_intersection_update_basic() {
        let code = r#"
def f():
    a = {1, 2, 3, 4}
    b = {2, 3, 5}
    a.intersection_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(
            result.contains("intersection")
                || result.contains("retain")
                || result.contains("clear")
        );
    }

    #[test]
    fn test_w18cd_set_170_intersection_update_strings() {
        let code = r#"
def f():
    a = {"a", "b", "c"}
    b = {"b", "c", "d"}
    a.intersection_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(
            result.contains("intersection")
                || result.contains("retain")
                || result.contains("clear")
        );
    }

    #[test]
    fn test_w18cd_set_171_intersection_update_disjoint() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    a.intersection_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("clear"));
    }

    #[test]
    fn test_w18cd_set_172_intersection_update_subset() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {1, 2, 3, 4}
    a.intersection_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("clear"));
    }

    // --- set.difference_update ---

    #[test]
    fn test_w18cd_set_173_difference_update_basic() {
        let code = r#"
def f():
    a = {1, 2, 3, 4}
    b = {2, 4}
    a.difference_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(
            result.contains("difference") || result.contains("retain") || result.contains("clear")
        );
    }

    #[test]
    fn test_w18cd_set_174_difference_update_strings() {
        let code = r#"
def f():
    a = {"x", "y", "z"}
    b = {"y"}
    a.difference_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("clear"));
    }

    #[test]
    fn test_w18cd_set_175_difference_update_no_overlap() {
        let code = r#"
def f():
    a = {1, 2}
    b = {3, 4}
    a.difference_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("clear"));
    }

    #[test]
    fn test_w18cd_set_176_difference_update_all() {
        let code = r#"
def f():
    a = {1, 2}
    b = {1, 2, 3}
    a.difference_update(b)
    return a
"#;
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("clear"));
    }

    // --- set.add ---

    #[test]
    fn test_w18cd_set_177_add_int() {
        let code = r#"
def f():
    s = {1, 2}
    s.add(3)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w18cd_set_178_add_string() {
        let code = r#"
def f():
    s = {"a", "b"}
    s.add("c")
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    #[test]
    fn test_w18cd_set_179_add_duplicate() {
        let code = r#"
def f():
    s = {1, 2, 3}
    s.add(2)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- set.discard ---

    #[test]
    fn test_w18cd_set_180_discard_existing() {
        let code = r#"
def f():
    s = {1, 2, 3}
    s.discard(2)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w18cd_set_181_discard_missing() {
        let code = r#"
def f():
    s = {1, 2, 3}
    s.discard(99)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w18cd_set_182_discard_string() {
        let code = r#"
def f():
    s = {"a", "b"}
    s.discard("a")
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- set.remove ---

    #[test]
    fn test_w18cd_set_183_remove_int() {
        let code = r#"
def f():
    s = {1, 2, 3}
    s.remove(2)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    #[test]
    fn test_w18cd_set_184_remove_string() {
        let code = r#"
def f():
    s = {"x", "y", "z"}
    s.remove("y")
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("remove"));
    }

    // --- set.clear ---

    #[test]
    fn test_w18cd_set_185_clear_basic() {
        let code = r#"
def f():
    s = {1, 2, 3}
    s.clear()
    return s
"#;
        let result = transpile(code);
        assert!(result.contains(".clear()"));
    }

    // --- set.update ---

    #[test]
    fn test_w18cd_set_186_update_basic() {
        let code = r#"
def f():
    s = {1, 2}
    other = {3, 4}
    s.update(other)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("extend"));
    }

    #[test]
    fn test_w18cd_set_187_update_strings() {
        let code = r#"
def f():
    s = {"a"}
    other = {"b", "c"}
    s.update(other)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("extend"));
    }

    #[test]
    fn test_w18cd_set_188_update_overlap() {
        let code = r#"
def f():
    s = {1, 2, 3}
    other = {2, 3, 4}
    s.update(other)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert") || result.contains("extend"));
    }

    // --- set operations in conditional ---

    #[test]
    fn test_w18cd_set_189_issubset_in_if() {
        let code = r#"
def f():
    perms = {"read", "write"}
    required = {"read"}
    if required.issubset(perms):
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("is_subset"));
    }

    #[test]
    fn test_w18cd_set_190_isdisjoint_in_if() {
        let code = r#"
def f():
    forbidden = {"admin"}
    user_roles = {"reader"}
    if forbidden.isdisjoint(user_roles):
        return True
    return False
"#;
        let result = transpile(code);
        assert!(result.contains("is_disjoint"));
    }

    // --- set operations combined ---

    #[test]
    fn test_w18cd_set_191_union_then_intersection() {
        let code = r#"
def f():
    a = {1, 2}
    b = {2, 3}
    c = {2, 4}
    ab = a.union(b)
    return ab
"#;
        let result = transpile(code);
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w18cd_set_192_difference_then_union() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {2}
    diff = a.difference(b)
    return diff
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    // --- set operations with params ---

    #[test]
    fn test_w18cd_set_193_union_params() {
        let code = r#"
def merge_sets(a, b):
    return a.union(b)
"#;
        let result = transpile(code);
        assert!(result.contains("union"));
    }

    #[test]
    fn test_w18cd_set_194_intersection_params() {
        let code = r#"
def common(a, b):
    return a.intersection(b)
"#;
        let result = transpile(code);
        assert!(result.contains("intersection"));
    }

    #[test]
    fn test_w18cd_set_195_difference_params() {
        let code = r#"
def subtract(a, b):
    return a.difference(b)
"#;
        let result = transpile(code);
        assert!(result.contains("difference"));
    }

    // --- set add in loop ---

    #[test]
    fn test_w18cd_set_196_add_in_loop() {
        let code = r#"
def f():
    s = set()
    for i in range(5):
        s.add(i)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("insert"));
    }

    // --- set issuperset with computed ---

    #[test]
    fn test_w18cd_set_197_issuperset_in_function() {
        let code = r#"
def has_all(required, available):
    return available.issuperset(required)
"#;
        let result = transpile(code);
        assert!(result.contains("is_superset"));
    }

    // --- set symmetric_difference assign ---

    #[test]
    fn test_w18cd_set_198_symmetric_difference_assign() {
        let code = r#"
def f():
    a = {1, 2, 3}
    b = {3, 4, 5}
    result = a.symmetric_difference(b)
    return result
"#;
        let result = transpile(code);
        assert!(result.contains("symmetric_difference"));
    }

    // --- set intersection_update in function ---

    #[test]
    fn test_w18cd_set_199_intersection_update_function() {
        let code = r#"
def keep_common(s, other):
    s.intersection_update(other)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("intersection") || result.contains("clear"));
    }

    // --- set difference_update in function ---

    #[test]
    fn test_w18cd_set_200_difference_update_function() {
        let code = r#"
def remove_items(s, to_remove):
    s.difference_update(to_remove)
    return s
"#;
        let result = transpile(code);
        assert!(result.contains("difference") || result.contains("clear"));
    }
}
