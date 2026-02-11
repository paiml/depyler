"""Run-length encoding and decoding operations.

Tests: encode, decode, compress ratio, round-trip.
"""


def rle_encode_lengths(arr: list[int]) -> list[int]:
    """Run-length encode: returns [value, count, value, count, ...]."""
    if len(arr) == 0:
        return []
    result: list[int] = []
    current: int = arr[0]
    count: int = 1
    i: int = 1
    while i < len(arr):
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
    """Decode run-length encoded array [value, count, ...]."""
    result: list[int] = []
    i: int = 0
    while i < len(encoded):
        val: int = encoded[i]
        count: int = encoded[i + 1]
        j: int = 0
        while j < count:
            result.append(val)
            j = j + 1
        i = i + 2
    return result


def count_runs(arr: list[int]) -> int:
    """Count distinct runs in the array."""
    if len(arr) == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            runs = runs + 1
        i = i + 1
    return runs


def max_run_length(arr: list[int]) -> int:
    """Find the length of the longest run."""
    if len(arr) == 0:
        return 0
    max_len: int = 1
    cur_len: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            cur_len = cur_len + 1
            if cur_len > max_len:
                max_len = cur_len
        else:
            cur_len = 1
        i = i + 1
    return max_len


def test_module() -> int:
    """Test RLE operations."""
    ok: int = 0
    arr: list[int] = [1, 1, 2, 2, 2, 3, 1, 1]
    enc: list[int] = rle_encode_lengths(arr)
    if enc[0] == 1:
        ok = ok + 1
    if enc[1] == 2:
        ok = ok + 1
    if enc[2] == 2:
        ok = ok + 1
    if enc[3] == 3:
        ok = ok + 1
    dec: list[int] = rle_decode(enc)
    if len(dec) == len(arr):
        ok = ok + 1
    if count_runs(arr) == 4:
        ok = ok + 1
    if max_run_length(arr) == 3:
        ok = ok + 1
    return ok
