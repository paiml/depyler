"""Topological sort using Kahn's algorithm (BFS-based)."""


def compute_in_degree(adj: list[list[int]], n: int) -> list[int]:
    """Compute in-degree of each node."""
    in_deg: list[int] = []
    i: int = 0
    while i < n:
        in_deg.append(0)
        i = i + 1
    u: int = 0
    while u < n:
        j: int = 0
        while j < len(adj[u]):
            v: int = adj[u][j]
            in_deg[v] = in_deg[v] + 1
            j = j + 1
        u = u + 1
    return in_deg


def topological_sort_kahn(adj: list[list[int]], n: int) -> list[int]:
    """Topological sort using Kahn's algorithm. Returns empty if cycle."""
    in_deg: list[int] = compute_in_degree(adj, n)
    queue: list[int] = []
    i: int = 0
    while i < n:
        if in_deg[i] == 0:
            queue.append(i)
        i = i + 1
    result: list[int] = []
    front: int = 0
    while front < len(queue):
        node: int = queue[front]
        front = front + 1
        result.append(node)
        j: int = 0
        while j < len(adj[node]):
            nb: int = adj[node][j]
            in_deg[nb] = in_deg[nb] - 1
            if in_deg[nb] == 0:
                queue.append(nb)
            j = j + 1
    if len(result) != n:
        return []
    return result


def has_cycle(adj: list[list[int]], n: int) -> int:
    """Detect if directed graph has a cycle. Returns 1 or 0."""
    result: list[int] = topological_sort_kahn(adj, n)
    if len(result) == 0 and n > 0:
        return 1
    return 0


def is_valid_topological_order(adj: list[list[int]], order: list[int], n: int) -> int:
    """Verify if given order is a valid topological ordering. Returns 1 or 0."""
    if len(order) != n:
        return 0
    position: list[int] = []
    i: int = 0
    while i < n:
        position.append(-1)
        i = i + 1
    idx: int = 0
    while idx < len(order):
        position[order[idx]] = idx
        idx = idx + 1
    u: int = 0
    while u < n:
        j: int = 0
        while j < len(adj[u]):
            v: int = adj[u][j]
            if position[u] >= position[v]:
                return 0
            j = j + 1
        u = u + 1
    return 1


def test_module() -> int:
    passed: int = 0

    # DAG: 0->1, 0->2, 1->3, 2->3
    adj1: list[list[int]] = [[1, 2], [3], [3], []]
    ts1: list[int] = topological_sort_kahn(adj1, 4)
    if len(ts1) == 4 and ts1[0] == 0:
        passed = passed + 1

    valid1: int = is_valid_topological_order(adj1, ts1, 4)
    if valid1 == 1:
        passed = passed + 1

    # Cycle: 0->1->2->0
    adj2: list[list[int]] = [[1], [2], [0]]
    ts2: list[int] = topological_sort_kahn(adj2, 3)
    if ts2 == []:
        passed = passed + 1

    if has_cycle(adj2, 3) == 1:
        passed = passed + 1

    if has_cycle(adj1, 4) == 0:
        passed = passed + 1

    # Single node
    adj3: list[list[int]] = [[]]
    ts3: list[int] = topological_sort_kahn(adj3, 1)
    if ts3 == [0]:
        passed = passed + 1

    in_deg: list[int] = compute_in_degree(adj1, 4)
    if in_deg[0] == 0 and in_deg[3] == 2:
        passed = passed + 1

    return passed
