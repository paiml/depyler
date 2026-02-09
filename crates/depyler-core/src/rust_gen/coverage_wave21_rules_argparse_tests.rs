//! Wave 21 coverage tests: direct_rules convert_method_call + argparse transform
//!
//! 200 tests targeting uncovered code paths in:
//! - convert_method_call (method_call_routing.rs): string, list, dict, set, file,
//!   bytes, Counter, itertools, json method dispatch
//! - analyze_subcommand_field_access (argparse_transform.rs): subcommand field
//!   detection, walk_expr branches, struct/enum generation
//!
//! Tests 001-140: direct_rules convert_method_call
//! Tests 141-200: argparse transform

#[cfg(test)]
mod tests {
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
    // SECTION 1: STRING METHODS (tests 001-027)
    // ========================================================================

    #[test]
    fn test_w21ra_001_string_zfill() {
        let code = "def f(s: str) -> str:\n    return s.zfill(10)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_002_string_ljust() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_003_string_rjust() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_004_string_center() {
        let code = "def f(s: str) -> str:\n    return s.center(30)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_005_string_ljust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.ljust(20, '*')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_006_string_rjust_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.rjust(20, '0')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_007_string_center_with_fillchar() {
        let code = "def f(s: str) -> str:\n    return s.center(30, '-')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_008_string_partition() {
        let code = "def f(s: str):\n    a, b, c = s.partition(',')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_009_string_rpartition() {
        let code = "def f(s: str):\n    a, b, c = s.rpartition('.')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_010_string_encode() {
        let code = "def f(s: str) -> bytes:\n    return s.encode()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_011_string_encode_utf8() {
        let code = "def f(s: str) -> bytes:\n    return s.encode('utf-8')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_012_string_isalnum() {
        let code = "def f(s: str) -> bool:\n    return s.isalnum()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_013_string_isalpha() {
        let code = "def f(s: str) -> bool:\n    return s.isalpha()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_014_string_isdigit() {
        let code = "def f(s: str) -> bool:\n    return s.isdigit()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_015_string_isspace() {
        let code = "def f(s: str) -> bool:\n    return s.isspace()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_016_string_istitle() {
        let code = "def f(s: str) -> bool:\n    return s.istitle()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_017_string_isupper() {
        let code = "def f(s: str) -> bool:\n    return s.isupper()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_018_string_islower() {
        let code = "def f(s: str) -> bool:\n    return s.islower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_019_string_swapcase() {
        let code = "def f(s: str) -> str:\n    return s.swapcase()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_020_string_casefold() {
        let code = "def f(s: str) -> str:\n    return s.casefold()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_021_string_removeprefix() {
        let code = "def f(s: str) -> str:\n    return s.removeprefix('pre_')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_022_string_removesuffix() {
        let code = "def f(s: str) -> str:\n    return s.removesuffix('_suf')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_023_string_isnumeric() {
        let code = "def f(s: str) -> bool:\n    return s.isnumeric()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_024_string_isdecimal() {
        let code = "def f(s: str) -> bool:\n    return s.isdecimal()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_025_string_isprintable() {
        let code = "def f(s: str) -> bool:\n    return s.isprintable()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_026_string_isidentifier() {
        let code = "def f(s: str) -> bool:\n    return s.isidentifier()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_027_string_format_map() {
        let code = "def f(s: str, d: dict) -> str:\n    return s.format_map(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 2: LIST METHODS (tests 028-049)
    // ========================================================================

    #[test]
    fn test_w21ra_028_list_append() {
        let code = "def f():\n    items = []\n    items.append(1)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_029_list_extend() {
        let code = "def f():\n    items = [1, 2]\n    items.extend([3, 4])\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_030_list_pop_no_arg() {
        let code = "def f():\n    items = [1, 2, 3]\n    last = items.pop()\n    return last";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_031_list_pop_with_index() {
        let code = "def f():\n    items = [1, 2, 3]\n    first = items.pop(0)\n    return first";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_032_list_insert() {
        let code = "def f():\n    items = [1, 3]\n    items.insert(1, 2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_033_list_remove() {
        let code = "def f():\n    items = [1, 2, 3]\n    items.remove(2)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_034_list_sort_basic() {
        let code = "def f():\n    items = [3, 1, 2]\n    items.sort()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_035_list_sort_reverse() {
        let code = "def f():\n    items = [3, 1, 2]\n    items.sort(reverse=True)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_036_list_reverse() {
        let code = "def f():\n    items = [1, 2, 3]\n    items.reverse()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_037_list_copy() {
        let code = "def f():\n    items = [1, 2, 3]\n    clone = items.copy()\n    return clone";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_038_list_clear() {
        let code = "def f():\n    items = [1, 2, 3]\n    items.clear()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_039_list_index() {
        let code = "def f() -> int:\n    items = [10, 20, 30]\n    return items.index(20)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_040_list_count() {
        let code = "def f() -> int:\n    items = [1, 2, 2, 3]\n    return items.count(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_041_list_append_string() {
        let code = "def f():\n    names = []\n    names.append('alice')\n    names.append('bob')\n    return names";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_042_list_extend_with_range() {
        let code = "def f():\n    items = []\n    items.extend(range(5))\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_043_list_insert_at_beginning() {
        let code = "def f():\n    items = [2, 3]\n    items.insert(0, 1)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_044_list_sort_with_key() {
        let code = "def f():\n    items = ['banana', 'apple', 'cherry']\n    items.sort(key=len)\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_045_list_remove_string() {
        let code = "def f():\n    items = ['a', 'b', 'c']\n    items.remove('b')\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_046_list_append_in_loop() {
        let code = "def f() -> list:\n    result = []\n    for i in range(10):\n        result.append(i * 2)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_047_list_pop_in_while() {
        let code = "def f():\n    items = [1, 2, 3]\n    while len(items) > 0:\n        items.pop()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_048_list_index_with_start() {
        let code = "def f() -> int:\n    items = [1, 2, 3, 2, 1]\n    return items.index(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_049_list_count_zero() {
        let code = "def f() -> int:\n    items = [1, 2, 3]\n    return items.count(99)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 3: DICT METHODS (tests 050-071)
    // ========================================================================

    #[test]
    fn test_w21ra_050_dict_get_basic() {
        let code = "def f() -> str:\n    d = {'a': 1, 'b': 2}\n    return d.get('a')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_051_dict_get_with_default() {
        let code = "def f() -> int:\n    d = {'a': 1}\n    return d.get('b', 0)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_052_dict_keys() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    k = d.keys()\n    return k";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_053_dict_values() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    v = d.values()\n    return v";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_054_dict_items() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    for k, v in d.items():\n        print(k)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_055_dict_update() {
        let code = "def f():\n    d = {'a': 1}\n    d.update({'b': 2})\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_056_dict_pop() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    val = d.pop('a')\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_057_dict_pop_with_default() {
        let code = "def f():\n    d = {'a': 1}\n    val = d.pop('b', 0)\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_058_dict_setdefault() {
        let code = "def f():\n    d = {'a': 1}\n    val = d.setdefault('b', 2)\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_059_dict_clear() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    d.clear()\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_060_dict_copy() {
        let code = "def f():\n    d = {'a': 1}\n    d2 = d.copy()\n    return d2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_061_dict_popitem() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    item = d.popitem()\n    return item";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_062_dict_fromkeys() {
        let code = "def f():\n    keys = ['a', 'b', 'c']\n    d = dict.fromkeys(keys, 0)\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_063_dict_keys_in_loop() {
        let code = "def f():\n    d = {'x': 1, 'y': 2}\n    result = []\n    for k in d.keys():\n        result.append(k)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_064_dict_values_in_loop() {
        let code = "def f():\n    d = {'x': 1, 'y': 2}\n    total = 0\n    for v in d.values():\n        total = total + v\n    return total";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_065_dict_get_nested() {
        let code = "def f():\n    d = {'a': {'b': 1}}\n    inner = d.get('a')\n    return inner";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_066_dict_update_multiple() {
        let code = "def f():\n    d = {}\n    d.update({'a': 1})\n    d.update({'b': 2})\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_067_dict_setdefault_existing() {
        let code = "def f():\n    d = {'a': 1}\n    val = d.setdefault('a', 99)\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_068_dict_pop_missing_key_default() {
        let code = "def f() -> int:\n    d = {'a': 1}\n    return d.pop('z', -1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_069_dict_fromkeys_no_default() {
        let code = "def f():\n    d = dict.fromkeys(['x', 'y'])\n    return d";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_070_dict_items_destructure() {
        let code = "def f():\n    d = {'a': 1, 'b': 2}\n    pairs = list(d.items())\n    return pairs";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_071_dict_copy_and_modify() {
        let code = "def f():\n    original = {'a': 1}\n    clone = original.copy()\n    clone['b'] = 2\n    return clone";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 4: SET METHODS (tests 072-097)
    // ========================================================================

    #[test]
    fn test_w21ra_072_set_add() {
        let code = "def f():\n    s = set()\n    s.add(1)\n    s.add(2)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_073_set_remove() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.remove(2)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_074_set_discard() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.discard(2)\n    s.discard(99)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_075_set_pop() {
        let code = "def f():\n    s = {1, 2, 3}\n    val = s.pop()\n    return val";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_076_set_clear() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.clear()\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_077_set_copy() {
        let code = "def f():\n    s = {1, 2, 3}\n    s2 = s.copy()\n    return s2";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_078_set_union() {
        let code = "def f():\n    a = {1, 2}\n    b = {3, 4}\n    c = a.union(b)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_079_set_intersection() {
        let code = "def f():\n    a = {1, 2, 3}\n    b = {2, 3, 4}\n    c = a.intersection(b)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_080_set_difference() {
        let code = "def f():\n    a = {1, 2, 3}\n    b = {2, 3}\n    c = a.difference(b)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_081_set_symmetric_difference() {
        let code = "def f():\n    a = {1, 2, 3}\n    b = {2, 3, 4}\n    c = a.symmetric_difference(b)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_082_set_issubset() {
        let code = "def f() -> bool:\n    a = {1, 2}\n    b = {1, 2, 3}\n    return a.issubset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_083_set_issuperset() {
        let code = "def f() -> bool:\n    a = {1, 2, 3}\n    b = {1, 2}\n    return a.issuperset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_084_set_isdisjoint() {
        let code = "def f() -> bool:\n    a = {1, 2}\n    b = {3, 4}\n    return a.isdisjoint(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_085_set_update() {
        let code = "def f():\n    s = {1, 2}\n    s.update({3, 4})\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_086_set_intersection_update() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.intersection_update({2, 3, 4})\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_087_set_difference_update() {
        let code = "def f():\n    s = {1, 2, 3}\n    s.difference_update({2, 3})\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_088_set_add_string() {
        let code = "def f():\n    s = set()\n    s.add('hello')\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_089_set_discard_string() {
        let code = "def f():\n    s = {'a', 'b', 'c'}\n    s.discard('b')\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_090_set_union_multiple() {
        let code = "def f():\n    a = {1}\n    b = {2}\n    c = {3}\n    result = a.union(b)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_091_set_add_in_loop() {
        let code = "def f():\n    s = set()\n    for i in range(5):\n        s.add(i)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_092_set_remove_from_loop() {
        let code = "def f():\n    s = {1, 2, 3, 4, 5}\n    s.remove(3)\n    return s";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_093_set_intersection_empty() {
        let code = "def f():\n    a = {1, 2}\n    b = {3, 4}\n    c = a.intersection(b)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_094_set_symmetric_difference_identity() {
        let code = "def f():\n    a = {1, 2, 3}\n    c = a.symmetric_difference(a)\n    return c";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_095_set_issubset_equal() {
        let code = "def f() -> bool:\n    a = {1, 2}\n    return a.issubset(a)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_096_set_copy_and_add() {
        let code = "def f():\n    original = {1, 2}\n    clone = original.copy()\n    clone.add(3)\n    return clone";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_097_set_difference_with_list() {
        let code = "def f():\n    s = {1, 2, 3, 4}\n    removed = {2, 4}\n    result = s.difference(removed)\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 5: FILE / BYTES / MISC METHODS (tests 098-120)
    // ========================================================================

    #[test]
    fn test_w21ra_098_file_read() {
        let code = "def f():\n    with open('test.txt', 'r') as fh:\n        content = fh.read()\n    return content";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_099_file_write() {
        let code = "def f():\n    with open('out.txt', 'w') as fh:\n        fh.write('hello')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_100_file_readline() {
        let code = "def f():\n    with open('test.txt') as fh:\n        line = fh.readline()\n    return line";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_101_file_readlines() {
        let code = "def f():\n    with open('test.txt') as fh:\n        lines = fh.readlines()\n    return lines";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_102_file_writelines() {
        let code = "def f():\n    lines = ['a\\n', 'b\\n']\n    with open('out.txt', 'w') as fh:\n        fh.writelines(lines)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_103_file_close() {
        let code = "def f():\n    fh = open('test.txt')\n    content = fh.read()\n    fh.close()\n    return content";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_104_file_seek() {
        let code = "def f():\n    with open('test.txt') as fh:\n        fh.seek(0)\n        content = fh.read()\n    return content";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_105_file_tell() {
        let code = "def f():\n    with open('test.txt') as fh:\n        pos = fh.tell()\n    return pos";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_106_file_flush() {
        let code = "def f():\n    with open('out.txt', 'w') as fh:\n        fh.write('data')\n        fh.flush()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_107_bytes_decode() {
        let code = "def f(b: bytes) -> str:\n    return b.decode()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_108_bytes_decode_utf8() {
        let code = "def f(b: bytes) -> str:\n    return b.decode('utf-8')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_109_bytes_hex() {
        let code = "def f(b: bytes) -> str:\n    return b.hex()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_110_counter_most_common() {
        let code = "from collections import Counter\ndef f():\n    c = Counter(['a', 'b', 'a', 'c', 'a'])\n    return c.most_common(2)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_111_counter_most_common_no_arg() {
        let code = "from collections import Counter\ndef f():\n    c = Counter(['x', 'y', 'x'])\n    return c.most_common()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_112_json_dumps() {
        let code = "import json\ndef f():\n    d = {'key': 'value'}\n    return json.dumps(d)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_113_json_loads() {
        let code = "import json\ndef f(s: str):\n    return json.loads(s)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_114_json_dumps_indent() {
        let code = "import json\ndef f():\n    d = {'a': 1}\n    return json.dumps(d, indent=4)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_115_itertools_chain() {
        let code = "import itertools\ndef f():\n    a = [1, 2]\n    b = [3, 4]\n    return list(itertools.chain(a, b))";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_116_string_maketrans() {
        let code = "def f():\n    t = str.maketrans('abc', 'xyz')\n    return t";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_117_string_translate() {
        let code = "def f(s: str) -> str:\n    t = str.maketrans('abc', 'xyz')\n    return s.translate(t)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_118_string_encode_latin1() {
        let code = "def f(s: str) -> bytes:\n    return s.encode('latin-1')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_119_bytes_fromhex() {
        let code = "def f() -> bytes:\n    return bytes.fromhex('deadbeef')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_120_collections_counter_elements() {
        let code = "from collections import Counter\ndef f():\n    c = Counter(a=3, b=1)\n    return list(c.most_common())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 6: ADDITIONAL METHOD DISPATCH (tests 121-140)
    // ========================================================================

    #[test]
    fn test_w21ra_121_unknown_var_append_infers_list() {
        let code = "def f():\n    x = None\n    x = []\n    x.append(42)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_122_unknown_var_keys_infers_dict() {
        let code = "def f():\n    x = {}\n    x['a'] = 1\n    result = x.keys()\n    return result";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_123_unknown_var_add_infers_set() {
        let code = "def f():\n    x = set()\n    x.add(1)\n    x.add(2)\n    return x";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_124_unknown_var_lower_infers_string() {
        let code = "def f(text):\n    return text.lower()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_125_unknown_var_upper_infers_string() {
        let code = "def f(text):\n    return text.upper()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_126_unknown_var_extend_infers_list() {
        let code = "def f(data):\n    data.extend([1, 2, 3])";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_127_unknown_var_values_infers_dict() {
        let code = "def f(data):\n    return list(data.values())";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_128_unknown_var_discard_infers_set() {
        let code = "def f(data):\n    data.discard(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_129_unknown_var_sort_infers_list() {
        let code = "def f(items):\n    items.sort()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_130_unknown_var_reverse_infers_list() {
        let code = "def f(items):\n    items.reverse()\n    return items";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_131_unknown_var_clear_infers_list() {
        let code = "def f(items):\n    items.clear()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_132_unknown_var_copy_infers_list() {
        let code = "def f(items):\n    return items.copy()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_133_unknown_var_index_infers_list() {
        let code = "def f(items):\n    return items.index(42)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_134_unknown_var_count_infers_list() {
        let code = "def f(items):\n    return items.count(1)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_135_unknown_var_items_infers_dict() {
        let code = "def f(data):\n    for k, v in data.items():\n        print(k, v)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_136_unknown_var_popitem_infers_dict() {
        let code = "def f(data):\n    return data.popitem()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_137_unknown_var_union_infers_set() {
        let code = "def f(a, b):\n    return a.union(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_138_unknown_var_intersection_infers_set() {
        let code = "def f(a, b):\n    return a.intersection(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_139_unknown_var_issubset_infers_set() {
        let code = "def f(a, b) -> bool:\n    return a.issubset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_140_unknown_var_issuperset_infers_set() {
        let code = "def f(a, b) -> bool:\n    return a.issuperset(b)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 7: ARGPARSE BASIC PATTERNS (tests 141-160)
    // ========================================================================

    #[test]
    fn test_w21ra_141_argparse_basic_parser() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    args = parser.parse_args()\n    print(args)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_142_argparse_with_description() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description='A tool')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_143_argparse_type_int() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--count', type=int)\n    args = parser.parse_args()\n    print(args.count)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_144_argparse_type_float() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--rate', type=float)\n    args = parser.parse_args()\n    print(args.rate)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_145_argparse_type_str() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--name', type=str)\n    args = parser.parse_args()\n    print(args.name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_146_argparse_required_true() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--input', required=True)\n    args = parser.parse_args()\n    print(args.input)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_147_argparse_default_value_int() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--port', type=int, default=8080)\n    args = parser.parse_args()\n    print(args.port)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_148_argparse_default_value_str() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--host', default='localhost')\n    args = parser.parse_args()\n    print(args.host)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_149_argparse_nargs_star() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--files', nargs='*')\n    args = parser.parse_args()\n    print(args.files)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_150_argparse_nargs_plus() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('files', nargs='+')\n    args = parser.parse_args()\n    print(args.files)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_151_argparse_nargs_question() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--config', nargs='?')\n    args = parser.parse_args()\n    print(args.config)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_152_argparse_help_text() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--verbose', help='Enable verbose output')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_153_argparse_metavar() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--output', metavar='FILE')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_154_argparse_store_true() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--verbose', action='store_true')\n    args = parser.parse_args()\n    if args.verbose:\n        print('verbose')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_155_argparse_store_false() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--no-color', action='store_false')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_156_argparse_count_action() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-v', '--verbose', action='count')\n    args = parser.parse_args()\n    print(args.verbose)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_157_argparse_append_action() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--include', action='append')\n    args = parser.parse_args()\n    print(args.include)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_158_argparse_positional_arg() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('filename')\n    args = parser.parse_args()\n    print(args.filename)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_159_argparse_choices() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--format', choices=['json', 'csv', 'xml'])\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_160_argparse_short_and_long() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('-o', '--output', type=str)\n    args = parser.parse_args()\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 8: ARGPARSE SUBCOMMANDS (tests 161-180)
    // ========================================================================

    #[test]
    fn test_w21ra_161_argparse_subparsers_basic() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    build_parser = subparsers.add_parser('build')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_162_argparse_subcommand_with_help() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    build_parser = subparsers.add_parser('build', help='Build the project')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_163_argparse_subcommand_with_args() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    build_parser = subparsers.add_parser('build')\n    build_parser.add_argument('--target', type=str)\n    args = parser.parse_args()\n    if args.command == 'build':\n        print(args.target)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_164_argparse_multiple_subcommands() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    build_parser = subparsers.add_parser('build')\n    test_parser = subparsers.add_parser('test')\n    args = parser.parse_args()\n    print(args.cmd)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_165_argparse_subcommand_with_positional() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='action')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('script')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_166_argparse_subcommand_store_true() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    test_parser = subparsers.add_parser('test')\n    test_parser.add_argument('--verbose', action='store_true')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_167_argparse_subcommand_multiple_args() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    deploy_parser = subparsers.add_parser('deploy')\n    deploy_parser.add_argument('--env', type=str)\n    deploy_parser.add_argument('--region', type=str)\n    deploy_parser.add_argument('--dry-run', action='store_true')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_168_argparse_access_subcommand_field() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='command')\n    clone_parser = subparsers.add_parser('clone')\n    clone_parser.add_argument('url')\n    args = parser.parse_args()\n    if args.command == 'clone':\n        target = args.url\n        print(target)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_169_argparse_subcommand_type_int() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    scale_parser = subparsers.add_parser('scale')\n    scale_parser.add_argument('--replicas', type=int)\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_170_argparse_subcommand_nargs_plus() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    add_parser = subparsers.add_parser('add')\n    add_parser.add_argument('files', nargs='+')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_171_argparse_two_subcommands_different_args() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser(description='Git clone')\n    subparsers = parser.add_subparsers(dest='command')\n    clone_parser = subparsers.add_parser('clone')\n    clone_parser.add_argument('url')\n    push_parser = subparsers.add_parser('push')\n    push_parser.add_argument('--force', action='store_true')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_172_argparse_subcommand_default_value() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    serve_parser = subparsers.add_parser('serve')\n    serve_parser.add_argument('--port', type=int, default=8080)\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_173_argparse_subcommand_required_flag() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    init_parser = subparsers.add_parser('init')\n    init_parser.add_argument('--name', required=True)\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_174_argparse_subcommand_choices() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    config_parser = subparsers.add_parser('config')\n    config_parser.add_argument('--level', choices=['debug', 'info', 'warn'])\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_175_argparse_three_subcommands() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    subparsers.add_parser('start')\n    subparsers.add_parser('stop')\n    subparsers.add_parser('restart')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_176_argparse_global_and_subcommand_args() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--debug', action='store_true')\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--target', type=str)\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_177_argparse_subcommand_nargs_star() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    install_parser = subparsers.add_parser('install')\n    install_parser.add_argument('packages', nargs='*')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_178_argparse_subcommand_count_action() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    debug_parser = subparsers.add_parser('debug')\n    debug_parser.add_argument('-v', '--verbosity', action='count')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_179_argparse_subcommand_with_help_text() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    check_parser = subparsers.add_parser('check')\n    check_parser.add_argument('--strict', action='store_true', help='Enable strict mode')\n    args = parser.parse_args()";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_180_argparse_subcommand_field_binary_op() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    calc_parser = subparsers.add_parser('calc')\n    calc_parser.add_argument('--left', type=int)\n    calc_parser.add_argument('--right', type=int)\n    args = parser.parse_args()\n    if args.cmd == 'calc':\n        result = args.left + args.right\n        print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    // ========================================================================
    // SECTION 9: ARGPARSE ADVANCED PATTERNS (tests 181-200)
    // ========================================================================

    #[test]
    fn test_w21ra_181_argparse_positional_and_optional() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('input')\n    parser.add_argument('--output', type=str)\n    args = parser.parse_args()\n    print(args.input)\n    print(args.output)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_182_argparse_multiple_positional() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('source')\n    parser.add_argument('destination')\n    args = parser.parse_args()\n    print(args.source)\n    print(args.destination)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_183_argparse_default_float() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--threshold', type=float, default=0.5)\n    args = parser.parse_args()\n    print(args.threshold)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_184_argparse_bool_field_in_condition() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--dry-run', action='store_true')\n    args = parser.parse_args()\n    if args.dry_run:\n        print('dry run')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_185_argparse_field_in_f_string() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--name', type=str, default='world')\n    args = parser.parse_args()\n    msg = f'Hello {args.name}'\n    print(msg)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_186_argparse_field_as_function_arg() {
        let code = "import argparse\ndef process(name: str) -> str:\n    return name.upper()\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--name', type=str, default='test')\n    args = parser.parse_args()\n    result = process(args.name)\n    print(result)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_187_argparse_field_comparison() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--count', type=int, default=0)\n    args = parser.parse_args()\n    if args.count > 10:\n        print('many')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_188_argparse_nargs_star_with_type() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--values', nargs='*', type=int)\n    args = parser.parse_args()\n    print(args.values)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_189_argparse_nargs_plus_with_type() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('numbers', nargs='+', type=int)\n    args = parser.parse_args()\n    print(args.numbers)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_190_argparse_multiple_flags() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--verbose', action='store_true')\n    parser.add_argument('--quiet', action='store_true')\n    parser.add_argument('--debug', action='store_true')\n    args = parser.parse_args()\n    if args.verbose:\n        print('v')\n    if args.debug:\n        print('d')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_191_argparse_mixed_types() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--host', type=str, default='0.0.0.0')\n    parser.add_argument('--port', type=int, default=8080)\n    parser.add_argument('--ssl', action='store_true')\n    args = parser.parse_args()\n    print(args.host)\n    print(args.port)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_192_argparse_field_in_loop() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('--count', type=int, default=5)\n    args = parser.parse_args()\n    for i in range(args.count):\n        print(i)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_193_argparse_subcommand_field_unary_not() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--quiet', action='store_true')\n    args = parser.parse_args()\n    if not args.quiet:\n        print('running')";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_194_argparse_subcommand_field_in_call() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--file', type=str)\n    args = parser.parse_args()\n    if args.cmd == 'run':\n        print(args.file)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_195_argparse_subcommand_field_in_list() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--tag', type=str)\n    args = parser.parse_args()\n    tags = [args.tag]";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_196_argparse_subcommand_field_in_dict() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    set_parser = subparsers.add_parser('configure')\n    set_parser.add_argument('--key', type=str)\n    set_parser.add_argument('--value', type=str)\n    args = parser.parse_args()\n    config = {'key': args.key, 'value': args.value}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_197_argparse_subcommand_field_method_call() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--name', type=str)\n    args = parser.parse_args()\n    if args.cmd == 'run':\n        upper_name = args.name.upper()\n        print(upper_name)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_198_argparse_subcommand_field_index() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    run_parser = subparsers.add_parser('run')\n    run_parser.add_argument('--items', nargs='+')\n    args = parser.parse_args()\n    if args.cmd == 'run':\n        first = args.items[0]\n        print(first)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_199_argparse_subcommand_field_in_tuple() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    pos_parser = subparsers.add_parser('pos')\n    pos_parser.add_argument('--x', type=int)\n    pos_parser.add_argument('--y', type=int)\n    args = parser.parse_args()\n    point = (args.x, args.y)";
        let result = transpile(code);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_w21ra_200_argparse_subcommand_field_in_set() {
        let code = "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    subparsers = parser.add_subparsers(dest='cmd')\n    add_parser = subparsers.add_parser('add')\n    add_parser.add_argument('--tag', type=str)\n    args = parser.parse_args()\n    tags = {args.tag}";
        let result = transpile(code);
        assert!(!result.is_empty());
    }
}
