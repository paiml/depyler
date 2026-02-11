"""Topological sort using Kahn's algorithm with adjacency lists.

Tests: in-degree calculation, topological ordering, cycle detection.
"""


def compute_in_degree(num_nodes: int, edges_from: list[int], edges_to: list[int]) -> list[int]:
    """Compute in-degree for each node from edge lists."""
    degree: list[int] = [0] * num_nodes
    i: int = 0
    while i < len(edges_to):
        degree[edges_to[i]] = degree[edges_to[i]] + 1
        i = i + 1
    return degree


def topo_sort_count(num_nodes: int, edges_from: list[int], edges_to: list[int]) -> int:
    """Return count of nodes in topological order (less than num_nodes means cycle)."""
    degree: list[int] = compute_in_degree(num_nodes, edges_from, edges_to)
    queue: list[int] = []
    i: int = 0
    while i < num_nodes:
        if degree[i] == 0:
            queue.append(i)
        i = i + 1
    processed: int = 0
    while len(queue) > 0:
        node: int = queue[0]
        queue = queue[1:]
        processed = processed + 1
        j: int = 0
        while j < len(edges_from):
            if edges_from[j] == node:
                target: int = edges_to[j]
                degree[target] = degree[target] - 1
                if degree[target] == 0:
                    queue.append(target)
            j = j + 1
    return processed


def has_cycle_val(num_nodes: int, edges_from: list[int], edges_to: list[int]) -> int:
    """Detect if directed graph has a cycle. Returns 1 if cycle, 0 otherwise."""
    count: int = topo_sort_count(num_nodes, edges_from, edges_to)
    if count < num_nodes:
        return 1
    return 0


def max_path_length(num_nodes: int, edges_from: list[int], edges_to: list[int]) -> int:
    """Find longest path in a DAG using topological order."""
    degree: list[int] = compute_in_degree(num_nodes, edges_from, edges_to)
    dist: list[int] = [0] * num_nodes
    queue: list[int] = []
    i: int = 0
    while i < num_nodes:
        if degree[i] == 0:
            queue.append(i)
        i = i + 1
    while len(queue) > 0:
        node: int = queue[0]
        queue = queue[1:]
        j: int = 0
        while j < len(edges_from):
            if edges_from[j] == node:
                target: int = edges_to[j]
                candidate: int = dist[node] + 1
                if candidate > dist[target]:
                    dist[target] = candidate
                degree[target] = degree[target] - 1
                if degree[target] == 0:
                    queue.append(target)
            j = j + 1
    best: int = 0
    k: int = 0
    while k < num_nodes:
        if dist[k] > best:
            best = dist[k]
        k = k + 1
    return best


def test_module() -> None:
    ef: list[int] = [0, 0, 1, 2]
    et: list[int] = [1, 2, 3, 3]
    deg: list[int] = compute_in_degree(4, ef, et)
    assert deg[0] == 0
    assert deg[3] == 2
    assert topo_sort_count(4, ef, et) == 4
    assert has_cycle_val(4, ef, et) == 0
    cf: list[int] = [0, 1, 2]
    ct: list[int] = [1, 2, 0]
    assert has_cycle_val(3, cf, ct) == 1
    assert max_path_length(4, ef, et) == 2
