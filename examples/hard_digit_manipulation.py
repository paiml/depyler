"""Digit manipulation operations: extract, reverse, and transform digits."""


def extract_digits(n: int) -> list[int]:
    """Extract all digits from a number into a list."""
    if n == 0:
        result: list[int] = [0]
        return result
    negative: int = 0
    if n < 0:
        negative = 1
        n = -n
    digits: list[int] = []
    while n > 0:
        remainder: int = n % 10
        digits.append(remainder)
        n = n // 10
    # Reverse the digits
    left: int = 0
    right_idx: int = len(digits) - 1
    while left < right_idx:
        temp: int = digits[left]
        digits[left] = digits[right_idx]
        digits[right_idx] = temp
        left = left + 1
        right_idx = right_idx - 1
    if negative == 1:
        digits[0] = -digits[0]
    result2: list[int] = digits
    return result2


def sum_digit_squares(n: int) -> int:
    """Sum of squares of each digit."""
    if n < 0:
        n = -n
    total: int = 0
    while n > 0:
        d: int = n % 10
        total = total + d * d
        n = n // 10
    return total


def rotate_digits_left(n: int, positions: int) -> int:
    """Rotate digits of a number to the left by given positions."""
    if n <= 0:
        return n
    digits: list[int] = []
    temp_n: int = n
    while temp_n > 0:
        digits.append(temp_n % 10)
        temp_n = temp_n // 10
    # Reverse to get correct order
    left: int = 0
    right_idx: int = len(digits) - 1
    while left < right_idx:
        swap: int = digits[left]
        digits[left] = digits[right_idx]
        digits[right_idx] = swap
        left = left + 1
        right_idx = right_idx - 1
    count: int = len(digits)
    if count == 0:
        return 0
    pos: int = positions % count
    result: int = 0
    i: int = 0
    while i < count:
        src: int = (i + pos) % count
        result = result * 10 + digits[src]
        i = i + 1
    return result


def count_digit_occurrences(n: int, digit: int) -> int:
    """Count how many times a digit appears in a number."""
    if n == 0 and digit == 0:
        return 1
    if n < 0:
        n = -n
    count: int = 0
    while n > 0:
        if n % 10 == digit:
            count = count + 1
        n = n // 10
    return count


def test_module() -> int:
    """Test digit manipulation functions."""
    ok: int = 0

    d1: list[int] = extract_digits(123)
    if len(d1) == 3 and d1[0] == 1 and d1[1] == 2 and d1[2] == 3:
        ok = ok + 1

    d2: list[int] = extract_digits(0)
    if len(d2) == 1 and d2[0] == 0:
        ok = ok + 1

    if sum_digit_squares(123) == 14:
        ok = ok + 1

    if sum_digit_squares(99) == 162:
        ok = ok + 1

    if rotate_digits_left(1234, 1) == 2341:
        ok = ok + 1

    if count_digit_occurrences(11211, 1) == 4:
        ok = ok + 1

    if count_digit_occurrences(0, 0) == 1:
        ok = ok + 1

    return ok
