# Python-to-Rust Language Mapping Specification

> **Complete technical specification for Python language constructs to Rust
> transpilation**

This document defines the precise mapping between Python language features and
their Rust equivalents as implemented by Depyler.

---

## 🎯 Specification Overview

### Supported Python Version

- **Target**: Python 3.8+
- **Type Hints**: Required for optimal transpilation
- **Compatibility**: PEP 484, 585, 604 type annotations

### Rust Target

- **Version**: Rust 1.75+
- **Edition**: 2021
- **Features**: Safe Rust only (no unsafe blocks)

---

## 📊 Type System Mapping

### Primitive Types

| Python Type | Rust Type          | Notes                       |
| ----------- | ------------------ | --------------------------- |
| `int`       | `i32` / `i64`      | Configurable based on range |
| `float`     | `f64`              | Always 64-bit for precision |
| `str`       | `String` / `&str`  | Strategy-dependent          |
| `bool`      | `bool`             | Direct mapping              |
| `None`      | `()` / `Option<T>` | Context-dependent           |

#### Type Inference Rules

```python
# Python
x = 42              # -> i32 (default)
y = 42000000000     # -> i64 (range detection)
z = 3.14            # -> f64
name = "Alice"      # -> String (owned by default)
active = True       # -> bool
```

```rust
// Generated Rust
let x: i32 = 42;
let y: i64 = 42000000000;
let z: f64 = 3.14;
let name: String = "Alice".to_string();
let active: bool = true;
```

### Collection Types

| Python Type        | Rust Type       | Implementation |
| ------------------ | --------------- | -------------- |
| `List[T]`          | `Vec<T>`        | Dynamic array  |
| `Dict[K, V]`       | `HashMap<K, V>` | Hash table     |
| `Set[T]`           | `HashSet<T>`    | Hash-based set |
| `Tuple[T, U, ...]` | `(T, U, ...)`   | Product type   |

#### Collection Examples

```python
# Python
numbers: List[int] = [1, 2, 3, 4, 5]
scores: Dict[str, float] = {"alice": 95.5, "bob": 87.2}
unique_ids: Set[int] = {1, 2, 3}
point: Tuple[float, float] = (3.14, 2.71)
```

```rust
// Generated Rust
use std::collections::{HashMap, HashSet};

let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
let mut scores: HashMap<String, f64> = HashMap::new();
scores.insert("alice".to_string(), 95.5);
scores.insert("bob".to_string(), 87.2);
let unique_ids: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let point: (f64, f64) = (3.14, 2.71);
```

### Optional Types

| Python Pattern   | Rust Type   | Strategy            |
| ---------------- | ----------- | ------------------- |
| `Optional[T]`    | `Option<T>` | Explicit handling   |
| `Union[T, None]` | `Option<T>` | Normalized form     |
| `T \| None`      | `Option<T>` | Python 3.10+ syntax |

```python
# Python
def find_user(id: int) -> Optional[User]:
    if id in users:
        return users[id]
    return None
```

```rust
// Generated Rust
fn find_user(id: i32) -> Option<User> {
    users.get(&id).cloned()
}
```

---

## 🔧 Function Mapping

### Function Signatures

| Python Feature    | Rust Equivalent   | Notes                      |
| ----------------- | ----------------- | -------------------------- |
| Function def      | `fn` declaration  | Direct mapping             |
| Return annotation | Return type       | Required for transpilation |
| Default arguments | Option parameters | Converted to Option<T>     |
| Keyword arguments | Struct parameters | When beneficial            |

#### Basic Functions

```python
# Python
def add(a: int, b: int) -> int:
    """Add two integers."""
    return a + b

def greet(name: str, greeting: str = "Hello") -> str:
    """Greet someone with optional greeting."""
    return f"{greeting}, {name}!"
```

```rust
// Generated Rust
/// Add two integers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greet someone with optional greeting.
pub fn greet(name: String, greeting: Option<String>) -> String {
    let greeting = greeting.unwrap_or_else(|| "Hello".to_string());
    format!("{}, {}!", greeting, name)
}
```

### Method Mapping

| Python Pattern  | Rust Pattern        | Implementation      |
| --------------- | ------------------- | ------------------- |
| Instance method | `&self` method      | Immutable reference |
| Mutating method | `&mut self` method  | Mutable reference   |
| Class method    | Associated function | Static function     |
| Static method   | Associated function | No self parameter   |

