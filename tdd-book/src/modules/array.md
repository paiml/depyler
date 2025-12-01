# array

## array.array() - Create typed arrays.

## Array indexing and slicing.

## Array modification methods.

## Array conversion methods.

## Array comparison operations.

## Array iteration and membership.

## Array concatenation operations.

## Test all array typecodes.

## Edge cases and special scenarios.

### Basic: Create integer array.

```python
def test_create_integer_array(self):
    """Basic: Create integer array."""
    arr = array.array('i', [1, 2, 3])
    assert len(arr) == 3
    assert arr[0] == 1
```

**Verification**: ✅ Tested in CI

### Basic: Create float array.

```python
def test_create_float_array(self):
    """Basic: Create float array."""
    arr = array.array('f', [1.0, 2.5, 3.14])
    assert len(arr) == 3
    assert abs(arr[1] - 2.5) < 0.001
```

**Verification**: ✅ Tested in CI

### Basic: Create empty array.

```python
def test_create_empty_array(self):
    """Basic: Create empty array."""
    arr = array.array('i')
    assert len(arr) == 0
```

**Verification**: ✅ Tested in CI

### Feature: Create from any iterable.

```python
def test_create_from_iterable(self):
    """Feature: Create from any iterable."""
    arr = array.array('i', range(5))
    assert len(arr) == 5
    assert arr[4] == 4
```

**Verification**: ✅ Tested in CI

### Property: typecode attribute.

```python
def test_typecode_attribute(self):
    """Property: typecode attribute."""
    arr = array.array('f')
    assert arr.typecode == 'f'
```

**Verification**: ✅ Tested in CI

### Property: itemsize matches type.

```python
def test_itemsize_attribute(self):
    """Property: itemsize matches type."""
    arr_int = array.array('i')
    arr_float = array.array('d')
    assert arr_int.itemsize == 4
    assert arr_float.itemsize == 8
```

**Verification**: ✅ Tested in CI

### Basic: Create signed byte array.

```python
def test_create_signed_byte_array(self):
    """Basic: Create signed byte array."""
    arr = array.array('b', [-128, 0, 127])
    assert arr[0] == -128
    assert arr[2] == 127
```

**Verification**: ✅ Tested in CI

### Basic: Create unsigned byte array.

```python
def test_create_unsigned_byte_array(self):
    """Basic: Create unsigned byte array."""
    arr = array.array('B', [0, 128, 255])
    assert arr[1] == 128
    assert arr[2] == 255
```

**Verification**: ✅ Tested in CI

### Basic: Create double precision array.

```python
def test_create_double_array(self):
    """Basic: Create double precision array."""
    arr = array.array('d', [3.14159265359])
    assert abs(arr[0] - 3.14159265359) < 1e-10
```

**Verification**: ✅ Tested in CI

### Error: Wrong value type raises TypeError.

```python
def test_error_wrong_type(self):
    """Error: Wrong value type raises TypeError."""
    with pytest.raises(TypeError):
        array.array('i', [1, 2, 'three'])
```

**Verification**: ✅ Tested in CI

### Error: Invalid typecode raises ValueError.

```python
def test_error_invalid_typecode(self):
    """Error: Invalid typecode raises ValueError."""
    with pytest.raises(ValueError):
        array.array('x', [1, 2, 3])
```

**Verification**: ✅ Tested in CI

### Error: Value overflow raises OverflowError.

```python
def test_error_overflow(self):
    """Error: Value overflow raises OverflowError."""
    with pytest.raises(OverflowError):
        array.array('b', [128])
```

**Verification**: ✅ Tested in CI

### Basic: Index access.

```python
def test_index_access(self):
    """Basic: Index access."""
    arr = array.array('i', [10, 20, 30])
    assert arr[0] == 10
    assert arr[2] == 30
```

**Verification**: ✅ Tested in CI

### Feature: Negative indexing.

```python
def test_negative_index(self):
    """Feature: Negative indexing."""
    arr = array.array('i', [10, 20, 30])
    assert arr[-1] == 30
    assert arr[-2] == 20
```

