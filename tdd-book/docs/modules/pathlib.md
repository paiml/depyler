# pathlib

## Path() - Create Path objects from strings.

## Path properties - name, stem, suffix, parent, parts.

## Path checks - exists(), is_file(), is_dir(), is_absolute().

## Path manipulation - with_name(), with_suffix(), with_stem().

## Path I/O - read_text(), write_text(), read_bytes(), write_bytes().

## Directory operations - mkdir(), rmdir(), iterdir().

## Path resolution - resolve(), absolute().

## Globbing - glob(), rglob().

### Basic: Create Path from string.

```python
def test_path_basic(self):
    """Basic: Create Path from string."""
    p = Path('/home/user')
    assert isinstance(p, Path)
    assert str(p) == '/home/user'
```

**Verification**: ✅ Tested in CI

### Feature: Create Path from multiple parts.

```python
def test_path_from_parts(self):
    """Feature: Create Path from multiple parts."""
    p = Path('home', 'user', 'documents')
    if sys.platform == 'win32':
        assert str(p) == 'home\\user\\documents'
    else:
        assert str(p) == 'home/user/documents'
```

**Verification**: ✅ Tested in CI

### Feature: Use / operator to join paths.

```python
def test_path_slash_operator(self):
    """Feature: Use / operator to join paths."""
    p = Path('/home') / 'user' / 'documents'
    if sys.platform == 'win32':
        assert 'user' in str(p) and 'documents' in str(p)
    else:
        assert str(p) == '/home/user/documents'
```

**Verification**: ✅ Tested in CI

### Feature: Current directory as Path('.').

```python
def test_path_current_directory(self):
    """Feature: Current directory as Path('.')."""
    p = Path('.')
    assert isinstance(p, Path)
    assert p.exists()
```

**Verification**: ✅ Tested in CI

### Edge: Empty string creates current directory path.

```python
def test_path_empty_string(self):
    """Edge: Empty string creates current directory path."""
    p = Path('')
    assert str(p) == '.'
```

**Verification**: ✅ Tested in CI

### Basic: Get filename with name property.

```python
def test_path_name(self):
    """Basic: Get filename with name property."""
    p = Path('/home/user/document.txt')
    assert p.name == 'document.txt'
```

**Verification**: ✅ Tested in CI

### Feature: Get filename without extension.

```python
def test_path_stem(self):
    """Feature: Get filename without extension."""
    p = Path('/home/user/document.txt')
    assert p.stem == 'document'
```

**Verification**: ✅ Tested in CI

### Feature: Get file extension.

```python
def test_path_suffix(self):
    """Feature: Get file extension."""
    p = Path('/home/user/document.txt')
    assert p.suffix == '.txt'
```

**Verification**: ✅ Tested in CI

### Feature: Get all extensions for compound suffixes.

```python
def test_path_suffixes(self):
    """Feature: Get all extensions for compound suffixes."""
    p = Path('/home/user/archive.tar.gz')
    assert p.suffixes == ['.tar', '.gz']
```

**Verification**: ✅ Tested in CI

### Feature: Get parent directory.

```python
def test_path_parent(self):
    """Feature: Get parent directory."""
    p = Path('/home/user/documents/file.txt')
    assert str(p.parent) == '/home/user/documents'
```

**Verification**: ✅ Tested in CI

### Feature: Get all parent directories.

```python
def test_path_parents(self):
    """Feature: Get all parent directories."""
    p = Path('/home/user/documents')
    parents = list(p.parents)
    assert '/home/user' in str(parents[0])
    assert '/home' in str(parents[1])
```

**Verification**: ✅ Tested in CI

### Property: Path split into parts.

```python
def test_path_parts(self):
    """Property: Path split into parts."""
    p = Path('/home/user/documents')
    if sys.platform == 'win32':
        assert 'user' in p.parts and 'documents' in p.parts
    else:
        assert p.parts == ('/', 'home', 'user', 'documents')
```

**Verification**: ✅ Tested in CI

### Edge: Files without extension have empty suffix.

```python
def test_path_no_suffix(self):
    """Edge: Files without extension have empty suffix."""
    p = Path('/home/user/README')
    assert p.suffix == ''
    assert p.stem == 'README'
```

