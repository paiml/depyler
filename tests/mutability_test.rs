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

// ============================================================================
// Early Returns with Mutation
// ============================================================================

#[test]
fn test_early_return_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    valid: bool

def process_with_early_return(state: State, threshold: int) -> bool:
    state.counter = state.counter + 1

    if state.counter > threshold:
        state.valid = False
        return False

    state.valid = True
    return True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_multiple_early_returns_different_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    status: str
    counter: int
    flag: bool

def multi_exit(state: State, value: int) -> int:
    if value < 0:
        state.status = "negative"
        return -1

    if value == 0:
        return 0

    if value > 100:
        state.counter = 100
        state.flag = True
        return 100

    state.counter = value
    return value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Method Chaining and Fluent APIs
// ============================================================================

#[test]
fn test_multiple_method_calls_same_field() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    items: list[int]

def chain_operations(state: State) -> None:
    state.items.append(1)
    state.items.append(2)
    state.items.append(3)
    state.items.extend([4, 5, 6])
    state.items.clear()
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_alternating_field_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    list_a: list[int]
    list_b: list[int]
    counter: int

def alternate_mutations(state: State) -> None:
    state.list_a.append(1)
    state.counter = state.counter + 1
    state.list_b.append(2)
    state.list_a.extend([3, 4])
    state.counter = state.counter + 1
    state.list_b.extend([5, 6])
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Exception Handling with Mutation
// ============================================================================

#[test]
fn test_try_except_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    error_count: int
    success_count: int
    last_error: str

def risky_operation(state: State, value: int) -> bool:
    try:
        result = 100 / value
        state.success_count = state.success_count + 1
        return True
    except:
        state.error_count = state.error_count + 1
        state.last_error = "Division error"
        return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_try_except_finally_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    attempts: int
    errors: int
    cleanups: int

def operation_with_cleanup(state: State) -> None:
    try:
        state.attempts = state.attempts + 1
        # risky operation
    except:
        state.errors = state.errors + 1
    finally:
        state.cleanups = state.cleanups + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Aliasing and Reference Patterns
// ============================================================================

// ============================================================================
// Partial Mutation Paths
// ============================================================================

#[test]
fn test_mutation_in_some_branches_only() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    status: str

def partial_mutation(state: State, flag: bool) -> None:
    if flag:
        state.counter = state.counter + 1
    # else branch doesn't mutate
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_nested_conditional_partial_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int
    flag: bool
    status: str

def nested_partial(state: State, a: bool, b: bool) -> None:
    if a:
        if b:
            state.value = 100
        # else: no mutation
    else:
        state.flag = True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Read-After-Write Patterns
// ============================================================================

#[test]
fn test_read_after_write_same_field() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def read_after_write(state: State) -> int:
    state.counter = 100
    result = state.counter + 50
    state.counter = result
    return state.counter
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_complex_read_write_dependencies() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    a: int
    b: int
    c: int

def dependent_mutations(state: State) -> None:
    state.a = 10
    state.b = state.a * 2
    state.c = state.a + state.b
    state.a = state.c
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Recursive Functions with Mutation
// ============================================================================

#[test]
fn test_recursive_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    items: list[int]

def recursive_mutate(state: State, n: int) -> None:
    if n <= 0:
        return

    state.counter = state.counter + 1
    state.items.append(n)
    recursive_mutate(state, n - 1)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_mutual_recursion_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def func_a(state: State, n: int) -> None:
    if n <= 0:
        return
    state.counter = state.counter + 1
    func_b(state, n - 1)

def func_b(state: State, n: int) -> None:
    if n <= 0:
        return
    state.counter = state.counter + 2
    func_a(state, n - 1)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Optional/Nullable Field Mutations
// ============================================================================

