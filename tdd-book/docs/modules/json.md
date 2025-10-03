# json

## json.loads() - Deserialize JSON string to Python object.

## json.dumps() - Serialize Python object to JSON string.

## JSON round-trip: dumps → loads should preserve data.

## JSON edge cases and quirks.

## json.load() - Deserialize from file object.

## json.dump() - Serialize to file object.

### Basic: Parse simple JSON types.

```python
def test_loads_basic_types(self):
    """Basic: Parse simple JSON types."""
    assert json.loads('null') is None
    assert json.loads('true') is True
    assert json.loads('false') is False
    assert json.loads('42') == 42
    assert json.loads('3.14') == 3.14
    assert json.loads('"hello"') == 'hello'
```

**Verification**: ✅ Tested in CI

### Basic: Parse JSON array to Python list.

```python
def test_loads_array(self):
    """Basic: Parse JSON array to Python list."""
    result = json.loads('[1, 2, 3]')
    assert result == [1, 2, 3]
    assert isinstance(result, list)
```

**Verification**: ✅ Tested in CI

### Basic: Parse JSON object to Python dict.

```python
def test_loads_object(self):
    """Basic: Parse JSON object to Python dict."""
    result = json.loads('{"name": "Alice", "age": 30}')
    assert result == {'name': 'Alice', 'age': 30}
    assert isinstance(result, dict)
```

**Verification**: ✅ Tested in CI

### Property: Nested structures are preserved.

```python
def test_loads_nested_structure(self):
    """Property: Nested structures are preserved."""
    json_str = '{"users": [{"name": "Alice"}, {"name": "Bob"}]}'
    result = json.loads(json_str)
    assert len(result['users']) == 2
    assert result['users'][0]['name'] == 'Alice'
```

**Verification**: ✅ Tested in CI

### Edge: Empty arrays and objects.

```python
def test_loads_empty_structures(self):
    """Edge: Empty arrays and objects."""
    assert json.loads('[]') == []
    assert json.loads('{}') == {}
```

**Verification**: ✅ Tested in CI

### Edge: Unicode characters are handled.

```python
def test_loads_unicode(self):
    """Edge: Unicode characters are handled."""
    result = json.loads('{"emoji": "\\u2764"}')
    assert result == {'emoji': '❤'}
```

**Verification**: ✅ Tested in CI

### Error: Invalid JSON raises JSONDecodeError.

```python
def test_loads_invalid_json_raises(self):
    """Error: Invalid JSON raises JSONDecodeError."""
    with pytest.raises(json.JSONDecodeError):
        json.loads('{invalid}')
    with pytest.raises(json.JSONDecodeError):
        json.loads('[1, 2,]')
```

**Verification**: ✅ Tested in CI

### Edge: Duplicate keys - last value wins.

```python
def test_loads_strict_duplicate_keys(self):
    """Edge: Duplicate keys - last value wins."""
    result = json.loads('{"key": 1, "key": 2}')
    assert result == {'key': 2}
```

**Verification**: ✅ Tested in CI

### Basic: Serialize simple Python types.

```python
def test_dumps_basic_types(self):
    """Basic: Serialize simple Python types."""
    assert json.dumps(None) == 'null'
    assert json.dumps(True) == 'true'
    assert json.dumps(False) == 'false'
    assert json.dumps(42) == '42'
    assert json.dumps('hello') == '"hello"'
```

**Verification**: ✅ Tested in CI

### Basic: Serialize Python list to JSON array.

```python
def test_dumps_list(self):
    """Basic: Serialize Python list to JSON array."""
    result = json.dumps([1, 2, 3])
    assert result == '[1, 2, 3]'
```

**Verification**: ✅ Tested in CI

### Basic: Serialize Python dict to JSON object.

```python
def test_dumps_dict(self):
    """Basic: Serialize Python dict to JSON object."""
    result = json.dumps({'name': 'Alice'})
    assert json.loads(result) == {'name': 'Alice'}
```

**Verification**: ✅ Tested in CI

### Property: Nested structures are serialized correctly.

```python
def test_dumps_nested_structure(self):
    """Property: Nested structures are serialized correctly."""
    data = {'users': [{'name': 'Alice'}, {'name': 'Bob'}]}
    result = json.dumps(data)
    assert json.loads(result) == data
```

**Verification**: ✅ Tested in CI

### Feature: Pretty-printing with indent.

```python
def test_dumps_with_indent(self):
    """Feature: Pretty-printing with indent."""
    data = {'name': 'Alice', 'age': 30}
    result = json.dumps(data, indent=2)
    assert '\n' in result
    assert '  ' in result
```

**Verification**: ✅ Tested in CI

### Feature: ASCII encoding option.

```python
def test_dumps_ensure_ascii(self):
    """Feature: ASCII encoding option."""
    data = {'emoji': '❤'}
    result_ascii = json.dumps(data, ensure_ascii=True)
    assert '\\u' in result_ascii
    result_unicode = json.dumps(data, ensure_ascii=False)
    assert '❤' in result_unicode
```

**Verification**: ✅ Tested in CI

### Feature: Deterministic ordering with sort_keys.

