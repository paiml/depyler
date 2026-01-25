//! DEPYLER-0969: Deque Type Tracking Tests
//!
//! This test module validates that `deque()` constructor calls are properly tracked
//! in `var_types` to enable correct truthiness conversion.
//!
//! Key conversions:
//! - `while queue:` → `while !queue.is_empty()` (when queue is VecDeque)
//! - `if stack:` → `if !stack.is_empty()` (when stack is a collection)

use depyler_core::DepylerPipeline;

#[test]
fn test_deque_truthiness_in_while_loop() {
    // Python BFS pattern with deque should use .is_empty() in while condition
    let python = r#"
from collections import deque

def bfs(start: int) -> list[int]:
    queue = deque([start])
    visited = []
    while queue:
        node = queue.popleft()
        visited.append(node)
    return visited
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Should generate !queue.is_empty() (not just `queue` which is invalid)
    assert!(
        rust_code.contains(".is_empty()"),
        "Should convert `while queue:` to `while !queue.is_empty()`\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_queue_variable_heuristic() {
    // Variable named 'queue' should trigger is_empty() conversion even without deque()
    let python = r#"
def process_items(items: list[int]) -> list[int]:
    queue = items.copy()
    result = []
    while queue:
        item = queue.pop()
        result.append(item)
    return result
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // The heuristic should trigger based on variable name 'queue'
    assert!(
        rust_code.contains(".is_empty()"),
        "Variable named 'queue' should use .is_empty() heuristic\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_stack_variable_heuristic() {
    // Variable named 'stack' should trigger is_empty() conversion
    let python = r#"
def dfs(start: int) -> list[int]:
    stack = [start]
    visited = []
    while stack:
        node = stack.pop()
        visited.append(node)
    return visited
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // The heuristic should trigger based on variable name 'stack'
    assert!(
        rust_code.contains(".is_empty()"),
        "Variable named 'stack' should use .is_empty() heuristic\n\nGenerated:\n{}",
        rust_code
    );
}

#[test]
fn test_visited_set_truthiness() {
    // Variable named 'visited' that is a set should use .is_empty()
    let python = r#"
def has_visited(nodes: list[int]) -> bool:
    visited = set()
    for node in nodes:
        visited.add(node)
    if visited:
        return True
    return False
"#;

    let pipeline = DepylerPipeline::new();
    let result = pipeline.transpile(python);
    assert!(
        result.is_ok(),
        "Transpilation should succeed: {:?}",
        result.err()
    );

    let rust_code = result.unwrap();

    // Sets should use .is_empty()
    assert!(
        rust_code.contains(".is_empty()"),
        "Set truthiness should use .is_empty()\n\nGenerated:\n{}",
        rust_code
    );
}