#[test]
fn test_optional_field_mutation() {
    let python = r#"
from dataclasses import dataclass
from typing import Optional

@dataclass
class State:
    value: Optional[int]
    name: Optional[str]

def mutate_optional(state: State) -> None:
    state.value = 42
    state.name = "test"

    if state.value is not None:
        state.value = state.value + 10
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_optional_to_none() {
    let python = r#"
from dataclasses import dataclass
from typing import Optional

@dataclass
class State:
    value: Optional[int]

def clear_optional(state: State) -> None:
    state.value = None
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Comprehensions with Side Effects
// ============================================================================

#[test]
fn test_comprehension_with_state_read() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    threshold: int

def filter_values(state: State) -> list[int]:
    return [v for v in state.values if v > state.threshold]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &State"));
}

#[test]
fn test_comprehension_result_mutates_state() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    filtered: list[int]

def update_filtered(state: State, threshold: int) -> None:
    state.filtered = [v * 2 for v in state.values if v > threshold]
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Mutation Order Dependencies
// ============================================================================

#[test]
fn test_order_dependent_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int
    y: int
    z: int

def order_matters(state: State) -> None:
    # Order matters: x depends on original y, y depends on original z
    temp_y = state.y
    temp_z = state.z
    state.x = temp_y
    state.y = temp_z
    state.z = state.x + state.y
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Deep Nested Object Mutations
// ============================================================================

#[test]
fn test_deeply_nested_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Level3:
    value: int

@dataclass
class Level2:
    level3: Level3
    count: int

@dataclass
class Level1:
    level2: Level2
    flag: bool

@dataclass
class State:
    level1: Level1

def deep_mutate(state: State) -> None:
    state.level1.level2.level3.value = 100
    state.level1.level2.count = 50
    state.level1.flag = True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_nested_list_of_objects() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Item:
    value: int

@dataclass
class Container:
    items: list[Item]

@dataclass
class State:
    containers: list[Container]

def mutate_nested_objects(state: State) -> None:
    state.containers[0].items[0].value = 999
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Mutation Through Return Values
// ============================================================================

#[test]
fn test_return_and_mutate_separately() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    result: int

def compute_and_store(state: State) -> int:
    state.counter = state.counter + 1
    result = state.counter * 2
    state.result = result
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_return_tuple_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def multi_return(state: State) -> tuple[int, int]:
    old_value = state.counter
    state.counter = state.counter + 1
    return (old_value, state.counter)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Mutation in Nested Loops
// ============================================================================

#[test]
fn test_nested_loop_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    matrix: list[list[int]]

def nested_loop_mutate(state: State) -> None:
    for row in state.matrix:
        for value in row:
            state.counter = state.counter + value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_nested_loop_with_break_and_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    found: bool
    position: int

def find_in_nested(state: State, matrix: list[list[int]], target: int) -> None:
    for i, row in enumerate(matrix):
        for j, value in enumerate(row):
            if value == target:
                state.found = True
                state.position = i * 100 + j
                return
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Boolean Flag Patterns
// ============================================================================

#[test]
fn test_boolean_flag_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    initialized: bool
    configured: bool
    ready: bool

def setup(state: State) -> None:
    state.initialized = True
    state.configured = True
    state.ready = state.initialized and state.configured
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_flag_toggle() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    active: bool

def toggle(state: State) -> None:
    state.active = not state.active
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Accumulator Patterns
// ============================================================================

#[test]
fn test_accumulator_with_reduce_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    total: int
    count: int
    average: int

def accumulate(state: State, values: list[int]) -> None:
    state.total = 0
    state.count = 0

    for value in values:
        state.total = state.total + value
        state.count = state.count + 1

    if state.count > 0:
        state.average = state.total / state.count
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

#[test]
fn test_running_statistics() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    min_val: int
    max_val: int
    sum_val: int

def update_stats(state: State, new_value: int) -> None:
    if new_value < state.min_val:
        state.min_val = new_value

    if new_value > state.max_val:
        state.max_val = new_value

    state.sum_val = state.sum_val + new_value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// State Machine Patterns
// ============================================================================

#[test]
fn test_state_machine_transitions() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    status: str
    transition_count: int

def transition(state: State, event: str) -> None:
    state.transition_count = state.transition_count + 1

    if state.status == "idle" and event == "start":
        state.status = "running"
    elif state.status == "running" and event == "pause":
        state.status = "paused"
    elif state.status == "paused" and event == "resume":
        state.status = "running"
    elif state.status == "running" and event == "stop":
        state.status = "stopped"
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Mutation with Guard Clauses
// ============================================================================

#[test]
fn test_guard_clauses_with_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    error_code: int
    processed: bool
    result: int

def process_with_guards(state: State, value: int) -> bool:
    if value < 0:
        state.error_code = -1
        return False

    if value > 1000:
        state.error_code = -2
        return False

    state.processed = True
    state.result = value * 2
    return True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
}

