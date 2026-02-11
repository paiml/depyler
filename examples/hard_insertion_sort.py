def insertion_sort(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    i = 1
    while i < len(result):
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = key
        i = i + 1
    return result


def binary_search_insert_pos(arr: list[int], val: int, low: int, high: int) -> int:
    while low < high:
        mid: int = (low + high) // 2
        if arr[mid] < val:
            low = mid + 1
        else:
            high = mid
    return low


def binary_insertion_sort(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    i = 1
    while i < len(result):
        key: int = result[i]
        pos: int = binary_search_insert_pos(result, key, 0, i)
        j: int = i
        while j > pos:
            result[j] = result[j - 1]
            j = j - 1
        result[pos] = key
        i = i + 1
    return result


def is_sorted(arr: list[int]) -> int:
    if len(arr) <= 1:
        return 1
    i: int = 0
    while i < len(arr) - 1:
        if arr[i] > arr[i + 1]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    passed: int = 0
    if insertion_sort([3, 1, 2]) == [1, 2, 3]:
        passed = passed + 1
    if insertion_sort([]) == []:
        passed = passed + 1
    if insertion_sort([1]) == [1]:
        passed = passed + 1
    if insertion_sort([5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5]:
        passed = passed + 1
    if binary_insertion_sort([3, 1, 2]) == [1, 2, 3]:
        passed = passed + 1
    if binary_insertion_sort([5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5]:
        passed = passed + 1
    if is_sorted([1, 2, 3]) == 1:
        passed = passed + 1
    if is_sorted([3, 1, 2]) == 0:
        passed = passed + 1
    if binary_search_insert_pos([1, 3, 5], 4, 0, 3) == 2:
        passed = passed + 1
    return passed
