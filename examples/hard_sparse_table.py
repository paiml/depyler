"""Sparse table for range minimum queries."""


def log2_floor(n: int) -> int:
    """Compute floor(log2(n))."""
    if n <= 0:
        return 0
    result: int = 0
    val: int = n
    while val > 1:
        val = val // 2
        result = result + 1
    return result


def build_sparse_table(arr: list[int]) -> list[int]:
    """Build sparse table as flat array. Table[k][i] = min of arr[i..i+2^k-1].
    Layout: table[k * n + i] for k levels, n elements."""
    n: int = len(arr)
    if n == 0:
        return []
    max_k: int = log2_floor(n) + 1
    table: list[int] = []
    total: int = max_k * n
    idx: int = 0
    while idx < total:
        table.append(0)
        idx = idx + 1
    i: int = 0
    while i < n:
        table[i] = arr[i]
        i = i + 1
    k: int = 1
    while k < max_k:
        i = 0
        half: int = 1
        p: int = 0
        while p < k - 1:
            half = half * 2
            p = p + 1
        while i + half * 2 - 1 < n:
            left_val: int = table[(k - 1) * n + i]
            right_val: int = table[(k - 1) * n + i + half]
            if left_val < right_val:
                table[k * n + i] = left_val
            else:
                table[k * n + i] = right_val
            i = i + 1
        k = k + 1
    return table


def range_min_naive(arr: list[int], left: int, right: int) -> int:
    """Naive range minimum query for verification."""
    if left > right:
        return 0
    best: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] < best:
            best = arr[i]
        i = i + 1
    return best


def range_max_naive(arr: list[int], left: int, right: int) -> int:
    """Naive range maximum query."""
    if left > right:
        return 0
    best: int = arr[left]
    i: int = left + 1
    while i <= right:
        if arr[i] > best:
            best = arr[i]
        i = i + 1
    return best


def array_min(arr: list[int]) -> int:
    """Find minimum of entire array."""
    if len(arr) == 0:
        return 0
    return range_min_naive(arr, 0, len(arr) - 1)


def test_module() -> int:
    """Test sparse table operations."""
    ok: int = 0
    arr: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
    if range_min_naive(arr, 0, 3) == 1:
        ok = ok + 1
    if range_min_naive(arr, 4, 7) == 2:
        ok = ok + 1
    if range_min_naive(arr, 0, 7) == 1:
        ok = ok + 1
    if range_max_naive(arr, 0, 7) == 9:
        ok = ok + 1
    if array_min(arr) == 1:
        ok = ok + 1
    if log2_floor(8) == 3:
        ok = ok + 1
    if log2_floor(1) == 0:
        ok = ok + 1
    return ok
