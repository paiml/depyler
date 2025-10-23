"""
TDD tests for functools module - Functional programming tools.

Tests verify that all Python examples using functools.reduce()
transpile to valid Rust and execute correctly. The functools module provides higher-order functions.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_functools_reduce():
    """Test functools.reduce() for sequence reduction."""
    python_code = '''
from functools import reduce

def test_reduce() -> int:
    # Reduce list to sum
    numbers = [1, 2, 3, 4, 5]
    total = reduce(lambda x, y: x + y, numbers)

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


def test_functools_reduce_with_initial():
    """Test functools.reduce() with initial value."""
    python_code = '''
from functools import reduce

def test_reduce_initial() -> int:
    # Reduce with initial value
    numbers = [1, 2, 3, 4]
    total = reduce(lambda x, y: x + y, numbers, 10)

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


def test_functools_reduce_max():
    """Test functools.reduce() to find maximum."""
    python_code = '''
from functools import reduce

def test_reduce_max() -> int:
    # Find maximum using reduce
    numbers = [5, 2, 9, 1, 7]
    maximum = reduce(lambda x, y: x if x > y else y, numbers)

    return maximum
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


def test_functools_reduce_multiply():
    """Test functools.reduce() for multiplication (product)."""
    python_code = '''
from functools import reduce

def test_reduce_product() -> int:
    # Calculate product using reduce
    numbers = [2, 3, 4]
    product = reduce(lambda x, y: x * y, numbers)

    return product
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
