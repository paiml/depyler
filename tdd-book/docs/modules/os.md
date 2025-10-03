# os

## os.path.join() - Combine path components intelligently.

## os.path.exists() - Check if path exists.

### Basic usage: Join two path components.

```python
def test_join_basic(self):
    """Basic usage: Join two path components."""
    result = os.path.join('home', 'user')
    if sys.platform == 'win32':
        assert result == 'home\\user'
    else:
        assert result == 'home/user'
```

**Verification**: ✅ Tested in CI

### Edge: Absolute path in arguments overrides previous.

```python
def test_join_absolute_override(self):
    """Edge: Absolute path in arguments overrides previous."""
    result = os.path.join('/home', '/etc')
    assert result == '/etc'
```

**Verification**: ✅ Tested in CI

### Edge: Empty strings are handled correctly.

```python
def test_join_empty_components(self):
    """Edge: Empty strings are handled correctly."""
    assert os.path.join('a', '', 'b') == os.path.join('a', 'b')
```

**Verification**: ✅ Tested in CI

### Error: Non-string arguments raise TypeError.

```python
@pytest.mark.parametrize('bad_input', [None, 123, [], {}])
def test_join_type_error(self, bad_input):
    """Error: Non-string arguments raise TypeError."""
    with pytest.raises(TypeError):
        os.path.join('path', bad_input)
```

**Verification**: ✅ Tested in CI

### Property: Joining any strings should not crash.

```python
@given(st.lists(st.text(), min_size=1, max_size=10))
def test_join_arbitrary_strings(self, components):
    """Property: Joining any strings should not crash."""
    result = os.path.join(*components)
    assert isinstance(result, str)
```

**Verification**: ✅ Tested in CI

### Happy path: Existing file returns True.

```python
def test_exists_true(self, tmp_path):
    """Happy path: Existing file returns True."""
    file = tmp_path / 'test.txt'
    file.write_text('data')
    assert os.path.exists(str(file)) is True
```

**Verification**: ✅ Tested in CI

### Happy path: Non-existent path returns False.

```python
def test_exists_false(self):
    """Happy path: Non-existent path returns False."""
    assert os.path.exists('/nonexistent/path') is False
```

**Verification**: ✅ Tested in CI

### Edge: Broken symlink returns False.

```python
def test_exists_broken_symlink(self, tmp_path):
    """Edge: Broken symlink returns False."""
    if sys.platform == 'win32':
        pytest.skip('Symlinks require admin on Windows')
    target = tmp_path / 'target'
    link = tmp_path / 'link'
    link.symlink_to(target)
    assert os.path.exists(str(link)) is False
```

**Verification**: ✅ Tested in CI

### Edge: Permission denied on parent directory.

```python
def test_exists_permission_denied(self, tmp_path):
    """Edge: Permission denied on parent directory."""
    if os.getuid() == 0:
        pytest.skip('Cannot test permissions as root')
    restricted = tmp_path / 'restricted'
    restricted.mkdir(mode=0)
    try:
        result = os.path.exists(str(restricted / 'file'))
        assert result is False
    finally:
        restricted.chmod(493)
```

**Verification**: ✅ Tested in CI
