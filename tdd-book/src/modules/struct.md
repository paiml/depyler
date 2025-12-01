# struct

## struct.pack() - Pack values into bytes.

## struct.unpack() - Unpack bytes into values.

## struct.calcsize() - Calculate format size.

## struct.pack_into() - Pack into existing buffer.

## struct.unpack_from() - Unpack from buffer at offset.

## struct.Struct() - Pre-compiled format object.

## Edge cases and special scenarios.

### Basic: Pack single integer.

```python
def test_pack_integer(self):
    """Basic: Pack single integer."""
    packed = struct.pack('i', 42)
    assert isinstance(packed, bytes)
    assert len(packed) == 4
```

**Verification**: ✅ Tested in CI

### Basic: Pack multiple values.

```python
def test_pack_multiple_values(self):
    """Basic: Pack multiple values."""
    packed = struct.pack('iif', 1, 2, 3.14)
    assert isinstance(packed, bytes)
    assert len(packed) == 12
```

**Verification**: ✅ Tested in CI

### Feature: Big-endian byte order.

```python
def test_pack_big_endian(self):
    """Feature: Big-endian byte order."""
    packed_big = struct.pack('>I', 305419896)
    assert packed_big[0] == 18
```

**Verification**: ✅ Tested in CI

### Feature: Little-endian byte order.

```python
def test_pack_little_endian(self):
    """Feature: Little-endian byte order."""
    packed_little = struct.pack('<I', 305419896)
    assert packed_little[0] == 120
```

**Verification**: ✅ Tested in CI

### Feature: Native byte order (@).

```python
def test_pack_native_endian(self):
    """Feature: Native byte order (@)."""
    packed = struct.pack('@I', 42)
    assert isinstance(packed, bytes)
    assert len(packed) >= 4
```

**Verification**: ✅ Tested in CI

### Basic: Pack fixed-length string.

```python
def test_pack_string(self):
    """Basic: Pack fixed-length string."""
    packed = struct.pack('5s', b'hello')
    assert packed == b'hello'
```

**Verification**: ✅ Tested in CI

### Edge: Short string gets null-padded.

```python
def test_pack_string_padded(self):
    """Edge: Short string gets null-padded."""
    packed = struct.pack('5s', b'hi')
    assert packed == b'hi\x00\x00\x00'
```

**Verification**: ✅ Tested in CI

### Basic: Pack single character.

```python
def test_pack_char(self):
    """Basic: Pack single character."""
    packed = struct.pack('c', b'A')
    assert packed == b'A'
```

**Verification**: ✅ Tested in CI

### Basic: Pack boolean values.

```python
def test_pack_boolean(self):
    """Basic: Pack boolean values."""
    packed_true = struct.pack('?', True)
    packed_false = struct.pack('?', False)
    assert packed_true == b'\x01'
    assert packed_false == b'\x00'
```

**Verification**: ✅ Tested in CI

### Basic: Pack float value.

```python
def test_pack_float(self):
    """Basic: Pack float value."""
    packed = struct.pack('f', 3.14)
    assert isinstance(packed, bytes)
    assert len(packed) == 4
```

**Verification**: ✅ Tested in CI

### Basic: Pack double precision float.

```python
def test_pack_double(self):
    """Basic: Pack double precision float."""
    packed = struct.pack('d', 3.14159265359)
    assert isinstance(packed, bytes)
    assert len(packed) == 8
```

**Verification**: ✅ Tested in CI

### Basic: Pack signed byte.

```python
def test_pack_signed_byte(self):
    """Basic: Pack signed byte."""
    packed = struct.pack('b', -128)
    assert isinstance(packed, bytes)
    assert len(packed) == 1
```

**Verification**: ✅ Tested in CI

### Basic: Pack unsigned byte.

```python
def test_pack_unsigned_byte(self):
    """Basic: Pack unsigned byte."""
    packed = struct.pack('B', 255)
    assert isinstance(packed, bytes)
    assert len(packed) == 1
```

**Verification**: ✅ Tested in CI

### Basic: Pack short integer.

```python
def test_pack_short(self):
    """Basic: Pack short integer."""
    packed = struct.pack('h', 32767)
    assert isinstance(packed, bytes)
    assert len(packed) == 2
```

**Verification**: ✅ Tested in CI

### Basic: Pack long long integer.

```python
def test_pack_long_long(self):
    """Basic: Pack long long integer."""
    packed = struct.pack('q', 9223372036854775807)
    assert isinstance(packed, bytes)
    assert len(packed) == 8
```

**Verification**: ✅ Tested in CI

### Error: Wrong value type raises struct.error.

```python
def test_pack_error_wrong_type(self):
    """Error: Wrong value type raises struct.error."""
    with pytest.raises(struct.error):
        struct.pack('i', 'not an int')
```

