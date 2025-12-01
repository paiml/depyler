# memoryview

## memoryview() - Create memory views.

## Memoryview indexing and slicing.

## Memoryview conversion methods.

## Memoryview casting to different formats.

## Memoryview comparison operations.

## Memoryview release and context manager.

## Memoryview iteration and membership.

## Advanced slicing operations.

## Different format types.

## Edge cases and special scenarios.

### Basic: Create from bytes.

```python
def test_create_from_bytes(self):
    """Basic: Create from bytes."""
    data = b'hello'
    mv = memoryview(data)
    assert len(mv) == 5
    assert mv[0] == ord('h')
```

**Verification**: ✅ Tested in CI

### Basic: Create from bytearray.

```python
def test_create_from_bytearray(self):
    """Basic: Create from bytearray."""
    data = bytearray(b'world')
    mv = memoryview(data)
    assert len(mv) == 5
    assert mv[0] == ord('w')
```

**Verification**: ✅ Tested in CI

### Basic: Create from array.array.

```python
def test_create_from_array(self):
    """Basic: Create from array.array."""
    data = array.array('i', [1, 2, 3])
    mv = memoryview(data)
    assert len(mv) == 3
```

**Verification**: ✅ Tested in CI

### Property: bytes creates read-only view.

```python
def test_readonly_from_bytes(self):
    """Property: bytes creates read-only view."""
    data = b'test'
    mv = memoryview(data)
    assert mv.readonly is True
```

**Verification**: ✅ Tested in CI

### Property: bytearray creates writable view.

```python
def test_writable_from_bytearray(self):
    """Property: bytearray creates writable view."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.readonly is False
```

**Verification**: ✅ Tested in CI

### Property: obj references underlying object.

```python
def test_obj_attribute(self):
    """Property: obj references underlying object."""
    data = b'test'
    mv = memoryview(data)
    assert mv.obj is data
```

**Verification**: ✅ Tested in CI

### Property: format describes item format.

```python
def test_format_attribute(self):
    """Property: format describes item format."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.format == 'B'
```

**Verification**: ✅ Tested in CI

### Property: itemsize is bytes per item.

```python
def test_itemsize_attribute(self):
    """Property: itemsize is bytes per item."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.itemsize == 1
```

**Verification**: ✅ Tested in CI

### Property: ndim is number of dimensions.

```python
def test_ndim_attribute(self):
    """Property: ndim is number of dimensions."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.ndim == 1
```

**Verification**: ✅ Tested in CI

### Property: shape is tuple of dimensions.

```python
def test_shape_attribute(self):
    """Property: shape is tuple of dimensions."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.shape == (4,)
```

**Verification**: ✅ Tested in CI

### Property: strides is byte steps.

```python
def test_strides_attribute(self):
    """Property: strides is byte steps."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.strides == (1,)
```

**Verification**: ✅ Tested in CI

### Error: Non-buffer object raises TypeError.

```python
def test_error_non_buffer_object(self):
    """Error: Non-buffer object raises TypeError."""
    with pytest.raises(TypeError):
        memoryview('string')
```

**Verification**: ✅ Tested in CI

### Error: List raises TypeError.

```python
def test_error_non_buffer_list(self):
    """Error: List raises TypeError."""
    with pytest.raises(TypeError):
        memoryview([1, 2, 3])
```

**Verification**: ✅ Tested in CI

### Basic: Index access.

```python
def test_index_access(self):
    """Basic: Index access."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    assert mv[0] == ord('h')
    assert mv[4] == ord('o')
```

**Verification**: ✅ Tested in CI

### Feature: Negative indexing.

```python
def test_negative_index(self):
    """Feature: Negative indexing."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    assert mv[-1] == ord('o')
    assert mv[-5] == ord('h')
```

**Verification**: ✅ Tested in CI

### Feature: Slice access.

```python
def test_slice_access(self):
    """Feature: Slice access."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    sliced = mv[1:4]
    assert len(sliced) == 3
    assert bytes(sliced) == b'ell'
```

**Verification**: ✅ Tested in CI

### Property: Slice returns memoryview.

```python
def test_slice_returns_memoryview(self):
    """Property: Slice returns memoryview."""
    data = bytearray(b'test')
    mv = memoryview(data)
    sliced = mv[1:3]
    assert isinstance(sliced, memoryview)
```

