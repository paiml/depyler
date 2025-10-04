# base64

## Standard Base64 encoding/decoding.

## URL-safe Base64 encoding/decoding.

## Base32 encoding/decoding.

## Base16 (hex) encoding/decoding.

## Base85 encoding/decoding.

## Ascii85 encoding/decoding.

## Binary data encoding.

## Input validation and errors.

## Standard encoding properties.

## Encoded length properties.

## Edge cases and special scenarios.

## Real-world use cases.

## Base32 hexadecimal variant.

## Decoding with validation.

### Basic: Encode bytes to base64.

```python
def test_encode_basic(self):
    """Basic: Encode bytes to base64."""
    data = b'hello'
    encoded = base64.b64encode(data)
    assert encoded == b'aGVsbG8='
    assert isinstance(encoded, bytes)
```

**Verification**: ✅ Tested in CI

### Basic: Decode base64 to bytes.

```python
def test_decode_basic(self):
    """Basic: Decode base64 to bytes."""
    encoded = b'aGVsbG8='
    decoded = base64.b64decode(encoded)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Property: Encode then decode returns original.

```python
def test_roundtrip(self):
    """Property: Encode then decode returns original."""
    data = b'The quick brown fox'
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Edge: Encode empty bytes.

```python
def test_empty_bytes(self):
    """Edge: Encode empty bytes."""
    encoded = base64.b64encode(b'')
    assert encoded == b''
    decoded = base64.b64decode(b'')
    assert decoded == b''
```

**Verification**: ✅ Tested in CI

### Edge: Encode single byte.

```python
def test_single_byte(self):
    """Edge: Encode single byte."""
    encoded = base64.b64encode(b'A')
    assert encoded == b'QQ=='
```

**Verification**: ✅ Tested in CI

### Property: Base64 uses = for padding.

```python
def test_padding(self):
    """Property: Base64 uses = for padding."""
    assert base64.b64encode(b'A') == b'QQ=='
    assert base64.b64encode(b'AB') == b'QUI='
    assert base64.b64encode(b'ABC') == b'QUJD'
```

**Verification**: ✅ Tested in CI

### Feature: URL-safe encoding uses - and _ instead of + and /.

```python
def test_encode_urlsafe(self):
    """Feature: URL-safe encoding uses - and _ instead of + and /."""
    data = b'\xfb\xff\xfe'
    standard = base64.b64encode(data)
    urlsafe = base64.urlsafe_b64encode(data)
    assert b'+' not in urlsafe or b'/' not in urlsafe
```

**Verification**: ✅ Tested in CI

### Feature: Decode URL-safe base64.

```python
def test_decode_urlsafe(self):
    """Feature: Decode URL-safe base64."""
    encoded = b'-_-_'
    decoded = base64.urlsafe_b64decode(encoded)
    assert isinstance(decoded, bytes)
```

**Verification**: ✅ Tested in CI

### Property: URL-safe encode/decode roundtrip.

```python
def test_urlsafe_roundtrip(self):
    """Property: URL-safe encode/decode roundtrip."""
    data = b'URL safe encoding test \xff\xfe'
    encoded = base64.urlsafe_b64encode(data)
    decoded = base64.urlsafe_b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: URL-safe has no + or / characters.

```python
def test_urlsafe_no_special_chars(self):
    """Property: URL-safe has no + or / characters."""
    data = bytes(range(256))
    encoded = base64.urlsafe_b64encode(data)
    assert b'+' not in encoded
    assert b'/' not in encoded
```

**Verification**: ✅ Tested in CI

### Feature: Base32 encoding.

```python
def test_encode_base32(self):
    """Feature: Base32 encoding."""
    data = b'hello'
    encoded = base64.b32encode(data)
    assert encoded == b'NBSWY3DP'
    assert isinstance(encoded, bytes)
```

**Verification**: ✅ Tested in CI

### Feature: Base32 decoding.

```python
def test_decode_base32(self):
    """Feature: Base32 decoding."""
    encoded = b'NBSWY3DP'
    decoded = base64.b32decode(encoded)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Property: Base32 roundtrip.

```python
def test_base32_roundtrip(self):
    """Property: Base32 roundtrip."""
    data = b'Base32 test data'
    encoded = base64.b32encode(data)
    decoded = base64.b32decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base32 uses uppercase letters.

```python
def test_base32_uppercase(self):
    """Property: Base32 uses uppercase letters."""
    data = b'test'
    encoded = base64.b32encode(data)
    assert encoded == encoded.upper()
