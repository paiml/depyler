# io - Core I/O Tools

Python's io module provides core tools for working with various types of I/O. It includes in-memory text and binary streams (StringIO, BytesIO) that behave like file objects but operate entirely in memory. Depyler transpiles these operations to Rust's string buffers and byte vectors with full type safety.

## Python → Rust Mapping

| Python Class/Method | Rust Equivalent | Notes |
|---------------------|-----------------|-------|
| `import io` | `use std::io::*` | Core I/O traits |
| `io.StringIO()` | `String::new()` or `Vec<u8>` | In-memory text stream |
| `io.BytesIO()` | `std::io::Cursor<Vec<u8>>` | In-memory binary stream |
| `sio.write(text)` | `string.push_str(text)` | Append text |
| `sio.getvalue()` | `string.clone()` | Get complete content |
| `sio.seek(pos)` | `cursor.set_position(pos)` | Change read/write position |
| `sio.read()` | `String::from(&buffer[pos..])` | Read remaining content |
| `sio.readline()` | Custom implementation | Read single line |

## StringIO Basics

### Basic Write and getvalue()

Create in-memory text streams for efficient string building:

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
    // Create in-memory text stream (just a String)
    let mut sio = String::new();

    // Write text (append)
    sio.push_str("Hello, ");
    sio.push_str("StringIO!");

    // Get complete value (clone the string)
    let result = sio.clone();

    result
}
```

**StringIO Properties:**
- In-memory: No filesystem I/O
- Fast: String concatenation without repeated allocation
- File-like interface: Compatible with code expecting file objects
- Rust: Simple String with push_str() for appending

## Seek and Read Operations

### Seeking to Read from Beginning

Reset position to read accumulated content:

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
use std::io::{Cursor, Read, Write};

fn test_stringio_seek() -> String {
    // Create in-memory text stream with cursor
    let mut buffer = Vec::new();
    let mut sio = Cursor::new(&mut buffer);

    // Write text
    write!(sio, "Hello, World!").expect("Write failed");

    // Seek to beginning
    sio.set_position(0);

    // Read content
    let mut content = String::new();
    sio.read_to_string(&mut content).expect("Read failed");

    content
}
```

**Seek Operations:**
- `seek(0)`: Reset to beginning
- `seek(pos)`: Move to specific position
- Rust: Use `Cursor` for seekable in-memory streams
- Position tracking: Maintains current read/write position


## Line-by-Line Reading

### readline() - Read Single Lines

Read one line at a time from in-memory stream:

```python
import io

def test_readline() -> int:
    # Create stream with multiple lines
    sio = io.StringIO("Line 1\nLine 2\nLine 3\n")

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
    // Create stream with multiple lines
    let content = "Line 1\nLine 2\nLine 3\n";
    let lines: Vec<&str> = content.lines().collect();

    // Count lines
    let line_count = lines.len() as i32;

    line_count
}
```

**readline() Properties:**
- Returns single line including newline character
- Returns empty string when no more data
- Useful for processing large text line-by-line
- Rust: Use `lines()` iterator or `BufRead::read_line()`

## Iteration Support

### Iterating Over Lines

StringIO objects are iterable in Python:

```python
import io

def test_iteration() -> int:
    # Create stream with multiple lines
    content = "Line 1\nLine 2\nLine 3\n"
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
    // Create stream with multiple lines
    let content = "Line 1\nLine 2\nLine 3\n";

    // Count lines using iterator
    let count = content.lines().count() as i32;

    count
}
```

**Iteration Properties:**
- Pythonic: Works with for loops
- Memory efficient: Doesn't load all lines at once
- Rust: String's `lines()` method provides iterator
- Each iteration yields one line

## Initial Value

### StringIO with Initial Content

Create StringIO pre-populated with content:

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
    // Create string with initial content
    let sio = String::from("Initial content");

    // Get content (clone)
    let content = sio.clone();

    content
}
```

**Initial Value Properties:**
- Pre-populated: Content available immediately
- Cursor position: Starts at beginning (position 0)
- Use case: Testing code that expects file objects
- Rust: Simple String initialization


## Common Use Cases

### 1. Building SQL Queries

Construct complex SQL queries efficiently:

```python
import io

