"""Maximum bipartite matching using augmenting paths (small graphs)."""


def try_augment(adj: list[int], n_left: int, n_right: int, u: int, visited: list[int], match_right: list[int]) -> int:
    """Try to find augmenting path from left vertex u. Returns 1 on success."""
    v: int = 0
    while v < n_right:
        if adj[u * n_right + v] == 1 and visited[v] == 0:
            visited[v] = 1
            if match_right[v] == 0 - 1:
                match_right[v] = u
                return 1
            prev: int = match_right[v]
            found: int = try_augment(adj, n_left, n_right, prev, visited, match_right)
            if found == 1:
                match_right[v] = u
                return 1
        v = v + 1
    return 0


def max_matching(adj: list[int], n_left: int, n_right: int) -> int:
    """Find maximum bipartite matching size."""
    match_right: list[int] = []
    i: int = 0
    while i < n_right:
        match_right.append(0 - 1)
        i = i + 1
    result: int = 0
    u: int = 0
    while u < n_left:
        visited: list[int] = []
        i = 0
        while i < n_right:
            visited.append(0)
            i = i + 1
        found: int = try_augment(adj, n_left, n_right, u, visited, match_right)
        if found == 1:
            result = result + 1
        u = u + 1
    return result


def count_adj_edges(adj: list[int], n_left: int, n_right: int) -> int:
    """Count total edges in bipartite graph."""
    count: int = 0
    i: int = 0
    while i < n_left:
        j: int = 0
        while j < n_right:
            if adj[i * n_right + j] == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def left_degree(adj: list[int], n_right: int, u: int) -> int:
    """Degree of left vertex u."""
    deg: int = 0
    j: int = 0
    while j < n_right:
        if adj[u * n_right + j] == 1:
            deg = deg + 1
        j = j + 1
    return deg


def test_module() -> int:
    """Test maximum bipartite matching."""
    ok: int = 0
    adj1: list[int] = [1, 1, 0, 0, 1, 0, 0, 0, 1]
    if max_matching(adj1, 3, 3) == 3:
        ok = ok + 1
    adj2: list[int] = [1, 0, 0, 1, 0, 0]
    if max_matching(adj2, 2, 3) == 2:
        ok = ok + 1
    adj3: list[int] = [1, 0, 1, 0]
    if max_matching(adj3, 2, 2) == 1:
        ok = ok + 1
    if count_adj_edges(adj1, 3, 3) == 4:
        ok = ok + 1
    if left_degree(adj1, 3, 0) == 2:
        ok = ok + 1
    empty: list[int] = [0, 0, 0, 0]
    if max_matching(empty, 2, 2) == 0:
        ok = ok + 1
    full: list[int] = [1, 1, 1, 1]
    if max_matching(full, 2, 2) == 2:
        ok = ok + 1
    return ok