**Verification**: ✅ Tested in CI

### Feature: Modify writable memoryview.

```python
def test_modify_writable_view(self):
    """Feature: Modify writable memoryview."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    mv[0] = ord('H')
    assert data[0] == ord('H')
    assert data == b'Hello'
```

**Verification**: ✅ Tested in CI

### Error: Cannot modify read-only view.

```python
def test_error_modify_readonly(self):
    """Error: Cannot modify read-only view."""
    data = b'hello'
    mv = memoryview(data)
    with pytest.raises(TypeError):
        mv[0] = ord('H')
```

**Verification**: ✅ Tested in CI

### Error: Index out of range.

```python
def test_error_index_out_of_range(self):
    """Error: Index out of range."""
    data = bytearray(b'test')
    mv = memoryview(data)
    with pytest.raises(IndexError):
        _ = mv[10]
```

**Verification**: ✅ Tested in CI

### Basic: Convert to bytes.

```python
def test_tobytes(self):
    """Basic: Convert to bytes."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    b = mv.tobytes()
    assert isinstance(b, bytes)
    assert b == b'hello'
```

**Verification**: ✅ Tested in CI

### Basic: Convert to list.

```python
def test_tolist(self):
    """Basic: Convert to list."""
    data = bytearray(b'abc')
    mv = memoryview(data)
    lst = mv.tolist()
    assert isinstance(lst, list)
    assert lst == [ord('a'), ord('b'), ord('c')]
```

**Verification**: ✅ Tested in CI

### Feature: Convert to hex string.

```python
def test_hex(self):
    """Feature: Convert to hex string."""
    data = bytearray(b'\x00\xff\xaa')
    mv = memoryview(data)
    h = mv.hex()
    assert h == '00ffaa'
```

**Verification**: ✅ Tested in CI

### Property: tobytes preserves data.

```python
def test_tobytes_preserves_data(self):
    """Property: tobytes preserves data."""
    original = bytearray(range(256))
    mv = memoryview(original)
    b = mv.tobytes()
    assert b == bytes(original)
```

**Verification**: ✅ Tested in CI

### Feature: tolist with array.

```python
def test_tolist_array(self):
    """Feature: tolist with array."""
    data = array.array('i', [1, 2, 3, 4])
    mv = memoryview(data)
    lst = mv.tolist()
    assert lst == [1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Cast byte view to integers.

```python
def test_cast_bytes_to_int(self):
    """Feature: Cast byte view to integers."""
    data = bytearray(b'\x01\x00\x00\x00\x02\x00\x00\x00')
    mv = memoryview(data)
    int_view = mv.cast('i')
    assert len(int_view) == 2
    assert int_view[0] == 1
    assert int_view[1] == 2
```

**Verification**: ✅ Tested in CI

### Feature: Cast int array to bytes.

```python
def test_cast_int_to_bytes(self):
    """Feature: Cast int array to bytes."""
    data = array.array('i', [1, 2, 3])
    mv = memoryview(data)
    byte_view = mv.cast('B')
    assert len(byte_view) == 12
```

**Verification**: ✅ Tested in CI

### Property: Cast changes itemsize.

```python
def test_cast_changes_itemsize(self):
    """Property: Cast changes itemsize."""
    data = bytearray(8)
    mv = memoryview(data)
    assert mv.itemsize == 1
    int_view = mv.cast('i')
    assert int_view.itemsize == 4
```

**Verification**: ✅ Tested in CI

### Property: Cast changes shape.

```python
def test_cast_changes_shape(self):
    """Property: Cast changes shape."""
    data = bytearray(8)
    mv = memoryview(data)
    assert mv.shape == (8,)
    int_view = mv.cast('i')
    assert int_view.shape == (2,)
```

**Verification**: ✅ Tested in CI

### Error: Cast with incompatible size.

```python
def test_error_cast_size_mismatch(self):
    """Error: Cast with incompatible size."""
    data = bytearray(5)
    mv = memoryview(data)
    with pytest.raises(TypeError):
        mv.cast('i')
```

**Verification**: ✅ Tested in CI

### Basic: Memoryview equality.

```python
def test_equality(self):
    """Basic: Memoryview equality."""
    data1 = bytearray(b'test')
    data2 = bytearray(b'test')
    mv1 = memoryview(data1)
    mv2 = memoryview(data2)
    assert mv1 == mv2
