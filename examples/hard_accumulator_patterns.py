# Accumulator patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def running_sum(nums: list[int]) -> list[int]:
    """Compute running sum of a list."""
    result: list[int] = []
    total: int = 0
    for n in nums:
        total = total + n
        result.append(total)
    return result


def running_product(nums: list[int]) -> list[int]:
    """Compute running product of a list."""
    result: list[int] = []
    prod: int = 1
    for n in nums:
        prod = prod * n
        result.append(prod)
    return result


def running_max(nums: list[int]) -> list[int]:
    """Compute running maximum of a list."""
    if len(nums) == 0:
        return []
    result: list[int] = []
    current_max: int = nums[0]
    for n in nums:
        if n > current_max:
            current_max = n
        result.append(current_max)
    return result


def count_above_threshold(nums: list[int], threshold: int) -> int:
    """Count how many elements exceed the threshold."""
    count: int = 0
    for n in nums:
        if n > threshold:
            count = count + 1
    return count


def weighted_sum(values: list[int], weights: list[int]) -> int:
    """Compute weighted sum of values."""
    total: int = 0
    i: int = 0
    limit: int = len(values)
    if len(weights) < limit:
        limit = len(weights)
    while i < limit:
        total = total + values[i] * weights[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test all accumulator pattern functions."""
    assert running_sum([1, 2, 3, 4]) == [1, 3, 6, 10]
    assert running_sum([]) == []
    assert running_sum([5]) == [5]
    assert running_product([1, 2, 3, 4]) == [1, 2, 6, 24]
    assert running_product([2, 3]) == [2, 6]
    assert running_max([3, 1, 4, 1, 5, 9]) == [3, 3, 4, 4, 5, 9]
    assert running_max([5, 4, 3, 2, 1]) == [5, 5, 5, 5, 5]
    assert count_above_threshold([1, 5, 3, 7, 2], 3) == 2
    assert count_above_threshold([1, 2, 3], 10) == 0
    assert weighted_sum([1, 2, 3], [10, 20, 30]) == 140
    assert weighted_sum([5, 10], [2, 3]) == 40
    return 0


if __name__ == "__main__":
    test_module()
