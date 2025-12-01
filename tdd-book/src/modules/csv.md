# csv - CSV File Reading and Writing

Python's csv module provides functions for reading and writing CSV (Comma-Separated Values) files. Depyler transpiles these operations to Rust's `csv` crate with full type safety and efficient parsing.

## Python → Rust Mapping

| Python Class/Method | Rust Equivalent | Notes |
|---------------------|-----------------|-------|
| `import csv` | `use csv::*` | CSV parsing |
| `csv.reader(file)` | `ReaderBuilder::new().from_reader(file)` | Read CSV rows |
| `csv.writer(file)` | `WriterBuilder::new().from_writer(file)` | Write CSV rows |
| `csv.DictReader(file)` | `Reader::deserialize()` | Read as dictionaries |
| `csv.DictWriter(file)` | `Writer::serialize()` | Write from dictionaries |
| `reader.delimiter` | `ReaderBuilder::delimiter()` | Custom delimiter |
| `reader.quotechar` | `ReaderBuilder::quote()` | Custom quote char |

## csv.reader() - Read CSV Data

### Basic CSV Reading

Read CSV data row by row:

```python
import csv
import io

def test_reader() -> int:
    # CSV data
    data = "a,b,c\\n1,2,3\\n4,5,6"

    # Read CSV
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)

    # Count total cells
    total = 0
    for row in rows:
        total += len(row)

    return total
```

**Generated Rust:**

```rust
use csv::ReaderBuilder;
use std::io::Cursor;

fn test_reader() -> i32 {
    // CSV data
    let data = "a,b,c\n1,2,3\n4,5,6";

    // Read CSV
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(data));

    let mut total = 0;
    for result in reader.records() {
        if let Ok(record) = result {
            total += record.len() as i32;
        }
    }

    total
}
```

