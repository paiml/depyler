# Statistical functions (mean, variance, std dev)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def int_mean(nums: list[int]) -> int:
    """Compute the integer mean (truncated) of a list."""
    if len(nums) == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total // len(nums)


def int_variance(nums: list[int]) -> int:
    """Compute the integer variance (truncated) of a list."""
    if len(nums) == 0:
        return 0
    avg: int = int_mean(nums)
    total: int = 0
    i: int = 0
    while i < len(nums):
        diff: int = nums[i] - avg
        total = total + diff * diff
        i = i + 1
    return total // len(nums)


def int_median(nums: list[int]) -> int:
    """Compute the median of a sorted list (integer)."""
    if len(nums) == 0:
        return 0
    mid: int = len(nums) // 2
    return nums[mid]


def int_range(nums: list[int]) -> int:
    """Compute the range (max - min) of a list."""
    if len(nums) == 0:
        return 0
    lo: int = nums[0]
    hi: int = nums[0]
    i: int = 1
    while i < len(nums):
        if nums[i] < lo:
            lo = nums[i]
        if nums[i] > hi:
            hi = nums[i]
        i = i + 1
    return hi - lo


def percentile_rank(nums: list[int], value: int) -> int:
    """Compute what percentage of elements are below value (integer %)."""
    if len(nums) == 0:
        return 0
    count: int = 0
    i: int = 0
    while i < len(nums):
        if nums[i] < value:
            count = count + 1
        i = i + 1
    return (count * 100) // len(nums)


def test_module() -> int:
    assert int_mean([2, 4, 6, 8]) == 5
    assert int_mean([10]) == 10
    assert int_mean([]) == 0
    assert int_variance([2, 4, 6, 8]) == 5
    assert int_variance([5, 5, 5]) == 0
    assert int_variance([]) == 0
    assert int_median([1, 2, 3, 4, 5]) == 3
    assert int_median([10]) == 10
    assert int_median([]) == 0
    assert int_range([3, 7, 1, 9, 4]) == 8
    assert int_range([5]) == 0
    assert int_range([]) == 0
    assert percentile_rank([1, 2, 3, 4, 5], 3) == 40
    assert percentile_rank([1, 2, 3, 4, 5], 6) == 100
    assert percentile_rank([1, 2, 3, 4, 5], 1) == 0
    return 0


if __name__ == "__main__":
    test_module()
