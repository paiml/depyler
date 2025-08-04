# API Reference

## Table of Contents

### Functions
- [`add`](#add)
- [`multiply`](#multiply)

### Classes
- [`Calculator`](#calculator)


---

# Generated Rust Documentation

This documentation was automatically generated from Python source code by the Depyler transpiler.

<details>
<summary>Original Python Source</summary>

```python
#!/usr/bin/env python3
"""Minimal example for documentation generation."""


def add(x: int, y: int) -> int:
    """Add two numbers together.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Sum of x and y
    """
    return x + y


def multiply(x: int, y: int) -> int:
    """Multiply two numbers.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Product of x and y
    """
    return x * y


class Calculator:
    """A simple calculator class."""
    
    def __init__(self):
        """Initialize the calculator."""
        self.result = 0
    
    def compute_sum(self, x: int, y: int) -> int:
        """Compute the sum of two numbers.
        
        Args:
            x: First number
            y: Second number
            
        Returns:
            Sum of x and y
        """
        self.result = x + y
        return self.result
```

</details>

## Module Overview

- **Functions**: 2
- **Classes**: 1
- **Imports**: 0


## Functions

### `add`

```rust
fn add(x: i32, y: i32) -> i32
```

Add two numbers together.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Sum of x and y
    

**Parameters:**
- `x`: i32
- `y`: i32

**Returns:** i32

**Properties:**
- Pure function (no side effects)
- Always terminates
- Panic-free

**Example:**

```rust
let result = add(42, 42);
```

---

### `multiply`

```rust
fn multiply(x: i32, y: i32) -> i32
```

Multiply two numbers.
    
    Args:
        x: First number
        y: Second number
        
    Returns:
        Product of x and y
    

**Parameters:**
- `x`: i32
- `y`: i32

**Returns:** i32

**Properties:**
- Pure function (no side effects)
- Always terminates
- Panic-free

**Example:**

```rust
let result = multiply(42, 42);
```

---


## Classes

### `Calculator`

A simple calculator class.

**Fields:**
- `result`: i32

**Methods:**

#### `__init__`
```rust
fn __init__(&self)
```
Initialize the calculator.

#### `compute_sum`
```rust
fn compute_sum(&self, x: i32, y: i32) -> i32
```
Compute the sum of two numbers.
        
        Args:
            x: First number
            y: Second number
            
        Returns:
            Sum of x and y
        

---


## Migration Notes

### Python to Rust Migration

When migrating from Python to the generated Rust code, note:

1. **Type Safety**: All types are now statically checked at compile time
2. **Memory Management**: Rust's ownership system ensures memory safety
3. **Error Handling**: Python exceptions are converted to Rust `Result` types
4. **Performance**: Expect significant performance improvements

