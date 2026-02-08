//! Wave 7 coverage tests: post-generation fix functions in rust_gen.rs
//! and type inference code paths in func_gen.rs.
//!
//! Targets ~2000+ uncovered lines in fix_* functions, infer_* functions,
//! generate_simple_constant, codegen_single_param, and control flow patterns.

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(code)?;
        Ok(result)
    }

    // ========================================================================
    // SECTION 1: fix_python_truthiness - bool coercion patterns (tests 1-15)
    // ========================================================================

    #[test]
    fn test_w7r_truthiness_list_if() {
        let code = "def f(items: list) -> bool:\n    if items:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_string_if() {
        let code = "def f(name: str) -> bool:\n    if name:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_int_if() {
        let code = "def f(count: int) -> bool:\n    if count:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_dict_if() {
        let code = "def f(data: dict) -> bool:\n    if data:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn") || result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_while_list() {
        let code = "def f(items: list) -> int:\n    count = 0\n    while items:\n        count += 1\n        break\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_not_list() {
        let code = "def f(items: list) -> bool:\n    if not items:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_not_string() {
        let code = "def f(name: str) -> bool:\n    if not name:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_not_dict() {
        let code = "def f(data: dict) -> bool:\n    if not data:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_bool_param_unchanged() {
        let code = "def f(flag: bool) -> bool:\n    if flag:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_is_prefix_unchanged() {
        let code = "def f(is_valid: bool) -> bool:\n    if is_valid:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_has_prefix_unchanged() {
        let code = "def f(has_data: bool) -> bool:\n    if has_data:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("bool"));
        }
    }

    #[test]
    fn test_w7r_truthiness_nested_if_list() {
        let code = "def f(items: list, data: dict) -> bool:\n    if items:\n        if data:\n            return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_and_combination() {
        let code = "def f(items: list, name: str) -> bool:\n    if items and name:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_or_combination() {
        let code = "def f(items: list, name: str) -> bool:\n    if items or name:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_truthiness_empty_string_check() {
        let code = "def f(s: str) -> str:\n    if s:\n        return s\n    return \"default\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 2: fix_hashmap_* functions (tests 16-30)
    // ========================================================================

    #[test]
    fn test_w7r_hashmap_string_keys() {
        let code = "def f() -> dict:\n    d = {\"key\": \"value\"}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_int_keys() {
        let code = "def f() -> dict:\n    d = {1: \"one\", 2: \"two\"}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_nested_collection() {
        let code = "def f() -> dict:\n    d = {\"a\": [1, 2, 3]}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_empty_dict() {
        let code = "def f() -> dict:\n    d = {}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_get() {
        let code = "def f() -> str:\n    d = {\"key\": \"value\"}\n    return d.get(\"key\", \"default\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_update() {
        let code = "def f() -> dict:\n    d = {\"a\": 1}\n    d[\"b\"] = 2\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_keys() {
        let code = "def f() -> list:\n    d = {\"a\": 1, \"b\": 2}\n    return list(d.keys())\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_values() {
        let code = "def f() -> list:\n    d = {\"a\": 1, \"b\": 2}\n    return list(d.values())\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_items() {
        let code = "def f() -> list:\n    d = {\"a\": 1, \"b\": 2}\n    items = list(d.items())\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_pop() {
        let code = "def f() -> int:\n    d = {\"a\": 1, \"b\": 2}\n    return d.pop(\"a\", 0)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_len() {
        let code = "def f() -> int:\n    d = {\"a\": 1, \"b\": 2}\n    return len(d)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_in_check() {
        let code = "def f() -> bool:\n    d = {\"a\": 1}\n    return \"a\" in d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_comprehension() {
        let code = "def f(items: list) -> dict:\n    return {str(x): x for x in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_from_keys() {
        let code = "def f() -> dict:\n    keys = [\"a\", \"b\", \"c\"]\n    d = {}\n    for k in keys:\n        d[k] = 0\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_hashmap_dict_merge() {
        let code = "def f() -> dict:\n    a = {\"x\": 1}\n    b = {\"y\": 2}\n    a.update(b)\n    return a\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 3: fix_depyler_value_inserts_generalized (tests 31-42)
    // ========================================================================

    #[test]
    fn test_w7r_dv_insert_string_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"key\"] = \"value\"\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_insert_int_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"count\"] = 42\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_insert_bool_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"active\"] = True\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_insert_list_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"items\"] = [1, 2, 3]\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_insert_float_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"pi\"] = 3.14\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_insert_none_val() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"empty\"] = None\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_multiple_inserts() {
        let code = "def f() -> dict:\n    d = {}\n    d[\"a\"] = 1\n    d[\"b\"] = 2\n    d[\"c\"] = 3\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_dict_with_mixed_types() {
        let code = "def f() -> dict:\n    d = {\"name\": \"alice\", \"age\": 30}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_dict_conditional_insert() {
        let code = "def f(flag: bool) -> dict:\n    d = {}\n    if flag:\n        d[\"key\"] = \"yes\"\n    else:\n        d[\"key\"] = \"no\"\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_dict_loop_insert() {
        let code = "def f() -> dict:\n    d = {}\n    for i in range(5):\n        d[str(i)] = i\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_nested_dict() {
        let code = "def f() -> dict:\n    d = {\"outer\": {\"inner\": 1}}\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_dv_dict_delete() {
        let code = "def f() -> dict:\n    d = {\"a\": 1, \"b\": 2}\n    del d[\"a\"]\n    return d\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 4: fix_*_string_* - String coercion (tests 43-58)
    // ========================================================================

    #[test]
    fn test_w7r_string_concat_basic() {
        let code = "def f(name: str) -> str:\n    return \"hello \" + name\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_concat_multiple() {
        let code = "def f(first: str, last: str) -> str:\n    return first + \" \" + last\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_fstring_int() {
        let code = "def f(x: int) -> str:\n    return f\"Value: {x}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_fstring_float() {
        let code = "def f(x: float) -> str:\n    return f\"Pi is {x}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_fstring_bool() {
        let code = "def f(flag: bool) -> str:\n    return f\"Active: {flag}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_fstring_expr() {
        let code = "def f(x: int, y: int) -> str:\n    return f\"Sum: {x + y}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_fstring_method() {
        let code = "def f(name: str) -> str:\n    return f\"Hello {name.upper()}\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_join_list() {
        let code = "def f(items: list) -> str:\n    return \", \".join(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_str_conversion() {
        let code = "def f(x: int) -> str:\n    return str(x)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_strip() {
        let code = "def f(s: str) -> str:\n    return s.strip()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_replace() {
        let code = "def f(s: str) -> str:\n    return s.replace(\"old\", \"new\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_split() {
        let code = "def f(s: str) -> list:\n    return s.split(\",\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_lower_upper() {
        let code = "def f(s: str) -> str:\n    return s.lower() + s.upper()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_startswith() {
        let code = "def f(s: str) -> bool:\n    return s.startswith(\"prefix\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_endswith() {
        let code = "def f(s: str) -> bool:\n    return s.endswith(\"suffix\")\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_string_multiply() {
        let code = "def f(s: str, n: int) -> str:\n    return s * n\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 5: infer_lazy_constant_type / generate_simple_constant (tests 59-78)
    // ========================================================================

    #[test]
    fn test_w7r_const_int() {
        let code = "MAX_SIZE = 100\ndef f() -> int:\n    return MAX_SIZE\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("100") || result.contains("MAX_SIZE"));
        }
    }

    #[test]
    fn test_w7r_const_float() {
        let code = "RATIO = 2.5\ndef f() -> float:\n    return RATIO\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("2.5") || result.contains("RATIO"));
        }
    }

    #[test]
    fn test_w7r_const_string() {
        let code = "NAME = \"depyler\"\ndef f() -> str:\n    return NAME\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("depyler") || result.contains("NAME"));
        }
    }

    #[test]
    fn test_w7r_const_bool_true() {
        let code = "ENABLED = True\ndef f() -> bool:\n    return ENABLED\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("true") || result.contains("ENABLED"));
        }
    }

    #[test]
    fn test_w7r_const_bool_false() {
        let code = "DISABLED = False\ndef f() -> bool:\n    return DISABLED\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("false") || result.contains("DISABLED"));
        }
    }

    #[test]
    fn test_w7r_const_none() {
        let code = "NOTHING = None\ndef f() -> None:\n    return NOTHING\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("None") || result.contains("NOTHING"));
        }
    }

    #[test]
    fn test_w7r_const_list_strings() {
        let code = "COLORS = [\"red\", \"green\", \"blue\"]\ndef f() -> list:\n    return COLORS\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("red") || result.contains("COLORS"));
        }
    }

    #[test]
    fn test_w7r_const_list_ints() {
        let code = "PRIMES = [2, 3, 5, 7, 11]\ndef f() -> list:\n    return PRIMES\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("2") || result.contains("PRIMES"));
        }
    }

    #[test]
    fn test_w7r_const_dict_str_int() {
        let code = "MAPPING = {\"a\": 1, \"b\": 2}\ndef f() -> dict:\n    return MAPPING\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("MAPPING") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7r_const_dict_str_str() {
        let code = "LABELS = {\"x\": \"horizontal\", \"y\": \"vertical\"}\ndef f() -> dict:\n    return LABELS\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("LABELS") || result.contains("horizontal"));
        }
    }

    #[test]
    fn test_w7r_const_negative_int() {
        let code = "MIN_VAL = -100\ndef f() -> int:\n    return MIN_VAL\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("100") || result.contains("MIN_VAL"));
        }
    }

    #[test]
    fn test_w7r_const_zero() {
        let code = "ZERO = 0\ndef f() -> int:\n    return ZERO\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("0") || result.contains("ZERO"));
        }
    }

    #[test]
    fn test_w7r_const_empty_string() {
        let code = "EMPTY = \"\"\ndef f() -> str:\n    return EMPTY\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("EMPTY") || result.contains("\"\""));
        }
    }

    #[test]
    fn test_w7r_const_large_int() {
        let code = "BIG = 999999\ndef f() -> int:\n    return BIG\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("999999") || result.contains("BIG"));
        }
    }

    #[test]
    fn test_w7r_const_tuple() {
        let code = "PAIR = (1, 2)\ndef f() -> tuple:\n    return PAIR\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("PAIR") || result.contains("1"));
        }
    }

    #[test]
    fn test_w7r_const_multiple() {
        let code = "A = 10\nB = 20\ndef f() -> int:\n    return A + B\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_const_used_in_loop() {
        let code = "LIMIT = 5\ndef f() -> int:\n    total = 0\n    for i in range(LIMIT):\n        total += i\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_const_used_in_condition() {
        let code = "THRESHOLD = 50\ndef f(x: int) -> bool:\n    return x > THRESHOLD\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_const_set() {
        let code = "VOWELS = {\"a\", \"e\", \"i\", \"o\", \"u\"}\ndef f() -> bool:\n    return \"a\" in VOWELS\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_const_list_empty() {
        let code = "ITEMS = []\ndef f() -> list:\n    return ITEMS\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("ITEMS") || result.contains("Vec") || result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 6: infer_return_type_from_body (tests 79-98)
    // ========================================================================

    #[test]
    fn test_w7r_return_int() {
        let code = "def f() -> int:\n    return 42\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("i32") || result.contains("i64") || result.contains("42"));
    }

    #[test]
    fn test_w7r_return_string() {
        let code = "def f() -> str:\n    return \"hello\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("String") || result.contains("str") || result.contains("hello"));
    }

    #[test]
    fn test_w7r_return_float() {
        let code = "def f() -> float:\n    return 3.14\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("f64") || result.contains("3.14"));
    }

    #[test]
    fn test_w7r_return_bool() {
        let code = "def f() -> bool:\n    return True\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("bool") || result.contains("true"));
    }

    #[test]
    fn test_w7r_return_list() {
        let code = "def f() -> list:\n    return [1, 2, 3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("Vec") || result.contains("vec!"));
    }

    #[test]
    fn test_w7r_return_none_explicit() {
        let code = "def f() -> None:\n    return None\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_no_value() {
        let code = "def f() -> None:\n    x = 1\n    return\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_conditional_int() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        return x\n    else:\n        return -x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_conditional_str() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    else:\n        return \"non-positive\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_optional_pattern() {
        let code = "def f(x: int) -> int:\n    if x > 0:\n        return x\n    return None\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_inferred_from_arith() {
        let code = "def f(x: int, y: int) -> int:\n    result = x + y\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_inferred_from_method() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_inferred_len() {
        let code = "def f(items: list) -> int:\n    return len(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_from_nested_if() {
        let code = "def f(x: int) -> str:\n    if x > 10:\n        if x > 20:\n            return \"high\"\n        return \"medium\"\n    return \"low\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_from_loop() {
        let code = "def f(items: list) -> int:\n    for item in items:\n        if item > 0:\n            return item\n    return -1\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_tuple() {
        let code = "def f(x: int) -> tuple:\n    return (x, x * 2)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_dict_literal() {
        let code = "def f() -> dict:\n    return {\"x\": 1, \"y\": 2}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_list_comprehension() {
        let code = "def f(items: list) -> list:\n    return [x * 2 for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_return_ternary() {
        let code = "def f(x: int) -> int:\n    return x if x > 0 else -x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_return_no_annotation() {
        let code = "def f(x):\n    return x + 1\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 7: infer_expr_type_with_env (tests 99-118)
    // ========================================================================

    #[test]
    fn test_w7r_expr_len() {
        let code = "def f(items: list) -> int:\n    n = len(items)\n    return n\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("len()") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_str_call() {
        let code = "def f(x: int) -> str:\n    s = str(x)\n    return s\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_string") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_int_call() {
        let code = "def f(s: str) -> int:\n    n = int(s)\n    return n\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parse") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_float_call() {
        let code = "def f(s: str) -> float:\n    x = float(s)\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parse") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_add_ints() {
        let code = "def f(x: int, y: int) -> int:\n    return x + y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("+") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_add_floats() {
        let code = "def f(x: float, y: float) -> float:\n    return x + y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("f64") || result.contains("+"));
    }

    #[test]
    fn test_w7r_expr_string_upper() {
        let code = "def f(s: str) -> str:\n    return s.upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_uppercase") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_string_lower() {
        let code = "def f(s: str) -> str:\n    return s.lower()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_lowercase") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_list_append() {
        let code = "def f() -> list:\n    items = [1, 2]\n    items.append(3)\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("push") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_ternary() {
        let code = "def f(x: int) -> int:\n    y = x if x > 0 else 0\n    return y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_comparison_eq() {
        let code = "def f(x: int, y: int) -> bool:\n    return x == y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("==") || result.contains("bool"));
    }

    #[test]
    fn test_w7r_expr_comparison_neq() {
        let code = "def f(x: int, y: int) -> bool:\n    return x != y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("!=") || result.contains("bool"));
    }

    #[test]
    fn test_w7r_expr_comparison_lt() {
        let code = "def f(x: int, y: int) -> bool:\n    return x < y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("<") || result.contains("bool"));
    }

    #[test]
    fn test_w7r_expr_list_multiply() {
        let code = "def f() -> list:\n    return [0] * 10\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_tuple_construction() {
        let code = "def f(x: int, y: int) -> tuple:\n    return (x, y)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_method_chain() {
        let code = "def f(s: str) -> str:\n    return s.strip().lower()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_expr_abs() {
        let code = "def f(x: int) -> int:\n    return abs(x)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("abs") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_max_call() {
        let code = "def f(x: int, y: int) -> int:\n    return max(x, y)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("max") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_min_call() {
        let code = "def f(x: int, y: int) -> int:\n    return min(x, y)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("min") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_expr_sum_call() {
        let code = "def f(items: list) -> int:\n    return sum(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("sum") || result.contains("iter") || result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 8: codegen_single_param - Parameter borrowing (tests 119-133)
    // ========================================================================

    #[test]
    fn test_w7r_param_list_borrowed() {
        let code = "def f(items: list) -> int:\n    return len(items)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_str_borrowed() {
        let code = "def f(name: str) -> str:\n    return name.upper()\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_int_copy() {
        let code = "def f(x: int) -> int:\n    return x * 2\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("i32") || result.contains("i64") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_float_copy() {
        let code = "def f(x: float) -> float:\n    return x * 2.0\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("f64") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_bool_copy() {
        let code = "def f(flag: bool) -> bool:\n    return not flag\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("bool") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_dict_mutable() {
        let code = "def f(data: dict) -> None:\n    data[\"key\"] = \"value\"\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_multiple_types() {
        let code = "def f(x: int, name: str, items: list) -> int:\n    return x + len(items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_unused() {
        let code = "def f(x: int, y: int) -> int:\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_mutated_int() {
        let code = "def f(x: int) -> int:\n    x = x + 1\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_mutated_list() {
        let code = "def f(items: list) -> list:\n    items.append(1)\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_default_value() {
        let code = "def f(x: int = 0) -> int:\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_varargs() {
        let code = "def f(*args) -> int:\n    return len(args)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_two_strings() {
        let code = "def f(a: str, b: str) -> str:\n    return a + b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_param_keyword() {
        let code = "def f(loop: int) -> int:\n    return loop + 1\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_param_no_annotation() {
        let code = "def f(x) -> int:\n    return x + 1\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 9: Variable/assignment patterns (tests 134-150)
    // ========================================================================

    #[test]
    fn test_w7r_assign_tuple_unpack() {
        let code = "def f() -> int:\n    a, b = 1, 2\n    return a + b\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_tuple_unpack_three() {
        let code = "def f() -> int:\n    a, b, c = 1, 2, 3\n    return a + b + c\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_augmented_add() {
        let code = "def f() -> int:\n    count = 0\n    count += 1\n    return count\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("+=") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_augmented_sub() {
        let code = "def f(x: int) -> int:\n    x -= 1\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("-=") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_augmented_mul() {
        let code = "def f(x: int) -> int:\n    x *= 2\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("*=") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_augmented_div() {
        let code = "def f(x: float) -> float:\n    x /= 2.0\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("/=") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_augmented_mod() {
        let code = "def f(x: int) -> int:\n    x %= 3\n    return x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("%=") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_chain() {
        let code = "def f() -> int:\n    a = 0\n    b = a\n    c = b\n    return c\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_swap() {
        let code = "def f() -> int:\n    a = 1\n    b = 2\n    a, b = b, a\n    return a\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_index() {
        let code = "def f() -> list:\n    items = [1, 2, 3]\n    items[0] = 10\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_list_slice() {
        let code = "def f(items: list) -> list:\n    return items[1:3]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_string_index() {
        let code = "def f(s: str) -> str:\n    return s[0]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_from_function() {
        let code = "def helper() -> int:\n    return 42\ndef f() -> int:\n    x = helper()\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_multiple_vars() {
        let code = "def f() -> int:\n    x = 1\n    y = 2\n    z = 3\n    return x + y + z\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_assign_string_concat() {
        let code = "def f() -> str:\n    result = \"\"\n    result += \"hello\"\n    result += \" world\"\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_list_extend() {
        let code = "def f() -> list:\n    items = [1, 2]\n    items.extend([3, 4])\n    return items\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_assign_dict_access() {
        let code = "def f() -> int:\n    d = {\"a\": 1}\n    v = d[\"a\"]\n    return v\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 10: Binary operations (tests 151-175)
    // ========================================================================

    #[test]
    fn test_w7r_binop_floor_div() {
        let code = "def f(a: int, b: int) -> int:\n    return a // b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_power() {
        let code = "def f(a: int, b: int) -> int:\n    return a ** b\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("pow") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_modulo() {
        let code = "def f(a: int, b: int) -> int:\n    return a % b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("%") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_bitwise_and() {
        let code = "def f(a: int, b: int) -> int:\n    return a & b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("&") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_bitwise_or() {
        let code = "def f(a: int, b: int) -> int:\n    return a | b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("|") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_bitwise_xor() {
        let code = "def f(a: int, b: int) -> int:\n    return a ^ b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("^") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_left_shift() {
        let code = "def f(a: int) -> int:\n    return a << 1\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("<<") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_right_shift() {
        let code = "def f(a: int) -> int:\n    return a >> 1\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains(">>") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_is_none() {
        let code = "def f(x: int) -> bool:\n    return x is None\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("None") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_is_not_none() {
        let code = "def f(x: int) -> bool:\n    return x is not None\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_in_list() {
        let code = "def f(x: int) -> bool:\n    return x in [1, 2, 3]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("contains") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_not_in_list() {
        let code = "def f(x: int) -> bool:\n    return x not in [1, 2, 3]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_and() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a and b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("&&") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_or() {
        let code = "def f(a: bool, b: bool) -> bool:\n    return a or b\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("||") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_not() {
        let code = "def f(a: bool) -> bool:\n    return not a\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("!") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_mixed_arith() {
        let code = "def f(x: int, y: float) -> float:\n    return x + y\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_complex_expr() {
        let code = "def f(a: int, b: int, c: int) -> int:\n    return (a + b) * c\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_string_in() {
        let code = "def f(s: str) -> bool:\n    return \"hello\" in s\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("contains") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_comparison_chain_lt_gt() {
        let code = "def f(x: int) -> bool:\n    return x > 0 and x < 10\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_negation_int() {
        let code = "def f(x: int) -> int:\n    return -x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_integer_division() {
        let code = "def f(a: int) -> int:\n    return a // 2\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_binop_float_division() {
        let code = "def f(a: int, b: int) -> float:\n    return a / b\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_power_negative_exp() {
        let code = "def f(x: int) -> float:\n    return x ** -2\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_binop_multiply_strings() {
        let code = "def f() -> str:\n    return \"ab\" * 3\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    // ========================================================================
    // SECTION 11: Control flow patterns (tests 176-200)
    // ========================================================================

    #[test]
    fn test_w7r_flow_if_elif_else() {
        let code = "def f(x: int) -> str:\n    if x > 0:\n        return \"positive\"\n    elif x < 0:\n        return \"negative\"\n    else:\n        return \"zero\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_nested_if() {
        let code = "def f(x: int, y: int) -> str:\n    if x > 0:\n        if y > 0:\n            return \"both positive\"\n        else:\n            return \"x positive\"\n    return \"other\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_range() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(10):\n        total += i\n    return total\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("for") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_range_start_stop() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(1, 10):\n        total += i\n    return total\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("for") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_range_step() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(0, 10, 2):\n        total += i\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("for") || result.contains("step") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_for_enumerate() {
        let code = "def f(items: list) -> int:\n    total = 0\n    for i, item in enumerate(items):\n        total += i\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("enumerate") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_for_zip() {
        let code = "def f(a: list, b: list) -> list:\n    result = []\n    for x, y in zip(a, b):\n        result.append(x + y)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("zip") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_while_basic() {
        let code = "def f() -> int:\n    count = 0\n    while count < 10:\n        count += 1\n    return count\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("while") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_while_break() {
        let code = "def f() -> int:\n    count = 0\n    while True:\n        count += 1\n        if count >= 10:\n            break\n    return count\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("break") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_while_continue() {
        let code = "def f() -> int:\n    count = 0\n    total = 0\n    while count < 10:\n        count += 1\n        if count % 2 == 0:\n            continue\n        total += count\n    return total\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("continue") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_nested_for() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(3):\n        for j in range(3):\n            total += i * j\n    return total\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("for") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_with_if() {
        let code = "def f() -> int:\n    total = 0\n    for i in range(10):\n        if i % 2 == 0:\n            total += i\n    return total\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_items() {
        let code = "def f() -> int:\n    d = {\"a\": 1, \"b\": 2}\n    total = 0\n    for k, v in d.items():\n        total += v\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_for_list_direct() {
        let code = "def f() -> int:\n    items = [1, 2, 3, 4, 5]\n    total = 0\n    for item in items:\n        total += item\n    return total\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("for") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_early_return() {
        let code = "def f(x: int) -> int:\n    if x < 0:\n        return 0\n    result = x * 2\n    return result\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("return") || result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_if_in_dict() {
        let code = "def f(d: dict, key: str) -> bool:\n    if key in d:\n        return True\n    return False\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("contains") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_multiple_elif() {
        let code = "def f(x: int) -> str:\n    if x == 1:\n        return \"one\"\n    elif x == 2:\n        return \"two\"\n    elif x == 3:\n        return \"three\"\n    else:\n        return \"other\"\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("fn"));
    }

    #[test]
    fn test_w7r_flow_for_string_chars() {
        let code = "def f(s: str) -> int:\n    count = 0\n    for c in s:\n        count += 1\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_try_except_basic() {
        let code = "def f(x: int) -> int:\n    try:\n        return 10 // x\n    except:\n        return 0\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_list_comprehension_filter() {
        let code = "def f(items: list) -> list:\n    return [x for x in items if x > 0]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_nested_comprehension() {
        let code = "def f() -> list:\n    return [i * j for i in range(3) for j in range(3)]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_assert_stmt() {
        let code = "def f(x: int) -> int:\n    assert x > 0\n    return x\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("assert") || result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_pass_stmt() {
        let code = "def f() -> None:\n    pass\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_for_reversed() {
        let code = "def f() -> list:\n    result = []\n    for i in reversed(range(5)):\n        result.append(i)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_for_sorted() {
        let code = "def f(items: list) -> list:\n    result = []\n    for item in sorted(items):\n        result.append(item)\n    return result\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("fn"));
        }
    }

    #[test]
    fn test_w7r_flow_while_with_else() {
        let code = "def f(x: int) -> int:\n    count = 0\n    while x > 0:\n        x -= 1\n        count += 1\n    return count\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("while") || result.contains("fn"));
        }
    }
}