**Verification**: ✅ Tested in CI

### Error: Too many arguments raises struct.error.

```python
def test_pack_error_too_many_args(self):
    """Error: Too many arguments raises struct.error."""
    with pytest.raises(struct.error):
        struct.pack('i', 1, 2)
```

**Verification**: ✅ Tested in CI

### Error: Too few arguments raises struct.error.

```python
def test_pack_error_too_few_args(self):
    """Error: Too few arguments raises struct.error."""
    with pytest.raises(struct.error):
        struct.pack('ii', 1)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack single integer.

```python
def test_unpack_integer(self):
    """Basic: Unpack single integer."""
    packed = struct.pack('i', 42)
    unpacked = struct.unpack('i', packed)
    assert unpacked == (42,)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack multiple values.

```python
def test_unpack_multiple_values(self):
    """Basic: Unpack multiple values."""
    packed = struct.pack('iif', 1, 2, 3.14)
    unpacked = struct.unpack('iif', packed)
    assert len(unpacked) == 3
    assert unpacked[0] == 1
    assert unpacked[1] == 2
    assert abs(unpacked[2] - 3.14) < 0.01
```

**Verification**: ✅ Tested in CI

### Feature: Unpack big-endian.

```python
def test_unpack_big_endian(self):
    """Feature: Unpack big-endian."""
    packed = b'\x124Vx'
    unpacked = struct.unpack('>I', packed)
    assert unpacked == (305419896,)
```

**Verification**: ✅ Tested in CI

### Feature: Unpack little-endian.

```python
def test_unpack_little_endian(self):
    """Feature: Unpack little-endian."""
    packed = b'xV4\x12'
    unpacked = struct.unpack('<I', packed)
    assert unpacked == (305419896,)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack fixed-length string.

```python
def test_unpack_string(self):
    """Basic: Unpack fixed-length string."""
    packed = b'hello'
    unpacked = struct.unpack('5s', packed)
    assert unpacked == (b'hello',)
```

**Verification**: ✅ Tested in CI

### Edge: Unpack string containing nulls.

```python
def test_unpack_string_with_nulls(self):
    """Edge: Unpack string containing nulls."""
    packed = b'hi\x00\x00\x00'
    unpacked = struct.unpack('5s', packed)
    assert unpacked == (b'hi\x00\x00\x00',)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack boolean values.

```python
def test_unpack_boolean(self):
    """Basic: Unpack boolean values."""
    unpacked_true = struct.unpack('?', b'\x01')
    unpacked_false = struct.unpack('?', b'\x00')
    assert unpacked_true == (True,)
    assert unpacked_false == (False,)
```

**Verification**: ✅ Tested in CI

### Property: Float pack/unpack roundtrip.

```python
def test_unpack_float_roundtrip(self):
    """Property: Float pack/unpack roundtrip."""
    original = 3.14
    packed = struct.pack('f', original)
    unpacked = struct.unpack('f', packed)
    assert abs(unpacked[0] - original) < 0.0001
```

**Verification**: ✅ Tested in CI

### Property: Double pack/unpack roundtrip.

```python
def test_unpack_double_roundtrip(self):
    """Property: Double pack/unpack roundtrip."""
    original = 3.14159265359
    packed = struct.pack('d', original)
    unpacked = struct.unpack('d', packed)
    assert abs(unpacked[0] - original) < 1e-10
```

**Verification**: ✅ Tested in CI

### Error: Wrong buffer size raises struct.error.

```python
def test_unpack_error_wrong_size(self):
    """Error: Wrong buffer size raises struct.error."""
    with pytest.raises(struct.error):
        struct.unpack('i', b'\x01\x02')
```

**Verification**: ✅ Tested in CI

### Error: Empty buffer with format raises struct.error.

```python
def test_unpack_error_empty_buffer(self):
    """Error: Empty buffer with format raises struct.error."""
    with pytest.raises(struct.error):
        struct.unpack('i', b'')
```

**Verification**: ✅ Tested in CI

### Basic: Size of single integer.

```python
def test_calcsize_single_int(self):
    """Basic: Size of single integer."""
    size = struct.calcsize('i')
    assert size == 4
```

**Verification**: ✅ Tested in CI

### Basic: Size of multiple values.

```python
def test_calcsize_multiple_values(self):
    """Basic: Size of multiple values."""
    size = struct.calcsize('iif')
    assert size == 12
```

**Verification**: ✅ Tested in CI

### Basic: Size of char.

```python
def test_calcsize_char(self):
    """Basic: Size of char."""
    size = struct.calcsize('c')
    assert size == 1
```

**Verification**: ✅ Tested in CI

