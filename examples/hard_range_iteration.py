# For loops with ranges and step values for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def sum_range(start: int, end: int) -> int:
    """Sum integers from start to end (exclusive)."""
    total: int = 0
    i: int = start
    while i < end:
        total = total + i
        i = i + 1
    return total


def sum_range_step(start: int, end: int, step: int) -> int:
    """Sum integers from start to end with a given step."""
    if step <= 0:
        return 0
    total: int = 0
    i: int = start
    while i < end:
        total = total + i
        i = i + step
    return total


def count_divisible(start: int, end: int, divisor: int) -> int:
    """Count numbers in range divisible by divisor."""
    if divisor == 0:
        return 0
    count: int = 0
    i: int = start
    while i < end:
        if i % divisor == 0:
            count = count + 1
        i = i + 1
    return count


def triangular_number(n: int) -> int:
    """Compute the nth triangular number."""
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + i
        i = i + 1
    return total


def harmonic_sum_scaled(n: int, scale: int) -> int:
    """Compute scaled harmonic sum (scale/1 + scale/2 + ... + scale/n)."""
    if n <= 0 or scale <= 0:
        return 0
    total: int = 0
    i: int = 1
    while i <= n:
        total = total + scale // i
        i = i + 1
    return total


def test_module() -> int:
    """Test all range iteration functions."""
    assert sum_range(1, 6) == 15
    assert sum_range(0, 0) == 0
    assert sum_range(5, 5) == 0
    assert sum_range_step(0, 10, 2) == 20
    assert sum_range_step(1, 10, 3) == 12
    assert sum_range_step(0, 10, 0) == 0
    assert count_divisible(1, 20, 3) == 6
    assert count_divisible(1, 10, 2) == 4
    assert count_divisible(1, 10, 0) == 0
    assert triangular_number(1) == 1
    assert triangular_number(5) == 15
    assert triangular_number(100) == 5050
    assert harmonic_sum_scaled(4, 1200) == 2500
    return 0


if __name__ == "__main__":
    test_module()
