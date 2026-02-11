"""Counting patterns: occurrences, frequency distribution, mode finding.

Tests: count_occurrences, frequency_distribution, find_mode.
"""


def count_occurrences(arr: list[int], target: int) -> int:
    """Count how many times target appears in arr."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def frequency_distribution(arr: list[int]) -> dict[str, int]:
    """Return frequency of each element as string-keyed dict."""
    freq: dict[str, int] = {}
    i: int = 0
    while i < len(arr):
        key: str = str(arr[i])
        if key in freq:
            freq[key] = freq[key] + 1
        else:
            freq[key] = 1
        i = i + 1
    return freq


def find_mode(arr: list[int]) -> int:
    """Find the most frequent element. Returns first in case of tie."""
    if len(arr) == 0:
        return -1
    best_val: int = arr[0]
    best_count: int = 0
    i: int = 0
    while i < len(arr):
        c: int = count_occurrences(arr, arr[i])
        if c > best_count:
            best_count = c
            best_val = arr[i]
        i = i + 1
    return best_val


def count_unique(arr: list[int]) -> int:
    """Count unique elements in array."""
    seen: list[int] = []
    i: int = 0
    while i < len(arr):
        j: int = 0
        found: int = 0
        while j < len(seen):
            if seen[j] == arr[i]:
                found = 1
            j = j + 1
        if found == 0:
            seen.append(arr[i])
        i = i + 1
    return len(seen)


def test_module() -> int:
    """Test counting patterns."""
    ok: int = 0

    if count_occurrences([1, 2, 3, 2, 1, 2], 2) == 3:
        ok = ok + 1

    if count_occurrences([1, 2, 3], 5) == 0:
        ok = ok + 1

    freq: dict[str, int] = frequency_distribution([1, 2, 2, 3, 3, 3])
    if freq["3"] == 3:
        ok = ok + 1

    if find_mode([1, 2, 2, 3, 3, 3]) == 3:
        ok = ok + 1

    if find_mode([]) == -1:
        ok = ok + 1

    if count_unique([1, 2, 2, 3, 3, 3]) == 3:
        ok = ok + 1

    return ok
