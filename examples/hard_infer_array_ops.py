# Type inference test: Array manipulations
# Strategy: Only test_module() -> int annotated, everything else inferred


def arr_sum(arr: list[int]):
    """Sum all elements - return type inferred from arithmetic."""
    total = 0
    i = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return total


def arr_max_val(arr: list[int]):
    """Find maximum value - return type inferred."""
    if len(arr) == 0:
        return 0
    best = 0
    i = 0
    while i < len(arr):
        if arr[i] > best:
            best = arr[i]
        i = i + 1
    return best


def arr_min_val(arr: list[int]):
    """Find minimum value in array of non-negative ints."""
    if len(arr) == 0:
        return 0
    best = arr[0]
    i = 1
    while i < len(arr):
        if arr[i] < best:
            best = arr[i]
        i = i + 1
    return best


def arr_count_val(arr: list[int], target):
    """Count occurrences of target value."""
    count = 0
    i = 0
    while i < len(arr):
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def arr_reverse_sum(arr: list[int]):
    """Sum computed by traversing in reverse."""
    total = 0
    i = len(arr) - 1
    while i >= 0:
        total = total + arr[i]
        i = i - 1
    return total


def arr_dot_product(a: list[int], b: list[int]):
    """Dot product of two arrays."""
    n: int = len(a)
    if len(b) < n:
        n = len(b)
    total = 0
    i = 0
    while i < n:
        total = total + a[i] * b[i]
        i = i + 1
    return total


def arr_prefix_sums(arr: list[int]):
    """Compute prefix sums array and return its last element."""
    if len(arr) == 0:
        return 0
    running = 0
    i = 0
    while i < len(arr):
        running = running + arr[i]
        i = i + 1
    return running


def arr_second_largest(arr: list[int]):
    """Find second largest element."""
    if len(arr) < 2:
        return 0
    first = 0
    second = 0
    i = 0
    while i < len(arr):
        if arr[i] > first:
            second = first
            first = arr[i]
        elif arr[i] > second and arr[i] != first:
            second = arr[i]
        i = i + 1
    return second


def test_module() -> int:
    """Test all array ops inference functions."""
    total: int = 0

    nums: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]

    # arr_sum tests
    if arr_sum(nums) == 31:
        total = total + 1
    if arr_sum([]) == 0:
        total = total + 1

    # arr_max_val tests
    if arr_max_val(nums) == 9:
        total = total + 1
    if arr_max_val([]) == 0:
        total = total + 1

    # arr_min_val tests
    if arr_min_val(nums) == 1:
        total = total + 1

    # arr_count_val tests
    if arr_count_val(nums, 1) == 2:
        total = total + 1
    if arr_count_val(nums, 7) == 0:
        total = total + 1

    # arr_reverse_sum tests
    if arr_reverse_sum(nums) == 31:
        total = total + 1

    # arr_dot_product tests
    if arr_dot_product([1, 2, 3], [4, 5, 6]) == 32:
        total = total + 1
    if arr_dot_product([], [1, 2]) == 0:
        total = total + 1

    # arr_prefix_sums tests
    if arr_prefix_sums([1, 2, 3, 4]) == 10:
        total = total + 1

    # arr_second_largest tests
    if arr_second_largest([5, 3, 9, 7, 1]) == 7:
        total = total + 1
    if arr_second_largest([1]) == 0:
        total = total + 1

    return total
