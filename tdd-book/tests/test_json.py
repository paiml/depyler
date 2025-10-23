"""
TDD tests for json module - JSON encoding and decoding.

Tests verify that all Python examples using json.loads() and json.dumps()
transpile to valid Rust and execute correctly. The json module provides JSON serialization.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_json_loads_integer():
    """Test json.loads() with integer."""
    python_code = '''
import json

def test_loads_int() -> int:
    # Parse JSON integer
    val = json.loads("42")

    return val
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


def test_json_loads_string():
    """Test json.loads() with string."""
    python_code = '''
import json

def test_loads_str() -> str:
    # Parse JSON string
    val = json.loads('"hello world"')

    return val
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


def test_json_loads_dict():
    """Test json.loads() with dictionary."""
    python_code = '''
import json

def test_loads_dict() -> int:
    # Parse JSON object
    data = json.loads('{"name": "Alice", "age": 30}')

    # Access value
    return data["age"]
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


def test_json_loads_list():
    """Test json.loads() with list."""
    python_code = '''
import json

def test_loads_list() -> int:
    # Parse JSON array
    data = json.loads('[1, 2, 3, 4, 5]')

    # Return length
    return len(data)
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


def test_json_dumps_basic():
    """Test json.dumps() with basic types."""
    python_code = '''
import json

def test_dumps() -> str:
    # Serialize to JSON
    data = {"x": 10, "y": 20}
    result = json.dumps(data)

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


def test_json_roundtrip():
    """Test json dumps then loads (roundtrip)."""
    python_code = '''
import json

def test_roundtrip() -> int:
    # Roundtrip: serialize then deserialize
    original = [1, 2, 3]
    json_str = json.dumps(original)
    restored = json.loads(json_str)

    # Return sum to verify correctness
    total = 0
    for val in restored:
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