**Verification**: ✅ Tested in CI

### Edge: Hidden files (starting with .) handled correctly.

```python
def test_path_hidden_file(self):
    """Edge: Hidden files (starting with .) handled correctly."""
    p = Path('/home/user/.bashrc')
    assert p.name == '.bashrc'
    assert p.stem == '.bashrc'
    assert p.suffix == ''
```

**Verification**: ✅ Tested in CI

### Basic: Path.exists() returns True for existing file.

```python
def test_exists_true(self, tmp_path):
    """Basic: Path.exists() returns True for existing file."""
    file = tmp_path / 'test.txt'
    file.write_text('data')
    assert file.exists() is True
```

**Verification**: ✅ Tested in CI

### Basic: Path.exists() returns False for non-existent path.

```python
def test_exists_false(self):
    """Basic: Path.exists() returns False for non-existent path."""
    p = Path('/nonexistent/path')
    assert p.exists() is False
```

**Verification**: ✅ Tested in CI

### Feature: is_file() returns True for files.

```python
def test_is_file(self, tmp_path):
    """Feature: is_file() returns True for files."""
    file = tmp_path / 'test.txt'
    file.write_text('data')
    assert file.is_file() is True
    assert tmp_path.is_file() is False
```

**Verification**: ✅ Tested in CI

### Feature: is_dir() returns True for directories.

```python
def test_is_dir(self, tmp_path):
    """Feature: is_dir() returns True for directories."""
    assert tmp_path.is_dir() is True
    file = tmp_path / 'test.txt'
    file.write_text('data')
    assert file.is_dir() is False
```

**Verification**: ✅ Tested in CI

### Feature: is_absolute() checks for absolute paths.

```python
def test_is_absolute(self):
    """Feature: is_absolute() checks for absolute paths."""
    assert Path('/home/user').is_absolute() is True
    assert Path('relative/path').is_absolute() is False
```

**Verification**: ✅ Tested in CI

### Feature: is_symlink() detects symbolic links.

```python
def test_is_symlink(self, tmp_path):
    """Feature: is_symlink() detects symbolic links."""
    if sys.platform == 'win32':
        pytest.skip('Symlinks require admin on Windows')
    target = tmp_path / 'target.txt'
    target.write_text('data')
    link = tmp_path / 'link.txt'
    link.symlink_to(target)
    assert link.is_symlink() is True
    assert target.is_symlink() is False
```

**Verification**: ✅ Tested in CI

### Basic: Replace filename with with_name().

```python
def test_with_name(self):
    """Basic: Replace filename with with_name()."""
    p = Path('/home/user/old.txt')
    new_p = p.with_name('new.txt')
    assert str(new_p) == '/home/user/new.txt'
```

**Verification**: ✅ Tested in CI

### Feature: Change file extension with with_suffix().

```python
def test_with_suffix(self):
    """Feature: Change file extension with with_suffix()."""
    p = Path('/home/user/document.txt')
    new_p = p.with_suffix('.md')
    assert str(new_p) == '/home/user/document.md'
```

**Verification**: ✅ Tested in CI

### Feature: Change filename without extension (Python 3.9+).

```python
def test_with_stem(self):
    """Feature: Change filename without extension (Python 3.9+)."""
    if not hasattr(Path, 'with_stem'):
        pytest.skip('with_stem() requires Python 3.9+')
    p = Path('/home/user/old.txt')
    new_p = p.with_stem('new')
    assert str(new_p) == '/home/user/new.txt'
```

**Verification**: ✅ Tested in CI

### Edge: Remove suffix with empty string.

```python
def test_with_suffix_remove(self):
    """Edge: Remove suffix with empty string."""
    p = Path('/home/user/document.txt')
    new_p = p.with_suffix('')
    assert str(new_p) == '/home/user/document'
```

**Verification**: ✅ Tested in CI

### Edge: with_suffix() only changes last extension.

```python
def test_with_suffix_compound(self):
    """Edge: with_suffix() only changes last extension."""
    p = Path('/home/user/archive.tar.gz')
    new_p = p.with_suffix('.bz2')
    assert str(new_p) == '/home/user/archive.tar.bz2'
```

**Verification**: ✅ Tested in CI

### Basic: Write text to file with write_text().

