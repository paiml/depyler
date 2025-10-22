"""
TDD tests for os module examples.

Tests verify that all Python examples in the os.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_os_getcwd():
    """Test os.getcwd() for getting current working directory."""
    python_code = '''
import os

def get_current_directory() -> str:
    # Get current working directory
    cwd = os.getcwd()

    return cwd
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_os_listdir():
    """Test os.listdir() for listing directory contents."""
    python_code = '''
import os

def list_current_directory() -> int:
    # List files in current directory
    cwd = os.getcwd()
    files = os.listdir(cwd)

    # Return count of files
    return len(files)
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_os_path_operations():
    """Test os.path checking operations (exists, isfile, isdir)."""
    python_code = '''
import os

def check_path_operations() -> bool:
    # Check if current directory exists
    cwd = os.getcwd()
    exists = os.path.exists(cwd)
    is_dir = os.path.isdir(cwd)

    # Current directory should exist and be a directory
    return exists and is_dir
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_os_path_components():
    """Test os.path.basename() and os.path.dirname()."""
    python_code = '''
import os

def get_path_components() -> str:
    # Get components from a path
    path = "/home/user/document.txt"
    filename = os.path.basename(path)
    directory = os.path.dirname(path)

    # Return filename (should be "document.txt")
    return filename
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_os_getenv():
    """Test os.getenv() for reading environment variables."""
    python_code = '''
import os

def get_environment_variable() -> str:
    # Get environment variable with default
    home = os.getenv("HOME", "/default/home")

    # Get variable that might not exist
    custom = os.getenv("MY_CUSTOM_VAR", "default_value")

    return custom
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)
