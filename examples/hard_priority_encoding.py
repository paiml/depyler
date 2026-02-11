"""Priority and encoding operations.

Implements priority encoding, priority queues using arrays,
and value encoding schemes.
"""


def priority_encode(value: int, num_bits: int) -> int:
    """Find position of highest set bit (priority encoder).

    Returns -1 if no bits set.
    """
    pos: int = num_bits - 1
    while pos >= 0:
        if (value >> pos) & 1 == 1:
            return pos
        pos = pos - 1
    return -1


def one_hot_encode(position: int) -> int:
    """Create one-hot encoding for given bit position."""
    result: int = 1 << position
    return result


def one_hot_decode(value: int, num_bits: int) -> int:
    """Decode one-hot value to position. Returns -1 if not valid one-hot."""
    count: int = 0
    pos: int = -1
    i: int = 0
    while i < num_bits:
        if (value >> i) & 1 == 1:
            count = count + 1
            pos = i
        i = i + 1
    if count == 1:
        return pos
    return -1


def thermometer_encode(value: int) -> int:
    """Create thermometer encoding: n ones for value n."""
    result: int = (1 << value) - 1
    return result


def thermometer_decode(value: int, num_bits: int) -> int:
    """Decode thermometer encoding to integer value (count of consecutive 1s from LSB)."""
    count: int = 0
    i: int = 0
    while i < num_bits:
        if (value >> i) & 1 == 1:
            count = count + 1
        else:
            i = num_bits
        i = i + 1
    return count


def gray_encode(value: int) -> int:
    """Encode integer to Gray code."""
    result: int = value ^ (value >> 1)
    return result


def test_module() -> int:
    """Test priority encoding operations."""
    ok: int = 0

    pri: int = priority_encode(0b1010, 8)
    if pri == 3:
        ok = ok + 1

    oh: int = one_hot_encode(3)
    if oh == 8:
        ok = ok + 1

    decoded: int = one_hot_decode(8, 8)
    if decoded == 3:
        ok = ok + 1

    therm: int = thermometer_encode(4)
    if therm == 0b1111:
        ok = ok + 1

    td: int = thermometer_decode(0b0111, 8)
    if td == 3:
        ok = ok + 1

    gc: int = gray_encode(5)
    if gc == 7:
        ok = ok + 1

    return ok
