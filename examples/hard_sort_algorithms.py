# Sorting algorithms (bubble, selection, insertion)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def bubble_sort(nums: list[int]) -> list[int]:
    """Sort a list using bubble sort (returns new list)."""
    result: list[int] = []
    k: int = 0
    while k < len(nums):
        result.append(nums[k])
        k = k + 1
    n: int = len(result)
    i: int = 0
    while i < n:
        j: int = 0
        while j < n - i - 1:
            if result[j] > result[j + 1]:
                temp: int = result[j]
                result[j] = result[j + 1]
                result[j + 1] = temp
            j = j + 1
        i = i + 1
    return result


def selection_sort(nums: list[int]) -> list[int]:
    """Sort a list using selection sort (returns new list)."""
    result: list[int] = []
    k: int = 0
    while k < len(nums):
        result.append(nums[k])
        k = k + 1
    n: int = len(result)
    i: int = 0
    while i < n:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if result[j] < result[min_idx]:
                min_idx = j
            j = j + 1
        if min_idx != i:
            temp: int = result[i]
            result[i] = result[min_idx]
            result[min_idx] = temp
        i = i + 1
    return result


def insertion_sort(nums: list[int]) -> list[int]:
    """Sort a list using insertion sort (returns new list)."""
    result: list[int] = []
    k: int = 0
    while k < len(nums):
        result.append(nums[k])
        k = k + 1
    i: int = 1
    while i < len(result):
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = key
        i = i + 1
    return result


def is_sorted(nums: list[int]) -> bool:
    """Check if a list is sorted in non-decreasing order."""
    if len(nums) < 2:
        return True
    i: int = 0
    while i < len(nums) - 1:
        if nums[i] > nums[i + 1]:
            return False
        i = i + 1
    return True


def test_module() -> int:
    assert bubble_sort([5, 3, 8, 1, 9, 2]) == [1, 2, 3, 5, 8, 9]
    assert bubble_sort([]) == []
    assert bubble_sort([1]) == [1]
    assert selection_sort([5, 3, 8, 1, 9, 2]) == [1, 2, 3, 5, 8, 9]
    assert selection_sort([3, 1]) == [1, 3]
    assert insertion_sort([5, 3, 8, 1, 9, 2]) == [1, 2, 3, 5, 8, 9]
    assert insertion_sort([1, 2, 3]) == [1, 2, 3]
    assert is_sorted([1, 2, 3, 4]) == True
    assert is_sorted([4, 3, 2, 1]) == False
    assert is_sorted([]) == True
    assert is_sorted([7]) == True
    return 0


if __name__ == "__main__":
    test_module()