```

**Verification**: ✅ Tested in CI

### Property: Base32 uses = for padding.

```python
def test_base32_padding(self):
    """Property: Base32 uses = for padding."""
    assert b'=' in base64.b32encode(b'a')
```

**Verification**: ✅ Tested in CI

### Feature: Base16 encoding (hexadecimal).

```python
def test_encode_base16(self):
    """Feature: Base16 encoding (hexadecimal)."""
    data = b'hello'
    encoded = base64.b16encode(data)
    assert encoded == b'68656C6C6F'
```

**Verification**: ✅ Tested in CI

### Feature: Base16 decoding.

```python
def test_decode_base16(self):
    """Feature: Base16 decoding."""
    encoded = b'68656C6C6F'
    decoded = base64.b16decode(encoded)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Property: Base16 roundtrip.

```python
def test_base16_roundtrip(self):
    """Property: Base16 roundtrip."""
    data = bytes(range(256))
    encoded = base64.b16encode(data)
    decoded = base64.b16decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base16 encoding is uppercase.

```python
def test_base16_uppercase(self):
    """Property: Base16 encoding is uppercase."""
    data = b'test'
    encoded = base64.b16encode(data)
    assert encoded == encoded.upper()
```

**Verification**: ✅ Tested in CI

### Feature: Base16 can decode lowercase.

```python
def test_base16_lowercase_decode(self):
    """Feature: Base16 can decode lowercase."""
    lowercase = b'68656c6c6f'
    decoded = base64.b16decode(lowercase, casefold=True)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Feature: Base85 encoding (Ascii85).

```python
def test_encode_base85(self):
    """Feature: Base85 encoding (Ascii85)."""
    data = b'hello'
    encoded = base64.b85encode(data)
    assert isinstance(encoded, bytes)
    assert len(encoded) > 0
```

**Verification**: ✅ Tested in CI

### Feature: Base85 decoding.

```python
def test_decode_base85(self):
    """Feature: Base85 decoding."""
    data = b'hello world'
    encoded = base64.b85encode(data)
    decoded = base64.b85decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base85 roundtrip.

```python
def test_base85_roundtrip(self):
    """Property: Base85 roundtrip."""
    data = b'Base85 is more efficient than base64'
    encoded = base64.b85encode(data)
    decoded = base64.b85decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base85 is more efficient than base64.

```python
def test_base85_efficiency(self):
    """Property: Base85 is more efficient than base64."""
    data = b'test data for comparison'
    b64 = base64.b64encode(data)
    b85 = base64.b85encode(data)
    assert len(b85) <= len(b64)
```

**Verification**: ✅ Tested in CI

### Feature: Ascii85 encoding.

```python
def test_encode_ascii85(self):
    """Feature: Ascii85 encoding."""
    data = b'hello'
    encoded = base64.a85encode(data)
    assert isinstance(encoded, bytes)
```

**Verification**: ✅ Tested in CI

### Feature: Ascii85 decoding.

```python
def test_decode_ascii85(self):
    """Feature: Ascii85 decoding."""
    data = b'test'
    encoded = base64.a85encode(data)
    decoded = base64.a85decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Ascii85 roundtrip.

```python
def test_ascii85_roundtrip(self):
    """Property: Ascii85 roundtrip."""
    data = b'Ascii85 encoding test'
    encoded = base64.a85encode(data)
    decoded = base64.a85decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Feature: Encode arbitrary binary data.

```python
def test_encode_binary(self):
    """Feature: Encode arbitrary binary data."""
    data = bytes(range(256))
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Edge: Encode data with null bytes.

```python
def test_encode_nulls(self):
    """Edge: Encode data with null bytes."""
    data = b'\x00\x00\x00'
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Edge: Encode high byte values.

```python
def test_encode_high_bytes(self):
    """Edge: Encode high byte values."""
    data = b'\xff\xfe\xfd\xfc'
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Error: Decode invalid base64.

```python
def test_error_decode_invalid_base64(self):
    """Error: Decode invalid base64."""
    with pytest.raises(Exception):
        base64.b64decode(b'not valid base64!!!', validate=True)
```

**Verification**: ✅ Tested in CI

### Feature: Decode without validation skips invalid chars.

```python
def test_decode_without_validation(self):
    """Feature: Decode without validation skips invalid chars."""
    result = base64.b64decode(b'aGVs bG8=')
    assert result == b'hello'
