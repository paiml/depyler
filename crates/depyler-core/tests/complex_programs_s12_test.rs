//! Session 12 Batch 60: Complex program patterns
//!
//! End-to-end transpilation of complex real-world programs that
//! exercise many code paths simultaneously, maximizing coverage.

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
fn test_s12_b60_mini_database() {
    let code = r##"
class Table:
    def __init__(self, name: str, columns: list):
        self.name = name
        self.columns = columns
        self.rows = []

    def insert(self, row: dict):
        self.rows.append(row)

    def select(self, column: str) -> list:
        result = []
        for row in self.rows:
            if column in row:
                result.append(row[column])
        return result

    def where_eq(self, column: str, value) -> list:
        result = []
        for row in self.rows:
            if column in row and row[column] == value:
                result.append(row)
        return result

    def count(self) -> int:
        return len(self.rows)

    def delete_where(self, column: str, value) -> int:
        original = len(self.rows)
        self.rows = [r for r in self.rows if column not in r or r[column] != value]
        return original - len(self.rows)
"##;
    let result = transpile(code);
    assert!(result.contains("Table"), "Got: {}", result);
}

#[test]
fn test_s12_b60_calculator() {
    let code = r##"
class Calculator:
    def __init__(self):
        self.result = 0.0
        self.history = []

    def add(self, x: float) -> float:
        self.history.append(f"+ {x}")
        self.result += x
        return self.result

    def subtract(self, x: float) -> float:
        self.history.append(f"- {x}")
        self.result -= x
        return self.result

    def multiply(self, x: float) -> float:
        self.history.append(f"* {x}")
        self.result *= x
        return self.result

    def divide(self, x: float) -> float:
        if x == 0.0:
            raise ValueError("Cannot divide by zero")
        self.history.append(f"/ {x}")
        self.result /= x
        return self.result

    def reset(self):
        self.result = 0.0
        self.history = []

    def get_history(self) -> list:
        return self.history
"##;
    let result = transpile(code);
    assert!(result.contains("Calculator"), "Got: {}", result);
}

#[test]
fn test_s12_b60_tokenizer() {
    let code = r##"
def tokenize(text: str) -> list:
    tokens = []
    current = ""
    for c in text:
        if c.isspace():
            if current:
                tokens.append(current)
                current = ""
        elif c in "+-*/()=<>!":
            if current:
                tokens.append(current)
                current = ""
            tokens.append(c)
        elif c == '"':
            if current:
                tokens.append(current)
                current = ""
            # Read string literal
            current = '"'
        else:
            current += c
    if current:
        tokens.append(current)
    return tokens
"##;
    let result = transpile(code);
    assert!(result.contains("fn tokenize"), "Got: {}", result);
}

#[test]
fn test_s12_b60_inventory_system() {
    let code = r##"
class Product:
    def __init__(self, name: str, price: float, quantity: int):
        self.name = name
        self.price = price
        self.quantity = quantity

class Inventory:
    def __init__(self):
        self.products = {}

    def add_product(self, name: str, price: float, quantity: int):
        if name in self.products:
            self.products[name].quantity += quantity
        else:
            self.products[name] = Product(name, price, quantity)

    def sell(self, name: str, qty: int) -> bool:
        if name not in self.products:
            return False
        if self.products[name].quantity < qty:
            return False
        self.products[name].quantity -= qty
        return True

    def total_value(self) -> float:
        total = 0.0
        for name, product in self.products.items():
            total += product.price * product.quantity
        return total

    def low_stock(self, threshold: int) -> list:
        result = []
        for name, product in self.products.items():
            if product.quantity < threshold:
                result.append(name)
        return result
"##;
    let result = transpile(code);
    assert!(result.contains("Inventory"), "Got: {}", result);
}

#[test]
fn test_s12_b60_event_system() {
    let code = r##"
class EventBus:
    def __init__(self):
        self.listeners = {}

    def on(self, event: str, callback):
        if event not in self.listeners:
            self.listeners[event] = []
        self.listeners[event].append(callback)

    def emit(self, event: str, data):
        if event in self.listeners:
            for callback in self.listeners[event]:
                callback(data)

    def off(self, event: str):
        if event in self.listeners:
            del self.listeners[event]

    def has_listeners(self, event: str) -> bool:
        return event in self.listeners and len(self.listeners[event]) > 0
"##;
    let result = transpile(code);
    assert!(result.contains("EventBus"), "Got: {}", result);
}

#[test]
fn test_s12_b60_csv_processor() {
    let code = r##"
def parse_csv(text: str) -> list:
    rows = []
    for line in text.strip().split("\n"):
        if line.strip():
            rows.append(line.split(","))
    return rows

def csv_to_dicts(text: str) -> list:
    rows = parse_csv(text)
    if len(rows) < 2:
        return []
    headers = rows[0]
    result = []
    for row in rows[1:]:
        record = {}
        for i in range(min(len(headers), len(row))):
            record[headers[i].strip()] = row[i].strip()
        result.append(record)
    return result

def filter_csv(records: list, column: str, value: str) -> list:
    return [r for r in records if column in r and r[column] == value]

def sort_csv(records: list, column: str) -> list:
    return sorted(records, key=lambda r: r.get(column, ""))
"##;
    let result = transpile(code);
    assert!(result.contains("fn parse_csv"), "Got: {}", result);
    assert!(result.contains("fn csv_to_dicts"), "Got: {}", result);
}