```python
def test_write_text_basic(self, tmp_path):
    """Basic: Write text to file with write_text()."""
    file = tmp_path / 'test.txt'
    file.write_text('Hello, World!')
    assert file.read_text() == 'Hello, World!'
```

**Verification**: ✅ Tested in CI

### Basic: Read text from file with read_text().

```python
def test_read_text_basic(self, tmp_path):
    """Basic: Read text from file with read_text()."""
    file = tmp_path / 'test.txt'
    file.write_text('Content')
    content = file.read_text()
    assert content == 'Content'
```

**Verification**: ✅ Tested in CI

### Feature: Write binary data with write_bytes().

```python
def test_write_bytes(self, tmp_path):
    """Feature: Write binary data with write_bytes()."""
    file = tmp_path / 'data.bin'
    data = b'\x00\x01\x02\x03'
    file.write_bytes(data)
    assert file.read_bytes() == data
```

**Verification**: ✅ Tested in CI

### Feature: Read binary data with read_bytes().

```python
def test_read_bytes(self, tmp_path):
    """Feature: Read binary data with read_bytes()."""
    file = tmp_path / 'data.bin'
    data = b'binary data'
    file.write_bytes(data)
    assert file.read_bytes() == data
```

**Verification**: ✅ Tested in CI

### Feature: Specify encoding when writing text.

```python
def test_write_text_encoding(self, tmp_path):
    """Feature: Specify encoding when writing text."""
    file = tmp_path / 'utf8.txt'
    file.write_text('Hello 世界', encoding='utf-8')
    content = file.read_text(encoding='utf-8')
    assert content == 'Hello 世界'
```

**Verification**: ✅ Tested in CI

### Edge: write_text() overwrites existing content.

```python
def test_write_text_overwrites(self, tmp_path):
    """Edge: write_text() overwrites existing content."""
    file = tmp_path / 'test.txt'
    file.write_text('first')
    file.write_text('second')
    assert file.read_text() == 'second'
```

**Verification**: ✅ Tested in CI

### Basic: Create directory with mkdir().

```python
def test_mkdir_basic(self, tmp_path):
    """Basic: Create directory with mkdir()."""
    new_dir = tmp_path / 'new_directory'
    new_dir.mkdir()
    assert new_dir.exists()
    assert new_dir.is_dir()
```

**Verification**: ✅ Tested in CI

### Feature: Create parent directories with parents=True.

```python
def test_mkdir_parents(self, tmp_path):
    """Feature: Create parent directories with parents=True."""
    nested = tmp_path / 'a' / 'b' / 'c'
    nested.mkdir(parents=True)
    assert nested.exists()
    assert (tmp_path / 'a').exists()
    assert (tmp_path / 'a' / 'b').exists()
```

**Verification**: ✅ Tested in CI

### Feature: mkdir() with exist_ok=True doesn't raise error.

```python
def test_mkdir_exist_ok(self, tmp_path):
    """Feature: mkdir() with exist_ok=True doesn't raise error."""
    new_dir = tmp_path / 'dir'
    new_dir.mkdir()
    new_dir.mkdir(exist_ok=True)
```

**Verification**: ✅ Tested in CI

### Feature: Remove empty directory with rmdir().

```python
def test_rmdir(self, tmp_path):
    """Feature: Remove empty directory with rmdir()."""
    new_dir = tmp_path / 'to_remove'
    new_dir.mkdir()
    assert new_dir.exists()
    new_dir.rmdir()
    assert not new_dir.exists()
```

**Verification**: ✅ Tested in CI

### Feature: Iterate directory contents with iterdir().

```python
def test_iterdir(self, tmp_path):
    """Feature: Iterate directory contents with iterdir()."""
    (tmp_path / 'file1.txt').write_text('a')
    (tmp_path / 'file2.txt').write_text('b')
    (tmp_path / 'subdir').mkdir()
    items = list(tmp_path.iterdir())
    assert len(items) == 3
    names = [p.name for p in items]
    assert 'file1.txt' in names
    assert 'file2.txt' in names
    assert 'subdir' in names
```

**Verification**: ✅ Tested in CI

### Error: mkdir() without parents raises if parent missing.

