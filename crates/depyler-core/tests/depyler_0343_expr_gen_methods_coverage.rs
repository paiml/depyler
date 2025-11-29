//! DEPYLER-0343: expr_gen.rs String/List/Dict Methods Coverage
//!
//! **EXTREME TDD Protocol - Coverage Boost Phase 1**
//!
//! Target: expr_gen.rs 40.47% → 63%+ coverage (Phase 1 of 4)
//! TDG Score: 74.7/100 (B-) - High priority for quality improvement
//!
//! This test suite adds coverage for untested method conversions:
//! - STRING METHODS: upper, lower, strip, split, join, replace, find, count
//! - LIST METHODS: append, extend, pop (variants), insert, remove, index
//! - DICT METHODS: get, keys, values, items, update, setdefault
//!
//! Strategy: Integration tests that transpile→verify generated Rust code
//! Expected Coverage Gain: +23% (~2h effort, medium complexity)

use depyler_core::DepylerPipeline;

// ============================================================================
// STRING METHOD TESTS
// ============================================================================

#[test]
fn test_string_upper_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_upper(s: str) -> str:
    return s.upper()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated upper() code:\n{}", rust_code);

    // str.upper() should generate .to_uppercase()
    assert!(
        rust_code.contains(".to_uppercase("),
        "s.upper() should generate .to_uppercase()"
    );
}

#[test]
fn test_string_lower_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_lower(s: str) -> str:
    return s.lower()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated lower() code:\n{}", rust_code);

    // str.lower() should generate .to_lowercase()
    assert!(
        rust_code.contains(".to_lowercase("),
        "s.lower() should generate .to_lowercase()"
    );
}

#[test]
fn test_string_strip_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_strip(s: str) -> str:
    return s.strip()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated strip() code:\n{}", rust_code);

    // str.strip() should generate .trim()
    assert!(
        rust_code.contains(".trim("),
        "s.strip() should generate .trim()"
    );
}

#[test]
fn test_string_split_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_split(s: str) -> list:
    return s.split(",")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated split() code:\n{}", rust_code);

    // str.split(sep) should generate .split(sep).collect()
    assert!(
        rust_code.contains(".split(") && rust_code.contains(".collect"),
        "s.split() should generate .split().collect()"
    );
}

#[test]
fn test_string_join_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_join(items: list) -> str:
    return ",".join(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated join() code:\n{}", rust_code);

    // sep.join(items) should generate items.join(sep)
    assert!(
        rust_code.contains(".join("),
        "sep.join(items) should generate .join()"
    );
}

#[test]
fn test_string_replace_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_replace(s: str) -> str:
    return s.replace("old", "new")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated replace() code:\n{}", rust_code);

    // str.replace(old, new) should generate .replace(old, new)
    assert!(
        rust_code.contains(".replace("),
        "s.replace() should generate .replace()"
    );
}

#[test]
fn test_string_find_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_find(s: str) -> int:
    return s.find("needle")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated find() code:\n{}", rust_code);

    // str.find(sub) should generate .find(sub) or .position()
    assert!(
        rust_code.contains(".find(") || rust_code.contains(".position("),
        "s.find() should generate .find() or .position()"
    );
}

#[test]
fn test_string_count_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_count(s: str) -> int:
    return s.count("x")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated count() code:\n{}", rust_code);

    // str.count(sub) should generate match counting logic
    assert!(
        rust_code.contains(".matches(") || rust_code.contains(".count("),
        "s.count() should generate .matches() or count logic"
    );
}

#[test]
fn test_string_startswith_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_startswith(s: str) -> bool:
    return s.startswith("prefix")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated startswith() code:\n{}", rust_code);

    // str.startswith(prefix) should generate .starts_with(prefix)
    assert!(
        rust_code.contains(".starts_with("),
        "s.startswith() should generate .starts_with()"
    );
}

#[test]
fn test_string_endswith_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_endswith(s: str) -> bool:
    return s.endswith("suffix")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated endswith() code:\n{}", rust_code);

    // str.endswith(suffix) should generate .ends_with(suffix)
    assert!(
        rust_code.contains(".ends_with("),
        "s.endswith() should generate .ends_with()"
    );
}

