# io - Core I/O Tools

Python's io module provides core tools for working with streams, including in-memory text and binary streams. Depyler transpiles these operations to Rust's standard I/O types with full type safety and efficient memory handling.

## Python → Rust Mapping

| Python Class/Method | Rust Equivalent | Notes |
|---------------------|-----------------|-------|
| `import io` | `use std::io::*` | Core I/O traits |
| `io.StringIO()` | `String` or `Cursor<String>` | In-memory text buffer |
| `sio.write(text)` | `string.push_str(text)` | Append to string |
| `sio.read()` | `string.clone()` | Read entire content |
| `sio.getvalue()` | `string.as_str()` | Get current value |
| `sio.seek(pos)` | Cursor position | Position tracking |
| `sio.readline()` | `lines().next()` | Line-by-line reading |
| `for line in sio` | `lines()` iterator | Iterate over lines |

## StringIO - In-Memory Text Streams

### Basic Write and Read Operations

Create and manipulate in-memory text streams:

```python
import io

def test_stringio() -> str:
    # Create in-memory text stream
    sio = io.StringIO()

    # Write text
    sio.write("Hello, ")
    sio.write("StringIO!")

    # Get complete value
    result = sio.getvalue()

    return result
```

**Generated Rust:**

```rust
fn test_stringio() -> String {
    // Create in-memory text stream (String)
    let mut sio = String::new();

    // Write text
    sio.push_str("Hello, ");
    sio.push_str("StringIO!");

    // Get complete value
    let result = sio.clone();

    result
}
```

**Key Differences:**
- Python: `StringIO` is a separate object with methods
- Rust: Uses `String` directly with `push_str()` for appending
- Both support efficient in-memory text manipulation
- Rust's `String` is UTF-8 validated

### Seek and Read Operations

Read from specific positions in the stream:

```python
import io

def test_stringio_seek() -> str:
    # Create in-memory text stream
    sio = io.StringIO()

    # Write text
    sio.write("Hello, World!")

    # Seek to beginning
    sio.seek(0)

    # Read content
    content = sio.read()

    return content
```

**Generated Rust:**

```rust
fn test_stringio_seek() -> String {
    // Create in-memory text stream
    let mut sio = String::new();

    // Write text
    sio.push_str("Hello, World!");

    // Read content (no seek needed for String)
    let content = sio.clone();

    content
}
```

**Seek Behavior:**
- Python: Explicit `seek(0)` required to read after writing
- Rust: String access doesn't require seek (always reads full content)
- For cursor-based operations, use `std::io::Cursor<String>`

### StringIO with Initial Value

```python
import io

def test_initial_value() -> str:
    # Create StringIO with initial content
    sio = io.StringIO("Initial content")

    # Read from beginning
    content = sio.read()

    return content
```

**Generated Rust:**

```rust
fn test_initial_value() -> String {
    // Create String with initial content
    let sio = String::from("Initial content");

    // Read content
    let content = sio.clone();

    content
}
```

## Line-Based Operations

### readline() - Read Single Lines

```python
import io

def test_readline() -> int:
    # Create stream with multiple lines
    sio = io.StringIO("Line 1\\nLine 2\\nLine 3\\n")

    # Read lines
    line_count = 0
    while True:
        line = sio.readline()
        if not line:
            break
        line_count += 1

    return line_count
```

**Generated Rust:**

```rust
fn test_readline() -> i32 {
    // Create string with multiple lines
    let content = String::from("Line 1\nLine 2\nLine 3\n");

    // Count lines by splitting
    let line_count = content.lines().count() as i32;

    line_count
}
```

### Iteration Over Lines

```python
import io

def test_iteration() -> int:
    # Create stream with multiple lines
    content = "Line 1\\nLine 2\\nLine 3\\n"
    sio = io.StringIO(content)

    # Count lines using iteration
    count = 0
    for line in sio:
        count += 1

    return count
```

**Generated Rust:**

```rust
fn test_iteration() -> i32 {
    // Create string with multiple lines
    let content = String::from("Line 1\nLine 2\nLine 3\n");

    // Count lines using iterator
    let count = content.lines().count() as i32;

    count
}
```

