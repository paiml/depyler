"""Simple compression: run-length encoding on integer arrays and delta encoding."""


def rle_encode(arr: list[int]) -> list[int]:
    """Run-length encode an array. Output: [value, count, value, count, ...]."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0:
        return result
    current: int = arr[0]
    count: int = 1
    i: int = 1
    while i < n:
        if arr[i] == current:
            count = count + 1
        else:
            result.append(current)
            result.append(count)
            current = arr[i]
            count = 1
        i = i + 1
    result.append(current)
    result.append(count)
    return result


def rle_decode(encoded: list[int]) -> list[int]:
    """Decode a run-length encoded array."""
    result: list[int] = []
    i: int = 0
    while i < len(encoded) - 1:
        value: int = encoded[i]
        count: int = encoded[i + 1]
        j: int = 0
        while j < count:
            result.append(value)
            j = j + 1
        i = i + 2
    return result


def delta_encode(arr: list[int]) -> list[int]:
    """Delta encode: store first value, then differences."""
    result: list[int] = []
    n: int = len(arr)
    if n == 0:
        return result
    result.append(arr[0])
    i: int = 1
    while i < n:
        prev: int = i - 1
        result.append(arr[i] - arr[prev])
        i = i + 1
    return result


def delta_decode(encoded: list[int]) -> list[int]:
    """Decode a delta-encoded array."""
    result: list[int] = []
    n: int = len(encoded)
    if n == 0:
        return result
    result.append(encoded[0])
    i: int = 1
    while i < n:
        prev_idx: int = len(result) - 1
        val: int = result[prev_idx] + encoded[i]
        result.append(val)
        i = i + 1
    return result


def test_module() -> int:
    """Test compression functions."""
    ok: int = 0

    arr1: list[int] = [1, 1, 1, 2, 2, 3]
    enc: list[int] = rle_encode(arr1)
    if enc[0] == 1 and enc[1] == 3 and enc[2] == 2 and enc[3] == 2 and enc[4] == 3 and enc[5] == 1:
        ok = ok + 1

    dec: list[int] = rle_decode(enc)
    if len(dec) == 6 and dec[0] == 1 and dec[3] == 2 and dec[5] == 3:
        ok = ok + 1

    arr2: list[int] = [10, 12, 15, 13]
    denc: list[int] = delta_encode(arr2)
    if denc[0] == 10 and denc[1] == 2 and denc[2] == 3 and denc[3] == -2:
        ok = ok + 1

    ddec: list[int] = delta_decode(denc)
    if ddec[0] == 10 and ddec[1] == 12 and ddec[2] == 15 and ddec[3] == 13:
        ok = ok + 1

    empty: list[int] = []
    if len(rle_encode(empty)) == 0:
        ok = ok + 1

    if len(delta_encode(empty)) == 0:
        ok = ok + 1

    return ok
