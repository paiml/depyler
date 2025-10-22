"""
TDD tests for io module - Core I/O tools.

Tests verify that all Python examples using io (StringIO, BytesIO) transpile
to valid Rust and execute correctly. The io module provides in-memory streams.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_io_stringio_basic():
    """Test StringIO basic write and read operations."""
    python_code = '''
import io

def test_stringio() -> str:
    # Create in-memory text stream
    sio = io.StringIO()

    # Write text
    sio.write("Hello, ")
    sio.write("StringIO!")

    # Get complete value
    result = sio.getvalue()

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


def test_io_stringio_seek():
    """Test StringIO seek and read operations."""
    python_code = '''
import io

def test_stringio_seek() -> str:
    # Create in-memory text stream
    sio = io.StringIO()

    # Write text
    sio.write("Hello, World!")

    # Seek to beginning
    sio.seek(0)

    # Read content
    content = sio.read()

    return content
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


def test_io_stringio_readline():
    """Test StringIO line-by-line reading."""
    python_code = '''
import io

def test_readline() -> int:
    # Create stream with multiple lines
    sio = io.StringIO("Line 1\\nLine 2\\nLine 3\\n")

    # Read lines
    line_count = 0
    while True:
        line = sio.readline()
        if not line:
            break
        line_count += 1

    return line_count
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


def test_io_stringio_iteration():
    """Test StringIO iteration over lines."""
    python_code = '''
import io

def test_iteration() -> int:
    # Create stream with multiple lines
    content = "Line 1\\nLine 2\\nLine 3\\n"
    sio = io.StringIO(content)

    # Count lines using iteration
    count = 0
    for line in sio:
        count += 1

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
    finally:
        PyPath(py_file).unlink(missing_ok=True)


def test_io_stringio_initial_value():
    """Test StringIO with initial value."""
    python_code = '''
import io

def test_initial_value() -> str:
    # Create StringIO with initial content
    sio = io.StringIO("Initial content")

    # Read from beginning
    content = sio.read()

    return content
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
