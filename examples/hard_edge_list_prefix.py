"""Prefix sums, range queries, and cumulative operations."""


def build_prefix_sum(arr: list[int]) -> list[int]:
    """Build prefix sum array. prefix[i] = sum(arr[0..i-1])."""
    prefix: list[int] = [0]
    i: int = 0
    while i < len(arr):
        last: int = prefix[len(prefix) - 1]
        prefix.append(last + arr[i])
        i = i + 1
    return prefix


def range_sum(prefix: list[int], lo: int, hi: int) -> int:
    """Query sum of range [lo, hi] using prefix sum array."""
    return prefix[hi + 1] - prefix[lo]


def build_prefix_max(arr: list[int]) -> list[int]:
    """Build prefix max array."""
    if len(arr) == 0:
        return []
    result: list[int] = [arr[0]]
    i: int = 1
    while i < len(arr):
        prev: int = result[i - 1]
        if arr[i] > prev:
            result.append(arr[i])
        else:
            result.append(prev)
        i = i + 1
    return result


def build_suffix_min(arr: list[int]) -> list[int]:
    """Build suffix min array (right to left)."""
    n: int = len(arr)
    if n == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(0)
        i = i + 1
    last_idx: int = n - 1
    result[last_idx] = arr[last_idx]
    i = n - 2
    while i >= 0:
        if arr[i] < result[i + 1]:
            result[i] = arr[i]
        else:
            result[i] = result[i + 1]
        i = i - 1
    return result


def count_range(arr: list[int], lo_val: int, hi_val: int) -> int:
    """Count elements in range [lo_val, hi_val]."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] >= lo_val and arr[i] <= hi_val:
            count = count + 1
        i = i + 1
    return count


def equilibrium_index(arr: list[int]) -> int:
    """Find index where left sum equals right sum. Return -1 if none."""
    n: int = len(arr)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    i = 0
    while i < n:
        right_sum: int = total - left_sum - arr[i]
        if left_sum == right_sum:
            return i
        left_sum = left_sum + arr[i]
        i = i + 1
    return -1


def max_subarray_sum(arr: list[int]) -> int:
    """Kadane's algorithm: find maximum subarray sum."""
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = arr[0]
    current: int = arr[0]
    i: int = 1
    while i < n:
        if current + arr[i] > arr[i]:
            current = current + arr[i]
        else:
            current = arr[i]
        if current > best:
            best = current
        i = i + 1
    return best


def product_except_self(arr: list[int]) -> list[int]:
    """For each index, compute product of all other elements."""
    n: int = len(arr)
    if n == 0:
        return []
    left_prod: list[int] = []
    i: int = 0
    while i < n:
        left_prod.append(1)
        i = i + 1
    i = 1
    while i < n:
        left_prod[i] = left_prod[i - 1] * arr[i - 1]
        i = i + 1
    right_prod: int = 1
    result: list[int] = []
    i = 0
    while i < n:
        result.append(0)
        i = i + 1
    i = n - 1
    while i >= 0:
        result[i] = left_prod[i] * right_prod
        right_prod = right_prod * arr[i]
        i = i - 1
    return result


def test_module() -> int:
    """Test all prefix sum and range query functions."""
    passed: int = 0
    prefix: list[int] = build_prefix_sum([1, 2, 3, 4, 5])
    if range_sum(prefix, 0, 4) == 15:
        passed = passed + 1
    if range_sum(prefix, 1, 3) == 9:
        passed = passed + 1
    if range_sum(prefix, 2, 2) == 3:
        passed = passed + 1
    pmax: list[int] = build_prefix_max([3, 1, 4, 1, 5])
    if pmax == [3, 3, 4, 4, 5]:
        passed = passed + 1
    smin: list[int] = build_suffix_min([3, 1, 4, 1, 5])
    if smin == [1, 1, 1, 1, 5]:
        passed = passed + 1
    if count_range([1, 5, 3, 7, 2, 8], 3, 7) == 3:
        passed = passed + 1
    eq: int = equilibrium_index([1, 3, 5, 2, 2])
    if eq == 2:
        passed = passed + 1
    ms: int = max_subarray_sum([0 - 2, 1, 0 - 3, 4, 0 - 1, 2, 1, 0 - 5, 4])
    if ms == 6:
        passed = passed + 1
    prod: list[int] = product_except_self([1, 2, 3, 4])
    if prod == [24, 12, 8, 6]:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