**Verification**: ✅ Tested in CI

### Basic: Modify via index.

```python
def test_index_assignment(self):
    """Basic: Modify via index."""
    arr = array.array('i', [1, 2, 3])
    arr[1] = 99
    assert arr[1] == 99
```

**Verification**: ✅ Tested in CI

### Feature: Slice access.

```python
def test_slice_access(self):
    """Feature: Slice access."""
    arr = array.array('i', [10, 20, 30, 40, 50])
    sliced = arr[1:4]
    assert len(sliced) == 3
    assert sliced[0] == 20
```

**Verification**: ✅ Tested in CI

### Feature: Slice assignment.

```python
def test_slice_assignment(self):
    """Feature: Slice assignment."""
    arr = array.array('i', [1, 2, 3, 4, 5])
    arr[1:3] = array.array('i', [99, 88])
    assert arr[1] == 99
    assert arr[2] == 88
```

**Verification**: ✅ Tested in CI

### Error: Index out of range.

```python
def test_error_index_out_of_range(self):
    """Error: Index out of range."""
    arr = array.array('i', [1, 2, 3])
    with pytest.raises(IndexError):
        _ = arr[10]
```

**Verification**: ✅ Tested in CI

### Basic: Append element.

```python
def test_append(self):
    """Basic: Append element."""
    arr = array.array('i', [1, 2])
    arr.append(3)
    assert len(arr) == 3
    assert arr[2] == 3
```

**Verification**: ✅ Tested in CI

### Basic: Extend with iterable.

```python
def test_extend(self):
    """Basic: Extend with iterable."""
    arr = array.array('i', [1, 2])
    arr.extend([3, 4, 5])
    assert len(arr) == 5
    assert arr[4] == 5
```

**Verification**: ✅ Tested in CI

### Feature: Extend with another array.

```python
def test_extend_with_array(self):
    """Feature: Extend with another array."""
    arr1 = array.array('i', [1, 2])
    arr2 = array.array('i', [3, 4])
    arr1.extend(arr2)
    assert len(arr1) == 4
```

**Verification**: ✅ Tested in CI

### Basic: Insert at position.

```python
def test_insert(self):
    """Basic: Insert at position."""
    arr = array.array('i', [1, 2, 3])
    arr.insert(1, 99)
    assert arr[1] == 99
    assert len(arr) == 4
```

**Verification**: ✅ Tested in CI

### Basic: Pop last element.

```python
def test_pop_default(self):
    """Basic: Pop last element."""
    arr = array.array('i', [1, 2, 3])
    val = arr.pop()
    assert val == 3
    assert len(arr) == 2
```

**Verification**: ✅ Tested in CI

### Feature: Pop at specific index.

```python
def test_pop_at_index(self):
    """Feature: Pop at specific index."""
    arr = array.array('i', [10, 20, 30])
    val = arr.pop(1)
    assert val == 20
    assert len(arr) == 2
    assert arr[1] == 30
```

**Verification**: ✅ Tested in CI

### Basic: Remove first occurrence.

```python
def test_remove(self):
    """Basic: Remove first occurrence."""
    arr = array.array('i', [1, 2, 3, 2])
    arr.remove(2)
    assert len(arr) == 3
    assert arr[1] == 3
```

**Verification**: ✅ Tested in CI

### Error: Remove non-existent value.

```python
def test_remove_error_not_found(self):
    """Error: Remove non-existent value."""
    arr = array.array('i', [1, 2, 3])
    with pytest.raises(ValueError):
        arr.remove(99)
```

**Verification**: ✅ Tested in CI

### Basic: Find index of value.

```python
def test_index_method(self):
    """Basic: Find index of value."""
    arr = array.array('i', [10, 20, 30])
    idx = arr.index(20)
    assert idx == 1
```

**Verification**: ✅ Tested in CI

### Error: Index of non-existent value.

