def merge_sorted(a: list[int], b: list[int]) -> list[int]:
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


def merge_k_sorted(arrays: list[list[int]]) -> list[int]:
    if len(arrays) == 0:
        return []
    result: list[int] = arrays[0]
    i: int = 1
    while i < len(arrays):
        result = merge_sorted(result, arrays[i])
        i = i + 1
    return result


def find_median_sorted(a: list[int], b: list[int]) -> int:
    merged: list[int] = merge_sorted(a, b)
    n: int = len(merged)
    if n == 0:
        return 0
    return merged[n // 2]


def count_common(a: list[int], b: list[int]) -> int:
    count: int = 0
    i: int = 0
    j: int = 0
    while i < len(a) and j < len(b):
        if a[i] == b[j]:
            count = count + 1
            i = i + 1
            j = j + 1
        elif a[i] < b[j]:
            i = i + 1
        else:
            j = j + 1
    return count


def test_module() -> int:
    passed: int = 0
    if merge_sorted([1, 3, 5], [2, 4, 6]) == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1
    if merge_sorted([], [1, 2]) == [1, 2]:
        passed = passed + 1
    if merge_sorted([1], []) == [1]:
        passed = passed + 1
    if merge_k_sorted([[1, 4], [2, 5], [3, 6]]) == [1, 2, 3, 4, 5, 6]:
        passed = passed + 1
    if merge_k_sorted([]) == []:
        passed = passed + 1
    if find_median_sorted([1, 3], [2, 4]) == 3:
        passed = passed + 1
    if count_common([1, 2, 3, 4], [2, 4, 6]) == 2:
        passed = passed + 1
    if count_common([1, 2], [3, 4]) == 0:
        passed = passed + 1
    return passed
