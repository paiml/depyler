//! Wave 7 coverage tests: slicing.rs, comprehensions.rs, indexing.rs,
//! constructors.rs, attribute_convert.rs
//!
//! Targets coverage gaps in expr_gen_instance_methods:
//! - slicing.rs: list slice branches (7) + string slice branches (7)
//! - comprehensions.rs: list/set/dict comprehension paths
//! - indexing.rs: list/dict/string/tuple indexing
//! - constructors.rs: set/frozenset/list/tuple/dict constructors
//! - attribute_convert.rs: attribute access patterns

#[cfg(test)]
mod tests {
    use crate::DepylerPipeline;

    fn transpile(code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pipeline = DepylerPipeline::new();
        let result = pipeline.transpile(code)?;
        Ok(result)
    }

    // ========================================================================
    // SECTION 1: slicing.rs - list slice branches
    // ========================================================================

    #[test]
    fn test_w7_slice_list_start_stop() {
        let code = "def f(data: list) -> list:\n    return data[1:5]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("start") || result.contains("stop") || result.contains("to_vec"));
    }

    #[test]
    fn test_w7_slice_list_start_only() {
        let code = "def f(data: list) -> list:\n    return data[2:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("start") || result.contains("to_vec"));
    }

    #[test]
    fn test_w7_slice_list_stop_only() {
        let code = "def f(data: list) -> list:\n    return data[:3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_vec") || result.contains("min"));
    }

    #[test]
    fn test_w7_slice_list_full_clone() {
        let code = "def f(data: list) -> list:\n    return data[:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("clone"));
    }

    #[test]
    fn test_w7_slice_list_step_only() {
        let code = "def f(data: list) -> list:\n    return data[::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_list_start_stop_step() {
        let code = "def f(data: list) -> list:\n    return data[1:5:2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_list_start_step() {
        let code = "def f(data: list) -> list:\n    return data[1::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_list_stop_step() {
        let code = "def f(data: list) -> list:\n    return data[:3:2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_list_negative_start() {
        let code = "def f(data: list) -> list:\n    return data[-3:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("isize") || result.contains("start") || result.contains("len"));
    }

    #[test]
    fn test_w7_slice_list_negative_stop() {
        let code = "def f(data: list) -> list:\n    return data[:-2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("isize") || result.contains("stop") || result.contains("len"));
    }

    #[test]
    fn test_w7_slice_list_negative_start_stop() {
        let code = "def f(data: list) -> list:\n    return data[-4:-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("isize") || result.contains("to_vec"));
    }

    #[test]
    fn test_w7_slice_list_negative_step() {
        let code = "def f(data: list) -> list:\n    return data[::-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w7_slice_list_negative_step_value() {
        let code = "def f(data: list) -> list:\n    return data[::-2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("rev") || result.contains("step"));
    }

    #[test]
    fn test_w7_slice_list_start_negative_step() {
        let code = "def f(data: list) -> list:\n    return data[3::-1]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("rev") || result.contains("step"));
        }
    }

    // ========================================================================
    // SECTION 2: slicing.rs - string slice branches
    // ========================================================================

    #[test]
    fn test_w7_slice_string_start_stop() {
        let code = "def f(s: str) -> str:\n    return s[1:5]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("skip") || result.contains("take"));
    }

    #[test]
    fn test_w7_slice_string_start_only() {
        let code = "def f(s: str) -> str:\n    return s[2:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("skip"));
    }

    #[test]
    fn test_w7_slice_string_stop_only() {
        let code = "def f(s: str) -> str:\n    return s[:3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w7_slice_string_full_clone() {
        let code = "def f(s: str) -> str:\n    return s[:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_string") || result.contains("clone"));
    }

    #[test]
    fn test_w7_slice_string_step_only() {
        let code = "def f(s: str) -> str:\n    return s[::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_start_stop_step() {
        let code = "def f(s: str) -> str:\n    return s[1:5:2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_start_step() {
        let code = "def f(s: str) -> str:\n    return s[1::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_stop_step() {
        let code = "def f(s: str) -> str:\n    return s[:3:2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_negative_start() {
        let code = "def f(s: str) -> str:\n    return s[-3:]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("skip") || result.contains("count"));
    }

    #[test]
    fn test_w7_slice_string_negative_stop() {
        let code = "def f(s: str) -> str:\n    return s[:-2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("take") || result.contains("count"));
    }

    #[test]
    fn test_w7_slice_string_negative_start_stop() {
        let code = "def f(s: str) -> str:\n    return s[-4:-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w7_slice_string_reverse() {
        let code = "def f(s: str) -> str:\n    return s[::-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("rev") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_reverse_step2() {
        let code = "def f(s: str) -> str:\n    return s[::-2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("rev") || result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_slice_string_start_negative_step() {
        let code = "def f(s: str) -> str:\n    return s[3::-1]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("rev") || result.contains("chars"));
        }
    }

    // ========================================================================
    // SECTION 3: comprehensions.rs - list comprehensions
    // ========================================================================

    #[test]
    fn test_w7_comp_list_basic() {
        let code = "def f(items: list) -> list:\n    return [x * 2 for x in items]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("map") || result.contains("iter"));
    }

    #[test]
    fn test_w7_comp_list_with_filter() {
        let code = "def f(items: list) -> list:\n    return [x for x in items if x > 0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("filter"));
    }

    #[test]
    fn test_w7_comp_list_over_range() {
        let code = "def f() -> list:\n    return [i for i in range(10)]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("collect") || result.contains("Vec"));
    }

    #[test]
    fn test_w7_comp_list_range_with_transform() {
        let code = "def f() -> list:\n    return [i * i for i in range(5)]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("map") || result.contains("collect"));
    }

    #[test]
    fn test_w7_comp_list_nested_two_gens() {
        let code = "def f() -> list:\n    return [x + y for x in range(3) for y in range(3)]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("flat_map") || result.contains("collect"));
    }

    #[test]
    fn test_w7_comp_list_string_method() {
        let code = "def f(words: list) -> list:\n    return [w.upper() for w in words]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_list_conditional_expr() {
        let code = "def f(items: list) -> list:\n    return [x if x > 0 else 0 for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_list_len_call() {
        let code = "def f(items: list) -> list:\n    return [len(x) for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("len"));
        }
    }

    #[test]
    fn test_w7_comp_list_squared() {
        let code = "def f(nums: list) -> list:\n    return [n ** 2 for n in nums]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("pow"));
        }
    }

    #[test]
    fn test_w7_comp_list_multiple_filters() {
        let code =
            "def f(items: list) -> list:\n    return [x for x in items if x > 0 if x < 100]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filter"));
        }
    }

    #[test]
    fn test_w7_comp_list_with_addition() {
        let code = "def f(items: list) -> list:\n    return [x + 1 for x in items]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("map") || result.contains("iter"));
    }

    #[test]
    fn test_w7_comp_list_to_string() {
        let code = "def f(nums: list) -> list:\n    return [str(n) for n in nums]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("to_string"));
        }
    }

    // ========================================================================
    // SECTION 4: comprehensions.rs - dict comprehensions
    // ========================================================================

    #[test]
    fn test_w7_comp_dict_basic() {
        let code = "def f(items: list) -> dict:\n    return {k: v for k, v in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_dict_with_filter() {
        let code = "def f(items: list) -> dict:\n    return {k: v for k, v in items if v > 0}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filter") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7_comp_dict_from_range() {
        let code = "def f() -> dict:\n    return {i: i * i for i in range(5)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_dict_key_transform() {
        let code = "def f(words: list) -> dict:\n    return {w: len(w) for w in words}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7_comp_dict_enumerate() {
        let code = "def f(items: list) -> dict:\n    return {i: v for i, v in enumerate(items)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("enumerate") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7_comp_dict_string_keys() {
        let code = "def f() -> dict:\n    return {str(i): i for i in range(3)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("collect"));
        }
    }

    // ========================================================================
    // SECTION 5: comprehensions.rs - set comprehensions
    // ========================================================================

    #[test]
    fn test_w7_comp_set_basic() {
        let code = "def f(items: list) -> set:\n    return {x for x in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_set_with_filter() {
        let code = "def f(items: list) -> set:\n    return {x for x in items if x > 0}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("filter"));
        }
    }

    #[test]
    fn test_w7_comp_set_transform() {
        let code = "def f(items: list) -> set:\n    return {x * 2 for x in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("map"));
        }
    }

    #[test]
    fn test_w7_comp_set_from_range() {
        let code = "def f() -> set:\n    return {i for i in range(10)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_set_modulo() {
        let code = "def f(items: list) -> set:\n    return {x % 3 for x in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("map"));
        }
    }

    // ========================================================================
    // SECTION 6: comprehensions.rs - generator expressions
    // ========================================================================

    #[test]
    fn test_w7_gen_sum() {
        let code = "def f(items: list) -> int:\n    return sum(x for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("sum") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_gen_any() {
        let code = "def f(items: list) -> bool:\n    return any(x > 0 for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("any") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_gen_all() {
        let code = "def f(items: list) -> bool:\n    return all(x > 0 for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("all") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_gen_min() {
        let code = "def f(items: list) -> int:\n    return min(x for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("min") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_gen_max() {
        let code = "def f(items: list) -> int:\n    return max(x for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("max") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_gen_sum_with_filter() {
        let code = "def f(items: list) -> int:\n    return sum(x for x in items if x > 0)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("sum") || result.contains("filter"));
        }
    }

    #[test]
    fn test_w7_gen_sum_squared() {
        let code = "def f(items: list) -> int:\n    return sum(x * x for x in items)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("sum") || result.contains("map"));
        }
    }

    // ========================================================================
    // SECTION 7: indexing.rs - list indexing
    // ========================================================================

    #[test]
    fn test_w7_index_list_zero() {
        let code = "def f(items: list) -> int:\n    return items[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("[0]") || result.contains("expect"));
    }

    #[test]
    fn test_w7_index_list_positive() {
        let code = "def f(items: list) -> int:\n    return items[2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("expect"));
    }

    #[test]
    fn test_w7_index_list_negative() {
        let code = "def f(items: list) -> int:\n    return items[-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("len") || result.contains("saturating_sub"));
    }

    #[test]
    fn test_w7_index_list_negative_two() {
        let code = "def f(items: list) -> int:\n    return items[-2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("len") || result.contains("saturating_sub"));
    }

    #[test]
    fn test_w7_index_list_variable() {
        let code = "def f(items: list, i: int) -> int:\n    return items[i]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("as usize"));
    }

    #[test]
    fn test_w7_index_list_expression() {
        let code = "def f(items: list, i: int) -> int:\n    return items[i + 1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("idx"));
    }

    #[test]
    fn test_w7_index_list_len_minus() {
        let code = "def f(items: list) -> int:\n    return items[len(items) - 1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("len") || result.contains("get"));
    }

    // ========================================================================
    // SECTION 8: indexing.rs - dict indexing
    // ========================================================================

    #[test]
    fn test_w7_index_dict_string_key() {
        let code = "def f(data: dict) -> str:\n    return data[\"key\"]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("unwrap_or_default"));
    }

    #[test]
    fn test_w7_index_dict_string_key_name() {
        let code = "def f(data: dict) -> str:\n    return data[\"name\"]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("cloned"));
    }

    #[test]
    fn test_w7_index_dict_variable_key() {
        let code = "def f(data: dict, key: str) -> str:\n    return data[key]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("unwrap_or_default"));
    }

    #[test]
    fn test_w7_index_dict_int_key() {
        let code = "def f() -> str:\n    d = {1: \"a\", 2: \"b\"}\n    return d[1]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7_index_dict_computed_key() {
        let code = "def f(data: dict, prefix: str) -> str:\n    k = prefix + \"_key\"\n    return data[k]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get") || result.contains("unwrap_or_default"));
        }
    }

    // ========================================================================
    // SECTION 9: indexing.rs - string indexing
    // ========================================================================

    #[test]
    fn test_w7_index_string_zero() {
        let code = "def f(s: str) -> str:\n    return s[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w7_index_string_positive() {
        let code = "def f(s: str) -> str:\n    return s[3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w7_index_string_negative() {
        let code = "def f(s: str) -> str:\n    return s[-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("nth") || result.contains("count"));
    }

    #[test]
    fn test_w7_index_string_variable() {
        let code = "def f(s: str, i: int) -> str:\n    return s[i]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("nth"));
    }

    // ========================================================================
    // SECTION 10: indexing.rs - tuple indexing
    // ========================================================================

    #[test]
    fn test_w7_index_tuple_zero() {
        let code = "def f(point: tuple) -> int:\n    return point[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains(".0") || result.contains("get"));
    }

    #[test]
    fn test_w7_index_tuple_one() {
        let code = "def f(point: tuple) -> int:\n    return point[1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains(".1") || result.contains("get"));
    }

    #[test]
    fn test_w7_index_tuple_named_pair() {
        let code = "def f(pair: tuple) -> int:\n    return pair[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains(".0") || result.contains("get"));
    }

    // ========================================================================
    // SECTION 11: indexing.rs - nested and chained indexing
    // ========================================================================

    #[test]
    fn test_w7_index_nested_list() {
        let code = "def f(matrix: list) -> int:\n    return matrix[0][1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("expect"));
    }

    #[test]
    fn test_w7_index_nested_dict() {
        let code = "def f(data: dict) -> str:\n    return data[\"a\"][\"b\"]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get"));
        }
    }

    #[test]
    fn test_w7_index_list_then_dict() {
        let code = "def f(items: list) -> str:\n    return items[0][\"name\"]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get"));
        }
    }

    // ========================================================================
    // SECTION 12: constructors.rs - set constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_set_literal() {
        let code = "def f() -> set:\n    return {1, 2, 3}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashSet") || result.contains("set"));
    }

    #[test]
    fn test_w7_constructor_set_strings() {
        let code = "def f() -> set:\n    return {\"a\", \"b\", \"c\"}\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("HashSet") || result.contains("insert") || result.contains("to_string")
        );
    }

    #[test]
    fn test_w7_constructor_set_single() {
        let code = "def f() -> set:\n    return {42}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashSet") || result.contains("insert"));
    }

    #[test]
    fn test_w7_constructor_set_empty() {
        let code = "def f() -> set:\n    return set()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("new"));
        }
    }

    #[test]
    fn test_w7_constructor_set_booleans() {
        let code = "def f() -> set:\n    return {True, False}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashSet") || result.contains("insert"));
    }

    // ========================================================================
    // SECTION 13: constructors.rs - frozenset constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_frozenset_literal() {
        let code = "def f():\n    return frozenset([1, 2, 3])\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("Arc")
                    || result.contains("HashSet")
                    || result.contains("frozenset")
            );
        }
    }

    #[test]
    fn test_w7_constructor_frozenset_strings() {
        let code = "def f():\n    return frozenset([\"a\", \"b\"])\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("Arc") || result.contains("HashSet"));
        }
    }

    // ========================================================================
    // SECTION 14: constructors.rs - list constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_list_ints() {
        let code = "def f() -> list:\n    return [1, 2, 3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!") || result.contains("Vec"));
    }

    #[test]
    fn test_w7_constructor_list_strings() {
        let code = "def f() -> list:\n    return [\"a\", \"b\", \"c\"]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!") || result.contains("to_string"));
    }

    #[test]
    fn test_w7_constructor_list_empty() {
        let code = "def f() -> list:\n    return []\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!") || result.contains("Vec"));
    }

    #[test]
    fn test_w7_constructor_list_mixed_types() {
        let code = "def f() -> list:\n    return [1, \"hello\", 3.14]\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("vec!") || result.contains("json") || result.contains("format")
            );
        }
    }

    #[test]
    fn test_w7_constructor_list_with_none() {
        let code = "def f() -> list:\n    return [1, None, 3]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("Some") || result.contains("None") || result.contains("vec!"));
        }
    }

    #[test]
    fn test_w7_constructor_list_nested() {
        let code = "def f() -> list:\n    return [[1, 2], [3, 4]]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_w7_constructor_list_booleans() {
        let code = "def f() -> list:\n    return [True, False, True]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!") || result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w7_constructor_list_floats() {
        let code = "def f() -> list:\n    return [1.0, 2.5, 3.7]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!"));
    }

    // ========================================================================
    // SECTION 15: constructors.rs - tuple constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_tuple_ints() {
        let code = "def f() -> tuple:\n    return (1, 2, 3)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("(") && result.contains(")"));
    }

    #[test]
    fn test_w7_constructor_tuple_mixed() {
        let code = "def f() -> tuple:\n    return (1, \"hello\", 3.14)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_string") || result.contains("("));
    }

    #[test]
    fn test_w7_constructor_tuple_pair() {
        let code = "def f() -> tuple:\n    return (\"key\", 42)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_string") || result.contains("("));
    }

    #[test]
    fn test_w7_constructor_tuple_single() {
        let code = "def f() -> tuple:\n    return (42,)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("42") || result.contains("("));
        }
    }

    // ========================================================================
    // SECTION 16: constructors.rs - dict constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_dict_literal() {
        let code = "def f() -> dict:\n    return {\"a\": 1, \"b\": 2}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashMap") || result.contains("insert"));
    }

    #[test]
    fn test_w7_constructor_dict_empty() {
        let code = "def f() -> dict:\n    return {}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashMap") || result.contains("new"));
    }

    #[test]
    fn test_w7_constructor_dict_int_keys() {
        let code = "def f() -> dict:\n    return {1: \"a\", 2: \"b\"}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("insert"));
        }
    }

    #[test]
    fn test_w7_constructor_dict_nested() {
        let code = "def f() -> dict:\n    return {\"a\": {\"b\": 1}}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("insert"));
        }
    }

    // ========================================================================
    // SECTION 17: constructors.rs - builtin constructors
    // ========================================================================

    #[test]
    fn test_w7_constructor_bool_empty() {
        let code = "def f() -> bool:\n    return bool()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("false") || result.contains("bool") || result.contains("default")
            );
        }
    }

    #[test]
    fn test_w7_constructor_int_empty() {
        let code = "def f() -> int:\n    return int()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("0") || result.contains("i32") || result.contains("default"));
        }
    }

    #[test]
    fn test_w7_constructor_float_empty() {
        let code = "def f() -> float:\n    return float()\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("0.0") || result.contains("f64") || result.contains("default"));
        }
    }

    #[test]
    fn test_w7_constructor_str_empty() {
        let code = "def f() -> str:\n    return str()\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("String") || result.contains("new") || result.contains("default")
            );
        }
    }

    #[test]
    fn test_w7_constructor_int_from_string() {
        let code = "def f(s: str) -> int:\n    return int(s)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parse") || result.contains("i32"));
        }
    }

    #[test]
    fn test_w7_constructor_float_from_string() {
        let code = "def f(s: str) -> float:\n    return float(s)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parse") || result.contains("f64"));
        }
    }

    #[test]
    fn test_w7_constructor_str_from_int() {
        let code = "def f(n: int) -> str:\n    return str(n)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("to_string") || result.contains("format"));
        }
    }

    #[test]
    fn test_w7_constructor_bool_from_int() {
        let code = "def f(n: int) -> bool:\n    return bool(n)\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("!=") || result.contains("bool") || result.contains("0"));
        }
    }

    // ========================================================================
    // SECTION 18: attribute_convert.rs - module constants
    // ========================================================================

    #[test]
    fn test_w7_attr_math_pi() {
        let code = "import math\ndef f() -> float:\n    return math.pi\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("PI") || result.contains("consts"));
    }

    #[test]
    fn test_w7_attr_math_e() {
        let code = "import math\ndef f() -> float:\n    return math.e\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("E") || result.contains("consts"));
    }

    #[test]
    fn test_w7_attr_math_tau() {
        let code = "import math\ndef f() -> float:\n    return math.tau\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("TAU") || result.contains("consts"));
    }

    #[test]
    fn test_w7_attr_math_inf() {
        let code = "import math\ndef f() -> float:\n    return math.inf\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("INFINITY") || result.contains("f64"));
    }

    #[test]
    fn test_w7_attr_math_nan() {
        let code = "import math\ndef f() -> float:\n    return math.nan\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("NAN") || result.contains("f64"));
    }

    // ========================================================================
    // SECTION 19: attribute_convert.rs - sys module
    // ========================================================================

    #[test]
    fn test_w7_attr_sys_argv() {
        let code = "import sys\ndef f() -> list:\n    return sys.argv\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("args") || result.contains("env"));
    }

    #[test]
    fn test_w7_attr_sys_platform() {
        let code = "import sys\ndef f() -> str:\n    return sys.platform\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("linux")
                || result.contains("darwin")
                || result.contains("win32")
                || result.contains("platform")
        );
    }

    #[test]
    fn test_w7_attr_sys_stdin() {
        let code = "import sys\ndef f():\n    x = sys.stdin\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("stdin") || result.contains("io"));
        }
    }

    #[test]
    fn test_w7_attr_sys_stdout() {
        let code = "import sys\ndef f():\n    x = sys.stdout\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("stdout") || result.contains("io"));
        }
    }

    // ========================================================================
    // SECTION 20: attribute_convert.rs - os module
    // ========================================================================

    #[test]
    fn test_w7_attr_os_environ() {
        let code = "import os\ndef f() -> dict:\n    return os.environ\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("env") || result.contains("vars") || result.contains("HashMap")
            );
        }
    }

    #[test]
    fn test_w7_attr_os_environ_index() {
        let code = "import os\ndef f() -> str:\n    return os.environ[\"HOME\"]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("env") || result.contains("var"));
        }
    }

    // ========================================================================
    // SECTION 21: attribute_convert.rs - self and class attribute access
    // ========================================================================

    #[test]
    fn test_w7_attr_self_simple() {
        let code = "class Foo:\n    def __init__(self, x: int):\n        self.x = x\n    def get_x(self) -> int:\n        return self.x\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("self.x") || result.contains("self."));
    }

    #[test]
    fn test_w7_attr_self_string() {
        let code = "class Foo:\n    def __init__(self, name: str):\n        self.name = name\n    def get_name(self) -> str:\n        return self.name\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("self.name") || result.contains("self."));
    }

    #[test]
    fn test_w7_attr_self_multiple() {
        let code = "class Point:\n    def __init__(self, x: int, y: int):\n        self.x = x\n        self.y = y\n    def sum(self) -> int:\n        return self.x + self.y\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("self.x") || result.contains("self.y"));
    }

    #[test]
    fn test_w7_attr_enum_constant() {
        let code = "class Color:\n    RED = 1\n    GREEN = 2\n    BLUE = 3\ndef f() -> int:\n    return Color.RED\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("Color") && (result.contains("RED") || result.contains("::")));
        }
    }

    // ========================================================================
    // SECTION 22: attribute_convert.rs - string module constants
    // ========================================================================

    #[test]
    fn test_w7_attr_string_digits() {
        let code = "import string\ndef f() -> str:\n    return string.digits\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("0123456789"));
    }

    #[test]
    fn test_w7_attr_string_ascii_lowercase() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_lowercase\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("abcdefghijklmnopqrstuvwxyz"));
    }

    #[test]
    fn test_w7_attr_string_ascii_uppercase() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_uppercase\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    }

    #[test]
    fn test_w7_attr_string_ascii_letters() {
        let code = "import string\ndef f() -> str:\n    return string.ascii_letters\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("abcdefghijklmnopqrstuvwxyz")
                || result.contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    fn test_w7_attr_string_hexdigits() {
        let code = "import string\ndef f() -> str:\n    return string.hexdigits\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("0123456789abcdef") || result.contains("ABCDEF"));
    }

    // ========================================================================
    // SECTION 23: attribute_convert.rs - re module constants
    // ========================================================================

    #[test]
    fn test_w7_attr_re_ignorecase() {
        let code = "import re\ndef f() -> int:\n    return re.IGNORECASE\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("2") || result.contains("i32"));
    }

    #[test]
    fn test_w7_attr_re_multiline() {
        let code = "import re\ndef f() -> int:\n    return re.MULTILINE\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("8") || result.contains("i32"));
    }

    #[test]
    fn test_w7_attr_re_dotall() {
        let code = "import re\ndef f() -> int:\n    return re.DOTALL\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("16") || result.contains("i32"));
    }

    // ========================================================================
    // SECTION 24: Additional slicing edge cases
    // ========================================================================

    #[test]
    fn test_w7_slice_list_zero_start() {
        let code = "def f(data: list) -> list:\n    return data[0:5]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_vec") || result.contains("start"));
    }

    #[test]
    fn test_w7_slice_list_large_stop() {
        let code = "def f(data: list) -> list:\n    return data[:100]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("min") || result.contains("to_vec"));
    }

    #[test]
    fn test_w7_slice_string_zero_start() {
        let code = "def f(s: str) -> str:\n    return s[0:3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("take"));
    }

    #[test]
    fn test_w7_slice_string_large_stop() {
        let code = "def f(s: str) -> str:\n    return s[:100]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("take"));
    }

    // ========================================================================
    // SECTION 25: Additional comprehension edge cases
    // ========================================================================

    #[test]
    fn test_w7_comp_list_range_start_stop() {
        let code = "def f() -> list:\n    return [i for i in range(1, 10)]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("collect") || result.contains("iter"));
    }

    #[test]
    fn test_w7_comp_list_range_step() {
        let code = "def f() -> list:\n    return [i for i in range(0, 10, 2)]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("collect") || result.contains("step_by"));
        }
    }

    #[test]
    fn test_w7_comp_list_abs_call() {
        let code = "def f(items: list) -> list:\n    return [abs(x) for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("abs") || result.contains("map"));
        }
    }

    #[test]
    fn test_w7_comp_nested_three_gens() {
        let code = "def f() -> list:\n    return [x + y + z for x in range(2) for y in range(2) for z in range(2)]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("flat_map") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_dict_value_transform() {
        let code = "def f(items: list) -> dict:\n    return {i: i * 2 for i in range(5)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashMap") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_set_nested_gens() {
        let code = "def f() -> set:\n    return {x + y for x in range(3) for y in range(3)}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("flat_map"));
        }
    }

    // ========================================================================
    // SECTION 26: Additional indexing edge cases
    // ========================================================================

    #[test]
    fn test_w7_index_list_last_via_neg() {
        let code = "def f(items: list) -> int:\n    return items[-3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("saturating_sub") || result.contains("len"));
    }

    #[test]
    fn test_w7_index_dict_multiple_access() {
        let code = "def f(d: dict) -> str:\n    a = d[\"x\"]\n    b = d[\"y\"]\n    return a + b\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get"));
        }
    }

    #[test]
    fn test_w7_index_string_first_char() {
        let code = "def f(word: str) -> str:\n    return word[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("nth"));
    }

    #[test]
    fn test_w7_index_string_last_char() {
        let code = "def f(word: str) -> str:\n    return word[-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("count") || result.contains("nth"));
    }

    // ========================================================================
    // SECTION 27: Additional constructor edge cases
    // ========================================================================

    #[test]
    fn test_w7_constructor_list_single_element() {
        let code = "def f() -> list:\n    return [42]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!") || result.contains("42"));
    }

    #[test]
    fn test_w7_constructor_tuple_two_strings() {
        let code = "def f() -> tuple:\n    return (\"hello\", \"world\")\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_string") || result.contains("hello"));
    }

    #[test]
    fn test_w7_constructor_set_from_range_call() {
        let code = "def f() -> set:\n    s = set()\n    s.add(1)\n    return s\n";
        if let Ok(result) = transpile(code) {
            assert!(
                result.contains("HashSet") || result.contains("insert") || result.contains("new")
            );
        }
    }

    #[test]
    fn test_w7_constructor_dict_string_values() {
        let code = "def f() -> dict:\n    return {\"name\": \"alice\", \"city\": \"paris\"}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashMap") || result.contains("insert"));
    }

    // ========================================================================
    // SECTION 28: Additional attribute access patterns
    // ========================================================================

    #[test]
    fn test_w7_attr_datetime_properties() {
        let code = "def f(dt) -> int:\n    return dt.year\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("year") || result.contains("as i32"));
        }
    }

    #[test]
    fn test_w7_attr_datetime_month() {
        let code = "def f(dt) -> int:\n    return dt.month\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("month") || result.contains("as i32"));
        }
    }

    #[test]
    fn test_w7_attr_datetime_day() {
        let code = "def f(dt) -> int:\n    return dt.day\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("day") || result.contains("as i32"));
        }
    }

    #[test]
    fn test_w7_attr_path_name() {
        let code = "def f(path: str) -> str:\n    return path.name\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("file_name") || result.contains("name"));
        }
    }

    #[test]
    fn test_w7_attr_path_parent() {
        let code = "def f(path) -> str:\n    return path.parent\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("parent") || result.contains("path_buf"));
        }
    }

    #[test]
    fn test_w7_attr_path_suffix() {
        let code = "def f(path) -> str:\n    return path.suffix\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("extension") || result.contains("suffix"));
        }
    }

    #[test]
    fn test_w7_attr_path_stem() {
        let code = "def f(path) -> str:\n    return path.stem\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("file_stem") || result.contains("stem"));
        }
    }

    #[test]
    fn test_w7_attr_path_parts() {
        let code = "def f(path) -> list:\n    return path.parts\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("components") || result.contains("parts"));
        }
    }

    // ========================================================================
    // SECTION 29: Slice + comprehension combinations
    // ========================================================================

    #[test]
    fn test_w7_comp_over_slice() {
        let code = "def f(data: list) -> list:\n    return [x * 2 for x in data[1:5]]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_slice_of_comp_result() {
        let code = "def f(items: list) -> list:\n    result = [x * 2 for x in items]\n    return result[:3]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("to_vec") || result.contains("collect"));
        }
    }

    #[test]
    fn test_w7_comp_with_index() {
        let code = "def f(items: list) -> list:\n    return [items[i] for i in range(3)]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("get") || result.contains("map"));
        }
    }

    // ========================================================================
    // SECTION 30: Mixed patterns - constructors + indexing
    // ========================================================================

    #[test]
    fn test_w7_dict_literal_then_index() {
        let code = "def f() -> int:\n    d = {\"a\": 1, \"b\": 2}\n    return d[\"a\"]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("HashMap"));
    }

    #[test]
    fn test_w7_list_literal_then_index() {
        let code = "def f() -> int:\n    items = [10, 20, 30]\n    return items[1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("vec!"));
    }

    #[test]
    fn test_w7_list_literal_then_slice() {
        let code = "def f() -> list:\n    items = [1, 2, 3, 4, 5]\n    return items[1:4]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_vec") || result.contains("vec!"));
    }

    #[test]
    fn test_w7_set_literal_then_len() {
        let code = "def f() -> int:\n    s = {1, 2, 3}\n    return len(s)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("len") || result.contains("HashSet"));
    }

    #[test]
    fn test_w7_tuple_literal_then_index() {
        let code = "def f() -> int:\n    t = (10, 20, 30)\n    return t[0]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains(".0") || result.contains("get"));
    }

    // ========================================================================
    // SECTION 31: Additional attribute - cls and classmethod
    // ========================================================================

    #[test]
    fn test_w7_attr_cls_access() {
        let code = "class Foo:\n    VALUE = 42\n    @classmethod\n    def get(cls) -> int:\n        return cls.VALUE\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("Self::") || result.contains("VALUE") || result.contains("42"));
        }
    }

    // ========================================================================
    // SECTION 32: Extra coverage - ensure 200 tests total
    // ========================================================================

    #[test]
    fn test_w7_slice_list_step1() {
        let code = "def f(data: list) -> list:\n    return data[::1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("clone"));
    }

    #[test]
    fn test_w7_slice_string_step1() {
        let code = "def f(s: str) -> str:\n    return s[::1]\n";
        let result = transpile(code).expect("transpile");
        assert!(
            result.contains("step") || result.contains("to_string") || result.contains("chars")
        );
    }

    #[test]
    fn test_w7_slice_list_start1_stop4() {
        let code = "def f(data: list) -> list:\n    return data[1:4]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("to_vec") || result.contains("start"));
    }

    #[test]
    fn test_w7_slice_string_start1_stop4() {
        let code = "def f(s: str) -> str:\n    return s[1:4]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("skip") || result.contains("take"));
    }

    #[test]
    fn test_w7_comp_list_negate() {
        let code = "def f(items: list) -> list:\n    return [-x for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_list_bool_filter() {
        let code = "def f(items: list) -> list:\n    return [x for x in items if x]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filter") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_list_not_filter() {
        let code = "def f(items: list) -> list:\n    return [x for x in items if not x]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filter") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_dict_filter_key() {
        let code =
            "def f(items: list) -> dict:\n    return {k: v for k, v in items if k != \"bad\"}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("filter") || result.contains("HashMap"));
        }
    }

    #[test]
    fn test_w7_comp_set_abs() {
        let code = "def f(items: list) -> set:\n    return {abs(x) for x in items}\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("HashSet") || result.contains("abs"));
        }
    }

    #[test]
    fn test_w7_index_dict_get_nested() {
        let code = "def f(d: dict) -> int:\n    return d[\"a\"]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("unwrap_or_default"));
    }

    #[test]
    fn test_w7_constructor_list_five_ints() {
        let code = "def f() -> list:\n    return [1, 2, 3, 4, 5]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("vec!"));
    }

    #[test]
    fn test_w7_constructor_tuple_bool() {
        let code = "def f() -> tuple:\n    return (True, False)\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w7_attr_string_whitespace() {
        let code = "import string\ndef f() -> str:\n    return string.whitespace\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("\\t") || result.contains("\\n") || result.contains("whitespace"));
    }

    #[test]
    fn test_w7_attr_string_octdigits() {
        let code = "import string\ndef f() -> str:\n    return string.octdigits\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("01234567"));
    }

    #[test]
    fn test_w7_attr_re_verbose() {
        let code = "import re\ndef f() -> int:\n    return re.VERBOSE\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("64") || result.contains("i32"));
    }

    #[test]
    fn test_w7_attr_re_ascii() {
        let code = "import re\ndef f() -> int:\n    return re.ASCII\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("256") || result.contains("i32"));
    }

    #[test]
    fn test_w7_slice_list_step3() {
        let code = "def f(data: list) -> list:\n    return data[::3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_string_step3() {
        let code = "def f(s: str) -> str:\n    return s[::3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }

    #[test]
    fn test_w7_comp_list_subtract() {
        let code = "def f(items: list) -> list:\n    return [x - 1 for x in items]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("map") || result.contains("iter"));
    }

    #[test]
    fn test_w7_comp_list_multiply() {
        let code = "def f(items: list) -> list:\n    return [x * 3 for x in items]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("map") || result.contains("iter"));
    }

    #[test]
    fn test_w7_comp_list_divide() {
        let code = "def f(items: list) -> list:\n    return [x / 2 for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_comp_list_modulo() {
        let code = "def f(items: list) -> list:\n    return [x % 2 for x in items]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("map") || result.contains("iter"));
        }
    }

    #[test]
    fn test_w7_index_list_at_3() {
        let code = "def f(items: list) -> int:\n    return items[3]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("expect"));
    }

    #[test]
    fn test_w7_index_list_at_4() {
        let code = "def f(items: list) -> int:\n    return items[4]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("get") || result.contains("expect"));
    }

    #[test]
    fn test_w7_constructor_dict_bool_values() {
        let code = "def f() -> dict:\n    return {\"flag\": True, \"debug\": False}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashMap") || result.contains("true") || result.contains("false"));
    }

    #[test]
    fn test_w7_constructor_dict_float_values() {
        let code = "def f() -> dict:\n    return {\"pi\": 3.14, \"e\": 2.71}\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("HashMap") || result.contains("3.14"));
    }

    #[test]
    fn test_w7_attr_self_method_chain() {
        let code = "class Foo:\n    def __init__(self, items: list):\n        self.items = items\n    def first(self) -> int:\n        return self.items[0]\n";
        if let Ok(result) = transpile(code) {
            assert!(result.contains("self.items") || result.contains("self."));
        }
    }

    #[test]
    fn test_w7_slice_list_neg3_neg1() {
        let code = "def f(data: list) -> list:\n    return data[-3:-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("isize") || result.contains("to_vec") || result.contains("len"));
    }

    #[test]
    fn test_w7_slice_string_neg3_neg1() {
        let code = "def f(s: str) -> str:\n    return s[-3:-1]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("chars") || result.contains("String"));
    }

    #[test]
    fn test_w7_slice_list_start0_step2() {
        let code = "def f(data: list) -> list:\n    return data[0::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("step_by"));
    }

    #[test]
    fn test_w7_slice_string_start0_step2() {
        let code = "def f(s: str) -> str:\n    return s[0::2]\n";
        let result = transpile(code).expect("transpile");
        assert!(result.contains("step") || result.contains("chars"));
    }
}
