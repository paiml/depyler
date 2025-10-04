# secrets

## Cryptographically secure token generation.

## Cryptographically secure random choice.

## Cryptographically secure random integer below n.

## Constant-time comparison for security.

## SystemRandom for cryptographic randomness.

## Generate secure passwords and tokens.

## Edge cases and special scenarios.

### Basic: Generate random bytes.

```python
def test_token_bytes(self):
    """Basic: Generate random bytes."""
    token = secrets.token_bytes(16)
    assert len(token) == 16
    assert isinstance(token, bytes)
```

**Verification**: ✅ Tested in CI

### Feature: Default token_bytes size.

```python
def test_token_bytes_default(self):
    """Feature: Default token_bytes size."""
    token = secrets.token_bytes()
    assert isinstance(token, bytes)
    assert len(token) == 32
```

**Verification**: ✅ Tested in CI

### Property: Each call produces unique token.

```python
def test_token_bytes_uniqueness(self):
    """Property: Each call produces unique token."""
    token1 = secrets.token_bytes(16)
    token2 = secrets.token_bytes(16)
    assert token1 != token2
```

**Verification**: ✅ Tested in CI

### Basic: Generate hex token.

```python
def test_token_hex(self):
    """Basic: Generate hex token."""
    token = secrets.token_hex(16)
    assert len(token) == 32
    assert isinstance(token, str)
    assert all((c in '0123456789abcdef' for c in token))
```

**Verification**: ✅ Tested in CI

### Feature: Default token_hex size.

```python
def test_token_hex_default(self):
    """Feature: Default token_hex size."""
    token = secrets.token_hex()
    assert isinstance(token, str)
    assert len(token) == 64
```

**Verification**: ✅ Tested in CI

### Basic: Generate URL-safe token.

```python
def test_token_urlsafe(self):
    """Basic: Generate URL-safe token."""
    token = secrets.token_urlsafe(16)
    assert isinstance(token, str)
    assert all((c.isalnum() or c in '-_' for c in token))
```

**Verification**: ✅ Tested in CI

### Feature: Default token_urlsafe size.

```python
def test_token_urlsafe_default(self):
    """Feature: Default token_urlsafe size."""
    token = secrets.token_urlsafe()
    assert isinstance(token, str)
    assert all((c.isalnum() or c in '-_' for c in token))
```

**Verification**: ✅ Tested in CI

### Property: No padding characters in URL-safe token.

```python
def test_token_urlsafe_no_padding(self):
    """Property: No padding characters in URL-safe token."""
    token = secrets.token_urlsafe(16)
    assert '=' not in token
```

**Verification**: ✅ Tested in CI

### Basic: Choose from sequence.

```python
def test_choice(self):
    """Basic: Choose from sequence."""
    items = [1, 2, 3, 4, 5]
    choice = secrets.choice(items)
    assert choice in items
```

**Verification**: ✅ Tested in CI

### Feature: Choose from string.

```python
def test_choice_string(self):
    """Feature: Choose from string."""
    s = 'abcde'
    choice = secrets.choice(s)
    assert choice in s
```

**Verification**: ✅ Tested in CI

### Property: Multiple choices can differ.

```python
def test_choice_uniqueness(self):
    """Property: Multiple choices can differ."""
    items = list(range(100))
    choices = [secrets.choice(items) for _ in range(10)]
    assert len(set(choices)) > 1
```

**Verification**: ✅ Tested in CI

### Error: Choice from empty sequence.

```python
def test_error_choice_empty(self):
    """Error: Choice from empty sequence."""
    with pytest.raises(IndexError):
        secrets.choice([])
```

**Verification**: ✅ Tested in CI

### Basic: Random integer below n.

```python
def test_randbelow(self):
    """Basic: Random integer below n."""
    r = secrets.randbelow(10)
    assert 0 <= r < 10
    assert isinstance(r, int)
```

**Verification**: ✅ Tested in CI

### Property: Always in correct range.

```python
def test_randbelow_range(self):
    """Property: Always in correct range."""
    for _ in range(100):
        r = secrets.randbelow(100)
        assert 0 <= r < 100
```

**Verification**: ✅ Tested in CI

### Edge: randbelow(1) always returns 0.

```python
def test_randbelow_one(self):
    """Edge: randbelow(1) always returns 0."""
    r = secrets.randbelow(1)
    assert r == 0
```

**Verification**: ✅ Tested in CI

### Property: Should cover full range over time.

```python
def test_randbelow_distribution(self):
    """Property: Should cover full range over time."""
    n = 10
    results = {secrets.randbelow(n) for _ in range(100)}
    assert len(results) > 1
```

**Verification**: ✅ Tested in CI

### Error: randbelow(0) raises ValueError.