// ============================================================================
// Builder Pattern-Like Mutations
// ============================================================================

#[test]
fn test_builder_pattern_mutations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    host: str
    port: int
    timeout: int
    retries: int

def build_config(config: Config) -> None:
    config.host = "localhost"
    config.port = 8080
    config.timeout = 30
    config.retries = 3
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("config: &mut Config"));
}

#[test]
fn test_conditional_builder() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    host: str
    port: int
    use_ssl: bool
    ssl_port: int

def configure(config: Config, enable_ssl: bool) -> None:
    config.host = "localhost"

    if enable_ssl:
        config.use_ssl = True
        config.port = 443
        config.ssl_port = 443
    else:
        config.use_ssl = False
        config.port = 80
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("config: &mut Config"));
}

// ============================================================================
// Mixed Mutability Function Chains
// ============================================================================

#[test]
fn test_mutable_calls_immutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    threshold: int

def check_threshold(state: State) -> bool:
    return state.counter > state.threshold

def increment_if_below(state: State) -> None:
    if check_threshold(state):
        return
    state.counter = state.counter + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn check_threshold(state: &State)"));
    assert!(rust_code.contains("fn increment_if_below(state: &mut State)"));
}

#[test]
fn test_immutable_calls_immutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]
    threshold: int

def sum_values(state: State) -> int:
    total = 0
    for v in state.values:
        total = total + v
    return total

def average_above_threshold(state: State) -> bool:
    total = sum_values(state)
    avg = total / len(state.values) if len(state.values) > 0 else 0
    return avg > state.threshold
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn sum_values(state: &State)"));
    assert!(rust_code.contains("fn average_above_threshold(state: &State)"));
}

#[test]
fn test_mutable_calls_multiple_immutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int
    y: int
    result: int

def get_x(state: State) -> int:
    return state.x

def get_y(state: State) -> int:
    return state.y

def compute_result(state: State) -> None:
    a = get_x(state)
    b = get_y(state)
    state.result = a + b
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn get_x(state: &State)"));
    assert!(rust_code.contains("fn get_y(state: &State)"));
    assert!(rust_code.contains("fn compute_result(state: &mut State)"));
}

// ============================================================================
// Pass-Through Functions
// ============================================================================

#[test]
fn test_passthrough_to_mutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def increment(state: State) -> None:
    state.counter = state.counter + 1

def passthrough_increment(state: State) -> None:
    increment(state)
    increment(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn increment(state: &mut State)"));
    assert!(rust_code.contains("fn passthrough_increment(state: &mut State)"));
}

#[test]
fn test_passthrough_to_immutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def read_value(state: State) -> int:
    return state.value

def passthrough_read(state: State) -> int:
    return read_value(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn read_value(state: &State)"));
    assert!(rust_code.contains("fn passthrough_read(state: &State)"));
}

#[test]
fn test_passthrough_mixed_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    max_value: int

def get_max(state: State) -> int:
    return state.max_value

def increment(state: State) -> None:
    state.counter = state.counter + 1

def process(state: State) -> None:
    max_val = get_max(state)
    if state.counter < max_val:
        increment(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn get_max(state: &State)"));
    assert!(rust_code.contains("fn increment(state: &mut State)"));
    assert!(rust_code.contains("fn process(state: &mut State)"));
}

// ============================================================================
// Multiple State Parameters
// ============================================================================

#[test]
fn test_two_states_both_mutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class StateA:
    value: int

@dataclass
class StateB:
    value: int

def increment_both(a: StateA, b: StateB) -> None:
    a.value = a.value + 1
    b.value = b.value + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("a: &mut StateA"));
    assert!(rust_code.contains("b: &mut StateB"));
}

#[test]
fn test_two_states_one_mutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Source:
    value: int

@dataclass
class Dest:
    value: int

def copy_value(source: Source, dest: Dest) -> None:
    dest.value = source.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("source: &Source"));
    assert!(rust_code.contains("dest: &mut Dest"));
}

