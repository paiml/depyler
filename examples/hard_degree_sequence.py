"""Graph degree sequence operations.

Tests: compute degree, sort degree sequence, max degree, is regular.
"""


def compute_degrees(n: int, edges_u: list[int], edges_v: list[int]) -> list[int]:
    """Compute degree of each node."""
    deg: list[int] = []
    i: int = 0
    while i < n:
        deg.append(0)
        i = i + 1
    e: int = len(edges_u)
    i = 0
    while i < e:
        deg[edges_u[i]] = deg[edges_u[i]] + 1
        deg[edges_v[i]] = deg[edges_v[i]] + 1
        i = i + 1
    return deg


def max_degree(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Maximum degree in the graph."""
    deg: list[int] = compute_degrees(n, edges_u, edges_v)
    best: int = 0
    for d in deg:
        if d > best:
            best = d
    return best


def min_degree(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Minimum degree in the graph."""
    deg: list[int] = compute_degrees(n, edges_u, edges_v)
    if len(deg) == 0:
        return 0
    best: int = deg[0]
    for d in deg:
        if d < best:
            best = d
    return best


def is_regular(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Returns 1 if all nodes have same degree."""
    deg: list[int] = compute_degrees(n, edges_u, edges_v)
    if n <= 1:
        return 1
    first: int = deg[0]
    i: int = 1
    while i < n:
        if deg[i] != first:
            return 0
        i = i + 1
    return 1


def sum_degrees(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Sum of all degrees (should be 2 * edges)."""
    deg: list[int] = compute_degrees(n, edges_u, edges_v)
    total: int = 0
    for d in deg:
        total = total + d
    return total


def test_module() -> int:
    """Test degree sequence."""
    ok: int = 0
    eu: list[int] = [0, 0, 1]
    ev: list[int] = [1, 2, 2]
    d: list[int] = compute_degrees(3, eu, ev)
    if d[0] == 2 and d[1] == 2 and d[2] == 2:
        ok = ok + 1
    if max_degree(3, eu, ev) == 2:
        ok = ok + 1
    if is_regular(3, eu, ev) == 1:
        ok = ok + 1
    if sum_degrees(3, eu, ev) == 6:
        ok = ok + 1
    eu2: list[int] = [0, 0, 0]
    ev2: list[int] = [1, 2, 3]
    if is_regular(4, eu2, ev2) == 0:
        ok = ok + 1
    if max_degree(4, eu2, ev2) == 3:
        ok = ok + 1
    return ok
