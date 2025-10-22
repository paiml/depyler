"""
TDD tests for random module examples.

Tests verify that all Python examples in the random.md book chapter
transpile to valid Rust and execute correctly.
"""
import subprocess
import tempfile
from pathlib import Path


def test_random_basic_functions():
    """Test basic random functions: random, randint, choice."""
    python_code = '''
import random

def random_basic() -> float:
    # Random float [0.0, 1.0)
    rand_val = random.random()

    # Random integer [1, 10]
    rand_int = random.randint(1, 10)

    # Random choice from list
    choices: list[str] = ["apple", "banana", "cherry"]
    choice = random.choice(choices)

    return rand_val
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


def test_random_uniform():
    """Test random.uniform() for floating-point ranges."""
    python_code = '''
import random

def random_uniform() -> float:
    # Random float in range [1.0, 10.0]
    val = random.uniform(1.0, 10.0)
    return val
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


def test_random_shuffle():
    """Test random.shuffle() for list randomization."""
    python_code = '''
import random

def random_shuffle() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5]
    random.shuffle(numbers)
    return numbers
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


def test_random_seed():
    """Test random.seed() for reproducible randomness."""
    python_code = '''
import random

def random_seed() -> float:
    random.seed(42)
    val = random.random()
    return val
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


def test_random_sample():
    """Test random.sample() for sampling without replacement."""
    python_code = '''
import random

def random_sample() -> list[int]:
    numbers: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sample = random.sample(numbers, 3)
    return sample
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
