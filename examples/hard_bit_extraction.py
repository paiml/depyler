"""Extract bit fields from integers.

Implements operations for extracting, isolating, and
manipulating specific bit fields within integer values.
"""


def extract_bits(value: int, start: int, width: int) -> int:
    """Extract a bit field from value starting at bit position start with given width."""
    mask: int = (1 << width) - 1
    result: int = (value >> start) & mask
    return result


def pack_fields(field_a: int, field_b: int, field_c: int) -> int:
    """Pack three 8-bit fields into a single integer.

    field_a occupies bits 0-7, field_b bits 8-15, field_c bits 16-23.
    """
    masked_a: int = field_a & 0xFF
    masked_b: int = (field_b & 0xFF) << 8
    masked_c: int = (field_c & 0xFF) << 16
    result: int = masked_a | masked_b | masked_c
    return result


def unpack_field(packed: int, field_index: int) -> int:
    """Unpack the field at given index (0, 1, or 2) from a packed integer."""
    shift: int = field_index * 8
    result: int = (packed >> shift) & 0xFF
    return result


def count_set_bits_in_range(value: int, start: int, end: int) -> int:
    """Count set bits in range [start, end) of value."""
    count: int = 0
    pos: int = start
    while pos < end:
        bit: int = (value >> pos) & 1
        count = count + bit
        pos = pos + 1
    return count


def isolate_lowest_set_bit(value: int) -> int:
    """Isolate the lowest set bit of value."""
    if value == 0:
        return 0
    result: int = value & (-value)
    return result


def test_module() -> int:
    """Test bit extraction operations."""
    ok: int = 0

    extracted: int = extract_bits(0b11010110, 2, 4)
    if extracted == 0b0101:
        ok = ok + 1

    packed: int = pack_fields(0x12, 0x34, 0x56)
    f0: int = unpack_field(packed, 0)
    f1: int = unpack_field(packed, 1)
    f2: int = unpack_field(packed, 2)
    if f0 == 0x12 and f1 == 0x34 and f2 == 0x56:
        ok = ok + 1

    bits: int = count_set_bits_in_range(0b11110000, 4, 8)
    if bits == 4:
        ok = ok + 1

    lowest: int = isolate_lowest_set_bit(0b10100)
    if lowest == 0b00100:
        ok = ok + 1

    zero_result: int = isolate_lowest_set_bit(0)
    if zero_result == 0:
        ok = ok + 1

    return ok
