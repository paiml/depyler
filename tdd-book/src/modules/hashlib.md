# hashlib - Cryptographic Hash Functions

Python's hashlib module provides secure hash and message digest algorithms. Depyler transpiles these operations to Rust's cryptographic crates with constant-time implementations and hardware acceleration.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import hashlib` | `use sha2::*, md5::*, sha1::*` | Cryptographic hashing |
| `hashlib.md5()` | `Md5::new()` | MD5 (insecure, legacy only) |
| `hashlib.sha1()` | `Sha1::new()` | SHA-1 (deprecated) |
| `hashlib.sha256()` | `Sha256::new()` | SHA-256 (recommended) |
| `hashlib.sha512()` | `Sha512::new()` | SHA-512 (high security) |
| `hash.update(data)` | `hasher.update(data)` | Incremental hashing |
| `hash.hexdigest()` | `format!("{:x}", hasher.finalize())` | Hex output |
| `hash.digest()` | `hasher.finalize()` | Binary output |

## MD5 Hashing (Legacy)

### Basic MD5 Hash

**⚠️ Warning**: MD5 is cryptographically broken. Use only for non-security purposes (checksums, cache keys).

```python
import hashlib

def test_md5() -> str:
    # Create MD5 hash
    data = "hello world"
    hash_obj = hashlib.md5(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

    return result
```

**Generated Rust:**

```rust
use md5::{Md5, Digest};

fn test_md5() -> String {
    // Create MD5 hash
    let data = "hello world";
    let mut hasher = Md5::new();
    hasher.update(data.as_bytes());

    // Get hexadecimal digest
    let result = format!("{:x}", hasher.finalize());

    result
}
```

**MD5 Properties:**
- Output: 128 bits (32 hex characters)
- Speed: ~400 MB/s (software)
- Security: ❌ Broken (collisions found)
- Use case: Checksums, non-security identifiers

## SHA-256 Hashing (Recommended)

### Basic SHA-256 Hash

SHA-256 is the industry standard for secure hashing:

```python
import hashlib

def test_sha256() -> str:
    # Create SHA256 hash
    data = "hello world"
    hash_obj = hashlib.sha256(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

    return result
```

**Generated Rust:**

```rust
use sha2::{Sha256, Digest};

fn test_sha256() -> String {
    // Create SHA256 hash
    let data = "hello world";
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    // Get hexadecimal digest
    let result = format!("{:x}", hasher.finalize());

    result
}
```

**SHA-256 Properties:**
- Output: 256 bits (64 hex characters)
- Speed: ~150 MB/s (software), ~500 MB/s (hardware)
- Security: ✅ Secure (no practical attacks)
- Use case: Passwords, data integrity, digital signatures

## SHA-1 Hashing (Deprecated)

### Basic SHA-1 Hash

**⚠️ Deprecation Notice**: SHA-1 is deprecated due to collision vulnerabilities. Use SHA-256 or higher.

```python
import hashlib

def test_sha1() -> str:
    # Create SHA1 hash
    data = "test data"
    hash_obj = hashlib.sha1(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

    return result
```

**Generated Rust:**

```rust
use sha1::{Sha1, Digest};

fn test_sha1() -> String {
    // Create SHA1 hash
    let data = "test data";
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());

    // Get hexadecimal digest
    let result = format!("{:x}", hasher.finalize());

    result
}
```

**SHA-1 Properties:**
- Output: 160 bits (40 hex characters)
- Speed: ~600 MB/s (software)
- Security: ⚠️ Broken (collision attacks exist)
- Use case: Legacy systems, git commits (safe context)

## SHA-512 Hashing (High Security)

### Basic SHA-512 Hash

SHA-512 provides maximum security with larger output:

```python
import hashlib

def test_sha512() -> str:
    # Create SHA512 hash
    data = "secure data"
    hash_obj = hashlib.sha512(data.encode('utf-8'))

    # Get hexadecimal digest (first 16 chars for brevity)
    full_hash = hash_obj.hexdigest()
    result = full_hash[:16]

    return result
```

**Generated Rust:**

```rust
use sha2::{Sha512, Digest};

fn test_sha512() -> String {
    // Create SHA512 hash
    let data = "secure data";
    let mut hasher = Sha512::new();
    hasher.update(data.as_bytes());

    // Get hexadecimal digest (first 16 chars)
    let full_hash = format!("{:x}", hasher.finalize());
    full_hash[..16].to_string()
}
```

**SHA-512 Properties:**
- Output: 512 bits (128 hex characters)
- Speed: ~200 MB/s (64-bit), faster on 64-bit systems
- Security: ✅ Highly secure
- Use case: Maximum security requirements, long-term protection

## Incremental Hashing

### Using update() for Streaming

Hash data incrementally without loading everything into memory:

```python
import hashlib

def test_update() -> str:
    # Create hash with incremental updates
    hash_obj = hashlib.sha256()
    hash_obj.update("hello".encode('utf-8'))
    hash_obj.update(" ".encode('utf-8'))
    hash_obj.update("world".encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

    return result
```

**Generated Rust:**

```rust
use sha2::{Sha256, Digest};

fn test_update() -> String {
    // Create hash with incremental updates
    let mut hasher = Sha256::new();
    hasher.update(b"hello");
    hasher.update(b" ");
    hasher.update(b"world");

    // Get hexadecimal digest
    let result = format!("{:x}", hasher.finalize());

    result
}
```

**Incremental Hashing Benefits:**
- Memory efficient for large files
- Allows streaming data processing
- Same result as hashing concatenated data
- `hash("a" + "b") == hash("a").update("b")`

### Hash Comparison for Data Integrity

Verify data hasn't changed by comparing hashes:

```python
import hashlib

def test_comparison() -> int:
    # Hash same data twice
    data = "important data"

    hash1 = hashlib.sha256(data.encode('utf-8')).hexdigest()
    hash2 = hashlib.sha256(data.encode('utf-8')).hexdigest()

    # Hashes should be identical
    if hash1 == hash2:
        return 1
    else:
        return 0
```

**Generated Rust:**

```rust
use sha2::{Sha256, Digest};

fn test_comparison() -> i32 {
    // Hash same data twice
    let data = "important data";

    let mut hasher1 = Sha256::new();
    hasher1.update(data.as_bytes());
    let hash1 = format!("{:x}", hasher1.finalize());

    let mut hasher2 = Sha256::new();
    hasher2.update(data.as_bytes());
    let hash2 = format!("{:x}", hasher2.finalize());

    // Hashes should be identical
    if hash1 == hash2 { 1 } else { 0 }
}
```

**Hash Comparison Properties:**
- Deterministic: same input → same hash
- Collision resistance: different inputs → different hashes (with high probability)
- Avalanche effect: tiny change → completely different hash
- Use case: File integrity, password verification, data deduplication

## Common Use Cases

### 1. File Integrity Verification

```python
import hashlib

def hash_file(filename: str) -> str:
    """Calculate SHA-256 hash of a file."""
    hasher = hashlib.sha256()

    with open(filename, 'rb') as f:
        # Read file in chunks for memory efficiency
        while chunk := f.read(8192):
            hasher.update(chunk)

    return hasher.hexdigest()
```

### 2. Password Hashing (Basic)

**⚠️ Warning**: Never use plain hashlib for passwords. Use `bcrypt`, `scrypt`, or `argon2` instead.

```python
import hashlib

def simple_password_hash(password: str, salt: str) -> str:
    """Basic password hashing (NOT secure for production)."""
    # Combine password and salt
    salted = (password + salt).encode('utf-8')

    # Hash multiple times (still not secure enough)
    result = hashlib.sha256(salted).hexdigest()
    for _ in range(10000):
        result = hashlib.sha256(result.encode('utf-8')).hexdigest()

    return result
```

### 3. Cache Key Generation

```python
import hashlib
import json

def cache_key(data: dict) -> str:
    """Generate cache key from dictionary."""
    # Serialize data to JSON
    json_str = json.dumps(data, sort_keys=True)

    # Hash the JSON
    hash_obj = hashlib.sha256(json_str.encode('utf-8'))

    # Return first 16 characters (sufficient for cache keys)
    return hash_obj.hexdigest()[:16]
```

### 4. Data Deduplication

```python
import hashlib

def content_hash(data: bytes) -> str:
    """Generate content-addressable hash."""
    return hashlib.sha256(data).hexdigest()

def is_duplicate(data: bytes, existing_hashes: set) -> bool:
    """Check if data is duplicate based on hash."""
    data_hash = content_hash(data)
    return data_hash in existing_hashes
```

## Performance Characteristics

| Algorithm | Output Size | Speed (MB/s) | Security | Use Case |
|-----------|-------------|--------------|----------|----------|
| MD5 | 128 bits | ~400 | ❌ Broken | Checksums only |
| SHA-1 | 160 bits | ~600 | ⚠️ Weak | Legacy systems |
| SHA-256 | 256 bits | ~150 | ✅ Secure | General purpose |
| SHA-512 | 512 bits | ~200 | ✅ Secure | High security |

**Performance Notes:**
- Rust implementations use hardware acceleration (AES-NI, SHA extensions)
- SHA-256 has best speed/security balance
- SHA-512 faster on 64-bit systems
- All algorithms are O(n) in input size

**Rust Performance Advantages:**
- Zero-copy operations where possible
- SIMD optimizations automatically applied
- Constant-time implementations (timing attack resistant)
- No GIL (Python's Global Interpreter Lock)

## Security Considerations

**DO:**
- ✅ Use SHA-256 or higher for security applications
- ✅ Use dedicated password hashing (bcrypt, argon2)
- ✅ Verify file integrity with SHA-256
- ✅ Use salt for password hashing
- ✅ Hash before comparison (constant-time)

**DON'T:**
- ❌ Use MD5 for security (collisions exist)
- ❌ Use SHA-1 for new systems
- ❌ Use plain hashing for passwords
- ❌ Roll your own crypto
- ❌ Expose hash timing information

**Timing Attack Prevention:**
```rust
// ✅ GOOD: Constant-time comparison
use subtle::ConstantTimeEq;

fn verify_hash(hash1: &[u8], hash2: &[u8]) -> bool {
    hash1.ct_eq(hash2).into()
}

// ❌ BAD: Timing attack vulnerable
fn verify_hash_bad(hash1: &[u8], hash2: &[u8]) -> bool {
    hash1 == hash2  // Stops at first mismatch
}
```

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_hashlib.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_hashlib.py -v
```

**Expected Output:**
```
tests/test_hashlib.py::test_hashlib_md5_basic PASSED                     [ 16%]
tests/test_hashlib.py::test_hashlib_sha256_basic PASSED                  [ 33%]
tests/test_hashlib.py::test_hashlib_sha1_basic PASSED                    [ 50%]
tests/test_hashlib.py::test_hashlib_sha512_basic PASSED                  [ 66%]
tests/test_hashlib.py::test_hashlib_update_incremental PASSED            [ 83%]
tests/test_hashlib.py::test_hashlib_hash_comparison PASSED               [100%]

====== 6 passed in 0.XX s ======
```

## Hash Function Selection Guide

**Choose SHA-256 when:**
- General-purpose hashing needed
- File integrity verification
- Digital signatures
- Best balance of speed and security

**Choose SHA-512 when:**
- Maximum security required
- Long-term data protection
- 64-bit systems (performance advantage)
- Sensitive data handling

**Choose MD5 when:**
- Non-security use cases only
- Legacy system compatibility required
- Checksums for corruption detection
- Maximum performance needed

**Never use for passwords:**
- Use `bcrypt` (Rust: `bcrypt` crate)
- Use `argon2` (Rust: `argon2` crate)  
- Use `scrypt` (Rust: `scrypt` crate)

## Cryptographic Properties

**Preimage Resistance:**
- Given hash H, computationally infeasible to find input x where hash(x) = H
- ✅ SHA-256, SHA-512: Secure
- ❌ MD5: Broken

**Collision Resistance:**
- Computationally infeasible to find x ≠ y where hash(x) = hash(y)
- ✅ SHA-256, SHA-512: Secure
- ❌ MD5, SHA-1: Broken (collisions found)

**Avalanche Effect:**
- Changing one bit in input changes ~50% of output bits
- All algorithms demonstrate strong avalanche effect

**Determinism:**
- Same input always produces same output
- Critical for caching and integrity verification