#[test]
fn test_two_states_both_immutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class StateA:
    value: int

@dataclass
class StateB:
    value: int

def compare_states(a: StateA, b: StateB) -> bool:
    return a.value == b.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("a: &StateA"));
    assert!(rust_code.contains("b: &StateB"));
}

#[test]
fn test_chain_with_multiple_states() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Input:
    value: int

@dataclass
class Output:
    result: int

def transform(input: Input, output: Output) -> None:
    output.result = input.value * 2

def process_chain(input: Input, output: Output) -> None:
    transform(input, output)
    output.result = output.result + 10
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn transform(input: &Input, output: &mut Output)"));
    assert!(rust_code.contains("fn process_chain(input: &Input, output: &mut Output)"));
}

// ============================================================================
// Conditional Function Calls with Different Mutability
// ============================================================================

// ============================================================================
// Deep Call Chains
// ============================================================================

#[test]
fn test_deep_chain_all_mutable() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def level_3(state: State) -> None:
    state.counter = state.counter + 1

def level_2(state: State) -> None:
    level_3(state)
    state.counter = state.counter + 1

def level_1(state: State) -> None:
    level_2(state)
    state.counter = state.counter + 1

def top_level(state: State) -> None:
    level_1(state)
    state.counter = state.counter + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn level_3(state: &mut State)"));
    assert!(rust_code.contains("fn level_2(state: &mut State)"));
    assert!(rust_code.contains("fn level_1(state: &mut State)"));
    assert!(rust_code.contains("fn top_level(state: &mut State)"));
}

#[test]
fn test_deep_chain_mixed_mutability() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    threshold: int

def read_threshold(state: State) -> int:
    return state.threshold

def check_and_increment(state: State) -> None:
    threshold = read_threshold(state)
    if state.counter < threshold:
        state.counter = state.counter + 1

def process_multiple(state: State) -> None:
    check_and_increment(state)
    check_and_increment(state)

def top_process(state: State) -> None:
    process_multiple(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn read_threshold(state: &State)"));
    assert!(rust_code.contains("fn check_and_increment(state: &mut State)"));
    assert!(rust_code.contains("fn process_multiple(state: &mut State)"));
    assert!(rust_code.contains("fn top_process(state: &mut State)"));
}

// ============================================================================
// Loop-Based Function Calls
// ============================================================================

#[test]
fn test_loop_calling_mutable_function() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def increment(state: State) -> None:
    state.counter = state.counter + 1

def increment_n_times(state: State, n: int) -> None:
    for i in range(n):
        increment(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn increment(state: &mut State)"));
    assert!(rust_code.contains("fn increment_n_times(state: &mut State"));
}

#[test]
fn test_loop_calling_immutable_function() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    values: list[int]

def get_sum(state: State) -> int:
    total = 0
    for v in state.values:
        total = total + v
    return total

def sum_multiple_times(state: State, n: int) -> int:
    total = 0
    for i in range(n):
        total = total + get_sum(state)
    return total
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn get_sum(state: &State)"));
    assert!(rust_code.contains("fn sum_multiple_times(state: &State"));
}

#[test]
fn test_loop_with_conditional_mutable_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    threshold: int

def increment(state: State) -> None:
    state.counter = state.counter + 1

def process_items(state: State, items: list[int]) -> None:
    for item in items:
        if item > state.threshold:
            increment(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn increment(state: &mut State)"));
    assert!(rust_code.contains("fn process_items(state: &mut State"));
}

// ============================================================================
// Return Value Transformations
// ============================================================================

#[test]
fn test_immutable_return_used_for_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int
    result: int

def compute(state: State) -> int:
    return state.value * 2

def apply_computation(state: State) -> None:
    state.result = compute(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn compute(state: &State)"));
    assert!(rust_code.contains("fn apply_computation(state: &mut State)"));
}

#[test]
fn test_chain_of_transformations() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    input: int
    intermediate: int
    output: int

def step1(state: State) -> int:
    return state.input * 2

def step2(value: int) -> int:
    return value + 10

def apply_pipeline(state: State) -> None:
    state.intermediate = step1(state)
    state.output = step2(state.intermediate)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn step1(state: &State)"));
    assert!(rust_code.contains("fn apply_pipeline(state: &mut State)"));
}

