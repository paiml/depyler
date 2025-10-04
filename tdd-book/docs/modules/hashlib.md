# hashlib

## Basic cryptographic hash functions.

## Incremental hash updates.

## Different hash algorithms.

## Hash object copying.

## Hash function properties.

## Hash object attributes.

## BLAKE2 hash functions with parameters.

## Available algorithms.

## Edge cases and special scenarios.

## Use cases for password hashing.

## Use cases for file hashing.

## Scrypt key derivation function.

### Basic: MD5 hash.

```python
def test_md5_basic(self):
    """Basic: MD5 hash."""
    h = hashlib.md5(b'hello')
    assert len(h.digest()) == 16
    assert isinstance(h.hexdigest(), str)
    assert len(h.hexdigest()) == 32
```

**Verification**: ✅ Tested in CI

### Basic: SHA-1 hash.

```python
def test_sha1_basic(self):
    """Basic: SHA-1 hash."""
    h = hashlib.sha1(b'hello')
    assert len(h.digest()) == 20
    assert len(h.hexdigest()) == 40
```

**Verification**: ✅ Tested in CI

### Basic: SHA-256 hash.

```python
def test_sha256_basic(self):
    """Basic: SHA-256 hash."""
    h = hashlib.sha256(b'hello')
    assert len(h.digest()) == 32
    assert len(h.hexdigest()) == 64
```

**Verification**: ✅ Tested in CI

### Basic: SHA-512 hash.

```python
def test_sha512_basic(self):
    """Basic: SHA-512 hash."""
    h = hashlib.sha512(b'hello')
    assert len(h.digest()) == 64
    assert len(h.hexdigest()) == 128
```

**Verification**: ✅ Tested in CI

### Feature: digest() returns bytes, hexdigest() returns hex string.

```python
def test_digest_vs_hexdigest(self):
    """Feature: digest() returns bytes, hexdigest() returns hex string."""
    h = hashlib.sha256(b'test')
    digest = h.digest()
    hexdigest = h.hexdigest()
    assert isinstance(digest, bytes)
    assert isinstance(hexdigest, str)
    assert hexdigest == digest.hex()
```

**Verification**: ✅ Tested in CI

### Basic: Update hash with data.

```python
def test_update_single(self):
    """Basic: Update hash with data."""
    h = hashlib.sha256()
    h.update(b'hello')
    result1 = h.hexdigest()
    h2 = hashlib.sha256(b'hello')
    result2 = h2.hexdigest()
    assert result1 == result2
```

**Verification**: ✅ Tested in CI

### Feature: Multiple updates concatenate.

```python
def test_update_multiple(self):
    """Feature: Multiple updates concatenate."""
    h1 = hashlib.sha256()
    h1.update(b'hello')
    h1.update(b'world')
    h2 = hashlib.sha256(b'helloworld')
    assert h1.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Property: Incremental hashing equals one-shot.

```python
def test_update_incremental(self):
    """Property: Incremental hashing equals one-shot."""
    data = b'The quick brown fox jumps over the lazy dog'
    h1 = hashlib.sha256(data)
    h2 = hashlib.sha256()
    for byte in data:
        h2.update(bytes([byte]))
    assert h1.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Edge: Update with empty bytes.

```python
def test_update_empty(self):
    """Edge: Update with empty bytes."""
    h = hashlib.sha256(b'test')
    digest1 = h.hexdigest()
    h.update(b'')
    digest2 = h.hexdigest()
    assert digest1 == digest2
```

**Verification**: ✅ Tested in CI

### Feature: SHA-224 hash.

```python
def test_sha224(self):
    """Feature: SHA-224 hash."""
    h = hashlib.sha224(b'test')
    assert len(h.digest()) == 28
```

**Verification**: ✅ Tested in CI

### Feature: SHA-384 hash.

```python
def test_sha384(self):
    """Feature: SHA-384 hash."""
    h = hashlib.sha384(b'test')
    assert len(h.digest()) == 48
```

**Verification**: ✅ Tested in CI

### Feature: SHA3-256 hash.

```python
def test_sha3_256(self):
    """Feature: SHA3-256 hash."""
    h = hashlib.sha3_256(b'test')
    assert len(h.digest()) == 32
```

**Verification**: ✅ Tested in CI

### Feature: SHA3-512 hash.

```python
def test_sha3_512(self):
    """Feature: SHA3-512 hash."""
    h = hashlib.sha3_512(b'test')
    assert len(h.digest()) == 64
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2b hash.

```python
def test_blake2b(self):
    """Feature: BLAKE2b hash."""
    h = hashlib.blake2b(b'test')
    assert len(h.digest()) == 64
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2s hash.

```python
def test_blake2s(self):
    """Feature: BLAKE2s hash."""
    h = hashlib.blake2s(b'test')
    assert len(h.digest()) == 32
```

