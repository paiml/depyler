"""Roman numeral conversion operations.

Tests: integer to roman value, roman digit values, numeral validation.
"""


def int_to_roman_value(num: int) -> int:
    """Convert integer to sum of roman digit weights as verification.
    Returns the input if valid roman range, else -1."""
    if num <= 0:
        return -1
    if num > 3999:
        return -1
    return num


def count_roman_digits(num: int) -> int:
    """Count how many roman numeral characters needed to represent num."""
    if num <= 0:
        return 0
    count: int = 0
    remaining: int = num
    values: list[int] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
    i: int = 0
    while i < len(values):
        while remaining >= values[i]:
            count = count + 1
            remaining = remaining - values[i]
        i = i + 1
    return count


def roman_digit_sum(num: int) -> int:
    """Sum of individual roman digit values (e.g., XIV = 10+1+5 = 16, but value is 14).
    This returns the raw sum without subtractive rule."""
    if num <= 0:
        return 0
    total: int = 0
    remaining: int = num
    values: list[int] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
    digit_sums: list[int] = [1000, 1100, 500, 600, 100, 110, 50, 60, 10, 11, 5, 6, 1]
    i: int = 0
    while i < len(values):
        while remaining >= values[i]:
            total = total + digit_sums[i]
            remaining = remaining - values[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test roman numeral operations."""
    ok: int = 0
    if int_to_roman_value(14) == 14:
        ok = ok + 1
    if int_to_roman_value(0) == -1:
        ok = ok + 1
    if count_roman_digits(3) == 3:
        ok = ok + 1
    if count_roman_digits(4) == 2:
        ok = ok + 1
    if count_roman_digits(9) == 2:
        ok = ok + 1
    if roman_digit_sum(4) == 6:
        ok = ok + 1
    return ok
