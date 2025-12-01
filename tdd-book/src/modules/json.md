# json - JSON Encoder and Decoder

Python's json module provides functions for encoding and decoding JSON (JavaScript Object Notation) data. Depyler transpiles these operations to Rust's `serde_json` crate with full type safety and efficient serialization.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import json` | `use serde_json::*` | JSON serialization |
| `json.loads(str)` | `serde_json::from_str(str)` | Parse JSON string |
| `json.dumps(obj)` | `serde_json::to_string(obj)` | Serialize to JSON |
| `json.load(file)` | `serde_json::from_reader(file)` | Parse from file |
| `json.dump(obj, file)` | `serde_json::to_writer(file, obj)` | Serialize to file |
| `dumps(obj, indent=2)` | `to_string_pretty(obj)` | Pretty-print JSON |

## json.loads() - Deserialize JSON

### Basic Types

Parse JSON strings to Python objects:

```python
import json

def test_loads() -> str:
    # Parse basic JSON types
    null_val = json.loads("null")
    bool_val = json.loads("true")
    int_val = json.loads("42")
    str_val = json.loads('"hello"')

    # Return concatenated result
    result = str(null_val) + "," + str(bool_val) + "," + str(int_val) + "," + str_val

    return result
```

**Generated Rust:**

```rust
use serde_json::Value;

fn test_loads() -> String {
    // Parse basic JSON types
    let null_val: Value = serde_json::from_str("null").unwrap();
    let bool_val: Value = serde_json::from_str("true").unwrap();
    let int_val: Value = serde_json::from_str("42").unwrap();
    let str_val: Value = serde_json::from_str("\"hello\"").unwrap();

    // Return concatenated result
    let result = format!("{},{},{},{}",
        null_val,
        bool_val,
        int_val.as_i64().unwrap(),
        str_val.as_str().unwrap()
    );

    result
}
```

**JSON Type Mapping:**

| JSON Type | Python Type | Rust Type (`Value` enum) |
|-----------|-------------|--------------------------|
| `null` | `None` | `Value::Null` |
| `true`/`false` | `bool` | `Value::Bool(bool)` |
| `123` | `int` | `Value::Number` (i64, u64, f64) |
| `"text"` | `str` | `Value::String(String)` |
| `[...]` | `list` | `Value::Array(Vec<Value>)` |
| `{...}` | `dict` | `Value::Object(Map)` |

### Parsing Dictionaries

```python
import json

def test_loads_dict() -> str:
    # Parse JSON object to dict
    data = json.loads('{"name": "Alice", "age": 30}')

    # Access values
    name = data["name"]
    age = data["age"]

    return name + "," + str(age)
```

**Generated Rust:**

```rust
use serde_json::Value;