```python
def test_index_error_not_found(self):
    """Error: Index of non-existent value."""
    arr = array.array('i', [1, 2, 3])
    with pytest.raises(ValueError):
        arr.index(99)
```

**Verification**: ✅ Tested in CI

### Basic: Count occurrences.

```python
def test_count(self):
    """Basic: Count occurrences."""
    arr = array.array('i', [1, 2, 2, 3, 2])
    assert arr.count(2) == 3
    assert arr.count(1) == 1
```

**Verification**: ✅ Tested in CI

### Basic: Reverse in place.

```python
def test_reverse(self):
    """Basic: Reverse in place."""
    arr = array.array('i', [1, 2, 3])
    arr.reverse()
    assert arr[0] == 3
    assert arr[2] == 1
```

**Verification**: ✅ Tested in CI

### Feature: Delete all elements via slice.

```python
def test_del_all_elements(self):
    """Feature: Delete all elements via slice."""
    arr = array.array('i', [1, 2, 3])
    del arr[:]
    assert len(arr) == 0
```

**Verification**: ✅ Tested in CI

### Basic: Convert to list.

```python
def test_tolist(self):
    """Basic: Convert to list."""
    arr = array.array('i', [1, 2, 3])
    lst = arr.tolist()
    assert isinstance(lst, list)
    assert lst == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Basic: Initialize from list.

```python
def test_fromlist(self):
    """Basic: Initialize from list."""
    arr = array.array('i')
    arr.fromlist([10, 20, 30])
    assert len(arr) == 3
    assert arr[1] == 20
```

**Verification**: ✅ Tested in CI

### Basic: Convert to bytes.

```python
def test_tobytes(self):
    """Basic: Convert to bytes."""
    arr = array.array('i', [1, 2])
    b = arr.tobytes()
    assert isinstance(b, bytes)
    assert len(b) == 8
```

**Verification**: ✅ Tested in CI

### Basic: Initialize from bytes.

```python
def test_frombytes(self):
    """Basic: Initialize from bytes."""
    arr = array.array('i', [1, 2])
    b = arr.tobytes()
    arr2 = array.array('i')
    arr2.frombytes(b)
    assert arr2.tolist() == [1, 2]
```

**Verification**: ✅ Tested in CI

### Property: Bytes roundtrip preserves data.

```python
def test_roundtrip_bytes(self):
    """Property: Bytes roundtrip preserves data."""
    original = array.array('d', [3.14, 2.71, 1.41])
    b = original.tobytes()
    recovered = array.array('d')
    recovered.frombytes(b)
    for i in range(len(original)):
        assert abs(original[i] - recovered[i]) < 1e-10
```

**Verification**: ✅ Tested in CI

### Feature: Get buffer info.

```python
def test_buffer_info(self):
    """Feature: Get buffer info."""
    arr = array.array('i', [1, 2, 3])
    addr, length = arr.buffer_info()
    assert isinstance(addr, int)
    assert length == 3
```

**Verification**: ✅ Tested in CI

### Basic: Array equality.

```python
def test_equality(self):
    """Basic: Array equality."""
    arr1 = array.array('i', [1, 2, 3])
    arr2 = array.array('i', [1, 2, 3])
    assert arr1 == arr2
```

**Verification**: ✅ Tested in CI

### Basic: Array inequality.

```python
def test_inequality(self):
    """Basic: Array inequality."""
    arr1 = array.array('i', [1, 2, 3])
    arr2 = array.array('i', [1, 2, 4])
    assert arr1 != arr2
```

**Verification**: ✅ Tested in CI

### Feature: Lexicographic comparison.

```python
def test_less_than(self):
    """Feature: Lexicographic comparison."""
    arr1 = array.array('i', [1, 2])
    arr2 = array.array('i', [1, 3])
    assert arr1 < arr2
```

**Verification**: ✅ Tested in CI

### Edge: Different typecodes with same values are equal.

```python
def test_equality_different_types(self):
    """Edge: Different typecodes with same values are equal."""
    arr1 = array.array('i', [1, 2])
    arr2 = array.array('f', [1.0, 2.0])
    assert arr1 == arr2
