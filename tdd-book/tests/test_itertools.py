"""
TDD tests for itertools module - Iterator building blocks.

Tests verify that all Python examples using itertools functions
transpile to valid Rust and execute correctly. The itertools module provides efficient iterators.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_itertools_chain():
    """Test itertools.chain() for chaining iterables."""
    python_code = '''
from itertools import chain

def test_chain() -> int:
    # Chain multiple lists together
    list1 = [1, 2, 3]
    list2 = [4, 5, 6]
    list3 = [7, 8, 9]

    result = list(chain(list1, list2, list3))

    # Return sum of all elements
    total = sum(result)

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


def test_itertools_islice():
    """Test itertools.islice() for slicing iterables."""
    python_code = '''
from itertools import islice

def test_islice() -> int:
    # Take first 5 elements from range
    numbers = range(100)
    first_five = list(islice(numbers, 5))

    # Sum of first five: 0+1+2+3+4 = 10
    total = sum(first_five)

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


def test_itertools_repeat():
    """Test itertools.repeat() for repeating elements."""
    python_code = '''
from itertools import repeat

def test_repeat() -> int:
    # Repeat value 5 times
    repeated = list(repeat(10, 5))

    # Sum: 10*5 = 50
    total = sum(repeated)

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


def test_itertools_count():
    """Test itertools.count() for infinite counter."""
    python_code = '''
from itertools import count, islice

def test_count() -> int:
    # Create counter starting at 10
    counter = count(10)

    # Take first 5 values: 10, 11, 12, 13, 14
    first_five = list(islice(counter, 5))

    # Sum: 10+11+12+13+14 = 60
    total = sum(first_five)

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


def test_itertools_zip_longest():
    """Test itertools.zip_longest() for zipping with padding."""
    python_code = '''
from itertools import zip_longest

def test_zip_longest() -> int:
    # Zip lists of different lengths
    list1 = [1, 2, 3]
    list2 = [10, 20]

    # zip_longest pads shorter list with None (or fillvalue)
    result = list(zip_longest(list1, list2, fillvalue=0))

    # Count total elements: [(1,10), (2,20), (3,0)]
    count = len(result)

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


def test_itertools_product():
    """Test itertools.product() for cartesian product."""
    python_code = '''
from itertools import product

def test_product() -> int:
    # Cartesian product of two lists
    list1 = [1, 2]
    list2 = [10, 20]

    result = list(product(list1, list2))

    # Should have 2*2 = 4 combinations: [(1,10), (1,20), (2,10), (2,20)]
    count = len(result)

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
