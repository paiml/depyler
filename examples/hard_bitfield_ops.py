"""Bitfield operations: set, get, toggle, and count bits in packed integers."""


def set_bit(value: int, position: int) -> int:
    """Set bit at given position (0-indexed from LSB)."""
    return value | (1 << position)


def clear_bit(value: int, position: int) -> int:
    """Clear bit at given position."""
    mask: int = ~(1 << position)
    return value & mask


def toggle_bit(value: int, position: int) -> int:
    """Toggle bit at given position."""
    return value ^ (1 << position)


def get_bit(value: int, position: int) -> int:
    """Get bit at given position. Returns 0 or 1."""
    return (value >> position) & 1


def count_set_bits(value: int) -> int:
    """Count number of set bits (popcount)."""
    if value < 0:
        value = -value
    count: int = 0
    while value > 0:
        count = count + (value & 1)
        value = value >> 1
    return count


def extract_bitfield(value: int, start: int, width: int) -> int:
    """Extract a bitfield of given width starting at start position."""
    mask: int = (1 << width) - 1
    return (value >> start) & mask


def pack_two_shorts(high: int, low: int) -> int:
    """Pack two 16-bit values into one 32-bit integer."""
    return ((high & 65535) << 16) | (low & 65535)


def unpack_high(packed: int) -> int:
    """Extract high 16 bits from packed integer."""
    return (packed >> 16) & 65535


def unpack_low(packed: int) -> int:
    """Extract low 16 bits from packed integer."""
    return packed & 65535


def test_module() -> int:
    """Test bitfield operations."""
    ok: int = 0

    if set_bit(0, 3) == 8:
        ok = ok + 1

    if clear_bit(15, 1) == 13:
        ok = ok + 1

    if toggle_bit(5, 1) == 7:
        ok = ok + 1

    if get_bit(5, 0) == 1 and get_bit(5, 1) == 0:
        ok = ok + 1

    if count_set_bits(7) == 3:
        ok = ok + 1

    if extract_bitfield(255, 2, 4) == 15:
        ok = ok + 1

    packed: int = pack_two_shorts(100, 200)
    if unpack_high(packed) == 100 and unpack_low(packed) == 200:
        ok = ok + 1

    return ok
