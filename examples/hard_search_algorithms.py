# Various search algorithms (linear, binary, interpolation)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def linear_search(nums: list[int], target: int) -> int:
    """Linear search, returns index or -1."""
    i: int = 0
    while i < len(nums):
        if nums[i] == target:
            return i
        i = i + 1
    return -1


def binary_search(nums: list[int], target: int) -> int:
    """Binary search on a sorted list, returns index or -1."""
    lo: int = 0
    hi: int = len(nums) - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if nums[mid] == target:
            return mid
        elif nums[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def find_min(nums: list[int]) -> int:
    """Find the minimum value in a list. Returns 0 if empty."""
    if len(nums) == 0:
        return 0
    result: int = nums[0]
    i: int = 1
    while i < len(nums):
        if nums[i] < result:
            result = nums[i]
        i = i + 1
    return result


def find_max(nums: list[int]) -> int:
    """Find the maximum value in a list. Returns 0 if empty."""
    if len(nums) == 0:
        return 0
    result: int = nums[0]
    i: int = 1
    while i < len(nums):
        if nums[i] > result:
            result = nums[i]
        i = i + 1
    return result


def count_occurrences(nums: list[int], target: int) -> int:
    """Count how many times target appears in nums."""
    count: int = 0
    i: int = 0
    while i < len(nums):
        if nums[i] == target:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    assert linear_search([10, 20, 30, 40], 30) == 2
    assert linear_search([10, 20, 30], 99) == -1
    assert linear_search([], 1) == -1
    assert binary_search([1, 3, 5, 7, 9, 11], 7) == 3
    assert binary_search([1, 3, 5, 7, 9], 4) == -1
    assert binary_search([], 1) == -1
    assert find_min([5, 3, 8, 1, 9]) == 1
    assert find_min([42]) == 42
    assert find_max([5, 3, 8, 1, 9]) == 9
    assert find_max([42]) == 42
    assert count_occurrences([1, 2, 3, 2, 1, 2], 2) == 3
    assert count_occurrences([1, 2, 3], 5) == 0
    return 0


if __name__ == "__main__":
    test_module()
