"""Binary tree stored in array: level order traversal, node counting."""


def left_child(i: int) -> int:
    """Get index of left child."""
    return 2 * i + 1


def right_child(i: int) -> int:
    """Get index of right child."""
    return 2 * i + 2


def parent_index(i: int) -> int:
    """Get index of parent node."""
    if i == 0:
        return -1
    return (i - 1) // 2


def count_nodes(tree: list[int], sentinel: int) -> int:
    """Count non-sentinel nodes in tree array."""
    count: int = 0
    i: int = 0
    while i < len(tree):
        if tree[i] != sentinel:
            count = count + 1
        i = i + 1
    return count


def tree_height(tree: list[int], sentinel: int) -> int:
    """Compute height of binary tree stored in array."""
    if len(tree) == 0:
        return 0
    # Find last non-sentinel
    last: int = len(tree) - 1
    while last >= 0 and tree[last] == sentinel:
        last = last - 1
    if last < 0:
        return 0
    height: int = 0
    idx: int = last
    while idx > 0:
        idx = parent_index(idx)
        height = height + 1
    return height + 1


def level_order(tree: list[int], sentinel: int) -> list[int]:
    """Return level-order traversal of non-sentinel nodes."""
    result: list[int] = []
    i: int = 0
    while i < len(tree):
        if tree[i] != sentinel:
            result.append(tree[i])
        i = i + 1
    return result


def sum_at_level(tree: list[int], level: int, sentinel: int) -> int:
    """Sum of all nodes at a given level (0-indexed)."""
    start: int = 0
    lv: int = 0
    nodes_at_level: int = 1
    while lv < level:
        start = start + nodes_at_level
        nodes_at_level = nodes_at_level * 2
        lv = lv + 1
    total: int = 0
    i: int = start
    while i < start + nodes_at_level and i < len(tree):
        if tree[i] != sentinel:
            total = total + tree[i]
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Tree:     1
    #         /   \
    #        2     3
    #       / \
    #      4   5
    tree1: list[int] = [1, 2, 3, 4, 5, -1, -1]

    if left_child(0) == 1:
        passed = passed + 1

    if right_child(0) == 2:
        passed = passed + 1

    if count_nodes(tree1, -1) == 5:
        passed = passed + 1

    h1: int = tree_height(tree1, -1)
    if h1 == 3:
        passed = passed + 1

    lo: list[int] = level_order(tree1, -1)
    if lo == [1, 2, 3, 4, 5]:
        passed = passed + 1

    if sum_at_level(tree1, 0, -1) == 1:
        passed = passed + 1

    if sum_at_level(tree1, 1, -1) == 5:
        passed = passed + 1

    if sum_at_level(tree1, 2, -1) == 9:
        passed = passed + 1

    return passed
