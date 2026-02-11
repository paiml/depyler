"""Graph connectivity using adjacency list (flat arrays).

Tests: is connected, count components, has path, node degree.
"""


def find_root(parent: list[int], x: int) -> int:
    """Find root with path compression via iteration."""
    r: int = x
    while parent[r] != r:
        r = parent[r]
    return r


def union_nodes(parent: list[int], rank: list[int], a: int, b: int) -> None:
    """Union by rank."""
    ra: int = find_root(parent, a)
    rb: int = find_root(parent, b)
    if ra == rb:
        return
    if rank[ra] < rank[rb]:
        parent[ra] = rb
    elif rank[ra] > rank[rb]:
        parent[rb] = ra
    else:
        parent[rb] = ra
        rank[ra] = rank[ra] + 1


def count_components(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Count connected components. edges_u[i]-edges_v[i] is an edge."""
    parent: list[int] = []
    rank: list[int] = []
    i: int = 0
    while i < n:
        parent.append(i)
        rank.append(0)
        i = i + 1
    e: int = len(edges_u)
    i = 0
    while i < e:
        union_nodes(parent, rank, edges_u[i], edges_v[i])
        i = i + 1
    components: int = 0
    i = 0
    while i < n:
        if find_root(parent, i) == i:
            components = components + 1
        i = i + 1
    return components


def has_path(n: int, edges_u: list[int], edges_v: list[int], src: int, dst: int) -> int:
    """Returns 1 if path exists from src to dst."""
    parent: list[int] = []
    rank: list[int] = []
    i: int = 0
    while i < n:
        parent.append(i)
        rank.append(0)
        i = i + 1
    e: int = len(edges_u)
    i = 0
    while i < e:
        union_nodes(parent, rank, edges_u[i], edges_v[i])
        i = i + 1
    if find_root(parent, src) == find_root(parent, dst):
        return 1
    return 0


def node_degree(node: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Count degree of a node."""
    deg: int = 0
    i: int = 0
    while i < len(edges_u):
        if edges_u[i] == node or edges_v[i] == node:
            deg = deg + 1
        i = i + 1
    return deg


def test_module() -> int:
    """Test graph connectivity."""
    ok: int = 0
    eu: list[int] = [0, 1, 3]
    ev: list[int] = [1, 2, 4]
    if count_components(5, eu, ev) == 2:
        ok = ok + 1
    if has_path(5, eu, ev, 0, 2) == 1:
        ok = ok + 1
    if has_path(5, eu, ev, 0, 4) == 0:
        ok = ok + 1
    if node_degree(1, eu, ev) == 2:
        ok = ok + 1
    if count_components(3, [], []) == 3:
        ok = ok + 1
    return ok
