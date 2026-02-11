def find_min_index(arr: list[int], start: int) -> int:
    min_idx: int = start
    i: int = start + 1
    while i < len(arr):
        if arr[i] < arr[min_idx]:
            min_idx = i
        i = i + 1
    return min_idx


def selection_sort(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < len(result) - 1:
        min_idx: int = find_min_index(result, i)
        tmp: int = result[i]
        result[i] = result[min_idx]
        result[min_idx] = tmp
        i = i + 1
    return result


def find_max_index(arr: list[int], start: int, end: int) -> int:
    max_idx: int = start
    i: int = start + 1
    while i <= end:
        if arr[i] > arr[max_idx]:
            max_idx = i
        i = i + 1
    return max_idx


def count_inversions(arr: list[int]) -> int:
    count: int = 0
    i: int = 0
    while i < len(arr):
        j: int = i + 1
        while j < len(arr):
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0
    if selection_sort([3, 1, 2]) == [1, 2, 3]:
        passed = passed + 1
    if selection_sort([]) == []:
        passed = passed + 1
    if selection_sort([5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5]:
        passed = passed + 1
    if selection_sort([1, 2, 3]) == [1, 2, 3]:
        passed = passed + 1
    if find_min_index([5, 3, 1, 4], 0) == 2:
        passed = passed + 1
    if find_max_index([5, 3, 8, 4], 0, 3) == 2:
        passed = passed + 1
    if count_inversions([3, 1, 2]) == 2:
        passed = passed + 1
    if count_inversions([1, 2, 3]) == 0:
        passed = passed + 1
    return passed
