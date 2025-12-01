# secrets - Cryptographically Strong Random Numbers

Python's secrets module provides cryptographically secure random number generation suitable for managing secrets like account authentication, tokens, and security-sensitive data. Depyler transpiles these operations to Rust's `rand` crate with cryptographically secure RNGs.

## Python → Rust Mapping

| Python Function | Rust Equivalent | Notes |
|-----------------|-----------------|-------|
| `import secrets` | `use rand::*` | Secure random generation |
| `secrets.token_hex(n)` | `rand::thread_rng().gen::<[u8; n]>()` to hex | Random hex string |
| `secrets.token_urlsafe(n)` | Base64 URL-safe encoding | URL-safe random token |
| `secrets.token_bytes(n)` | `rand::thread_rng().gen::<[u8; n]>()` | Random bytes |
| `secrets.randbelow(n)` | `rng.gen_range(0..n)` | Random int [0, n) |
| `secrets.choice(seq)` | `seq.choose(&mut rng)` | Random element |
| `secrets.compare_digest(a, b)` | `constant_time_eq(a, b)` | Timing-safe comparison |

## Random Token Generation

### token_hex() - Hexadecimal Tokens

Generate random tokens as hexadecimal strings:

```python
import secrets

def test_token_hex() -> int:
    # Generate random hex token (16 bytes = 32 hex chars)
    token = secrets.token_hex(16)

    # Verify length
    length = len(token)

    return length
```

**Generated Rust:**

```rust
use rand::{Rng, thread_rng};

fn test_token_hex() -> i32 {
    // Generate random hex token (16 bytes = 32 hex chars)
    let mut rng = thread_rng();
    let bytes: [u8; 16] = rng.gen();
    let token = hex::encode(bytes);

    // Verify length
    token.len() as i32
}
```

**token_hex() Properties:**
- Output: Hexadecimal string (2 chars per byte)
- Use case: Session tokens, API keys, reset tokens
- Security: Cryptographically secure (not predictable)
- Default size: 32 bytes (64 hex characters)

### token_urlsafe() - URL-Safe Tokens

Generate tokens safe for URLs and filenames:

```python
import secrets

def test_token_urlsafe() -> int:
    # Generate URL-safe token
    token = secrets.token_urlsafe(16)

    # Verify it's a string and has length
    length = len(token)

    return length
```

**Generated Rust:**

```rust
use rand::{Rng, thread_rng};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

fn test_token_urlsafe() -> i32 {
    // Generate URL-safe token
    let mut rng = thread_rng();
    let bytes: [u8; 16] = rng.gen();
    let token = URL_SAFE_NO_PAD.encode(bytes);

    // Verify it's a string and has length
    token.len() as i32
}
```

**token_urlsafe() Properties:**
- Output: Base64 URL-safe string (no padding)
- Characters: A-Z, a-z, 0-9, -, _ (no +, /, =)
- Use case: URL parameters, file names, cookies
- Safe: No URL encoding needed

### token_bytes() - Raw Random Bytes

Generate cryptographically strong random bytes:

```python
import secrets

def test_token_bytes() -> int:
    # Generate random bytes
    token = secrets.token_bytes(16)

    # Verify length
    length = len(token)

    return length
```

**Generated Rust:**

```rust
use rand::{Rng, thread_rng};

fn test_token_bytes() -> i32 {
    // Generate random bytes
    let mut rng = thread_rng();
    let token: [u8; 16] = rng.gen();

    // Verify length
    token.len() as i32
}
```

**token_bytes() Properties:**
- Output: Raw bytes (not text-safe)
- Use case: Encryption keys, salt values, nonces
- Security: Maximum entropy
- Binary safe: Can contain any byte value

## Random Selection

### randbelow() - Random Integer Below N

Generate random integer in range [0, n):

```python
import secrets

def test_randbelow() -> int:
    # Generate random number below 100
    num = secrets.randbelow(100)

    # Verify it's in valid range (0-99)
    if num >= 0 and num < 100:
        return 1
    else:
        return 0
```

**Generated Rust:**

```rust
use rand::{Rng, thread_rng};

fn test_randbelow() -> i32 {
    // Generate random number below 100
    let mut rng = thread_rng();
    let num = rng.gen_range(0..100);

    // Verify it's in valid range (0-99)
    if num >= 0 && num < 100 {
        1
    } else {
        0
    }
}
```

