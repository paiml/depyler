"""Test pathlib module - Object-oriented filesystem paths.

This module tests pathlib's Path class and related functionality for
working with filesystem paths in an object-oriented way.
"""

import pathlib
import sys
import pytest
from pathlib import Path


class TestPathCreation:
    """Path() - Create Path objects from strings."""

    def test_path_basic(self):
        """Basic: Create Path from string."""
        p = Path('/home/user')
        assert isinstance(p, Path)
        assert str(p) == '/home/user'

    def test_path_from_parts(self):
        """Feature: Create Path from multiple parts."""
        p = Path('home', 'user', 'documents')
        if sys.platform == 'win32':
            assert str(p) == 'home\\user\\documents'
        else:
            assert str(p) == 'home/user/documents'

    def test_path_slash_operator(self):
        """Feature: Use / operator to join paths."""
        p = Path('/home') / 'user' / 'documents'
        if sys.platform == 'win32':
            assert 'user' in str(p) and 'documents' in str(p)
        else:
            assert str(p) == '/home/user/documents'

    def test_path_current_directory(self):
        """Feature: Current directory as Path('.')."""
        p = Path('.')
        assert isinstance(p, Path)
        assert p.exists()

    def test_path_empty_string(self):
        """Edge: Empty string creates current directory path."""
        p = Path('')
        assert str(p) == '.'


class TestPathProperties:
    """Path properties - name, stem, suffix, parent, parts."""

    def test_path_name(self):
        """Basic: Get filename with name property."""
        p = Path('/home/user/document.txt')
        assert p.name == 'document.txt'

    def test_path_stem(self):
        """Feature: Get filename without extension."""
        p = Path('/home/user/document.txt')
        assert p.stem == 'document'

    def test_path_suffix(self):
        """Feature: Get file extension."""
        p = Path('/home/user/document.txt')
        assert p.suffix == '.txt'

    def test_path_suffixes(self):
        """Feature: Get all extensions for compound suffixes."""
        p = Path('/home/user/archive.tar.gz')
        assert p.suffixes == ['.tar', '.gz']

    def test_path_parent(self):
        """Feature: Get parent directory."""
        p = Path('/home/user/documents/file.txt')
        assert str(p.parent) == '/home/user/documents'

    def test_path_parents(self):
        """Feature: Get all parent directories."""
        p = Path('/home/user/documents')
        parents = list(p.parents)
        assert '/home/user' in str(parents[0])
        assert '/home' in str(parents[1])

    def test_path_parts(self):
        """Property: Path split into parts."""
        p = Path('/home/user/documents')
        if sys.platform == 'win32':
            assert 'user' in p.parts and 'documents' in p.parts
        else:
            assert p.parts == ('/', 'home', 'user', 'documents')

    def test_path_no_suffix(self):
        """Edge: Files without extension have empty suffix."""
        p = Path('/home/user/README')
        assert p.suffix == ''
        assert p.stem == 'README'

    def test_path_hidden_file(self):
        """Edge: Hidden files (starting with .) handled correctly."""
        p = Path('/home/user/.bashrc')
        assert p.name == '.bashrc'
        assert p.stem == '.bashrc'
        assert p.suffix == ''


class TestPathChecks:
    """Path checks - exists(), is_file(), is_dir(), is_absolute()."""

    def test_exists_true(self, tmp_path):
        """Basic: Path.exists() returns True for existing file."""
        file = tmp_path / 'test.txt'
        file.write_text('data')
        assert file.exists() is True

    def test_exists_false(self):
        """Basic: Path.exists() returns False for non-existent path."""
        p = Path('/nonexistent/path')
        assert p.exists() is False

    def test_is_file(self, tmp_path):
        """Feature: is_file() returns True for files."""
        file = tmp_path / 'test.txt'
        file.write_text('data')
        assert file.is_file() is True
        assert tmp_path.is_file() is False

    def test_is_dir(self, tmp_path):
        """Feature: is_dir() returns True for directories."""
        assert tmp_path.is_dir() is True
        file = tmp_path / 'test.txt'
        file.write_text('data')
        assert file.is_dir() is False

    def test_is_absolute(self):
        """Feature: is_absolute() checks for absolute paths."""
        assert Path('/home/user').is_absolute() is True
        assert Path('relative/path').is_absolute() is False

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


class TestPathManipulation:
    """Path manipulation - with_name(), with_suffix(), with_stem()."""

    def test_with_name(self):
        """Basic: Replace filename with with_name()."""
        p = Path('/home/user/old.txt')
        new_p = p.with_name('new.txt')
        assert str(new_p) == '/home/user/new.txt'

    def test_with_suffix(self):
        """Feature: Change file extension with with_suffix()."""
        p = Path('/home/user/document.txt')
        new_p = p.with_suffix('.md')
        assert str(new_p) == '/home/user/document.md'

    def test_with_stem(self):
        """Feature: Change filename without extension (Python 3.9+)."""
        if not hasattr(Path, 'with_stem'):
            pytest.skip('with_stem() requires Python 3.9+')

        p = Path('/home/user/old.txt')
        new_p = p.with_stem('new')
        assert str(new_p) == '/home/user/new.txt'

    def test_with_suffix_remove(self):
        """Edge: Remove suffix with empty string."""
        p = Path('/home/user/document.txt')
        new_p = p.with_suffix('')
        assert str(new_p) == '/home/user/document'

    def test_with_suffix_compound(self):
        """Edge: with_suffix() only changes last extension."""
        p = Path('/home/user/archive.tar.gz')
        new_p = p.with_suffix('.bz2')
        assert str(new_p) == '/home/user/archive.tar.bz2'


