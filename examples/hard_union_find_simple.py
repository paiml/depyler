"""Union-Find (disjoint set) using arrays.

Tests: find, union, connected components count.
"""


def uf_init(n: int) -> list[int]:
    """Initialize union-find with n elements."""
    parent: list[int] = []
    i: int = 0
    while i < n:
        parent.append(i)
        i = i + 1
    return parent


def uf_find(parent: list[int], x: int) -> int:
    """Find root of element x."""
    root: int = x
    while parent[root] != root:
        root = parent[root]
    return root


def uf_union(parent: list[int], x: int, y: int) -> list[int]:
    """Union two elements by connecting their roots."""
    result: list[int] = []
    i: int = 0
    while i < len(parent):
        result.append(parent[i])
        i = i + 1
    rx: int = x
    while result[rx] != rx:
        rx = result[rx]
    ry: int = y
    while result[ry] != ry:
        ry = result[ry]
    if rx != ry:
        result[rx] = ry
    return result


def uf_connected_val(parent: list[int], x: int, y: int) -> int:
    """Check if two elements are in the same set. Returns 1 if connected, 0 otherwise."""
    rx: int = uf_find(parent, x)
    ry: int = uf_find(parent, y)
    if rx == ry:
        return 1
    return 0


def uf_count_components(parent: list[int]) -> int:
    """Count number of connected components."""
    n: int = len(parent)
    count: int = 0
    i: int = 0
    while i < n:
        root: int = i
        while parent[root] != root:
            root = parent[root]
        if root == i:
            count = count + 1
        i = i + 1
    return count


def test_module() -> None:
    p: list[int] = uf_init(5)
    assert len(p) == 5
    assert uf_count_components(p) == 5
    p = uf_union(p, 0, 1)
    assert uf_connected_val(p, 0, 1) == 1
    assert uf_connected_val(p, 0, 2) == 0
    assert uf_count_components(p) == 4
    p = uf_union(p, 2, 3)
    assert uf_count_components(p) == 3
    p = uf_union(p, 1, 3)
    assert uf_connected_val(p, 0, 3) == 1
    assert uf_count_components(p) == 2
