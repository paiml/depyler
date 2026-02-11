# Prefix sum, suffix sum, sliding window
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def prefix_sum(nums: list[int]) -> list[int]:
    """Compute prefix sums of a list."""
    result: list[int] = []
    total: int = 0
    i: int = 0
    while i < len(nums):
        total = total + nums[i]
        result.append(total)
        i = i + 1
    return result


def suffix_sum(nums: list[int]) -> list[int]:
    """Compute suffix sums of a list."""
    result: list[int] = []
    i: int = 0
    while i < len(nums):
        result.append(0)
        i = i + 1
    if len(nums) == 0:
        return result
    idx: int = len(nums) - 1
    total: int = 0
    while idx >= 0:
        total = total + nums[idx]
        result[idx] = total
        idx = idx - 1
    return result


def max_subarray_sum(nums: list[int]) -> int:
    """Find the maximum subarray sum (Kadane's algorithm)."""
    if len(nums) == 0:
        return 0
    max_sum: int = nums[0]
    current: int = nums[0]
    i: int = 1
    while i < len(nums):
        if current + nums[i] > nums[i]:
            current = current + nums[i]
        else:
            current = nums[i]
        if current > max_sum:
            max_sum = current
        i = i + 1
    return max_sum


def sliding_window_sum(nums: list[int], window_size: int) -> list[int]:
    """Compute sums over a sliding window of given size."""
    if window_size <= 0 or window_size > len(nums):
        return []
    result: list[int] = []
    window_sum: int = 0
    i: int = 0
    while i < window_size:
        window_sum = window_sum + nums[i]
        i = i + 1
    result.append(window_sum)
    j: int = window_size
    while j < len(nums):
        window_sum = window_sum + nums[j] - nums[j - window_size]
        result.append(window_sum)
        j = j + 1
    return result


def test_module() -> int:
    assert prefix_sum([1, 2, 3, 4]) == [1, 3, 6, 10]
    assert prefix_sum([]) == []
    assert prefix_sum([5]) == [5]
    assert suffix_sum([1, 2, 3, 4]) == [10, 9, 7, 4]
    assert suffix_sum([]) == []
    assert max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
    assert max_subarray_sum([1, 2, 3]) == 6
    assert max_subarray_sum([-1, -2, -3]) == -1
    assert sliding_window_sum([1, 2, 3, 4, 5], 3) == [6, 9, 12]
    assert sliding_window_sum([1, 2, 3], 1) == [1, 2, 3]
    assert sliding_window_sum([1, 2], 3) == []
    return 0


if __name__ == "__main__":
    test_module()
