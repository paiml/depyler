"""Fenwick tree (Binary Indexed Tree) for prefix sums."""


def fenwick_build(arr: list[int]) -> list[int]:
    """Build Fenwick tree from array. Index 1-based, tree[0] unused."""
    n: int = len(arr)
    tree: list[int] = []
    i: int = 0
    while i <= n:
        tree.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        fenwick_update(tree, j + 1, arr[j], n)
        j = j + 1
    return tree


def fenwick_update(tree: list[int], idx: int, delta: int, n: int) -> int:
    """Add delta to index idx (1-based). Returns 0."""
    i: int = idx
    while i <= n:
        tree[i] = tree[i] + delta
        i = i + (i & (-i))
    return 0


def fenwick_prefix_sum(tree: list[int], idx: int) -> int:
    """Get prefix sum from 1 to idx (1-based)."""
    s: int = 0
    i: int = idx
    while i > 0:
        s = s + tree[i]
        i = i - (i & (-i))
    return s


def fenwick_range_sum(tree: list[int], l: int, r: int) -> int:
    """Get sum from index l to r (1-based, inclusive)."""
    if l <= 1:
        return fenwick_prefix_sum(tree, r)
    return fenwick_prefix_sum(tree, r) - fenwick_prefix_sum(tree, l - 1)


def fenwick_point_query(tree: list[int], idx: int) -> int:
    """Get single element value at idx (1-based)."""
    if idx <= 1:
        return fenwick_prefix_sum(tree, idx)
    return fenwick_prefix_sum(tree, idx) - fenwick_prefix_sum(tree, idx - 1)


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 2, 3, 4, 5]
    ft: list[int] = fenwick_build(arr1)

    # Prefix sum [1..5]
    ps1: int = fenwick_prefix_sum(ft, 5)
    if ps1 == 15:
        passed = passed + 1

    # Prefix sum [1..3]
    ps2: int = fenwick_prefix_sum(ft, 3)
    if ps2 == 6:
        passed = passed + 1

    # Range sum [2..4]
    rs1: int = fenwick_range_sum(ft, 2, 4)
    if rs1 == 9:
        passed = passed + 1

    # Point query at index 3
    pq1: int = fenwick_point_query(ft, 3)
    if pq1 == 3:
        passed = passed + 1

    # Update: add 10 to index 3
    fenwick_update(ft, 3, 10, 5)
    ps3: int = fenwick_prefix_sum(ft, 5)
    if ps3 == 25:
        passed = passed + 1

    # Single element
    arr2: list[int] = [42]
    ft2: list[int] = fenwick_build(arr2)
    if fenwick_prefix_sum(ft2, 1) == 42:
        passed = passed + 1

    # Range sum [1..1]
    if fenwick_range_sum(ft2, 1, 1) == 42:
        passed = passed + 1

    return passed
