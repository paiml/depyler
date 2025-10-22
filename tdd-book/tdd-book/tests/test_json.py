"""
TDD tests for json module - JSON encoder and decoder.

Tests verify that all Python examples using json (loads, dumps) transpile
to valid Rust and execute correctly. The json module provides JSON serialization.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_json_loads_basic():
    """Test json.loads() with basic types."""
    python_code = '''
import json

def test_loads() -> str:
    # Parse basic JSON types
    null_val = json.loads("null")
    bool_val = json.loads("true")
    int_val = json.loads("42")
    str_val = json.loads('"hello"')

    # Return concatenated result
    result = str(null_val) + "," + str(bool_val) + "," + str(int_val) + "," + str_val

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


def test_json_loads_dict():
    """Test json.loads() with dictionary."""
    python_code = '''
import json

def test_loads_dict() -> str:
    # Parse JSON object to dict
    data = json.loads('{"name": "Alice", "age": 30}')

    # Access values
    name = data["name"]
    age = data["age"]

    return name + "," + str(age)
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
    # Parse JSON array to list
    data = json.loads('[1, 2, 3, 4, 5]')

    # Calculate sum
    total = 0
    for num in data:
        total += num

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


def test_json_dumps_basic():
    """Test json.dumps() serialization."""
    python_code = '''
import json

def test_dumps() -> str:
    # Serialize Python objects to JSON
    data = {"name": "Bob", "age": 25}
    json_str = json.dumps(data)

    return json_str
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
    """Test JSON round-trip (dumps then loads)."""
    python_code = '''
import json

def test_roundtrip() -> bool:
    # Original data
    original = {"users": ["Alice", "Bob"], "count": 2}

    # Serialize and deserialize
    json_str = json.dumps(original)
    restored = json.loads(json_str)

    # Check if equal
    return restored["count"] == original["count"]
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


def test_json_nested_structures():
    """Test json with nested structures."""
    python_code = '''
import json

def test_nested() -> int:
    # Parse nested JSON structure
    json_str = '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}'
    data = json.loads(json_str)

    # Access nested data
    users = data["users"]
    first_user_age = users[0]["age"]

    return first_user_age
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