```

**Verification**: ✅ Tested in CI

### Basic: Memoryview inequality.

```python
def test_inequality(self):
    """Basic: Memoryview inequality."""
    data1 = bytearray(b'test')
    data2 = bytearray(b'best')
    mv1 = memoryview(data1)
    mv2 = memoryview(data2)
    assert mv1 != mv2
```

**Verification**: ✅ Tested in CI

### Feature: Compare with bytes.

```python
def test_compare_with_bytes(self):
    """Feature: Compare with bytes."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv == b'test'
```

**Verification**: ✅ Tested in CI

### Feature: Compare with bytearray.

```python
def test_compare_with_bytearray(self):
    """Feature: Compare with bytearray."""
    data = b'test'
    mv = memoryview(data)
    assert mv == bytearray(b'test')
```

**Verification**: ✅ Tested in CI

### Feature: Release memoryview.

```python
def test_release_method(self):
    """Feature: Release memoryview."""
    data = bytearray(b'test')
    mv = memoryview(data)
    mv.release()
    with pytest.raises(ValueError):
        _ = mv[0]
```

**Verification**: ✅ Tested in CI

### Feature: Use as context manager.

```python
def test_context_manager(self):
    """Feature: Use as context manager."""
    data = bytearray(b'test')
    with memoryview(data) as mv:
        assert mv[0] == ord('t')
    with pytest.raises(ValueError):
        _ = mv[0]
```

**Verification**: ✅ Tested in CI

### Edge: Double release is safe.

```python
def test_double_release(self):
    """Edge: Double release is safe."""
    data = bytearray(b'test')
    mv = memoryview(data)
    mv.release()
    mv.release()
```

**Verification**: ✅ Tested in CI

### Edge: Released views cannot be accessed.

```python
def test_released_equality_check(self):
    """Edge: Released views cannot be accessed."""
    data1 = bytearray(b'test')
    mv1 = memoryview(data1)
    mv1.release()
    with pytest.raises(ValueError):
        _ = len(mv1)
```

**Verification**: ✅ Tested in CI

### Basic: Iterate over memoryview.

```python
def test_iteration(self):
    """Basic: Iterate over memoryview."""
    data = bytearray(b'abc')
    mv = memoryview(data)
    result = list(mv)
    assert result == [ord('a'), ord('b'), ord('c')]
```

**Verification**: ✅ Tested in CI

### Feature: Iterate over int array view.

```python
def test_iteration_int_array(self):
    """Feature: Iterate over int array view."""
    data = array.array('i', [10, 20, 30])
    mv = memoryview(data)
    result = list(mv)
    assert result == [10, 20, 30]
```

**Verification**: ✅ Tested in CI

### Feature: Membership test.

```python
def test_membership(self):
    """Feature: Membership test."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    assert ord('h') in mv
    assert ord('z') not in mv
```

**Verification**: ✅ Tested in CI

### Basic: Length function.

```python
def test_len(self):
    """Basic: Length function."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    assert len(mv) == 5
```

**Verification**: ✅ Tested in CI

### Feature: Slice with step.

```python
def test_slice_with_step(self):
    """Feature: Slice with step."""
    data = bytearray(b'abcdefgh')
    mv = memoryview(data)
    sliced = mv[::2]
    assert bytes(sliced) == b'aceg'
```

**Verification**: ✅ Tested in CI

### Feature: Reverse slice.

```python
def test_reverse_slice(self):
    """Feature: Reverse slice."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    reversed_mv = mv[::-1]
    assert bytes(reversed_mv) == b'olleh'
```

**Verification**: ✅ Tested in CI

### Feature: Slice of a slice.

```python
def test_nested_slicing(self):
    """Feature: Slice of a slice."""
    data = bytearray(b'abcdefgh')
    mv = memoryview(data)
    sliced1 = mv[2:6]
    sliced2 = sliced1[1:3]
    assert bytes(sliced2) == b'de'
```

**Verification**: ✅ Tested in CI

### Feature: Modify through slice.

```python
def test_modify_through_slice(self):
    """Feature: Modify through slice."""
    data = bytearray(b'hello')
    mv = memoryview(data)
    sliced = mv[1:4]
    sliced[0] = ord('a')
    assert data == b'hallo'
