"""Checksum verification operations.

Implements various checksum algorithms for data integrity
verification using integer arithmetic.
"""


def simple_checksum(data: list[int], size: int) -> int:
    """Compute a simple additive checksum modulo 256."""
    total: int = 0
    i: int = 0
    while i < size:
        total = total + data[i]
        i = i + 1
    result: int = total % 256
    return result


def xor_checksum(data: list[int], size: int) -> int:
    """Compute XOR checksum of all data elements."""
    result: int = 0
    i: int = 0
    while i < size:
        result = result ^ data[i]
        i = i + 1
    return result


def fletcher_checksum(data: list[int], size: int) -> int:
    """Compute Fletcher-16 like checksum.

    Returns sum1 in lower byte, sum2 in upper byte.
    """
    sum1: int = 0
    sum2: int = 0
    i: int = 0
    while i < size:
        sum1 = (sum1 + data[i]) % 255
        sum2 = (sum2 + sum1) % 255
        i = i + 1
    result: int = (sum2 << 8) | sum1
    return result


def verify_checksum(data: list[int], size: int, expected: int) -> int:
    """Verify data against expected simple checksum. Returns 1 if valid."""
    computed: int = simple_checksum(data, size)
    if computed == expected:
        return 1
    return 0


def luhn_check(digits: list[int], size: int) -> int:
    """Simplified Luhn-like check. Returns 1 if sum mod 10 is 0."""
    total: int = 0
    i: int = 0
    while i < size:
        val: int = digits[i]
        pos_from_right: int = size - 1 - i
        if pos_from_right % 2 == 1:
            val = val * 2
            if val > 9:
                val = val - 9
        total = total + val
        i = i + 1
    if total % 10 == 0:
        return 1
    return 0


def test_module() -> int:
    """Test checksum verification operations."""
    ok: int = 0

    data: list[int] = [10, 20, 30, 40]
    cs: int = simple_checksum(data, 4)
    if cs == 100:
        ok = ok + 1

    xcs: int = xor_checksum(data, 4)
    if xcs == (10 ^ 20 ^ 30 ^ 40):
        ok = ok + 1

    valid: int = verify_checksum(data, 4, 100)
    if valid == 1:
        ok = ok + 1

    invalid: int = verify_checksum(data, 4, 99)
    if invalid == 0:
        ok = ok + 1

    fc: int = fletcher_checksum(data, 4)
    if fc > 0:
        ok = ok + 1

    return ok
