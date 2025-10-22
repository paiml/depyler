"""
TDD tests for datetime module examples.

Tests verify that all Python examples in the datetime.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_datetime_creation():
    """Test creating datetime objects."""
    python_code = '''
from datetime import datetime

def create_datetime() -> datetime:
    # Create datetime object
    dt = datetime(2024, 10, 22, 14, 30, 0)  # Oct 22, 2024, 2:30 PM

    return dt
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


def test_datetime_now():
    """Test getting current datetime."""
    python_code = '''
from datetime import datetime

def get_current_time() -> datetime:
    # Get current datetime
    now = datetime.now()

    return now
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


def test_datetime_formatting():
    """Test formatting datetime to string."""
    python_code = '''
from datetime import datetime

def format_datetime() -> str:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Format datetime to string
    formatted = dt.strftime("%Y-%m-%d %H:%M:%S")  # "2024-10-22 14:30:00"

    return formatted
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


def test_datetime_parsing():
    """Test parsing datetime from string."""
    python_code = '''
from datetime import datetime

def parse_datetime() -> datetime:
    # Parse datetime from string
    dt = datetime.strptime("2024-10-22 14:30:00", "%Y-%m-%d %H:%M:%S")

    return dt
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


def test_datetime_arithmetic():
    """Test datetime arithmetic with timedelta."""
    python_code = '''
from datetime import datetime, timedelta

def datetime_arithmetic() -> datetime:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Add 7 days
    future_dt = dt + timedelta(days=7)

    return future_dt
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


def test_datetime_components():
    """Test accessing datetime components."""
    python_code = '''
from datetime import datetime

def datetime_components() -> int:
    dt = datetime(2024, 10, 22, 14, 30, 0)

    # Access individual components
    year: int = dt.year      # 2024
    month: int = dt.month    # 10
    day: int = dt.day        # 22
    hour: int = dt.hour      # 14
    minute: int = dt.minute  # 30

    return year
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
