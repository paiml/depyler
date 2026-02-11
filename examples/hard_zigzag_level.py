"""Zigzag level order traversal using array-based binary tree.

Tests: zigzag traversal, level sum, level width.
"""


def zigzag_flatten(tree: list[int], size: int) -> list[int]:
    """Zigzag level-order flatten of array-based binary tree."""
    result: list[int] = []
    level_start: int = 0
    level_size: int = 1
    left_to_right: int = 1
    while level_start < size:
        level_vals: list[int] = []
        i: int = 0
        while i < level_size:
            idx: int = level_start + i
            if idx < size:
                if tree[idx] != -1:
                    level_vals.append(tree[idx])
            i = i + 1
        if left_to_right == 0:
            j: int = len(level_vals) - 1
            while j >= 0:
                result.append(level_vals[j])
                j = j - 1
        else:
            k: int = 0
            while k < len(level_vals):
                result.append(level_vals[k])
                k = k + 1
        if left_to_right == 1:
            left_to_right = 0
        else:
            left_to_right = 1
        level_start = level_start + level_size
        level_size = level_size * 2
    return result


def level_sums(tree: list[int], size: int) -> list[int]:
    """Compute sum at each level of array-based binary tree."""
    sums: list[int] = []
    level_start: int = 0
    level_size: int = 1
    while level_start < size:
        total: int = 0
        i: int = 0
        while i < level_size:
            idx: int = level_start + i
            if idx < size:
                if tree[idx] != -1:
                    total = total + tree[idx]
            i = i + 1
        sums.append(total)
        level_start = level_start + level_size
        level_size = level_size * 2
    return sums


def max_level_width(tree: list[int], size: int) -> int:
    """Find maximum width among all levels."""
    max_w: int = 0
    level_start: int = 0
    level_size: int = 1
    while level_start < size:
        count: int = 0
        i: int = 0
        while i < level_size:
            idx: int = level_start + i
            if idx < size:
                if tree[idx] != -1:
                    count = count + 1
            i = i + 1
        if count > max_w:
            max_w = count
        level_start = level_start + level_size
        level_size = level_size * 2
    return max_w


def test_module() -> int:
    """Test zigzag level operations."""
    ok: int = 0
    tree: list[int] = [1, 2, 3, 4, 5, 6, 7]
    zz: list[int] = zigzag_flatten(tree, 7)
    if zz[0] == 1:
        ok = ok + 1
    if zz[1] == 3:
        ok = ok + 1
    sums: list[int] = level_sums(tree, 7)
    if sums[0] == 1:
        ok = ok + 1
    if sums[1] == 5:
        ok = ok + 1
    if max_level_width(tree, 7) == 4:
        ok = ok + 1
    return ok
