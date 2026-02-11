# Pathological multi-function: Recursive functions with multiple base cases
# Tests: recursive decomposition, multiple termination conditions


def power_recursive(x: int, n: int) -> int:
    """Compute x^n recursively with fast exponentiation."""
    if n == 0:
        return 1
    if n == 1:
        return x
    if n < 0:
        return 0
    if n % 2 == 0:
        half: int = power_recursive(x, n // 2)
        return half * half
    return x * power_recursive(x, n - 1)


def binary_search_recursive(nums: list[int], target: int, lo: int, hi: int) -> int:
    """Binary search returning index or -1."""
    if lo > hi:
        return 0 - 1
    mid: int = (lo + hi) // 2
    if nums[mid] == target:
        return mid
    if nums[mid] < target:
        return binary_search_recursive(nums, target, mid + 1, hi)
    return binary_search_recursive(nums, target, lo, mid - 1)


def merge_sorted(a: list[int], b: list[int]) -> list[int]:
    """Merge two sorted lists into one sorted list."""
    result: list[int] = []
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            result.append(a[i])
            i = i + 1
        else:
            result.append(b[j])
            j = j + 1
    while i < len(a):
        result.append(a[i])
        i = i + 1
    while j < len(b):
        result.append(b[j])
        j = j + 1
    return result


def sum_recursive(nums: list[int], idx: int) -> int:
    """Sum list elements recursively from idx to end."""
    if idx >= len(nums):
        return 0
    return nums[idx] + sum_recursive(nums, idx + 1)


def count_digits_recursive(n: int) -> int:
    """Count digits in a number recursively."""
    if n < 0:
        return count_digits_recursive(0 - n)
    if n < 10:
        return 1
    return 1 + count_digits_recursive(n // 10)


def flatten_depth(vals: list[int], depths: list[int], target_depth: int) -> list[int]:
    """Filter values that have depth <= target_depth (simulated nested structure)."""
    result: list[int] = []
    i: int = 0
    while i < len(vals):
        if depths[i] <= target_depth:
            result.append(vals[i])
        i = i + 1
    return result


def partition_list(nums: list[int], pivot: int) -> list[int]:
    """Partition: elements < pivot first, then >= pivot."""
    less: list[int] = []
    greater_eq: list[int] = []
    i: int = 0
    while i < len(nums):
        if nums[i] < pivot:
            less.append(nums[i])
        else:
            greater_eq.append(nums[i])
        i = i + 1
    return merge_sorted(less, greater_eq)


def test_module() -> int:
    passed: int = 0
    # Test 1: power
    if power_recursive(2, 10) == 1024:
        passed = passed + 1
    # Test 2: power base case
    if power_recursive(5, 0) == 1:
        passed = passed + 1
    # Test 3: binary search found
    sorted_list: list[int] = [1, 3, 5, 7, 9, 11, 13]
    if binary_search_recursive(sorted_list, 7, 0, 6) == 3:
        passed = passed + 1
    # Test 4: binary search not found
    if binary_search_recursive(sorted_list, 6, 0, 6) == 0 - 1:
        passed = passed + 1
    # Test 5: merge sorted
    merged: list[int] = merge_sorted([1, 3, 5], [2, 4, 6])
    if merged[0] == 1 and merged[3] == 4 and merged[5] == 6:
        passed = passed + 1
    # Test 6: sum recursive
    if sum_recursive([10, 20, 30, 40], 0) == 100:
        passed = passed + 1
    # Test 7: count digits
    if count_digits_recursive(12345) == 5:
        passed = passed + 1
    return passed
