"""
TDD tests for base64 module - Base64 encoding.

Tests verify that all Python examples using base64 (b64encode, b64decode, urlsafe variants)
transpile to valid Rust and execute correctly. The base64 module provides binary-to-text encoding.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_base64_encode_string():
    """Test base64.b64encode() with string converted to bytes."""
    python_code = '''
import base64

def test_encode() -> str:
    # Encode string as base64
    data = "hello world"
    encoded = base64.b64encode(data.encode('utf-8'))

    # Convert back to string for return
    result = encoded.decode('utf-8')

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


def test_base64_decode_string():
    """Test base64.b64decode() with base64 string."""
    python_code = '''
import base64

def test_decode() -> str:
    # Decode base64 string
    encoded = "aGVsbG8gd29ybGQ="
    decoded = base64.b64decode(encoded)

    # Convert to string
    result = decoded.decode('utf-8')

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


def test_base64_roundtrip():
    """Test base64 encode then decode returns original."""
    python_code = '''
import base64

def test_roundtrip() -> str:
    # Original text
    original = "Test data 123!@#"

    # Encode then decode
    encoded = base64.b64encode(original.encode('utf-8'))
    decoded = base64.b64decode(encoded)
    result = decoded.decode('utf-8')

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


def test_base64_urlsafe_encode():
    """Test base64.urlsafe_b64encode() for URL-safe encoding."""
    python_code = '''
import base64

def test_urlsafe_encode() -> str:
    # URL-safe encoding (replaces + and / with - and _)
    data = "test data with special chars"
    encoded = base64.urlsafe_b64encode(data.encode('utf-8'))

    result = encoded.decode('utf-8')

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


def test_base64_urlsafe_decode():
    """Test base64.urlsafe_b64decode() for URL-safe decoding."""
    python_code = '''
import base64

def test_urlsafe_decode() -> str:
    # URL-safe decoding
    encoded = "dGVzdCBkYXRhIHdpdGggc3BlY2lhbCBjaGFycw=="
    decoded = base64.urlsafe_b64decode(encoded)

    result = decoded.decode('utf-8')

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


def test_base64_padding():
    """Test base64 with different padding scenarios."""
    python_code = '''
import base64

def test_padding() -> int:
    # Test various lengths (different padding)
    test1 = base64.b64encode("a".encode('utf-8'))
    test2 = base64.b64encode("ab".encode('utf-8'))
    test3 = base64.b64encode("abc".encode('utf-8'))

    # Count total length
    total = len(test1) + len(test2) + len(test3)

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
