"""Zigzag rearrangement of arrays.

Tests: zigzag order, alternating high-low, wave form, zigzag sum.
"""


def zigzag_arrange(arr: list[int]) -> list[int]:
    """Rearrange array in zigzag: a < b > c < d > e ..."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < n - 1:
        even: int = i % 2
        if even == 0:
            if result[i] > result[i + 1]:
                tmp: int = result[i]
                result[i] = result[i + 1]
                result[i + 1] = tmp
        else:
            if result[i] < result[i + 1]:
                tmp2: int = result[i]
                result[i] = result[i + 1]
                result[i + 1] = tmp2
        i = i + 1
    return result


def is_zigzag(arr: list[int]) -> int:
    """Check if array is in zigzag order. Returns 1 if yes, 0 if no."""
    n: int = len(arr)
    if n <= 2:
        return 1
    i: int = 0
    while i < n - 1:
        even: int = i % 2
        if even == 0:
            if arr[i] > arr[i + 1]:
                return 0
        else:
            if arr[i] < arr[i + 1]:
                return 0
        i = i + 1
    return 1


def wave_sort(arr: list[int]) -> list[int]:
    """Sort array in wave form: a >= b <= c >= d ..."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < n - 1:
        j: int = i + 1
        while j < n:
            if result[j] < result[i]:
                tmp: int = result[i]
                result[i] = result[j]
                result[j] = tmp
            j = j + 1
        i = i + 1
    i = 1
    while i < n:
        tmp3: int = result[i]
        result[i] = result[i - 1]
        result[i - 1] = tmp3
        i = i + 2
    return result


def zigzag_max_length(arr: list[int]) -> int:
    """Length of longest zigzag subsequence."""
    n: int = len(arr)
    if n <= 1:
        return n
    up: int = 1
    down: int = 1
    i: int = 1
    while i < n:
        if arr[i] > arr[i - 1]:
            up = down + 1
        elif arr[i] < arr[i - 1]:
            down = up + 1
        i = i + 1
    if up > down:
        return up
    return down


def test_module() -> int:
    """Test zigzag operations."""
    ok: int = 0
    z: list[int] = zigzag_arrange([4, 3, 7, 8, 6, 2, 1])
    if is_zigzag(z) == 1:
        ok = ok + 1
    if is_zigzag([1, 3, 2, 4, 1]) == 1:
        ok = ok + 1
    if is_zigzag([1, 2, 3, 4, 5]) == 0:
        ok = ok + 1
    w: list[int] = wave_sort([3, 1, 2, 4, 5])
    if len(w) == 5:
        ok = ok + 1
    if zigzag_max_length([1, 7, 4, 9, 2, 5]) == 6:
        ok = ok + 1
    return ok
