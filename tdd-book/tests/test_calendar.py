"""
TDD tests for calendar module examples.

Tests verify that all Python examples in the calendar.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_calendar_weekday():
    """Test calendar.weekday() to get day of week."""
    python_code = '''
import calendar

def get_weekday() -> int:
    # Get weekday for January 1, 2000 (Saturday = 5)
    day = calendar.weekday(2000, 1, 1)

    return day
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


def test_calendar_isleap():
    """Test calendar.isleap() for leap year detection."""
    python_code = '''
import calendar

def check_leap_year() -> bool:
    # Check if 2000 is a leap year (it is)
    is_leap = calendar.isleap(2000)

    # Check if 2001 is a leap year (it's not)
    not_leap = calendar.isleap(2001)

    return is_leap and not not_leap
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


def test_calendar_leapdays():
    """Test calendar.leapdays() to count leap days in range."""
    python_code = '''
import calendar

def count_leap_days() -> int:
    # Count leap days between 2000 and 2020
    count = calendar.leapdays(2000, 2020)

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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)


def test_calendar_monthrange():
    """Test calendar.monthrange() for month info."""
    python_code = '''
import calendar

def get_month_info() -> int:
    # Get info for January 2000
    # Returns (weekday of first day, number of days)
    first_weekday, num_days = calendar.monthrange(2000, 1)

    # January has 31 days
    return num_days
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


def test_calendar_monthcalendar():
    """Test calendar.monthcalendar() for calendar matrix."""
    python_code = '''
import calendar

def get_month_calendar() -> int:
    # Get calendar matrix for October 2025
    # Returns list of weeks, each week is list of 7 days
    cal = calendar.monthcalendar(2025, 10)

    # Count non-zero days using traditional loops
    count = 0
    for week in cal:
        for day in week:
            if day != 0:
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
        assert 'Properties Verified' in result.stdout, "Verification failed"

    finally:
        Path(py_file).unlink(missing_ok=True)
