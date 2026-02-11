def partition(arr: list[int], low: int, high: int) -> int:
    pivot: int = arr[high]
    i: int = low
    j: int = low
    while j < high:
        if arr[j] <= pivot:
            tmp: int = arr[i]
            arr[i] = arr[j]
            arr[j] = tmp
            i = i + 1
        j = j + 1
    tmp2: int = arr[i]
    arr[i] = arr[high]
    arr[high] = tmp2
    return i


def quick_select(arr: list[int], k: int) -> int:
    work: list[int] = []
    i: int = 0
    while i < len(arr):
        work.append(arr[i])
        i = i + 1
    low: int = 0
    high: int = len(work) - 1
    while low <= high:
        pi: int = partition(work, low, high)
        if pi == k:
            return work[pi]
        elif pi < k:
            low = pi + 1
        else:
            high = pi - 1
    return -1


def find_kth_smallest(arr: list[int], k: int) -> int:
    return quick_select(arr, k - 1)


def find_median(arr: list[int]) -> int:
    n: int = len(arr)
    return quick_select(arr, n // 2)


def test_module() -> int:
    passed: int = 0
    if find_kth_smallest([7, 2, 1, 6, 8, 5], 1) == 1:
        passed = passed + 1
    if find_kth_smallest([7, 2, 1, 6, 8, 5], 3) == 5:
        passed = passed + 1
    if find_kth_smallest([7, 2, 1, 6, 8, 5], 6) == 8:
        passed = passed + 1
    if find_median([3, 1, 2, 5, 4]) == 3:
        passed = passed + 1
    if quick_select([10, 20, 30], 0) == 10:
        passed = passed + 1
    if quick_select([10, 20, 30], 2) == 30:
        passed = passed + 1
    if find_kth_smallest([42], 1) == 42:
        passed = passed + 1
    return passed
