"""Various check digit algorithms.

Tests: Luhn checksum, ISBN-10 check, digit root check, weighted checksum.
"""


def luhn_checksum(digits: list[int]) -> int:
    """Compute Luhn checksum from list of digits."""
    total: int = 0
    n: int = len(digits)
    parity: int = n % 2
    i: int = 0
    while i < n:
        d: int = digits[i]
        if i % 2 == parity:
            d = d * 2
            if d > 9:
                d = d - 9
        total = total + d
        i = i + 1
    return total % 10


def luhn_valid(digits: list[int]) -> int:
    """Check if digit sequence passes Luhn validation. Returns 1 or 0."""
    if luhn_checksum(digits) == 0:
        return 1
    return 0


def weighted_checksum(digits: list[int], weights: list[int]) -> int:
    """Compute weighted checksum: sum(digit[i] * weight[i]) mod 10."""
    total: int = 0
    n: int = len(digits)
    i: int = 0
    while i < n:
        total = total + digits[i] * weights[i]
        i = i + 1
    return total % 10


def isbn10_check_digit(digits9: list[int]) -> int:
    """Compute ISBN-10 check digit from first 9 digits. Returns 0-10."""
    total: int = 0
    i: int = 0
    while i < 9:
        total = total + digits9[i] * (10 - i)
        i = i + 1
    remainder: int = total % 11
    check: int = 11 - remainder
    if check == 11:
        check = 0
    return check


def verhoeff_digit_sum(n: int) -> int:
    """Simple alternating digit sum for verification."""
    val: int = n
    if val < 0:
        val = -val
    total: int = 0
    pos: int = 0
    while val > 0:
        d: int = val % 10
        if pos % 2 == 0:
            total = total + d
        else:
            total = total - d
        val = val // 10
        pos = pos + 1
    if total < 0:
        total = -total
    return total % 10


def test_module() -> int:
    """Test check digit operations."""
    ok: int = 0
    if luhn_checksum([7, 9, 9, 2, 7, 3, 9, 8, 7, 1]) == 0:
        ok = ok + 1
    if luhn_valid([7, 9, 9, 2, 7, 3, 9, 8, 7, 1]) == 1:
        ok = ok + 1
    w: list[int] = [1, 2, 1, 2, 1]
    if weighted_checksum([1, 2, 3, 4, 5], w) == 5:
        ok = ok + 1
    isbn9: list[int] = [0, 3, 0, 6, 4, 0, 6, 1, 5]
    check: int = isbn10_check_digit(isbn9)
    if check == 2:
        ok = ok + 1
    if verhoeff_digit_sum(1234) == 2:
        ok = ok + 1
    return ok
