"""Array rotation operations.

Tests: rotate right, rotate left, rotation index, check sorted rotated.
"""


def rotate_right(arr: list[int], k: int) -> list[int]:
    """Rotate array right by k positions."""
    n: int = len(arr)
    if n == 0:
        return arr
    kk: int = k % n
    if kk == 0:
        return arr
    result: list[int] = []
    i: int = n - kk
    while i < n:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < n - kk:
        result.append(arr[i])
        i = i + 1
    return result


def rotate_left(arr: list[int], k: int) -> list[int]:
    """Rotate array left by k positions."""
    n: int = len(arr)
    if n == 0:
        return arr
    kk: int = k % n
    return rotate_right(arr, n - kk)


def find_rotation_count(arr: list[int]) -> int:
    """Find how many times a sorted array was rotated right.
    Returns index of minimum element."""
    n: int = len(arr)
    if n == 0:
        return 0
    min_idx: int = 0
    i: int = 1
    while i < n:
        if arr[i] < arr[min_idx]:
            min_idx = i
        i = i + 1
    return min_idx


def is_sorted_rotated(arr: list[int]) -> int:
    """Check if array is a rotation of a sorted array. Returns 1 if yes."""
    n: int = len(arr)
    if n <= 1:
        return 1
    breaks: int = 0
    i: int = 0
    while i < n - 1:
        if arr[i] > arr[i + 1]:
            breaks = breaks + 1
        i = i + 1
    if arr[n - 1] > arr[0]:
        breaks = breaks + 1
    if breaks <= 1:
        return 1
    return 0


def test_module() -> int:
    """Test array rotation."""
    ok: int = 0
    r: list[int] = rotate_right([1, 2, 3, 4, 5], 2)
    if r[0] == 4 and r[1] == 5 and r[2] == 1:
        ok = ok + 1
    l: list[int] = rotate_left([1, 2, 3, 4, 5], 2)
    if l[0] == 3 and l[1] == 4 and l[2] == 5:
        ok = ok + 1
    if find_rotation_count([4, 5, 1, 2, 3]) == 2:
        ok = ok + 1
    if find_rotation_count([1, 2, 3, 4]) == 0:
        ok = ok + 1
    if is_sorted_rotated([3, 4, 5, 1, 2]) == 1:
        ok = ok + 1
    if is_sorted_rotated([3, 1, 4, 2]) == 0:
        ok = ok + 1
    return ok