```python
# Python
class Counter:
    def __init__(self, start: int = 0):
        self.value = start
    
    def get(self) -> int:
        return self.value
    
    def increment(self) -> None:
        self.value += 1
    
    @classmethod
    def from_string(cls, s: str) -> 'Counter':
        return cls(int(s))
```

```rust
// Generated Rust
#[derive(Debug, Clone)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(start: Option<i32>) -> Self {
        Self {
            value: start.unwrap_or(0),
        }
    }
    
    pub fn get(&self) -> i32 {
        self.value
    }
    
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    pub fn from_string(s: String) -> Self {
        Self {
            value: s.parse().unwrap_or(0),
        }
    }
}
```

---

## 🔄 Control Flow Mapping

### Conditional Statements

| Python Construct | Rust Construct    | Notes                 |
| ---------------- | ----------------- | --------------------- |
| `if/elif/else`   | `if/else if/else` | Direct mapping        |
| Ternary operator | `if` expression   | Rust if is expression |
| Pattern matching | `match` statement | When beneficial       |

```python
# Python
def classify_number(n: int) -> str:
    if n > 0:
        return "positive"
    elif n < 0:
        return "negative"
    else:
        return "zero"

# Ternary
sign = "positive" if x > 0 else "non-positive"
```

```rust
// Generated Rust
pub fn classify_number(n: i32) -> String {
    if n > 0 {
        "positive".to_string()
    } else if n < 0 {
        "negative".to_string()
    } else {
        "zero".to_string()
    }
}

// Ternary equivalent
let sign = if x > 0 { "positive" } else { "non-positive" }.to_string();
```

### Loop Constructs

| Python Loop         | Rust Equivalent          | Implementation   |
| ------------------- | ------------------------ | ---------------- |
| `for x in iterable` | `for x in iterable`      | Iterator-based   |
| `while condition`   | `while condition`        | Direct mapping   |
| `for i in range(n)` | `for i in 0..n`          | Range syntax     |
| List comprehension  | `map`/`filter`/`collect` | Functional style |

```python
# Python
def sum_squares(numbers: List[int]) -> int:
    total = 0
    for num in numbers:
        total += num * num
    return total

# List comprehension
squares = [x * x for x in numbers if x > 0]
```

```rust
// Generated Rust
pub fn sum_squares(numbers: Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += num * num;
    }
    total
}

// List comprehension equivalent
let squares: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * x)
    .collect();
```

---

## ⚙️ Operator Mapping

### Arithmetic Operators

| Python Operator | Rust Operator | Notes                   |
| --------------- | ------------- | ----------------------- |
| `+`             | `+`           | Addition                |
| `-`             | `-`           | Subtraction             |
| `*`             | `*`           | Multiplication          |
| `/`             | `/`           | Division (float result) |
| `//`            | `/`           | Integer division        |
| `%`             | `%`           | Modulo                  |
| `**`            | `.pow()`      | Exponentiation          |

### Comparison Operators

| Python Operator | Rust Operator  | Notes                 |
| --------------- | -------------- | --------------------- |
| `==`            | `==`           | Equality              |
| `!=`            | `!=`           | Inequality            |
| `<`             | `<`            | Less than             |
| `<=`            | `<=`           | Less than or equal    |
| `>`             | `>`            | Greater than          |
| `>=`            | `>=`           | Greater than or equal |
| `is`            | `std::ptr::eq` | Identity comparison   |
| `in`            | `.contains()`  | Membership testing    |

### Logical Operators

| Python Operator | Rust Operator | Notes       |
| --------------- | ------------- | ----------- |
| `and`           | `&&`          | Logical AND |
| `or`            | `\|\|`        | Logical OR  |
| `not`           | `!`           | Logical NOT |

### Bitwise Operators

| Python Operator | Rust Operator | Notes       |
| --------------- | ------------- | ----------- |
| `&`             | `&`           | Bitwise AND |
| `\|`            | `\|`          | Bitwise OR  |
| `^`             | `^`           | Bitwise XOR |
| `~`             | `!`           | Bitwise NOT |
| `<<`            | `<<`          | Left shift  |
| `>>`            | `>>`          | Right shift |

---

## 🎯 Advanced Features

### Error Handling