fn test_loads_dict() -> String {
    // Parse JSON object to dict
    let data: Value = serde_json::from_str(r#"{"name": "Alice", "age": 30}"#)
        .unwrap();

    // Access values
    let name = data["name"].as_str().unwrap();
    let age = data["age"].as_i64().unwrap();

    format!("{},{}", name, age)
}
```

### Parsing Lists

```python
import json

def test_loads_list() -> int:
    # Parse JSON array to list
    data = json.loads('[1, 2, 3, 4, 5]')

    # Calculate sum
    total = 0
    for num in data:
        total += num

    return total
```

**Generated Rust:**

```rust
use serde_json::Value;

fn test_loads_list() -> i32 {
    // Parse JSON array to list
    let data: Value = serde_json::from_str("[1, 2, 3, 4, 5]")
        .unwrap();

    // Calculate sum
    let mut total = 0;
    if let Some(array) = data.as_array() {
        for num in array {
            if let Some(n) = num.as_i64() {
                total += n as i32;
            }
        }
    }

    total
}
```

## json.dumps() - Serialize to JSON

### Basic Serialization

```python
import json

def test_dumps() -> str:
    # Serialize Python objects to JSON
    data = {"name": "Bob", "age": 25}
    json_str = json.dumps(data)

    return json_str
```

**Generated Rust:**

```rust
use serde_json::json;

fn test_dumps() -> String {
    // Serialize Rust data to JSON
    let data = json!({
        "name": "Bob",
        "age": 25
    });

    let json_str = serde_json::to_string(&data).unwrap();

    json_str
}
```

## JSON Round-Trip

### Serialize and Deserialize

```python
import json

def test_roundtrip() -> bool:
    # Original data
    original = {"users": ["Alice", "Bob"], "count": 2}

    # Serialize and deserialize
    json_str = json.dumps(original)
    restored = json.loads(json_str)

    # Check if equal
    return restored["count"] == original["count"]
```

**Generated Rust:**

```rust
use serde_json::{json, Value};

fn test_roundtrip() -> bool {
    // Original data
    let original = json!({
        "users": ["Alice", "Bob"],
        "count": 2
    });

    // Serialize and deserialize
    let json_str = serde_json::to_string(&original).unwrap();
    let restored: Value = serde_json::from_str(&json_str).unwrap();

    // Check if equal
    restored["count"].as_i64().unwrap() == original["count"].as_i64().unwrap()
}
```

## Nested Structures

### Complex JSON Parsing

```python
import json

def test_nested() -> int:
    # Parse nested JSON structure
    json_str = '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}'
    data = json.loads(json_str)

    # Access nested data
    users = data["users"]
    first_user_age = users[0]["age"]

    return first_user_age
```

**Generated Rust:**

```rust
use serde_json::Value;

fn test_nested() -> i32 {
    // Parse nested JSON structure
    let json_str = r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#;
    let data: Value = serde_json::from_str(json_str).unwrap();

    // Access nested data
    let users = data["users"].as_array().unwrap();
    let first_user_age = users[0]["age"].as_i64().unwrap() as i32;

    first_user_age
}
```

## Common Use Cases

### 1. API Response Parsing

```python
import json

def parse_api_response(response_text: str) -> dict:
    data = json.loads(response_text)
    return {
        'status': data['status'],
        'count': len(data['results']),
        'first_id': data['results'][0]['id'] if data['results'] else None
    }
```

### 2. Configuration Files

```python
import json

def load_config(config_str: str) -> dict:
    config = json.loads(config_str)
    return {
        'host': config.get('database', {}).get('host', 'localhost'),
        'port': config.get('database', {}).get('port', 5432),
        'debug': config.get('debug', False)
    }
```

### 3. Data Serialization

```python
import json

def serialize_user(user_id: int, name: str, email: str) -> str:
    user_data = {
        'id': user_id,
        'name': name,
        'email': email,
        'created_at': '2024-01-01T00:00:00Z'
    }
    return json.dumps(user_data)
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `loads(str)` | O(n) | O(n) | n = JSON string length |
| `dumps(obj)` | O(n) | O(n) | n = object complexity |
| Small objects | ~1-10 μs | ~0.5-5 μs | Rust typically faster |
| Large objects | ~100-1000 μs | ~50-500 μs | Rust 2x faster |
| Memory usage | Higher | Lower | Zero-copy where possible |

**Performance Notes:**
- `serde_json` uses zero-copy deserialization where possible
- Rust's strong typing enables compile-time optimizations
- Python's dynamic typing requires runtime type checking
- Both use efficient JSON parsers (state machines)

## Safety and Guarantees

**Type Safety:**
- Python: Dynamic types, runtime errors possible
- Rust: `Value` enum provides type safety at runtime
- Access methods return `Option` (safe unwrapping required)
- Type mismatches caught at deserialization time

**Error Handling:**
- Python: Raises `JSONDecodeError` for invalid JSON
- Rust: Returns `Result<T, Error>` for all operations
- Both provide detailed error messages with line/column info
- Rust's error handling is explicit (no uncaught exceptions)

**Important Notes:**
- JSON has no integer size limits (Python `int`, Rust uses i64/u64)
- Floating point precision may vary between implementations
- Unicode strings handled correctly in both languages
- Rust's `serde` provides compile-time serialization guarantees

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_json.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_json.py -v
```

**Expected Output:**
```
tests/test_json.py::test_json_loads_basic PASSED         [ 16%]
tests/test_json.py::test_json_loads_dict PASSED          [ 33%]
tests/test_json.py::test_json_loads_list PASSED          [ 50%]
tests/test_json.py::test_json_dumps_basic PASSED         [ 66%]
tests/test_json.py::test_json_roundtrip PASSED           [ 83%]
tests/test_json.py::test_json_nested_structures PASSED   [100%]

====== 6 passed in 0.XX s ======
```

## Advanced: Typed Deserialization

For better type safety in Rust, use strongly-typed structs:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

fn typed_json_example() -> Result<User, serde_json::Error> {
    let json_str = r#"{"name": "Alice", "age": 30, "email": "alice@example.com"}"#;

    // Deserialize directly to User struct
    let user: User = serde_json::from_str(json_str)?;

    Ok(user)
}
```

**Benefits of Typed Deserialization:**
- Compile-time type checking
- No runtime type errors
- Better IDE autocomplete
- Automatic validation
- Zero-cost abstractions

## JSON Standards and Compatibility

**JSON Specification (RFC 8259):**
- UTF-8 encoding required
- String escaping: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`
- Numbers: No leading zeros, scientific notation supported
- Objects: Key-value pairs with string keys
- Arrays: Ordered sequences

**Python json module:**
- Follows RFC 8259
- Additional options: `indent`, `sort_keys`, `ensure_ascii`
- Supports custom encoders/decoders

**Rust serde_json:**
- Strict RFC 8259 compliance
- Options: `to_string_pretty()` for formatting
- Supports custom serializers via `serde` traits

## Performance Tips

**Optimization strategies:**
- Use typed deserialization in Rust when structure is known
- Avoid repeated parsing (cache parsed JSON)
- Stream large JSON files instead of loading fully
- Use `&str` references where possible (avoid clones)
- Pre-allocate buffers for serialization

**Example: Streaming JSON Arrays**
```rust
use serde_json::Deserializer;

fn stream_json_array(json_str: &str) {
    let stream = Deserializer::from_str(json_str)
        .into_iter::<serde_json::Value>();

    for value in stream {
        match value {
            Ok(v) => println!("{}", v),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
```

## Comparison: json vs Other Formats

| Feature | JSON | XML | YAML | MessagePack |
|---------|------|-----|------|-------------|
| Human-readable | ✅ Yes | ✅ Yes | ✅ Yes | ❌ Binary |
| Size | Medium | Large | Medium | Small |
| Parse speed | Fast | Slow | Medium | Very fast |
| Data types | Limited | Limited | Rich | Rich |
| Comments | ❌ No | ✅ Yes | ✅ Yes | ❌ No |
| Ubiquity | Very high | High | Medium | Low |

**When to use JSON:**
- Web APIs (de facto standard)
- Configuration files (simple structure)
- Data interchange between systems
- Logging and event data

**When not to use JSON:**
- Binary data (use MessagePack or Protocol Buffers)
- Comments needed (use YAML or TOML)
- Very large datasets (use streaming formats)
- Complex schemas (use XML with XSD)