class TestPathIO:
    """Path I/O - read_text(), write_text(), read_bytes(), write_bytes()."""

    def test_write_text_basic(self, tmp_path):
        """Basic: Write text to file with write_text()."""
        file = tmp_path / 'test.txt'
        file.write_text('Hello, World!')
        assert file.read_text() == 'Hello, World!'

    def test_read_text_basic(self, tmp_path):
        """Basic: Read text from file with read_text()."""
        file = tmp_path / 'test.txt'
        file.write_text('Content')
        content = file.read_text()
        assert content == 'Content'

    def test_write_bytes(self, tmp_path):
        """Feature: Write binary data with write_bytes()."""
        file = tmp_path / 'data.bin'
        data = b'\x00\x01\x02\x03'
        file.write_bytes(data)
        assert file.read_bytes() == data

    def test_read_bytes(self, tmp_path):
        """Feature: Read binary data with read_bytes()."""
        file = tmp_path / 'data.bin'
        data = b'binary data'
        file.write_bytes(data)
        assert file.read_bytes() == data

    def test_write_text_encoding(self, tmp_path):
        """Feature: Specify encoding when writing text."""
        file = tmp_path / 'utf8.txt'
        file.write_text('Hello 世界', encoding='utf-8')
        content = file.read_text(encoding='utf-8')
        assert content == 'Hello 世界'

    def test_write_text_overwrites(self, tmp_path):
        """Edge: write_text() overwrites existing content."""
        file = tmp_path / 'test.txt'
        file.write_text('first')
        file.write_text('second')
        assert file.read_text() == 'second'


class TestDirectoryOperations:
    """Directory operations - mkdir(), rmdir(), iterdir()."""

    def test_mkdir_basic(self, tmp_path):
        """Basic: Create directory with mkdir()."""
        new_dir = tmp_path / 'new_directory'
        new_dir.mkdir()
        assert new_dir.exists()
        assert new_dir.is_dir()

    def test_mkdir_parents(self, tmp_path):
        """Feature: Create parent directories with parents=True."""
        nested = tmp_path / 'a' / 'b' / 'c'
        nested.mkdir(parents=True)
        assert nested.exists()
        assert (tmp_path / 'a').exists()
        assert (tmp_path / 'a' / 'b').exists()

    def test_mkdir_exist_ok(self, tmp_path):
        """Feature: mkdir() with exist_ok=True doesn't raise error."""
        new_dir = tmp_path / 'dir'
        new_dir.mkdir()
        new_dir.mkdir(exist_ok=True)  # Should not raise

    def test_rmdir(self, tmp_path):
        """Feature: Remove empty directory with rmdir()."""
        new_dir = tmp_path / 'to_remove'
        new_dir.mkdir()
        assert new_dir.exists()
        new_dir.rmdir()
        assert not new_dir.exists()

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

    def test_mkdir_parents_false_raises(self, tmp_path):
        """Error: mkdir() without parents raises if parent missing."""
        nested = tmp_path / 'a' / 'b' / 'c'
        with pytest.raises(FileNotFoundError):
            nested.mkdir(parents=False)

    def test_mkdir_exists_raises(self, tmp_path):
        """Error: mkdir() on existing directory raises FileExistsError."""
        new_dir = tmp_path / 'dir'
        new_dir.mkdir()
        with pytest.raises(FileExistsError):
            new_dir.mkdir(exist_ok=False)


class TestPathResolution:
    """Path resolution - resolve(), absolute()."""

    def test_resolve_basic(self, tmp_path):
        """Basic: Resolve relative path to absolute."""
        file = tmp_path / 'test.txt'
        file.write_text('data')

        # Create relative path and resolve
        relative = Path(file.name)
        resolved = (tmp_path / relative).resolve()
        assert resolved.is_absolute()

    def test_absolute(self, tmp_path):
        """Feature: Convert to absolute path with absolute()."""
        # Note: absolute() doesn't resolve symlinks like resolve()
        file = tmp_path / 'test.txt'
        file.write_text('data')

        absolute = file.absolute()
        assert absolute.is_absolute()

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


class TestGlobbing:
    """Globbing - glob(), rglob()."""

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

    def test_glob_recursive(self, tmp_path):
        """Feature: Recursive glob with ** pattern."""
        (tmp_path / 'file.txt').write_text('a')
        subdir = tmp_path / 'subdir'
        subdir.mkdir()
        (subdir / 'nested.txt').write_text('b')

        all_txt = list(tmp_path.glob('**/*.txt'))
        assert len(all_txt) == 2

    def test_rglob(self, tmp_path):
        """Feature: Recursive glob with rglob()."""
        (tmp_path / 'file.txt').write_text('a')
        subdir = tmp_path / 'subdir'
        subdir.mkdir()
        (subdir / 'nested.txt').write_text('b')

        all_txt = list(tmp_path.rglob('*.txt'))
        assert len(all_txt) == 2

    def test_glob_no_matches(self, tmp_path):
        """Edge: glob() returns empty for no matches."""
        matches = list(tmp_path.glob('*.nonexistent'))
        assert matches == []

    def test_glob_question_mark(self, tmp_path):
        """Feature: Use ? wildcard in glob patterns."""
        (tmp_path / 'file1.txt').write_text('a')
        (tmp_path / 'file2.txt').write_text('b')
        (tmp_path / 'file10.txt').write_text('c')

        # ? matches single character
        matches = list(tmp_path.glob('file?.txt'))
        assert len(matches) == 2  # file1.txt and file2.txt only
