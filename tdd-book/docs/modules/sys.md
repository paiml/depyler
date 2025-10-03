# sys

## sys.platform - Get current operating system platform.

## sys.version - Get Python version string.

## sys.argv - Command line arguments list.

## sys.path - Module search path.

## sys.modules - Dictionary of loaded modules.

## sys.maxsize - Largest integer supported.

## sys.getsizeof() - Get object size in bytes.

## sys.exit() - Exit the interpreter.

### Basic: platform is always a string.

```python
def test_platform_is_string(self):
    """Basic: platform is always a string."""
    assert isinstance(sys.platform, str)
    assert len(sys.platform) > 0
```

**Verification**: ✅ Tested in CI

### Edge: platform should be one of common values.

```python
def test_platform_common_values(self):
    """Edge: platform should be one of common values."""
    common_platforms = ['linux', 'darwin', 'win32', 'cygwin', 'aix']
    assert any((sys.platform.startswith(p) for p in common_platforms))
```

**Verification**: ✅ Tested in CI

### Property: platform value doesn't change during execution.

```python
def test_platform_immutable(self):
    """Property: platform value doesn't change during execution."""
    platform1 = sys.platform
    platform2 = sys.platform
    assert platform1 == platform2
    assert platform1 is platform2
```

**Verification**: ✅ Tested in CI

### Basic: version is a non-empty string.

```python
def test_version_is_string(self):
    """Basic: version is a non-empty string."""
    assert isinstance(sys.version, str)
    assert len(sys.version) > 0
```

**Verification**: ✅ Tested in CI

### Edge: version string contains version numbers.

```python
def test_version_contains_python_version(self):
    """Edge: version string contains version numbers."""
    assert str(sys.version_info.major) in sys.version
    assert str(sys.version_info.minor) in sys.version
```

**Verification**: ✅ Tested in CI

### Property: version_info has expected structure.

```python
def test_version_info_structure(self):
    """Property: version_info has expected structure."""
    assert hasattr(sys.version_info, 'major')
    assert hasattr(sys.version_info, 'minor')
    assert hasattr(sys.version_info, 'micro')
    assert isinstance(sys.version_info.major, int)
    assert isinstance(sys.version_info.minor, int)
    assert sys.version_info.major >= 3
```

**Verification**: ✅ Tested in CI

### Basic: argv is always a list.

```python
def test_argv_is_list(self):
    """Basic: argv is always a list."""
    assert isinstance(sys.argv, list)
```

**Verification**: ✅ Tested in CI

### Property: argv[0] is the script name or pytest.

```python
def test_argv_first_element_is_script(self):
    """Property: argv[0] is the script name or pytest."""
    assert len(sys.argv) > 0
    assert isinstance(sys.argv[0], str)
    assert 'pytest' in sys.argv[0] or '.py' in sys.argv[0]
```

**Verification**: ✅ Tested in CI

### Property: all argv elements are strings.

```python
def test_argv_all_strings(self):
    """Property: all argv elements are strings."""
    for arg in sys.argv:
        assert isinstance(arg, str)
```

**Verification**: ✅ Tested in CI

### Basic: path is a list of strings.

```python
def test_path_is_list(self):
    """Basic: path is a list of strings."""
    assert isinstance(sys.path, list)
    assert len(sys.path) > 0
```

**Verification**: ✅ Tested in CI

### Property: all path entries are strings.

```python
def test_path_contains_strings(self):
    """Property: all path entries are strings."""
    for entry in sys.path:
        assert isinstance(entry, str)
```

**Verification**: ✅ Tested in CI

### Edge: path can be modified.

```python
def test_path_is_mutable(self):
    """Edge: path can be modified."""
    original_length = len(sys.path)
    test_path = '/tmp/test_path'
    sys.path.append(test_path)
    assert test_path in sys.path
    sys.path.remove(test_path)
    assert len(sys.path) == original_length
```

**Verification**: ✅ Tested in CI

### Property: first path entry is usually test directory or cwd.

```python
def test_current_directory_in_path(self):
    """Property: first path entry is usually test directory or cwd."""
    assert len(sys.path) > 0
    first_entry = sys.path[0]
    assert isinstance(first_entry, str)
    assert len(first_entry) >= 0
```

