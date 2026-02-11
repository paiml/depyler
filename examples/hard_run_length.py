"""Run length encoding/decoding variants.

Tests: consecutive counting, list building, paired encoding.
"""


def rle_encode_lengths(arr: list[int]) -> list[int]:
    """Return run lengths only (not values)."""
    if len(arr) == 0:
        return []
    result: list[int] = []
    count: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            count += 1
        else:
            result.append(count)
            count = 1
        i += 1
    result.append(count)
    return result


def rle_encode_values(arr: list[int]) -> list[int]:
    """Return unique consecutive values."""
    if len(arr) == 0:
        return []
    result: list[int] = []
    result.append(arr[0])
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
        i += 1
    return result


def rle_decode(values: list[int], lengths: list[int]) -> list[int]:
    """Decode RLE given values and lengths."""
    result: list[int] = []
    i: int = 0
    while i < len(values) and i < len(lengths):
        j: int = 0
        while j < lengths[i]:
            result.append(values[i])
            j += 1
        i += 1
    return result


def count_runs(arr: list[int]) -> int:
    """Count number of runs in array."""
    if len(arr) == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            runs += 1
        i += 1
    return runs


def longest_run(arr: list[int]) -> int:
    """Find the longest run length."""
    if len(arr) == 0:
        return 0
    best: int = 1
    current: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            current += 1
            if current > best:
                best = current
        else:
            current = 1
        i += 1
    return best


def test_module() -> int:
    """Test run-length encoding operations."""
    ok: int = 0

    lengths: list[int] = rle_encode_lengths([1, 1, 2, 2, 2, 3])
    if lengths == [2, 3, 1]:
        ok += 1

    values: list[int] = rle_encode_values([1, 1, 2, 2, 2, 3])
    if values == [1, 2, 3]:
        ok += 1

    decoded: list[int] = rle_decode([1, 2, 3], [2, 3, 1])
    if decoded == [1, 1, 2, 2, 2, 3]:
        ok += 1

    runs: int = count_runs([1, 1, 2, 3, 3])
    if runs == 3:
        ok += 1

    lr: int = longest_run([1, 2, 2, 2, 3, 3])
    if lr == 3:
        ok += 1

    return ok
