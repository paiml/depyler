"""Tree diameter from adjacency list encoded as flat arrays."""


def bfs_farthest(adj_start: list[int], adj_end: list[int], adj_to: list[int], n: int, source: int) -> list[int]:
    """BFS from source. Returns [farthest_node, max_distance].

    adj_start[u]..adj_end[u] gives range in adj_to for neighbors of u.
    """
    dist: list[int] = []
    i: int = 0
    while i < n:
        dist.append(0 - 1)
        i = i + 1
    dist[source] = 0
    queue: list[int] = [source]
    farthest: int = source
    max_dist: int = 0
    while len(queue) > 0:
        curr: int = queue[0]
        queue.pop(0)
        start: int = adj_start[curr]
        end: int = adj_end[curr]
        idx: int = start
        while idx < end:
            neighbor: int = adj_to[idx]
            if dist[neighbor] == 0 - 1:
                dist[neighbor] = dist[curr] + 1
                if dist[neighbor] > max_dist:
                    max_dist = dist[neighbor]
                    farthest = neighbor
                queue.append(neighbor)
            idx = idx + 1
    result: list[int] = [farthest, max_dist]
    return result


def build_adj_lists(n: int, edges_u: list[int], edges_v: list[int]) -> list[int]:
    """Build CSR-style adjacency. Returns [adj_start(n), adj_end(n), adj_to(2*m)]."""
    m: int = len(edges_u)
    degree: list[int] = []
    i: int = 0
    while i < n:
        degree.append(0)
        i = i + 1
    i = 0
    while i < m:
        degree[edges_u[i]] = degree[edges_u[i]] + 1
        degree[edges_v[i]] = degree[edges_v[i]] + 1
        i = i + 1
    adj_start: list[int] = []
    offset: int = 0
    i = 0
    while i < n:
        adj_start.append(offset)
        offset = offset + degree[i]
        i = i + 1
    adj_end: list[int] = []
    i = 0
    while i < n:
        adj_end.append(adj_start[i])
        i = i + 1
    adj_to: list[int] = []
    i = 0
    while i < 2 * m:
        adj_to.append(0)
        i = i + 1
    i = 0
    while i < m:
        u: int = edges_u[i]
        v: int = edges_v[i]
        adj_to[adj_end[u]] = v
        adj_end[u] = adj_end[u] + 1
        adj_to[adj_end[v]] = u
        adj_end[v] = adj_end[v] + 1
        i = i + 1
    packed: list[int] = []
    i = 0
    while i < n:
        packed.append(adj_start[i])
        i = i + 1
    i = 0
    while i < n:
        packed.append(adj_end[i])
        i = i + 1
    i = 0
    while i < 2 * m:
        packed.append(adj_to[i])
        i = i + 1
    return packed


def tree_diameter(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Compute tree diameter using two BFS passes."""
    if n <= 1:
        return 0
    m: int = len(edges_u)
    packed: list[int] = build_adj_lists(n, edges_u, edges_v)
    adj_start: list[int] = []
    adj_end: list[int] = []
    adj_to: list[int] = []
    i: int = 0
    while i < n:
        adj_start.append(packed[i])
        i = i + 1
    i = 0
    while i < n:
        adj_end.append(packed[n + i])
        i = i + 1
    i = 0
    while i < 2 * m:
        adj_to.append(packed[2 * n + i])
        i = i + 1
    first: list[int] = bfs_farthest(adj_start, adj_end, adj_to, n, 0)
    far_node: int = first[0]
    second: list[int] = bfs_farthest(adj_start, adj_end, adj_to, n, far_node)
    return second[1]


def test_module() -> int:
    """Test tree diameter."""
    passed: int = 0

    eu1: list[int] = [0, 1, 2, 3]
    ev1: list[int] = [1, 2, 3, 4]
    if tree_diameter(5, eu1, ev1) == 4:
        passed = passed + 1

    eu2: list[int] = [0, 0, 0]
    ev2: list[int] = [1, 2, 3]
    if tree_diameter(4, eu2, ev2) == 2:
        passed = passed + 1

    eu3: list[int] = [0]
    ev3: list[int] = [1]
    if tree_diameter(2, eu3, ev3) == 1:
        passed = passed + 1

    empty_u: list[int] = []
    empty_v: list[int] = []
    if tree_diameter(1, empty_u, empty_v) == 0:
        passed = passed + 1

    eu4: list[int] = [0, 1, 1]
    ev4: list[int] = [1, 2, 3]
    if tree_diameter(4, eu4, ev4) == 3:
        passed = passed + 1

    eu5: list[int] = [0, 1, 2, 3, 4]
    ev5: list[int] = [1, 2, 3, 4, 5]
    if tree_diameter(6, eu5, ev5) == 5:
        passed = passed + 1

    return passed
