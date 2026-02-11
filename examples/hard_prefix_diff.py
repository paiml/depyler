"""Prefix sums and difference arrays for range operations."""


def build_prefix_sum(arr: list[int]) -> list[int]:
    """Build prefix sum array where prefix[i] = sum(arr[0..i])."""
    n: int = len(arr)
    prefix: list[int] = []
    i: int = 0
    while i < n:
        prefix.append(0)
        i = i + 1
    if n == 0:
        return prefix
    prefix[0] = arr[0]
    i = 1
    while i < n:
        prefix[i] = prefix[i - 1] + arr[i]
        i = i + 1
    return prefix


def range_sum(prefix: list[int], left: int, right: int) -> int:
    """Sum of elements in range [left, right] using prefix sums."""
    total: int = prefix[right]
    if left > 0:
        total = total - prefix[left - 1]
    return total


def apply_diff_updates(n: int, updates: list[int], offsets: list[int], num_updates: int) -> list[int]:
    """Apply range increment updates using difference array.
    updates = flat [start, end, val, start, end, val, ...] triples.
    offsets stores starting index of each triple.
    """
    diff: list[int] = []
    j: int = 0
    while j <= n:
        diff.append(0)
        j = j + 1
    u: int = 0
    while u < num_updates:
        base: int = offsets[u]
        start: int = updates[base]
        end: int = updates[base + 1]
        val: int = updates[base + 2]
        diff[start] = diff[start] + val
        if end + 1 <= n:
            diff[end + 1] = diff[end + 1] - val
        u = u + 1
    result: list[int] = []
    k: int = 0
    while k < n:
        result.append(0)
        k = k + 1
    if n > 0:
        result[0] = diff[0]
    i: int = 1
    while i < n:
        result[i] = result[i - 1] + diff[i]
        i = i + 1
    return result


def equilibrium_index(arr: list[int]) -> int:
    """Find index where left sum equals right sum. Returns -1 if none."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            return j
        left_sum = left_sum + arr[j]
        j = j + 1
    return -1


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 2, 3, 4, 5]
    psum: list[int] = build_prefix_sum(arr1)
    if range_sum(psum, 1, 3) == 9:
        passed = passed + 1

    if range_sum(psum, 0, 4) == 15:
        passed = passed + 1

    updates: list[int] = [1, 3, 2, 0, 2, 3]
    offsets: list[int] = [0, 3]
    result: list[int] = apply_diff_updates(5, updates, offsets, 2)
    if result[2] == 5:
        passed = passed + 1

    if result[0] == 3:
        passed = passed + 1

    arr2: list[int] = [1, 3, 5, 2, 2]
    if equilibrium_index(arr2) == 2:
        passed = passed + 1

    arr3: list[int] = [1, 2, 3]
    if equilibrium_index(arr3) == -1:
        passed = passed + 1

    if range_sum(psum, 2, 2) == 3:
        passed = passed + 1

    return passed