```python
def test_mkdir_parents_false_raises(self, tmp_path):
    """Error: mkdir() without parents raises if parent missing."""
    nested = tmp_path / 'a' / 'b' / 'c'
    with pytest.raises(FileNotFoundError):
        nested.mkdir(parents=False)
```

**Verification**: ✅ Tested in CI

### Error: mkdir() on existing directory raises FileExistsError.

```python
def test_mkdir_exists_raises(self, tmp_path):
    """Error: mkdir() on existing directory raises FileExistsError."""
    new_dir = tmp_path / 'dir'
    new_dir.mkdir()
    with pytest.raises(FileExistsError):
        new_dir.mkdir(exist_ok=False)
```

**Verification**: ✅ Tested in CI

### Basic: Resolve relative path to absolute.

```python
def test_resolve_basic(self, tmp_path):
    """Basic: Resolve relative path to absolute."""
    file = tmp_path / 'test.txt'
    file.write_text('data')
    relative = Path(file.name)
    resolved = (tmp_path / relative).resolve()
    assert resolved.is_absolute()
```

**Verification**: ✅ Tested in CI

### Feature: Convert to absolute path with absolute().

```python
def test_absolute(self, tmp_path):
    """Feature: Convert to absolute path with absolute()."""
    file = tmp_path / 'test.txt'
    file.write_text('data')
    absolute = file.absolute()
    assert absolute.is_absolute()
```

**Verification**: ✅ Tested in CI

### Feature: resolve() follows symbolic links.

```python
def test_resolve_symlinks(self, tmp_path):
    """Feature: resolve() follows symbolic links."""
    if sys.platform == 'win32':
        pytest.skip('Symlinks require admin on Windows')
    target = tmp_path / 'target.txt'
    target.write_text('data')
    link = tmp_path / 'link.txt'
    link.symlink_to(target)
    resolved = link.resolve()
    assert resolved == target.resolve()
```

**Verification**: ✅ Tested in CI

### Basic: Find files matching pattern with glob().

```python
def test_glob_basic(self, tmp_path):
    """Basic: Find files matching pattern with glob()."""
    (tmp_path / 'file1.txt').write_text('a')
    (tmp_path / 'file2.txt').write_text('b')
    (tmp_path / 'file3.md').write_text('c')
    txt_files = list(tmp_path.glob('*.txt'))
    assert len(txt_files) == 2
    names = [p.name for p in txt_files]
    assert 'file1.txt' in names
    assert 'file2.txt' in names
```

**Verification**: ✅ Tested in CI

### Feature: Recursive glob with ** pattern.

```python
def test_glob_recursive(self, tmp_path):
    """Feature: Recursive glob with ** pattern."""
    (tmp_path / 'file.txt').write_text('a')
    subdir = tmp_path / 'subdir'
    subdir.mkdir()
    (subdir / 'nested.txt').write_text('b')
    all_txt = list(tmp_path.glob('**/*.txt'))
    assert len(all_txt) == 2
```

**Verification**: ✅ Tested in CI

### Feature: Recursive glob with rglob().

```python
def test_rglob(self, tmp_path):
    """Feature: Recursive glob with rglob()."""
    (tmp_path / 'file.txt').write_text('a')
    subdir = tmp_path / 'subdir'
    subdir.mkdir()
    (subdir / 'nested.txt').write_text('b')
    all_txt = list(tmp_path.rglob('*.txt'))
    assert len(all_txt) == 2
```

**Verification**: ✅ Tested in CI

### Edge: glob() returns empty for no matches.

```python
def test_glob_no_matches(self, tmp_path):
    """Edge: glob() returns empty for no matches."""
    matches = list(tmp_path.glob('*.nonexistent'))
    assert matches == []
```

**Verification**: ✅ Tested in CI

### Feature: Use ? wildcard in glob patterns.

```python
def test_glob_question_mark(self, tmp_path):
    """Feature: Use ? wildcard in glob patterns."""
    (tmp_path / 'file1.txt').write_text('a')
    (tmp_path / 'file2.txt').write_text('b')
    (tmp_path / 'file10.txt').write_text('c')
    matches = list(tmp_path.glob('file?.txt'))
    assert len(matches) == 2
```

**Verification**: ✅ Tested in CI
