"""Union-Find (Disjoint Set) with path compression using lists.

Tests: make_set, find, union, connected, count components.
"""


def uf_create(n: int) -> list[int]:
    """Create union-find of n elements. parent[i]=i initially."""
    parent: list[int] = []
    i: int = 0
    while i < n:
        parent.append(i)
        i = i + 1
    return parent


def uf_rank_create(n: int) -> list[int]:
    """Create rank array for union by rank."""
    rank: list[int] = []
    i: int = 0
    while i < n:
        rank.append(0)
        i = i + 1
    return rank


def uf_find(parent: list[int], x: int) -> int:
    """Find root with path compression."""
    root: int = x
    while parent[root] != root:
        root = parent[root]
    curr: int = x
    while curr != root:
        nxt: int = parent[curr]
        parent[curr] = root
        curr = nxt
    return root


def uf_union(parent: list[int], rank: list[int], x: int, y: int) -> int:
    """Union by rank. Returns 1 if merged, 0 if already same set."""
    rx: int = uf_find(parent, x)
    ry: int = uf_find(parent, y)
    if rx == ry:
        return 0
    rank_rx: int = rank[rx]
    rank_ry: int = rank[ry]
    if rank_rx < rank_ry:
        parent[rx] = ry
    elif rank_rx > rank_ry:
        parent[ry] = rx
    else:
        parent[ry] = rx
        rank[rx] = rank_rx + 1
    return 1


def uf_connected(parent: list[int], x: int, y: int) -> int:
    """Return 1 if x and y are in the same set."""
    if uf_find(parent, x) == uf_find(parent, y):
        return 1
    return 0


def uf_count_components(parent: list[int]) -> int:
    """Count number of distinct components."""
    n: int = len(parent)
    count: int = 0
    i: int = 0
    while i < n:
        root: int = uf_find(parent, i)
        if root == i:
            count = count + 1
        i = i + 1
    return count


def uf_component_sizes(parent: list[int]) -> list[int]:
    """Return list of component sizes."""
    n: int = len(parent)
    sizes: list[int] = []
    i: int = 0
    while i < n:
        sizes.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        root: int = uf_find(parent, j)
        sizes[root] = sizes[root] + 1
        j = j + 1
    result: list[int] = []
    k: int = 0
    while k < n:
        if sizes[k] > 0:
            result.append(sizes[k])
        k = k + 1
    return result


def test_module() -> int:
    """Test union-find operations."""
    passed: int = 0

    p: list[int] = uf_create(6)
    r: list[int] = uf_rank_create(6)

    uf_union(p, r, 0, 1)
    uf_union(p, r, 2, 3)
    uf_union(p, r, 4, 5)

    if uf_connected(p, 0, 1) == 1:
        passed = passed + 1

    if uf_connected(p, 0, 2) == 0:
        passed = passed + 1

    if uf_count_components(p) == 3:
        passed = passed + 1

    uf_union(p, r, 1, 3)
    if uf_connected(p, 0, 2) == 1:
        passed = passed + 1

    if uf_count_components(p) == 2:
        passed = passed + 1

    uf_union(p, r, 0, 5)
    if uf_count_components(p) == 1:
        passed = passed + 1

    cs: list[int] = uf_component_sizes(p)
    if len(cs) == 1:
        passed = passed + 1

    return passed
