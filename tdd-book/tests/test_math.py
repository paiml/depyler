"""
TDD tests for math module examples.

Tests verify that all Python examples in the math.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_math_basic_functions():
    """Test basic math functions: sqrt, abs, ceil, floor."""
    python_code = '''
import math

def math_basic() -> float:
    x: float = 16.7
    sqrt_val = math.sqrt(16.0)
    abs_val = math.fabs(-5.5)
    ceil_val = math.ceil(x)
    floor_val = math.floor(x)
    return sqrt_val + abs_val
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


def test_math_power_functions():
    """Test power and exponential functions: pow, exp, log."""
    python_code = '''
import math

def math_power() -> float:
    base: float = 2.0
    exponent: float = 3.0
    power_val = math.pow(base, exponent)
    exp_val = math.exp(1.0)
    log_val = math.log(10.0)
    return power_val
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


def test_math_trigonometric():
    """Test trigonometric functions: sin, cos, tan."""
    python_code = '''
import math

def math_trig() -> float:
    angle: float = math.pi / 4.0
    sin_val = math.sin(angle)
    cos_val = math.cos(angle)
    tan_val = math.tan(angle)
    return sin_val
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


def test_math_constants():
    """Test math constants: pi, e."""
    python_code = '''
import math

def math_constants() -> float:
    pi_val: float = math.pi
    e_val: float = math.e
    tau_val: float = math.tau
    return pi_val + e_val
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


def test_math_rounding():
    """Test rounding functions: round, trunc."""
    python_code = '''
import math

def math_rounding() -> float:
    x: float = 3.7
    y: float = -2.3
    ceil_x = math.ceil(x)
    floor_y = math.floor(y)
    trunc_x = math.trunc(x)
    return float(ceil_x)
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


def test_math_hyperbolic():
    """Test hyperbolic functions: sinh, cosh, tanh."""
    python_code = '''
import math

def math_hyperbolic() -> float:
    x: float = 1.0
    sinh_val = math.sinh(x)
    cosh_val = math.cosh(x)
    tanh_val = math.tanh(x)
    return sinh_val
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
