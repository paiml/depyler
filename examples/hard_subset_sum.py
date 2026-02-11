"""Subset sum variants.

Tests: subset sum exists, count subsets, min subset size, exact subset sum.
"""


def subset_sum_exists(arr: list[int], target: int) -> int:
    """Check if any subset sums to target. Returns 1 or 0."""
    n: int = len(arr)
    count: int = 1 << n
    i: int = 0
    while i < count:
        total: int = 0
        j: int = 0
        while j < n:
            bit: int = (i >> j) % 2
            if bit == 1:
                total = total + arr[j]
            j = j + 1
        if total == target:
            return 1
        i = i + 1
    return 0


def count_subset_sums(arr: list[int], target: int) -> int:
    """Count number of subsets that sum to target."""
    n: int = len(arr)
    count: int = 1 << n
    result: int = 0
    i: int = 0
    while i < count:
        total: int = 0
        j: int = 0
        while j < n:
            bit: int = (i >> j) % 2
            if bit == 1:
                total = total + arr[j]
            j = j + 1
        if total == target:
            result = result + 1
        i = i + 1
    return result


def min_subset_size(arr: list[int], target: int) -> int:
    """Minimum number of elements that sum to target. Returns -1 if impossible."""
    n: int = len(arr)
    count: int = 1 << n
    best: int = n + 1
    i: int = 0
    while i < count:
        total: int = 0
        bits: int = 0
        j: int = 0
        while j < n:
            bit: int = (i >> j) % 2
            if bit == 1:
                total = total + arr[j]
                bits = bits + 1
            j = j + 1
        if total == target and bits < best:
            best = bits
        i = i + 1
    if best == n + 1:
        return -1
    return best


def max_subset_sum_le(arr: list[int], limit: int) -> int:
    """Maximum subset sum that does not exceed limit."""
    n: int = len(arr)
    count: int = 1 << n
    best: int = 0
    i: int = 0
    while i < count:
        total: int = 0
        j: int = 0
        while j < n:
            bit: int = (i >> j) % 2
            if bit == 1:
                total = total + arr[j]
            j = j + 1
        if total <= limit and total > best:
            best = total
        i = i + 1
    return best


def test_module() -> int:
    """Test subset sum operations."""
    ok: int = 0
    if subset_sum_exists([3, 7, 1, 8], 11) == 1:
        ok = ok + 1
    if subset_sum_exists([3, 7, 1, 8], 20) == 0:
        ok = ok + 1
    if count_subset_sums([1, 2, 3, 4], 5) == 3:
        ok = ok + 1
    if min_subset_size([1, 2, 3, 4], 6) == 2:
        ok = ok + 1
    if min_subset_size([1, 2, 3], 10) == -1:
        ok = ok + 1
    if max_subset_sum_le([3, 5, 7], 10) == 10:
        ok = ok + 1
    return ok
