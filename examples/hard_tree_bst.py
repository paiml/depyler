"""BST operations using sorted array representation."""


def bst_insert(arr: list[int], val: int) -> list[int]:
    """Insert value maintaining sorted order (simulated BST)."""
    result: list[int] = []
    inserted: int = 0
    i: int = 0
    while i < len(arr):
        if inserted == 0 and val <= arr[i]:
            result.append(val)
            inserted = 1
        result.append(arr[i])
        i = i + 1
    if inserted == 0:
        result.append(val)
    return result


def bst_search(arr: list[int], val: int) -> int:
    """Binary search in sorted array. Returns index or -1."""
    left: int = 0
    right: int = len(arr) - 1
    while left <= right:
        mid: int = left + (right - left) // 2
        if arr[mid] == val:
            return mid
        if arr[mid] < val:
            left = mid + 1
        else:
            right = mid - 1
    return -1


def bst_min(arr: list[int]) -> int:
    """Find minimum in sorted array (BST). Returns -1 if empty."""
    if len(arr) == 0:
        return -1
    return arr[0]


def bst_max(arr: list[int]) -> int:
    """Find maximum in sorted array (BST). Returns -1 if empty."""
    if len(arr) == 0:
        return -1
    return arr[len(arr) - 1]


def bst_count_range(arr: list[int], low: int, high: int) -> int:
    """Count elements in range [low, high]."""
    count: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] >= low and arr[i] <= high:
            count = count + 1
        elif arr[i] > high:
            i = len(arr)
        i = i + 1
    return count


def bst_delete(arr: list[int], val: int) -> list[int]:
    """Delete first occurrence of val from sorted array."""
    result: list[int] = []
    deleted: int = 0
    i: int = 0
    while i < len(arr):
        if deleted == 0 and arr[i] == val:
            deleted = 1
        else:
            result.append(arr[i])
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    bst: list[int] = []
    bst = bst_insert(bst, 5)
    bst = bst_insert(bst, 3)
    bst = bst_insert(bst, 7)
    bst = bst_insert(bst, 1)
    if bst == [1, 3, 5, 7]:
        passed = passed + 1

    if bst_search(bst, 3) == 1:
        passed = passed + 1

    if bst_search(bst, 10) == -1:
        passed = passed + 1

    if bst_min(bst) == 1:
        passed = passed + 1

    if bst_max(bst) == 7:
        passed = passed + 1

    if bst_count_range(bst, 2, 6) == 2:
        passed = passed + 1

    bst2: list[int] = bst_delete(bst, 3)
    if bst2 == [1, 5, 7]:
        passed = passed + 1

    if bst_min([]) == -1:
        passed = passed + 1

    return passed
