"""
TDD tests for re module - Regular expression operations.

Tests verify that all Python examples using re.search(), re.match(), re.findall(), etc.
transpile to valid Rust and execute correctly. The re module provides regex pattern matching.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_re_search_basic():
    """Test re.search() with basic pattern."""
    python_code = '''
import re

def test_search() -> int:
    # Search for pattern
    text = "hello world"
    match = re.search(r"world", text)

    # Return 1 if found, 0 otherwise
    if match:
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


def test_re_match_start():
    """Test re.match() from string start."""
    python_code = '''
import re

def test_match() -> int:
    # Match from start
    text = "hello world"
    match = re.match(r"hello", text)

    # Return 1 if matches, 0 otherwise
    if match:
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


def test_re_compile():
    """Test re.compile() for pre-compiled patterns."""
    python_code = '''
import re

def test_compile() -> int:
    # Compile pattern
    pattern = re.compile(r"[0-9]+")

    # Use compiled pattern
    text = "there are 42 apples"
    match = pattern.search(text)

    if match:
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


def test_re_findall():
    """Test re.findall() to find all matches."""
    python_code = '''
import re

def test_findall() -> int:
    # Find all matches
    text = "1 apple, 2 oranges, 3 bananas"
    matches = re.findall(r"[0-9]+", text)

    # Return count of matches
    return len(matches)
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


def test_re_sub():
    """Test re.sub() for pattern substitution."""
    python_code = '''
import re

def test_sub() -> str:
    # Replace pattern with string
    text = "hello world"
    result = re.sub(r"world", "universe", text)

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


def test_re_groups():
    """Test re.search() with capturing groups."""
    python_code = '''
import re

def test_groups() -> int:
    # Pattern with groups
    text = "John: 30 years old"
    match = re.search(r"(\w+): (\d+)", text)

    if match:
        # Return 1 if we got groups
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
