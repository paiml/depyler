"""Simple compression: RLE encode/decode, delta encode/decode.

Tests: rle_encode, rle_decode, delta_encode, delta_decode, count_runs.
"""


def rle_encode_str(text: str) -> str:
    """Run-length encode a string. e.g., 'aaabbc' -> 'a3b2c1'."""
    n: int = len(text)
    if n == 0:
        return ""
    result: str = ""
    i: int = 0
    while i < n:
        ch: str = text[i]
        count: int = 1
        j: int = i + 1
        while j < n:
            if text[j] == ch:
                count = count + 1
                j = j + 1
            else:
                break
        result = result + ch
        result = result + int_to_str(count)
        i = j
    return result


def int_to_str(val: int) -> str:
    """Convert non-negative integer to string."""
    if val == 0:
        return "0"
    result: str = ""
    remaining: int = val
    while remaining > 0:
        digit: int = remaining % 10
        if digit == 0:
            result = "0" + result
        elif digit == 1:
            result = "1" + result
        elif digit == 2:
            result = "2" + result
        elif digit == 3:
            result = "3" + result
        elif digit == 4:
            result = "4" + result
        elif digit == 5:
            result = "5" + result
        elif digit == 6:
            result = "6" + result
        elif digit == 7:
            result = "7" + result
        elif digit == 8:
            result = "8" + result
        else:
            result = "9" + result
        remaining = remaining // 10
    return result


def rle_encode_list(arr: list[int]) -> list[int]:
    """Run-length encode list. Returns [val, count, val, count, ...]."""
    n: int = len(arr)
    if n == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < n:
        val: int = arr[i]
        count: int = 1
        j: int = i + 1
        while j < n:
            if arr[j] == val:
                count = count + 1
                j = j + 1
            else:
                break
        result.append(val)
        result.append(count)
        i = j
    return result


def rle_decode_list(encoded: list[int]) -> list[int]:
    """Decode run-length encoded list."""
    result: list[int] = []
    n: int = len(encoded)
    i: int = 0
    while i < n:
        val: int = encoded[i]
        count: int = encoded[i + 1]
        j: int = 0
        while j < count:
            result.append(val)
            j = j + 1
        i = i + 2
    return result


def delta_encode(arr: list[int]) -> list[int]:
    """Delta encoding: store differences between consecutive elements."""
    n: int = len(arr)
    if n == 0:
        return []
    result: list[int] = [arr[0]]
    i: int = 1
    while i < n:
        result.append(arr[i] - arr[i - 1])
        i = i + 1
    return result


def delta_decode(encoded: list[int]) -> list[int]:
    """Decode delta-encoded list."""
    n: int = len(encoded)
    if n == 0:
        return []
    result: list[int] = [encoded[0]]
    i: int = 1
    while i < n:
        last: int = result[i - 1]
        result.append(last + encoded[i])
        i = i + 1
    return result


def count_runs(arr: list[int]) -> int:
    """Count number of runs (consecutive same-value groups)."""
    n: int = len(arr)
    if n == 0:
        return 0
    runs: int = 1
    i: int = 1
    while i < n:
        if arr[i] != arr[i - 1]:
            runs = runs + 1
        i = i + 1
    return runs


def test_module() -> int:
    """Test compression algorithms."""
    passed: int = 0

    encoded: str = rle_encode_str("aaabbc")
    if encoded == "a3b2c1":
        passed = passed + 1

    el: list[int] = rle_encode_list([1, 1, 1, 2, 2, 3])
    if el == [1, 3, 2, 2, 3, 1]:
        passed = passed + 1

    dl: list[int] = rle_decode_list([1, 3, 2, 2, 3, 1])
    if dl == [1, 1, 1, 2, 2, 3]:
        passed = passed + 1

    de: list[int] = delta_encode([10, 12, 15, 13, 20])
    if de == [10, 2, 3, -2, 7]:
        passed = passed + 1

    dd: list[int] = delta_decode([10, 2, 3, -2, 7])
    if dd == [10, 12, 15, 13, 20]:
        passed = passed + 1

    if count_runs([1, 1, 2, 2, 2, 3]) == 3:
        passed = passed + 1

    if rle_encode_str("") == "":
        passed = passed + 1

    return passed
