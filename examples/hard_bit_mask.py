"""Bit masking operations: set, clear, toggle bits."""


def set_bit(n: int, pos: int) -> int:
    """Set the bit at given position."""
    mask: int = 1 << pos
    return n | mask


def clear_bit(n: int, pos: int) -> int:
    """Clear the bit at given position."""
    mask: int = ~(1 << pos)
    return n & mask


def toggle_bit(n: int, pos: int) -> int:
    """Toggle the bit at given position."""
    mask: int = 1 << pos
    return n ^ mask


def is_bit_set(n: int, pos: int) -> int:
    """Return 1 if bit at pos is set, else 0."""
    mask: int = 1 << pos
    if (n & mask) != 0:
        return 1
    return 0


def count_bits_in_range(n: int, low: int, high: int) -> int:
    """Count set bits between positions low and high inclusive."""
    count: int = 0
    pos: int = low
    while pos <= high:
        if is_bit_set(n, pos) == 1:
            count = count + 1
        pos = pos + 1
    return count


def test_module() -> int:
    passed: int = 0

    if set_bit(0, 3) == 8:
        passed = passed + 1
    if clear_bit(15, 1) == 13:
        passed = passed + 1
    if toggle_bit(5, 1) == 7:
        passed = passed + 1
    if toggle_bit(7, 1) == 5:
        passed = passed + 1
    if is_bit_set(10, 1) == 1:
        passed = passed + 1
    if is_bit_set(10, 2) == 0:
        passed = passed + 1
    if count_bits_in_range(255, 0, 3) == 4:
        passed = passed + 1
    if count_bits_in_range(170, 0, 7) == 4:
        passed = passed + 1

    return passed
