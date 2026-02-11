"""Bit toggle operations.

Implements various bit toggling algorithms including
single bit toggle, range toggle, and conditional toggling.
"""


def toggle_bit(value: int, pos: int) -> int:
    """Toggle a single bit at given position."""
    result: int = value ^ (1 << pos)
    return result


def toggle_range(value: int, start: int, end: int) -> int:
    """Toggle all bits in range [start, end) inclusive of start."""
    result: int = value
    pos: int = start
    while pos < end:
        result = result ^ (1 << pos)
        pos = pos + 1
    return result


def toggle_even_bits(value: int, num_bits: int) -> int:
    """Toggle all bits at even positions (0, 2, 4, ...)."""
    result: int = value
    pos: int = 0
    while pos < num_bits:
        result = result ^ (1 << pos)
        pos = pos + 2
    return result


def toggle_odd_bits(value: int, num_bits: int) -> int:
    """Toggle all bits at odd positions (1, 3, 5, ...)."""
    result: int = value
    pos: int = 1
    while pos < num_bits:
        result = result ^ (1 << pos)
        pos = pos + 2
    return result


def count_toggles_to_match(a: int, b: int, num_bits: int) -> int:
    """Count how many bit toggles needed to convert a to b."""
    diff: int = a ^ b
    count: int = 0
    pos: int = 0
    while pos < num_bits:
        if (diff >> pos) & 1 == 1:
            count = count + 1
        pos = pos + 1
    return count


def apply_toggle_sequence(value: int, toggles: list[int], num_toggles: int) -> int:
    """Apply a sequence of bit position toggles."""
    result: int = value
    i: int = 0
    while i < num_toggles:
        pos: int = toggles[i]
        result = result ^ (1 << pos)
        i = i + 1
    return result


def test_module() -> int:
    """Test bit toggle operations."""
    ok: int = 0

    t1: int = toggle_bit(0b1010, 0)
    if t1 == 0b1011:
        ok = ok + 1

    t2: int = toggle_range(0b0000, 1, 4)
    if t2 == 0b1110:
        ok = ok + 1

    t3: int = toggle_even_bits(0b0000, 4)
    if t3 == 0b0101:
        ok = ok + 1

    t4: int = toggle_odd_bits(0b0000, 4)
    if t4 == 0b1010:
        ok = ok + 1

    toggles_needed: int = count_toggles_to_match(0b1010, 0b0101, 4)
    if toggles_needed == 4:
        ok = ok + 1

    seq: list[int] = [0, 2]
    t5: int = apply_toggle_sequence(0b0000, seq, 2)
    if t5 == 0b0101:
        ok = ok + 1

    return ok
