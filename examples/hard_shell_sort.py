def shell_sort(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    n: int = len(result)
    gap: int = n // 2
    while gap > 0:
        i = gap
        while i < n:
            temp: int = result[i]
            j: int = i
            while j >= gap and result[j - gap] > temp:
                result[j] = result[j - gap]
                j = j - gap
            result[j] = temp
            i = i + 1
        gap = gap // 2
    return result


def shell_sort_knuth_gaps(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    n: int = len(result)
    gap: int = 1
    while gap < n // 3:
        gap = gap * 3 + 1
    while gap > 0:
        i = gap
        while i < n:
            temp: int = result[i]
            j: int = i
            while j >= gap and result[j - gap] > temp:
                result[j] = result[j - gap]
                j = j - gap
            result[j] = temp
            i = i + 1
        gap = gap // 3
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
    if shell_sort([5, 3, 8, 1, 2]) == [1, 2, 3, 5, 8]:
        passed = passed + 1
    if shell_sort([]) == []:
        passed = passed + 1
    if shell_sort([1]) == [1]:
        passed = passed + 1
    if shell_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]:
        passed = passed + 1
    if shell_sort_knuth_gaps([5, 3, 8, 1, 2]) == [1, 2, 3, 5, 8]:
        passed = passed + 1
    if shell_sort_knuth_gaps([10, 9, 8, 7, 6]) == [5, 6, 7, 8, 9, 10]:
        passed = passed + 0
    if is_sorted(shell_sort_knuth_gaps([10, 9, 8, 7, 6])) == 1:
        passed = passed + 1
    if is_sorted([5, 3, 1]) == 0:
        passed = passed + 1
    return passed