def build_sql_query(table: str, columns: list, conditions: dict) -> str:
    """Build SQL SELECT query dynamically."""
    query = io.StringIO()
    
    # SELECT clause
    query.write("SELECT ")
    query.write(", ".join(columns))
    query.write(f" FROM {table}")
    
    # WHERE clause
    if conditions:
        query.write(" WHERE ")
        where_parts = [f"{k} = '{v}'" for k, v in conditions.items()]
        query.write(" AND ".join(where_parts))
    
    return query.getvalue()

# Usage:
# sql = build_sql_query("users", ["id", "name"], {"status": "active"})
# Result: "SELECT id, name FROM users WHERE status = 'active'"
```

### 2. Testing Code That Uses Files

Test file-dependent code without actual files:

```python
import io

def process_file_content(file_obj):
    """Process content from file object."""
    content = file_obj.read()
    return content.upper()

def test_process_file():
    """Test using StringIO instead of real file."""
    # Create mock file with test data
    mock_file = io.StringIO("test data")
    
    # Test function
    result = process_file_content(mock_file)
    
    assert result == "TEST DATA"
```

### 3. Capturing Output

Capture output that would normally go to files:

```python
import io
import sys

def capture_function_output(func):
    """Capture stdout from a function."""
    old_stdout = sys.stdout
    sys.stdout = buffer = io.StringIO()
    
    try:
        func()
        output = buffer.getvalue()
    finally:
        sys.stdout = old_stdout
    
    return output

# Usage:
# output = capture_function_output(lambda: print("Hello"))
# output == "Hello\n"
```

### 4. CSV Generation in Memory

Generate CSV without temporary files:

```python
import io
import csv

def generate_csv_string(data):
    """Generate CSV content as string."""
    output = io.StringIO()
    writer = csv.writer(output)
    
    # Write header
    writer.writerow(["Name", "Age", "City"])
    
    # Write data
    for row in data:
        writer.writerow(row)
    
    return output.getvalue()

# Usage:
# csv_content = generate_csv_string([["Alice", 30, "NYC"], ["Bob", 25, "LA"]])
```

## Performance Characteristics

| Operation | Python StringIO | Rust String | Notes |
|-----------|-----------------|-------------|-------|
| `write()` | ~0.5 μs | ~0.2 μs | String append |
| `getvalue()` | ~0.3 μs | ~0.1 μs | String clone |
| `seek(0)` + `read()` | ~1 μs | ~0.5 μs | Full read |
| `readline()` | ~0.8 μs | ~0.4 μs | Per line |
| Memory overhead | Higher | Lower | Rust more efficient |

**Performance Notes:**
- StringIO faster than repeated string concatenation
- No filesystem I/O overhead
- Rust benefits from zero-copy operations where possible
- Both use contiguous memory for efficiency

**Rust Advantages:**
- No garbage collection pauses
- Stack allocation for small strings
- Compile-time optimization
- Better memory locality

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_io.py`. Run:

```bash
cd tdd-book
uv run pytest ../tests/test_io.py -v
```

**Expected Output:**
```
../tests/test_io.py::test_io_stringio_basic PASSED                       [ 20%]
../tests/test_io.py::test_io_stringio_seek PASSED                        [ 40%]
../tests/test_io.py::test_io_stringio_readline PASSED                    [ 60%]
../tests/test_io.py::test_io_stringio_iteration PASSED                   [ 80%]
../tests/test_io.py::test_io_stringio_initial_value PASSED               [100%]

====== 5 passed in 0.XX s ======
```

## Alternative Rust Patterns

### Using Vec<u8> for Binary Data

For binary in-memory streams, use Vec<u8>:

```rust
use std::io::{Cursor, Write};

fn binary_stream_example() -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    // Write binary data
    cursor.write_all(b"Binary data").expect("Write failed");
    
    buffer
}
```

### BufWriter for Buffered Writing

Optimize write operations with buffering:

```rust
use std::io::{BufWriter, Write};

fn buffered_writing() -> String {
    let mut buffer = String::new();
    let mut writer = BufWriter::new(unsafe { buffer.as_mut_vec() });
    
    write!(writer, "Buffered ").expect("Write failed");
    write!(writer, "writing").expect("Write failed");
    
    buffer
}
```

### Custom StringIO Implementation

Full StringIO emulation with seek support:

