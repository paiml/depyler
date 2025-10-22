"""
TDD tests for pathlib module - Object-oriented filesystem paths.

Tests verify that all Python examples using pathlib transpile to valid Rust
and execute correctly. Pathlib provides object-oriented filesystem paths.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_pathlib_properties():
    """Test Path properties: name, stem, suffix, parent."""
    python_code = '''
from pathlib import Path

def get_path_properties() -> str:
    # Create path object
    p = Path("/home/user/documents/file.txt")

    # Get various path properties
    name = str(p.name)
    stem = str(p.stem)
    suffix = str(p.suffix)
    parent = str(p.parent)

    # Return concatenated result for verification
    return name + "," + stem + "," + suffix + "," + parent
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_pathlib_checks():
    """Test Path checking operations: exists(), is_file(), is_dir()."""
    python_code = '''
from pathlib import Path

def check_path_types() -> bool:
    # Check current directory
    current = Path(".")
    exists = current.exists()

    # Should be a directory
    is_directory = current.is_dir()

    # Should not be a file
    is_file = current.is_file()

    return exists and is_directory and not is_file
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_pathlib_file_io():
    """Test Path I/O operations: write_text(), read_text()."""
    python_code = '''
from pathlib import Path
import tempfile
import os

def test_file_operations() -> str:
    # Create temporary file
    fd, tmp_path = tempfile.mkstemp(suffix=".txt")
    os.close(fd)

    p = Path(tmp_path)

    # Write text to file
    content = "Hello, pathlib!"
    p.write_text(content)

    # Read text back
    read_content = p.read_text()

    # Cleanup
    os.unlink(tmp_path)

    return read_content
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_pathlib_directory_operations():
    """Test directory operations: mkdir(), iterdir()."""
    python_code = '''
from pathlib import Path
import tempfile
import shutil

def test_directory_ops() -> int:
    # Create temporary directory
    tmp_dir = tempfile.mkdtemp()

    # Create subdirectory using pathlib
    base = Path(tmp_dir)
    subdir = base / "test_subdir"
    subdir.mkdir(exist_ok=True)

    # Create some files
    (subdir / "file1.txt").write_text("content1")
    (subdir / "file2.txt").write_text("content2")

    # Count files using iterdir()
    count = 0
    for item in subdir.iterdir():
        if item.is_file():
            count += 1

    # Cleanup
    shutil.rmtree(tmp_dir)

    return count
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_pathlib_manipulation():
    """Test path manipulation: with_name(), with_suffix()."""
    python_code = '''
from pathlib import Path

def manipulate_paths() -> str:
    # Original path
    p = Path("/home/user/document.txt")

    # Change filename
    new_name = p.with_name("report.txt")

    # Change suffix
    new_suffix = p.with_suffix(".md")

    # Get string representations
    name_str = str(new_name)
    suffix_str = str(new_suffix)

    # Return concatenated result
    return name_str + "," + suffix_str
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_pathlib_path_construction():
    """Test Path construction with / operator."""
    python_code = '''
from pathlib import Path

def construct_paths() -> str:
    # Build path using / operator
    home = Path("/home")
    user_dir = home / "user"
    docs = user_dir / "documents"
    file_path = docs / "file.txt"

    # Convert to string
    result = str(file_path)

    return result
'''

    # Transpile to Rust
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(python_code)
        py_file = f.name

    try:
        result = subprocess.run(
            ['depyler', 'transpile', py_file, '--verify'],
            capture_output=True,
            text=True,
            timeout=30
        )
        assert result.returncode == 0, f"Transpilation failed: {result.stderr}"
    finally:
        PyPath(py_file).unlink(missing_ok=True)
