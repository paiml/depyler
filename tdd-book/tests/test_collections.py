"""
TDD tests for collections module examples.

Tests verify that all Python examples in the collections.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_list_basic_operations():
    """Test basic list operations: append, extend, remove, pop."""
    python_code = '''
def list_operations() -> list[int]:
    numbers: list[int] = [1, 2, 3]
    numbers.append(4)
    numbers.extend([5, 6])
    numbers.remove(3)
    last = numbers.pop()
    return numbers
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


def test_dict_basic_operations():
    """Test basic dict operations: get, keys, values, items, pop."""
    python_code = '''
def dict_operations() -> dict[str, int]:
    scores: dict[str, int] = {"alice": 95, "bob": 87}
    alice_score = scores.get("alice", 0)
    scores["charlie"] = 92
    bob_score = scores.pop("bob")
    return scores
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


def test_set_basic_operations():
    """Test basic set operations: add, remove, discard, union."""
    python_code = '''
def set_operations() -> set[int]:
    numbers: set[int] = {1, 2, 3}
    numbers.add(4)
    numbers.discard(2)
    other: set[int] = {4, 5, 6}
    result = numbers.union(other)
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_list_comprehension():
    """Test list comprehension transpilation."""
    python_code = '''
def list_comp() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5]
    squares = [x * x for x in numbers if x % 2 == 0]
    return squares
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