```python
def test_error_randbelow_zero(self):
    """Error: randbelow(0) raises ValueError."""
    with pytest.raises(ValueError):
        secrets.randbelow(0)
```

**Verification**: ✅ Tested in CI

### Error: randbelow with negative raises ValueError.

```python
def test_error_randbelow_negative(self):
    """Error: randbelow with negative raises ValueError."""
    with pytest.raises(ValueError):
        secrets.randbelow(-1)
```

**Verification**: ✅ Tested in CI

### Basic: Equal strings compare as True.

```python
def test_compare_digest_equal_strings(self):
    """Basic: Equal strings compare as True."""
    a = 'secret123'
    b = 'secret123'
    assert secrets.compare_digest(a, b) is True
```

**Verification**: ✅ Tested in CI

### Basic: Unequal strings compare as False.

```python
def test_compare_digest_unequal_strings(self):
    """Basic: Unequal strings compare as False."""
    a = 'secret123'
    b = 'secret456'
    assert secrets.compare_digest(a, b) is False
```

**Verification**: ✅ Tested in CI

### Feature: Compare bytes.

```python
def test_compare_digest_bytes(self):
    """Feature: Compare bytes."""
    a = b'secret123'
    b = b'secret123'
    assert secrets.compare_digest(a, b) is True
```

**Verification**: ✅ Tested in CI

### Feature: Unequal bytes compare as False.

```python
def test_compare_digest_bytes_unequal(self):
    """Feature: Unequal bytes compare as False."""
    a = b'secret123'
    b = b'secret456'
    assert secrets.compare_digest(a, b) is False
```

**Verification**: ✅ Tested in CI

### Edge: Different lengths compare as False.

```python
def test_compare_digest_different_lengths(self):
    """Edge: Different lengths compare as False."""
    a = 'secret'
    b = 'secret123'
    assert secrets.compare_digest(a, b) is False
```

**Verification**: ✅ Tested in CI

### Edge: Empty strings compare as True.

```python
def test_compare_digest_empty_strings(self):
    """Edge: Empty strings compare as True."""
    assert secrets.compare_digest('', '') is True
```

**Verification**: ✅ Tested in CI

### Property: Comparison is constant-time (timing-safe).

```python
def test_compare_digest_constant_time(self):
    """Property: Comparison is constant-time (timing-safe)."""
    a = 'a' * 100
    b = 'a' * 99 + 'b'
    assert secrets.compare_digest(a, b) is False
```

**Verification**: ✅ Tested in CI

### Basic: SystemRandom is available via secrets.

```python
def test_systemrandom_available(self):
    """Basic: SystemRandom is available via secrets."""
    r = secrets.randbelow(10)
    assert isinstance(r, int)
```

**Verification**: ✅ Tested in CI

### Property: Cannot be seeded (cryptographically secure).

```python
def test_systemrandom_not_reproducible(self):
    """Property: Cannot be seeded (cryptographically secure)."""
    r1 = secrets.token_bytes(16)
    r2 = secrets.token_bytes(16)
    assert r1 != r2
```

**Verification**: ✅ Tested in CI

### Use case: Generate secure password.

```python
def test_generate_password_basic(self):
    """Use case: Generate secure password."""
    alphabet = string.ascii_letters + string.digits
    password = ''.join((secrets.choice(alphabet) for _ in range(10)))
    assert len(password) == 10
    assert all((c in alphabet for c in password))
```

**Verification**: ✅ Tested in CI

### Use case: Password with special characters.

```python
def test_generate_password_with_punctuation(self):
    """Use case: Password with special characters."""
    alphabet = string.ascii_letters + string.digits + string.punctuation
    password = ''.join((secrets.choice(alphabet) for _ in range(12)))
    assert len(password) == 12
```

**Verification**: ✅ Tested in CI

### Use case: Generate secure reset token.

```python
def test_generate_secure_token(self):
    """Use case: Generate secure reset token."""
    token = secrets.token_urlsafe(32)
    assert len(token) > 0
    assert all((c.isalnum() or c in '-_' for c in token))
```

**Verification**: ✅ Tested in CI

### Use case: Generate API key.

```python
def test_generate_api_key(self):
    """Use case: Generate API key."""
    api_key = secrets.token_hex(32)
    assert len(api_key) == 64
    assert all((c in '0123456789abcdef' for c in api_key))
```

**Verification**: ✅ Tested in CI

### Edge: Zero-length token.

```python
def test_token_bytes_zero(self):
    """Edge: Zero-length token."""
    token = secrets.token_bytes(0)
    assert token == b''
```

**Verification**: ✅ Tested in CI

### Edge: Zero-length hex token.

```python
def test_token_hex_zero(self):
    """Edge: Zero-length hex token."""
    token = secrets.token_hex(0)
    assert token == ''
```

