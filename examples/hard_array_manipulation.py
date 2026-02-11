# Array manipulation (rotate, reverse sections, merge sorted)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def reverse_list(nums: list[int]) -> list[int]:
    """Reverse a list."""
    result: list[int] = []
    i: int = len(nums) - 1
    while i >= 0:
        result.append(nums[i])
        i = i - 1
    return result


def rotate_left(nums: list[int], k: int) -> list[int]:
    """Rotate a list left by k positions."""
    if len(nums) == 0:
        return []
    shift: int = k % len(nums)
    result: list[int] = []
    i: int = shift
    while i < len(nums):
        result.append(nums[i])
        i = i + 1
    j: int = 0
    while j < shift:
        result.append(nums[j])
        j = j + 1
    return result


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


def remove_duplicates_sorted(nums: list[int]) -> list[int]:
    """Remove duplicates from a sorted list."""
    if len(nums) == 0:
        return []
    result: list[int] = [nums[0]]
    i: int = 1
    while i < len(nums):
        if nums[i] != nums[i - 1]:
            result.append(nums[i])
        i = i + 1
    return result


def test_module() -> int:
    assert reverse_list([1, 2, 3, 4, 5]) == [5, 4, 3, 2, 1]
    assert reverse_list([]) == []
    assert reverse_list([42]) == [42]
    assert rotate_left([1, 2, 3, 4, 5], 2) == [3, 4, 5, 1, 2]
    assert rotate_left([1, 2, 3], 0) == [1, 2, 3]
    assert rotate_left([], 3) == []
    assert merge_sorted([1, 3, 5], [2, 4, 6]) == [1, 2, 3, 4, 5, 6]
    assert merge_sorted([], [1, 2]) == [1, 2]
    assert merge_sorted([1, 2], []) == [1, 2]
    assert remove_duplicates_sorted([1, 1, 2, 3, 3, 3, 4]) == [1, 2, 3, 4]
    assert remove_duplicates_sorted([5]) == [5]
    assert remove_duplicates_sorted([]) == []
    return 0


if __name__ == "__main__":
    test_module()
