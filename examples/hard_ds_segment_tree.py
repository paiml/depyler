"""Segment tree using flat list for range sum and range max queries.

Tests: build, query_sum, query_max, point_update.
"""


def seg_build_sum(arr: list[int]) -> list[int]:
    """Build segment tree for range sum queries. Size = 4*n."""
    n: int = len(arr)
    sz: int = 4 * n
    tree: list[int] = []
    i: int = 0
    while i < sz:
        tree.append(0)
        i = i + 1
    if n > 0:
        seg_build_sum_helper(tree, arr, 1, 0, n - 1)
    return tree


def seg_build_sum_helper(tree: list[int], arr: list[int], node: int, start: int, end: int) -> int:
    """Recursive helper to build sum segment tree."""
    if start == end:
        tree[node] = arr[start]
        return arr[start]
    mid: int = (start + end) // 2
    left_child: int = 2 * node
    right_child: int = 2 * node + 1
    left_sum: int = seg_build_sum_helper(tree, arr, left_child, start, mid)
    right_sum: int = seg_build_sum_helper(tree, arr, right_child, mid + 1, end)
    tree[node] = left_sum + right_sum
    return tree[node]


def seg_query_sum(tree: list[int], node: int, start: int, end: int, l: int, r: int) -> int:
    """Query range sum [l, r]."""
    if r < start:
        return 0
    if l > end:
        return 0
    if l <= start:
        if end <= r:
            return tree[node]
    mid: int = (start + end) // 2
    left_child: int = 2 * node
    right_child: int = 2 * node + 1
    left_val: int = seg_query_sum(tree, left_child, start, mid, l, r)
    right_val: int = seg_query_sum(tree, right_child, mid + 1, end, l, r)
    return left_val + right_val


def seg_update_sum(tree: list[int], node: int, start: int, end: int, idx: int, val: int) -> int:
    """Point update: set arr[idx] = val."""
    if start == end:
        tree[node] = val
        return val
    mid: int = (start + end) // 2
    left_child: int = 2 * node
    right_child: int = 2 * node + 1
    if idx <= mid:
        seg_update_sum(tree, left_child, start, mid, idx, val)
    else:
        seg_update_sum(tree, right_child, mid + 1, end, idx, val)
    tree[node] = tree[left_child] + tree[right_child]
    return tree[node]


def seg_build_max(arr: list[int]) -> list[int]:
    """Build segment tree for range max queries."""
    n: int = len(arr)
    sz: int = 4 * n
    tree: list[int] = []
    i: int = 0
    while i < sz:
        tree.append(0)
        i = i + 1
    if n > 0:
        seg_build_max_helper(tree, arr, 1, 0, n - 1)
    return tree


def seg_build_max_helper(tree: list[int], arr: list[int], node: int, start: int, end: int) -> int:
    """Recursive helper to build max segment tree."""
    if start == end:
        tree[node] = arr[start]
        return arr[start]
    mid: int = (start + end) // 2
    left_child: int = 2 * node
    right_child: int = 2 * node + 1
    left_max: int = seg_build_max_helper(tree, arr, left_child, start, mid)
    right_max: int = seg_build_max_helper(tree, arr, right_child, mid + 1, end)
    if left_max > right_max:
        tree[node] = left_max
    else:
        tree[node] = right_max
    return tree[node]


def seg_query_max(tree: list[int], node: int, start: int, end: int, l: int, r: int) -> int:
    """Query range max [l, r]."""
    if r < start:
        return -999999999
    if l > end:
        return -999999999
    if l <= start:
        if end <= r:
            return tree[node]
    mid: int = (start + end) // 2
    left_child: int = 2 * node
    right_child: int = 2 * node + 1
    left_val: int = seg_query_max(tree, left_child, start, mid, l, r)
    right_val: int = seg_query_max(tree, right_child, mid + 1, end, l, r)
    if left_val > right_val:
        return left_val
    return right_val


def test_module() -> int:
    """Test segment tree operations."""
    passed: int = 0

    arr: list[int] = [1, 3, 5, 7, 9, 11]
    n: int = len(arr)
    st: list[int] = seg_build_sum(arr)

    total: int = seg_query_sum(st, 1, 0, n - 1, 0, 5)
    if total == 36:
        passed = passed + 1

    partial: int = seg_query_sum(st, 1, 0, n - 1, 1, 3)
    if partial == 15:
        passed = passed + 1

    seg_update_sum(st, 1, 0, n - 1, 2, 10)
    new_partial: int = seg_query_sum(st, 1, 0, n - 1, 1, 3)
    if new_partial == 20:
        passed = passed + 1

    arr2: list[int] = [4, 2, 7, 1, 9, 3]
    n2: int = len(arr2)
    mt: list[int] = seg_build_max(arr2)

    mx: int = seg_query_max(mt, 1, 0, n2 - 1, 0, 5)
    if mx == 9:
        passed = passed + 1

    mx2: int = seg_query_max(mt, 1, 0, n2 - 1, 0, 2)
    if mx2 == 7:
        passed = passed + 1

    single: int = seg_query_sum(st, 1, 0, n - 1, 3, 3)
    if single == 7:
        passed = passed + 1

    return passed
