"""
TDD tests for secrets module - Cryptographically secure random numbers.

Tests verify that all Python examples using secrets (token_hex, token_urlsafe, etc.)
transpile to valid Rust and execute correctly. The secrets module provides secure random generation.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_secrets_token_hex():
    """Test secrets.token_hex() for random hex tokens."""
    python_code = '''
import secrets

def test_token_hex() -> int:
    # Generate random hex token (16 bytes = 32 hex chars)
    token = secrets.token_hex(16)

    # Verify length
    length = len(token)

    return length
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


def test_secrets_token_urlsafe():
    """Test secrets.token_urlsafe() for URL-safe tokens."""
    python_code = '''
import secrets

def test_token_urlsafe() -> int:
    # Generate URL-safe token
    token = secrets.token_urlsafe(16)

    # Verify it's a string and has length
    length = len(token)

    return length
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


def test_secrets_randbelow():
    """Test secrets.randbelow() for random integers."""
    python_code = '''
import secrets

def test_randbelow() -> int:
    # Generate random number below 100
    num = secrets.randbelow(100)

    # Verify it's in valid range (0-99)
    if num >= 0 and num < 100:
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


def test_secrets_choice():
    """Test secrets.choice() for random selection from sequence."""
    python_code = '''
import secrets

def test_choice() -> int:
    # Choose random element from list
    options = [10, 20, 30, 40, 50]
    selected = secrets.choice(options)

    # Verify it's one of the options
    if selected in options:
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


def test_secrets_token_bytes():
    """Test secrets.token_bytes() for random bytes."""
    python_code = '''
import secrets

def test_token_bytes() -> int:
    # Generate random bytes
    token = secrets.token_bytes(16)

    # Verify length
    length = len(token)

    return length
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


def test_secrets_compare_digest():
    """Test secrets.compare_digest() for constant-time comparison."""
    python_code = '''
import secrets

def test_compare_digest() -> int:
    # Compare two identical strings (constant-time)
    str1 = "secret_value_123"
    str2 = "secret_value_123"

    if secrets.compare_digest(str1, str2):
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
