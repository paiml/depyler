"""Segment tree for range sum queries."""


def build_segment_tree(arr: list[int]) -> list[int]:
    """Build segment tree from array. Tree has size 4*n."""
    n: int = len(arr)
    if n == 0:
        return []
    size: int = 4 * n
    tree: list[int] = []
    i: int = 0
    while i < size:
        tree.append(0)
        i = i + 1
    build_helper(arr, tree, 0, 0, n - 1)
    return tree


def build_helper(arr: list[int], tree: list[int], node: int, start: int, end: int) -> int:
    """Recursive helper to build segment tree. Returns node sum."""
    if start == end:
        tree[node] = arr[start]
        return tree[node]
    mid: int = start + (end - start) // 2
    left_sum: int = build_helper(arr, tree, 2 * node + 1, start, mid)
    right_sum: int = build_helper(arr, tree, 2 * node + 2, mid + 1, end)
    tree[node] = left_sum + right_sum
    return tree[node]


def query_sum(tree: list[int], node: int, start: int, end: int, l: int, r: int) -> int:
    """Query range sum [l, r]."""
    if r < start or end < l:
        return 0
    if l <= start and end <= r:
        return tree[node]
    mid: int = start + (end - start) // 2
    left_sum: int = query_sum(tree, 2 * node + 1, start, mid, l, r)
    right_sum: int = query_sum(tree, 2 * node + 2, mid + 1, end, l, r)
    return left_sum + right_sum


def update_tree(tree: list[int], node: int, start: int, end: int, idx: int, val: int) -> int:
    """Point update: set arr[idx] = val. Returns new node sum."""
    if start == end:
        tree[node] = val
        return val
    mid: int = start + (end - start) // 2
    if idx <= mid:
        update_tree(tree, 2 * node + 1, start, mid, idx, val)
    else:
        update_tree(tree, 2 * node + 2, mid + 1, end, idx, val)
    tree[node] = tree[2 * node + 1] + tree[2 * node + 2]
    return tree[node]


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [1, 3, 5, 7, 9, 11]
    tree1: list[int] = build_segment_tree(arr1)
    n: int = len(arr1)

    # Sum of [0, 5]
    s1: int = query_sum(tree1, 0, 0, n - 1, 0, n - 1)
    if s1 == 36:
        passed = passed + 1

    # Sum of [1, 3]
    s2: int = query_sum(tree1, 0, 0, n - 1, 1, 3)
    if s2 == 15:
        passed = passed + 1

    # Sum of single element [2, 2]
    s3: int = query_sum(tree1, 0, 0, n - 1, 2, 2)
    if s3 == 5:
        passed = passed + 1

    # Update index 2 to 10
    update_tree(tree1, 0, 0, n - 1, 2, 10)
    s4: int = query_sum(tree1, 0, 0, n - 1, 0, n - 1)
    if s4 == 41:
        passed = passed + 1

    # Sum of [0, 0]
    s5: int = query_sum(tree1, 0, 0, n - 1, 0, 0)
    if s5 == 1:
        passed = passed + 1

    # Empty
    tree2: list[int] = build_segment_tree([])
    if tree2 == []:
        passed = passed + 1

    return passed
