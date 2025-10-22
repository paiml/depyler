"""
TDD tests for time module examples.

Tests verify that all Python examples in the time.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_time_timestamp():
    """Test getting current Unix timestamp."""
    python_code = '''
import time

def get_timestamp() -> float:
    # Get current Unix timestamp
    timestamp = time.time()

    return timestamp
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


def test_time_sleep():
    """Test time.sleep() for pausing execution."""
    python_code = '''
import time

def sleep_example() -> float:
    start = time.time()

    # Sleep for 0.1 seconds
    time.sleep(0.1)

    end = time.time()
    elapsed = end - start

    return elapsed
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


def test_time_perf_counter():
    """Test high-resolution performance counter."""
    python_code = '''
import time

def measure_performance() -> float:
    # Start performance counter
    start = time.perf_counter()

    # Some operation
    result = sum(range(1000))

    # End performance counter
    end = time.perf_counter()
    elapsed = end - start

    return elapsed
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


def test_time_monotonic():
    """Test monotonic clock (always increases)."""
    python_code = '''
import time

def monotonic_example() -> float:
    # Get monotonic time
    start = time.monotonic()

    # Some operation
    result = sum(range(100))

    end = time.monotonic()
    elapsed = end - start

    return elapsed
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


def test_time_process_time():
    """Test process CPU time measurement."""
    python_code = '''
import time

def measure_cpu_time() -> float:
    # Measure CPU time used by process
    start = time.process_time()

    # CPU-intensive operation
    result = sum(range(10000))

    end = time.process_time()
    cpu_time = end - start

    return cpu_time
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
