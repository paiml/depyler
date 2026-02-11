"""Zigzag encoding for signed integers (protobuf-style)."""


def zigzag_encode(val: int) -> int:
    """Encode a signed integer using zigzag encoding."""
    if val >= 0:
        return val * 2
    return (-val) * 2 - 1


def zigzag_decode(val: int) -> int:
    """Decode a zigzag-encoded integer back to signed."""
    if val % 2 == 0:
        return val // 2
    return -((val + 1) // 2)


def zigzag_encode_array(arr: list[int]) -> list[int]:
    """Encode an array of signed integers using zigzag encoding."""
    result: list[int] = []
    i: int = 0
    length: int = len(arr)
    while i < length:
        encoded: int = zigzag_encode(arr[i])
        result.append(encoded)
        i = i + 1
    return result


def zigzag_decode_array(arr: list[int]) -> list[int]:
    """Decode an array of zigzag-encoded integers."""
    result: list[int] = []
    i: int = 0
    length: int = len(arr)
    while i < length:
        decoded: int = zigzag_decode(arr[i])
        result.append(decoded)
        i = i + 1
    return result


def zigzag_delta_encode(arr: list[int]) -> list[int]:
    """Delta + zigzag encode: store differences between consecutive values."""
    if len(arr) == 0:
        return []
    result: list[int] = [zigzag_encode(arr[0])]
    i: int = 1
    length: int = len(arr)
    while i < length:
        diff: int = arr[i] - arr[i - 1]
        result.append(zigzag_encode(diff))
        i = i + 1
    return result


def test_module() -> int:
    """Test zigzag encoding operations."""
    passed: int = 0

    if zigzag_encode(0) == 0:
        passed = passed + 1

    if zigzag_encode(-1) == 1:
        passed = passed + 1

    if zigzag_encode(1) == 2:
        passed = passed + 1

    if zigzag_decode(0) == 0:
        passed = passed + 1

    if zigzag_decode(1) == -1:
        passed = passed + 1

    if zigzag_decode(2) == 1:
        passed = passed + 1

    arr: list[int] = [0, -1, 1, -2, 2]
    enc: list[int] = zigzag_encode_array(arr)
    dec: list[int] = zigzag_decode_array(enc)
    if dec[0] == 0 and dec[1] == -1 and dec[2] == 1:
        passed = passed + 1

    delta: list[int] = zigzag_delta_encode([10, 12, 11, 15])
    if delta[0] == 20 and delta[1] == 4:
        passed = passed + 1

    return passed
