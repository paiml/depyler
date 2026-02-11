"""Checksum algorithms: Luhn, parity, simple hash.

Tests: digit extraction, modular arithmetic, list traversal.
"""


def luhn_checksum(digits: list[int]) -> int:
    """Compute Luhn checksum for a list of digits."""
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
        total += d
        i += 1
    return total % 10


def check_luhn(digits: list[int]) -> bool:
    """Check if digit sequence passes Luhn check."""
    cs: int = luhn_checksum(digits)
    return cs == 0


def parity_bit(values: list[int]) -> int:
    """Compute even parity bit for a list of 0/1 values."""
    count: int = 0
    for v in values:
        if v == 1:
            count += 1
    return count % 2


def simple_hash(text: str, modulus: int) -> int:
    """Simple hash function: sum of char codes mod modulus."""
    if modulus == 0:
        return 0
    total: int = 0
    for ch in text:
        total += ord(ch)
    return total % modulus


def checksum_xor(values: list[int]) -> int:
    """XOR checksum of all values."""
    result: int = 0
    for v in values:
        result = result ^ v
    return result


def digit_sum(n: int) -> int:
    """Sum of digits of a non-negative integer."""
    if n < 0:
        n = -n
    total: int = 0
    while n > 0:
        total += n % 10
        n = n // 10
    return total


def digital_root(n: int) -> int:
    """Compute digital root (repeated digit sum)."""
    if n < 0:
        n = -n
    while n >= 10:
        n = digit_sum(n)
    return n


def test_module() -> int:
    """Test checksum operations."""
    ok: int = 0

    cs: int = luhn_checksum([7, 9, 9, 2, 7, 3, 9, 8, 7, 1])
    if cs == 0:
        ok += 1

    p: int = parity_bit([1, 0, 1, 1])
    if p == 1:
        ok += 1

    p2: int = parity_bit([1, 1, 1, 1])
    if p2 == 0:
        ok += 1

    h: int = simple_hash("abc", 100)
    if h == 94:
        ok += 1

    x: int = checksum_xor([1, 2, 3])
    if x == 0:
        ok += 1

    ds: int = digit_sum(12345)
    if ds == 15:
        ok += 1

    dr: int = digital_root(9875)
    if dr == 2:
        ok += 1

    return ok