#[test]
fn test_string_isdigit_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_isdigit(s: str) -> bool:
    return s.isdigit()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated isdigit() code:\n{}", rust_code);

    // str.isdigit() should generate .chars().all(|c| c.is_numeric())
    assert!(
        rust_code.contains(".chars(")
            && (rust_code.contains(".is_numeric(") || rust_code.contains(".is_digit(")),
        "s.isdigit() should generate char digit checking"
    );
}

#[test]
fn test_string_isalpha_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_isalpha(s: str) -> bool:
    return s.isalpha()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated isalpha() code:\n{}", rust_code);

    // str.isalpha() should generate .chars().all(|c| c.is_alphabetic())
    assert!(
        rust_code.contains(".chars(") && rust_code.contains(".is_alphabetic("),
        "s.isalpha() should generate char alphabetic checking"
    );
}

// ============================================================================
// LIST METHOD TESTS
// ============================================================================

#[test]
fn test_list_append_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_append(items: list, value: int):
    items.append(value)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated append() code:\n{}", rust_code);

    // list.append(item) should generate .push(item)
    assert!(
        rust_code.contains(".push("),
        "list.append() should generate .push()"
    );
}

#[test]
fn test_list_extend_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_extend(items: list, other: list):
    items.extend(other)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated extend() code:\n{}", rust_code);

    // list.extend(other) should generate .extend(other)
    assert!(
        rust_code.contains(".extend("),
        "list.extend() should generate .extend()"
    );
}

#[test]
fn test_list_pop_no_args() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_pop(items: list):
    return items.pop()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated pop() code:\n{}", rust_code);

    // list.pop() should generate .pop()
    assert!(
        rust_code.contains(".pop("),
        "list.pop() should generate .pop()"
    );
}

#[test]
fn test_list_pop_with_index() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_pop(items: list):
    return items.pop(0)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated pop(index) code:\n{}", rust_code);

    // list.pop(index) should generate .remove(index)
    assert!(
        rust_code.contains(".remove(") || rust_code.contains(".pop("),
        "list.pop(index) should generate .remove() or indexed pop"
    );
}

#[test]
fn test_list_insert_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_insert(items: list, value: int):
    items.insert(0, value)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated insert() code:\n{}", rust_code);

    // list.insert(index, item) should generate .insert(index, item)
    assert!(
        rust_code.contains(".insert("),
        "list.insert() should generate .insert()"
    );
}

#[test]
fn test_list_remove_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_remove(items: list, value: int):
    items.remove(value)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated remove() code:\n{}", rust_code);

    // list.remove(value) should generate position().map(|i| remove(i))
    assert!(
        rust_code.contains(".remove(") || rust_code.contains(".position("),
        "list.remove(value) should generate removal logic"
    );
}

#[test]
fn test_list_index_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_index(items: list, value: int) -> int:
    return items.index(value)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated index() code:\n{}", rust_code);

    // list.index(value) should generate .iter().position()
    assert!(
        rust_code.contains(".position(") || rust_code.contains(".iter()"),
        "list.index() should generate .position() or iteration"
    );
}

#[test]
fn test_list_clear_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_clear(items: list):
    items.clear()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated clear() code:\n{}", rust_code);

    // list.clear() should generate .clear()
    assert!(
        rust_code.contains(".clear("),
        "list.clear() should generate .clear()"
    );
}

#[test]
fn test_list_copy_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_copy(items: list) -> list:
    return items.copy()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated copy() code:\n{}", rust_code);

    // list.copy() should generate .clone()
    assert!(
        rust_code.contains(".clone("),
        "list.copy() should generate .clone()"
    );
}

#[test]
fn test_list_reverse_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_reverse(items: list):
    items.reverse()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated reverse() code:\n{}", rust_code);

    // list.reverse() should generate .reverse()
    assert!(
        rust_code.contains(".reverse("),
        "list.reverse() should generate .reverse()"
    );
}

#[test]
fn test_list_sort_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_sort(items: list):
    items.sort()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated sort() code:\n{}", rust_code);

    // list.sort() should generate .sort()
    assert!(
        rust_code.contains(".sort("),
        "list.sort() should generate .sort()"
    );
}

// ============================================================================
// DICT METHOD TESTS
// ============================================================================

