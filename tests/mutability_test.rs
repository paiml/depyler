#![allow(non_snake_case)]

use depyler_core::DepylerPipeline;

// ============================================================================
// String Assignment
// ============================================================================

#[test]
fn test_string_constant_assignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    name: str
    status: str

def assign_string_constant(state: State) -> None:
    state.name = "Alpha"
    state.status = "Active"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_string_variable_assignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    name: str

def assign_string_variable(state: State) -> None:
    current_name = state.name
    new_name = current_name
    state.name = new_name
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_string_comparison() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    name: str
    status: str

def compare_strings(state: State) -> bool:
    if state.name == "Alpha":
        return True
    return state.status == "Active"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_string_concatenation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    name: str

def mutate_strings(state: State) -> None:
    prefix = "New"
    state.name = prefix + state.name
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_multiple_string_operations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    name: str
    result: str

def multiple_string_ops(state: State) -> None:
    result_str: str = ""
    result_str = "NotAttempted"
    state.result = result_str
    
    if state.name == "Alpha":
        result_str = "Attempted"
        state.result = result_str
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("let mut result_str"));
}

#[test]
fn test_string_conditional_assignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    status: str
    result: str

def string_comparison_assignment(state: State) -> None:
    attempt_result: str = ""
    
    if state.status == "Processing":
        attempt_result = "Attempted"
    else:
        attempt_result = "NotAttempted"
    
    state.result = attempt_result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("let mut attempt_result"));
}

// ============================================================================
// Function State Passing
// ============================================================================

#[test]
fn test_simple_function_call() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def helper_function_a(state: State) -> None:
    state.counter = state.counter + 10

def caller_function_simple(state: State) -> None:
    helper_function_a(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_multiple_function_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    name: str

def helper_function_a(state: State) -> None:
    state.counter = state.counter + 10

def helper_function_b(state: State) -> None:
    state.name = "Updated"

def caller_function_simple(state: State) -> None:
    helper_function_a(state)
    helper_function_b(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_function_with_return() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    flag: bool

def helper_function_with_return(state: State) -> bool:
    state.flag = True
    return state.counter > 50

def caller_function_with_return(state: State) -> None:
    result = helper_function_with_return(state)
    if result:
        state.counter = 100
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_function_with_params() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    name: str

def helper_function_with_params(state: State, add_value: int, set_name: str) -> None:
    state.counter = state.counter + add_value
    state.name = set_name

def caller_function_with_params(state: State) -> None:
    helper_function_with_params(state, 20, "Alpha")
    if state.counter > 30:
        helper_function_with_params(state, 10, "Beta")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // Note: Currently the transpiler may use value types for state when there are additional parameters
    // This assertion tests for the ideal case where state should be &mut State
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("add_value:"));
    assert!(rust_code.contains("set_name:"));
}

#[test]
fn test_conditional_function_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    name: str

def helper_function_a(state: State) -> None:
    state.counter = state.counter + 10

def helper_function_b(state: State) -> None:
    state.name = "Updated"

def caller_function_conditional(state: State) -> None:
    if state.counter > 0:
        helper_function_a(state)
    else:
        helper_function_b(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_nested_function_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    flag: bool

def helper_function_a(state: State) -> None:
    state.counter = state.counter + 10

def helper_function_with_return(state: State) -> bool:
    state.flag = True
    return state.counter > 50

def caller_function_simple(state: State) -> None:
    helper_function_a(state)

def caller_function_with_return(state: State) -> None:
    result = helper_function_with_return(state)
    if result:
        state.counter = 100

def top_level_function(state: State) -> None:
    caller_function_simple(state)
    caller_function_with_return(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// List/Array Mutability
// ============================================================================

#[test]
fn test_list_append() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def append_to_list(state: State) -> None:
    state.items.append(42)
    state.items.append(100)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // When calling mutating methods on fields, state must be &mut
    // The method call itself doesn't need explicit &mut (it's implicit)
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("state.items.push"));
}

#[test]
fn test_list_element_assignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def modify_list_element(state: State) -> None:
    state.values[0] = 999
    state.values[2] = state.values[1] + 10
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_list_read_only() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def read_list(state: State) -> int:
    total = 0
    for val in state.values:
        total = total + val
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_list_extend() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[str]

def extend_list(state: State) -> None:
    new_items = ["a", "b", "c"]
    state.items.extend(new_items)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // When calling mutating methods on fields, state must be &mut
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("state.items.extend"));
}