// ============================================================================
// Callback-Style Patterns
// ============================================================================

#[test]
fn test_function_calls_another_with_result() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    valid: bool

def validate(state: State) -> bool:
    return state.counter > 0

def process_if_valid(state: State) -> None:
    if validate(state):
        state.counter = state.counter + 1
        state.valid = True
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn validate(state: &State)"));
    assert!(rust_code.contains("fn process_if_valid(state: &mut State)"));
}

#[test]
fn test_multiple_validators_before_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int
    flag: bool
    result: int

def check_value(state: State) -> bool:
    return state.value > 0

def check_flag(state: State) -> bool:
    return state.flag

def update_if_valid(state: State) -> None:
    if check_value(state) and check_flag(state):
        state.result = state.value * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn check_value(state: &State)"));
    assert!(rust_code.contains("fn check_flag(state: &State)"));
    assert!(rust_code.contains("fn update_if_valid(state: &mut State)"));
}

// ============================================================================
// State Splitting Patterns
// ============================================================================

#[test]
fn test_function_operates_on_different_objects() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    multiplier: int

@dataclass
class State:
    value: int

def apply_config(state: State, config: Config) -> None:
    state.value = state.value * config.multiplier
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("state: &mut State"));
    assert!(rust_code.contains("config: &Config"));
}

#[test]
fn test_three_objects_chain() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Input:
    value: int

@dataclass
class Config:
    factor: int

@dataclass
class Output:
    result: int

def process(input: Input, config: Config, output: Output) -> None:
    output.result = input.value * config.factor
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("input: &Input"));
    assert!(rust_code.contains("config: &Config"));
    assert!(rust_code.contains("output: &mut Output"));
}

// ============================================================================
// Partial Application Patterns
// ============================================================================

#[test]
fn test_helper_with_extra_params() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def add_amount(state: State, amount: int) -> None:
    state.value = state.value + amount

def increment_by_ten(state: State) -> None:
    add_amount(state, 10)

def increment_by_value(state: State, value: int) -> None:
    add_amount(state, value)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn add_amount(state: &mut State"));
    assert!(rust_code.contains("fn increment_by_ten(state: &mut State)"));
    assert!(rust_code.contains("fn increment_by_value(state: &mut State"));
}

#[test]
fn test_wrapper_functions_with_defaults() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int
    y: int

def set_values(state: State, x: int, y: int) -> None:
    state.x = x
    state.y = y

def reset(state: State) -> None:
    set_values(state, 0, 0)

def set_to_ten(state: State) -> None:
    set_values(state, 10, 10)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn set_values(state: &mut State"));
    assert!(rust_code.contains("fn reset(state: &mut State)"));
    assert!(rust_code.contains("fn set_to_ten(state: &mut State)"));
}

// ============================================================================
// Interleaved Reads and Writes
// ============================================================================

#[test]
fn test_read_write_read_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int
    previous: int

def get_counter(state: State) -> int:
    return state.counter

def save_and_increment(state: State) -> None:
    old_value = get_counter(state)
    state.previous = old_value
    state.counter = state.counter + 1
    new_value = get_counter(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn get_counter(state: &State)"));
    assert!(rust_code.contains("fn save_and_increment(state: &mut State)"));
}

#[test]
fn test_alternating_read_write_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int
    doubled: int

def get_value(state: State) -> int:
    return state.value

def set_value(state: State, val: int) -> None:
    state.value = val

def double_value(state: State) -> None:
    current = get_value(state)
    doubled = current * 2
    state.doubled = doubled
    set_value(state, doubled)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn get_value(state: &State)"));
    assert!(rust_code.contains("fn set_value(state: &mut State"));
    assert!(rust_code.contains("fn double_value(state: &mut State)"));
}

// ============================================================================
// Complex Multi-Object Scenarios
// ============================================================================

#[test]
fn test_aggregation_from_multiple_sources() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Source1:
    value: int

@dataclass
class Source2:
    value: int

@dataclass
class Aggregate:
    total: int

def aggregate(s1: Source1, s2: Source2, agg: Aggregate) -> None:
    agg.total = s1.value + s2.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("s1: &Source1"));
    assert!(rust_code.contains("s2: &Source2"));
    assert!(rust_code.contains("agg: &mut Aggregate"));
}