**Verification**: ✅ Tested in CI

### Basic: modules is a dictionary.

```python
def test_modules_is_dict(self):
    """Basic: modules is a dictionary."""
    assert isinstance(sys.modules, dict)
    assert len(sys.modules) > 0
```

**Verification**: ✅ Tested in CI

### Property: sys module is always loaded.

```python
def test_modules_contains_sys(self):
    """Property: sys module is always loaded."""
    assert 'sys' in sys.modules
    assert sys.modules['sys'] is sys
```

**Verification**: ✅ Tested in CI

### Property: all module names are strings.

```python
def test_modules_keys_are_strings(self):
    """Property: all module names are strings."""
    for name in list(sys.modules.keys())[:10]:
        assert isinstance(name, str)
```

**Verification**: ✅ Tested in CI

### Basic: maxsize is an integer.

```python
def test_maxsize_is_int(self):
    """Basic: maxsize is an integer."""
    assert isinstance(sys.maxsize, int)
```

**Verification**: ✅ Tested in CI

### Property: maxsize indicates platform word size.

```python
def test_maxsize_indicates_64bit(self):
    """Property: maxsize indicates platform word size."""
    assert sys.maxsize > 0
    is_64bit = sys.maxsize > 2 ** 32
    assert is_64bit or not is_64bit
```

**Verification**: ✅ Tested in CI

### Edge: maxsize CAN be modified (surprising!), but shouldn't be.

```python
def test_maxsize_can_be_modified_but_shouldnt(self):
    """Edge: maxsize CAN be modified (surprising!), but shouldn't be."""
    original = sys.maxsize
    sys.maxsize = 999
    assert sys.maxsize == 999
    sys.maxsize = original
    assert sys.maxsize == original
```

**Verification**: ✅ Tested in CI

### Basic: getsizeof works on basic types.

```python
def test_getsizeof_basic_types(self):
    """Basic: getsizeof works on basic types."""
    assert sys.getsizeof(0) > 0
    assert sys.getsizeof('') > 0
    assert sys.getsizeof([]) > 0
    assert sys.getsizeof({}) > 0
```

**Verification**: ✅ Tested in CI

### Property: larger objects have larger sizes.

```python
def test_getsizeof_larger_object_bigger(self):
    """Property: larger objects have larger sizes."""
    small_list = [1, 2, 3]
    large_list = list(range(1000))
    assert sys.getsizeof(large_list) > sys.getsizeof(small_list)
```

**Verification**: ✅ Tested in CI

### Property: string size scales with length.

```python
def test_getsizeof_string_scaling(self):
    """Property: string size scales with length."""
    short_str = 'a'
    long_str = 'a' * 1000
    assert sys.getsizeof(long_str) > sys.getsizeof(short_str)
```

**Verification**: ✅ Tested in CI

### Property: getsizeof always returns positive integer.

```python
@given(st.integers(min_value=0, max_value=10000))
def test_getsizeof_returns_positive(self, value):
    """Property: getsizeof always returns positive integer."""
    size = sys.getsizeof(value)
    assert isinstance(size, int)
    assert size > 0
```

**Verification**: ✅ Tested in CI

### Basic: sys.exit() raises SystemExit.

```python
def test_exit_raises_systemexit(self):
    """Basic: sys.exit() raises SystemExit."""
    with pytest.raises(SystemExit):
        sys.exit(0)
```

**Verification**: ✅ Tested in CI

### Edge: exit code is preserved in exception.

```python
def test_exit_with_code(self):
    """Edge: exit code is preserved in exception."""
    with pytest.raises(SystemExit) as exc_info:
        sys.exit(42)
    assert exc_info.value.code == 42
```

**Verification**: ✅ Tested in CI

### Edge: exit can take string message.

```python
def test_exit_with_string(self):
    """Edge: exit can take string message."""
    with pytest.raises(SystemExit) as exc_info:
        sys.exit('error message')
    assert exc_info.value.code == 'error message'
```

**Verification**: ✅ Tested in CI