**randbelow() Properties:**
- Range: [0, n) - includes 0, excludes n
- Use case: Random indices, dice rolls, random selection
- Uniform distribution: All values equally likely
- Security: No modulo bias (uses rejection sampling)

### choice() - Random Element from Sequence

Securely choose random element from sequence:

```python
import secrets

def test_choice() -> int:
    # Choose random element from list
    options = [10, 20, 30, 40, 50]
    selected = secrets.choice(options)

    # Verify it's one of the options
    if selected in options:
        return 1
    else:
        return 0
```

**Generated Rust:**

```rust
use rand::{seq::SliceRandom, thread_rng};

fn test_choice() -> i32 {
    // Choose random element from list
    let options = vec![10, 20, 30, 40, 50];
    let mut rng = thread_rng();
    let selected = *options.choose(&mut rng).unwrap();

    // Verify it's one of the options
    if options.contains(&selected) {
        1
    } else {
        0
    }
}
```

**choice() Properties:**
- Input: Any sequence (list, tuple, string)
- Output: Single randomly chosen element
- Use case: Random selection, weighted choices (with preprocessing)
- Uniform: Each element equally likely

## Secure Comparison

### compare_digest() - Constant-Time Comparison

Prevent timing attacks with constant-time string comparison:

```python
import secrets

def test_compare_digest() -> int:
    # Compare two identical strings (constant-time)
    str1 = "secret_value_123"
    str2 = "secret_value_123"

    if secrets.compare_digest(str1, str2):
        return 1
    else:
        return 0
```

**Generated Rust:**

```rust
use subtle::ConstantTimeEq;

fn test_compare_digest() -> i32 {
    // Compare two identical strings (constant-time)
    let str1 = b"secret_value_123";
    let str2 = b"secret_value_123";

    if str1.ct_eq(str2).into() {
        1
    } else {
        0
    }
}
```

**compare_digest() Properties:**
- Timing-safe: Takes same time regardless of where strings differ
- Use case: Password verification, HMAC comparison, token validation
- Security: Prevents timing side-channel attacks
- Important: Regular `==` leaks timing information

**Timing Attack Example:**
```python
# ❌ BAD: Vulnerable to timing attack
def verify_token_unsafe(user_token, valid_token):
    return user_token == valid_token  # Stops at first mismatch

# ✅ GOOD: Timing-safe comparison
def verify_token_safe(user_token, valid_token):
    return secrets.compare_digest(user_token, valid_token)
```

## Common Use Cases

### 1. Session Token Generation

```python
import secrets

def generate_session_token() -> str:
    """Generate secure session token."""
    return secrets.token_urlsafe(32)  # 256 bits of randomness

# Usage: Set as HTTP cookie
# Set-Cookie: session=<token>; Secure; HttpOnly; SameSite=Strict
```

### 2. Password Reset Token

```python
import secrets
import hashlib

def create_reset_token() -> tuple:
    """Create password reset token and hash."""
    # Generate token
    token = secrets.token_urlsafe(32)

    # Hash for database storage
    token_hash = hashlib.sha256(token.encode()).hexdigest()

    return token, token_hash  # Send token, store hash
```

### 3. API Key Generation

```python
import secrets

def generate_api_key() -> str:
    """Generate API key with prefix."""
    prefix = "sk_live_"
    random_part = secrets.token_hex(24)  # 192 bits
    return prefix + random_part
```

### 4. CSRF Token

```python
import secrets

def generate_csrf_token() -> str:
    """Generate Cross-Site Request Forgery token."""
    return secrets.token_hex(16)  # 128 bits sufficient for CSRF
```

## Performance Characteristics

| Operation | Speed | Entropy | Use Case |
|-----------|-------|---------|----------|
| `token_hex(16)` | Fast | 128 bits | Session tokens |
| `token_urlsafe(16)` | Fast | 128 bits | URL tokens |
| `token_bytes(32)` | Fastest | 256 bits | Crypto keys |
| `randbelow(n)` | Fast | log₂(n) bits | Random int |
| `choice(seq)` | Fast | log₂(len) bits | Random element |
| `compare_digest()` | Constant | N/A | Secure comparison |

