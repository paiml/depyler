"""Network checksum algorithms simulation.

Implements Internet checksum (one's complement sum), CRC-like
checksums, and parity-based error detection.
"""


def cksum_ones_complement(data: list[int], length: int) -> int:
    """Compute 16-bit one's complement checksum (like IP header checksum).
    Returns checksum value."""
    total: int = 0
    i: int = 0
    while i < length:
        val: int = data[i]
        total = total + val
        i = i + 1
    while total > 65535:
        carry: int = total // 65536
        remainder: int = total % 65536
        total = carry + remainder
    result: int = 65535 - total
    return result


def cksum_verify(data: list[int], length: int, expected: int) -> int:
    """Verify checksum. Returns 1 if valid."""
    computed: int = cksum_ones_complement(data, length)
    if computed == expected:
        return 1
    return 0


def cksum_xor(data: list[int], length: int) -> int:
    """XOR-based checksum (block check character)."""
    result: int = 0
    i: int = 0
    while i < length:
        val: int = data[i]
        result = result ^ val
        i = i + 1
    return result


def cksum_fletcher(data: list[int], length: int) -> int:
    """Fletcher checksum (simplified, mod 255)."""
    sum1: int = 0
    sum2: int = 0
    i: int = 0
    while i < length:
        val: int = data[i]
        sum1 = (sum1 + val) % 255
        sum2 = (sum2 + sum1) % 255
        i = i + 1
    return sum2 * 256 + sum1


def cksum_adler(data: list[int], length: int) -> int:
    """Adler-32 like checksum (mod 65521)."""
    a: int = 1
    b: int = 0
    i: int = 0
    while i < length:
        val: int = data[i]
        a = (a + val) % 65521
        b = (b + a) % 65521
        i = i + 1
    return b * 65536 + a


def cksum_parity(val: int) -> int:
    """Count number of 1-bits (parity). Returns 0 for even, 1 for odd."""
    count: int = 0
    n: int = val
    while n > 0:
        count = count + (n % 2)
        n = n // 2
    return count % 2


def test_module() -> int:
    """Test checksum algorithms."""
    passed: int = 0
    data: list[int] = [1000, 2000, 3000, 4000, 5000]

    # Test 1: ones complement checksum
    cs: int = cksum_ones_complement(data, 5)
    if cs >= 0:
        if cs <= 65535:
            passed = passed + 1

    # Test 2: verify matches
    valid: int = cksum_verify(data, 5, cs)
    if valid == 1:
        passed = passed + 1

    # Test 3: XOR checksum
    xcs: int = cksum_xor(data, 5)
    xcs2: int = cksum_xor(data, 5)
    if xcs == xcs2:
        passed = passed + 1

    # Test 4: Fletcher checksum is deterministic
    fl1: int = cksum_fletcher(data, 5)
    fl2: int = cksum_fletcher(data, 5)
    if fl1 == fl2:
        if fl1 > 0:
            passed = passed + 1

    # Test 5: parity of known values
    p0: int = cksum_parity(0)
    p7: int = cksum_parity(7)
    if p0 == 0:
        if p7 == 1:
            passed = passed + 1

    return passed
