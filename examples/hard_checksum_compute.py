"""Checksum computations: Luhn variant, Fletcher, and Adler checksums on integer arrays."""


def fletcher16(data: list[int]) -> int:
    """Compute Fletcher-16 checksum on an array of values (0-255)."""
    sum1: int = 0
    sum2: int = 0
    i: int = 0
    while i < len(data):
        sum1 = (sum1 + data[i]) % 255
        sum2 = (sum2 + sum1) % 255
        i = i + 1
    return (sum2 * 256) + sum1


def adler32(data: list[int]) -> int:
    """Compute Adler-32 checksum on an array of values."""
    a: int = 1
    b: int = 0
    mod: int = 65521
    i: int = 0
    while i < len(data):
        a = (a + data[i]) % mod
        b = (b + a) % mod
        i = i + 1
    return (b * 65536) + a


def xor_checksum(data: list[int]) -> int:
    """Simple XOR checksum of all elements."""
    result: int = 0
    i: int = 0
    while i < len(data):
        result = result ^ data[i]
        i = i + 1
    return result


def sum_complement_check(data: list[int]) -> int:
    """Compute sum complement checksum: 256 - (sum mod 256).
    When appended to data, total sum mod 256 should be 0."""
    total: int = 0
    i: int = 0
    while i < len(data):
        total = total + data[i]
        i = i + 1
    return (256 - (total % 256)) % 256


def test_module() -> int:
    """Test checksum computation functions."""
    ok: int = 0

    data1: list[int] = [1, 2, 3, 4, 5]
    f16: int = fletcher16(data1)
    if f16 > 0:
        ok = ok + 1

    a32: int = adler32(data1)
    if a32 > 0:
        ok = ok + 1

    if xor_checksum(data1) == 1:
        ok = ok + 1

    data2: list[int] = [5, 5]
    if xor_checksum(data2) == 0:
        ok = ok + 1

    chk: int = sum_complement_check(data1)
    total: int = 15 + chk
    if total % 256 == 0:
        ok = ok + 1

    empty: list[int] = []
    if xor_checksum(empty) == 0:
        ok = ok + 1

    return ok
