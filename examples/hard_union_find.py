"""Union-Find with path compression and union by rank using flat arrays."""


def uf_init(n: int) -> list[int]:
    """Initialize parent and rank arrays as flat [parent0..parentN, rank0..rankN]."""
    data: list[int] = []
    i: int = 0
    while i < n:
        data.append(i)
        i = i + 1
    j: int = 0
    while j < n:
        data.append(0)
        j = j + 1
    return data


def uf_find(data: list[int], x: int, n: int) -> int:
    """Find root of x with path compression."""
    root: int = x
    while data[root] != root:
        root = data[root]
    current: int = x
    while current != root:
        next_node: int = data[current]
        data[current] = root
        current = next_node
    return root


def uf_union(data: list[int], x: int, y: int, n: int) -> int:
    """Union two sets by rank. Returns 1 if merged, 0 if already same set."""
    rx: int = uf_find(data, x, n)
    ry: int = uf_find(data, y, n)
    if rx == ry:
        return 0
    rank_x: int = data[n + rx]
    rank_y: int = data[n + ry]
    if rank_x < rank_y:
        data[rx] = ry
    elif rank_x > rank_y:
        data[ry] = rx
    else:
        data[ry] = rx
        data[n + rx] = rank_x + 1
    return 1


def count_components(data: list[int], n: int) -> int:
    """Count number of connected components."""
    count: int = 0
    i: int = 0
    while i < n:
        if uf_find(data, i, n) == i:
            count = count + 1
        i = i + 1
    return count


def is_connected(data: list[int], x: int, y: int, n: int) -> int:
    """Check if x and y are in the same component. Returns 1/0."""
    if uf_find(data, x, n) == uf_find(data, y, n):
        return 1
    return 0


def test_module() -> int:
    passed: int = 0

    data: list[int] = uf_init(5)
    uf_union(data, 0, 1, 5)
    uf_union(data, 2, 3, 5)
    if is_connected(data, 0, 1, 5) == 1:
        passed = passed + 1

    if is_connected(data, 0, 2, 5) == 0:
        passed = passed + 1

    if count_components(data, 5) == 3:
        passed = passed + 1

    uf_union(data, 1, 3, 5)
    if is_connected(data, 0, 3, 5) == 1:
        passed = passed + 1

    if count_components(data, 5) == 2:
        passed = passed + 1

    data2: list[int] = uf_init(3)
    if count_components(data2, 3) == 3:
        passed = passed + 1

    merged: int = uf_union(data, 0, 1, 5)
    if merged == 0:
        passed = passed + 1

    return passed