| Python Pattern        | Rust Pattern   | Strategy                |
| --------------------- | -------------- | ----------------------- |
| `try/except`          | `Result<T, E>` | Explicit error handling |
| `raise Exception`     | `Err(error)`   | Return error            |
| Exception propagation | `?` operator   | Error bubbling          |

```python
# Python
def divide(a: float, b: float) -> float:
    try:
        if b == 0:
            raise ValueError("Division by zero")
        return a / b
    except ValueError as e:
        print(f"Error: {e}")
        return 0.0
```

```rust
// Generated Rust
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
}

pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn safe_divide(a: f64, b: f64) -> f64 {
    match divide(a, b) {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {:?}", e);
            0.0
        }
    }
}
```

### Context Managers

| Python Pattern   | Rust Pattern | Implementation     |
| ---------------- | ------------ | ------------------ |
| `with` statement | RAII + Drop  | Automatic cleanup  |
| File handling    | `std::fs`    | Built-in functions |
| Resource cleanup | Drop trait   | Automatic          |

```python
# Python
def read_config(filename: str) -> dict:
    with open(filename, 'r') as f:
        return json.load(f)
```

```rust
// Generated Rust
use std::fs;
use serde_json;

pub fn read_config(filename: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let config: serde_json::Value = serde_json::from_str(&content)?;
    Ok(config)
}
```

### Generators and Iterators

| Python Pattern       | Rust Pattern     | Implementation  |
| -------------------- | ---------------- | --------------- |
| Generator function   | Iterator impl    | Custom iterator |
| `yield`              | Iterator::next() | State machine   |
| Generator expression | Iterator chain   | Lazy evaluation |

```python
# Python
def fibonacci(n: int):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b
```

```rust
// Generated Rust
pub struct Fibonacci {
    current: u64,
    next: u64,
    remaining: usize,
}

impl Fibonacci {
    pub fn new(n: usize) -> Self {
        Self {
            current: 0,
            next: 1,
            remaining: n,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let current = self.current;
            let next = self.current + self.next;
            self.current = self.next;
            self.next = next;
            self.remaining -= 1;
            Some(current)
        }
    }
}

pub fn fibonacci(n: usize) -> Fibonacci {
    Fibonacci::new(n)
}
```

---

## 🔧 String Handling Strategies

### String Strategy Selection

| Strategy       | Use Case          | Rust Type  | Performance |
| -------------- | ----------------- | ---------- | ----------- |
| `always_owned` | Modifying strings | `String`   | High memory |
| `zero_copy`    | Read-only strings | `&str`     | Low memory  |
| `cow`          | Mixed usage       | `Cow<str>` | Balanced    |
| `smart`        | Auto-detection    | Mixed      | Optimal     |

#### Configuration Examples

```python
# @depyler: string_strategy = "zero_copy"
def process_text(text: str) -> str:
    return text.upper()

# @depyler: string_strategy = "always_owned"  
def modify_text(text: str) -> str:
    return text + " (modified)"
```

```rust
// Zero-copy strategy
pub fn process_text(text: &str) -> String {
    text.to_uppercase()
}

// Always-owned strategy
pub fn modify_text(text: String) -> String {
    format!("{} (modified)", text)
}
```

---

## 🛡️ Memory Management

### Ownership Strategies

| Strategy   | Description         | Use Case              | Performance |
| ---------- | ------------------- | --------------------- | ----------- |
| `owned`    | Take ownership      | Consuming functions   | High safety |
| `borrowed` | Immutable reference | Read-only access      | Zero-copy   |
| `mutable`  | Mutable reference   | In-place modification | Efficient   |
| `smart`    | Context-aware       | Mixed patterns        | Balanced    |

### Lifetime Management

| Python Pattern   | Rust Lifetime | Strategy            |
| ---------------- | ------------- | ------------------- |
| Return reference | `'a` lifetime | Explicit annotation |
| Borrowed data    | `&'a T`       | Reference borrowing |
| Self-reference   | `&'a self`    | Method borrowing    |

---

## 📋 Limitations and Workarounds

### Currently Unsupported

| Python Feature       | Status         | Workaround            |
| -------------------- | -------------- | --------------------- |
| `async/await`        | Planned (v0.2) | Use sync alternatives |
| Multiple inheritance | Complex        | Use composition       |
| Metaclasses          | Complex        | Use macros/traits     |
| `eval()`/`exec()`    | Security risk  | Static alternatives   |
| Dynamic typing       | Limited        | Add type hints        |

