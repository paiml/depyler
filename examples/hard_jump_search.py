"""Jump/block search algorithm for sorted arrays."""


def isqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def jump_search(arr: list[int], target: int) -> int:
    """Jump search: skip by sqrt(n) blocks, then linear scan. Returns index or -1."""
    n: int = len(arr)
    if n == 0:
        return -1
    step: int = isqrt(n)
    if step == 0:
        step = 1
    prev: int = 0
    curr: int = step
    while curr < n and arr[curr] <= target:
        prev = curr
        curr = curr + step
    idx: int = prev
    limit: int = curr
    if limit > n:
        limit = n
    while idx < limit:
        if arr[idx] == target:
            return idx
        idx = idx + 1
    return -1


def jump_search_first(arr: list[int], target: int) -> int:
    """Find first occurrence of target using jump search."""
    idx: int = jump_search(arr, target)
    if idx == -1:
        return -1
    while idx > 0 and arr[idx - 1] == target:
        idx = idx - 1
    return idx


def jump_search_last(arr: list[int], target: int) -> int:
    """Find last occurrence of target using jump search."""
    idx: int = jump_search(arr, target)
    if idx == -1:
        return -1
    last_idx: int = len(arr) - 1
    while idx < last_idx and arr[idx + 1] == target:
        idx = idx + 1
    return idx


def block_count(arr: list[int], target: int) -> int:
    """Count occurrences using first and last jump search."""
    first: int = jump_search_first(arr, target)
    if first == -1:
        return 0
    last: int = jump_search_last(arr, target)
    return last - first + 1


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
    if jump_search(arr1, 11) == 5:
        passed = passed + 1

    if jump_search(arr1, 1) == 0:
        passed = passed + 1

    if jump_search(arr1, 4) == -1:
        passed = passed + 1

    arr2: list[int] = [2, 4, 4, 4, 6, 8, 10]
    if jump_search_first(arr2, 4) == 1:
        passed = passed + 1

    if jump_search_last(arr2, 4) == 3:
        passed = passed + 1

    if block_count(arr2, 4) == 3:
        passed = passed + 1

    if isqrt(16) == 4:
        passed = passed + 1

    if isqrt(0) == 0:
        passed = passed + 1

    return passed
