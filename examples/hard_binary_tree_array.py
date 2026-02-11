"""Binary tree stored in array: insert (BST), inorder traversal, height.

Tests: bst_insert, inorder_traversal, tree_height, tree_search.
"""


def bst_create(capacity: int) -> list[int]:
    """Create empty BST array. -1 means empty slot."""
    arr: list[int] = []
    i: int = 0
    while i < capacity:
        arr.append(-1)
        i = i + 1
    return arr


def bst_insert(tree: list[int], value: int) -> list[int]:
    """Insert value into BST stored as array. Left=2*i+1, Right=2*i+2."""
    result: list[int] = tree[:]
    idx: int = 0
    while idx < len(result):
        if result[idx] == -1:
            result[idx] = value
            return result
        if value < result[idx]:
            idx = 2 * idx + 1
        else:
            idx = 2 * idx + 2
    return result


def inorder_collect(tree: list[int], idx: int, result: list[int]) -> list[int]:
    """Collect inorder traversal into result list (iterative with stack)."""
    out: list[int] = result[:]
    stack: list[int] = []
    current: int = idx
    cont: int = 1
    while cont == 1:
        while current < len(tree) and tree[current] != -1:
            stack.append(current)
            current = 2 * current + 1
        if len(stack) == 0:
            cont = 0
        else:
            current = stack[len(stack) - 1]
            stack = stack[:len(stack) - 1]
            out.append(tree[current])
            current = 2 * current + 2
    return out


def inorder_traversal(tree: list[int]) -> list[int]:
    """Get inorder traversal of BST."""
    return inorder_collect(tree, 0, [])


def tree_height(tree: list[int], idx: int) -> int:
    """Compute height of tree rooted at idx (iterative level-based)."""
    if idx >= len(tree) or tree[idx] == -1:
        return 0
    queue: list[int] = [idx]
    height: int = 0
    while len(queue) > 0:
        level_size: int = len(queue)
        height = height + 1
        i: int = 0
        while i < level_size:
            node: int = queue[0]
            queue = queue[1:]
            left: int = 2 * node + 1
            right: int = 2 * node + 2
            if left < len(tree) and tree[left] != -1:
                queue.append(left)
            if right < len(tree) and tree[right] != -1:
                queue.append(right)
            i = i + 1
    return height


def tree_search(tree: list[int], value: int) -> int:
    """Search for value in BST. Returns 1 if found, 0 otherwise."""
    idx: int = 0
    while idx < len(tree):
        if tree[idx] == -1:
            return 0
        if tree[idx] == value:
            return 1
        if value < tree[idx]:
            idx = 2 * idx + 1
        else:
            idx = 2 * idx + 2
    return 0


def tree_min(tree: list[int]) -> int:
    """Find minimum value in BST. Returns -1 if empty."""
    idx: int = 0
    while idx < len(tree) and tree[idx] != -1:
        left: int = 2 * idx + 1
        if left < len(tree) and tree[left] != -1:
            idx = left
        else:
            return tree[idx]
    return -1


def test_module() -> int:
    """Test binary tree array operations."""
    ok: int = 0

    t: list[int] = bst_create(31)
    t = bst_insert(t, 5)
    t = bst_insert(t, 3)
    t = bst_insert(t, 7)
    t = bst_insert(t, 1)
    t = bst_insert(t, 4)

    order: list[int] = inorder_traversal(t)
    if order == [1, 3, 4, 5, 7]:
        ok = ok + 1

    if tree_search(t, 3) == 1:
        ok = ok + 1

    if tree_search(t, 6) == 0:
        ok = ok + 1

    h: int = tree_height(t, 0)
    if h == 3:
        ok = ok + 1

    if tree_min(t) == 1:
        ok = ok + 1

    empty: list[int] = bst_create(7)
    if tree_height(empty, 0) == 0:
        ok = ok + 1

    if tree_min(empty) == -1:
        ok = ok + 1

    return ok