#[test]
fn test_list_clear() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def clear_list(state: State) -> None:
    state.items.clear()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // When calling mutating methods on fields, state must be &mut
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("state.items.clear"));
}

// ============================================================================
// For Loop Mutability
// ============================================================================

#[test]
fn test_for_loop_with_state_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    sum: int

def loop_with_mutation(state: State) -> None:
    for i in range(10):
        state.counter = state.counter + 1
        state.sum = state.sum + i
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_for_loop_read_only() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def loop_read_only(state: State) -> int:
    total = 0
    for value in state.values:
        total = total + value
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_for_loop_mutating_list_elements() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def double_values(state: State) -> None:
    for i in range(len(state.values)):
        state.values[i] = state.values[i] * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_for_loop_with_conditional_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    values: list[int]

def conditional_loop_mutation(state: State) -> None:
    for value in state.values:
        if value > 50:
            state.counter = state.counter + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Rust-Style Iterator Borrowing
// ============================================================================

#[test]
fn test_iterate_with_immutable_reference() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def iterate_immutable(state: State) -> int:
    total = 0
    for item in state.items:  # Should translate to: for item in &state.items
        total = total + item
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    assert!(rust_code.contains("&state.items"));
}

#[test]
fn test_iterate_with_mutable_reference_and_modify() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def iterate_and_modify_elements(state: State) -> None:
    # In Rust this should use: for item in &mut state.values
    for i in range(len(state.values)):
        state.values[i] = state.values[i] * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_iterate_without_modifying_items() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    names: list[str]

def print_names(state: State) -> None:
    for name in state.names:  # Read-only iteration
        pass  # In real code: print(name)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // Should use &State since we're only reading
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_iterate_and_collect_references() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def find_large_values(state: State) -> list[int]:
    result: list[int] = []
    for value in state.values:
        if value > 100:
            result.append(value)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_enumerate_with_index() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[str]

def find_index(state: State, target: str) -> int:
    for i, item in enumerate(state.items):
        if item == target:
            return i
    return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    // DEPYLER-0318: For enumerate with field access, should use .iter().enumerate()
    assert!(rust_code.contains(".iter().enumerate()"));
}

#[test]
fn test_enumerate_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    indices: list[int]

def collect_large_indices(state: State) -> None:
    for i, value in enumerate(state.values):
        if value > 50:
            state.indices.append(i)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_iterate_multiple_lists_zip() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values_a: list[int]
    values_b: list[int]

def sum_pairs(state: State) -> int:
    total = 0
    for a, b in zip(state.values_a, state.values_b):
        total = total + a + b
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    // Should use .iter().zip() in Rust
}

#[test]
fn test_reverse_iteration() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def sum_reversed(state: State) -> int:
    total = 0
    for item in reversed(state.items):
        total = total + item
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    // DEPYLER-0318: For reversed with field access, should use .iter().rev()
    assert!(rust_code.contains(".iter().rev()"));
}

#[test]
fn test_consuming_iteration_with_filter() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    filtered: list[int]

def filter_values(state: State, threshold: int) -> None:
    state.filtered = [v for v in state.values if v > threshold]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    // List comprehension should use .iter() and .collect()
}

#[test]
fn test_nested_loop_iterations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    matrix: list[list[int]]

def sum_matrix(state: State) -> int:
    total = 0
    for row in state.matrix:
        for value in row:
            total = total + value
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_iter_mut_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Item:
    value: int

@dataclass
class State:
    items: list[Item]

def increment_all_items(state: State) -> None:
    for item in state.items:
        item.value = item.value + 1  # Modifying items in-place
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("&mut state.items") || rust_code.contains("state.items.iter_mut"));
}

#[test]
fn test_loop_variable_reassignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def iterate_with_reassignment(state: State) -> int:
    x: int = 0
    for item in state.items:
        x = item
    return x
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    assert!(rust_code.contains("let mut x"));
    assert!(rust_code.contains("&state.items"));
}

#[test]
fn test_nested_field_access_iteration() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Inner:
    items: list[int]

@dataclass
class Middle:
    inner: Inner

@dataclass
class State:
    middle: Middle

def process(state: State) -> int:
    total = 0
    for item in state.middle.inner.items:
        total = total + item
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    assert!(rust_code.contains("&state.middle.inner.items"));
}

#[test]
fn test_nested_field_access_with_enumerate() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Inner:
    items: list[str]

