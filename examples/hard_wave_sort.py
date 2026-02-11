"""Wave sort and alternating sort patterns.

Tests: wave sort, verify wave, alternating min-max.
"""


def wave_sort(arr: list[int]) -> list[int]:
    """Sort array in wave form: a[0] >= a[1] <= a[2] >= a[3] ..."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    n: int = len(result)
    i2: int = 0
    while i2 < n:
        j: int = i2 + 1
        while j < n:
            if result[i2] > result[j]:
                tmp: int = result[i2]
                result[i2] = result[j]
                result[j] = tmp
            j = j + 1
        i2 = i2 + 1
    k: int = 0
    while k < n - 1:
        tmp2: int = result[k]
        result[k] = result[k + 1]
        result[k + 1] = tmp2
        k = k + 2
    return result


def is_wave_sorted(arr: list[int]) -> int:
    """Check if array is in wave form. Returns 1 if yes."""
    i: int = 0
    while i < len(arr):
        if i > 0:
            if i % 2 == 0:
                if arr[i] > arr[i - 1]:
                    return 0
            else:
                if arr[i] < arr[i - 1]:
                    return 0
        i = i + 1
    return 1


def alternating_sum_diff(arr: list[int]) -> int:
    """Compute arr[0] - arr[1] + arr[2] - arr[3] + ..."""
    total: int = 0
    i: int = 0
    while i < len(arr):
        if i % 2 == 0:
            total = total + arr[i]
        else:
            total = total - arr[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test wave sort operations."""
    ok: int = 0
    arr: list[int] = [3, 6, 5, 10, 7, 20]
    wave: list[int] = wave_sort(arr)
    if is_wave_sorted(wave) == 1:
        ok = ok + 1
    if len(wave) == 6:
        ok = ok + 1
    already_wave: list[int] = [5, 3, 6, 2, 7, 1]
    if is_wave_sorted(already_wave) == 1:
        ok = ok + 1
    if alternating_sum_diff([1, 2, 3, 4]) == -2:
        ok = ok + 1
    if alternating_sum_diff([10, 3, 7]) == 14:
        ok = ok + 1
    return ok