**Performance Notes:**
- All functions use cryptographically secure RNG (CSPRNG)
- Rust's `rand` crate uses hardware RNG when available (RDRAND, RDSEED)
- No performance penalty vs `random` module for generation
- `compare_digest()` has constant-time guarantee
- Thread-local RNG initialization only once per thread

## Security Best Practices

**DO:**
- ✅ Use `secrets` for security-sensitive randomness
- ✅ Use sufficient entropy (≥128 bits for tokens)
- ✅ Use `compare_digest()` for token validation
- ✅ Store token hashes, not tokens
- ✅ Use URL-safe tokens in URLs
- ✅ Set token expiration times

**DON'T:**
- ❌ Use `random` module for security
- ❌ Use predictable seeds
- ❌ Reuse tokens across users
- ❌ Log or expose tokens
- ❌ Use short tokens (<16 bytes)
- ❌ Compare tokens with `==`

**Token Size Guidelines:**
```python
# Minimum recommended sizes
SESSION_TOKEN = 32  # 256 bits - long-term sessions
API_KEY = 24        # 192 bits - API authentication  
CSRF_TOKEN = 16     # 128 bits - CSRF protection
RESET_TOKEN = 32    # 256 bits - password reset
NONCE = 16          # 128 bits - one-time use
```

## Entropy and Security

**Entropy Calculations:**
- `token_hex(n)`: n × 8 bits (each byte = 8 bits)
- `token_urlsafe(n)`: ~n × 6 bits (Base64 encoding)
- `randbelow(n)`: log₂(n) bits

**Security Levels:**
- 128 bits: Secure against brute force (2¹²⁸ attempts)
- 192 bits: High security
- 256 bits: Maximum security (AES-256 equivalent)

**Attack Resistance:**
- Brute force: 2^(entropy) attempts needed
- Timing attacks: `compare_digest()` prevents
- Side-channel: CSPRNG resistant
- Prediction: Cryptographically impossible

## Testing

All examples in this chapter are verified by the test suite in `tdd-book/tests/test_secrets.py`. Run:

```bash
cd tdd-book
uv run pytest tests/test_secrets.py -v
```

**Expected Output:**
```
tests/test_secrets.py::test_secrets_token_hex PASSED                     [ 16%]
tests/test_secrets.py::test_secrets_token_urlsafe PASSED                 [ 33%]
tests/test_secrets.py::test_secrets_randbelow PASSED                     [ 50%]
tests/test_secrets.py::test_secrets_choice PASSED                        [ 66%]
tests/test_secrets.py::test_secrets_token_bytes PASSED                   [ 83%]
tests/test_secrets.py::test_secrets_compare_digest PASSED                [100%]

====== 6 passed in 0.XX s ======
```

## Comparison: secrets vs random

| Feature | `secrets` | `random` |
|---------|-----------|----------|
| Security | ✅ Cryptographic | ❌ Pseudo-random |
| Predictable | ❌ No | ✅ Yes (seedable) |
| Use for passwords | ✅ Yes | ❌ Never |
| Use for tokens | ✅ Yes | ❌ Never |
| Use for simulations | ⚠️ Overkill | ✅ Perfect |
| Use for games | ⚠️ Overkill | ✅ Ideal |
| Performance | Fast | Faster |
| Seedable | ❌ No | ✅ Yes |

**When to use secrets:**
- Password generation
- Security tokens (session, API, CSRF)
- Cryptographic keys and nonces
- Authentication challenges
- Security-sensitive random selection

**When to use random instead:**
- Simulations and modeling
- Games and entertainment
- Monte Carlo methods
- Non-security random sampling
- Reproducible randomness needed

## CSPRNG Implementation

**Python's secrets module:**
- Uses `os.urandom()` (reads from `/dev/urandom` on Unix)
- Falls back to CryptGenRandom on Windows
- Guaranteed cryptographically secure

**Rust's rand crate:**
- Uses `ThreadRng` by default (thread-local CSPRNG)
- ChaCha20-based CSPRNG
- Hardware RNG when available (RDRAND/RDSEED)
- Automatically reseeded

**Both provide:**
- Unpredictable output
- Uniform distribution
- No observable patterns
- Resistance to state compromise

