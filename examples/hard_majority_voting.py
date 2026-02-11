"""Boyer-Moore majority voting algorithm and variants.

Tests: candidate tracking, verification pass, frequency analysis.
"""


def majority_element(arr: list[int]) -> int:
    """Find majority element using Boyer-Moore voting."""
    if len(arr) == 0:
        return -1
    candidate: int = arr[0]
    count: int = 1
    i: int = 1
    while i < len(arr):
        if count == 0:
            candidate = arr[i]
            count = 1
        elif arr[i] == candidate:
            count += 1
        else:
            count -= 1
        i += 1
    return candidate


def verify_majority(arr: list[int], candidate: int) -> bool:
    """Verify that candidate is actually a majority element."""
    count: int = 0
    for v in arr:
        if v == candidate:
            count += 1
    return count > len(arr) // 2


def find_majority_if_exists(arr: list[int]) -> int:
    """Find majority element, return -1 if none exists."""
    candidate: int = majority_element(arr)
    if verify_majority(arr, candidate):
        return candidate
    return -1


def element_frequency(arr: list[int], target: int) -> int:
    """Count frequency of target in array."""
    count: int = 0
    for v in arr:
        if v == target:
            count += 1
    return count


def most_frequent(arr: list[int]) -> int:
    """Find most frequent element (brute force)."""
    if len(arr) == 0:
        return -1
    best: int = arr[0]
    best_count: int = 0
    i: int = 0
    while i < len(arr):
        c: int = element_frequency(arr, arr[i])
        if c > best_count:
            best_count = c
            best = arr[i]
        i += 1
    return best


def appears_more_than_n_over_k(arr: list[int], target: int, k: int) -> bool:
    """Check if target appears more than n/k times."""
    if k == 0:
        return False
    threshold: int = len(arr) // k
    count: int = element_frequency(arr, target)
    return count > threshold


def test_module() -> int:
    """Test majority voting operations."""
    ok: int = 0

    m: int = majority_element([3, 3, 4, 2, 3, 3, 3])
    if m == 3:
        ok += 1

    if verify_majority([3, 3, 4, 2, 3, 3, 3], 3):
        ok += 1

    r: int = find_majority_if_exists([1, 2, 3])
    if r == -1:
        ok += 1

    r2: int = find_majority_if_exists([1, 1, 1, 2, 3])
    if r2 == 1:
        ok += 1

    mf: int = most_frequent([1, 2, 2, 3, 3, 3])
    if mf == 3:
        ok += 1

    if appears_more_than_n_over_k([1, 1, 1, 2, 3, 4, 5], 1, 3):
        ok += 1

    return ok
