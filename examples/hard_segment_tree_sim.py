"""Segment tree operations using a flat list representation."""


def build_segment_tree(arr: list[int]) -> list[int]:
    """Build a segment tree for range sum queries. Returns flat tree."""
    n: int = len(arr)
    tree: list[int] = []
    i: int = 0
    while i < 4 * n:
        tree.append(0)
        i = i + 1
    if n > 0:
        build_helper(arr, tree, 1, 0, n - 1)
    return tree


def build_helper(arr: list[int], tree: list[int], node: int, start: int, end: int) -> int:
    """Recursively build segment tree. Returns sum at node."""
    if start == end:
        tree[node] = arr[start]
        return tree[node]
    mid: int = (start + end) // 2
    left_sum: int = build_helper(arr, tree, 2 * node, start, mid)
    right_sum: int = build_helper(arr, tree, 2 * node + 1, mid + 1, end)
    tree[node] = left_sum + right_sum
    return tree[node]


def query_sum(tree: list[int], node: int, start: int, end: int, l: int, r: int) -> int:
    """Query range sum [l, r]."""
    if r < start or end < l:
        return 0
    if l <= start and end <= r:
        return tree[node]
    mid: int = (start + end) // 2
    left_val: int = query_sum(tree, 2 * node, start, mid, l, r)
    right_val: int = query_sum(tree, 2 * node + 1, mid + 1, end, l, r)
    return left_val + right_val


def update_tree(tree: list[int], node: int, start: int, end: int, idx: int, val: int) -> int:
    """Update element at idx to val. Returns new sum at node."""
    if start == end:
        tree[node] = val
        return val
    mid: int = (start + end) // 2
    if idx <= mid:
        update_tree(tree, 2 * node, start, mid, idx, val)
    else:
        update_tree(tree, 2 * node + 1, mid + 1, end, idx, val)
    tree[node] = tree[2 * node] + tree[2 * node + 1]
    return tree[node]


def range_sum(arr: list[int], l: int, r: int) -> int:
    """Simple range sum for verification."""
    total: int = 0
    i: int = l
    while i <= r:
        total = total + arr[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test segment tree."""
    passed: int = 0

    arr: list[int] = [1, 3, 5, 7, 9, 11]
    n: int = len(arr)
    tree: list[int] = build_segment_tree(arr)

    total: int = query_sum(tree, 1, 0, n - 1, 0, n - 1)
    if total == 36:
        passed = passed + 1

    partial: int = query_sum(tree, 1, 0, n - 1, 1, 3)
    if partial == 15:
        passed = passed + 1

    single: int = query_sum(tree, 1, 0, n - 1, 2, 2)
    if single == 5:
        passed = passed + 1

    update_tree(tree, 1, 0, n - 1, 2, 10)
    new_total: int = query_sum(tree, 1, 0, n - 1, 0, n - 1)
    if new_total == 41:
        passed = passed + 1

    if range_sum(arr, 0, 2) == 9:
        passed = passed + 1

    if query_sum(tree, 1, 0, n - 1, 0, 0) == 1:
        passed = passed + 1

    return passed