#[test]
fn test_fan_out_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Source:
    value: int

@dataclass
class Dest1:
    value: int

@dataclass
class Dest2:
    value: int

def distribute(source: Source, d1: Dest1, d2: Dest2) -> None:
    d1.value = source.value
    d2.value = source.value * 2
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("source: &Source"));
    assert!(rust_code.contains("d1: &mut Dest1"));
    assert!(rust_code.contains("d2: &mut Dest2"));
}

// ============================================================================
// Self-Referential Updates
// ============================================================================

#[test]
fn test_value_depends_on_self() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def double_counter(state: State) -> None:
    state.counter = state.counter * 2

def apply_n_times(state: State, n: int) -> None:
    for i in range(n):
        double_counter(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn double_counter(state: &mut State)"));
    assert!(rust_code.contains("fn apply_n_times(state: &mut State"));
}

#[test]
fn test_fibonacci_like_updates() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    prev: int
    curr: int

def advance(state: State) -> None:
    next_val = state.prev + state.curr
    state.prev = state.curr
    state.curr = next_val
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn advance(state: &mut State)"));
}

// ============================================================================
// Observer Pattern-Like
// ============================================================================

#[test]
fn test_notify_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Subject:
    value: int
    changed: bool

@dataclass
class Observer:
    last_seen: int
    notify_count: int

def notify(observer: Observer, value: int) -> None:
    observer.last_seen = value
    observer.notify_count = observer.notify_count + 1

def update_subject(subject: Subject, observer: Observer) -> None:
    subject.value = subject.value + 1
    subject.changed = True
    notify(observer, subject.value)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn notify(observer: &mut Observer"));
    assert!(rust_code.contains("fn update_subject(subject: &mut Subject, observer: &mut Observer)"));
}

// ============================================================================
// Transaction-Like Patterns
// ============================================================================

#[test]
fn test_begin_commit_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int
    pending: int
    committed: bool

def begin_transaction(state: State, value: int) -> None:
    state.pending = value
    state.committed = False

def commit_transaction(state: State) -> None:
    state.value = state.pending
    state.committed = True

def do_transaction(state: State, value: int) -> None:
    begin_transaction(state, value)
    commit_transaction(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn begin_transaction(state: &mut State"));
    assert!(rust_code.contains("fn commit_transaction(state: &mut State)"));
    assert!(rust_code.contains("fn do_transaction(state: &mut State"));
}

// ============================================================================
// Factory/Constructor-Like Patterns
// ============================================================================

#[test]
fn test_initialize_pattern() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    initialized: bool
    value: int
    name: str

def initialize(state: State, value: int, name: str) -> None:
    state.value = value
    state.name = name
    state.initialized = True

def create_default(state: State) -> None:
    initialize(state, 0, "default")

def create_custom(state: State) -> None:
    initialize(state, 100, "custom")
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn initialize(state: &mut State"));
    assert!(rust_code.contains("fn create_default(state: &mut State)"));
    assert!(rust_code.contains("fn create_custom(state: &mut State)"));
}

// ============================================================================
// Edge Cases from Design Document
// ============================================================================

#[test]
fn test_read_only_function_not_contaminated() {
    // This test ensures read-only functions don't get &mut
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def get_counter(state: State) -> int:
    return state.counter

def increment(state: State) -> None:
    state.counter += 1

def conditional_operation(state: State, should_increment: bool) -> int:
    if should_increment:
        increment(state)
    return get_counter(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // get_counter should use &State (read-only)
    assert!(
        rust_code.contains("fn get_counter(state: &State)")
            || rust_code.contains("pub fn get_counter(state: &State)"),
        "get_counter should use &State, not &mut State"
    );

    // increment should use &mut State
    assert!(
        rust_code.contains("fn increment(state: &mut State)")
            || rust_code.contains("pub fn increment(state: &mut State)")
    );

    // conditional_operation should use &mut State (calls increment)
    assert!(
        rust_code.contains("fn conditional_operation(state: &mut State")
            || rust_code.contains("pub fn conditional_operation(state: &mut State")
    );
}

#[test]
fn test_conditional_mutation_paths() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def maybe_update(state: State, do_update: bool) -> int:
    if do_update:
        state.value = 42
    return state.value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Conservative: should use &mut because mutation happens in one path
    assert!(
        rust_code.contains("fn maybe_update(state: &mut State")
            || rust_code.contains("pub fn maybe_update(state: &mut State")
    );
}

#[test]
fn test_self_recursive_function() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    count: int

def countdown(state: State, n: int) -> None:
    if n > 0:
        state.count = n
        countdown(state, n - 1)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should converge to &mut State
    assert!(
        rust_code.contains("fn countdown(state: &mut State")
            || rust_code.contains("pub fn countdown(state: &mut State")
    );
}

