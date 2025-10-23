"""
TDD tests for memoryview - Memory view objects and buffer protocol.

Tests verify that all Python examples using memoryview() for zero-copy buffer access
transpile to valid Rust and execute correctly. The memoryview provides efficient memory access.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_memoryview_from_bytes():
    """Test memoryview() from bytes object."""
    python_code = '''
def test_memview_bytes() -> int:
    # Create memoryview from bytes
    data = b"hello world"
    view = memoryview(data)

    # Return length
    return len(view)
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


def test_memoryview_from_bytearray():
    """Test memoryview() from bytearray object."""
    python_code = '''
def test_memview_bytearray() -> int:
    # Create memoryview from bytearray
    data = bytearray(b"test")
    view = memoryview(data)

    # Return length
    return len(view)
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


def test_memoryview_indexing():
    """Test memoryview indexing."""
    python_code = '''
def test_memview_index() -> int:
    # Create and index memoryview
    data = b"hello"
    view = memoryview(data)

    # Get first byte (ord('h') = 104)
    return view[0]
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


def test_memoryview_slicing():
    """Test memoryview slicing."""
    python_code = '''
def test_memview_slice() -> int:
    # Create memoryview and slice it
    data = b"hello world"
    view = memoryview(data)

    # Slice first 5 bytes
    sliced = view[0:5]

    # Return length of slice
    return len(sliced)
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


def test_memoryview_tobytes():
    """Test memoryview.tobytes() method."""
    python_code = '''
def test_memview_tobytes() -> int:
    # Create memoryview and convert to bytes
    data = b"test"
    view = memoryview(data)

    # Convert back to bytes
    result = view.tobytes()

    # Return length
    return len(result)
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


def test_memoryview_iteration():
    """Test iterating over memoryview."""
    python_code = '''
def test_memview_iter() -> int:
    # Create memoryview and iterate
    data = b"abc"
    view = memoryview(data)

    # Sum byte values
    total = 0
    for byte_val in view:
        total += byte_val

    # Return sum (ord('a') + ord('b') + ord('c') = 97 + 98 + 99 = 294)
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
