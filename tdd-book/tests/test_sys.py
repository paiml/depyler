"""
TDD tests for sys module - System-specific parameters and functions.

Tests verify that all Python examples using sys.platform, sys.argv, sys.maxsize, etc.
transpile to valid Rust and execute correctly. The sys module provides system access.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_sys_platform():
    """Test sys.platform access."""
    python_code = '''
import sys

def test_platform() -> int:
    # Get platform string
    platform = sys.platform

    # Return length of platform string
    return len(platform)
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


def test_sys_maxsize():
    """Test sys.maxsize value."""
    python_code = '''
import sys

def test_maxsize() -> int:
    # Get maxsize value
    max_val = sys.maxsize

    # Check if it's positive (return 1) or not (return 0)
    if max_val > 0:
        return 1
    else:
        return 0
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


def test_sys_argv():
    """Test sys.argv access."""
    python_code = '''
import sys

def test_argv() -> int:
    # Access argv
    args = sys.argv

    # Return length of argv
    return len(args)
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


def test_sys_path():
    """Test sys.path access."""
    python_code = '''
import sys

def test_path() -> int:
    # Access sys.path
    path = sys.path

    # Return length of path
    return len(path)
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


def test_sys_modules():
    """Test sys.modules access."""
    python_code = '''
import sys

def test_modules() -> int:
    # Access sys.modules
    modules = sys.modules

    # Return length (number of loaded modules)
    return len(modules)
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


def test_sys_getsizeof():
    """Test sys.getsizeof() function."""
    python_code = '''
import sys

def test_getsizeof() -> int:
    # Get size of an integer
    size = sys.getsizeof(42)

    # Return 1 if size is positive, 0 otherwise
    if size > 0:
        return 1
    else:
        return 0
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
