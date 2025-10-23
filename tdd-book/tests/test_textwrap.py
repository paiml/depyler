"""
TDD tests for textwrap module - Text wrapping and formatting.

Tests verify that all Python examples using textwrap.wrap(), textwrap.fill(), etc.
transpile to valid Rust and execute correctly. The textwrap module provides text formatting.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_textwrap_wrap():
    """Test textwrap.wrap() for line splitting."""
    python_code = '''
import textwrap

def test_wrap() -> int:
    # Wrap text to multiple lines
    text = "This is a very long line of text that needs to be wrapped"
    lines = textwrap.wrap(text, width=20)

    # Return number of lines
    return len(lines)
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


def test_textwrap_fill():
    """Test textwrap.fill() for wrapping and joining."""
    python_code = '''
import textwrap

def test_fill() -> int:
    # Fill wraps and joins with newlines
    text = "This is a very long line of text that needs to be wrapped"
    result = textwrap.fill(text, width=20)

    # Return length of result
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


def test_textwrap_shorten():
    """Test textwrap.shorten() for text shortening."""
    python_code = '''
import textwrap

def test_shorten() -> int:
    # Shorten text to width
    text = "This is a very long line of text"
    result = textwrap.shorten(text, width=20)

    # Return length of shortened text
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


def test_textwrap_dedent():
    """Test textwrap.dedent() for removing indentation."""
    python_code = '''
import textwrap

def test_dedent() -> int:
    # Remove common indentation
    text = "    hello\\n    world"
    result = textwrap.dedent(text)

    # Return length of dedented text
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


def test_textwrap_indent():
    """Test textwrap.indent() for adding prefix."""
    python_code = '''
import textwrap

def test_indent() -> int:
    # Add prefix to all lines
    text = "hello\\nworld"
    result = textwrap.indent(text, "> ")

    # Return length of indented text
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


def test_textwrap_textwrapper():
    """Test textwrap.TextWrapper class."""
    python_code = '''
import textwrap

def test_wrapper() -> int:
    # Create and use TextWrapper
    wrapper = textwrap.TextWrapper(width=20)
    text = "This is a very long line of text"
    lines = wrapper.wrap(text)

    # Return number of lines
    return len(lines)
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
