"""
TDD tests for statistics module examples.

Tests verify that all Python examples in the statistics.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_statistics_mean():
    """Test statistics.mean() for arithmetic mean."""
    python_code = '''
import statistics

def stats_mean() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate mean (average)
    avg = statistics.mean(data)  # 3.0

    return avg
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


def test_statistics_median():
    """Test statistics.median() for middle value."""
    python_code = '''
import statistics

def stats_median() -> float:
    # Odd number of values
    data1: list[int] = [1, 2, 3, 4, 5]
    median1 = statistics.median(data1)  # 3

    # Even number of values
    data2: list[int] = [1, 2, 3, 4]
    median2 = statistics.median(data2)  # 2.5

    return median1
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


def test_statistics_mode():
    """Test statistics.mode() for most common value."""
    python_code = '''
import statistics

def stats_mode() -> int:
    data: list[int] = [1, 2, 2, 3, 3, 3, 4]

    # Find most common value
    most_common = statistics.mode(data)  # 3

    return most_common
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


def test_statistics_stdev():
    """Test statistics.stdev() for standard deviation."""
    python_code = '''
import statistics

def stats_stdev() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate standard deviation
    std = statistics.stdev(data)

    return std
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


def test_statistics_variance():
    """Test statistics.variance() calculation."""
    python_code = '''
import statistics

def stats_variance() -> float:
    data: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate variance
    var = statistics.variance(data)

    return var
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


def test_statistics_quantiles():
    """Test statistics.quantiles() for percentiles."""
    python_code = '''
import statistics

def stats_quantiles() -> list[float]:
    data: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Calculate quartiles (4-quantiles)
    quartiles = statistics.quantiles(data, n=4)

    return quartiles
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
