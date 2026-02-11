"""Bit rotation operations: rotate bits left and right within a word."""


def rotate_left_32(value: int, shift: int) -> int:
    """Rotate bits left within a 32-bit word."""
    shift = shift % 32
    mask: int = 4294967295
    value = value & mask
    result: int = ((value << shift) | (value >> (32 - shift))) & mask
    return result


def rotate_right_32(value: int, shift: int) -> int:
    """Rotate bits right within a 32-bit word."""
    shift = shift % 32
    mask: int = 4294967295
    value = value & mask
    result: int = ((value >> shift) | (value << (32 - shift))) & mask
    return result


def circular_shift_array(arr: list[int], bits: int) -> list[int]:
    """Apply circular bit shift to each element in array."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        shifted: int = rotate_left_32(arr[i], bits)
        result.append(shifted)
        i = i + 1
    return result


def count_rotations_to_match(a: int, b: int) -> int:
    """Count minimum left rotations of 8-bit value a to match b."""
    mask: int = 255
    a = a & mask
    b = b & mask
    rot: int = 0
    while rot < 8:
        current: int = ((a << rot) | (a >> (8 - rot))) & mask
        if current == b:
            return rot
        rot = rot + 1
    return -1


def test_module() -> int:
    """Test bit rotation functions."""
    ok: int = 0

    if rotate_left_32(1, 1) == 2:
        ok = ok + 1

    if rotate_left_32(1, 31) == 2147483648:
        ok = ok + 1

    if rotate_right_32(2, 1) == 1:
        ok = ok + 1

    if rotate_right_32(1, 1) == 2147483648:
        ok = ok + 1

    arr: list[int] = [1, 2, 4]
    shifted: list[int] = circular_shift_array(arr, 1)
    if shifted[0] == 2 and shifted[1] == 4 and shifted[2] == 8:
        ok = ok + 1

    if count_rotations_to_match(1, 4) == 2:
        ok = ok + 1

    if count_rotations_to_match(1, 3) == -1:
        ok = ok + 1

    return ok
