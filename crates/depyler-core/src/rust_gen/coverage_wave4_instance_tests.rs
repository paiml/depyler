//! Coverage wave 4: expr_gen_instance_methods submodule coverage boost tests
//!
//! Targets uncovered branches in: method_call_routing.rs, lambda_generators.rs,
//! type_helpers.rs, comprehensions.rs, constructors.rs, dict_constructors.rs,
//! instance_dispatch.rs, attribute_convert.rs

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

// =============================================================================
// Section 1: method_call_routing.rs - Unknown/DepylerValue type inference
// =============================================================================

#[test]
fn test_mcr_unknown_var_append_type_inference() {
    let code = transpile(
        "def process(items):\n    items.append(42)\n    return items",
    );
    assert!(!code.is_empty(), "append on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_insert_type_inference() {
    let code = transpile(
        "def process(items):\n    items.insert(0, 'hello')\n    return items",
    );
    assert!(!code.is_empty(), "insert on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_extend_type_inference() {
    let code = transpile(
        "def process(items):\n    items.extend([1, 2])\n    return items",
    );
    assert!(!code.is_empty(), "extend on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_pop_type_inference() {
    let code = transpile(
        "def process(items):\n    return items.pop()",
    );
    assert!(!code.is_empty(), "pop on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_remove_type_inference() {
    let code = transpile(
        "def process(items):\n    items.remove(5)\n    return items",
    );
    assert!(!code.is_empty(), "remove on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_sort_type_inference() {
    let code = transpile(
        "def process(items):\n    items.sort()\n    return items",
    );
    assert!(!code.is_empty(), "sort on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_reverse_type_inference() {
    let code = transpile(
        "def process(items):\n    items.reverse()\n    return items",
    );
    assert!(!code.is_empty(), "reverse on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_clear_type_inference() {
    let code = transpile(
        "def process(items):\n    items.clear()\n    return items",
    );
    assert!(!code.is_empty(), "clear on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_copy_type_inference() {
    let code = transpile(
        "def process(items):\n    return items.copy()",
    );
    assert!(!code.is_empty(), "copy on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_index_type_inference() {
    let code = transpile(
        "def process(items):\n    return items.index(5)",
    );
    assert!(!code.is_empty(), "index on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_count_type_inference() {
    let code = transpile(
        "def process(items):\n    return items.count(5)",
    );
    assert!(!code.is_empty(), "count on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_lower_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.lower()",
    );
    assert!(!code.is_empty(), "lower on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_upper_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.upper()",
    );
    assert!(!code.is_empty(), "upper on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_strip_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.strip()",
    );
    assert!(!code.is_empty(), "strip on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_split_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.split(',')",
    );
    assert!(!code.is_empty(), "split on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_join_string_inference() {
    let code = transpile(
        "def process(sep):\n    return sep.join(['a', 'b', 'c'])",
    );
    assert!(!code.is_empty(), "join on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_replace_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.replace('a', 'b')",
    );
    assert!(!code.is_empty(), "replace on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_startswith_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.startswith('hello')",
    );
    assert!(!code.is_empty(), "startswith on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_endswith_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.endswith('world')",
    );
    assert!(!code.is_empty(), "endswith on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_find_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.find('x')",
    );
    assert!(!code.is_empty(), "find on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_encode_string_inference() {
    let code = transpile(
        "def process(text):\n    return text.encode()",
    );
    assert!(!code.is_empty(), "encode on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_keys_dict_inference() {
    let code = transpile(
        "def process(data):\n    return data.keys()",
    );
    assert!(!code.is_empty(), "keys on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_values_dict_inference() {
    let code = transpile(
        "def process(data):\n    return data.values()",
    );
    assert!(!code.is_empty(), "values on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_items_dict_inference() {
    let code = transpile(
        "def process(data):\n    return data.items()",
    );
    assert!(!code.is_empty(), "items on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_get_dict_inference() {
    let code = transpile(
        "def process(data):\n    return data.get('key', 'default')",
    );
    assert!(!code.is_empty(), "get on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_setdefault_dict_inference() {
    let code = transpile(
        "def process(data):\n    return data.setdefault('key', 0)",
    );
    assert!(!code.is_empty(), "setdefault on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_update_dict_inference() {
    let code = transpile(
        "def process(data):\n    data.update({'key': 'val'})\n    return data",
    );
    assert!(!code.is_empty(), "update on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_iter_inference() {
    let code = transpile(
        "def process(items):\n    for x in items.iter():\n        pass",
    );
    assert!(!code.is_empty(), "iter on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_add_set_inference() {
    let code = transpile(
        "def process(items):\n    items.add(42)\n    return items",
    );
    assert!(!code.is_empty(), "add on unknown var (set): {}", code);
}

#[test]
fn test_mcr_unknown_var_discard_set_inference() {
    let code = transpile(
        "def process(items):\n    items.discard(42)\n    return items",
    );
    assert!(!code.is_empty(), "discard on unknown var (set): {}", code);
}

#[test]
fn test_mcr_unknown_var_intersection_set_inference() {
    let code = transpile(
        "def process(a):\n    b = {1, 2, 3}\n    return a.intersection(b)",
    );
    assert!(!code.is_empty(), "intersection on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_union_set_inference() {
    let code = transpile(
        "def process(a):\n    b = {1, 2, 3}\n    return a.union(b)",
    );
    assert!(!code.is_empty(), "union on unknown var: {}", code);
}

#[test]
fn test_mcr_unknown_var_issubset_set_inference() {
    let code = transpile(
        "def process(a):\n    b = {1, 2, 3}\n    return a.issubset(b)",
    );
    assert!(!code.is_empty(), "issubset on unknown var: {}", code);
}

// =============================================================================
// Section 2: method_call_routing.rs - Counter.most_common, hasher, asyncio
// =============================================================================

#[test]
fn test_mcr_counter_most_common_with_n() {
    let code = transpile(
        "def top_words(words: list):\n    from collections import Counter\n    counts = Counter(words)\n    return counts.most_common(5)",
    );
    assert!(!code.is_empty(), "Counter.most_common(n): {}", code);
}

#[test]
fn test_mcr_counter_most_common_no_arg() {
    let code = transpile(
        "def all_counts(words: list):\n    from collections import Counter\n    counts = Counter(words)\n    return counts.most_common()",
    );
    assert!(!code.is_empty(), "Counter.most_common(): {}", code);
}

#[test]
fn test_mcr_hasher_hexdigest() {
    let code = transpile(
        "def hash_data(hasher, data: str) -> str:\n    hasher.update(data)\n    return hasher.hexdigest()",
    );
    assert!(!code.is_empty(), "hasher hexdigest: {}", code);
}

#[test]
fn test_mcr_dict_fromkeys() {
    let code = transpile(
        "def make_dict(keys: list) -> dict:\n    return dict.fromkeys(keys, 0)",
    );
    assert!(!code.is_empty(), "dict.fromkeys: {}", code);
}

#[test]
fn test_mcr_dict_fromkeys_no_default() {
    let code = transpile(
        "def make_dict(keys: list) -> dict:\n    return dict.fromkeys(keys)",
    );
    assert!(!code.is_empty(), "dict.fromkeys no default: {}", code);
}

#[test]
fn test_mcr_int_from_bytes_big() {
    let code = transpile(
        "def decode_int(data: bytes) -> int:\n    return int.from_bytes(data, 'big')",
    );
    assert!(!code.is_empty(), "int.from_bytes big: {}", code);
}

#[test]
fn test_mcr_int_from_bytes_little() {
    let code = transpile(
        "def decode_int(data: bytes) -> int:\n    return int.from_bytes(data, 'little')",
    );
    assert!(!code.is_empty(), "int.from_bytes little: {}", code);
}

#[test]
fn test_mcr_asyncio_sleep() {
    let code = transpile(
        "import asyncio\ndef wait():\n    asyncio.sleep(1.5)",
    );
    assert!(!code.is_empty(), "asyncio.sleep: {}", code);
}

#[test]
fn test_mcr_asyncio_run() {
    let code = transpile(
        "import asyncio\ndef main():\n    asyncio.run(None)",
    );
    assert!(!code.is_empty(), "asyncio.run: {}", code);
}

#[test]
fn test_mcr_colorsys_rgb_to_hsv() {
    let code = transpile(
        "import colorsys\ndef convert(r: float, g: float, b: float):\n    return colorsys.rgb_to_hsv(r, g, b)",
    );
    assert!(!code.is_empty(), "colorsys.rgb_to_hsv: {}", code);
}

#[test]
fn test_mcr_colorsys_hsv_to_rgb() {
    let code = transpile(
        "import colorsys\ndef convert(h: float, s: float, v: float):\n    return colorsys.hsv_to_rgb(h, s, v)",
    );
    assert!(!code.is_empty(), "colorsys.hsv_to_rgb: {}", code);
}

#[test]
fn test_mcr_colorsys_rgb_to_hls() {
    let code = transpile(
        "import colorsys\ndef convert(r: float, g: float, b: float):\n    return colorsys.rgb_to_hls(r, g, b)",
    );
    assert!(!code.is_empty(), "colorsys.rgb_to_hls: {}", code);
}

#[test]
fn test_mcr_colorsys_hls_to_rgb() {
    let code = transpile(
        "import colorsys\ndef convert(h: float, l: float, s: float):\n    return colorsys.hls_to_rgb(h, l, s)",
    );
    assert!(!code.is_empty(), "colorsys.hls_to_rgb: {}", code);
}

// =============================================================================
// Section 3: method_call_routing.rs - static methods, module methods
// =============================================================================

#[test]
fn test_mcr_static_method_class_call() {
    let code = transpile(
        "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def origin():\n        return Point(0, 0)\n\ndef test():\n    p = Point.origin()\n    return p",
    );
    assert!(!code.is_empty(), "static method call: {}", code);
}

#[test]
fn test_mcr_string_method_on_non_module_var() {
    let code = transpile(
        "def process(msg: str) -> str:\n    return msg.title()",
    );
    assert!(!code.is_empty(), "title on str var: {}", code);
}

#[test]
fn test_mcr_string_center() {
    let code = transpile(
        "def format_text(text: str) -> str:\n    return text.center(20)",
    );
    assert!(!code.is_empty(), "str.center: {}", code);
}

#[test]
fn test_mcr_string_ljust() {
    let code = transpile(
        "def format_text(text: str) -> str:\n    return text.ljust(20)",
    );
    assert!(!code.is_empty(), "str.ljust: {}", code);
}

#[test]
fn test_mcr_string_rjust() {
    let code = transpile(
        "def format_text(text: str) -> str:\n    return text.rjust(20)",
    );
    assert!(!code.is_empty(), "str.rjust: {}", code);
}

#[test]
fn test_mcr_string_zfill() {
    let code = transpile(
        "def pad_num(num: str) -> str:\n    return num.zfill(5)",
    );
    assert!(!code.is_empty(), "str.zfill: {}", code);
}

#[test]
fn test_mcr_string_isdigit() {
    let code = transpile(
        "def check(s: str) -> bool:\n    return s.isdigit()",
    );
    assert!(!code.is_empty(), "str.isdigit: {}", code);
}

#[test]
fn test_mcr_string_isalpha() {
    let code = transpile(
        "def check(s: str) -> bool:\n    return s.isalpha()",
    );
    assert!(!code.is_empty(), "str.isalpha: {}", code);
}

#[test]
fn test_mcr_string_isalnum() {
    let code = transpile(
        "def check(s: str) -> bool:\n    return s.isalnum()",
    );
    assert!(!code.is_empty(), "str.isalnum: {}", code);
}

// =============================================================================
// Section 4: lambda_generators.rs
// =============================================================================

#[test]
fn test_lg_lambda_with_captured_string_var() {
    let code = transpile(
        "def make_greeter(greeting: str):\n    return lambda name: greeting + ' ' + name",
    );
    assert!(!code.is_empty(), "lambda captures string: {}", code);
}

#[test]
fn test_lg_lambda_with_captured_list_var() {
    let code = transpile(
        "def make_checker(allowed: list):\n    return lambda x: x in allowed",
    );
    assert!(!code.is_empty(), "lambda captures list: {}", code);
}

#[test]
fn test_lg_lambda_nested_body() {
    let code = transpile(
        "def apply():\n    double = lambda x: x * 2\n    return double(5)",
    );
    assert!(!code.is_empty(), "lambda nested body: {}", code);
}

#[test]
fn test_lg_fstring_with_collection_debug_format() {
    let code = transpile(
        "def show_items(items: list) -> str:\n    return f'Items: {items}'",
    );
    assert!(
        !code.is_empty(),
        "fstring with collection debug: {}",
        code
    );
}

#[test]
fn test_lg_fstring_with_multiple_vars() {
    let code = transpile(
        "def greeting(name: str, age: int) -> str:\n    return f'Hello {name}, you are {age} years old'",
    );
    assert!(!code.is_empty(), "fstring with multiple vars: {}", code);
}

#[test]
fn test_lg_fstring_method_call_result() {
    let code = transpile(
        "def show_split(text: str) -> str:\n    return f'Parts: {text.split()}'",
    );
    assert!(!code.is_empty(), "fstring with method result: {}", code);
}

#[test]
fn test_lg_ternary_basic() {
    let code = transpile(
        "def choose(cond: bool, a: int, b: int) -> int:\n    return a if cond else b",
    );
    assert!(!code.is_empty(), "ternary basic: {}", code);
}

#[test]
fn test_lg_ternary_float_int_coercion() {
    let code = transpile(
        "def safe_div(a: float, b: float) -> float:\n    return a / b if b != 0 else 0",
    );
    assert!(!code.is_empty(), "ternary float coercion: {}", code);
}

#[test]
fn test_lg_ternary_string_truthiness() {
    let code = transpile(
        "def get_name(name: str) -> str:\n    return name if name else 'anonymous'",
    );
    assert!(!code.is_empty(), "ternary string truthiness: {}", code);
}

#[test]
fn test_lg_ternary_list_truthiness() {
    let code = transpile(
        "def get_first(items: list) -> int:\n    return items[0] if items else -1",
    );
    assert!(!code.is_empty(), "ternary list truthiness: {}", code);
}

#[test]
fn test_lg_ternary_optional_truthiness() {
    let code = transpile(
        "def maybe_val(x: int = None) -> int:\n    return x if x else 0",
    );
    assert!(!code.is_empty(), "ternary optional truthiness: {}", code);
}

#[test]
fn test_lg_ternary_identity_pattern() {
    let code = transpile(
        "def identity_or_default(val: str) -> str:\n    return val if val else 'default'",
    );
    assert!(!code.is_empty(), "ternary identity pattern: {}", code);
}

#[test]
fn test_lg_sorted_identity_key() {
    let code = transpile(
        "def sort_nums(nums: list) -> list:\n    return sorted(nums)",
    );
    assert!(!code.is_empty(), "sorted identity key: {}", code);
}

#[test]
fn test_lg_sorted_with_key_function() {
    let code = transpile(
        "def sort_by_abs(nums: list) -> list:\n    return sorted(nums, key=lambda x: abs(x))",
    );
    assert!(!code.is_empty(), "sorted with key: {}", code);
}

#[test]
fn test_lg_sorted_with_reverse() {
    let code = transpile(
        "def sort_desc(nums: list) -> list:\n    return sorted(nums, reverse=True)",
    );
    assert!(!code.is_empty(), "sorted with reverse: {}", code);
}

#[test]
fn test_lg_generator_expression_basic() {
    let code = transpile(
        "def gen_squares(n: int) -> list:\n    return list(x * x for x in range(n))",
    );
    assert!(!code.is_empty(), "generator expression: {}", code);
}

#[test]
fn test_lg_generator_expression_with_filter() {
    let code = transpile(
        "def gen_evens(n: int) -> list:\n    return list(x for x in range(n) if x % 2 == 0)",
    );
    assert!(!code.is_empty(), "generator with filter: {}", code);
}

#[test]
fn test_lg_walrus_operator() {
    let code = transpile(
        "def find_long(words: list) -> list:\n    return [w for w in words if len(w) > 3]",
    );
    assert!(!code.is_empty(), "walrus-like filter: {}", code);
}

#[test]
fn test_lg_yield_inside_generator() {
    let code = transpile(
        "def counter(n: int):\n    i = 0\n    while i < n:\n        yield i\n        i += 1",
    );
    assert!(!code.is_empty(), "yield in generator: {}", code);
}

#[test]
fn test_lg_named_expr_walrus() {
    let code = transpile(
        "def process(data: list) -> list:\n    return [y for x in data if (y := x * 2) > 5]",
    );
    assert!(!code.is_empty(), "walrus named expr: {}", code);
}

// =============================================================================
// Section 5: type_helpers.rs
// =============================================================================

#[test]
fn test_th_is_string_index_with_string_literal() {
    let code = transpile(
        "def get_val(config: dict) -> str:\n    return config['name']",
    );
    assert!(code.contains("get"), "string index dict access: {}", code);
}

#[test]
fn test_th_is_string_index_with_int_literal() {
    let code = transpile(
        "def get_item(items: list) -> int:\n    return items[0]",
    );
    assert!(!code.is_empty(), "int index list access: {}", code);
}

#[test]
fn test_th_is_numeric_index_arithmetic() {
    let code = transpile(
        "def get_middle(items: list) -> int:\n    return items[len(items) // 2]",
    );
    assert!(!code.is_empty(), "arithmetic index: {}", code);
}

#[test]
fn test_th_is_string_base_str_var() {
    let code = transpile(
        "def char_at(text: str, idx: int) -> str:\n    return text[idx]",
    );
    assert!(!code.is_empty(), "string base indexing: {}", code);
}

#[test]
fn test_th_is_dict_expr_literal() {
    let code = transpile(
        "def get_from_dict() -> str:\n    d = {'a': 1}\n    return d.keys()",
    );
    assert!(!code.is_empty(), "dict literal expr: {}", code);
}

#[test]
fn test_th_is_dict_expr_attribute() {
    let code = transpile(
        "class Config:\n    def __init__(self):\n        self.settings = {}\n    def get_keys(self):\n        return self.settings.keys()",
    );
    assert!(!code.is_empty(), "dict attribute expr: {}", code);
}

#[test]
fn test_th_is_set_expr_literal() {
    let code = transpile(
        "def make_set() -> set:\n    s = {1, 2, 3}\n    s.add(4)\n    return s",
    );
    assert!(!code.is_empty(), "set literal expr: {}", code);
}

#[test]
fn test_th_is_path_expr_variable() {
    let code = transpile(
        "from pathlib import Path\ndef get_name():\n    path = Path('/tmp/test.txt')\n    return path.name",
    );
    assert!(!code.is_empty(), "path expr variable: {}", code);
}

#[test]
fn test_th_is_string_type_method_call() {
    let code = transpile(
        "def process(text: str) -> list:\n    return list(text.upper())",
    );
    assert!(!code.is_empty(), "string type method call: {}", code);
}

#[test]
fn test_th_is_string_type_str_call() {
    let code = transpile(
        "def to_str(num: int) -> str:\n    return str(num)",
    );
    assert!(!code.is_empty(), "str() call: {}", code);
}

#[test]
fn test_th_is_list_expr_from_function() {
    let code = transpile(
        "def make_list() -> list:\n    return list(range(10))",
    );
    assert!(!code.is_empty(), "list expr from function: {}", code);
}

#[test]
fn test_th_is_tuple_base_detection() {
    let code = transpile(
        "def get_pair() -> int:\n    pair = (1, 2)\n    return pair[0]",
    );
    assert!(!code.is_empty(), "tuple base detection: {}", code);
}

#[test]
fn test_th_expr_is_option_attribute() {
    let code = transpile(
        "def check(x: int = None) -> bool:\n    return x is not None",
    );
    assert!(!code.is_empty(), "option expr check: {}", code);
}

// =============================================================================
// Section 6: comprehensions.rs
// =============================================================================

#[test]
fn test_comp_list_comp_with_filter() {
    let code = transpile(
        "def filter_pos(nums: list) -> list:\n    return [x for x in nums if x > 0]",
    );
    assert!(code.contains("filter"), "list comp with filter: {}", code);
}

#[test]
fn test_comp_list_comp_transform() {
    let code = transpile(
        "def double_list(nums: list) -> list:\n    return [x * 2 for x in nums]",
    );
    assert!(code.contains("map"), "list comp transform: {}", code);
}

#[test]
fn test_comp_nested_list_comp() {
    let code = transpile(
        "def flatten(matrix: list) -> list:\n    return [x for row in matrix for x in row]",
    );
    assert!(
        code.contains("flat_map"),
        "nested list comp: {}",
        code
    );
}

#[test]
fn test_comp_set_comp_basic() {
    let code = transpile(
        "def unique_squares(nums: list) -> set:\n    return {x * x for x in nums}",
    );
    assert!(code.contains("HashSet"), "set comp: {}", code);
}

#[test]
fn test_comp_set_comp_with_filter() {
    let code = transpile(
        "def even_set(nums: list) -> set:\n    return {x for x in nums if x % 2 == 0}",
    );
    assert!(code.contains("filter"), "set comp with filter: {}", code);
}

#[test]
fn test_comp_dict_comp_basic() {
    let code = transpile(
        "def make_squares(n: int) -> dict:\n    return {i: i * i for i in range(n)}",
    );
    assert!(code.contains("HashMap"), "dict comp: {}", code);
}

#[test]
fn test_comp_dict_comp_with_filter() {
    let code = transpile(
        "def even_squares(n: int) -> dict:\n    return {i: i * i for i in range(n) if i % 2 == 0}",
    );
    assert!(
        code.contains("filter"),
        "dict comp with filter: {}",
        code
    );
}

#[test]
fn test_comp_dict_comp_from_items() {
    let code = transpile(
        "def swap_dict(d: dict) -> dict:\n    return {v: k for k, v in d.items()}",
    );
    assert!(!code.is_empty(), "dict comp from items: {}", code);
}

// =============================================================================
// Section 7: constructors.rs
// =============================================================================

#[test]
fn test_cons_list_with_strings() {
    let code = transpile(
        "def make_list() -> list:\n    return ['hello', 'world']",
    );
    assert!(
        code.contains("to_string"),
        "list with strings: {}",
        code
    );
}

#[test]
fn test_cons_list_with_none_elements() {
    let code = transpile(
        "def make_optional_list() -> list:\n    return [1, None, 3]",
    );
    assert!(code.contains("Some") || code.contains("None"), "list with None: {}", code);
}

#[test]
fn test_cons_tuple_basic() {
    let code = transpile(
        "def make_pair(a: int, b: str):\n    return (a, b)",
    );
    assert!(!code.is_empty(), "tuple basic: {}", code);
}

#[test]
fn test_cons_set_with_strings() {
    let code = transpile(
        "def make_set() -> set:\n    return {'apple', 'banana', 'cherry'}",
    );
    assert!(
        code.contains("to_string"),
        "set with strings: {}",
        code
    );
}

#[test]
fn test_cons_frozenset_basic() {
    let code = transpile(
        "def make_frozen():\n    return frozenset([1, 2, 3])",
    );
    assert!(!code.is_empty(), "frozenset: {}", code);
}

#[test]
fn test_cons_list_empty() {
    let code = transpile(
        "def make_empty() -> list:\n    return []",
    );
    assert!(code.contains("vec!"), "empty list: {}", code);
}

#[test]
fn test_cons_set_empty() {
    let code = transpile(
        "def make_empty_set() -> set:\n    return set()",
    );
    assert!(!code.is_empty(), "empty set: {}", code);
}

#[test]
fn test_cons_tuple_with_string_literal() {
    let code = transpile(
        "def record() -> tuple:\n    return (1, 'hello', 3.14)",
    );
    assert!(
        code.contains("to_string"),
        "tuple with string literal: {}",
        code
    );
}

#[test]
fn test_cons_list_with_float_coercion() {
    let code = transpile(
        "def make_floats() -> list:\n    x: list = [0, 1, 2]\n    return x",
    );
    assert!(!code.is_empty(), "list float coercion: {}", code);
}

// =============================================================================
// Section 8: dict_constructors.rs
// =============================================================================

#[test]
fn test_dc_dict_with_string_keys_values() {
    let code = transpile(
        "def make_config() -> dict:\n    return {'host': 'localhost', 'port': '8080'}",
    );
    assert!(code.contains("to_string"), "dict string kv: {}", code);
}

#[test]
fn test_dc_dict_with_none_value() {
    let code = transpile(
        "def make_optional() -> dict:\n    return {'a': 1, 'b': None}",
    );
    assert!(code.contains("None"), "dict with None: {}", code);
}

#[test]
fn test_dc_dict_empty() {
    let code = transpile(
        "def make_empty() -> dict:\n    return {}",
    );
    assert!(code.contains("HashMap"), "empty dict: {}", code);
}

#[test]
fn test_dc_dict_nested() {
    let code = transpile(
        "def make_nested() -> dict:\n    return {'outer': {'inner': 1}}",
    );
    assert!(!code.is_empty(), "nested dict: {}", code);
}

#[test]
fn test_dc_dict_homogeneous_int() {
    let code = transpile(
        "def make_int_dict() -> dict:\n    return {'a': 1, 'b': 2, 'c': 3}",
    );
    assert!(code.contains("HashMap"), "homogeneous int dict: {}", code);
}

#[test]
fn test_dc_dict_with_list_values() {
    let code = transpile(
        "def make_list_dict() -> dict:\n    return {'items': [1, 2, 3], 'tags': ['a', 'b']}",
    );
    assert!(!code.is_empty(), "dict with list values: {}", code);
}

#[test]
fn test_dc_dict_result_function_value() {
    let code = transpile(
        "def make_computed() -> dict:\n    x = 10\n    y = 20\n    return {'sum': x + y, 'diff': x - y}",
    );
    assert!(!code.is_empty(), "dict computed values: {}", code);
}

// =============================================================================
// Section 9: instance_dispatch.rs - argparse, sys I/O, file, path, datetime
// =============================================================================

#[test]
fn test_id_parse_args() {
    let code = transpile(
        "import argparse\ndef main():\n    parser = argparse.ArgumentParser()\n    parser.add_argument('name')\n    args = parser.parse_args()\n    print(args.name)",
    );
    assert!(!code.is_empty(), "parse_args: {}", code);
}

#[test]
fn test_id_file_read_no_args() {
    let code = transpile(
        "def read_file(f) -> str:\n    return f.read()",
    );
    assert!(
        code.contains("read_to_string"),
        "file read(): {}",
        code
    );
}

#[test]
fn test_id_file_read_with_size() {
    let code = transpile(
        "def read_chunk(f, size: int) -> bytes:\n    return f.read(size)",
    );
    assert!(code.contains("read"), "file read(size): {}", code);
}

#[test]
fn test_id_file_readlines() {
    let code = transpile(
        "def get_lines(f) -> list:\n    return f.readlines()",
    );
    assert!(
        code.contains("BufReader"),
        "file readlines: {}",
        code
    );
}

#[test]
fn test_id_file_readline() {
    let code = transpile(
        "def get_line(f) -> str:\n    return f.readline()",
    );
    assert!(code.contains("read_line"), "file readline: {}", code);
}

#[test]
fn test_id_file_write() {
    let code = transpile(
        "def write_file(f, content: str):\n    f.write(content)",
    );
    assert!(
        code.contains("write_all"),
        "file write: {}",
        code
    );
}

#[test]
fn test_id_file_close() {
    let code = transpile(
        "def close_file(f):\n    f.close()",
    );
    assert!(!code.is_empty(), "file close: {}", code);
}

#[test]
fn test_id_path_stat() {
    let code = transpile(
        "def get_stat(path):\n    return path.stat()",
    );
    assert!(
        code.contains("metadata"),
        "path.stat(): {}",
        code
    );
}

#[test]
fn test_id_path_absolute() {
    let code = transpile(
        "def get_abs(path):\n    return path.absolute()",
    );
    assert!(
        code.contains("canonicalize"),
        "path.absolute(): {}",
        code
    );
}

#[test]
fn test_id_path_resolve() {
    let code = transpile(
        "def resolve_path(path):\n    return path.resolve()",
    );
    assert!(
        code.contains("canonicalize"),
        "path.resolve(): {}",
        code
    );
}

#[test]
fn test_id_datetime_isoformat() {
    let code = transpile(
        "def to_iso(dt) -> str:\n    return dt.isoformat()",
    );
    assert!(!code.is_empty(), "datetime isoformat: {}", code);
}

#[test]
fn test_id_datetime_strftime() {
    let code = transpile(
        "def format_date(dt) -> str:\n    return dt.strftime('%Y-%m-%d')",
    );
    assert!(!code.is_empty(), "datetime strftime: {}", code);
}

#[test]
fn test_id_datetime_timestamp() {
    let code = transpile(
        "def to_unix(dt) -> float:\n    return dt.timestamp()",
    );
    assert!(!code.is_empty(), "datetime timestamp: {}", code);
}

#[test]
fn test_id_csv_writerow() {
    let code = transpile(
        "def write_row(writer, row: dict):\n    writer.writerow(row)",
    );
    assert!(
        code.contains("serialize"),
        "csv writerow: {}",
        code
    );
}

#[test]
fn test_id_csv_writeheader() {
    let code = transpile(
        "def write_header(writer):\n    writer.writeheader()",
    );
    assert!(!code.is_empty(), "csv writeheader: {}", code);
}

#[test]
fn test_id_regex_group_zero() {
    let code = transpile(
        "import re\ndef get_match(text: str) -> str:\n    m = re.search('pattern', text)\n    return m.group(0)",
    );
    assert!(!code.is_empty(), "regex match group(0): {}", code);
}

#[test]
fn test_id_regex_group_n() {
    let code = transpile(
        "import re\ndef get_group(text: str) -> str:\n    m = re.search('(\\\\w+)', text)\n    return m.group(1)",
    );
    assert!(!code.is_empty(), "regex match group(n): {}", code);
}

#[test]
fn test_id_string_encode() {
    let code = transpile(
        "def to_bytes(text: str) -> bytes:\n    return text.encode()",
    );
    assert!(!code.is_empty(), "str.encode: {}", code);
}

#[test]
fn test_id_string_decode() {
    let code = transpile(
        "def from_bytes(data) -> str:\n    return data.decode()",
    );
    assert!(!code.is_empty(), "bytes.decode: {}", code);
}

// =============================================================================
// Section 10: attribute_convert.rs
// =============================================================================

#[test]
fn test_ac_type_name_pattern() {
    let code = transpile(
        "def get_type(x) -> str:\n    return type(x).__name__",
    );
    assert!(
        code.contains("type_name"),
        "type(x).__name__: {}",
        code
    );
}

#[test]
fn test_ac_os_environ() {
    let code = transpile(
        "import os\ndef get_env() -> dict:\n    return os.environ",
    );
    assert!(code.contains("env::vars"), "os.environ: {}", code);
}

#[test]
fn test_ac_datetime_date_min() {
    let code = transpile(
        "def earliest():\n    return date.min",
    );
    assert!(!code.is_empty(), "date.min: {}", code);
}

#[test]
fn test_ac_datetime_date_max() {
    let code = transpile(
        "def latest():\n    return date.max",
    );
    assert!(!code.is_empty(), "date.max: {}", code);
}

#[test]
fn test_ac_datetime_datetime_min() {
    let code = transpile(
        "def earliest_dt():\n    return datetime.min",
    );
    assert!(!code.is_empty(), "datetime.min: {}", code);
}

#[test]
fn test_ac_datetime_datetime_max() {
    let code = transpile(
        "def latest_dt():\n    return datetime.max",
    );
    assert!(!code.is_empty(), "datetime.max: {}", code);
}

#[test]
fn test_ac_time_min() {
    let code = transpile(
        "def earliest_time():\n    return time.min",
    );
    assert!(!code.is_empty(), "time.min: {}", code);
}

#[test]
fn test_ac_time_max() {
    let code = transpile(
        "def latest_time():\n    return time.max",
    );
    assert!(!code.is_empty(), "time.max: {}", code);
}

#[test]
fn test_ac_timedelta_min() {
    let code = transpile(
        "def smallest_td():\n    return timedelta.min",
    );
    assert!(!code.is_empty(), "timedelta.min: {}", code);
}

#[test]
fn test_ac_timedelta_max() {
    let code = transpile(
        "def largest_td():\n    return timedelta.max",
    );
    assert!(!code.is_empty(), "timedelta.max: {}", code);
}

#[test]
fn test_ac_timedelta_resolution() {
    let code = transpile(
        "def td_res():\n    return timedelta.resolution",
    );
    assert!(!code.is_empty(), "timedelta.resolution: {}", code);
}

#[test]
fn test_ac_time_resolution() {
    let code = transpile(
        "def time_res():\n    return time.resolution",
    );
    assert!(!code.is_empty(), "time.resolution: {}", code);
}

#[test]
fn test_ac_math_pi() {
    let code = transpile(
        "import math\ndef get_pi() -> float:\n    return math.pi",
    );
    assert!(code.contains("PI"), "math.pi: {}", code);
}

#[test]
fn test_ac_math_e() {
    let code = transpile(
        "import math\ndef get_e() -> float:\n    return math.e",
    );
    assert!(code.contains("E"), "math.e: {}", code);
}

#[test]
fn test_ac_math_tau() {
    let code = transpile(
        "import math\ndef get_tau() -> float:\n    return math.tau",
    );
    assert!(code.contains("TAU"), "math.tau: {}", code);
}

#[test]
fn test_ac_math_inf() {
    let code = transpile(
        "import math\ndef get_inf() -> float:\n    return math.inf",
    );
    assert!(code.contains("INFINITY"), "math.inf: {}", code);
}

#[test]
fn test_ac_math_nan() {
    let code = transpile(
        "import math\ndef get_nan() -> float:\n    return math.nan",
    );
    assert!(code.contains("NAN"), "math.nan: {}", code);
}

#[test]
fn test_ac_sys_argv() {
    let code = transpile(
        "import sys\ndef get_args() -> list:\n    return sys.argv",
    );
    assert!(code.contains("args()"), "sys.argv: {}", code);
}

#[test]
fn test_ac_sys_platform() {
    let code = transpile(
        "import sys\ndef get_platform() -> str:\n    return sys.platform",
    );
    assert!(!code.is_empty(), "sys.platform: {}", code);
}

#[test]
fn test_ac_sys_stdin() {
    let code = transpile(
        "import sys\ndef get_stdin():\n    return sys.stdin",
    );
    assert!(code.contains("stdin"), "sys.stdin: {}", code);
}

#[test]
fn test_ac_sys_stdout() {
    let code = transpile(
        "import sys\ndef get_stdout():\n    return sys.stdout",
    );
    assert!(code.contains("stdout"), "sys.stdout: {}", code);
}

#[test]
fn test_ac_sys_stderr() {
    let code = transpile(
        "import sys\ndef get_stderr():\n    return sys.stderr",
    );
    assert!(code.contains("stderr"), "sys.stderr: {}", code);
}

#[test]
fn test_ac_string_ascii_lowercase() {
    let code = transpile(
        "import string\ndef get_lower() -> str:\n    return string.ascii_lowercase",
    );
    assert!(
        code.contains("abcdefghijklmnopqrstuvwxyz"),
        "string.ascii_lowercase: {}",
        code
    );
}

#[test]
fn test_ac_string_digits() {
    let code = transpile(
        "import string\ndef get_digits() -> str:\n    return string.digits",
    );
    assert!(
        code.contains("0123456789"),
        "string.digits: {}",
        code
    );
}

#[test]
fn test_ac_re_ignorecase() {
    let code = transpile(
        "import re\ndef get_flag() -> int:\n    return re.IGNORECASE",
    );
    assert!(!code.is_empty(), "re.IGNORECASE: {}", code);
}

#[test]
fn test_ac_re_multiline() {
    let code = transpile(
        "import re\ndef get_flag() -> int:\n    return re.MULTILINE",
    );
    assert!(!code.is_empty(), "re.MULTILINE: {}", code);
}

#[test]
fn test_ac_pathlib_name() {
    let code = transpile(
        "from pathlib import Path\ndef get_name() -> str:\n    path = Path('/tmp/test.txt')\n    return path.name",
    );
    assert!(
        code.contains("file_name"),
        "pathlib path.name: {}",
        code
    );
}

#[test]
fn test_ac_pathlib_suffix() {
    let code = transpile(
        "from pathlib import Path\ndef get_ext() -> str:\n    path = Path('/tmp/test.txt')\n    return path.suffix",
    );
    assert!(
        code.contains("extension"),
        "pathlib path.suffix: {}",
        code
    );
}

#[test]
fn test_ac_pathlib_stem() {
    let code = transpile(
        "from pathlib import Path\ndef get_stem() -> str:\n    path = Path('/tmp/test.txt')\n    return path.stem",
    );
    assert!(
        code.contains("file_stem"),
        "pathlib path.stem: {}",
        code
    );
}

#[test]
fn test_ac_pathlib_parent() {
    let code = transpile(
        "from pathlib import Path\ndef get_parent():\n    path = Path('/tmp/test.txt')\n    return path.parent",
    );
    assert!(code.contains("parent"), "pathlib path.parent: {}", code);
}

#[test]
fn test_ac_datetime_year_property() {
    let code = transpile(
        "def get_year(dt) -> int:\n    return dt.year",
    );
    assert!(code.contains("year"), "dt.year property: {}", code);
}

#[test]
fn test_ac_datetime_month_property() {
    let code = transpile(
        "def get_month(dt) -> int:\n    return dt.month",
    );
    assert!(code.contains("month"), "dt.month property: {}", code);
}

#[test]
fn test_ac_datetime_day_property() {
    let code = transpile(
        "def get_day(dt) -> int:\n    return dt.day",
    );
    assert!(code.contains("day"), "dt.day property: {}", code);
}

#[test]
fn test_ac_timedelta_days_property() {
    let code = transpile(
        "def get_days(td) -> int:\n    return td.days",
    );
    assert!(code.contains("days"), "td.days property: {}", code);
}

#[test]
fn test_ac_timedelta_seconds_property() {
    let code = transpile(
        "def get_secs(td) -> int:\n    return td.seconds",
    );
    assert!(
        code.contains("seconds"),
        "td.seconds property: {}",
        code
    );
}

#[test]
fn test_ac_enum_constant_access() {
    let code = transpile(
        "class Color:\n    RED = 1\n    GREEN = 2\n    BLUE = 3\n\ndef get_red() -> int:\n    return Color.RED",
    );
    assert!(
        code.contains("Color::RED"),
        "enum constant access: {}",
        code
    );
}

#[test]
fn test_ac_cls_attribute_classmethod() {
    let code = transpile(
        "class Counter:\n    count = 0\n    @classmethod\n    def increment(cls):\n        cls.count += 1",
    );
    assert!(!code.is_empty(), "cls attribute: {}", code);
}

#[test]
fn test_ac_stat_st_size() {
    let code = transpile(
        "def get_size(stats) -> int:\n    return stats.st_size",
    );
    assert!(code.contains("len"), "stat.st_size: {}", code);
}

#[test]
fn test_ac_stat_st_mtime() {
    let code = transpile(
        "def get_mtime(stats) -> float:\n    return stats.st_mtime",
    );
    assert!(code.contains("modified"), "stat.st_mtime: {}", code);
}

#[test]
fn test_ac_fractions_numerator() {
    let code = transpile(
        "def get_num(f) -> int:\n    return f.numerator",
    );
    assert!(code.contains("numer"), "fraction numerator: {}", code);
}

#[test]
fn test_ac_fractions_denominator() {
    let code = transpile(
        "def get_den(f) -> int:\n    return f.denominator",
    );
    assert!(code.contains("denom"), "fraction denominator: {}", code);
}

// =============================================================================
// Section 11: method_call_routing.rs - pathlib instance methods
// =============================================================================

#[test]
fn test_mcr_pathlib_write_text() {
    let code = transpile(
        "from pathlib import Path\ndef save(path, content: str):\n    path.write_text(content)",
    );
    assert!(!code.is_empty(), "path.write_text: {}", code);
}

#[test]
fn test_mcr_pathlib_read_text() {
    let code = transpile(
        "from pathlib import Path\ndef load(path) -> str:\n    return path.read_text()",
    );
    assert!(!code.is_empty(), "path.read_text: {}", code);
}

#[test]
fn test_mcr_pathlib_exists() {
    let code = transpile(
        "from pathlib import Path\ndef check(path) -> bool:\n    return path.exists()",
    );
    assert!(!code.is_empty(), "path.exists: {}", code);
}

#[test]
fn test_mcr_pathlib_is_file() {
    let code = transpile(
        "from pathlib import Path\ndef check(path) -> bool:\n    return path.is_file()",
    );
    assert!(!code.is_empty(), "path.is_file: {}", code);
}

#[test]
fn test_mcr_pathlib_is_dir() {
    let code = transpile(
        "from pathlib import Path\ndef check(path) -> bool:\n    return path.is_dir()",
    );
    assert!(!code.is_empty(), "path.is_dir: {}", code);
}

#[test]
fn test_mcr_pathlib_mkdir() {
    let code = transpile(
        "from pathlib import Path\ndef create(path):\n    path.mkdir()",
    );
    assert!(!code.is_empty(), "path.mkdir: {}", code);
}

#[test]
fn test_mcr_pathlib_unlink() {
    let code = transpile(
        "from pathlib import Path\ndef remove(path):\n    path.unlink()",
    );
    assert!(!code.is_empty(), "path.unlink: {}", code);
}

#[test]
fn test_mcr_pathlib_with_suffix() {
    let code = transpile(
        "from pathlib import Path\ndef change_ext(path):\n    return path.with_suffix('.txt')",
    );
    assert!(!code.is_empty(), "path.with_suffix: {}", code);
}

#[test]
fn test_mcr_pathlib_glob_pattern() {
    let code = transpile(
        "from pathlib import Path\ndef find_files(path) -> list:\n    return path.glob('*.py')",
    );
    assert!(!code.is_empty(), "path.glob: {}", code);
}

// =============================================================================
// Section 12: datetime instance methods
// =============================================================================

#[test]
fn test_mcr_datetime_total_seconds() {
    let code = transpile(
        "def get_total(td) -> float:\n    return td.total_seconds()",
    );
    assert!(!code.is_empty(), "td.total_seconds: {}", code);
}

#[test]
fn test_mcr_datetime_weekday() {
    let code = transpile(
        "def get_weekday(dt) -> int:\n    return dt.weekday()",
    );
    assert!(!code.is_empty(), "dt.weekday: {}", code);
}

#[test]
fn test_mcr_datetime_isoweekday() {
    let code = transpile(
        "def get_iso_weekday(dt) -> int:\n    return dt.isoweekday()",
    );
    assert!(!code.is_empty(), "dt.isoweekday: {}", code);
}

// =============================================================================
// Section 13: Additional coverage targets
// =============================================================================

#[test]
fn test_mcr_base64_decode_chain() {
    let code = transpile(
        "import base64\ndef decode_b64(data: str) -> str:\n    return base64.b64encode(data.encode()).decode()",
    );
    assert!(!code.is_empty(), "base64 encode/decode chain: {}", code);
}

#[test]
fn test_mcr_collections_counter() {
    let code = transpile(
        "import collections\ndef count_items(items: list) -> dict:\n    return collections.Counter(items)",
    );
    assert!(!code.is_empty(), "collections.Counter: {}", code);
}

#[test]
fn test_mcr_collections_deque() {
    let code = transpile(
        "import collections\ndef make_deque(items: list):\n    return collections.deque(items)",
    );
    assert!(!code.is_empty(), "collections.deque: {}", code);
}

#[test]
fn test_mcr_collections_defaultdict() {
    let code = transpile(
        "import collections\ndef make_dd():\n    return collections.defaultdict(int)",
    );
    assert!(
        !code.is_empty(),
        "collections.defaultdict: {}",
        code
    );
}

#[test]
fn test_id_sys_stdin_read() {
    let code = transpile(
        "import sys\ndef read_input():\n    return sys.stdin.read()",
    );
    assert!(!code.is_empty(), "sys.stdin.read: {}", code);
}

#[test]
fn test_id_sys_stdout_write() {
    let code = transpile(
        "import sys\ndef write_output(msg: str):\n    sys.stdout.write(msg)",
    );
    assert!(!code.is_empty(), "sys.stdout.write: {}", code);
}

#[test]
fn test_id_sys_stderr_write() {
    let code = transpile(
        "import sys\ndef write_err(msg: str):\n    sys.stderr.write(msg)",
    );
    assert!(!code.is_empty(), "sys.stderr.write: {}", code);
}

#[test]
fn test_ac_exception_returncode() {
    let code = transpile(
        "def handle_error(e):\n    return e.returncode",
    );
    assert!(!code.is_empty(), "exception returncode: {}", code);
}

#[test]
fn test_ac_tempfile_name() {
    let code = transpile(
        "def get_temp_name(temp):\n    return temp.name",
    );
    assert!(!code.is_empty(), "tempfile name: {}", code);
}

#[test]
fn test_lg_await_expression() {
    let code = transpile(
        "async def fetch(url: str) -> str:\n    return await get_data(url)",
    );
    assert!(!code.is_empty(), "await expression: {}", code);
}

#[test]
fn test_lg_fstring_empty() {
    let code = transpile(
        "def empty_str() -> str:\n    return f''",
    );
    assert!(!code.is_empty(), "empty fstring: {}", code);
}

#[test]
fn test_lg_fstring_plain_literal() {
    let code = transpile(
        "def plain_str() -> str:\n    return f'hello world'",
    );
    assert!(!code.is_empty(), "plain fstring: {}", code);
}

#[test]
fn test_comp_set_operation_intersection() {
    let code = transpile(
        "def intersect(a: set, b: set) -> set:\n    return a & b",
    );
    assert!(
        code.contains("intersection"),
        "set intersection: {}",
        code
    );
}

#[test]
fn test_comp_set_operation_union() {
    let code = transpile(
        "def unite(a: set, b: set) -> set:\n    return a | b",
    );
    assert!(code.contains("union"), "set union: {}", code);
}

#[test]
fn test_comp_set_operation_difference() {
    let code = transpile(
        "def diff(a: set, b: set) -> set:\n    return a - b",
    );
    assert!(
        code.contains("difference"),
        "set difference: {}",
        code
    );
}

#[test]
fn test_comp_set_operation_symmetric_diff() {
    let code = transpile(
        "def sym_diff(a: set, b: set) -> set:\n    return a ^ b",
    );
    assert!(
        code.contains("symmetric_difference"),
        "set xor: {}",
        code
    );
}

#[test]
fn test_ac_property_method_access() {
    let code = transpile(
        "class Circle:\n    def __init__(self, radius: float):\n        self.radius = radius\n    @property\n    def area(self) -> float:\n        return 3.14159 * self.radius * self.radius\n\ndef get_area(c) -> float:\n    return c.area",
    );
    assert!(!code.is_empty(), "property method access: {}", code);
}

#[test]
fn test_id_datetime_date_component() {
    let code = transpile(
        "def get_date(dt):\n    return dt.date()",
    );
    assert!(!code.is_empty(), "dt.date() component: {}", code);
}

#[test]
fn test_id_datetime_time_component() {
    let code = transpile(
        "def get_time(dt):\n    return dt.time()",
    );
    assert!(!code.is_empty(), "dt.time() component: {}", code);
}

#[test]
fn test_ac_string_printable() {
    let code = transpile(
        "import string\ndef get_printable() -> str:\n    return string.printable",
    );
    assert!(!code.is_empty(), "string.printable: {}", code);
}

#[test]
fn test_ac_string_punctuation() {
    let code = transpile(
        "import string\ndef get_punct() -> str:\n    return string.punctuation",
    );
    assert!(!code.is_empty(), "string.punctuation: {}", code);
}

#[test]
fn test_ac_string_whitespace() {
    let code = transpile(
        "import string\ndef get_ws() -> str:\n    return string.whitespace",
    );
    assert!(!code.is_empty(), "string.whitespace: {}", code);
}

#[test]
fn test_ac_string_hexdigits() {
    let code = transpile(
        "import string\ndef get_hex() -> str:\n    return string.hexdigits",
    );
    assert!(!code.is_empty(), "string.hexdigits: {}", code);
}

#[test]
fn test_ac_string_octdigits() {
    let code = transpile(
        "import string\ndef get_oct() -> str:\n    return string.octdigits",
    );
    assert!(!code.is_empty(), "string.octdigits: {}", code);
}

#[test]
fn test_ac_sys_version_info() {
    let code = transpile(
        "import sys\ndef get_version():\n    return sys.version_info",
    );
    assert!(!code.is_empty(), "sys.version_info: {}", code);
}

#[test]
fn test_mcr_string_rfind() {
    let code = transpile(
        "def find_last(text: str, sub: str) -> int:\n    return text.rfind(sub)",
    );
    assert!(!code.is_empty(), "str.rfind: {}", code);
}

#[test]
fn test_mcr_string_splitlines() {
    let code = transpile(
        "def get_lines(text: str) -> list:\n    return text.splitlines()",
    );
    assert!(!code.is_empty(), "str.splitlines: {}", code);
}

#[test]
fn test_mcr_string_lstrip() {
    let code = transpile(
        "def trim_left(text: str) -> str:\n    return text.lstrip()",
    );
    assert!(!code.is_empty(), "str.lstrip: {}", code);
}

#[test]
fn test_mcr_string_rstrip() {
    let code = transpile(
        "def trim_right(text: str) -> str:\n    return text.rstrip()",
    );
    assert!(!code.is_empty(), "str.rstrip: {}", code);
}

#[test]
fn test_mcr_string_capitalize() {
    let code = transpile(
        "def cap(text: str) -> str:\n    return text.capitalize()",
    );
    assert!(!code.is_empty(), "str.capitalize: {}", code);
}

#[test]
fn test_dc_dict_homogeneous_float() {
    let code = transpile(
        "def make_stats() -> dict:\n    return {'mean': 1.5, 'std': 0.5}",
    );
    assert!(!code.is_empty(), "dict homogeneous float: {}", code);
}

#[test]
fn test_dc_dict_with_bool_values() {
    let code = transpile(
        "def make_flags() -> dict:\n    return {'active': True, 'verified': False}",
    );
    assert!(!code.is_empty(), "dict with bool values: {}", code);
}

#[test]
fn test_cons_list_with_mixed_int_float() {
    let code = transpile(
        "def make_numbers() -> list:\n    return [1, 2.5, 3, 4.0]",
    );
    assert!(!code.is_empty(), "list mixed int/float: {}", code);
}

#[test]
fn test_ac_stat_st_ctime() {
    let code = transpile(
        "def get_ctime(stats) -> float:\n    return stats.st_ctime",
    );
    assert!(
        code.contains("created"),
        "stat.st_ctime: {}",
        code
    );
}

#[test]
fn test_ac_stat_st_atime() {
    let code = transpile(
        "def get_atime(stats) -> float:\n    return stats.st_atime",
    );
    assert!(
        code.contains("accessed"),
        "stat.st_atime: {}",
        code
    );
}

#[test]
fn test_th_is_string_variable_key() {
    let code = transpile(
        "def lookup(config: dict, key: str):\n    return config[key]",
    );
    assert!(!code.is_empty(), "string variable key: {}", code);
}

#[test]
fn test_lg_truthiness_int_var() {
    let code = transpile(
        "def check(count: int) -> str:\n    return 'yes' if count else 'no'",
    );
    assert!(code.contains("!= 0"), "int truthiness: {}", code);
}

#[test]
fn test_lg_truthiness_float_var() {
    let code = transpile(
        "def check(val: float) -> str:\n    return 'yes' if val else 'no'",
    );
    assert!(code.contains("!= 0.0"), "float truthiness: {}", code);
}

#[test]
fn test_lg_truthiness_dict_var() {
    let code = transpile(
        "def check(data: dict) -> str:\n    return 'yes' if data else 'no'",
    );
    assert!(
        code.contains("is_empty"),
        "dict truthiness: {}",
        code
    );
}

#[test]
fn test_lg_truthiness_set_var() {
    let code = transpile(
        "def check(items: set) -> str:\n    return 'yes' if items else 'no'",
    );
    assert!(
        code.contains("is_empty"),
        "set truthiness: {}",
        code
    );
}

#[test]
fn test_mcr_string_format_method() {
    let code = transpile(
        "def fmt(template: str, name: str) -> str:\n    return template.format(name)",
    );
    assert!(!code.is_empty(), "str.format: {}", code);
}

#[test]
fn test_mcr_string_hex_method() {
    let code = transpile(
        "def to_hex(data: str) -> str:\n    return data.hex()",
    );
    assert!(!code.is_empty(), "str.hex: {}", code);
}

#[test]
fn test_ac_math_sin_value() {
    let code = transpile(
        "import math\ndef get_sin():\n    fn = math.sin\n    return fn(1.0)",
    );
    assert!(!code.is_empty(), "math.sin as value: {}", code);
}

#[test]
fn test_ac_math_sqrt_value() {
    let code = transpile(
        "import math\ndef get_sqrt():\n    fn = math.sqrt\n    return fn(4.0)",
    );
    assert!(!code.is_empty(), "math.sqrt as value: {}", code);
}
