"""Power set and subset operations.

Tests: subset count, subset sum count, k-subset count, complement size.
"""


def power_set_size(n: int) -> int:
    """Number of subsets of a set with n elements (2^n)."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result


def count_subsets_with_sum(arr: list[int], target: int) -> int:
    """Count subsets that sum to target using bitmask enumeration."""
    n: int = len(arr)
    count: int = 0
    mask: int = 0
    total_masks: int = power_set_size(n)
    while mask < total_masks:
        total: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                total = total + arr[bit]
            bit = bit + 1
        if total == target:
            count = count + 1
        mask = mask + 1
    return count


def count_k_subsets(n: int, k: int) -> int:
    """Count subsets of size exactly k from n elements (C(n,k))."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    kk: int = k
    if kk > n - kk:
        kk = n - kk
    result: int = 1
    i: int = 0
    while i < kk:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def max_subset_sum_no_adjacent(arr: list[int]) -> int:
    """Maximum sum of non-adjacent elements."""
    n: int = len(arr)
    if n == 0:
        return 0
    if n == 1:
        return arr[0]
    prev2: int = arr[0]
    prev1: int = arr[0]
    if arr[1] > prev1:
        prev1 = arr[1]
    i: int = 2
    while i < n:
        candidate: int = prev2 + arr[i]
        curr: int = prev1
        if candidate > curr:
            curr = candidate
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1


def test_module() -> int:
    """Test power set operations."""
    ok: int = 0
    if power_set_size(0) == 1:
        ok = ok + 1
    if power_set_size(3) == 8:
        ok = ok + 1
    if power_set_size(5) == 32:
        ok = ok + 1
    if count_subsets_with_sum([1, 2, 3], 3) == 2:
        ok = ok + 1
    if count_subsets_with_sum([1, 1, 1], 2) == 3:
        ok = ok + 1
    if count_k_subsets(5, 2) == 10:
        ok = ok + 1
    if count_k_subsets(4, 0) == 1:
        ok = ok + 1
    if max_subset_sum_no_adjacent([3, 2, 7, 10]) == 13:
        ok = ok + 1
    return ok
