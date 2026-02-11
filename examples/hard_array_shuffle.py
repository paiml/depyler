"""Deterministic array shuffling.

Tests: Fisher-Yates with seed, interleave shuffle, riffle shuffle, unshuffle.
"""


def seeded_random(seed: int, bound: int) -> int:
    """Simple deterministic pseudo-random number in [0, bound)."""
    val: int = ((seed * 1103515245 + 12345) // 65536) % 32768
    if val < 0:
        val = -val
    return val % bound


def fisher_yates_shuffle(arr: list[int], seed: int) -> list[int]:
    """Deterministic Fisher-Yates shuffle with seed."""
    result: list[int] = []
    i: int = 0
    n: int = len(arr)
    while i < n:
        result.append(arr[i])
        i = i + 1
    s: int = seed
    i = n - 1
    while i > 0:
        s = s * 1103515245 + 12345
        j_val: int = s % 32768
        if j_val < 0:
            j_val = -j_val
        j: int = j_val % (i + 1)
        tmp: int = result[i]
        result[i] = result[j]
        result[j] = tmp
        i = i - 1
    return result


def interleave_shuffle(arr: list[int]) -> list[int]:
    """Interleave first half and second half."""
    n: int = len(arr)
    mid: int = n // 2
    result: list[int] = []
    i: int = 0
    while i < mid:
        result.append(arr[i])
        if mid + i < n:
            result.append(arr[mid + i])
        i = i + 1
    if n % 2 == 1:
        result.append(arr[n - 1])
    return result


def is_permutation(arr1: list[int], arr2: list[int]) -> int:
    """Check if arr2 is a permutation of arr1. Returns 1 or 0."""
    n1: int = len(arr1)
    n2: int = len(arr2)
    if n1 != n2:
        return 0
    sorted1: list[int] = []
    sorted2: list[int] = []
    i: int = 0
    while i < n1:
        sorted1.append(arr1[i])
        sorted2.append(arr2[i])
        i = i + 1
    i = 0
    while i < n1:
        j: int = i + 1
        while j < n1:
            if sorted1[j] < sorted1[i]:
                tmp: int = sorted1[i]
                sorted1[i] = sorted1[j]
                sorted1[j] = tmp
            if sorted2[j] < sorted2[i]:
                tmp2: int = sorted2[i]
                sorted2[i] = sorted2[j]
                sorted2[j] = tmp2
            j = j + 1
        i = i + 1
    i = 0
    while i < n1:
        if sorted1[i] != sorted2[i]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test shuffle operations."""
    ok: int = 0
    orig: list[int] = [1, 2, 3, 4, 5]
    shuffled: list[int] = fisher_yates_shuffle(orig, 42)
    if is_permutation(orig, shuffled) == 1:
        ok = ok + 1
    if len(shuffled) == 5:
        ok = ok + 1
    inter: list[int] = interleave_shuffle([1, 2, 3, 4, 5, 6])
    if inter[0] == 1 and inter[1] == 4:
        ok = ok + 1
    if is_permutation([1, 2, 3], [3, 1, 2]) == 1:
        ok = ok + 1
    if is_permutation([1, 2, 3], [1, 2, 4]) == 0:
        ok = ok + 1
    return ok
