"""
TDD tests for csv module - CSV file reading and writing.

Tests verify that all Python examples using csv (reader, writer, DictReader, DictWriter)
transpile to valid Rust and execute correctly. The csv module provides CSV serialization.
"""
import subprocess
import tempfile
from pathlib import Path as PyPath


def test_csv_reader_basic():
    """Test csv.reader() with basic CSV data."""
    python_code = '''
import csv
import io

def test_reader() -> int:
    # CSV data
    data = "a,b,c\\n1,2,3\\n4,5,6"

    # Read CSV
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)

    # Count total cells
    total = 0
    for row in rows:
        total += len(row)

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


def test_csv_writer_basic():
    """Test csv.writer() basic write operations."""
    python_code = '''
import csv
import io

def test_writer() -> str:
    # Create CSV writer
    output = io.StringIO()
    writer = csv.writer(output)

    # Write rows
    writer.writerow(["name", "age"])
    writer.writerow(["Alice", "30"])
    writer.writerow(["Bob", "25"])

    # Get CSV string
    result = output.getvalue()

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


def test_csv_dictreader():
    """Test csv.DictReader() for reading with headers."""
    python_code = '''
import csv
import io

def test_dictreader() -> int:
    # CSV data with header
    data = "name,age\\nAlice,30\\nBob,25"

    # Read as dictionaries
    reader = csv.DictReader(io.StringIO(data))

    # Sum ages
    total_age = 0
    for row in reader:
        total_age += int(row["age"])

    return total_age
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


def test_csv_dictwriter():
    """Test csv.DictWriter() for writing dictionaries."""
    python_code = '''
import csv
import io

def test_dictwriter() -> str:
    # Create DictWriter
    output = io.StringIO()
    fieldnames = ["name", "age"]
    writer = csv.DictWriter(output, fieldnames=fieldnames)

    # Write header and rows
    writer.writeheader()
    writer.writerow({"name": "Alice", "age": "30"})
    writer.writerow({"name": "Bob", "age": "25"})

    # Get CSV string
    result = output.getvalue()

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


def test_csv_custom_delimiter():
    """Test CSV with custom delimiter."""
    python_code = '''
import csv
import io

def test_custom_delimiter() -> int:
    # TSV data (tab-separated)
    data = "a\\tb\\tc\\n1\\t2\\t3"

    # Read with tab delimiter
    reader = csv.reader(io.StringIO(data), delimiter="\\t")
    rows = list(reader)

    # Count rows
    return len(rows)
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


def test_csv_quoted_fields():
    """Test CSV with quoted fields."""
    python_code = '''
import csv
import io

def test_quoted() -> str:
    # CSV with quoted field containing comma
    data = '"Hello, World",123,test'

    # Read CSV
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)

    # Return first field
    return rows[0][0]
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
