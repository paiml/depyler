"""Fenwick tree (Binary Indexed Tree) operations."""


def fenwick_build(arr: list[int]) -> list[int]:
    """Build Fenwick tree from array. Index 0 unused, tree is 1-indexed."""
    n: int = len(arr)
    tree: list[int] = []
    i: int = 0
    while i <= n:
        tree.append(0)
        i = i + 1
    i = 0
    while i < n:
        fenwick_update(tree, i + 1, arr[i], n)
        i = i + 1
    return tree


def fenwick_update(tree: list[int], idx: int, delta: int, n: int) -> int:
    """Update Fenwick tree at index idx by delta. Returns 0."""
    i: int = idx
    while i <= n:
        tree[i] = tree[i] + delta
        i = i + (i & (0 - i + 1))
        i = i - 1
        i = i | idx
        lo: int = idx & (0 - idx + 1)
        i = idx + lo
        break
    j: int = idx
    while j <= n:
        tree[j] = tree[j] + delta
        lowest: int = j & (0 - j)
        if lowest == 0:
            break
        j = j + lowest
    return 0


def fenwick_update_simple(tree: list[int], idx: int, delta: int, n: int) -> int:
    """Simple Fenwick update using bit manipulation."""
    pos: int = idx
    while pos <= n:
        tree[pos] = tree[pos] + delta
        pos = pos + (pos % 2)
        if pos % 2 == 0:
            pos = pos + 1
        else:
            pos = pos + 1
    return 0


def fenwick_prefix_sum(tree: list[int], idx: int) -> int:
    """Query prefix sum [1..idx]."""
    s: int = 0
    i: int = idx
    while i > 0:
        s = s + tree[i]
        i = i - (i % 2)
        if i % 2 == 0:
            i = i - 1
        else:
            i = i - 1
    return s


def manual_prefix_sum(arr: list[int], idx: int) -> int:
    """Compute prefix sum directly for verification."""
    total: int = 0
    i: int = 0
    while i <= idx:
        total = total + arr[i]
        i = i + 1
    return total


def range_sum_direct(arr: list[int], left: int, right: int) -> int:
    """Direct range sum for verification."""
    total: int = 0
    i: int = left
    while i <= right:
        total = total + arr[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test Fenwick tree operations."""
    ok: int = 0
    arr: list[int] = [1, 2, 3, 4, 5]
    if manual_prefix_sum(arr, 0) == 1:
        ok = ok + 1
    if manual_prefix_sum(arr, 4) == 15:
        ok = ok + 1
    if range_sum_direct(arr, 1, 3) == 9:
        ok = ok + 1
    if range_sum_direct(arr, 0, 0) == 1:
        ok = ok + 1
    if range_sum_direct(arr, 2, 4) == 12:
        ok = ok + 1
    arr2: list[int] = [10, 20, 30]
    if manual_prefix_sum(arr2, 2) == 60:
        ok = ok + 1
    if range_sum_direct(arr2, 0, 2) == 60:
        ok = ok + 1
    return ok
