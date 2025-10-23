# base64 - Base64 Encoding

Python's base64 module provides functions for encoding and decoding data using Base64, Base32, Base16, and other ASCII-based binary-to-text encoding schemes. Depyler transpiles these operations to Rust's `base64` crate with full type safety.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import base64` | `use base64::*` | Base64 encoding |
| `b64encode(data)` | `base64::encode(data)` | Encode to base64 |
| `b64decode(data)` | `base64::decode(data)` | Decode from base64 |
| `urlsafe_b64encode(data)` | `base64::encode_config(data, URL_SAFE)` | URL-safe encoding |
| `urlsafe_b64decode(data)` | `base64::decode_config(data, URL_SAFE)` | URL-safe decoding |
| `b32encode(data)` | Custom Base32 impl | Base32 encoding |
| `b16encode(data)` | `hex::encode(data)` | Hexadecimal encoding |

## Standard Base64 Encoding

### Basic Encoding

Encode text to Base64:

```python
import base64

def test_encode() -> str:
    # Encode string as base64
    data = "hello world"
    encoded = base64.b64encode(data.encode('utf-8'))

    # Convert back to string for return
    result = encoded.decode('utf-8')

    return result
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_encode() -> String {
    // Encode string as base64
    let data = "hello world";
    let encoded = general_purpose::STANDARD.encode(data.as_bytes());

    // Result is already a String
    encoded
}
```

**Base64 Alphabet:**
- Characters: `A-Z`, `a-z`, `0-9`, `+`, `/`
- Padding: `=` (used when data length is not multiple of 3)
- Output: Always ASCII text (safe for transmission)

### Basic Decoding

Decode Base64 to text:

```python
import base64

def test_decode() -> str:
    # Decode base64 string
    encoded = "aGVsbG8gd29ybGQ="
    decoded = base64.b64decode(encoded)

    # Convert to string
    result = decoded.decode('utf-8')

    return result
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_decode() -> String {
    // Decode base64 string
    let encoded = "aGVsbG8gd29ybGQ=";
    let decoded = general_purpose::STANDARD.decode(encoded)
        .expect("Invalid base64");

    // Convert to string
    String::from_utf8(decoded).expect("Invalid UTF-8")
}
```

## Round-Trip Encoding

### Encode and Decode

Verify encoding preserves data:

```python
import base64

def test_roundtrip() -> str:
    # Original text
    original = "Test data 123!@#"

    # Encode then decode
    encoded = base64.b64encode(original.encode('utf-8'))
    decoded = base64.b64decode(encoded)
    result = decoded.decode('utf-8')

    return result
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_roundtrip() -> String {
    // Original text
    let original = "Test data 123!@#";

    // Encode then decode
    let encoded = general_purpose::STANDARD.encode(original.as_bytes());
    let decoded = general_purpose::STANDARD.decode(&encoded)
        .expect("Invalid base64");

    String::from_utf8(decoded).expect("Invalid UTF-8")
}
```

**Round-Trip Property:**
- `decode(encode(x)) == x` for all valid inputs
- Encoding is deterministic (same input → same output)
- Decoding validates input format

## URL-Safe Base64

### URL-Safe Encoding

Encode for use in URLs and filenames:

```python
import base64

def test_urlsafe_encode() -> str:
    # URL-safe encoding (replaces + and / with - and _)
    data = "test data with special chars"
    encoded = base64.urlsafe_b64encode(data.encode('utf-8'))

    result = encoded.decode('utf-8')

    return result
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_urlsafe_encode() -> String {
    // URL-safe encoding (replaces + and / with - and _)
    let data = "test data with special chars";
    let encoded = general_purpose::URL_SAFE.encode(data.as_bytes());

    encoded
}
```

**URL-Safe Differences:**
- Standard: Uses `+` and `/`
- URL-Safe: Uses `-` and `_`
- Avoids characters that need URL encoding
- Same padding rules apply

### URL-Safe Decoding

```python
import base64

def test_urlsafe_decode() -> str:
    # URL-safe decoding
    encoded = "dGVzdCBkYXRhIHdpdGggc3BlY2lhbCBjaGFycw=="
    decoded = base64.urlsafe_b64decode(encoded)

    result = decoded.decode('utf-8')

    return result
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_urlsafe_decode() -> String {
    // URL-safe decoding
    let encoded = "dGVzdCBkYXRhIHdpdGggc3BlY2lhbCBjaGFycw==";
    let decoded = general_purpose::URL_SAFE.decode(encoded)
        .expect("Invalid base64");

    String::from_utf8(decoded).expect("Invalid UTF-8")
}
```

## Base64 Padding

### Understanding Padding

Base64 padding ensures output length is multiple of 4:

```python
import base64

def test_padding() -> int:
    # Test various lengths (different padding)
    test1 = base64.b64encode("a".encode('utf-8'))  # "YQ=="
    test2 = base64.b64encode("ab".encode('utf-8'))  # "YWI="
    test3 = base64.b64encode("abc".encode('utf-8'))  # "YWJj"

    # Count total length
    total = len(test1) + len(test2) + len(test3)

    return total
```

**Generated Rust:**

```rust
use base64::{Engine as _, engine::general_purpose};

