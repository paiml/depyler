"""Array splitting operations.

Implements algorithms for splitting arrays into parts
based on various criteria.
"""


def split_at_index(arr: list[int], size: int, idx: int) -> list[int]:
    """Split array at index, return sizes of left and right parts packed.

    Returns [left_size, right_size, ...left_elements..., ...right_elements...].
    """
    left_size: int = idx
    right_size: int = size - idx
    result: list[int] = [left_size, right_size]
    i: int = 0
    while i < size:
        result.append(arr[i])
        i = i + 1
    return result


def split_into_chunks(arr: list[int], size: int, chunk_size: int) -> list[int]:
    """Split array into chunks of given size. Returns chunk count followed by all elements."""
    if chunk_size <= 0:
        return [0]
    num_chunks: int = (size + chunk_size - 1) // chunk_size
    result: list[int] = [num_chunks]
    i: int = 0
    while i < size:
        result.append(arr[i])
        i = i + 1
    return result


def split_by_predicate(arr: list[int], size: int, threshold: int) -> list[int]:
    """Split into elements below and at/above threshold.

    Returns [below_count, above_count, ...below..., ...above...].
    """
    below: list[int] = []
    above: list[int] = []
    i: int = 0
    while i < size:
        if arr[i] < threshold:
            below.append(arr[i])
        else:
            above.append(arr[i])
        i = i + 1
    result: list[int] = [len(below), len(above)]
    j: int = 0
    while j < len(below):
        result.append(below[j])
        j = j + 1
    k: int = 0
    while k < len(above):
        result.append(above[k])
        k = k + 1
    return result


def equal_sum_split(arr: list[int], size: int) -> int:
    """Find index where left sum equals right sum. Returns -1 if impossible."""
    total: int = 0
    i: int = 0
    while i < size:
        total = total + arr[i]
        i = i + 1
    left: int = 0
    j: int = 0
    while j < size:
        right: int = total - left - arr[j]
        if left == right:
            return j
        left = left + arr[j]
        j = j + 1
    return -1


def test_module() -> int:
    """Test array splitting operations."""
    ok: int = 0

    arr1: list[int] = [1, 2, 3, 4, 5]
    tmp_split: list[int] = split_at_index(arr1, 5, 2)
    if tmp_split[0] == 2 and tmp_split[1] == 3:
        ok = ok + 1

    tmp_chunks: list[int] = split_into_chunks(arr1, 5, 2)
    if tmp_chunks[0] == 3:
        ok = ok + 1

    arr2: list[int] = [1, 5, 2, 8, 3]
    tmp_pred: list[int] = split_by_predicate(arr2, 5, 4)
    if tmp_pred[0] == 3 and tmp_pred[1] == 2:
        ok = ok + 1

    arr3: list[int] = [1, 3, 5, 2, 2]
    eq_idx: int = equal_sum_split(arr3, 5)
    if eq_idx == 2:
        ok = ok + 1

    return ok