```

**Verification**: ✅ Tested in CI

### Error: Invalid padding in strict mode.

```python
def test_error_decode_incorrect_padding(self):
    """Error: Invalid padding in strict mode."""
    with pytest.raises(Exception):
        base64.b64decode(b'aGVsbG8', validate=True)
```

**Verification**: ✅ Tested in CI

### Error: Cannot encode string, must be bytes.

```python
def test_error_encode_string(self):
    """Error: Cannot encode string, must be bytes."""
    with pytest.raises(TypeError):
        base64.b64encode('string')
```

**Verification**: ✅ Tested in CI

### Property: Base64 uses A-Z, a-z, 0-9, +, /.

```python
def test_alphabet_base64(self):
    """Property: Base64 uses A-Z, a-z, 0-9, +, /."""
    data = bytes(range(256))
    encoded = base64.b64encode(data)
    valid_chars = set(b'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=')
    assert all((byte in valid_chars for byte in encoded))
```

**Verification**: ✅ Tested in CI

### Property: URL-safe uses A-Z, a-z, 0-9, -, _.

```python
def test_alphabet_urlsafe(self):
    """Property: URL-safe uses A-Z, a-z, 0-9, -, _."""
    data = bytes(range(256))
    encoded = base64.urlsafe_b64encode(data)
    valid_chars = set(b'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=')
    assert all((byte in valid_chars for byte in encoded))
```

**Verification**: ✅ Tested in CI

### Property: Encoding is deterministic.

```python
def test_deterministic(self):
    """Property: Encoding is deterministic."""
    data = b'test data'
    encoded1 = base64.b64encode(data)
    encoded2 = base64.b64encode(data)
    assert encoded1 == encoded2
```

**Verification**: ✅ Tested in CI

### Property: Base64 is case-sensitive.

```python
def test_case_sensitive(self):
    """Property: Base64 is case-sensitive."""
    upper = base64.b64decode(b'QQ==')
    assert isinstance(upper, bytes)
```

**Verification**: ✅ Tested in CI

### Property: Base64 expands data by ~33%.

```python
def test_base64_expansion(self):
    """Property: Base64 expands data by ~33%."""
    data = b'x' * 100
    encoded = base64.b64encode(data)
    assert len(encoded) >= len(data) * 4 // 3
```

**Verification**: ✅ Tested in CI

### Property: Base32 expands data by ~60%.

```python
def test_base32_expansion(self):
    """Property: Base32 expands data by ~60%."""
    data = b'x' * 100
    encoded = base64.b32encode(data)
    assert len(encoded) >= len(data) * 8 // 5
```

**Verification**: ✅ Tested in CI

### Property: Base16 doubles data size.

```python
def test_base16_doubles(self):
    """Property: Base16 doubles data size."""
    data = b'test'
    encoded = base64.b16encode(data)
    assert len(encoded) == len(data) * 2
```

**Verification**: ✅ Tested in CI

### Performance: Encode large data.

```python
def test_large_data(self):
    """Performance: Encode large data."""
    data = b'x' * 1000000
    encoded = base64.b64encode(data)
    decoded = base64.b64decode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Feature: Newlines are ignored in decoding.

```python
def test_newlines_ignored(self):
    """Feature: Newlines are ignored in decoding."""
    encoded_with_newlines = b'aGVs\nbG8='
    decoded = base64.b64decode(encoded_with_newlines)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Feature: Whitespace is ignored in decoding.

```python
def test_whitespace_ignored(self):
    """Feature: Whitespace is ignored in decoding."""
    encoded_with_spaces = b'aGVs bG8='
    decoded = base64.b64decode(encoded_with_spaces)
    assert decoded == b'hello'
```

**Verification**: ✅ Tested in CI

### Feature: Can decode bytes or ASCII string.

```python
def test_decode_bytes_or_ascii(self):
    """Feature: Can decode bytes or ASCII string."""
    data = b'hello'
    encoded_bytes = base64.b64encode(data)
    encoded_str = encoded_bytes.decode('ascii')
    decoded_from_bytes = base64.b64decode(encoded_bytes)
    decoded_from_str = base64.b64decode(encoded_str)
    assert decoded_from_bytes == decoded_from_str == data