fn test_padding() -> i32 {
    // Test various lengths (different padding)
    let test1 = general_purpose::STANDARD.encode("a".as_bytes());  // "YQ=="
    let test2 = general_purpose::STANDARD.encode("ab".as_bytes());  // "YWI="
    let test3 = general_purpose::STANDARD.encode("abc".as_bytes());  // "YWJj"

    // Count total length
    let total = (test1.len() + test2.len() + test3.len()) as i32;

    total
}
```

**Padding Rules:**
- Input: 1 byte → Output: 4 chars with `==`
- Input: 2 bytes → Output: 4 chars with `=`
- Input: 3 bytes → Output: 4 chars (no padding)
- Output length: `ceil(input_len * 4 / 3)` rounded to multiple of 4

## Common Use Cases

### 1. Encode Binary Data for JSON

```python
import base64
import json

def encode_binary_for_json(data: bytes) -> str:
    # Encode binary data as base64 string
    encoded = base64.b64encode(data).decode('utf-8')

    # Include in JSON
    json_data = json.dumps({"data": encoded})

    return json_data
```

### 2. Email Attachments (MIME)

```python
import base64

def encode_email_attachment(file_data: bytes) -> str:
    # Encode file data for email transmission
    encoded = base64.b64encode(file_data).decode('utf-8')

    # Split into 76-character lines (MIME requirement)
    lines = [encoded[i:i+76] for i in range(0, len(encoded), 76)]

    return '\\n'.join(lines)
```

### 3. Basic Authentication Header

```python
import base64

def create_basic_auth_header(username: str, password: str) -> str:
    # Create HTTP Basic Auth header
    credentials = f"{username}:{password}"
    encoded = base64.b64encode(credentials.encode('utf-8')).decode('utf-8')

    return f"Basic {encoded}"
```

## Performance Characteristics

| Operation | Python | Rust | Notes |
|-----------|--------|------|-------|
| Encode 1KB | ~10 μs | ~3 μs | Rust 3x faster |
| Decode 1KB | ~12 μs | ~4 μs | Rust 3x faster |
| Encode 1MB | ~10 ms | ~3 ms | Linear scaling |
| Decode 1MB | ~12 ms | ~4 ms | Linear scaling |

**Performance Notes:**
- Rust's `base64` crate uses SIMD when available
- Both are O(n) in input size
- Encoding typically faster than decoding
- URL-safe has same performance as standard

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_base64.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_base64.py -v
```

**Expected Output:**
```
tests/test_base64.py::test_base64_encode_string PASSED   [ 16%]
tests/test_base64.py::test_base64_decode_string PASSED   [ 33%]
tests/test_base64.py::test_base64_roundtrip PASSED       [ 50%]
tests/test_base64.py::test_base64_urlsafe_encode PASSED  [ 66%]
tests/test_base64.py::test_base64_urlsafe_decode PASSED  [ 83%]
tests/test_base64.py::test_base64_padding PASSED         [100%]

====== 6 passed in 0.XX s ======
```

## Base64 Standard (RFC 4648)

**Encoding Process:**
1. Convert input to binary (8-bit bytes)
2. Group into 6-bit chunks
3. Map each 6-bit value to Base64 character
4. Add padding to make length multiple of 4

**Character Set:**
```
Value: 0-25  → A-Z
Value: 26-51 → a-z
Value: 52-61 → 0-9
Value: 62    → + (or - for URL-safe)
Value: 63    → / (or _ for URL-safe)
Padding:     → =
```

**Size Overhead:**
- Base64 increases size by ~33%
- 3 bytes input → 4 bytes output
- Tradeoff: Binary safety for size

## Safety and Error Handling

**Type Safety:**
- Python: Works with `bytes` and `str`
- Rust: Strong typing with `&[u8]` and `Vec<u8>`
- Both validate UTF-8 when converting to strings

**Error Handling:**
- Python: Raises `binascii.Error` for invalid input
- Rust: Returns `Result` types
- Invalid characters rejected
- Invalid padding detected

**Best Practices:**
```rust
// ❌ BAD: Unwrapping can panic
let decoded = base64::decode(input).unwrap();

// ✅ GOOD: Handle errors explicitly
match base64::decode(input) {
    Ok(decoded) => { /* use decoded */ },
    Err(e) => eprintln!("Invalid base64: {}", e),
}
```

## Comparison: Encoding Schemes

| Scheme | Chars/Byte | Overhead | Use Case |
|--------|-----------|----------|----------|
| Base64 | 1.33 | +33% | General purpose |
| Base64 URL | 1.33 | +33% | URLs, filenames |
| Base32 | 1.60 | +60% | Human-readable IDs |
| Base16 (Hex) | 2.00 | +100% | Debugging, hashes |
| Base85 | 1.25 | +25% | Efficiency-critical |

**When to use Base64:**
- Transmitting binary data over text protocols
- Embedding images in HTML/CSS/JSON
- Email attachments (MIME)
- HTTP Basic Authentication
- Storing binary in text databases

**When not to use Base64:**
- File storage (keep as binary)
- Performance-critical paths (use binary protocols)
- Human-readable identifiers (use Base32 or hex)
- Large data transfers (use compression first)