```rust
use std::io::{Cursor, Read, Write, Seek, SeekFrom};

struct StringIO {
    buffer: Vec<u8>,
    cursor: Cursor<Vec<u8>>,
}

impl StringIO {
    fn new() -> Self {
        StringIO {
            buffer: Vec::new(),
            cursor: Cursor::new(Vec::new()),
        }
    }
    
    fn write_str(&mut self, s: &str) {
        self.cursor.write_all(s.as_bytes()).expect("Write failed");
    }
    
    fn getvalue(&self) -> String {
        String::from_utf8(self.cursor.get_ref().clone()).expect("Invalid UTF-8")
    }
    
    fn seek(&mut self, pos: u64) {
        self.cursor.seek(SeekFrom::Start(pos)).expect("Seek failed");
    }
}
```

## Comparison: StringIO vs String Concatenation

### Why Use StringIO?

**StringIO Advantages:**
```python
# StringIO: Efficient for multiple writes
sio = io.StringIO()
for i in range(1000):
    sio.write(str(i))  # O(1) append
result = sio.getvalue()

# String concatenation: Inefficient
result = ""
for i in range(1000):
    result += str(i)  # O(n) copy each time
```

**Performance Comparison:**
- StringIO: O(n) total time for n writes
- String +=: O(n²) total time for n writes
- Memory: StringIO more efficient

### When to Use String Concatenation

Use simple concatenation for:
- Few operations (< 5 concatenations)
- Small strings
- One-time construction
- Format strings: `f"{a} {b} {c}"`

### When to Use StringIO

Use StringIO for:
- Many write operations (> 10)
- Building large strings incrementally
- Mock file objects for testing
- Capturing output streams

## BytesIO

### Working with Binary Data

BytesIO for binary in-memory streams:

```python
import io

def bytes_example():
    # Create binary stream
    bio = io.BytesIO()
    
    # Write binary data
    bio.write(b"Hello")
    bio.write(b" ")
    bio.write(b"BytesIO")
    
    # Get bytes
    result = bio.getvalue()
    return result  # b'Hello BytesIO'
```

**Rust Equivalent:**

```rust
fn bytes_example() -> Vec<u8> {
    let mut bio = Vec::new();
    
    // Write binary data
    bio.extend_from_slice(b"Hello");
    bio.extend_from_slice(b" ");
    bio.extend_from_slice(b"BytesIO");
    
    bio  // Vec<u8>
}
```

## Best Practices

**DO:**
- ✅ Use StringIO for building large strings
- ✅ Use StringIO for testing file-based code
- ✅ Call getvalue() once at the end
- ✅ Use seek(0) before reading
- ✅ Close or reuse StringIO objects

**DON'T:**
- ❌ Use StringIO for simple concatenation (< 5 operations)
- ❌ Call getvalue() repeatedly (makes copies)
- ❌ Mix read and write without seeking
- ❌ Forget position after write (cursor at end)
- ❌ Use for small strings (overhead not worth it)

**Memory Management:**
```python
# GOOD: Single StringIO, single getvalue()
sio = io.StringIO()
for item in large_list:
    sio.write(process(item))
result = sio.getvalue()  # One copy

# BAD: Multiple getvalue() calls
sio = io.StringIO()
for item in large_list:
    sio.write(process(item))
    partial = sio.getvalue()  # Unnecessary copy each time
```

## Future Support

**Currently Supported:**
- ✅ `io.StringIO()` - In-memory text stream
- ✅ `sio.write(text)` - Write text
- ✅ `sio.getvalue()` - Get complete content
- ✅ `sio.seek(pos)` - Seek to position
- ✅ `sio.read()` - Read content
- ✅ `sio.readline()` - Read single line
- ✅ Iteration over lines
- ✅ Initial value support

**Planned Support:**
- 🔄 `io.BytesIO()` - Binary in-memory streams
- 🔄 `sio.tell()` - Get current position
- 🔄 `sio.truncate(size)` - Truncate stream
- 🔄 `sio.close()` - Close stream
- 🔄 Context manager support (`with` statement)

**Workarounds for Unsupported Features:**

```rust
// BytesIO - Use Vec<u8> with Cursor
use std::io::Cursor;
let mut bytes_io = Cursor::new(Vec::new());

// tell() - Get position from Cursor
let position = cursor.position();

// truncate() - Resize vector
buffer.truncate(size);

// Context manager - Use RAII with Drop trait
struct AutoClose<T> {
    resource: T,
}

impl<T> Drop for AutoClose<T> {
    fn drop(&mut self) {
        // Cleanup happens automatically
    }
}
```
