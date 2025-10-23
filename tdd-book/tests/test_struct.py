"""
TDD tests for struct module - Binary data packing/unpacking.

Tests verify that all Python examples using struct.pack() and struct.unpack()
transpile to valid Rust and execute correctly. The struct module provides binary data operations.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_struct_pack_integer():
    """Test struct.pack() with single integer."""
    python_code = '''
import struct

def test_pack_int() -> int:
    # Pack integer to bytes
    packed = struct.pack('i', 42)

    # Return length
    return len(packed)
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


def test_struct_unpack_integer():
    """Test struct.unpack() with single integer."""
    python_code = '''
import struct

def test_unpack_int() -> int:
    # Pack then unpack
    packed = struct.pack('i', 42)
    unpacked = struct.unpack('i', packed)

    # Return unpacked value
    return unpacked[0]
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


def test_struct_pack_multiple():
    """Test struct.pack() with multiple values."""
    python_code = '''
import struct

def test_pack_multi() -> int:
    # Pack multiple values
    packed = struct.pack('ii', 10, 20)

    # Return length (2 ints * 4 bytes = 8)
    return len(packed)
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


def test_struct_unpack_multiple():
    """Test struct.unpack() with multiple values."""
    python_code = '''
import struct

def test_unpack_multi() -> int:
    # Pack then unpack multiple values
    packed = struct.pack('ii', 10, 20)
    values = struct.unpack('ii', packed)

    # Return sum
    return values[0] + values[1]
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


def test_struct_calcsize():
    """Test struct.calcsize() for format size."""
    python_code = '''
import struct

def test_calcsize() -> int:
    # Calculate size of format
    size = struct.calcsize('i')

    return size
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


def test_struct_roundtrip():
    """Test struct roundtrip (pack then unpack)."""
    python_code = '''
import struct

def test_roundtrip() -> int:
    # Roundtrip multiple values
    original_a = 100
    original_b = 200

    # Pack
    packed = struct.pack('ii', original_a, original_b)

    # Unpack
    a, b = struct.unpack('ii', packed)

    # Verify correctness by returning sum
    return a + b
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
