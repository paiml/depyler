"""Array rotation variant operations.

Implements multiple array rotation strategies including
block rotation, juggling rotation, and rotation search.
"""


def rotate_left(arr: list[int], size: int, positions: int) -> list[int]:
    """Rotate array left by given positions using copy."""
    if size == 0:
        return []
    effective: int = positions % size
    result: list[int] = []
    i: int = effective
    while i < size:
        result.append(arr[i])
        i = i + 1
    j: int = 0
    while j < effective:
        result.append(arr[j])
        j = j + 1
    return result


def rotate_right(arr: list[int], size: int, positions: int) -> list[int]:
    """Rotate array right by given positions."""
    if size == 0:
        return []
    effective: int = positions % size
    left_amount: int = size - effective
    tmp_result: list[int] = rotate_left(arr, size, left_amount)
    return tmp_result


def is_rotation(arr1: list[int], arr2: list[int], size: int) -> int:
    """Check if arr2 is a rotation of arr1. Returns 1 if yes."""
    if size == 0:
        return 1
    start: int = -1
    i: int = 0
    while i < size:
        if arr2[i] == arr1[0]:
            start = i
            i = size
        i = i + 1
    if start == -1:
        return 0
    j: int = 0
    while j < size:
        idx: int = (start + j) % size
        if arr2[idx] != arr1[j]:
            return 0
        j = j + 1
    return 1


def find_rotation_count(arr: list[int], size: int) -> int:
    """Find how many positions a sorted array was rotated left.

    Returns index of the minimum element.
    """
    if size == 0:
        return 0
    min_idx: int = 0
    min_val: int = arr[0]
    i: int = 1
    while i < size:
        if arr[i] < min_val:
            min_val = arr[i]
            min_idx = i
        i = i + 1
    return min_idx


def test_module() -> int:
    """Test array rotation operations."""
    ok: int = 0

    arr: list[int] = [1, 2, 3, 4, 5]
    tmp_left: list[int] = rotate_left(arr, 5, 2)
    if tmp_left[0] == 3 and tmp_left[1] == 4 and tmp_left[2] == 5:
        ok = ok + 1

    tmp_right: list[int] = rotate_right(arr, 5, 2)
    if tmp_right[0] == 4 and tmp_right[1] == 5 and tmp_right[2] == 1:
        ok = ok + 1

    arr2: list[int] = [3, 4, 5, 1, 2]
    is_rot: int = is_rotation(arr, arr2, 5)
    if is_rot == 1:
        ok = ok + 1

    rot_count: int = find_rotation_count(arr2, 5)
    if rot_count == 3:
        ok = ok + 1

    return ok
