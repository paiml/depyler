"""Check Hamiltonian path in small graph (adjacency matrix)."""


def is_edge(adj: list[int], n: int, u: int, v: int) -> int:
    """Returns 1 if edge exists between u and v."""
    return adj[u * n + v]


def has_hamiltonian_path_helper(adj: list[int], n: int, visited: list[int], cur: int, depth: int) -> int:
    """Backtracking helper for Hamiltonian path."""
    if depth == n:
        return 1
    v: int = 0
    while v < n:
        if is_edge(adj, n, cur, v) == 1 and visited[v] == 0:
            visited[v] = 1
            found: int = has_hamiltonian_path_helper(adj, n, visited, v, depth + 1)
            if found == 1:
                return 1
            visited[v] = 0
        v = v + 1
    return 0


def has_hamiltonian_path(adj: list[int], n: int) -> int:
    """Check if graph has a Hamiltonian path starting from any vertex."""
    start: int = 0
    while start < n:
        visited: list[int] = []
        i: int = 0
        while i < n:
            visited.append(0)
            i = i + 1
        visited[start] = 1
        found: int = has_hamiltonian_path_helper(adj, n, visited, start, 1)
        if found == 1:
            return 1
        start = start + 1
    return 0


def count_edges(adj: list[int], n: int) -> int:
    """Count edges in undirected graph."""
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if adj[i * n + j] == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def degree_of(adj: list[int], n: int, v: int) -> int:
    """Compute degree of vertex v."""
    deg: int = 0
    j: int = 0
    while j < n:
        if adj[v * n + j] == 1:
            deg = deg + 1
        j = j + 1
    return deg


def test_module() -> int:
    """Test Hamiltonian path checking."""
    ok: int = 0
    path4: list[int] = [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0]
    if has_hamiltonian_path(path4, 4) == 1:
        ok = ok + 1
    disconn: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    if has_hamiltonian_path(disconn, 4) == 0:
        ok = ok + 1
    complete3: list[int] = [0, 1, 1, 1, 0, 1, 1, 1, 0]
    if has_hamiltonian_path(complete3, 3) == 1:
        ok = ok + 1
    if count_edges(complete3, 3) == 3:
        ok = ok + 1
    if degree_of(complete3, 3, 0) == 2:
        ok = ok + 1
    if count_edges(path4, 4) == 3:
        ok = ok + 1
    single: list[int] = [0]
    if has_hamiltonian_path(single, 1) == 1:
        ok = ok + 1
    return ok
