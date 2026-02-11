"""Bit manipulation: set, clear, toggle, count set bits, power of two check.

Tests: set_bit, clear_bit, toggle_bit, count_set_bits, is_power_of_two.
"""


def set_bit(n: int, pos: int) -> int:
    """Set bit at position pos (0-indexed from right)."""
    return n | (1 << pos)


def clear_bit(n: int, pos: int) -> int:
    """Clear bit at position pos."""
    mask: int = ~(1 << pos)
    return n & mask


def toggle_bit(n: int, pos: int) -> int:
    """Toggle bit at position pos."""
    return n ^ (1 << pos)


def get_bit(n: int, pos: int) -> int:
    """Get bit value at position pos (returns 0 or 1)."""
    return (n >> pos) & 1


def count_set_bits(n: int) -> int:
    """Count number of set bits (Kernighan's algorithm)."""
    count: int = 0
    val: int = n
    while val > 0:
        val = val & (val - 1)
        count = count + 1
    return count


def is_power_of_two(n: int) -> int:
    """Check if n is a power of two. Returns 1 if yes, 0 otherwise."""
    if n <= 0:
        return 0
    if n & (n - 1) == 0:
        return 1
    return 0


def highest_set_bit(n: int) -> int:
    """Find position of the highest set bit. Returns -1 for 0."""
    if n == 0:
        return -1
    pos: int = 0
    val: int = n
    while val > 1:
        val = val >> 1
        pos = pos + 1
    return pos


def test_module() -> int:
    """Test bit operations."""
    ok: int = 0

    if set_bit(0, 2) == 4:
        ok = ok + 1

    if clear_bit(7, 1) == 5:
        ok = ok + 1

    if toggle_bit(5, 1) == 7:
        ok = ok + 1

    if get_bit(5, 2) == 1:
        ok = ok + 1

    if count_set_bits(7) == 3:
        ok = ok + 1

    if count_set_bits(0) == 0:
        ok = ok + 1

    if is_power_of_two(8) == 1:
        ok = ok + 1

    if is_power_of_two(6) == 0:
        ok = ok + 1

    if highest_set_bit(8) == 3:
        ok = ok + 1

    return ok