```

**Verification**: ✅ Tested in CI

### Format 'B': Unsigned byte.

```python
def test_format_unsigned_byte(self):
    """Format 'B': Unsigned byte."""
    data = bytearray([0, 128, 255])
    mv = memoryview(data)
    assert mv.format == 'B'
    assert mv.itemsize == 1
```

**Verification**: ✅ Tested in CI

### Format 'i': Signed integer.

```python
def test_format_signed_int(self):
    """Format 'i': Signed integer."""
    data = array.array('i', [1, -1, 100])
    mv = memoryview(data)
    assert mv.format == 'i'
    assert mv.itemsize == 4
```

**Verification**: ✅ Tested in CI

### Format 'f': Float.

```python
def test_format_float(self):
    """Format 'f': Float."""
    data = array.array('f', [1.0, 2.5, 3.14])
    mv = memoryview(data)
    assert mv.format == 'f'
    assert mv.itemsize == 4
```

**Verification**: ✅ Tested in CI

### Format 'd': Double.

```python
def test_format_double(self):
    """Format 'd': Double."""
    data = array.array('d', [3.14159])
    mv = memoryview(data)
    assert mv.format == 'd'
    assert mv.itemsize == 8
```

**Verification**: ✅ Tested in CI

### Edge: Empty memoryview.

```python
def test_empty_memoryview(self):
    """Edge: Empty memoryview."""
    data = bytearray()
    mv = memoryview(data)
    assert len(mv) == 0
    assert mv.tobytes() == b''
```

**Verification**: ✅ Tested in CI

### Edge: Single byte view.

```python
def test_single_byte_view(self):
    """Edge: Single byte view."""
    data = bytearray(b'x')
    mv = memoryview(data)
    assert len(mv) == 1
    assert mv[0] == ord('x')
```

**Verification**: ✅ Tested in CI

### Performance: Large memoryview.

```python
def test_large_memoryview(self):
    """Performance: Large memoryview."""
    data = bytearray(10000)
    mv = memoryview(data)
    assert len(mv) == 10000
```

**Verification**: ✅ Tested in CI

### Property: bytes conversion roundtrip.

```python
def test_bytes_conversion_roundtrip(self):
    """Property: bytes conversion roundtrip."""
    original = bytearray(range(256))
    mv = memoryview(original)
    converted = bytearray(mv.tobytes())
    assert converted == original
```

**Verification**: ✅ Tested in CI

### Feature: Bool conversion.

```python
def test_bool_conversion(self):
    """Feature: Bool conversion."""
    empty = memoryview(bytearray())
    filled = memoryview(bytearray(b'x'))
    assert not empty
    assert filled
```

**Verification**: ✅ Tested in CI

### Edge: Writable memoryviews are not hashable.

```python
def test_hash_error(self):
    """Edge: Writable memoryviews are not hashable."""
    data = bytearray(b'test')
    mv = memoryview(data)
    with pytest.raises(ValueError, match='cannot hash writable memoryview'):
        hash(mv)
```

**Verification**: ✅ Tested in CI

### Property: c_contiguous and f_contiguous.

```python
def test_contiguous_check(self):
    """Property: c_contiguous and f_contiguous."""
    data = bytearray(b'test')
    mv = memoryview(data)
    assert mv.c_contiguous is True
    assert mv.f_contiguous is True
```

**Verification**: ✅ Tested in CI

### Property: Modifications visible through view.

```python
def test_shared_memory_modification(self):
    """Property: Modifications visible through view."""
    data = bytearray(b'hello')
    mv1 = memoryview(data)
    mv2 = memoryview(data)
    mv1[0] = ord('H')
    assert mv2[0] == ord('H')
    assert data[0] == ord('H')
```

**Verification**: ✅ Tested in CI

### Error: Zero step in slice.

```python
def test_zero_step_slice_error(self):
    """Error: Zero step in slice."""
    data = bytearray(b'test')
    mv = memoryview(data)
    with pytest.raises(ValueError):
        _ = mv[::0]
```

**Verification**: ✅ Tested in CI

### Property: Single index returns int.

```python
def test_getitem_returns_int(self):
    """Property: Single index returns int."""
    data = bytearray(b'test')
    mv = memoryview(data)
    item = mv[0]
    assert isinstance(item, int)
    assert item == ord('t')
```

**Verification**: ✅ Tested in CI