## Common Use Cases

### 1. Build String Incrementally

```python
import io

def build_report(items: list) -> str:
    output = io.StringIO()

    output.write("Report Header\n")
    output.write("=" * 40 + "\n")

    for item in items:
        output.write(f"Item: {item}\n")

    output.write("\nReport Footer\n")

    return output.getvalue()
```

### 2. Parse Multi-Line Text

```python
import io

def parse_config(text: str) -> dict:
    config = {}
    stream = io.StringIO(text)

    for line in stream:
        line = line.strip()
        if '=' in line:
            key, value = line.split('=', 1)
            config[key.strip()] = value.strip()

    return config
```

### 3. CSV-Like Processing

```python
import io

def process_csv_like(data: str) -> int:
    stream = io.StringIO(data)
    count = 0

    for line in stream:
        fields = line.strip().split(',')
        if len(fields) >= 3:
            count += 1

    return count
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| `StringIO()` | O(1) | O(1) | Create empty buffer |
| `write(text)` | O(n) | O(n) | n = text length |
| `getvalue()` | O(1) | O(n) | Python caches, Rust clones |
| `read()` | O(n) | O(n) | n = content size |
| `seek(pos)` | O(1) | O(1) | Position update |
| `readline()` | O(n) | O(n) | n = line length |
| Iteration | O(n) | O(n) | n = content size |

**Performance Notes:**
- Rust's `String` has amortized O(1) append via capacity doubling
- Python's `StringIO` uses similar strategy internally
- Rust avoids position tracking overhead for simple cases
- Both efficiently handle incremental string building

## Safety and Guarantees

**Type Safety:**
- Python: `StringIO` handles only text (str), `BytesIO` for bytes
- Rust: `String` is UTF-8 validated, `Vec<u8>` for arbitrary bytes
- Type system prevents mixing text and binary data
- No runtime encoding errors in Rust

**Memory Safety:**
- Both use dynamic arrays with capacity management
- Rust prevents buffer overruns at compile time
- Python raises exceptions for invalid operations
- Rust's borrow checker ensures no concurrent mutation

**Important Notes:**
- StringIO position is mutable state (use carefully)
- Reading doesn't consume content (unlike file reads)
- Both support unlimited growth (memory permitting)
- Rust cloning is explicit, Python copying requires explicit call

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_io.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_io.py -v
```

**Expected Output:**
```
tests/test_io.py::test_io_stringio_basic PASSED          [ 20%]
tests/test_io.py::test_io_stringio_seek PASSED           [ 40%]
tests/test_io.py::test_io_stringio_readline PASSED       [ 60%]
tests/test_io.py::test_io_stringio_iteration PASSED      [ 80%]
tests/test_io.py::test_io_stringio_initial_value PASSED  [100%]

====== 5 passed in 0.XX s ======
```

## Comparison: StringIO vs String Concatenation

| Feature | `StringIO` | String `+` operator |
|---------|-----------|---------------------|
| Performance | O(n) total | O(n²) for n appends |
| Memory | Single buffer | Multiple allocations |
| Use case | Many appends | Few concatenations |
| Readability | Method calls | Operator syntax |
| Python idiom | Large builders | Small strings |

**Recommendation:** Use StringIO for building strings with many operations, use `+` for simple cases.

## Advanced: Cursor-Based I/O

For more complex I/O operations in Rust, use `Cursor`:

```rust
use std::io::{Cursor, Read, Write, Seek, SeekFrom};

fn cursor_example() -> String {
    let mut cursor = Cursor::new(Vec::new());

    // Write data
    cursor.write_all(b"Hello, ").unwrap();
    cursor.write_all(b"World!").unwrap();

    // Seek to beginning
    cursor.seek(SeekFrom::Start(0)).unwrap();

    // Read data
    let mut buffer = String::new();
    cursor.read_to_string(&mut buffer).unwrap();

    buffer
}
```

**Cursor Features:**
- Implements `Read`, `Write`, `Seek` traits
- Provides file-like API for in-memory buffers
- Tracks position automatically
- Supports both `Vec<u8>` and `&[u8]`

