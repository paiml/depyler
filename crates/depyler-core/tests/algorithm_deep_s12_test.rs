//! Session 12 Batch 69: Algorithm patterns for deep codegen coverage
//!
//! Complex algorithm implementations that exercise many codegen
//! paths simultaneously through feature interaction.

use depyler_core::ast_bridge::AstBridge;
use depyler_core::rust_gen::generate_rust_file;
use depyler_core::type_mapper::TypeMapper;
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

#[test]
fn test_s12_b69_quicksort() {
    let code = r#"
def quicksort(items: list) -> list:
    if len(items) <= 1:
        return items
    pivot = items[0]
    left = [x for x in items[1:] if x <= pivot]
    right = [x for x in items[1:] if x > pivot]
    return quicksort(left) + [pivot] + quicksort(right)
"#;
    let result = transpile(code);
    assert!(result.contains("fn quicksort"), "Got: {}", result);
}

#[test]
fn test_s12_b69_merge_sort() {
    let code = r#"
def merge_sort(items: list) -> list:
    if len(items) <= 1:
        return items
    mid = len(items) // 2
    left = merge_sort(items[:mid])
    right = merge_sort(items[mid:])
    return merge(left, right)

def merge(left: list, right: list) -> list:
    result = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn merge_sort"), "Got: {}", result);
    assert!(result.contains("fn merge"), "Got: {}", result);
}

#[test]
fn test_s12_b69_dijkstra() {
    let code = r#"
def shortest_path(graph: dict, start: str, end: str) -> int:
    dist = {start: 0}
    visited = set()
    queue = [start]
    while queue:
        current = queue[0]
        for i in range(1, len(queue)):
            if dist.get(queue[i], 999999) < dist.get(current, 999999):
                current = queue[i]
        queue.remove(current)
        if current == end:
            return dist[current]
        if current in visited:
            continue
        visited.add(current)
        if current in graph:
            for neighbor, weight in graph[current]:
                new_dist = dist[current] + weight
                if new_dist < dist.get(neighbor, 999999):
                    dist[neighbor] = new_dist
                    queue.append(neighbor)
    return -1
"#;
    let result = transpile(code);
    assert!(result.contains("fn shortest_path"), "Got: {}", result);
}

#[test]
fn test_s12_b69_topological_sort() {
    let code = r#"
def topo_sort(graph: dict) -> list:
    in_degree = {}
    for node in graph:
        if node not in in_degree:
            in_degree[node] = 0
        for neighbor in graph[node]:
            if neighbor not in in_degree:
                in_degree[neighbor] = 0
            in_degree[neighbor] += 1
    queue = []
    for node, degree in in_degree.items():
        if degree == 0:
            queue.append(node)
    result = []
    while queue:
        node = queue.pop(0)
        result.append(node)
        if node in graph:
            for neighbor in graph[node]:
                in_degree[neighbor] -= 1
                if in_degree[neighbor] == 0:
                    queue.append(neighbor)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn topo_sort"), "Got: {}", result);
}

#[test]
fn test_s12_b69_knapsack() {
    let code = r#"
def knapsack(weights: list, values: list, capacity: int) -> int:
    n = len(weights)
    dp = []
    for i in range(n + 1):
        row = [0] * (capacity + 1)
        dp.append(row)
    for i in range(1, n + 1):
        for w in range(capacity + 1):
            dp[i][w] = dp[i - 1][w]
            if weights[i - 1] <= w:
                val = dp[i - 1][w - weights[i - 1]] + values[i - 1]
                if val > dp[i][w]:
                    dp[i][w] = val
    return dp[n][capacity]
"#;
    let result = transpile(code);
    assert!(result.contains("fn knapsack"), "Got: {}", result);
}

#[test]
fn test_s12_b69_lcs() {
    let code = r#"
def lcs(s1: str, s2: str) -> str:
    m = len(s1)
    n = len(s2)
    dp = []
    for i in range(m + 1):
        row = [0] * (n + 1)
        dp.append(row)
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
    result = ""
    i = m
    j = n
    while i > 0 and j > 0:
        if s1[i - 1] == s2[j - 1]:
            result = s1[i - 1] + result
            i -= 1
            j -= 1
        elif dp[i - 1][j] > dp[i][j - 1]:
            i -= 1
        else:
            j -= 1
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn lcs"), "Got: {}", result);
}

#[test]
fn test_s12_b69_trie_operations() {
    let code = r##"
class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False
        self.count = 0

def build_trie(words: list) -> TrieNode:
    root = TrieNode()
    for word in words:
        node = root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
            node.count += 1
        node.is_end = True
    return root

def count_prefix(root: TrieNode, prefix: str) -> int:
    node = root
    for char in prefix:
        if char not in node.children:
            return 0
        node = node.children[char]
    return node.count
"##;
    let result = transpile(code);
    assert!(result.contains("fn build_trie"), "Got: {}", result);
    assert!(result.contains("fn count_prefix"), "Got: {}", result);
}

#[test]
fn test_s12_b69_matrix_multiply() {
    let code = r#"
def mat_mul(a: list, b: list) -> list:
    rows_a = len(a)
    cols_a = len(a[0])
    cols_b = len(b[0])
    result = []
    for i in range(rows_a):
        row = []
        for j in range(cols_b):
            total = 0.0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result
"#;
    let result = transpile(code);
    assert!(result.contains("fn mat_mul"), "Got: {}", result);
}
