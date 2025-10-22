"""
TDD tests for fractions module examples.

Tests verify that all Python examples in the fractions.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_fraction_basic_arithmetic():
    """Test basic fraction arithmetic operations."""
    python_code = '''
from fractions import Fraction

def fraction_basic() -> Fraction:
    a: Fraction = Fraction(1, 2)  # 1/2
    b: Fraction = Fraction(1, 3)  # 1/3

    # Basic arithmetic
    sum_val = a + b     # 5/6
    diff_val = a - b    # 1/6
    prod_val = a * b    # 1/6
    quot_val = a / b    # 3/2

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


def test_fraction_simplification():
    """Test automatic fraction simplification."""
    python_code = '''
from fractions import Fraction

def fraction_simplify() -> Fraction:
    # Fractions are automatically simplified
    f1: Fraction = Fraction(6, 9)    # Simplifies to 2/3
    f2: Fraction = Fraction(10, 15)  # Simplifies to 2/3

    # They are equal after simplification
    equal = (f1 == f2)  # True

    return f1
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


def test_fraction_comparison():
    """Test fraction comparison operations."""
    python_code = '''
from fractions import Fraction

def fraction_comparison() -> bool:
    a: Fraction = Fraction(1, 2)
    b: Fraction = Fraction(2, 4)  # Same as 1/2
    c: Fraction = Fraction(3, 4)

    # Comparisons
    equal = (a == b)        # True
    less = (a < c)          # True
    greater = (c > a)       # True

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


def test_fraction_from_decimal():
    """Test creating fractions from floats and decimals."""
    python_code = '''
from fractions import Fraction

def fraction_from_decimal() -> Fraction:
    # From float (exact representation)
    f1: Fraction = Fraction(0.5)  # 1/2

    # From string (preferred for decimals)
    f2: Fraction = Fraction("0.333")  # 333/1000

    # Limit denominator
    f3: Fraction = Fraction("0.333").limit_denominator(10)  # 1/3

    return f3
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


def test_fraction_conversion():
    """Test converting fractions to other numeric types."""
    python_code = '''
from fractions import Fraction

def fraction_conversion() -> float:
    f: Fraction = Fraction(1, 4)

    # Convert to float
    float_val: float = float(f)  # 0.25

    # Convert to string
    str_val: str = str(f)  # "1/4"

    # Access numerator and denominator
    num: int = f.numerator    # 1
    denom: int = f.denominator  # 4

    return float_val
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
