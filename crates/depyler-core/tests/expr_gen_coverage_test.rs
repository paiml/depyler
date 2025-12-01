//! expr_gen.rs Coverage Expansion Tests
//!
//! DEPYLER-0151 Phase 2B: Property + Mutation testing for expression generation
//! Target: 64.15% → 75%+ coverage (492 missed lines)
//!
//! Test Structure (MANDATORY):
//! - Unit Tests: Basic expression transpilation validation
//! - Property Tests: Arbitrary input validation with proptest
//! - Mutation Tests: Documented mutation kill strategies

use depyler_core::DepylerPipeline;

// ============================================================================
// UNIT TESTS - Method Call Conversions
// ============================================================================

#[test]
fn test_list_method_append() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_list():
    items = [1, 2, 3]
    items.append(4)
    return items
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list append code:\n{}", rust_code);

    // Should generate .push() for list.append()
    assert!(
        rust_code.contains(".push("),
        "list.append() should transpile to .push()"
    );
}

#[test]
fn test_dict_method_get_with_default() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_dict():
    data = {"key": "value"}
    result = data.get("key", "default")
    return result
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict.get() code:\n{}", rust_code);

    // Should generate .get() with unwrap_or
    assert!(
        rust_code.contains(".get(") && rust_code.contains("unwrap_or"),
        "dict.get(key, default) should use .get().unwrap_or()"
    );
}

#[test]
fn test_string_method_upper() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "hello"
    return text.upper()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.upper() code:\n{}", rust_code);

    // Should generate .to_uppercase()
    assert!(
        rust_code.contains(".to_uppercase()"),
        "str.upper() should transpile to .to_uppercase()"
    );
}

// ============================================================================
// UNIT TESTS - Binary Operation Edge Cases
// ============================================================================

#[test]
fn test_floor_division_semantics() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def floor_div(a: int, b: int) -> int:
    return a // b
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated floor division code:\n{}", rust_code);

    // Should include Python floor division semantics (towards negative infinity)
    assert!(
        rust_code.contains("needs_adjustment") || rust_code.contains("signs_differ"),
        "Floor division should implement Python semantics"
    );
}

#[test]
fn test_power_operation_with_negative_exponent() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def power_calc():
    return 2 ** -1
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated power with negative exp code:\n{}", rust_code);

    // Should use .powf() for negative exponents
    assert!(
        rust_code.contains(".powf(") || rust_code.contains("as f64"),
        "Negative exponent should use float power"
    );
}

#[test]
fn test_set_literals_generate_hashset() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def set_create():
    a = {1, 2, 3}
    return a
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set creation code:\n{}", rust_code);

    // Should use HashSet
    assert!(
        rust_code.contains("HashSet"),
        "Set literals should generate HashSet"
    );
}

// ============================================================================
// UNIT TESTS - Slice Operations
// ============================================================================

#[test]
fn test_slice_with_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def slice_test():
    arr = [1, 2, 3, 4, 5, 6]
    return arr[::2]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated slice with step code:\n{}", rust_code);

    // Should use .step_by() for slice with step
    assert!(
        rust_code.contains(".step_by(") || rust_code.contains("step"),
        "Slice with step should use .step_by()"
    );
}

#[test]
fn test_slice_negative_step() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def reverse_slice():
    arr = [1, 2, 3, 4, 5]
    return arr[::-1]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated reverse slice code:\n{}", rust_code);

    // Should use .rev() for negative step
    assert!(
        rust_code.contains(".rev()"),
        "Slice [::-1] should use .rev()"
    );
}

// ============================================================================
// UNIT TESTS - Comprehensions
// ============================================================================

#[test]
fn test_list_comprehension_with_filter() {
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def list_comp():
    return [x * 2 for x in range(10) if x > 5]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list comprehension code:\n{}", rust_code);

    // Should use .filter() and .map()
    assert!(
        rust_code.contains(".filter(") && rust_code.contains(".map("),
        "List comprehension with condition should use .filter().map()"
    );
}