**Verification**: ✅ Tested in CI

### Feature: SHAKE128 variable-length hash.

```python
def test_shake_128(self):
    """Feature: SHAKE128 variable-length hash."""
    h = hashlib.shake_128(b'test')
    digest1 = h.hexdigest(16)
    assert len(digest1) == 32
    h2 = hashlib.shake_128(b'test')
    digest2 = h2.hexdigest(32)
    assert len(digest2) == 64
```

**Verification**: ✅ Tested in CI

### Feature: SHAKE256 variable-length hash.

```python
def test_shake_256(self):
    """Feature: SHAKE256 variable-length hash."""
    h = hashlib.shake_256(b'test')
    digest = h.hexdigest(64)
    assert len(digest) == 128
```

**Verification**: ✅ Tested in CI

### Basic: Copy preserves hash state.

```python
def test_copy_preserves_state(self):
    """Basic: Copy preserves hash state."""
    h1 = hashlib.sha256(b'hello')
    h2 = h1.copy()
    assert h1.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Property: Copies are independent.

```python
def test_copy_independent(self):
    """Property: Copies are independent."""
    h1 = hashlib.sha256(b'hello')
    h2 = h1.copy()
    h1.update(b'world')
    h2.update(b'python')
    assert h1.hexdigest() != h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Use case: Branch hash computation.

```python
def test_copy_before_finalize(self):
    """Use case: Branch hash computation."""
    h = hashlib.sha256(b'prefix')
    h1 = h.copy()
    h1.update(b'suffix1')
    h2 = h.copy()
    h2.update(b'suffix2')
    assert h1.hexdigest() != h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Property: Same input produces same hash.

```python
def test_deterministic(self):
    """Property: Same input produces same hash."""
    data = b'test data'
    h1 = hashlib.sha256(data).hexdigest()
    h2 = hashlib.sha256(data).hexdigest()
    assert h1 == h2
```

**Verification**: ✅ Tested in CI

### Edge: Hash of empty bytes.

```python
def test_empty_string_hash(self):
    """Edge: Hash of empty bytes."""
    h = hashlib.sha256(b'')
    assert h.hexdigest() == 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
```

**Verification**: ✅ Tested in CI

### Property: Different inputs produce different hashes.

```python
def test_different_input_different_hash(self):
    """Property: Different inputs produce different hashes."""
    h1 = hashlib.sha256(b'test1').hexdigest()
    h2 = hashlib.sha256(b'test2').hexdigest()
    assert h1 != h2
```

**Verification**: ✅ Tested in CI

### Property: Small input change produces completely different hash.

```python
def test_small_change_different_hash(self):
    """Property: Small input change produces completely different hash."""
    h1 = hashlib.sha256(b'test').hexdigest()
    h2 = hashlib.sha256(b'Test').hexdigest()
    assert h1 != h2
```

**Verification**: ✅ Tested in CI

### Property: Hash length is consistent for algorithm.

```python
def test_hash_length_consistent(self):
    """Property: Hash length is consistent for algorithm."""
    h1 = hashlib.sha256(b'short').hexdigest()
    h2 = hashlib.sha256(b'a' * 10000).hexdigest()
    assert len(h1) == len(h2) == 64
```

**Verification**: ✅ Tested in CI

### Feature: Hash object has name attribute.

```python
def test_name_attribute(self):
    """Feature: Hash object has name attribute."""
    h = hashlib.sha256()
    assert h.name == 'sha256'
```

**Verification**: ✅ Tested in CI

### Feature: Hash object has digest_size attribute.

```python
def test_digest_size(self):
    """Feature: Hash object has digest_size attribute."""
    assert hashlib.md5().digest_size == 16
    assert hashlib.sha1().digest_size == 20
    assert hashlib.sha256().digest_size == 32
    assert hashlib.sha512().digest_size == 64
```

**Verification**: ✅ Tested in CI

### Feature: Hash object has block_size attribute.

```python
def test_block_size(self):
    """Feature: Hash object has block_size attribute."""
    h = hashlib.sha256()
    assert hasattr(h, 'block_size')
    assert h.block_size > 0
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2b custom digest size.

```python
def test_blake2b_digest_size(self):
    """Feature: BLAKE2b custom digest size."""
    h = hashlib.blake2b(b'test', digest_size=32)
    assert len(h.digest()) == 32
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2b keyed hashing.

```python
def test_blake2b_key(self):
    """Feature: BLAKE2b keyed hashing."""
    key = b'secret key'
    h1 = hashlib.blake2b(b'message', key=key)
    h2 = hashlib.blake2b(b'message', key=key)
    assert h1.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Property: Different keys produce different hashes.

