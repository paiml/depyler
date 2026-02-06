//! Session 12 Batch 87: Data structure implementations
//!
//! Complex data structure implementations that exercise
//! class codegen, method generation, and type inference paths.

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
fn test_s12_b87_stack() {
    let code = r#"
class Stack:
    def __init__(self):
        self.items = []

    def push(self, item: int):
        self.items.append(item)

    def pop(self) -> int:
        return self.items.pop()

    def peek(self) -> int:
        return self.items[-1]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Stack"), "Got: {}", result);
}

#[test]
fn test_s12_b87_queue() {
    let code = r#"
class Queue:
    def __init__(self):
        self.items = []

    def enqueue(self, item: int):
        self.items.append(item)

    def dequeue(self) -> int:
        return self.items.pop(0)

    def front(self) -> int:
        return self.items[0]

    def is_empty(self) -> bool:
        return len(self.items) == 0

    def size(self) -> int:
        return len(self.items)
"#;
    let result = transpile(code);
    assert!(result.contains("Queue"), "Got: {}", result);
}

#[test]
fn test_s12_b87_hash_map() {
    let code = r#"
class HashMap:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.buckets = [[] for _ in range(capacity)]
        self.size = 0

    def _hash(self, key: str) -> int:
        h = 0
        for c in key:
            h = h * 31 + ord(c)
        return h % self.capacity

    def put(self, key: str, value: int):
        idx = self._hash(key)
        bucket = self.buckets[idx]
        for i, pair in enumerate(bucket):
            if pair[0] == key:
                bucket[i] = (key, value)
                return
        bucket.append((key, value))
        self.size += 1

    def get(self, key: str) -> int:
        idx = self._hash(key)
        for pair in self.buckets[idx]:
            if pair[0] == key:
                return pair[1]
        return -1
"#;
    let result = transpile(code);
    assert!(result.contains("HashMap"), "Got: {}", result);
}

#[test]
fn test_s12_b87_binary_search_tree() {
    let code = r#"
class TreeNode:
    def __init__(self, value: int):
        self.value = value
        self.left = None
        self.right = None

def bst_insert(root, value: int):
    if root is None:
        return TreeNode(value)
    if value < root.value:
        root.left = bst_insert(root.left, value)
    elif value > root.value:
        root.right = bst_insert(root.right, value)
    return root

def bst_search(root, target: int) -> bool:
    if root is None:
        return False
    if target == root.value:
        return True
    if target < root.value:
        return bst_search(root.left, target)
    return bst_search(root.right, target)

def inorder(root) -> list:
    if root is None:
        return []
    return inorder(root.left) + [root.value] + inorder(root.right)
"#;
    let result = transpile(code);
    assert!(result.contains("TreeNode"), "Got: {}", result);
    assert!(result.contains("fn bst_insert"), "Got: {}", result);
    assert!(result.contains("fn bst_search"), "Got: {}", result);
}

#[test]
fn test_s12_b87_ring_buffer() {
    let code = r#"
class RingBuffer:
    def __init__(self, capacity: int):
        self.data = [0] * capacity
        self.capacity = capacity
        self.head = 0
        self.count = 0

    def push(self, item: int):
        idx = (self.head + self.count) % self.capacity
        self.data[idx] = item
        if self.count < self.capacity:
            self.count += 1
        else:
            self.head = (self.head + 1) % self.capacity

    def get(self, index: int) -> int:
        if index >= self.count:
            return -1
        idx = (self.head + index) % self.capacity
        return self.data[idx]

    def size(self) -> int:
        return self.count
"#;
    let result = transpile(code);
    assert!(result.contains("RingBuffer"), "Got: {}", result);
}

#[test]
fn test_s12_b87_disjoint_set() {
    let code = r#"
class DisjointSet:
    def __init__(self, n: int):
        self.parent = list(range(n))
        self.rank = [0] * n

    def find(self, x: int) -> int:
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x: int, y: int) -> bool:
        px = self.find(x)
        py = self.find(y)
        if px == py:
            return False
        if self.rank[px] < self.rank[py]:
            self.parent[px] = py
        elif self.rank[px] > self.rank[py]:
            self.parent[py] = px
        else:
            self.parent[py] = px
            self.rank[px] += 1
        return True
"#;
    let result = transpile(code);
    assert!(result.contains("DisjointSet"), "Got: {}", result);
}

#[test]
fn test_s12_b87_lru_cache() {
    let code = r#"
class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = {}
        self.order = []

    def get(self, key: str) -> int:
        if key not in self.cache:
            return -1
        self.order.remove(key)
        self.order.append(key)
        return self.cache[key]

    def put(self, key: str, value: int):
        if key in self.cache:
            self.order.remove(key)
        elif len(self.cache) >= self.capacity:
            oldest = self.order.pop(0)
            del self.cache[oldest]
        self.cache[key] = value
        self.order.append(key)
"#;
    let result = transpile(code);
    assert!(result.contains("LRUCache"), "Got: {}", result);
}

#[test]
fn test_s12_b87_trie() {
    let code = r#"
class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str):
        node = self.root
        for c in word:
            if c not in node.children:
                node.children[c] = TrieNode()
            node = node.children[c]
        node.is_word = True

    def search(self, word: str) -> bool:
        node = self.root
        for c in word:
            if c not in node.children:
                return False
            node = node.children[c]
        return node.is_word

    def starts_with(self, prefix: str) -> bool:
        node = self.root
        for c in prefix:
            if c not in node.children:
                return False
            node = node.children[c]
        return True
"#;
    let result = transpile(code);
    assert!(result.contains("Trie"), "Got: {}", result);
}
