"""ISBN validation using digit arrays.

Tests: ISBN-10 check digit, ISBN-13 check digit, digit sum.
"""


def isbn10_check_digit(digits: list[int]) -> int:
    """Compute ISBN-10 check digit from first 9 digits.
    Returns check digit 0-10 (10 represents X)."""
    total: int = 0
    i: int = 0
    while i < 9:
        total = total + digits[i] * (10 - i)
        i = i + 1
    remainder: int = total % 11
    check: int = 11 - remainder
    if check == 11:
        check = 0
    return check


def isbn13_check_digit(digits: list[int]) -> int:
    """Compute ISBN-13 check digit from first 12 digits."""
    total: int = 0
    i: int = 0
    while i < 12:
        if i % 2 == 0:
            total = total + digits[i]
        else:
            total = total + digits[i] * 3
        i = i + 1
    remainder: int = total % 10
    check: int = 10 - remainder
    if check == 10:
        check = 0
    return check


def digit_weighted_sum(digits: list[int], weights: list[int]) -> int:
    """Compute weighted sum of digits."""
    total: int = 0
    n: int = len(digits)
    if len(weights) < n:
        n = len(weights)
    i: int = 0
    while i < n:
        total = total + digits[i] * weights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test ISBN operations."""
    ok: int = 0
    d9: list[int] = [0, 3, 0, 6, 4, 0, 6, 1, 5]
    check10: int = isbn10_check_digit(d9)
    if check10 == 2:
        ok = ok + 1
    d12: list[int] = [9, 7, 8, 0, 3, 0, 6, 4, 0, 6, 1, 5]
    check13: int = isbn13_check_digit(d12)
    if check13 == 7:
        ok = ok + 1
    w: list[int] = [1, 2, 3]
    d: list[int] = [4, 5, 6]
    if digit_weighted_sum(d, w) == 32:
        ok = ok + 1
    return ok
