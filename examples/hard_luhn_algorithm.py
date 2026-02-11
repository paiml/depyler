"""Luhn algorithm for credit card validation.

Tests: luhn check, generate check digit, double-and-sum.
"""


def luhn_double_digit(d: int) -> int:
    """Double a digit and subtract 9 if result > 9."""
    doubled: int = d * 2
    if doubled > 9:
        doubled = doubled - 9
    return doubled


def luhn_checksum(digits: list[int]) -> int:
    """Compute Luhn checksum of digit array."""
    total: int = 0
    n: int = len(digits)
    parity: int = n % 2
    i: int = 0
    while i < n:
        d: int = digits[i]
        if i % 2 == parity:
            d = luhn_double_digit(d)
        total = total + d
        i = i + 1
    return total % 10


def luhn_valid(digits: list[int]) -> int:
    """Return 1 if digits pass Luhn check, 0 otherwise."""
    if luhn_checksum(digits) == 0:
        return 1
    return 0


def compute_check_digit(digits: list[int]) -> int:
    """Compute check digit to append to make Luhn valid."""
    extended: list[int] = []
    i: int = 0
    while i < len(digits):
        extended.append(digits[i])
        i = i + 1
    extended.append(0)
    cs: int = luhn_checksum(extended)
    if cs == 0:
        return 0
    return 10 - cs


def test_module() -> int:
    """Test Luhn algorithm operations."""
    ok: int = 0
    if luhn_double_digit(6) == 3:
        ok = ok + 1
    if luhn_double_digit(4) == 8:
        ok = ok + 1
    valid_card: list[int] = [4, 5, 3, 9, 1, 4, 8, 8, 0, 3, 4, 3, 6, 4, 6, 7]
    if luhn_valid(valid_card) == 1:
        ok = ok + 1
    partial: list[int] = [7, 9, 9, 2, 7, 3, 9, 8, 7, 1]
    cd: int = compute_check_digit(partial)
    full: list[int] = []
    j: int = 0
    while j < len(partial):
        full.append(partial[j])
        j = j + 1
    full.append(cd)
    if luhn_valid(full) == 1:
        ok = ok + 1
    return ok
