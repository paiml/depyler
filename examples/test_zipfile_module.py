"""
Comprehensive test suite for zipfile module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests zipfile core features:
- Creating and reading ZIP archives
- Adding files to archives
- Extracting files
- Listing archive contents
- Compression methods
"""

import zipfile
from io import BytesIO


def test_zipfile_create_and_read():
    """Test creating a ZIP file and reading it back."""
    # Create in-memory ZIP
    buffer = BytesIO()

    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('test.txt', 'Hello, ZIP!')

    # Read it back
    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        content = zf.read('test.txt')
        assert content == b'Hello, ZIP!'

    print("PASS: test_zipfile_create_and_read")


def test_zipfile_multiple_files():
    """Test ZIP with multiple files."""
    buffer = BytesIO()

    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('file1.txt', 'Content 1')
        zf.writestr('file2.txt', 'Content 2')
        zf.writestr('file3.txt', 'Content 3')

    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        assert len(zf.namelist()) == 3
        assert 'file1.txt' in zf.namelist()
        assert 'file2.txt' in zf.namelist()
        assert 'file3.txt' in zf.namelist()

        assert zf.read('file2.txt') == b'Content 2'

    print("PASS: test_zipfile_multiple_files")


def test_zipfile_namelist():
    """Test listing files in ZIP archive."""
    buffer = BytesIO()

    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('alpha.txt', 'A')
        zf.writestr('beta.txt', 'B')
        zf.writestr('gamma.txt', 'C')

    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        names = zf.namelist()
        assert len(names) == 3
        assert 'alpha.txt' in names
        assert 'beta.txt' in names
        assert 'gamma.txt' in names

    print("PASS: test_zipfile_namelist")


def test_zipfile_getinfo():
    """Test getting file info from ZIP."""
    buffer = BytesIO()

    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('data.txt', 'Test data content')

    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        info = zf.getinfo('data.txt')
        assert info.filename == 'data.txt'
        assert info.file_size == len('Test data content')

    print("PASS: test_zipfile_getinfo")


def test_zipfile_compression():
    """Test ZIP with compression."""
    buffer = BytesIO()
    data = 'This is test data that should compress well! ' * 10

    # Create with compression
    with zipfile.ZipFile(buffer, 'w', zipfile.ZIP_DEFLATED) as zf:
        zf.writestr('compressed.txt', data)

    # Read back
    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        content = zf.read('compressed.txt').decode('utf-8')
        assert content == data

    print("PASS: test_zipfile_compression")


def test_zipfile_binary_data():
    """Test ZIP with binary data."""
    buffer = BytesIO()
    binary_data = bytes(range(256))

    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('binary.dat', binary_data)

    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        content = zf.read('binary.dat')
        assert content == binary_data

    print("PASS: test_zipfile_binary_data")


def test_zipfile_empty():
    """Test creating empty ZIP archive."""
    buffer = BytesIO()

    with zipfile.ZipFile(buffer, 'w') as zf:
        pass  # Create empty archive

    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        assert len(zf.namelist()) == 0

    print("PASS: test_zipfile_empty")


def test_zipfile_read_mode():
    """Test reading from existing ZIP."""
    buffer = BytesIO()

    # Create archive
    with zipfile.ZipFile(buffer, 'w') as zf:
        zf.writestr('readonly.txt', 'Read-only content')

    # Open in read mode
    buffer.seek(0)
    with zipfile.ZipFile(buffer, 'r') as zf:
        content = zf.read('readonly.txt')
        assert content == b'Read-only content'

    print("PASS: test_zipfile_read_mode")


def main():
    """Run all zipfile tests."""
    print("=" * 60)
    print("ZIPFILE MODULE TESTS")
    print("=" * 60)

    test_zipfile_create_and_read()
    test_zipfile_multiple_files()
    test_zipfile_namelist()
    test_zipfile_getinfo()
    test_zipfile_compression()
    test_zipfile_binary_data()
    test_zipfile_empty()
    test_zipfile_read_mode()

    print("=" * 60)
    print("ALL ZIPFILE TESTS PASSED!")
    print("Total tests: 8")
    print("=" * 60)


if __name__ == "__main__":
    main()