```python
def test_blake2b_different_keys(self):
    """Property: Different keys produce different hashes."""
    h1 = hashlib.blake2b(b'message', key=b'key1')
    h2 = hashlib.blake2b(b'message', key=b'key2')
    assert h1.hexdigest() != h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2s custom digest size.

```python
def test_blake2s_digest_size(self):
    """Feature: BLAKE2s custom digest size."""
    h = hashlib.blake2s(b'test', digest_size=16)
    assert len(h.digest()) == 16
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2b with salt.

```python
def test_blake2b_salt(self):
    """Feature: BLAKE2b with salt."""
    salt = b'random salt'
    h = hashlib.blake2b(b'message', salt=salt)
    assert len(h.digest()) == 64
```

**Verification**: ✅ Tested in CI

### Feature: BLAKE2b with personalization.

```python
def test_blake2b_person(self):
    """Feature: BLAKE2b with personalization."""
    person = b'my app'
    h = hashlib.blake2b(b'message', person=person)
    assert len(h.digest()) == 64
```

**Verification**: ✅ Tested in CI

### Feature: Guaranteed algorithms are available.

```python
def test_algorithms_guaranteed(self):
    """Feature: Guaranteed algorithms are available."""
    guaranteed = {'md5', 'sha1', 'sha224', 'sha256', 'sha384', 'sha512'}
    for algo in guaranteed:
        assert algo in hashlib.algorithms_guaranteed
```

**Verification**: ✅ Tested in CI

### Feature: Check available algorithms.

```python
def test_algorithms_available(self):
    """Feature: Check available algorithms."""
    assert 'sha256' in hashlib.algorithms_available
```

**Verification**: ✅ Tested in CI

### Feature: Create hash using new() with algorithm name.

```python
def test_new_with_algorithm_name(self):
    """Feature: Create hash using new() with algorithm name."""
    h1 = hashlib.new('sha256', b'test')
    h2 = hashlib.sha256(b'test')
    assert h1.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Performance: Hash large data.

```python
def test_large_data_hash(self):
    """Performance: Hash large data."""
    data = b'x' * 1000000
    h = hashlib.sha256(data)
    assert len(h.hexdigest()) == 64
```

**Verification**: ✅ Tested in CI

### Property: Can call digest() multiple times.

```python
def test_hash_after_digest(self):
    """Property: Can call digest() multiple times."""
    h = hashlib.sha256(b'test')
    digest1 = h.hexdigest()
    digest2 = h.hexdigest()
    assert digest1 == digest2
```

**Verification**: ✅ Tested in CI

### Property: Can update after calling digest().

```python
def test_update_after_digest(self):
    """Property: Can update after calling digest()."""
    h = hashlib.sha256(b'test')
    digest1 = h.hexdigest()
    h.update(b'more')
    digest2 = h.hexdigest()
    assert digest1 != digest2
```

**Verification**: ✅ Tested in CI

### Property: hexdigest() returns lowercase.

```python
def test_hex_lowercase(self):
    """Property: hexdigest() returns lowercase."""
    h = hashlib.sha256(b'TEST')
    hexdigest = h.hexdigest()
    assert hexdigest == hexdigest.lower()
```

**Verification**: ✅ Tested in CI

### Feature: Hash binary data.

```python
def test_binary_data(self):
    """Feature: Hash binary data."""
    data = bytes(range(256))
    h = hashlib.sha256(data)
    assert len(h.digest()) == 32
```

**Verification**: ✅ Tested in CI

### Error: Cannot hash string directly.

```python
def test_unicode_requires_encoding(self):
    """Error: Cannot hash string directly."""
    with pytest.raises(TypeError):
        hashlib.sha256('string')
```

**Verification**: ✅ Tested in CI

### Feature: SHAKE requires length parameter.

```python
def test_shake_requires_length(self):
    """Feature: SHAKE requires length parameter."""
    h = hashlib.shake_128(b'test')
    digest = h.digest(16)
    assert len(digest) == 16
```

**Verification**: ✅ Tested in CI

### Edge: BLAKE2 digest size limits.

```python
def test_blake2_max_digest_size(self):
    """Edge: BLAKE2 digest size limits."""
    h = hashlib.blake2b(b'test', digest_size=64)
    assert len(h.digest()) == 64
    h2 = hashlib.blake2s(b'test', digest_size=32)
    assert len(h2.digest()) == 32
```

**Verification**: ✅ Tested in CI

### Error: BLAKE2 digest size too large.

```python
def test_error_blake2_digest_size_too_large(self):
    """Error: BLAKE2 digest size too large."""
    with pytest.raises(ValueError):
        hashlib.blake2b(digest_size=65)
```

**Verification**: ✅ Tested in CI

### Error: BLAKE2s digest size too large.

```python
def test_error_blake2s_digest_size_too_large(self):
    """Error: BLAKE2s digest size too large."""
    with pytest.raises(ValueError):
        hashlib.blake2s(digest_size=33)