```

**Verification**: ✅ Tested in CI

### Basic: Iterate over array.

```python
def test_iteration(self):
    """Basic: Iterate over array."""
    arr = array.array('i', [10, 20, 30])
    result = []
    for val in arr:
        result.append(val)
    assert result == [10, 20, 30]
```

**Verification**: ✅ Tested in CI

### Feature: Membership test.

```python
def test_membership(self):
    """Feature: Membership test."""
    arr = array.array('i', [1, 2, 3])
    assert 2 in arr
    assert 99 not in arr
```

**Verification**: ✅ Tested in CI

### Basic: Length function.

```python
def test_len(self):
    """Basic: Length function."""
    arr = array.array('i', [1, 2, 3, 4, 5])
    assert len(arr) == 5
```

**Verification**: ✅ Tested in CI

### Feature: Concatenate with +.

```python
def test_concatenation(self):
    """Feature: Concatenate with +."""
    arr1 = array.array('i', [1, 2])
    arr2 = array.array('i', [3, 4])
    result = arr1 + arr2
    assert len(result) == 4
    assert result.tolist() == [1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Repeat with *.

```python
def test_repetition(self):
    """Feature: Repeat with *."""
    arr = array.array('i', [1, 2])
    result = arr * 3
    assert len(result) == 6
    assert result.tolist() == [1, 2, 1, 2, 1, 2]
```

**Verification**: ✅ Tested in CI

### Feature: In-place concatenation.

```python
def test_in_place_concatenation(self):
    """Feature: In-place concatenation."""
    arr = array.array('i', [1, 2])
    arr += array.array('i', [3, 4])
    assert len(arr) == 4
```

**Verification**: ✅ Tested in CI

### Feature: In-place repetition.

```python
def test_in_place_repetition(self):
    """Feature: In-place repetition."""
    arr = array.array('i', [1, 2])
    arr *= 2
    assert len(arr) == 4
    assert arr.tolist() == [1, 2, 1, 2]
```

**Verification**: ✅ Tested in CI

### Error: Cannot concatenate different typecodes.

```python
def test_error_concatenate_different_types(self):
    """Error: Cannot concatenate different typecodes."""
    arr1 = array.array('i', [1, 2])
    arr2 = array.array('f', [1.0, 2.0])
    with pytest.raises(TypeError):
        _ = arr1 + arr2
```

**Verification**: ✅ Tested in CI

### Typecode 'b': signed char (-128 to 127).

```python
def test_signed_char(self):
    """Typecode 'b': signed char (-128 to 127)."""
    arr = array.array('b', [-128, 0, 127])
    assert arr.itemsize == 1
    assert arr[0] == -128
```

**Verification**: ✅ Tested in CI

### Typecode 'B': unsigned char (0 to 255).

```python
def test_unsigned_char(self):
    """Typecode 'B': unsigned char (0 to 255)."""
    arr = array.array('B', [0, 128, 255])
    assert arr.itemsize == 1
    assert arr[2] == 255
```

**Verification**: ✅ Tested in CI

### Typecode 'h': signed short.

```python
def test_signed_short(self):
    """Typecode 'h': signed short."""
    arr = array.array('h', [-32768, 0, 32767])
    assert arr.itemsize == 2
```

**Verification**: ✅ Tested in CI

### Typecode 'H': unsigned short.

```python
def test_unsigned_short(self):
    """Typecode 'H': unsigned short."""
    arr = array.array('H', [0, 32768, 65535])
    assert arr.itemsize == 2
```

**Verification**: ✅ Tested in CI

### Typecode 'i': signed int.

```python
def test_signed_int(self):
    """Typecode 'i': signed int."""
    arr = array.array('i', [-2147483648, 0, 2147483647])
    assert arr.itemsize == 4
```

**Verification**: ✅ Tested in CI

### Typecode 'I': unsigned int.

```python
def test_unsigned_int(self):
    """Typecode 'I': unsigned int."""
    arr = array.array('I', [0, 2147483648, 4294967295])
    assert arr.itemsize == 4
```

**Verification**: ✅ Tested in CI

### Typecode 'l': signed long.

```python
def test_signed_long(self):
    """Typecode 'l': signed long."""
    arr = array.array('l', [-1, 0, 1])
    assert arr.itemsize >= 4
```

**Verification**: ✅ Tested in CI

### Typecode 'L': unsigned long.

```python
def test_unsigned_long(self):
    """Typecode 'L': unsigned long."""
    arr = array.array('L', [0, 1, 2])
    assert arr.itemsize >= 4
```

**Verification**: ✅ Tested in CI

### Typecode 'q': signed long long.

```python
def test_signed_long_long(self):
    """Typecode 'q': signed long long."""
    arr = array.array('q', [-9223372036854775808, 0, 9223372036854775807])
    assert arr.itemsize == 8
```

**Verification**: ✅ Tested in CI

### Typecode 'Q': unsigned long long.

```python
def test_unsigned_long_long(self):
    """Typecode 'Q': unsigned long long."""
    arr = array.array('Q', [0, 9223372036854775808])
    assert arr.itemsize == 8
```

**Verification**: ✅ Tested in CI

### Typecode 'f': float.

```python
def test_float(self):
    """Typecode 'f': float."""
    arr = array.array('f', [3.14])
    assert arr.itemsize == 4
    assert abs(arr[0] - 3.14) < 0.01
```

**Verification**: ✅ Tested in CI

### Typecode 'd': double.

```python
def test_double(self):
    """Typecode 'd': double."""
    arr = array.array('d', [3.14159265359])
    assert arr.itemsize == 8
    assert abs(arr[0] - 3.14159265359) < 1e-10
```

**Verification**: ✅ Tested in CI

### Edge: Single element array.

```python
def test_single_element(self):
    """Edge: Single element array."""
    arr = array.array('i', [42])
    assert len(arr) == 1
    assert arr[0] == 42
```

**Verification**: ✅ Tested in CI

### Performance: Large array creation.

```python
def test_large_array(self):
    """Performance: Large array creation."""
    arr = array.array('i', range(10000))
    assert len(arr) == 10000
    assert arr[9999] == 9999
```

**Verification**: ✅ Tested in CI

### Feature: Copy array via full slice.

```python
def test_copy_via_slice(self):
    """Feature: Copy array via full slice."""
    arr = array.array('i', [1, 2, 3])
    copy = arr[:]
    copy[0] = 99
    assert arr[0] == 1
```

**Verification**: ✅ Tested in CI

### Feature: Delete slice.

```python
def test_del_slice(self):
    """Feature: Delete slice."""
    arr = array.array('i', [1, 2, 3, 4, 5])
    del arr[1:3]
    assert len(arr) == 3
    assert arr.tolist() == [1, 4, 5]
```

**Verification**: ✅ Tested in CI

### Feature: Delete single element.

```python
def test_del_single_element(self):
    """Feature: Delete single element."""
    arr = array.array('i', [1, 2, 3])
    del arr[1]
    assert len(arr) == 2
    assert arr.tolist() == [1, 3]
```

**Verification**: ✅ Tested in CI

### Feature: Bool conversion.

```python
def test_bool_conversion(self):
    """Feature: Bool conversion."""
    empty = array.array('i')
    filled = array.array('i', [1])
    assert not empty
    assert filled
```

**Verification**: ✅ Tested in CI

### Edge: Minimum and maximum values.

```python
def test_min_max_values(self):
    """Edge: Minimum and maximum values."""
    arr = array.array('i', [-2147483648, 2147483647])
    assert min(arr) == -2147483648
    assert max(arr) == 2147483647
```

**Verification**: ✅ Tested in CI

### Edge: Multiply by zero.

```python
def test_zero_multiplication(self):
    """Edge: Multiply by zero."""
    arr = array.array('i', [1, 2, 3])
    result = arr * 0
    assert len(result) == 0
```

**Verification**: ✅ Tested in CI
