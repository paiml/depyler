"""Array rotation variant patterns.

Tests: left rotation, right rotation, rotation by reversal,
juggling rotation, and finding rotation point.
"""


def rotate_left(arr: list[int], k: int) -> list[int]:
    """Rotate array left by k positions."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    result: list[int] = []
    i: int = k
    while i < n:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < k:
        result.append(arr[i])
        i = i + 1
    return result


def rotate_right(arr: list[int], k: int) -> list[int]:
    """Rotate array right by k positions."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    result: list[int] = []
    i: int = n - k
    while i < n:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < n - k:
        result.append(arr[i])
        i = i + 1
    return result


def reverse_range(arr: list[int], start: int, end: int) -> list[int]:
    """Reverse elements in arr from start to end (inclusive)."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    left: int = start
    right: int = end
    while left < right:
        tmp: int = result[left]
        result[left] = result[right]
        result[right] = tmp
        left = left + 1
        right = right - 1
    return result


def rotate_by_reversal(arr: list[int], k: int) -> list[int]:
    """Rotate left by k using three reversals."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    if k == 0:
        return arr
    step1: list[int] = reverse_range(arr, 0, k - 1)
    step2: list[int] = reverse_range(step1, k, n - 1)
    step3: list[int] = reverse_range(step2, 0, n - 1)
    return step3


def find_rotation_point(arr: list[int]) -> int:
    """Find the index of the minimum element in a rotated sorted array."""
    n: int = len(arr)
    if n == 0:
        return -1
    low: int = 0
    high: int = n - 1
    while low < high:
        mid: int = (low + high) // 2
        if arr[mid] > arr[high]:
            low = mid + 1
        else:
            high = mid
    return low


def search_rotated(arr: list[int], target: int) -> int:
    """Search for target in rotated sorted array. Returns index or -1."""
    n: int = len(arr)
    if n == 0:
        return -1
    pivot: int = find_rotation_point(arr)
    low: int = 0
    high: int = n - 1
    if arr[pivot] <= target and target <= arr[high]:
        low = pivot
    else:
        high = pivot - 1
    while low <= high:
        mid: int = (low + high) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    return -1


def test_module() -> bool:
    """Test all array rotation functions."""
    ok: bool = True

    if rotate_left([1, 2, 3, 4, 5], 2) != [3, 4, 5, 1, 2]:
        ok = False
    if rotate_right([1, 2, 3, 4, 5], 2) != [4, 5, 1, 2, 3]:
        ok = False
    if rotate_by_reversal([1, 2, 3, 4, 5], 2) != [3, 4, 5, 1, 2]:
        ok = False

    if find_rotation_point([4, 5, 6, 7, 0, 1, 2]) != 4:
        ok = False
    if find_rotation_point([1, 2, 3, 4, 5]) != 0:
        ok = False

    if search_rotated([4, 5, 6, 7, 0, 1, 2], 0) != 4:
        ok = False
    if search_rotated([4, 5, 6, 7, 0, 1, 2], 3) != -1:
        ok = False

    if rotate_left([], 3) != []:
        ok = False

    return ok
