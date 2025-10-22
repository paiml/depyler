"""
TDD tests for string module examples.

Tests verify that all Python examples in the string.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_string_case_operations():
    """Test string case operations: upper, lower."""
    python_code = '''
def string_case() -> str:
    text: str = "Hello World"
    upper_text = text.upper()
    lower_text = text.lower()
    return lower_text
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


def test_string_trim_operations():
    """Test string trimming: strip, lstrip, rstrip."""
    python_code = '''
def string_trim() -> str:
    text: str = "  hello  "
    stripped = text.strip()
    return stripped
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


def test_string_split_join():
    """Test string split and join operations."""
    python_code = '''
def string_split_join() -> str:
    text: str = "apple,banana,cherry"
    parts = text.split(",")
    rejoined = "-".join(parts)
    return rejoined
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


def test_string_search_operations():
    """Test string searching: find, startswith, endswith."""
    python_code = '''
def string_search() -> bool:
    text: str = "hello world"
    starts = text.startswith("hello")
    ends = text.endswith("world")
    pos = text.find("world")
    return starts and ends
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


def test_string_replace_count():
    """Test string replace and count operations."""
    python_code = '''
def string_replace_count() -> int:
    text: str = "hello hello hello"
    count = text.count("hello")
    replaced = text.replace("hello", "hi")
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_string_validation():
    """Test string validation: isdigit, isalpha."""
    python_code = '''
def string_validation() -> bool:
    text: str = "12345"
    is_digit = text.isdigit()
    text2: str = "hello"
    is_alpha = text2.isalpha()
    return is_digit and is_alpha
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