#[test]
fn test_s12_b60_graph_algorithms() {
    let code = r##"
def dfs(graph: dict, start: str) -> list:
    visited = set()
    result = []
    stack = [start]
    while stack:
        node = stack.pop()
        if node in visited:
            continue
        visited.add(node)
        result.append(node)
        if node in graph:
            for neighbor in graph[node]:
                if neighbor not in visited:
                    stack.append(neighbor)
    return result

def bfs(graph: dict, start: str) -> list:
    visited = set()
    result = []
    queue = [start]
    while queue:
        node = queue.pop(0)
        if node in visited:
            continue
        visited.add(node)
        result.append(node)
        if node in graph:
            for neighbor in graph[node]:
                if neighbor not in visited:
                    queue.append(neighbor)
    return result

def has_cycle(graph: dict) -> bool:
    visited = set()
    for node in graph:
        if node in visited:
            continue
        stack = [(node, None)]
        while stack:
            current, parent = stack.pop()
            if current in visited:
                return True
            visited.add(current)
            if current in graph:
                for neighbor in graph[current]:
                    if neighbor != parent:
                        stack.append((neighbor, current))
    return False
"##;
    let result = transpile(code);
    assert!(result.contains("fn dfs"), "Got: {}", result);
    assert!(result.contains("fn bfs"), "Got: {}", result);
}

#[test]
fn test_s12_b60_string_processor() {
    let code = r##"
def camel_to_snake(name: str) -> str:
    result = ""
    for i, c in enumerate(name):
        if c.isupper() and i > 0:
            result += "_"
        result += c.lower()
    return result

def snake_to_camel(name: str) -> str:
    parts = name.split("_")
    result = parts[0]
    for part in parts[1:]:
        if part:
            result += part[0].upper() + part[1:]
    return result

def truncate(text: str, max_len: int, suffix: str) -> str:
    if len(text) <= max_len:
        return text
    return text[:max_len - len(suffix)] + suffix

def wrap_text(text: str, width: int) -> str:
    words = text.split()
    lines = []
    current_line = ""
    for word in words:
        if current_line and len(current_line) + 1 + len(word) > width:
            lines.append(current_line)
            current_line = word
        elif current_line:
            current_line += " " + word
        else:
            current_line = word
    if current_line:
        lines.append(current_line)
    return "\n".join(lines)
"##;
    let result = transpile(code);
    assert!(result.contains("fn camel_to_snake"), "Got: {}", result);
    assert!(result.contains("fn snake_to_camel"), "Got: {}", result);
    assert!(result.contains("fn truncate"), "Got: {}", result);
    assert!(result.contains("fn wrap_text"), "Got: {}", result);
}

#[test]
fn test_s12_b60_priority_queue() {
    let code = r#"
class PriorityQueue:
    def __init__(self):
        self.items = []

    def push(self, item, priority: int):
        self.items.append((priority, item))
        self.items.sort()

    def pop(self):
        if not self.items:
            return None
        return self.items.pop(0)[1]

    def peek(self):
        if not self.items:
            return None
        return self.items[0][1]

    def size(self) -> int:
        return len(self.items)

    def is_empty(self) -> bool:
        return len(self.items) == 0
"#;
    let result = transpile(code);
    assert!(result.contains("PriorityQueue"), "Got: {}", result);
}

#[test]
fn test_s12_b60_dp_algorithms() {
    let code = r#"
def longest_increasing(items: list) -> int:
    if not items:
        return 0
    n = len(items)
    dp = [1] * n
    for i in range(1, n):
        for j in range(i):
            if items[j] < items[i]:
                dp[i] = max(dp[i], dp[j] + 1)
    return max(dp)

def coin_change(coins: list, amount: int) -> int:
    dp = [amount + 1] * (amount + 1)
    dp[0] = 0
    for i in range(1, amount + 1):
        for coin in coins:
            if coin <= i:
                dp[i] = min(dp[i], dp[i - coin] + 1)
    if dp[amount] > amount:
        return -1
    return dp[amount]

def edit_distance(s1: str, s2: str) -> int:
    m = len(s1)
    n = len(s2)
    dp = []
    for i in range(m + 1):
        row = []
        for j in range(n + 1):
            if i == 0:
                row.append(j)
            elif j == 0:
                row.append(i)
            else:
                row.append(0)
        dp.append(row)
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1]
            else:
                dp[i][j] = 1 + min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1])
    return dp[m][n]
"#;
    let result = transpile(code);
    assert!(result.contains("fn longest_increasing"), "Got: {}", result);
    assert!(result.contains("fn coin_change"), "Got: {}", result);
    assert!(result.contains("fn edit_distance"), "Got: {}", result);
}
