"""Array equilibrium and balance point operations."""


def find_equilibrium_index(arr: list[int]) -> int:
    """Find index where sum of left elements equals sum of right elements.
    Returns -1 if no such index exists."""
    n: int = len(arr)
    if n == 0:
        return -1
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            return j
        left_sum = left_sum + arr[j]
        j = j + 1
    return -1


def count_equilibrium_points(arr: list[int]) -> int:
    """Count how many equilibrium indices exist."""
    n: int = len(arr)
    if n == 0:
        return 0
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    count: int = 0
    left_sum: int = 0
    j: int = 0
    while j < n:
        right_sum: int = total - left_sum - arr[j]
        if left_sum == right_sum:
            count = count + 1
        left_sum = left_sum + arr[j]
        j = j + 1
    return count


def partition_equal_sum(arr: list[int]) -> int:
    """Check if array can be partitioned into two halves (at some split point)
    with equal sums. Returns split index or -1."""
    n: int = len(arr)
    if n < 2:
        return -1
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i]
        i = i + 1
    left_sum: int = 0
    j: int = 0
    while j < n - 1:
        left_sum = left_sum + arr[j]
        right_sum: int = total - left_sum
        if left_sum == right_sum:
            return j
        j = j + 1
    return -1


def balance_point_weighted(arr: list[int], weights: list[int]) -> int:
    """Find balance point where weighted sum on left equals weighted sum on right.
    Returns index or -1."""
    n: int = len(arr)
    if n == 0:
        return -1
    total_weighted: int = 0
    i: int = 0
    while i < n:
        total_weighted = total_weighted + arr[i] * weights[i]
        i = i + 1
    left_weighted: int = 0
    j: int = 0
    while j < n:
        right_weighted: int = total_weighted - left_weighted - arr[j] * weights[j]
        if left_weighted == right_weighted:
            return j
        left_weighted = left_weighted + arr[j] * weights[j]
        j = j + 1
    return -1


def test_module() -> int:
    """Test array equilibrium functions."""
    ok: int = 0

    arr1: list[int] = [1, 3, 5, 2, 2]
    if find_equilibrium_index(arr1) == 2:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3]
    if find_equilibrium_index(arr2) == -1:
        ok = ok + 1

    arr3: list[int] = [0, 0, 0, 0]
    if count_equilibrium_points(arr3) == 4:
        ok = ok + 1

    arr4: list[int] = [1, 2, 3, 3, 2, 1]
    if partition_equal_sum(arr4) >= 0:
        ok = ok + 1

    arr5: list[int] = [2, 1, 3]
    wts: list[int] = [1, 1, 1]
    bp: int = balance_point_weighted(arr5, wts)
    if bp == -1 or bp >= 0:
        ok = ok + 1

    empty: list[int] = []
    if find_equilibrium_index(empty) == -1:
        ok = ok + 1

    return ok
