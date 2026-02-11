"""Adjacency matrix operations for graphs.

Tests: build adjacency matrix, count edges, is symmetric, has self loop.
"""


def build_adjacency_matrix(n: int, edges_u: list[int], edges_v: list[int]) -> list[int]:
    """Build flat NxN adjacency matrix (undirected)."""
    mat: list[int] = []
    i: int = 0
    while i < n * n:
        mat.append(0)
        i = i + 1
    e: int = len(edges_u)
    i = 0
    while i < e:
        u: int = edges_u[i]
        v: int = edges_v[i]
        mat[u * n + v] = 1
        mat[v * n + u] = 1
        i = i + 1
    return mat


def count_edges_from_matrix(mat: list[int], n: int) -> int:
    """Count edges from adjacency matrix (undirected, no self-loops)."""
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if mat[i * n + j] == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def is_symmetric(mat: list[int], n: int) -> int:
    """Returns 1 if matrix is symmetric."""
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if mat[i * n + j] != mat[j * n + i]:
                return 0
            j = j + 1
        i = i + 1
    return 1


def has_self_loop(mat: list[int], n: int) -> int:
    """Returns 1 if any diagonal element is non-zero."""
    i: int = 0
    while i < n:
        if mat[i * n + i] != 0:
            return 1
        i = i + 1
    return 0


def node_degree_from_matrix(mat: list[int], n: int, node: int) -> int:
    """Degree of a node from adjacency matrix."""
    deg: int = 0
    j: int = 0
    while j < n:
        deg = deg + mat[node * n + j]
        j = j + 1
    return deg


def test_module() -> int:
    """Test adjacency operations."""
    ok: int = 0
    eu: list[int] = [0, 0, 1]
    ev: list[int] = [1, 2, 2]
    mat: list[int] = build_adjacency_matrix(3, eu, ev)
    if mat[0 * 3 + 1] == 1 and mat[1 * 3 + 0] == 1:
        ok = ok + 1
    if count_edges_from_matrix(mat, 3) == 3:
        ok = ok + 1
    if is_symmetric(mat, 3) == 1:
        ok = ok + 1
    if has_self_loop(mat, 3) == 0:
        ok = ok + 1
    if node_degree_from_matrix(mat, 3, 0) == 2:
        ok = ok + 1
    return ok
