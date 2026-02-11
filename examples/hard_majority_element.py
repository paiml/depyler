"""Majority element finding using Boyer-Moore voting algorithm.

Tests: majority element, verify majority, frequency count.
"""


def majority_candidate(arr: list[int]) -> int:
    """Find majority element candidate using Boyer-Moore voting."""
    candidate: int = arr[0]
    count: int = 1
    i: int = 1
    while i < len(arr):
        if count == 0:
            candidate = arr[i]
            count = 1
        elif arr[i] == candidate:
            count = count + 1
        else:
            count = count - 1
        i = i + 1
    return candidate


def verify_majority(arr: list[int], candidate: int) -> int:
    """Verify if candidate is actual majority. Returns 1 if yes."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == candidate:
            count = count + 1
        i = i + 1
    if count * 2 > len(arr):
        return 1
    return 0


def element_frequency(arr: list[int], target: int) -> int:
    """Count frequency of target in array."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def most_frequent_value(arr: list[int]) -> int:
    """Find the most frequent value (first occurrence wins ties)."""
    if len(arr) == 0:
        return 0
    best_val: int = arr[0]
    best_count: int = 1
    i: int = 0
    while i < len(arr):
        count: int = 0
        j: int = 0
        while j < len(arr):
            if arr[j] == arr[i]:
                count = count + 1
            j = j + 1
        if count > best_count:
            best_count = count
            best_val = arr[i]
        i = i + 1
    return best_val


def test_module() -> int:
    """Test majority element operations."""
    ok: int = 0
    arr: list[int] = [3, 3, 4, 2, 3, 3, 5, 3, 3]
    cand: int = majority_candidate(arr)
    if cand == 3:
        ok = ok + 1
    if verify_majority(arr, 3) == 1:
        ok = ok + 1
    if verify_majority(arr, 4) == 0:
        ok = ok + 1
    if element_frequency(arr, 3) == 6:
        ok = ok + 1
    if most_frequent_value([1, 2, 2, 3, 3, 3]) == 3:
        ok = ok + 1
    return ok
