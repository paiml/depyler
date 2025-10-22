"""
TDD tests for decimal module examples.

Tests verify that all Python examples in the decimal.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_decimal_basic_arithmetic():
    """Test basic decimal arithmetic operations."""
    python_code = '''
from decimal import Decimal

def decimal_basic() -> Decimal:
    a: Decimal = Decimal("10.5")
    b: Decimal = Decimal("2.3")

    # Basic arithmetic
    sum_val = a + b
    diff_val = a - b
    prod_val = a * b
    quot_val = a / b

    return sum_val
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


def test_decimal_precision():
    """Test decimal precision control."""
    python_code = '''
from decimal import Decimal, getcontext

def decimal_precision() -> Decimal:
    # Set precision
    getcontext().prec = 10

    a: Decimal = Decimal("1.0")
    b: Decimal = Decimal("3.0")

    # Division with controlled precision
    result = a / b

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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_decimal_comparison():
    """Test decimal comparison operations."""
    python_code = '''
from decimal import Decimal

def decimal_comparison() -> bool:
    a: Decimal = Decimal("10.5")
    b: Decimal = Decimal("10.50")
    c: Decimal = Decimal("10.51")

    # Comparisons
    equal = (a == b)
    less = (a < c)
    greater = (c > a)

    return equal and less and greater
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


def test_decimal_rounding():
    """Test decimal rounding modes."""
    python_code = '''
from decimal import Decimal

def decimal_rounding() -> Decimal:
    a: Decimal = Decimal("10.567")

    # Quantize (round to 2 decimal places)
    rounded = a.quantize(Decimal("0.01"))

    return rounded
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


def test_decimal_string_conversion():
    """Test decimal to/from string conversion."""
    python_code = '''
from decimal import Decimal

def decimal_string_conversion() -> str:
    # Create from string
    a: Decimal = Decimal("123.456")

    # Convert to string
    s: str = str(a)

    return s
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
