"""Count inversions in an array using merge sort approach."""


def merge_count(arr: list[int], temp: list[int], left: int, mid: int, right: int) -> int:
    """Merge two halves and count inversions."""
    i: int = left
    j: int = mid + 1
    k: int = left
    inv_count: int = 0
    while i <= mid and j <= right:
        if arr[i] <= arr[j]:
            temp[k] = arr[i]
            i = i + 1
        else:
            temp[k] = arr[j]
            inv_count = inv_count + (mid - i + 1)
            j = j + 1
        k = k + 1
    while i <= mid:
        temp[k] = arr[i]
        i = i + 1
        k = k + 1
    while j <= right:
        temp[k] = arr[j]
        j = j + 1
        k = k + 1
    i = left
    while i <= right:
        arr[i] = temp[i]
        i = i + 1
    return inv_count


def merge_sort_count(arr: list[int], temp: list[int], left: int, right: int) -> int:
    """Recursive merge sort counting inversions."""
    inv_count: int = 0
    if left < right:
        mid: int = (left + right) // 2
        inv_count = inv_count + merge_sort_count(arr, temp, left, mid)
        inv_count = inv_count + merge_sort_count(arr, temp, mid + 1, right)
        inv_count = inv_count + merge_count(arr, temp, left, mid, right)
    return inv_count


def count_inversions(arr: list[int]) -> int:
    """Count inversions in array."""
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
    return merge_sort_count(work, temp, 0, length - 1)


def is_sorted_asc(arr: list[int]) -> int:
    """Returns 1 if array is sorted ascending (0 inversions)."""
    if count_inversions(arr) == 0:
        return 1
    return 0


def test_module() -> int:
    """Test inversion counting."""
    ok: int = 0
    a1: list[int] = [1, 2, 3, 4, 5]
    if count_inversions(a1) == 0:
        ok = ok + 1
    a2: list[int] = [5, 4, 3, 2, 1]
    if count_inversions(a2) == 10:
        ok = ok + 1
    a3: list[int] = [2, 4, 1, 3, 5]
    if count_inversions(a3) == 3:
        ok = ok + 1
    if is_sorted_asc(a1) == 1:
        ok = ok + 1
    a4: list[int] = [3, 1]
    if count_inversions(a4) == 1:
        ok = ok + 1
    empty: list[int] = []
    if count_inversions(empty) == 0:
        ok = ok + 1
    if is_sorted_asc(empty) == 1:
        ok = ok + 1
    return ok
