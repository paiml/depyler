"""Directory tree simulation.

Flat representation of a directory tree using parent pointers.
Supports mkdir, depth calculation, and subtree counting.
"""


def dt_init(capacity: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0 - 1)
        i = i + 1
    return result


def dt_init_zeros(capacity: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < capacity:
        result.append(0)
        i = i + 1
    return result


def dt_mkdir(names: list[int], parents: list[int],
             count_arr: list[int], name_id: int, parent_idx: int) -> int:
    """Create directory entry. Returns index of new dir."""
    idx: int = count_arr[0]
    names[idx] = name_id
    parents[idx] = parent_idx
    count_arr[0] = idx + 1
    return idx


def dt_depth(parents: list[int], idx: int) -> int:
    """Calculate depth from root (root = 0)."""
    depth: int = 0
    current: int = idx
    while current > 0:
        p: int = parents[current]
        if p < 0:
            return depth
        current = p
        depth = depth + 1
    return depth


def dt_is_ancestor(parents: list[int], ancestor: int, descendant: int) -> int:
    """Check if ancestor is an ancestor of descendant. Returns 1 if yes."""
    current: int = descendant
    steps: int = 0
    while steps < 100:
        if current == ancestor:
            return 1
        p: int = parents[current]
        if p < 0:
            return 0
        if current == 0:
            if ancestor == 0:
                return 1
            return 0
        current = p
        steps = steps + 1
    return 0


def dt_count_children(parents: list[int], count: int, parent_idx: int) -> int:
    """Count direct children of a directory."""
    total: int = 0
    i: int = 0
    while i < count:
        p: int = parents[i]
        if p == parent_idx:
            if i != parent_idx:
                total = total + 1
        i = i + 1
    return total


def dt_count_subtree(parents: list[int], count: int, root_idx: int) -> int:
    """Count all descendants (including self)."""
    total: int = 0
    i: int = 0
    while i < count:
        is_desc: int = dt_is_ancestor(parents, root_idx, i)
        if is_desc == 1:
            total = total + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test directory tree operations."""
    passed: int = 0
    cap: int = 16
    names: list[int] = dt_init(cap)
    parents: list[int] = dt_init(cap)
    cnt: list[int] = [0]

    # Create root (parent = -1)
    root: int = dt_mkdir(names, parents, cnt, 1, 0 - 1)
    # Create /home
    home: int = dt_mkdir(names, parents, cnt, 2, root)
    # Create /home/user
    user: int = dt_mkdir(names, parents, cnt, 3, home)
    # Create /etc
    etc: int = dt_mkdir(names, parents, cnt, 4, root)

    # Test 1: depth of root is 0
    d0: int = dt_depth(parents, root)
    if d0 == 0:
        passed = passed + 1

    # Test 2: depth of /home/user is 2
    d2: int = dt_depth(parents, user)
    if d2 == 2:
        passed = passed + 1

    # Test 3: root has 2 direct children
    children: int = dt_count_children(parents, cnt[0], root)
    if children == 2:
        passed = passed + 1

    # Test 4: ancestor check
    is_anc: int = dt_is_ancestor(parents, root, user)
    if is_anc == 1:
        passed = passed + 1

    # Test 5: subtree count
    sub: int = dt_count_subtree(parents, cnt[0], root)
    if sub == 4:
        passed = passed + 1

    return passed