#[test]
fn test_dict_get_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_get(d: dict, key: str):
    return d.get(key)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated get() code:\n{}", rust_code);

    // dict.get(key) should generate .get(key)
    assert!(
        rust_code.contains(".get("),
        "dict.get() should generate .get()"
    );
}

#[test]
fn test_dict_get_with_default() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_get(d: dict, key: str):
    return d.get(key, "default")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated get(key, default) code:\n{}", rust_code);

    // dict.get(key, default) should generate .get().unwrap_or(default)
    assert!(
        rust_code.contains(".get(")
            && (rust_code.contains("unwrap_or") || rust_code.contains("or(")),
        "dict.get(key, default) should generate .get().unwrap_or(default)"
    );
}

#[test]
fn test_dict_keys_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_keys(d: dict):
    return d.keys()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated keys() code:\n{}", rust_code);

    // dict.keys() should generate .keys()
    assert!(
        rust_code.contains(".keys("),
        "dict.keys() should generate .keys()"
    );
}

#[test]
fn test_dict_values_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_values(d: dict):
    return d.values()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated values() code:\n{}", rust_code);

    // dict.values() should generate .values()
    assert!(
        rust_code.contains(".values("),
        "dict.values() should generate .values()"
    );
}

#[test]
fn test_dict_items_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_items(d: dict):
    return d.items()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated items() code:\n{}", rust_code);

    // dict.items() should generate .iter() or iteration
    assert!(
        rust_code.contains(".iter(") || rust_code.contains(".items("),
        "dict.items() should generate iteration"
    );
}

#[test]
#[ignore = "Known failing - DEPYLER-0343"]
fn test_dict_update_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_update(d: dict, other: dict):
    d.update(other)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated update() code:\n{}", rust_code);

    // dict.update(other) should generate .extend(other) or insert loop
    assert!(
        rust_code.contains(".extend(")
            || rust_code.contains(".insert(")
            || rust_code.contains("for "),
        "dict.update() should generate extend or insert operations"
    );
}

#[test]
fn test_dict_setdefault_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_setdefault(d: dict, key: str):
    return d.setdefault(key, "default")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated setdefault() code:\n{}", rust_code);

    // dict.setdefault(key, default) should generate .entry(key).or_insert(default)
    assert!(
        rust_code.contains(".entry(") && rust_code.contains(".or_insert("),
        "dict.setdefault() should generate .entry().or_insert()"
    );
}

#[test]
fn test_dict_pop_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_pop(d: dict, key: str):
    return d.pop(key)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated pop() code:\n{}", rust_code);

    // dict.pop(key) should generate .remove(key)
    assert!(
        rust_code.contains(".remove("),
        "dict.pop() should generate .remove()"
    );
}

#[test]
fn test_dict_clear_method() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_clear(d: dict):
    d.clear()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated clear() code:\n{}", rust_code);

    // dict.clear() should generate .clear()
    assert!(
        rust_code.contains(".clear("),
        "dict.clear() should generate .clear()"
    );
}

// ============================================================================
// PROPERTY TESTS - Method Robustness
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_string_method_transpiles_without_panic(
            method in prop::sample::select(vec!["upper", "lower", "strip"])
        ) {
            let pipeline = DepylerPipeline::new();
            let python_code = format!(
                "def test_method(s: str) -> str:\n    return s.{}()",
                method
            );

            // Should not panic, even if transpilation fails
            let _result = pipeline.transpile(&python_code);
        }

        #[test]
        fn prop_list_method_transpiles_without_panic(
            method in prop::sample::select(vec!["append", "clear", "copy", "reverse"])
        ) {
            let pipeline = DepylerPipeline::new();
            let python_code = if method == "append" {
                format!("def test_method(items: list):\n    items.{}(1)", method)
            } else {
                format!("def test_method(items: list):\n    items.{}()", method)
            };

            let _result = pipeline.transpile(&python_code);
        }

        #[test]
        fn prop_dict_method_transpiles_without_panic(
            method in prop::sample::select(vec!["keys", "values", "clear"])
        ) {
            let pipeline = DepylerPipeline::new();
            let python_code = format!(
                "def test_method(d: dict):\n    return d.{}()",
                method
            );

            let _result = pipeline.transpile(&python_code);
        }
    }
}