@dataclass
class Middle:
    inner: Inner

@dataclass
class State:
    middle: Middle

def find_in_nested(state: State, target: str) -> int:
    for i, item in enumerate(state.middle.inner.items):
        if item == target:
            return i
    return -1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    assert!(rust_code.contains(".iter().enumerate()"));
}

#[test]
fn test_nested_field_access_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Item:
    value: int

@dataclass
class Inner:
    items: list[Item]

@dataclass
class Middle:
    inner: Inner

@dataclass
class State:
    middle: Middle

def increment_nested(state: State) -> None:
    for item in state.middle.inner.items:
        item.value = item.value + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    // DEPYLER-0318: Should use &mut for nested field access when mutating
    assert!(
        rust_code.contains("&mut state.middle.inner.items") || rust_code.contains(".iter_mut()")
    );
}

// ============================================================================
// Local Variable Mutability
// ============================================================================

#[test]
fn test_mutable_local_variable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def use_mutable_local(state: State) -> None:
    temp: int = 0
    temp = state.value
    temp = temp + 10
    state.value = temp
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("let mut temp"));
}

#[test]
fn test_mutable_local_list() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def build_local_list(state: State) -> None:
    temp_list: list[int] = []
    temp_list.append(1)
    temp_list.append(2)
    temp_list.extend(state.values)
    state.values = temp_list
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("let mut temp_list"));
}

#[test]
fn test_immutable_local_variable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def use_immutable_local(state: State) -> int:
    temp = state.value + 10
    return temp * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
    assert!(!rust_code.contains("let mut temp"));
}

// ============================================================================
// While Loop Mutability
// ============================================================================

#[test]
fn test_while_loop_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    limit: int

def while_loop_mutation(state: State) -> None:
    while state.counter < state.limit:
        state.counter = state.counter + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_while_loop_read_only() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    limit: int

def while_loop_read(state: State) -> int:
    temp = 0
    while temp < state.limit:
        temp = temp + 1
    return temp
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

// ============================================================================
// Dictionary/HashMap Mutability
// ============================================================================

#[test]
fn test_dict_field_insertion() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    data: dict[str, int]

def insert_into_dict(state: State) -> None:
    state.data["key1"] = 100
    state.data["key2"] = 200
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_dict_field_read() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    data: dict[str, int]

def read_from_dict(state: State) -> int:
    value = state.data.get("key1", 0)
    return value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

// ============================================================================
// Complex Nested Mutability
// ============================================================================

#[test]
fn test_nested_list_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    matrix: list[list[int]]

def modify_nested_list(state: State) -> None:
    state.matrix[0][1] = 999
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_multiple_field_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    items: list[str]
    data: dict[str, int]

def mutate_multiple_fields(state: State) -> None:
    state.counter = state.counter + 1
    state.items.append("new")
    state.data["key"] = 42
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    // Method calls don't need explicit &mut
    assert!(rust_code.contains("state.items.push"));
}

#[test]
fn test_conditional_mutation_branches() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    flag: bool
    counter: int
    items: list[int]

def conditional_mutations(state: State) -> None:
    if state.flag:
        state.counter = 100
    else:
        state.items.append(50)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    // Method calls don't need explicit &mut
    assert!(rust_code.contains("state.items.push"));
}

// ============================================================================
// Multiple Mutable Object Parameters
// ============================================================================

#[test]
fn test_two_mutable_object_parameters() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class RecordA:
    value: int
    name: str

@dataclass
class RecordB:
    count: int
    active: bool

def update_both(record_a: RecordA, record_b: RecordB) -> None:
    record_a.value = record_a.value + 10
    record_b.count = record_b.count + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("record_a: &mut RecordA"));
    assert!(rust_code.contains("record_b: &mut RecordB"));
}

#[test]
fn test_one_mutable_one_immutable_parameter() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    max_value: int
    threshold: int

@dataclass
class State:
    counter: int
    valid: bool

def update_state_from_config(state: State, config: Config) -> None:
    if state.counter > config.threshold:
        state.valid = True
    state.counter = config.max_value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("config: &Config"));
}

#[test]
fn test_mutable_object_with_primitive_parameters() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    name: str

def add_value(state: State, value: int, label: str) -> None:
    state.values.append(value)
    state.name = label
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("value: i32")); // i32 is the default for Python int
    assert!(rust_code.contains("label: &str"));
}

#[test]
fn test_three_object_parameters_mixed_mutability() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Source:
    data: list[int]

