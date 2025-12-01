# string - Common String Operations

Python's string type provides rich text manipulation capabilities. Depyler transpiles these to Rust's `String` and `&str` types with full type safety and Unicode support.

## Python → Rust Type Mapping

| Python Type | Rust Type | Notes |
|------------|-----------|-------|
| `str` | `String` | Owned, growable string |
| String literal | `&str` | String slice, borrowed |

## Case Transformation

### upper() / lower()

Convert strings to upper or lower case:

```python
def string_case() -> str:
    text: str = "Hello World"
    upper_text = text.upper()  # → to_uppercase()
    lower_text = text.lower()  # → to_lowercase()
    return lower_text  # "hello world"
```

**Generated Rust:**

```rust
fn string_case() -> String {
    let text: String = "Hello World".to_string();
    let upper_text = text.to_uppercase();
    let lower_text = text.to_lowercase();
    lower_text
}
```

**Method Coverage:**
- ✅ `upper()` → `to_uppercase()`
- ✅ `lower()` → `to_lowercase()`

## Whitespace Handling

### strip() / lstrip() / rstrip()

Remove leading/trailing whitespace:

```python
def string_trim() -> str:
    text: str = "  hello  "
    stripped = text.strip()  # → trim()
    return stripped  # "hello"
```

**Generated Rust:**

```rust
fn string_trim() -> String {
    let text: String = "  hello  ".to_string();
    let stripped = text.trim().to_string();
    stripped
}
```

**Method Coverage:**
- ✅ `strip()` → `trim()`
- ✅ `lstrip()` → `trim_start()`
- ✅ `rstrip()` → `trim_end()`

## Splitting and Joining

### split() / join()

Split strings into parts and join them back:

```python
def string_split_join() -> str:
    text: str = "apple,banana,cherry"
    
    # Split by delimiter
    parts = text.split(",")  # → split()
    
    # Join with new delimiter
    rejoined = "-".join(parts)  # → join()
    
    return rejoined  # "apple-banana-cherry"
```

**Generated Rust:**

```rust
fn string_split_join() -> String {
    let text: String = "apple,banana,cherry".to_string();
    
    let parts: Vec<String> = text
        .split(",")
        .map(|s| s.to_string())
        .collect();
    
    let rejoined = parts.join("-");
    
    rejoined
}
```

**Method Coverage:**
- ✅ `split(sep)` → `split(sep)`
- ✅ `join(iterable)` → `join()`

## Searching

### find() / startswith() / endswith()

Search for substrings and check prefixes/suffixes:

```python
def string_search() -> bool:
    text: str = "hello world"
    
    # Check prefix
    starts = text.startswith("hello")  # → starts_with()
    
    # Check suffix
    ends = text.endswith("world")  # → ends_with()
    
    # Find position
    pos = text.find("world")  # → find()
    
    return starts and ends  # True
```

**Generated Rust:**

```rust
fn string_search() -> bool {
    let text: String = "hello world".to_string();
    
    let starts = text.starts_with("hello");
    let ends = text.ends_with("world");
    let pos = text.find("world");
    
    starts && ends
}
```

**Method Coverage:**
- ✅ `find(sub)` → `find(sub)`
- ✅ `startswith(prefix)` → `starts_with(prefix)`
- ✅ `endswith(suffix)` → `ends_with(suffix)`

## Replacement

### replace()

Replace substrings with new values:

```python
def string_replace() -> str:
    text: str = "hello hello hello"
    replaced = text.replace("hello", "hi")  # → replace()
    return replaced  # "hi hi hi"
```

**Generated Rust:**

```rust
fn string_replace() -> String {
    let text: String = "hello hello hello".to_string();
    let replaced = text.replace("hello", "hi");
    replaced
}
```

**Method Coverage:**
- ✅ `replace(old, new)` → `replace(old, new)`

## Counting

### count()

Count occurrences of substring:

```python
def string_count() -> int:
    text: str = "hello hello hello"
    count = text.count("hello")  # → matches().count()
    return count  # 3
```

**Generated Rust:**

```rust
fn string_count() -> i32 {
    let text: String = "hello hello hello".to_string();
    let count = text.matches("hello").count() as i32;
    count
}
```

**Method Coverage:**
- ✅ `count(sub)` → `matches(sub).count()`

## Validation

### isdigit() / isalpha()

Check string contents:

```python
def string_validation() -> bool:
    text: str = "12345"
    is_digit = text.isdigit()  # → chars().all(|c| c.is_numeric())
    
    text2: str = "hello"
    is_alpha = text2.isalpha()  # → chars().all(|c| c.is_alphabetic())
    
    return is_digit and is_alpha  # True
```

**Generated Rust:**

```rust
fn string_validation() -> bool {
    let text: String = "12345".to_string();
    let is_digit = text.chars().all(|c| c.is_numeric());
    
    let text2: String = "hello".to_string();
    let is_alpha = text2.chars().all(|c| c.is_alphabetic());
    
    is_digit && is_alpha
}
```

**Method Coverage:**
- ✅ `isdigit()` → `chars().all(|c| c.is_numeric())`
- ✅ `isalpha()` → `chars().all(|c| c.is_alphabetic())`

## Complete Method Coverage

All 11 common string methods are supported:

| Python Method | Rust Equivalent | Status |
|--------------|-----------------|--------|
| `upper()` | `to_uppercase()` | ✅ |
| `lower()` | `to_lowercase()` | ✅ |
| `strip()` | `trim()` | ✅ |
| `startswith()` | `starts_with()` | ✅ |
| `endswith()` | `ends_with()` | ✅ |
| `split()` | `split()` | ✅ |
| `join()` | `join()` | ✅ |
| `find()` | `find()` | ✅ |
| `replace()` | `replace()` | ✅ |
| `count()` | `matches().count()` | ✅ |
| `isdigit()` | `chars().all(is_numeric)` | ✅ |
| `isalpha()` | `chars().all(is_alphabetic)` | ✅ |

## Unicode Safety

Rust's string handling provides strong Unicode guarantees:

```python
def unicode_example() -> str:
    text: str = "Hello 世界 🌍"
    upper = text.upper()
    return upper
```

**Generated Rust:**

```rust
fn unicode_example() -> String {
    let text: String = "Hello 世界 🌍".to_string();
    let upper = text.to_uppercase();
    upper
}
```

**Safety Guarantees:**
- All strings are valid UTF-8
- Character operations respect grapheme clusters
- No invalid Unicode sequences possible

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| upper/lower | O(n) | O(n) | Unicode-aware |
| strip | O(n) | O(n) | Efficient slicing |
| split | O(n) | O(n) | Iterator-based |
| find | O(nm) | O(nm) | Boyer-Moore available |

## Memory Safety Guarantees

Depyler's generated Rust code provides:

- **No buffer overflows**: All string operations are bounds-checked
- **UTF-8 validity**: Strings are always valid Unicode
- **No null terminator issues**: Length-prefixed strings
- **Ownership clarity**: Borrowed vs owned strings explicit

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_string.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_string.py -v
```