#[test]
fn test_mutually_recursive_functions() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int
    y: int

def func_a(state: State, n: int) -> None:
    if n > 0:
        state.x = n
        func_b(state, n - 1)

def func_b(state: State, n: int) -> None:
    if n > 0:
        state.y = n
        func_a(state, n - 1)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Both should use &mut State
    assert!(
        rust_code.contains("fn func_a(state: &mut State")
            || rust_code.contains("pub fn func_a(state: &mut State")
    );
    assert!(
        rust_code.contains("fn func_b(state: &mut State")
            || rust_code.contains("pub fn func_b(state: &mut State")
    );
}

#[test]
fn test_unused_parameter() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def unused_param(state: State) -> int:
    return 42
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Unused parameter - could be owned, but at least shouldn't be &mut
    // (Implementation may choose & or T depending on strategy)
    assert!(
        !rust_code.contains("fn unused_param(state: &mut State"),
        "Unused parameter should not require &mut"
    );
}

#[test]
fn test_field_level_precision() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    read_field: int
    write_field: int

def read_one_write_other(state: State, value: int) -> int:
    result = state.read_field
    state.write_field = value
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should use &mut because write_field is mutated
    assert!(
        rust_code.contains("fn read_one_write_other(state: &mut State")
            || rust_code.contains("pub fn read_one_write_other(state: &mut State")
    );
}

#[test]
fn test_method_call_mutation_tracking() {
    let python = r#"
from dataclasses import dataclass
from typing import List

@dataclass
class State:
    items: List[int]

def add_item(state: State, item: int) -> None:
    state.items.append(item)

def get_items_count(state: State) -> int:
    return len(state.items)

def process(state: State, item: int) -> int:
    add_item(state, item)
    return get_items_count(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // add_item mutates (append is mutating)
    assert!(
        rust_code.contains("fn add_item(state: &mut State")
            || rust_code.contains("pub fn add_item(state: &mut State")
    );

    // get_items_count only reads
    assert!(
        rust_code.contains("fn get_items_count(state: &State)")
            || rust_code.contains("pub fn get_items_count(state: &State)")
    );

    // process calls add_item, so needs &mut
    assert!(
        rust_code.contains("fn process(state: &mut State")
            || rust_code.contains("pub fn process(state: &mut State")
    );
}

#[test]
#[ignore] // TODO: Aliasing detection is an advanced feature - will be implemented in future
fn test_aliasing_through_local_variable() {
    let python = r#"
from dataclasses import dataclass
from typing import List

@dataclass
class State:
    items: List[int]

def mutate_via_alias(state: State, value: int) -> None:
    local_ref = state.items
    local_ref.append(value)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should detect mutation through alias
    assert!(
        rust_code.contains("fn mutate_via_alias(state: &mut State")
            || rust_code.contains("pub fn mutate_via_alias(state: &mut State")
    );
}

#[test]
fn test_chained_function_calls() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    a: int
    b: int
    c: int

def set_a(state: State, val: int) -> None:
    state.a = val

def set_b(state: State, val: int) -> None:
    state.b = val

def set_c(state: State, val: int) -> None:
    state.c = val

def set_all(state: State, val: int) -> None:
    set_a(state, val)
    set_b(state, val)
    set_c(state, val)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // All should use &mut
    assert!(
        rust_code.contains("fn set_a(state: &mut State")
            || rust_code.contains("pub fn set_a(state: &mut State")
    );
    assert!(
        rust_code.contains("fn set_b(state: &mut State")
            || rust_code.contains("pub fn set_b(state: &mut State")
    );
    assert!(
        rust_code.contains("fn set_c(state: &mut State")
            || rust_code.contains("pub fn set_c(state: &mut State")
    );
    assert!(
        rust_code.contains("fn set_all(state: &mut State")
            || rust_code.contains("pub fn set_all(state: &mut State")
    );
}