```python
def test_dumps_sort_keys(self):
    """Feature: Deterministic ordering with sort_keys."""
    data = {'z': 1, 'a': 2, 'm': 3}
    result = json.dumps(data, sort_keys=True)
    assert result == '{"a": 2, "m": 3, "z": 1}'
```

**Verification**: ✅ Tested in CI

### Error: Non-serializable types raise TypeError.

```python
def test_dumps_non_serializable_raises(self):
    """Error: Non-serializable types raise TypeError."""
    with pytest.raises(TypeError):
        json.dumps(set([1, 2, 3]))
    with pytest.raises(TypeError):
        json.dumps(complex(1, 2))
```

**Verification**: ✅ Tested in CI

### Feature: Custom separators for compact output.

```python
def test_dumps_custom_separators(self):
    """Feature: Custom separators for compact output."""
    data = {'a': 1, 'b': 2}
    compact = json.dumps(data, separators=(',', ':'))
    assert ', ' not in compact
    assert ': ' not in compact
```

**Verification**: ✅ Tested in CI

### Property: dumps → loads round-trip preserves data.

```python
@given(st.one_of(st.none(), st.booleans(), st.integers(), st.floats(allow_nan=False, allow_infinity=False), st.text(), st.lists(st.integers(), max_size=10), st.dictionaries(st.text(min_size=1), st.integers(), max_size=10)))
def test_roundtrip_preserves_data(self, value):
    """Property: dumps → loads round-trip preserves data."""
    if isinstance(value, float) and value != value:
        return
    json_str = json.dumps(value)
    result = json.loads(json_str)
    assert result == value
```

**Verification**: ✅ Tested in CI

### Property: Complex nested structures survive round-trip.

```python
def test_roundtrip_nested_structure(self):
    """Property: Complex nested structures survive round-trip."""
    original = {'users': [{'name': 'Alice', 'scores': [95, 87, 92]}, {'name': 'Bob', 'scores': [88, 91, 85]}], 'metadata': {'version': 1, 'count': 2}}
    json_str = json.dumps(original)
    result = json.loads(json_str)
    assert result == original
```

**Verification**: ✅ Tested in CI

### Edge: Infinity and NaN are allowed by default (non-standard JSON).

```python
def test_inf_nan_allowed_by_default(self):
    """Edge: Infinity and NaN are allowed by default (non-standard JSON)."""
    result_inf = json.dumps(float('inf'))
    assert result_inf == 'Infinity'
    result_nan = json.dumps(float('nan'))
    assert result_nan == 'NaN'
```

**Verification**: ✅ Tested in CI

### Edge: allow_nan=False rejects Infinity/NaN (strict JSON).

```python
def test_inf_nan_rejected_with_allow_nan_false(self):
    """Edge: allow_nan=False rejects Infinity/NaN (strict JSON)."""
    with pytest.raises(ValueError):
        json.dumps(float('inf'), allow_nan=False)
    with pytest.raises(ValueError):
        json.dumps(float('nan'), allow_nan=False)
```

**Verification**: ✅ Tested in CI

### Edge: Leading/trailing whitespace is ignored.

```python
def test_trailing_whitespace_ignored(self):
    """Edge: Leading/trailing whitespace is ignored."""
    assert json.loads('  42  ') == 42
    assert json.loads('\n\t"hello"\n') == 'hello'
```

**Verification**: ✅ Tested in CI

### Edge: Escape sequences are handled correctly.

```python
def test_escape_sequences(self):
    """Edge: Escape sequences are handled correctly."""
    assert json.loads('"line1\\nline2"') == 'line1\nline2'
    assert json.loads('"col1\\tcol2"') == 'col1\tcol2'
    assert json.loads('"path\\\\file"') == 'path\\file'
```

**Verification**: ✅ Tested in CI

### Edge: Large integers are preserved exactly.

```python
def test_large_numbers_preserved(self):
    """Edge: Large integers are preserved exactly."""
    large_num = 123456789012345678901234567890
    json_str = json.dumps(large_num)
    result = json.loads(json_str)
    assert result == large_num
```

**Verification**: ✅ Tested in CI

### Edge: Floating point precision limitations.

```python
def test_float_precision_quirks(self):
    """Edge: Floating point precision limitations."""
    original = 0.1 + 0.2
    json_str = json.dumps(original)
    result = json.loads(json_str)
    assert abs(result - 0.3) < 0.0001
```

**Verification**: ✅ Tested in CI

### Basic: Load JSON from file.

```python
def test_load_from_file(self, tmp_path):
    """Basic: Load JSON from file."""
    json_file = tmp_path / 'data.json'
    json_file.write_text('{"name": "Alice", "age": 30}')
    with open(json_file) as f:
        result = json.load(f)
    assert result == {'name': 'Alice', 'age': 30}
```

**Verification**: ✅ Tested in CI

### Basic: Dump JSON to file.

```python
def test_dump_to_file(self, tmp_path):
    """Basic: Dump JSON to file."""
    json_file = tmp_path / 'output.json'
    data = {'name': 'Bob', 'scores': [85, 90, 88]}
    with open(json_file, 'w') as f:
        json.dump(data, f)
    content = json_file.read_text()
    assert json.loads(content) == data
```

**Verification**: ✅ Tested in CI