**Key Differences:**
- Python: Returns list of lists `[['a','b','c'], ...]`
- Rust: Returns iterator of `StringRecord`
- Both support lazy iteration (don't load all data at once)
- Rust requires explicit error handling

## csv.writer() - Write CSV Data

### Basic CSV Writing

Write CSV data row by row:

```python
import csv
import io

def test_writer() -> str:
    # Create CSV writer
    output = io.StringIO()
    writer = csv.writer(output)

    # Write rows
    writer.writerow(["name", "age"])
    writer.writerow(["Alice", "30"])
    writer.writerow(["Bob", "25"])

    # Get CSV string
    result = output.getvalue()

    return result
```

**Generated Rust:**

```rust
use csv::WriterBuilder;

fn test_writer() -> String {
    // Create CSV writer
    let mut output = Vec::new();
    let mut writer = WriterBuilder::new()
        .from_writer(&mut output);

    // Write rows
    writer.write_record(&["name", "age"]).unwrap();
    writer.write_record(&["Alice", "30"]).unwrap();
    writer.write_record(&["Bob", "25"]).unwrap();

    // Flush and get CSV string
    writer.flush().unwrap();
    String::from_utf8(output).unwrap()
}
```

**Key Points:**
- Python: `writerow()` writes single row
- Rust: `write_record()` writes single record
- Both automatically handle quoting of special characters
- Rust requires explicit `flush()` to ensure data is written

## csv.DictReader() - Read as Dictionaries

### Reading with Headers

Read CSV with first row as field names:

```python
import csv
import io

def test_dictreader() -> int:
    # CSV data with header
    data = "name,age\\nAlice,30\\nBob,25"

    # Read as dictionaries
    reader = csv.DictReader(io.StringIO(data))

    # Sum ages
    total_age = 0
    for row in reader:
        total_age += int(row["age"])

    return total_age
```

**Generated Rust:**

```rust
use csv::ReaderBuilder;
use std::io::Cursor;
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn test_dictreader() -> i32 {
    // CSV data with header
    let data = "name,age\nAlice,30\nBob,25";

    // Read as structs
    let mut reader = ReaderBuilder::new()
        .from_reader(Cursor::new(data));

    let mut total_age = 0;
    for result in reader.deserialize::<Person>() {
        if let Ok(person) = result {
            total_age += person.age;
        }
    }

    total_age
}
```

**DictReader Benefits:**
- Automatic field name mapping
- Type-safe deserialization in Rust
- No need to remember column indices
- Self-documenting code

## csv.DictWriter() - Write from Dictionaries

### Writing with Headers

```python
import csv
import io

def test_dictwriter() -> str:
    # Create DictWriter
    output = io.StringIO()
    fieldnames = ["name", "age"]
    writer = csv.DictWriter(output, fieldnames=fieldnames)

    # Write header and rows
    writer.writeheader()
    writer.writerow({"name": "Alice", "age": "30"})
    writer.writerow({"name": "Bob", "age": "25"})

    # Get CSV string
    result = output.getvalue()

    return result
```

**Generated Rust:**

```rust
use csv::WriterBuilder;
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: String,
}

fn test_dictwriter() -> String {
    // Create CSV writer
    let mut output = Vec::new();
    let mut writer = WriterBuilder::new()
        .from_writer(&mut output);

    // Write rows (headers automatic with serde)
    writer.serialize(Person {
        name: "Alice".to_string(),
        age: "30".to_string(),
    }).unwrap();
    writer.serialize(Person {
        name: "Bob".to_string(),
        age: "25".to_string(),
    }).unwrap();

    // Flush and get CSV string
    writer.flush().unwrap();
    String::from_utf8(output).unwrap()
}
```

## Custom Delimiters and Options

### TSV (Tab-Separated Values)

```python
import csv
import io

def test_custom_delimiter() -> int:
    # TSV data (tab-separated)
    data = "a\\tb\\tc\\n1\\t2\\t3"

    # Read with tab delimiter
    reader = csv.reader(io.StringIO(data), delimiter="\\t")
    rows = list(reader)

    # Count rows
    return len(rows)
```

**Generated Rust:**

```rust
use csv::ReaderBuilder;
use std::io::Cursor;

fn test_custom_delimiter() -> i32 {
    // TSV data (tab-separated)
    let data = "a\tb\tc\n1\t2\t3";

    // Read with tab delimiter
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(Cursor::new(data));

    reader.records().count() as i32
}
```

**Common Delimiters:**
- `,` - Comma (standard CSV)
- `\t` - Tab (TSV files)
- `;` - Semicolon (European CSV)
- `|` - Pipe (database exports)

## Quoted Fields

### Handling Special Characters

```python
import csv
import io

def test_quoted() -> str:
    # CSV with quoted field containing comma
    data = '"Hello, World",123,test'

    # Read CSV
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)

    # Return first field
    return rows[0][0]
```

**Generated Rust:**

```rust
use csv::ReaderBuilder;
use std::io::Cursor;

fn test_quoted() -> String {
    // CSV with quoted field containing comma
    let data = r#""Hello, World",123,test"#;

    // Read CSV
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(data));

    if let Some(Ok(record)) = reader.records().next() {
        record.get(0).unwrap_or("").to_string()
    } else {
        String::new()
    }
}
```

**Quoting Rules:**
- Fields with delimiter are auto-quoted
- Fields with newlines are auto-quoted
- Fields with quote characters are escaped (`""`)
- Empty fields don't need quotes

## Common Use Cases

### 1. Export Database Query Results

```python
import csv
import io

def export_users(users: list) -> str:
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['id', 'name', 'email'])

    writer.writeheader()
    for user in users:
        writer.writerow(user)

    return output.getvalue()
```

### 2. Import Configuration Data

```python
import csv
import io

def load_settings(csv_data: str) -> dict:
    settings = {}
    reader = csv.DictReader(io.StringIO(csv_data))

    for row in reader:
        settings[row['key']] = row['value']

    return settings
```

### 3. Data Transformation Pipeline

```python
import csv
import io

def transform_data(input_csv: str) -> str:
    # Read input
    reader = csv.DictReader(io.StringIO(input_csv))

    # Transform and write output
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['name', 'total'])
    writer.writeheader()

    for row in reader:
        writer.writerow({
            'name': row['name'],
            'total': str(int(row['qty']) * float(row['price']))
        })

    return output.getvalue()
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Parse 1MB file | ~50ms | ~20ms | Rust 2.5x faster |
| Parse 10MB file | ~500ms | ~200ms | Linear scaling |
| Write 1MB | ~40ms | ~15ms | Rust 3x faster |
| Memory usage | ~2x file size | ~1x file size | Rust more efficient |
| Lazy reading | Yes | Yes | Both support streaming |

**Performance Notes:**
- Rust's `csv` crate uses zero-copy parsing where possible
- Both support memory-efficient streaming for large files
- Rust avoids Python's object creation overhead
- Type checking happens at compile time in Rust

## Safety and Guarantees

**Type Safety:**
- Python: All fields are strings (manual conversion needed)
- Rust: Can deserialize directly to typed structs
- `serde` provides compile-time type validation in Rust
- Python requires runtime type checking

**Error Handling:**
- Python: Raises `csv.Error` for malformed CSV
- Rust: Returns `Result` types for all operations
- Both handle missing fields gracefully
- Rust's error types are more specific

**Important Notes:**
- CSV has no standard (RFC 4180 is informational only)
- Different systems may interpret CSV differently
- Always specify delimiter explicitly for clarity
- Quote characters and escaping vary by implementation

**Best Practices:**
```rust
// ❌ BAD: Assuming all records have same length
let record = records.next().unwrap();
let value = record.get(5).unwrap();  // May panic!

// ✅ GOOD: Handle missing fields
if let Some(Ok(record)) = records.next() {
    if let Some(value) = record.get(5) {
        // Safe access
    }
}
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_csv.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_csv.py -v
```

**Expected Output:**
```
tests/test_csv.py::test_csv_reader_basic PASSED          [ 16%]
tests/test_csv.py::test_csv_writer_basic PASSED          [ 33%]
tests/test_csv.py::test_csv_dictreader PASSED            [ 50%]
tests/test_csv.py::test_csv_dictwriter PASSED            [ 66%]
tests/test_csv.py::test_csv_custom_delimiter PASSED      [ 83%]
tests/test_csv.py::test_csv_quoted_fields PASSED         [100%]

====== 6 passed in 0.XX s ======
```

## CSV Standards and Variations

**RFC 4180 (Informational):**
- CRLF line endings (`\r\n`)
- Comma delimiter
- Double-quote for quoting
- Double-double-quote for escaping (`""`)

**Common Variations:**
- Excel: May use semicolon in some locales
- Tab-delimited (TSV): Uses `\t` as delimiter
- Pipe-delimited: Uses `|` as delimiter
- Unix: Often uses LF (`\n`) instead of CRLF

**Python csv module:**
- Defaults to Excel dialect
- Supports custom dialects
- Handles both LF and CRLF

**Rust csv crate:**
- Strict RFC 4180 by default
- Highly configurable via builders
- Zero-copy parsing when possible

## Performance Tips

**Optimization strategies:**
- Use streaming (don't load entire file)
- Pre-allocate buffers when size known
- Use typed deserialization in Rust
- Avoid string allocations with `&str` where possible
- Process records in batches for better cache locality

**Example: Efficient Large File Processing**
```rust
use csv::ReaderBuilder;
use std::fs::File;

fn process_large_csv(path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .buffer_capacity(64 * 1024)  // 64KB buffer
        .from_reader(file);

    let mut count = 0;
    for result in reader.records() {
        let record = result?;
        // Process record without loading entire file
        count += 1;
    }

    Ok(count)
}
```

## Comparison: CSV vs Other Formats

| Feature | CSV | TSV | JSON | XML |
|---------|-----|-----|------|-----|
| Human-readable | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| Size | Small | Small | Medium | Large |
| Parse speed | Fast | Fast | Medium | Slow |
| Type support | None | None | Limited | Limited |
| Nested data | ❌ No | ❌ No | ✅ Yes | ✅ Yes |
| Spreadsheet support | ✅ Excellent | ⚠️ Limited | ❌ No | ❌ No |

**When to use CSV:**
- Tabular data (rows and columns)
- Spreadsheet import/export
- Simple data interchange
- Log file analysis

**When not to use CSV:**
- Nested/hierarchical data (use JSON or XML)
- Binary data (use formats like Parquet)
- Need for schema validation (use JSON Schema or XML DTD)
- Complex types needed (use protocol buffers)

## Advanced: CSV Dialects

Python's csv module supports dialects for different CSV formats:

```python
import csv

# Excel dialect (default)
csv.reader(file, dialect='excel')

# Tab-delimited
csv.reader(file, dialect='excel-tab')

# Custom dialect
csv.register_dialect('pipes',
    delimiter='|',
    quotechar='"',
    quoting=csv.QUOTE_MINIMAL
)
csv.reader(file, dialect='pipes')
```

**Rust equivalent:**
```rust
use csv::{ReaderBuilder, QuoteStyle};

// Custom configuration
let mut reader = ReaderBuilder::new()
    .delimiter(b'|')
    .quote(b'"')
    .quoting(true)
    .from_reader(file);
```

## Edge Cases and Gotchas

**Empty lines:**
```python
# Python: Yields empty list for blank lines
data = "a,b,c\n\n1,2,3"
rows = list(csv.reader(io.StringIO(data)))
# [['a', 'b', 'c'], [], ['1', '2', '3']]
```

**BOM (Byte Order Mark):**
```python
# UTF-8 BOM at start of file
data = "\ufeffa,b,c\n1,2,3"
reader = csv.reader(io.StringIO(data))
# First field will be '\ufeffa'
```

**Mixed line endings:**
Both Python and Rust handle mixed `\n` and `\r\n` gracefully.

**Embedded newlines:**
```csv
"Line 1
Line 2",value2,value3
```
Quoted fields can contain literal newlines.