```

**Verification**: ✅ Tested in CI

### Property: Hash is consistent across multiple objects.

```python
def test_hash_consistency_across_calls(self):
    """Property: Hash is consistent across multiple objects."""
    data = b'consistency test'
    hashes = [hashlib.sha256(data).hexdigest() for _ in range(10)]
    assert len(set(hashes)) == 1
```

**Verification**: ✅ Tested in CI

### Property: Different messages produce different MD5 hashes.

```python
def test_md5_collision_resistance(self):
    """Property: Different messages produce different MD5 hashes."""
    h1 = hashlib.md5(b'message1').hexdigest()
    h2 = hashlib.md5(b'message2').hexdigest()
    assert h1 != h2
```

**Verification**: ✅ Tested in CI

### Use case: Common SHA-256 hash.

```python
def test_sha256_common_hash(self):
    """Use case: Common SHA-256 hash."""
    h = hashlib.sha256(b'hello world')
    expected = 'b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9'
    assert h.hexdigest() == expected
```

**Verification**: ✅ Tested in CI

### Use case: PBKDF2 key derivation.

```python
def test_pbkdf2_hmac_basic(self):
    """Use case: PBKDF2 key derivation."""
    password = b'mypassword'
    salt = b'salt1234'
    key = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
    assert len(key) == 32
```

**Verification**: ✅ Tested in CI

### Property: Different passwords produce different keys.

```python
def test_pbkdf2_hmac_different_passwords(self):
    """Property: Different passwords produce different keys."""
    salt = b'salt'
    key1 = hashlib.pbkdf2_hmac('sha256', b'pass1', salt, 100000)
    key2 = hashlib.pbkdf2_hmac('sha256', b'pass2', salt, 100000)
    assert key1 != key2
```

**Verification**: ✅ Tested in CI

### Property: Different salts produce different keys.

```python
def test_pbkdf2_hmac_different_salts(self):
    """Property: Different salts produce different keys."""
    password = b'password'
    key1 = hashlib.pbkdf2_hmac('sha256', password, b'salt1', 100000)
    key2 = hashlib.pbkdf2_hmac('sha256', password, b'salt2', 100000)
    assert key1 != key2
```

**Verification**: ✅ Tested in CI

### Property: Different iterations produce different keys.

```python
def test_pbkdf2_hmac_iterations(self):
    """Property: Different iterations produce different keys."""
    password = b'password'
    salt = b'salt'
    key1 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
    key2 = hashlib.pbkdf2_hmac('sha256', password, salt, 200000)
    assert key1 != key2
```

**Verification**: ✅ Tested in CI

### Property: Same parameters produce same key.

```python
def test_pbkdf2_hmac_reproducible(self):
    """Property: Same parameters produce same key."""
    password = b'password'
    salt = b'salt'
    key1 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
    key2 = hashlib.pbkdf2_hmac('sha256', password, salt, 100000)
    assert key1 == key2
```

**Verification**: ✅ Tested in CI

### Use case: Hash file-like data in chunks.

```python
def test_hash_file_simulation(self):
    """Use case: Hash file-like data in chunks."""
    data = b'a' * 10000
    chunk_size = 1024
    h = hashlib.sha256()
    for i in range(0, len(data), chunk_size):
        chunk = data[i:i + chunk_size]
        h.update(chunk)
    h2 = hashlib.sha256(data)
    assert h.hexdigest() == h2.hexdigest()
```

**Verification**: ✅ Tested in CI

### Feature: scrypt key derivation.

```python
def test_scrypt_basic(self):
    """Feature: scrypt key derivation."""
    password = b'password'
    salt = b'salt'
    key = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
    assert len(key) == 64
```

**Verification**: ✅ Tested in CI

### Feature: scrypt with custom key length.

```python
def test_scrypt_custom_length(self):
    """Feature: scrypt with custom key length."""
    password = b'password'
    salt = b'salt'
    key = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1, dklen=32)
    assert len(key) == 32
```

**Verification**: ✅ Tested in CI

### Property: scrypt is reproducible.

```python
def test_scrypt_reproducible(self):
    """Property: scrypt is reproducible."""
    password = b'password'
    salt = b'salt'
    key1 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
    key2 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
    assert key1 == key2
```

**Verification**: ✅ Tested in CI

### Property: Different n parameter produces different keys.

```python
def test_scrypt_different_n(self):
    """Property: Different n parameter produces different keys."""
    password = b'password'
    salt = b'salt'
    key1 = hashlib.scrypt(password, salt=salt, n=16, r=8, p=1)
    key2 = hashlib.scrypt(password, salt=salt, n=32, r=8, p=1)
    assert key1 != key2
```

**Verification**: ✅ Tested in CI