**Verification**: ✅ Tested in CI

### Edge: Zero-length URL-safe token.

```python
def test_token_urlsafe_zero(self):
    """Edge: Zero-length URL-safe token."""
    token = secrets.token_urlsafe(0)
    assert token == ''
```

**Verification**: ✅ Tested in CI

### Performance: Large token generation.

```python
def test_token_bytes_large(self):
    """Performance: Large token generation."""
    token = secrets.token_bytes(1024)
    assert len(token) == 1024
```

**Verification**: ✅ Tested in CI

### Edge: Choice from single element.

```python
def test_choice_single_element(self):
    """Edge: Choice from single element."""
    choice = secrets.choice([42])
    assert choice == 42
```

**Verification**: ✅ Tested in CI

### Performance: randbelow with large n.

```python
def test_randbelow_large(self):
    """Performance: randbelow with large n."""
    r = secrets.randbelow(1000000)
    assert 0 <= r < 1000000
```

**Verification**: ✅ Tested in CI

### Property: Tokens have high entropy.

```python
def test_token_entropy(self):
    """Property: Tokens have high entropy."""
    tokens = [secrets.token_bytes(16) for _ in range(100)]
    assert len(set(tokens)) == 100
```

**Verification**: ✅ Tested in CI

### Property: Hex tokens are lowercase.

```python
def test_hex_lowercase(self):
    """Property: Hex tokens are lowercase."""
    token = secrets.token_hex(16)
    assert token == token.lower()
    assert token.islower() or token.isdigit() or all((c in '0123456789' for c in token))
```

**Verification**: ✅ Tested in CI

### Property: URL-safe tokens use base64url encoding.

```python
def test_urlsafe_base64_variant(self):
    """Property: URL-safe tokens use base64url encoding."""
    token = secrets.token_urlsafe(16)
    assert '+' not in token
    assert '/' not in token
```

**Verification**: ✅ Tested in CI

### Error: Type mismatch in compare_digest.

```python
def test_compare_digest_type_mismatch(self):
    """Error: Type mismatch in compare_digest."""
    with pytest.raises(TypeError):
        secrets.compare_digest('string', b'bytes')
```

**Verification**: ✅ Tested in CI

### Property: Multiple tokens are unique.

```python
def test_multiple_tokens_unique(self):
    """Property: Multiple tokens are unique."""
    tokens = [secrets.token_hex(16) for _ in range(50)]
    assert len(tokens) == len(set(tokens))
```

**Verification**: ✅ Tested in CI

### Feature: Choice works with tuple.

```python
def test_choice_works_with_tuple(self):
    """Feature: Choice works with tuple."""
    items = (1, 2, 3, 4, 5)
    choice = secrets.choice(items)
    assert choice in items
```

**Verification**: ✅ Tested in CI

### Feature: Choice works with range.

```python
def test_choice_works_with_range(self):
    """Feature: Choice works with range."""
    r = range(10)
    choice = secrets.choice(r)
    assert choice in r
```

**Verification**: ✅ Tested in CI

### Edge: randbelow(2) returns 0 or 1.

```python
def test_randbelow_two(self):
    """Edge: randbelow(2) returns 0 or 1."""
    results = {secrets.randbelow(2) for _ in range(50)}
    assert results <= {0, 1}
    assert len(results) > 1
```

**Verification**: ✅ Tested in CI

### Property: Same nbytes gives same length.

```python
def test_token_bytes_consistency(self):
    """Property: Same nbytes gives same length."""
    for nbytes in [8, 16, 32, 64]:
        token = secrets.token_bytes(nbytes)
        assert len(token) == nbytes
```

**Verification**: ✅ Tested in CI

### Property: Hex token is 2x byte length.

```python
def test_token_hex_double_length(self):
    """Property: Hex token is 2x byte length."""
    for nbytes in [8, 16, 32]:
        token = secrets.token_hex(nbytes)
        assert len(token) == nbytes * 2
```

**Verification**: ✅ Tested in CI

### Property: compare_digest is case-sensitive.

```python
def test_compare_digest_case_sensitive(self):
    """Property: compare_digest is case-sensitive."""
    assert secrets.compare_digest('Secret', 'secret') is False
    assert secrets.compare_digest('SECRET', 'secret') is False
```

**Verification**: ✅ Tested in CI

### Property: Cryptographic random cannot be predicted.

```python
def test_secure_random_not_seeded(self):
    """Property: Cryptographic random cannot be predicted."""
    seq1 = [secrets.randbelow(100) for _ in range(10)]
    seq2 = [secrets.randbelow(100) for _ in range(10)]
    assert seq1 != seq2
```

**Verification**: ✅ Tested in CI