### Basic: Size of fixed string.

```python
def test_calcsize_string(self):
    """Basic: Size of fixed string."""
    size = struct.calcsize('10s')
    assert size == 10
```

**Verification**: ✅ Tested in CI

### Basic: Size of boolean.

```python
def test_calcsize_boolean(self):
    """Basic: Size of boolean."""
    size = struct.calcsize('?')
    assert size == 1
```

**Verification**: ✅ Tested in CI

### Basic: Size of double.

```python
def test_calcsize_double(self):
    """Basic: Size of double."""
    size = struct.calcsize('d')
    assert size == 8
```

**Verification**: ✅ Tested in CI

### Basic: Size of long long.

```python
def test_calcsize_long_long(self):
    """Basic: Size of long long."""
    size = struct.calcsize('q')
    assert size == 8
```

**Verification**: ✅ Tested in CI

### Feature: Native format includes alignment.

```python
def test_calcsize_alignment_native(self):
    """Feature: Native format includes alignment."""
    size = struct.calcsize('@ci')
    assert size >= 5
```

**Verification**: ✅ Tested in CI

### Feature: Standard format has no alignment.

```python
def test_calcsize_standard_no_alignment(self):
    """Feature: Standard format has no alignment."""
    size = struct.calcsize('=ci')
    assert size == 5
```

**Verification**: ✅ Tested in CI

### Edge: Empty format has size 0.

```python
def test_calcsize_empty_format(self):
    """Edge: Empty format has size 0."""
    size = struct.calcsize('')
    assert size == 0
```

**Verification**: ✅ Tested in CI

### Basic: Pack into bytearray.

```python
def test_pack_into_basic(self):
    """Basic: Pack into bytearray."""
    buffer = bytearray(8)
    struct.pack_into('i', buffer, 0, 42)
    assert buffer[4:] == b'\x00\x00\x00\x00'
```

**Verification**: ✅ Tested in CI

### Feature: Pack at specific offset.

```python
def test_pack_into_with_offset(self):
    """Feature: Pack at specific offset."""
    buffer = bytearray(12)
    struct.pack_into('i', buffer, 4, 42)
    assert buffer[:4] == b'\x00\x00\x00\x00'
    assert buffer[4:8] != b'\x00\x00\x00\x00'
```

**Verification**: ✅ Tested in CI

### Basic: Pack multiple values into buffer.

```python
def test_pack_into_multiple_values(self):
    """Basic: Pack multiple values into buffer."""
    buffer = bytearray(12)
    struct.pack_into('iii', buffer, 0, 1, 2, 3)
    unpacked = struct.unpack_from('iii', buffer, 0)
    assert unpacked == (1, 2, 3)
```

**Verification**: ✅ Tested in CI

### Error: Buffer too small raises struct.error.

```python
def test_pack_into_error_buffer_too_small(self):
    """Error: Buffer too small raises struct.error."""
    buffer = bytearray(2)
    with pytest.raises(struct.error):
        struct.pack_into('i', buffer, 0, 42)
```

**Verification**: ✅ Tested in CI

### Error: Offset too large raises struct.error.

```python
def test_pack_into_error_offset_too_large(self):
    """Error: Offset too large raises struct.error."""
    buffer = bytearray(8)
    with pytest.raises(struct.error):
        struct.pack_into('i', buffer, 6, 42)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack from buffer start.

```python
def test_unpack_from_basic(self):
    """Basic: Unpack from buffer start."""
    buffer = struct.pack('iii', 1, 2, 3)
    unpacked = struct.unpack_from('i', buffer, 0)
    assert unpacked == (1,)
```

**Verification**: ✅ Tested in CI

### Feature: Unpack from specific offset.

```python
def test_unpack_from_with_offset(self):
    """Feature: Unpack from specific offset."""
    buffer = struct.pack('iii', 1, 2, 3)
    unpacked = struct.unpack_from('i', buffer, 4)
    assert unpacked == (2,)
```

**Verification**: ✅ Tested in CI

### Basic: Unpack multiple values from offset.

```python
def test_unpack_from_multiple_values(self):
    """Basic: Unpack multiple values from offset."""
    buffer = struct.pack('iiii', 1, 2, 3, 4)
    unpacked = struct.unpack_from('ii', buffer, 4)
    assert unpacked == (2, 3)
```

**Verification**: ✅ Tested in CI

### Feature: Default offset is 0.

```python
def test_unpack_from_default_offset(self):
    """Feature: Default offset is 0."""
    buffer = struct.pack('i', 42)
    unpacked = struct.unpack_from('i', buffer)
    assert unpacked == (42,)