### Partial Support

| Feature             | Support Level | Notes                    |
| ------------------- | ------------- | ------------------------ |
| Class inheritance   | Basic         | Single inheritance only  |
| Decorators          | Limited       | Function decorators only |
| Import system       | Basic         | Standard library mapping |
| Regular expressions | Full          | Via `regex` crate        |

---

## 🔬 Quality Assurance

### Verification Strategies

| Level       | Description         | Tools               |
| ----------- | ------------------- | ------------------- |
| Syntax      | Valid Rust code     | `rustc` compilation |
| Semantics   | Equivalent behavior | Property testing    |
| Performance | Expected speedup    | Benchmarking        |
| Safety      | Memory safety       | Static analysis     |

### Testing Framework

```rust
// Generated property tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::*;
    
    #[quickcheck]
    fn test_add_commutative(a: i32, b: i32) -> bool {
        add(a, b) == add(b, a)
    }
    
    #[quickcheck]
    fn test_multiply_associative(a: i32, b: i32, c: i32) -> bool {
        multiply(multiply(a, b), c) == multiply(a, multiply(b, c))
    }
}
```

---

## 📚 Implementation Examples

### Complete Example: Calculator Module

```python
# calculator.py
from typing import List, Optional
from enum import Enum

class Operation(Enum):
    ADD = "add"
    SUBTRACT = "subtract" 
    MULTIPLY = "multiply"
    DIVIDE = "divide"

class Calculator:
    def __init__(self):
        self.history: List[float] = []
    
    def calculate(self, a: float, b: float, op: Operation) -> Optional[float]:
        """Perform calculation and store in history."""
        result = None
        
        if op == Operation.ADD:
            result = a + b
        elif op == Operation.SUBTRACT:
            result = a - b
        elif op == Operation.MULTIPLY:
            result = a * b
        elif op == Operation.DIVIDE:
            if b != 0:
                result = a / b
        
        if result is not None:
            self.history.append(result)
        
        return result
    
    def get_history(self) -> List[float]:
        """Get calculation history."""
        return self.history.copy()
    
    def clear_history(self) -> None:
        """Clear calculation history."""
        self.history.clear()
```

```rust
// Generated calculator.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct Calculator {
    history: Vec<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    /// Perform calculation and store in history.
    pub fn calculate(&mut self, a: f64, b: f64, op: Operation) -> Option<f64> {
        let result = match op {
            Operation::Add => Some(a + b),
            Operation::Subtract => Some(a - b),
            Operation::Multiply => Some(a * b),
            Operation::Divide => {
                if b != 0.0 {
                    Some(a / b)
                } else {
                    None
                }
            }
        };
        
        if let Some(value) = result {
            self.history.push(value);
        }
        
        result
    }
    
    /// Get calculation history.
    pub fn get_history(&self) -> Vec<f64> {
        self.history.clone()
    }
    
    /// Clear calculation history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 🔧 Configuration Reference

### Annotation Syntax

```python
# Performance annotations
# @depyler: optimization_level = "aggressive"
# @depyler: performance_critical = "true"
# @depyler: optimization_hint = "vectorize"

# Memory annotations  
# @depyler: ownership = "borrowed" | "owned" | "smart"
# @depyler: string_strategy = "zero_copy" | "always_owned" | "cow"

# Safety annotations
# @depyler: bounds_checking = "explicit" | "implicit"
# @depyler: panic_behavior = "convert_to_result" | "allow"

# Concurrency annotations
# @depyler: thread_safety = "send_sync" | "none"
```

---

## 📈 Evolution and Future

### Roadmap Integration

| Version | New Features        | Specification Updates    |
| ------- | ------------------- | ------------------------ |
| v0.2.0  | Async/await support | Async function mapping   |
| v0.3.0  | Advanced classes    | Inheritance patterns     |
| v0.4.0  | Generic types       | Type parameter mapping   |
| v0.5.0  | Macro system        | Code generation patterns |

For the latest specification updates and implementation details, see:

- **[GitHub Repository](https://github.com/paiml/depyler)**
- **[API Documentation](https://docs.rs/depyler)**

---

_This specification is a living document, updated with each Depyler release to
reflect the latest Python-to-Rust mapping capabilities._
