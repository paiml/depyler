"""Simple CRC-like checksum computation on integer arrays."""


def crc_simple(data: list[int], poly: int) -> int:
    """Compute simple CRC checksum with given polynomial."""
    crc: int = 0
    i: int = 0
    while i < len(data):
        crc = crc ^ data[i]
        bit: int = 0
        while bit < 8:
            if crc % 2 == 1:
                crc = (crc // 2) ^ poly
            else:
                crc = crc // 2
            bit = bit + 1
        i = i + 1
    return crc


def checksum_add(data: list[int]) -> int:
    """Simple additive checksum."""
    total: int = 0
    i: int = 0
    while i < len(data):
        total = total + data[i]
        i = i + 1
    return total % 256


def checksum_xor(data: list[int]) -> int:
    """XOR checksum."""
    result: int = 0
    i: int = 0
    while i < len(data):
        result = result ^ data[i]
        i = i + 1
    return result


def fletcher16(data: list[int]) -> int:
    """Fletcher-16 checksum."""
    sum1: int = 0
    sum2: int = 0
    i: int = 0
    while i < len(data):
        sum1 = (sum1 + data[i]) % 255
        sum2 = (sum2 + sum1) % 255
        i = i + 1
    return sum2 * 256 + sum1


def verify_checksum(data: list[int], expected: int) -> int:
    """Verify additive checksum matches expected. Returns 1 if match."""
    actual: int = checksum_add(data)
    if actual == expected:
        return 1
    return 0


def test_module() -> int:
    """Test checksum computations."""
    ok: int = 0
    d1: list[int] = [1, 2, 3, 4, 5]
    if checksum_add(d1) == 15:
        ok = ok + 1
    if checksum_xor(d1) == 1:
        ok = ok + 1
    d2: list[int] = [0, 0, 0]
    if checksum_add(d2) == 0:
        ok = ok + 1
    if checksum_xor(d2) == 0:
        ok = ok + 1
    if verify_checksum(d1, 15) == 1:
        ok = ok + 1
    if verify_checksum(d1, 10) == 0:
        ok = ok + 1
    f1: int = fletcher16(d1)
    if f1 > 0:
        ok = ok + 1
    return ok
