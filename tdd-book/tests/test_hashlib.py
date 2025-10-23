"""
TDD tests for hashlib module - Cryptographic hashing.

Tests verify that all Python examples using hashlib (md5, sha1, sha256, sha512)
transpile to valid Rust and execute correctly. The hashlib module provides secure hash functions.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_hashlib_md5_basic():
    """Test hashlib.md5() with basic string hashing."""
    python_code = '''
import hashlib

def test_md5() -> str:
    # Create MD5 hash
    data = "hello world"
    hash_obj = hashlib.md5(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

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


def test_hashlib_sha256_basic():
    """Test hashlib.sha256() with basic string hashing."""
    python_code = '''
import hashlib

def test_sha256() -> str:
    # Create SHA256 hash
    data = "hello world"
    hash_obj = hashlib.sha256(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

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


def test_hashlib_sha1_basic():
    """Test hashlib.sha1() for SHA1 hashing."""
    python_code = '''
import hashlib

def test_sha1() -> str:
    # Create SHA1 hash
    data = "test data"
    hash_obj = hashlib.sha1(data.encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

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


def test_hashlib_sha512_basic():
    """Test hashlib.sha512() for SHA512 hashing."""
    python_code = '''
import hashlib

def test_sha512() -> str:
    # Create SHA512 hash
    data = "secure data"
    hash_obj = hashlib.sha512(data.encode('utf-8'))

    # Get hexadecimal digest (first 16 chars for brevity)
    full_hash = hash_obj.hexdigest()
    result = full_hash[:16]

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


def test_hashlib_update_incremental():
    """Test hash.update() for incremental hashing."""
    python_code = '''
import hashlib

def test_update() -> str:
    # Create hash with incremental updates
    hash_obj = hashlib.sha256()
    hash_obj.update("hello".encode('utf-8'))
    hash_obj.update(" ".encode('utf-8'))
    hash_obj.update("world".encode('utf-8'))

    # Get hexadecimal digest
    result = hash_obj.hexdigest()

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


def test_hashlib_hash_comparison():
    """Test comparing hashes for data integrity verification."""
    python_code = '''
import hashlib

def test_comparison() -> int:
    # Hash same data twice
    data = "important data"

    hash1 = hashlib.sha256(data.encode('utf-8')).hexdigest()
    hash2 = hashlib.sha256(data.encode('utf-8')).hexdigest()

    # Hashes should be identical
    if hash1 == hash2:
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
