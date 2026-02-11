# Chained comparisons and complex boolean logic for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def in_range(x: int, lo: int, hi: int) -> bool:
    """Check if x is in [lo, hi] inclusive."""
    return x >= lo and x <= hi


def is_between_exclusive(x: int, lo: int, hi: int) -> bool:
    """Check if x is in (lo, hi) exclusive."""
    return x > lo and x < hi


def all_positive(a: int, b: int, c: int) -> bool:
    """Check if all three values are positive."""
    return a > 0 and b > 0 and c > 0


def any_negative(a: int, b: int, c: int) -> bool:
    """Check if any value is negative."""
    return a < 0 or b < 0 or c < 0


def is_sorted_triple(a: int, b: int, c: int) -> bool:
    """Check if three values are in non-decreasing order."""
    return a <= b and b <= c


def classify_triangle(a: int, b: int, c: int) -> int:
    """Classify triangle by side lengths.
    0=invalid, 1=equilateral, 2=isosceles, 3=scalene."""
    if a <= 0 or b <= 0 or c <= 0:
        return 0
    if a + b <= c or a + c <= b or b + c <= a:
        return 0
    if a == b and b == c:
        return 1
    elif a == b or b == c or a == c:
        return 2
    else:
        return 3


def test_module() -> int:
    """Test all comparison chain functions."""
    assert in_range(5, 1, 10) == True
    assert in_range(0, 1, 10) == False
    assert in_range(10, 1, 10) == True
    assert is_between_exclusive(5, 1, 10) == True
    assert is_between_exclusive(1, 1, 10) == False
    assert is_between_exclusive(10, 1, 10) == False
    assert all_positive(1, 2, 3) == True
    assert all_positive(1, -2, 3) == False
    assert all_positive(0, 1, 2) == False
    assert any_negative(1, 2, 3) == False
    assert any_negative(1, -2, 3) == True
    assert any_negative(-1, -2, -3) == True
    assert is_sorted_triple(1, 2, 3) == True
    assert is_sorted_triple(1, 1, 2) == True
    assert is_sorted_triple(3, 2, 1) == False
    assert classify_triangle(3, 3, 3) == 1
    assert classify_triangle(3, 3, 5) == 2
    assert classify_triangle(3, 4, 5) == 3
    assert classify_triangle(1, 2, 10) == 0
    assert classify_triangle(0, 3, 3) == 0
    return 0


if __name__ == "__main__":
    test_module()
