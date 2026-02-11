"""Count inversions in an array (pairs where i < j but arr[i] > arr[j])."""


def count_inversions_brute(arr: list[int]) -> int:
    """Count inversions using brute force O(n^2)."""
    count: int = 0
    length: int = len(arr)
    i: int = 0
    while i < length:
        j: int = i + 1
        while j < length:
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def merge_count(arr: list[int], temp: list[int], left: int, right: int) -> int:
    """Merge two sorted halves and count split inversions."""
    mid: int = (left + right) // 2
    i: int = left
    j: int = mid + 1
    idx: int = left
    inversions: int = 0
    while i <= mid and j <= right:
        if arr[i] <= arr[j]:
            temp[idx] = arr[i]
            i = i + 1
        else:
            temp[idx] = arr[j]
            inversions = inversions + (mid - i + 1)
            j = j + 1
        idx = idx + 1
    while i <= mid:
        temp[idx] = arr[i]
        i = i + 1
        idx = idx + 1
    while j <= right:
        temp[idx] = arr[j]
        j = j + 1
        idx = idx + 1
    k: int = left
    while k <= right:
        arr[k] = temp[k]
        k = k + 1
    return inversions


def sort_and_count(arr: list[int], temp: list[int], left: int, right: int) -> int:
    """Recursively sort and count inversions via merge sort."""
    if left >= right:
        return 0
    mid: int = (left + right) // 2
    inv_left: int = sort_and_count(arr, temp, left, mid)
    inv_right: int = sort_and_count(arr, temp, mid + 1, right)
    inv_split: int = merge_count(arr, temp, left, right)
    return inv_left + inv_right + inv_split


def count_inversions_fast(arr: list[int]) -> int:
    """Count inversions using merge sort approach O(n log n)."""
    length: int = len(arr)
    if length <= 1:
        return 0
    work: list[int] = []
    temp: list[int] = []
    i: int = 0
    while i < length:
        work.append(arr[i])
        temp.append(0)
        i = i + 1
    last_idx: int = length - 1
    result: int = sort_and_count(work, temp, 0, last_idx)
    return result


def test_module() -> int:
    """Test inversion counting operations."""
    passed: int = 0

    r1: int = count_inversions_brute([1, 2, 3])
    if r1 == 0:
        passed = passed + 1

    r2: int = count_inversions_brute([3, 2, 1])
    if r2 == 3:
        passed = passed + 1

    r3: int = count_inversions_brute([2, 4, 1, 3, 5])
    if r3 == 3:
        passed = passed + 1

    r4: int = count_inversions_fast([1, 2, 3])
    if r4 == 0:
        passed = passed + 1

    r5: int = count_inversions_fast([3, 2, 1])
    if r5 == 3:
        passed = passed + 1

    r6: int = count_inversions_fast([2, 4, 1, 3, 5])
    if r6 == 3:
        passed = passed + 1

    r7: int = count_inversions_fast([])
    if r7 == 0:
        passed = passed + 1

    r8: int = count_inversions_fast([1])
    if r8 == 0:
        passed = passed + 1

    return passed
