"""
TDD tests for array module - Typed array operations.

Tests verify that all Python examples using array.array() with various typecodes
transpile to valid Rust and execute correctly. The array module provides efficient typed arrays.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_array_integer():
    """Test array.array() with integer typecode."""
    python_code = '''
import array

def test_int_array() -> int:
    # Create integer array
    arr = array.array('i', [1, 2, 3, 4, 5])

    # Access and sum elements
    total = 0
    for val in arr:
        total += val

    return total
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


def test_array_float():
    """Test array.array() with float typecode."""
    python_code = '''
import array

def test_float_array() -> int:
    # Create float array
    arr = array.array('f', [1.0, 2.5, 3.14])

    # Check length
    return len(arr)
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


def test_array_empty():
    """Test array.array() with empty initialization."""
    python_code = '''
import array

def test_empty_array() -> int:
    # Create empty integer array
    arr = array.array('i')

    # Add elements
    arr.append(10)
    arr.append(20)

    return len(arr)
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


def test_array_indexing():
    """Test array indexing and modification."""
    python_code = '''
import array

def test_array_index() -> int:
    # Create and index array
    arr = array.array('i', [10, 20, 30])

    # Modify via index
    arr[1] = 25

    # Return modified value
    return arr[1]
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


def test_array_from_range():
    """Test array.array() created from range."""
    python_code = '''
import array

def test_range_array() -> int:
    # Create array from range
    arr = array.array('i', range(5))

    # Return last element
    return arr[4]
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


def test_array_double():
    """Test array.array() with double typecode."""
    python_code = '''
import array

def test_double_array() -> int:
    # Create double-precision float array
    arr = array.array('d', [1.5, 2.5, 3.5])

    # Sum and return as int
    total = 0.0
    for val in arr:
        total += val

    return int(total)
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
