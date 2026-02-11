"""Base conversion variant operations.

Implements conversions between various number bases
using integer arithmetic.
"""


def to_base(n: int, base: int) -> list[int]:
    """Convert non-negative integer to given base. Returns digits list (least significant first)."""
    if n == 0:
        return [0]
    digits: list[int] = []
    remaining: int = n
    while remaining > 0:
        digit: int = remaining % base
        digits.append(digit)
        remaining = remaining // base
    return digits


def from_base(digits: list[int], size: int, base: int) -> int:
    """Convert digits in given base (least significant first) to integer."""
    result: int = 0
    multiplier: int = 1
    i: int = 0
    while i < size:
        result = result + digits[i] * multiplier
        multiplier = multiplier * base
        i = i + 1
    return result


def base_convert(n: int, from_base_val: int, to_base_val: int) -> list[int]:
    """Convert n from one base to another. N is in base-10. Returns digits in target base."""
    tmp_digits: list[int] = to_base(n, to_base_val)
    return tmp_digits


def count_digits_in_base(n: int, base: int) -> int:
    """Count how many digits n has in given base."""
    if n == 0:
        return 1
    count: int = 0
    remaining: int = n
    while remaining > 0:
        count = count + 1
        remaining = remaining // base
    return count


def is_palindrome_in_base(n: int, base: int) -> int:
    """Check if n is a palindrome when written in given base. Returns 1 if yes."""
    tmp_digits: list[int] = to_base(n, base)
    size: int = len(tmp_digits)
    i: int = 0
    while i < size // 2:
        other: int = size - 1 - i
        if tmp_digits[i] != tmp_digits[other]:
            return 0
        i = i + 1
    return 1


def sum_digits_in_base(n: int, base: int) -> int:
    """Sum all digits of n in given base."""
    total: int = 0
    remaining: int = n
    if remaining == 0:
        return 0
    while remaining > 0:
        total = total + remaining % base
        remaining = remaining // base
    return total


def test_module() -> int:
    """Test base conversion operations."""
    ok: int = 0

    tmp_bin: list[int] = to_base(10, 2)
    if tmp_bin[0] == 0 and tmp_bin[1] == 1 and tmp_bin[2] == 0 and tmp_bin[3] == 1:
        ok = ok + 1

    digits: list[int] = [0, 1, 0, 1]
    val: int = from_base(digits, 4, 2)
    if val == 10:
        ok = ok + 1

    dc: int = count_digits_in_base(255, 16)
    if dc == 2:
        ok = ok + 1

    pal: int = is_palindrome_in_base(9, 2)
    if pal == 1:
        ok = ok + 1

    sd: int = sum_digits_in_base(123, 10)
    if sd == 6:
        ok = ok + 1

    return ok
