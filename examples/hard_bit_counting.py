"""Bit counting and manipulation operations.

Tests: popcount, leading zeros, trailing zeros, bit parity, hamming distance.
"""


def popcount(n: int) -> int:
    """Count set bits in n."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def hamming_distance(a: int, b: int) -> int:
    """Hamming distance between two integers."""
    xor: int = a ^ b
    return popcount(xor)


def is_power_of_two(n: int) -> int:
    """Check if n is a power of two. Returns 1 if yes."""
    if n <= 0:
        return 0
    if n & (n - 1) == 0:
        return 1
    return 0


def highest_set_bit(n: int) -> int:
    """Position of highest set bit (0-indexed). Returns -1 for 0."""
    if n <= 0:
        return -1
    pos: int = 0
    val: int = n
    while val > 1:
        val = val >> 1
        pos = pos + 1
    return pos


def bit_parity(n: int) -> int:
    """Returns 1 if odd number of set bits, 0 if even."""
    return popcount(n) % 2


def test_module() -> int:
    """Test bit counting operations."""
    ok: int = 0
    if popcount(0) == 0:
        ok = ok + 1
    if popcount(7) == 3:
        ok = ok + 1
    if popcount(255) == 8:
        ok = ok + 1
    if hamming_distance(1, 4) == 2:
        ok = ok + 1
    if hamming_distance(7, 7) == 0:
        ok = ok + 1
    if is_power_of_two(8) == 1:
        ok = ok + 1
    if is_power_of_two(6) == 0:
        ok = ok + 1
    if highest_set_bit(8) == 3:
        ok = ok + 1
    if highest_set_bit(1) == 0:
        ok = ok + 1
    if bit_parity(7) == 1:
        ok = ok + 1
    if bit_parity(3) == 0:
        ok = ok + 1
    return ok