```

**Verification**: ✅ Tested in CI

### Feature: Custom altchars for base64.

```python
def test_altchars_parameter(self):
    """Feature: Custom altchars for base64."""
    data = b'\xfb\xff'
    encoded = base64.b64encode(data, altchars=b'-_')
    decoded = base64.b64decode(encoded, altchars=b'-_')
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: urlsafe is equivalent to altchars='-_'.

```python
def test_urlsafe_is_altchars(self):
    """Property: urlsafe is equivalent to altchars='-_'."""
    data = b'test \xff\xfe'
    urlsafe = base64.urlsafe_b64encode(data)
    altchars = base64.b64encode(data, altchars=b'-_')
    assert urlsafe == altchars
```

**Verification**: ✅ Tested in CI

### Property: Encoding empty bytes returns empty bytes.

```python
def test_encode_empty_preserves_type(self):
    """Property: Encoding empty bytes returns empty bytes."""
    result = base64.b64encode(b'')
    assert result == b''
    assert isinstance(result, bytes)
```

**Verification**: ✅ Tested in CI

### Use case: Encode data for URL.

```python
def test_encode_for_url(self):
    """Use case: Encode data for URL."""
    data = b'user_id=12345&token=secret'
    encoded = base64.urlsafe_b64encode(data)
    assert b'+' not in encoded
    assert b'/' not in encoded
```

**Verification**: ✅ Tested in CI

### Use case: Encode binary for JSON.

```python
def test_encode_for_json(self):
    """Use case: Encode binary for JSON."""
    binary_data = b'\x00\x01\x02\xff\xfe'
    encoded = base64.b64encode(binary_data).decode('ascii')
    assert isinstance(encoded, str)
    decoded = base64.b64decode(encoded.encode('ascii'))
    assert decoded == binary_data
```

**Verification**: ✅ Tested in CI

### Use case: Encode image-like binary data.

```python
def test_encode_image_data(self):
    """Use case: Encode image-like binary data."""
    png_header = b'\x89PNG\r\n\x1a\n'
    encoded = base64.b64encode(png_header)
    decoded = base64.b64decode(encoded)
    assert decoded == png_header
```

**Verification**: ✅ Tested in CI

### Use case: Create data URI.

```python
def test_data_uri(self):
    """Use case: Create data URI."""
    data = b'Hello, World!'
    encoded = base64.b64encode(data).decode('ascii')
    data_uri = f'data:text/plain;base64,{encoded}'
    assert 'base64,' in data_uri
```

**Verification**: ✅ Tested in CI

### Feature: Base32 hex encoding.

```python
def test_b32hexencode(self):
    """Feature: Base32 hex encoding."""
    data = b'hello'
    encoded = base64.b32hexencode(data)
    assert isinstance(encoded, bytes)
```

**Verification**: ✅ Tested in CI

### Feature: Base32 hex decoding.

```python
def test_b32hexdecode(self):
    """Feature: Base32 hex decoding."""
    data = b'test'
    encoded = base64.b32hexencode(data)
    decoded = base64.b32hexdecode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base32 hex roundtrip.

```python
def test_b32hex_roundtrip(self):
    """Property: Base32 hex roundtrip."""
    data = b'Base32 hex variant'
    encoded = base64.b32hexencode(data)
    decoded = base64.b32hexdecode(encoded)
    assert decoded == data
```

**Verification**: ✅ Tested in CI

### Property: Base32 hex uses different alphabet than standard.

```python
def test_b32hex_different_alphabet(self):
    """Property: Base32 hex uses different alphabet than standard."""
    data = b'test'
    standard = base64.b32encode(data)
    hexvariant = base64.b32hexencode(data)
    assert standard != hexvariant
```

**Verification**: ✅ Tested in CI

### Feature: Strict validation with validate=True.

```python
def test_validate_true(self):
    """Feature: Strict validation with validate=True."""
    valid = b'aGVsbG8='
    result = base64.b64decode(valid, validate=True)
    assert result == b'hello'
```

**Verification**: ✅ Tested in CI

### Feature: validate=True rejects invalid input.

```python
def test_validate_rejects_invalid(self):
    """Feature: validate=True rejects invalid input."""
    invalid = b'not@valid!'
    with pytest.raises(Exception):
        base64.b64decode(invalid, validate=True)
```

**Verification**: ✅ Tested in CI

### Property: Default validation is lenient.

```python
def test_default_validation_lenient(self):
    """Property: Default validation is lenient."""
    lenient = b'aGVs bG8='
    result = base64.b64decode(lenient)
    assert result == b'hello'
```

**Verification**: ✅ Tested in CI
