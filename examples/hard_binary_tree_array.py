"""Binary tree using array representation.

Tests: parent/child indexing, height calculation, level-order sum.
"""


def parent_index(i: int) -> int:
    """Get parent index of node at index i (0-based)."""
    if i <= 0:
        return -1
    return (i - 1) // 2


def left_child(i: int) -> int:
    """Get left child index of node at index i."""
    return 2 * i + 1


def right_child(i: int) -> int:
    """Get right child index of node at index i."""
    return 2 * i + 2


def tree_height(n: int) -> int:
    """Calculate height of a complete binary tree with n nodes."""
    if n <= 0:
        return 0
    height: int = 0
    nodes: int = n
    while nodes > 1:
        nodes = nodes // 2
        height = height + 1
    return height


def level_sum(tree: list[int], level: int) -> int:
    """Sum all nodes at a given level (0-indexed)."""
    start: int = 1
    i: int = 0
    while i < level:
        start = start * 2
        i = i + 1
    first: int = start - 1
    count: int = start
    total: int = 0
    j: int = 0
    while j < count and (first + j) < len(tree):
        total = total + tree[first + j]
        j = j + 1
    return total


def count_leaves(n: int) -> int:
    """Count leaf nodes in a complete binary tree with n nodes."""
    if n <= 0:
        return 0
    internal: int = n // 2
    return n - internal


def test_module() -> None:
    assert parent_index(0) == -1
    assert parent_index(1) == 0
    assert parent_index(2) == 0
    assert parent_index(5) == 2
    assert left_child(0) == 1
    assert right_child(0) == 2
    assert left_child(1) == 3
    assert tree_height(1) == 0
    assert tree_height(3) == 1
    assert tree_height(7) == 2
    assert tree_height(15) == 3
    tree: list[int] = [1, 2, 3, 4, 5, 6, 7]
    assert level_sum(tree, 0) == 1
    assert level_sum(tree, 1) == 5
    assert level_sum(tree, 2) == 22
    assert count_leaves(7) == 4
    assert count_leaves(1) == 1
    assert count_leaves(0) == 0
