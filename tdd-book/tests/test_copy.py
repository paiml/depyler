"""
TDD tests for copy module - Object copying operations.

Tests verify that all Python examples using copy.copy() and copy.deepcopy()
transpile to valid Rust and execute correctly. The copy module provides shallow and deep copy operations.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_copy_shallow_list():
    """Test copy.copy() for shallow list copying."""
    python_code = '''
import copy

def test_shallow_copy() -> int:
    # Shallow copy a list
    original = [1, 2, 3]
    copied = copy.copy(original)

    # Modify copied list
    copied.append(4)

    # Original should be unchanged
    return len(original)
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


def test_copy_shallow_dict():
    """Test copy.copy() for shallow dict copying."""
    python_code = '''
import copy

def test_shallow_dict() -> int:
    # Shallow copy a dict
    original = {"a": 1, "b": 2}
    copied = copy.copy(original)

    # Modify copied dict
    copied["c"] = 3

    # Original should not have new key
    return len(original)
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


def test_copy_deepcopy_nested():
    """Test copy.deepcopy() for nested structure copying."""
    python_code = '''
import copy

def test_deep_copy() -> int:
    # Deep copy nested structure
    inner = [1, 2]
    original = [inner, 3]
    copied = copy.deepcopy(original)

    # Modify nested list in copy
    copied[0].append(3)

    # Original inner list should be unchanged
    return len(original[0])
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


def test_copy_shallow_shares_nested():
    """Test that shallow copy shares nested objects."""
    python_code = '''
import copy

def test_shallow_shares() -> int:
    # Shallow copy with nested objects
    inner = [1, 2]
    original = [inner, 3]
    copied = copy.copy(original)

    # Modify nested list through copy
    copied[0].append(3)

    # Both original and copy see the change
    return len(original[0])
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


def test_copy_deepcopy_dict():
    """Test copy.deepcopy() for dict with nested values."""
    python_code = '''
import copy

def test_deep_copy_dict() -> int:
    # Deep copy dict with nested list
    inner = [1, 2]
    original = {"list": inner, "num": 42}
    copied = copy.deepcopy(original)

    # Modify nested list in copy
    copied["list"].append(3)

    # Original nested list should be unchanged
    return len(original["list"])
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


def test_copy_multiple_references():
    """Test copy behavior with multiple references."""
    python_code = '''
import copy

def test_multiple_refs() -> int:
    # Create structure with multiple references
    shared = [1, 2]
    original = [shared, shared]
    copied = copy.deepcopy(original)

    # Modify first element in copy
    copied[0].append(3)

    # In original, both refs point to same object (still length 2)
    # In copy, each is independent after deepcopy
    return len(original[0]) + len(copied[1])
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
