"""Counting sort and bucket operation patterns.

Tests: counting sort, radix sort, bucket frequency, top-k frequent,
and sort by frequency.
"""


def counting_sort(arr: list[int], max_val: int) -> list[int]:
    """Sort non-negative integers using counting sort."""
    count: list[int] = [0] * (max_val + 1)
    i: int = 0
    while i < len(arr):
        count[arr[i]] = count[arr[i]] + 1
        i = i + 1
    result: list[int] = []
    val: int = 0
    while val <= max_val:
        j: int = 0
        while j < count[val]:
            result.append(val)
            j = j + 1
        val = val + 1
    return result


def radix_sort_lsd(arr: list[int]) -> list[int]:
    """Radix sort (LSD) for non-negative integers."""
    n: int = len(arr)
    if n == 0:
        return []
    max_val: int = 0
    i: int = 0
    while i < n:
        if arr[i] > max_val:
            max_val = arr[i]
        i = i + 1
    result: list[int] = []
    i = 0
    while i < n:
        result.append(arr[i])
        i = i + 1
    exp: int = 1
    while max_val // exp > 0:
        buckets: list[list[int]] = []
        d: int = 0
        while d < 10:
            buckets.append([])
            d = d + 1
        i = 0
        while i < n:
            digit: int = (result[i] // exp) % 10
            buckets[digit].append(result[i])
            i = i + 1
        idx: int = 0
        d = 0
        while d < 10:
            j: int = 0
            while j < len(buckets[d]):
                result[idx] = buckets[d][j]
                idx = idx + 1
                j = j + 1
            d = d + 1
        exp = exp * 10
    return result


def frequency_count(arr: list[int]) -> dict[int, int]:
    """Count frequency of each element."""
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    return freq


def top_k_frequent(arr: list[int], k: int) -> list[int]:
    """Return k most frequent elements (order by frequency desc)."""
    freq: dict[int, int] = frequency_count(arr)
    keys: list[int] = []
    counts: list[int] = []
    for key in freq:
        keys.append(key)
        counts.append(freq[key])
    n: int = len(keys)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if counts[j] > counts[i]:
                tmp_c: int = counts[i]
                counts[i] = counts[j]
                counts[j] = tmp_c
                tmp_k: int = keys[i]
                keys[i] = keys[j]
                keys[j] = tmp_k
            j = j + 1
        i = i + 1
    result: list[int] = []
    i = 0
    while i < k and i < n:
        result.append(keys[i])
        i = i + 1
    return result


def sort_by_frequency(arr: list[int]) -> list[int]:
    """Sort elements by frequency (descending), then by value (ascending)."""
    freq: dict[int, int] = frequency_count(arr)
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    n: int = len(result)
    i = 0
    while i < n:
        j: int = i + 1
        while j < n:
            fi: int = freq[result[i]]
            fj: int = freq[result[j]]
            swap: bool = False
            if fj > fi:
                swap = True
            elif fj == fi and result[j] < result[i]:
                swap = True
            if swap:
                tmp: int = result[i]
                result[i] = result[j]
                result[j] = tmp
            j = j + 1
        i = i + 1
    return result


def test_module() -> bool:
    """Test all counting sort functions."""
    ok: bool = True

    if counting_sort([4, 2, 2, 8, 3, 3, 1], 8) != [1, 2, 2, 3, 3, 4, 8]:
        ok = False

    if radix_sort_lsd([170, 45, 75, 90, 802, 24, 2, 66]) != [2, 24, 45, 66, 75, 90, 170, 802]:
        ok = False
    if radix_sort_lsd([]) != []:
        ok = False

    freq: dict[int, int] = frequency_count([1, 2, 2, 3, 3, 3])
    if freq[3] != 3:
        ok = False
    if freq[1] != 1:
        ok = False

    top2: list[int] = top_k_frequent([1, 1, 1, 2, 2, 3], 2)
    if top2[0] != 1:
        ok = False
    if top2[1] != 2:
        ok = False

    sorted_freq: list[int] = sort_by_frequency([1, 1, 2, 2, 2, 3])
    if sorted_freq[0] != 2:
        ok = False

    return ok