// ============================================================================
// PROPERTY TESTS
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        #[test]
        fn prop_integer_binary_operations_transpile(a in -100i32..100i32, b in 1i32..100i32) {
            // Property: All basic binary operations should transpile without error
            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def binary_ops():
    return {} + {}
"#, a, b);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Binary operation transpilation failed: {:?}", result.err());
        }

        #[test]
        fn prop_list_operations_always_generate_vec(size in 1usize..10) {
            // Property: List creation should always generate vec! macro (non-empty lists)
            // Note: Empty lists may be optimized differently
            let pipeline = DepylerPipeline::new();
            let elements = (0..size).map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
            let python_code = format!(r#"
def make_list():
    return [{}]
"#, elements);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "List transpilation failed");

            let rust_code = result.unwrap();
            prop_assert!(
                rust_code.contains("vec!") || rust_code.contains("vec !"),
                "List should generate vec! macro"
            );
        }

        #[test]
        fn prop_dict_operations_require_hashmap(pairs in 0usize..5) {
            // Property: Dict creation should always require HashMap import
            let pipeline = DepylerPipeline::new();
            let items = (0..pairs)
                .map(|i| format!(r#""key{}": {}"#, i, i))
                .collect::<Vec<_>>()
                .join(", ");
            let python_code = format!(r#"
def make_dict():
    return {{{}}}
"#, items);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "Dict transpilation failed");

            let rust_code = result.unwrap();
            prop_assert!(
                rust_code.contains("HashMap"),
                "Dict should require HashMap"
            );
        }

        #[test]
        fn prop_range_calls_generate_valid_ranges(n in 1usize..20) {
            // Property: range(n) should generate valid Rust ranges
            let pipeline = DepylerPipeline::new();
            let python_code = format!(r#"
def use_range():
    total = 0
    for i in range({}):
        total = total + i
    return total
"#, n);

            let result = pipeline.transpile(&python_code);
            prop_assert!(result.is_ok(), "range() transpilation failed");

            let rust_code = result.unwrap();
            prop_assert!(
                rust_code.contains("..") || rust_code.contains("range"),
                "range() should generate Rust range syntax"
            );
        }
    }
}

// ============================================================================
// MUTATION TESTS
// ============================================================================

#[cfg(test)]
mod mutation_tests {
    use super::*;

    #[test]
    fn test_mutation_method_dispatch_correctness() {
        // Target Mutations:
        // 1. list.append → list.extend (wrong method selection)
        // 2. .push() → .pop() (wrong Rust method)
        // 3. Method parameter count (append takes 1 arg, not 0)
        //
        // Kill Strategy:
        // - Verify correct Rust method is generated (.push not .pop)
        // - Verify method takes correct number of parameters
        // - Mutation changing method dispatch would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def test_list_methods():
    items = []
    items.append(1)
    items.append(2)
    return items
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Changing .push() to .pop() would fail
        assert!(
            rust_code.matches(".push(").count() == 2,
            "MUTATION KILL: Must use .push() exactly 2 times for 2 append calls (found {} times)",
            rust_code.matches(".push(").count()
        );

        // Mutation Kill: Removing parameter would fail
        assert!(
            rust_code.contains(".push(1)") && rust_code.contains(".push(2)"),
            "MUTATION KILL: Must pass correct arguments to .push()"
        );
    }

    #[test]
    fn test_mutation_floor_division_semantics() {
        // Target Mutations:
        // 1. Python // → Rust / (wrong: truncates towards zero, not floor)
        // 2. Remove sign adjustment logic (wrong: breaks negative results)
        // 3. Remove remainder check (wrong: adjusts when not needed)
        //
        // Kill Strategy:
        // - Verify floor division includes sign/remainder checks
        // - Verify adjustment logic is present
        // - Mutation removing Python semantics would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def floor_div_test(a: int, b: int) -> int:
    return a // b
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Using simple / would fail for negative numbers
        assert!(
            rust_code.contains("needs_adjustment") || rust_code.contains("signs_differ"),
            "MUTATION KILL: Must include Python floor division adjustment logic"
        );

        // Mutation Kill: Removing remainder check would break correctness
        assert!(
            rust_code.contains("r_nonzero") || rust_code.contains("r != 0"),
            "MUTATION KILL: Must check remainder for adjustment decision"
        );

        // Mutation Kill: Removing sign check would break for mixed signs
        assert!(
            rust_code.contains("negative") || rust_code.contains("< 0"),
            "MUTATION KILL: Must check signs for floor division"
        );
    }

    #[test]
    fn test_mutation_comprehension_filter_order() {
        // Target Mutations:
        // 1. .filter() → .map() order swap (wrong: map then filter vs filter then map)
        // 2. Remove .filter() entirely (wrong: loses condition)
        // 3. Remove .collect() (wrong: returns iterator not Vec)
        //
        // Kill Strategy:
        // - Verify .filter() appears before .map() in chain
        // - Verify .collect() converts to Vec
        // - Mutation changing operation order would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def filtered_comp():
    return [x * 2 for x in range(10) if x > 5]
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Removing .filter() would include all elements
        assert!(
            rust_code.contains(".filter("),
            "MUTATION KILL: Must include .filter() for comprehension condition"
        );

        // Mutation Kill: Removing .map() would not transform elements
        assert!(
            rust_code.contains(".map("),
            "MUTATION KILL: Must include .map() for element transformation"
        );

        // Mutation Kill: Removing .collect() would return iterator
        assert!(
            rust_code.contains(".collect::<Vec<_>>()"),
            "MUTATION KILL: Must collect into Vec for list comprehension"
        );

        // Mutation Kill: Swapping .filter() and .map() order breaks semantics
        // Find positions of .filter and .map to verify order
        let filter_pos = rust_code.find(".filter(").expect(".filter( must exist");
        let map_pos = rust_code.find(".map(").expect(".map( must exist");
        assert!(
            filter_pos < map_pos,
            "MUTATION KILL: .filter() must appear before .map() in chain (filter at {}, map at {})",
            filter_pos,
            map_pos
        );
    }

    #[test]
    fn test_mutation_set_creation() {
        // Target Mutations:
        // 1. HashSet::new() → Vec::new() (wrong collection type)
        // 2. .insert() → .push() (wrong method for sets)
        // 3. Remove HashSet import (would fail compilation)
        //
        // Kill Strategy:
        // - Verify HashSet is used for set literals
        // - Verify .insert() is used (not .push())
        // - Mutation changing collection type would fail

        let pipeline = DepylerPipeline::new();
        let python_code = r#"
def make_set():
    s = {1, 2, 3}
    return s
"#;

        let rust_code = pipeline.transpile(python_code).unwrap();

        // Mutation Kill: Using Vec instead of HashSet would fail
        assert!(
            rust_code.contains("HashSet"),
            "MUTATION KILL: Set literals must use HashSet"
        );

        // Mutation Kill: Using .push() instead of .insert() would fail
        assert!(
            rust_code.matches(".insert(").count() >= 3,
            "MUTATION KILL: Must use .insert() for each set element (found {} times)",
            rust_code.matches(".insert(").count()
        );

        // Mutation Kill: Not importing HashSet would fail compilation
        assert!(
            rust_code.contains("use std::collections::HashSet"),
            "MUTATION KILL: Must import HashSet for set literals"
        );
    }
}

// ============================================================================
// DEPYLER-0171, 0172, 0173, 0174: Builtin Conversion Functions
// ============================================================================

#[test]
fn test_counter_builtin_conversion() {
    // DEPYLER-0171: Counter(iterable) should count elements and create HashMap
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from collections import Counter
def count_items(items):
    return Counter(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated Counter() code:\n{}", rust_code);

    // Should NOT generate HashMap::new(items) or HashMap(items)
    assert!(
        !rust_code.contains("HashMap::new(items)") && !rust_code.contains("HashMap(items)"),
        "Counter(items) should NOT use HashMap::new(items) or HashMap(items)"
    );

    // Should generate proper iterator collection
    assert!(
        rust_code.contains(".collect::<HashMap<") || rust_code.contains(".fold("),
        "Counter(items) should use .collect() or .fold() to count elements"
    );
}

#[test]
fn test_dict_builtin_conversion() {
    // DEPYLER-0172: dict(mapping) should convert mapping to HashMap
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def convert_to_dict(mapping):
    return dict(mapping)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict() code:\n{}", rust_code);

    // Should NOT generate: dict(mapping) - function not found error
    // Should generate proper conversion
    assert!(
        rust_code.contains(".collect::<HashMap<") || rust_code.contains("HashMap::from"),
        "dict(mapping) should use .collect() or HashMap::from()"
    );
}

#[test]
fn test_dict_empty_constructor() {
    // DEPYLER-0172: dict() with no args should create empty HashMap
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_empty_dict():
    return dict()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict() empty code:\n{}", rust_code);

    // Should generate HashMap::new()
    assert!(
        rust_code.contains("HashMap::new()"),
        "dict() with no args should generate HashMap::new()"
    );
}

#[test]
fn test_deque_builtin_conversion() {
    // DEPYLER-0173: deque(iterable) should create VecDeque from iterable
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
from collections import deque
def make_deque(items):
    return deque(items)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated deque() code:\n{}", rust_code);

    // Should NOT generate: VecDeque(items) - tuple struct error
    assert!(
        !rust_code.contains("VecDeque(items)"),
        "deque(items) should NOT use VecDeque(items)"
    );

    // Should generate proper conversion
    assert!(
        rust_code.contains("VecDeque::from(") || rust_code.contains(".collect::<VecDeque<"),
        "deque(items) should use VecDeque::from() or .collect()"
    );
}

#[test]
fn test_list_builtin_conversion() {
    // DEPYLER-0174: list(iterable) should convert iterable to Vec
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def convert_to_list(iterable):
    return list(iterable)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list() code:\n{}", rust_code);

    // Should NOT generate: list(iterable) - function not found error
    // Should generate proper conversion
    assert!(
        rust_code.contains(".collect::<Vec<")
            || rust_code.contains("Vec::from")
            || rust_code.contains(".to_vec()"),
        "list(iterable) should use .collect(), Vec::from(), or .to_vec()"
    );
}

#[test]
fn test_list_empty_constructor() {
    // DEPYLER-0174: list() with no args should create empty Vec
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_empty_list():
    return list()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list() empty code:\n{}", rust_code);

    // Should generate Vec::new() or vec![]
    assert!(
        rust_code.contains("Vec::new()")
            || rust_code.contains("vec![]")
            || rust_code.contains("vec ! []"),
        "list() with no args should generate Vec::new() or vec![]"
    );
}

// ============================================================================
// v3.19.1: Coverage Expansion Tests (Target: 58.77% → 80%)
// ============================================================================

#[test]
fn test_set_method_add() {
    // Test set.add() method conversion
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_set():
    s = {1, 2, 3}
    s.add(4)
    return s
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set.add() code:\n{}", rust_code);

    // Should generate .insert() for set.add()
    assert!(
        rust_code.contains(".insert("),
        "set.add() should transpile to .insert()"
    );
}

#[test]
fn test_set_method_remove() {
    // Test set.remove() method conversion
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_set():
    s = {1, 2, 3}
    s.remove(2)
    return s
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set.remove() code:\n{}", rust_code);

    // Should generate .remove() for set.remove()
    assert!(
        rust_code.contains(".remove("),
        "set.remove() should transpile to .remove()"
    );
}

#[test]
fn test_frozenset_literal() {
    // Test frozenset creation
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_frozenset():
    fs = frozenset([1, 2, 3])
    return fs
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated frozenset code:\n{}", rust_code);

    // Should use HashSet for frozenset
    assert!(
        rust_code.contains("HashSet"),
        "frozenset should use HashSet in Rust"
    );
}

#[test]
fn test_string_method_lower() {
    // Test str.lower() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "HELLO"
    return text.lower()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.lower() code:\n{}", rust_code);

    // Should generate .to_lowercase()
    assert!(
        rust_code.contains(".to_lowercase()"),
        "str.lower() should transpile to .to_lowercase()"
    );
}

#[test]
fn test_string_method_split() {
    // Test str.split() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "hello,world"
    return text.split(",")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.split() code:\n{}", rust_code);

    // Should generate .split() method
    assert!(
        rust_code.contains(".split("),
        "str.split() should transpile to .split()"
    );
}

#[test]
fn test_string_method_replace() {
    // Test str.replace() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "hello world"
    return text.replace("world", "Rust")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.replace() code:\n{}", rust_code);

    // Should generate .replace() method
    assert!(
        rust_code.contains(".replace("),
        "str.replace() should transpile to .replace()"
    );
}

#[test]
fn test_string_method_strip() {
    // Test str.strip() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "  hello  "
    return text.strip()
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.strip() code:\n{}", rust_code);

    // Should generate .trim() method
    assert!(
        rust_code.contains(".trim()"),
        "str.strip() should transpile to .trim()"
    );
}

#[test]
fn test_string_method_startswith() {
    // Test str.startswith() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "hello"
    return text.startswith("hel")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.startswith() code:\n{}", rust_code);

    // Should generate .starts_with() method
    assert!(
        rust_code.contains(".starts_with("),
        "str.startswith() should transpile to .starts_with()"
    );
}

#[test]
fn test_string_method_endswith() {
    // Test str.endswith() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_str():
    text = "hello"
    return text.endswith("lo")
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string.endswith() code:\n{}", rust_code);

    // Should generate .ends_with() method
    assert!(
        rust_code.contains(".ends_with("),
        "str.endswith() should transpile to .ends_with()"
    );
}

#[test]
fn test_dict_method_keys() {
    // Test dict.keys() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_dict():
    d = {"a": 1, "b": 2}
    return list(d.keys())
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict.keys() code:\n{}", rust_code);

    // Should generate .keys() method
    assert!(
        rust_code.contains(".keys()"),
        "dict.keys() should transpile to .keys()"
    );
}

#[test]
fn test_dict_method_values() {
    // Test dict.values() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_dict():
    d = {"a": 1, "b": 2}
    return list(d.values())
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict.values() code:\n{}", rust_code);

    // Should generate .values() method
    assert!(
        rust_code.contains(".values()"),
        "dict.values() should transpile to .values()"
    );
}

#[test]
fn test_dict_method_items() {
    // Test dict.items() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_dict():
    d = {"a": 1, "b": 2}
    for k, v in d.items():
        print(k, v)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict.items() code:\n{}", rust_code);

    // Should generate .iter() for dict iteration
    assert!(
        rust_code.contains(".iter()") || rust_code.contains(".items()"),
        "dict.items() should transpile to .iter()"
    );
}

#[test]
fn test_list_method_extend() {
    // Test list.extend() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_list():
    a = [1, 2]
    b = [3, 4]
    a.extend(b)
    return a
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list.extend() code:\n{}", rust_code);

    // Should generate .extend() for list.extend()
    assert!(
        rust_code.contains(".extend("),
        "list.extend() should transpile to .extend()"
    );
}

#[test]
fn test_list_method_remove() {
    // Test list.remove() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_list():
    items = [1, 2, 3]
    items.remove(2)
    return items
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list.remove() code:\n{}", rust_code);

    // Should generate remove logic (find index, then remove)
    assert!(
        rust_code.contains("position") || rust_code.contains(".remove("),
        "list.remove() should find and remove element"
    );
}

#[test]
fn test_list_method_pop_with_index() {
    // Test list.pop(index) method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_list():
    items = [1, 2, 3]
    val = items.pop(1)
    return val
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list.pop(1) code:\n{}", rust_code);

    // Should generate .remove(index) for list.pop(i)
    assert!(
        rust_code.contains(".remove("),
        "list.pop(index) should transpile to .remove(index)"
    );
}

#[test]
fn test_list_method_clear() {
    // Test list.clear() method
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_list():
    items = [1, 2, 3]
    items.clear()
    return items
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated list.clear() code:\n{}", rust_code);

    // Should generate .clear() for list.clear()
    assert!(
        rust_code.contains(".clear()"),
        "list.clear() should transpile to .clear()"
    );
}

#[test]
fn test_attribute_access_simple() {
    // Test simple attribute access
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

def get_x(p: Point) -> int:
    return p.x
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated attribute access code:\n{}", rust_code);

    // Should generate field access syntax
    assert!(
        rust_code.contains(".x"),
        "Attribute access p.x should transpile to .x field access"
    );
}

#[test]
fn test_tuple_unpacking() {
    // Test tuple unpacking in assignment
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def swap_values():
    a = 1
    b = 2
    a, b = b, a
    return (a, b)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated tuple unpacking code:\n{}", rust_code);

    // Should generate tuple pattern matching
    assert!(
        rust_code.contains("=") && rust_code.contains(","),
        "Tuple unpacking should work correctly"
    );
}

#[test]
fn test_lambda_simple() {
    // Test simple lambda expression
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def use_lambda():
    f = lambda x: x * 2
    return f(5)
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated lambda code:\n{}", rust_code);

    // Should generate closure syntax
    assert!(
        rust_code.contains("|") && rust_code.contains("*"),
        "Lambda should transpile to Rust closure"
    );
}

#[test]
fn test_ternary_expression() {
    // Test ternary (if-else) expression
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def test_ternary(x: int) -> str:
    return "positive" if x > 0 else "non-positive"
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated ternary expression code:\n{}", rust_code);

    // Should generate if-else expression
    assert!(
        rust_code.contains("if") && rust_code.contains("else"),
        "Ternary expression should use if-else"
    );
}

#[test]
fn test_set_comprehension() {
    // Test set comprehension
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_set_comp():
    return {x * 2 for x in range(5)}
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated set comprehension code:\n{}", rust_code);

    // Should use HashSet
    assert!(
        rust_code.contains("HashSet") && rust_code.contains(".collect"),
        "Set comprehension should create HashSet"
    );
}

#[test]
fn test_dict_comprehension() {
    // Test dict comprehension
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def make_dict_comp():
    return {x: x * 2 for x in range(5)}
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated dict comprehension code:\n{}", rust_code);

    // Should use HashMap
    assert!(
        rust_code.contains("HashMap") && rust_code.contains(".collect"),
        "Dict comprehension should create HashMap"
    );
}

#[test]
fn test_nested_list_comprehension() {
    // Test nested list comprehension
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def nested_comp():
    return [[y for y in range(3)] for x in range(2)]
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated nested comprehension code:\n{}", rust_code);

    // Should have nested .map() and .collect()
    assert!(
        rust_code.matches(".map(").count() >= 2 && rust_code.matches(".collect").count() >= 2,
        "Nested comprehension should have multiple .map() and .collect()"
    );
}

// ============================================================================
// DEPYLER-0649: String attribute containment - use .contains() not .get()
// ============================================================================

#[test]
fn test_DEPYLER_0649_string_attribute_containment_uses_contains() {
    // Bug: "string" in result.stdout generates .get().is_some() (dict style)
    // Expected: "string" in result.stdout generates .contains() (string style)
    let pipeline = DepylerPipeline::new();
    let python_code = r#"
def check_output(result):
    """Check if output contains expected text"""
    return "expected" in result.stdout
"#;

    let rust_code = pipeline.transpile(python_code).unwrap();
    println!("Generated string attribute containment code:\n{}", rust_code);

    // Should use .contains() for string containment, NOT .get().is_some()
    assert!(
        !rust_code.contains(".get("),
        "DEPYLER-0649: String attribute containment should NOT use .get() (dict style)"
    );
    assert!(
        rust_code.contains(".contains("),
        "DEPYLER-0649: String attribute containment should use .contains() (string style)"
    );
}
