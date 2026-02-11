"""Number encoding and decoding: variable-length, Gray code, and zigzag encoding."""


def to_gray_code(n: int) -> int:
    """Convert a number to Gray code."""
    return n ^ (n >> 1)


def from_gray_code(gray: int) -> int:
    """Convert Gray code back to binary."""
    mask: int = gray >> 1
    result: int = gray
    while mask > 0:
        result = result ^ mask
        mask = mask >> 1
    return result


def zigzag_encode_int(n: int) -> int:
    """Zigzag encode a signed integer to unsigned (protobuf style).
    Maps: 0->0, -1->1, 1->2, -2->3, 2->4, ..."""
    if n >= 0:
        return n * 2
    return (-n) * 2 - 1


def zigzag_decode_int(encoded: int) -> int:
    """Decode a zigzag-encoded integer back to signed."""
    if encoded % 2 == 0:
        return encoded // 2
    return -(encoded + 1) // 2


def varint_byte_count(n: int) -> int:
    """Count how many bytes a varint encoding would need (7 bits per byte)."""
    if n == 0:
        return 1
    if n < 0:
        n = -n
    count: int = 0
    while n > 0:
        count = count + 1
        n = n >> 7
    return count


def test_module() -> int:
    """Test number encoding functions."""
    ok: int = 0

    if to_gray_code(0) == 0:
        ok = ok + 1

    if to_gray_code(1) == 1:
        ok = ok + 1

    if to_gray_code(2) == 3:
        ok = ok + 1

    if from_gray_code(3) == 2:
        ok = ok + 1

    if from_gray_code(to_gray_code(7)) == 7:
        ok = ok + 1

    if zigzag_encode_int(0) == 0:
        ok = ok + 1

    if zigzag_encode_int(-1) == 1:
        ok = ok + 1

    if zigzag_encode_int(1) == 2:
        ok = ok + 1

    if zigzag_decode_int(zigzag_encode_int(-42)) == -42:
        ok = ok + 1

    if varint_byte_count(127) == 1:
        ok = ok + 1

    if varint_byte_count(128) == 2:
        ok = ok + 1

    return ok