@dataclass
class Destination:
    results: list[int]

@dataclass
class Config:
    multiplier: int

def process_data(source: Source, dest: Destination, config: Config) -> None:
    for value in source.data:
        dest.results.append(value * config.multiplier)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("source: &Source"));
    assert!(rust_code.contains("dest: &mut Destination"));
    assert!(rust_code.contains("config: &Config"));
}

#[test]
fn test_passing_mutable_object_to_helper() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Counter:
    value: int
    total: int

def increment(counter: Counter, amount: int) -> None:
    counter.value = counter.value + amount
    counter.total = counter.total + amount

def process(counter: Counter) -> None:
    increment(counter, 5)
    increment(counter, 10)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // Both functions should take &mut Counter
    assert!(rust_code.contains("counter: &mut Counter"));
    assert!(rust_code.contains("amount: i32")); // i32 is the default for Python int
}

#[test]
fn test_mutable_list_parameter() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    numbers: list[int]

def modify_list(items: list[int]) -> None:
    items.append(42)
    items.append(100)

def use_helper(state: State) -> None:
    modify_list(state.numbers)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // modify_list should take &mut Vec<i64>
    assert!(rust_code.contains("items: &mut Vec"));
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_nested_object_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Inner:
    value: int

@dataclass
class Outer:
    inner: Inner
    count: int

def mutate_nested(outer: Outer) -> None:
    outer.inner.value = 100
    outer.count = outer.count + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("outer: &mut Outer"));
}

#[test]
fn test_object_parameter_only_read() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Data:
    values: list[int]
    name: str

def calculate_sum(data: Data) -> int:
    total = 0
    for value in data.values:
        total = total + value
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // Should be immutable reference since only reading
    assert!(rust_code.contains("data: &Data"));
}

#[test]
fn test_multiple_objects_passed_through_chain() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Input:
    value: int

@dataclass
class Output:
    result: int

def helper(input: Input, output: Output) -> None:
    output.result = input.value * 2

def caller(input: Input, output: Output) -> None:
    helper(input, output)
    output.result = output.result + 10
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("input: &Input"));
    assert!(rust_code.contains("output: &mut Output"));
}

#[test]
fn test_conditional_mutation_of_parameters() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class StateA:
    flag: bool
    counter: int

@dataclass
class StateB:
    counter: int

def conditional_update(state_a: StateA, state_b: StateB) -> None:
    if state_a.flag:
        state_a.counter = 100
    else:
        state_b.counter = 200
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    // Both should be mutable since both can be mutated
    assert!(rust_code.contains("state_a: &mut StateA"));
    assert!(rust_code.contains("state_b: &mut StateB"));
}

#[test]
fn test_swap_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Container:
    value: int

def swap_values(a: Container, b: Container) -> None:
    temp = a.value
    a.value = b.value
    b.value = temp
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("a: &mut Container"));
    assert!(rust_code.contains("b: &mut Container"));
}

#[test]
fn test_dict_parameter_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    data: dict[str, int]

def update_dict(data: dict[str, int], key: str, value: int) -> None:
    data[key] = value

def use_helper(state: State) -> None:
    update_dict(state.data, "key1", 100)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("data: &mut HashMap"));
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_indirect_mutation_by_value() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int

def mutate(state: State) -> None:
    state.x = 1

def indirect_mutate(state: State) -> None:
    mutate(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("fn mutate(state: &mut State)"));
    assert!(rust_code.contains("fn indirect_mutate(mut state: State)"));
    assert!(rust_code.contains("mutate(&mut state)"));
}

#[test]
fn test_pass_by_value_chain() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int

def first(state: State) -> None:
    state.x = 10
    second(state)
    third(state)

def second(state: State) -> None:
    third(state)

def third(state: State) -> None:
    pass
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("pub fn first(mut state: State)"));
    assert!(rust_code.contains("second(state.clone())"));
    assert!(rust_code.contains("third(state.clone())"));
    assert!(rust_code.contains("second(mut state: State)"));
    assert!(rust_code.contains("third(mut state: State)"));
}

#[test]
fn test_multi_function_state_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int

def first(state: State) -> None:
    state.x = 10
    second(state)

def second(state: State) -> None:
    state.x = 20
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    assert!(rust_code.contains("pub fn first(mut state: State)"));
    assert!(rust_code.contains("pub fn second(state: &mut State)"));
    assert!(rust_code.contains("second(&mut state)"));
}