```

**Verification**: ✅ Tested in CI

### Error: Offset too large raises struct.error.

```python
def test_unpack_from_error_offset_too_large(self):
    """Error: Offset too large raises struct.error."""
    buffer = struct.pack('ii', 1, 2)
    with pytest.raises(struct.error):
        struct.unpack_from('i', buffer, 6)
```

**Verification**: ✅ Tested in CI

### Basic: Struct.pack() packs values.

```python
def test_struct_class_pack(self):
    """Basic: Struct.pack() packs values."""
    s = struct.Struct('ii')
    packed = s.pack(1, 2)
    assert isinstance(packed, bytes)
    assert len(packed) == 8
```

**Verification**: ✅ Tested in CI

### Basic: Struct.unpack() unpacks values.

```python
def test_struct_class_unpack(self):
    """Basic: Struct.unpack() unpacks values."""
    s = struct.Struct('ii')
    packed = s.pack(1, 2)
    unpacked = s.unpack(packed)
    assert unpacked == (1, 2)
```

**Verification**: ✅ Tested in CI

### Property: Struct.size matches calcsize().

```python
def test_struct_class_size_attribute(self):
    """Property: Struct.size matches calcsize()."""
    s = struct.Struct('iif')
    assert s.size == struct.calcsize('iif')
```

**Verification**: ✅ Tested in CI

### Property: Struct.format preserves format string.

```python
def test_struct_class_format_attribute(self):
    """Property: Struct.format preserves format string."""
    format_str = 'iif'
    s = struct.Struct(format_str)
    assert s.format == format_str
```

**Verification**: ✅ Tested in CI

### Feature: Struct.pack_into() works.

```python
def test_struct_class_pack_into(self):
    """Feature: Struct.pack_into() works."""
    s = struct.Struct('i')
    buffer = bytearray(4)
    s.pack_into(buffer, 0, 42)
    unpacked = s.unpack_from(buffer, 0)
    assert unpacked == (42,)
```

**Verification**: ✅ Tested in CI

### Feature: Struct.unpack_from() works.

```python
def test_struct_class_unpack_from(self):
    """Feature: Struct.unpack_from() works."""
    s = struct.Struct('ii')
    buffer = struct.pack('iii', 1, 2, 3)
    unpacked = s.unpack_from(buffer, 4)
    assert unpacked == (2, 3)
```

**Verification**: ✅ Tested in CI

### Edge: Pack zero values.

```python
def test_zero_value_packing(self):
    """Edge: Pack zero values."""
    packed = struct.pack('iii', 0, 0, 0)
    unpacked = struct.unpack('iii', packed)
    assert unpacked == (0, 0, 0)
```

**Verification**: ✅ Tested in CI

### Edge: Pack negative integers.

```python
def test_negative_values(self):
    """Edge: Pack negative integers."""
    packed = struct.pack('i', -42)
    unpacked = struct.unpack('i', packed)
    assert unpacked == (-42,)
```

**Verification**: ✅ Tested in CI

### Edge: Pack maximum values for types.

```python
def test_max_values(self):
    """Edge: Pack maximum values for types."""
    packed = struct.pack('i', 2147483647)
    unpacked = struct.unpack('i', packed)
    assert unpacked == (2147483647,)
```

**Verification**: ✅ Tested in CI

### Edge: Pack minimum values for types.

```python
def test_min_values(self):
    """Edge: Pack minimum values for types."""
    packed = struct.pack('i', -2147483648)
    unpacked = struct.unpack('i', packed)
    assert unpacked == (-2147483648,)
```

**Verification**: ✅ Tested in CI

### Edge: Unsigned overflow raises error.

```python
def test_unsigned_overflow_error(self):
    """Edge: Unsigned overflow raises error."""
    with pytest.raises(struct.error):
        struct.pack('B', 256)
```

**Verification**: ✅ Tested in CI

### Feature: Padding bytes (x).

```python
def test_padding_bytes(self):
    """Feature: Padding bytes (x)."""
    packed = struct.pack('=ixxi', 1, 2)
    assert len(packed) == 10
```

**Verification**: ✅ Tested in CI

### Feature: Repeat count in format.

```python
def test_repeat_count(self):
    """Feature: Repeat count in format."""
    packed = struct.pack('3i', 1, 2, 3)
    assert len(packed) == 12
```

**Verification**: ✅ Tested in CI

### Edge: Pack empty string.

```python
def test_empty_string_packing(self):
    """Edge: Pack empty string."""
    packed = struct.pack('0s', b'')
    assert packed == b''
```

**Verification**: ✅ Tested in CI

### Feature: Network (big-endian) byte order.

```python
def test_network_byte_order(self):
    """Feature: Network (big-endian) byte order."""
    packed = struct.pack('!I', 305419896)
    assert packed[0] == 18
```

**Verification**: ✅ Tested in CI