#[test]
fn test_read_through_multiple_functions() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def read_a(state: State) -> int:
    return state.value

def read_b(state: State) -> int:
    return read_a(state)

def read_c(state: State) -> int:
    return read_b(state)
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // All should use &State (read-only chain)
    assert!(
        rust_code.contains("fn read_a(state: &State)")
            || rust_code.contains("pub fn read_a(state: &State)"),
        "read_a should use &State"
    );
    assert!(
        rust_code.contains("fn read_b(state: &State)")
            || rust_code.contains("pub fn read_b(state: &State)"),
        "read_b should use &State"
    );
    assert!(
        rust_code.contains("fn read_c(state: &State)")
            || rust_code.contains("pub fn read_c(state: &State)"),
        "read_c should use &State"
    );
}

#[test]
fn test_mixed_read_write_chain() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    x: int
    y: int

def read_x(state: State) -> int:
    return state.x

def write_y(state: State, val: int) -> None:
    state.y = val

def read_then_write(state: State, val: int) -> int:
    result = read_x(state)
    write_y(state, val)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // read_x should be &State
    assert!(
        rust_code.contains("fn read_x(state: &State)")
            || rust_code.contains("pub fn read_x(state: &State)")
    );

    // write_y should be &mut State
    assert!(
        rust_code.contains("fn write_y(state: &mut State")
            || rust_code.contains("pub fn write_y(state: &mut State")
    );

    // read_then_write calls write_y, so needs &mut
    assert!(
        rust_code.contains("fn read_then_write(state: &mut State")
            || rust_code.contains("pub fn read_then_write(state: &mut State")
    );
}

#[test]
fn test_multiple_parameters_different_mutability() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Config:
    multiplier: int

@dataclass
class State:
    value: int

def apply_config(state: State, config: Config) -> None:
    state.value = state.value * config.multiplier
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // state is mutated, config is only read
    assert!(rust_code.contains("state: &mut State") && rust_code.contains("config: &Config"));
}

#[test]
fn test_nested_attribute_mutation() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class Inner:
    value: int

@dataclass
class Outer:
    inner: Inner

def mutate_nested(outer: Outer, val: int) -> None:
    outer.inner.value = val
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Should detect nested mutation
    assert!(
        rust_code.contains("fn mutate_nested(outer: &mut Outer")
            || rust_code.contains("pub fn mutate_nested(outer: &mut Outer")
    );
}

#[test]
fn test_index_assignment_mutation() {
    let python = r#"
from dataclasses import dataclass
from typing import List

@dataclass
class State:
    items: List[int]

def update_item(state: State, index: int, value: int) -> None:
    state.items[index] = value
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Index assignment is mutation
    assert!(
        rust_code.contains("fn update_item(state: &mut State")
            || rust_code.contains("pub fn update_item(state: &mut State")
    );
}

#[test]
fn test_augmented_assignment() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    counter: int

def increment_by(state: State, amount: int) -> None:
    state.counter += amount
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Augmented assignment is mutation
    assert!(
        rust_code.contains("fn increment_by(state: &mut State")
            || rust_code.contains("pub fn increment_by(state: &mut State")
    );
}

#[test]
fn test_return_prevents_mut_requirement() {
    let python = r#"
from dataclasses import dataclass

@dataclass
class State:
    value: int

def get_value_wrapper(state: State) -> int:
    return state.value

def use_wrapper(state: State) -> int:
    return get_value_wrapper(state) + 1
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(result.is_ok());
    let rust_code = result.unwrap();

    // Both should be read-only
    assert!(
        rust_code.contains("fn get_value_wrapper(state: &State)")
            || rust_code.contains("pub fn get_value_wrapper(state: &State)")
    );
    assert!(
        rust_code.contains("fn use_wrapper(state: &State)")
            || rust_code.contains("pub fn use_wrapper(state: &State)")
    );
}
