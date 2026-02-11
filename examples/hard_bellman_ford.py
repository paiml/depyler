"""Bellman-Ford shortest path using edge list representation."""


def bellman_ford(num_nodes: int, edges: list[int], num_edges: int, source: int) -> list[int]:
    """Bellman-Ford shortest paths from source.
    edges = flat [src, dst, weight, src, dst, weight, ...] triples.
    Returns dist array (999999999 = infinity, -1 = negative cycle detected at node).
    """
    inf: int = 999999999
    dist: list[int] = []
    i: int = 0
    while i < num_nodes:
        dist.append(inf)
        i = i + 1
    dist[source] = 0
    iteration: int = 0
    while iteration < num_nodes - 1:
        e: int = 0
        while e < num_edges:
            off: int = e * 3
            off1: int = off + 1
            off2: int = off + 2
            u: int = edges[off]
            v: int = edges[off1]
            w: int = edges[off2]
            if dist[u] != inf and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
            e = e + 1
        iteration = iteration + 1
    e2: int = 0
    while e2 < num_edges:
        off3: int = e2 * 3
        off4: int = off3 + 1
        off5: int = off3 + 2
        u2: int = edges[off3]
        v2: int = edges[off4]
        w2: int = edges[off5]
        if dist[u2] != inf and dist[u2] + w2 < dist[v2]:
            dist[v2] = -1
        e2 = e2 + 1
    return dist


def has_negative_cycle(num_nodes: int, edges: list[int], num_edges: int) -> int:
    """Check if graph has a negative weight cycle reachable from node 0. Returns 1/0."""
    dist: list[int] = bellman_ford(num_nodes, edges, num_edges, 0)
    i: int = 0
    while i < num_nodes:
        if dist[i] == -1:
            return 1
        i = i + 1
    return 0


def shortest_path_cost(num_nodes: int, edges: list[int], num_edges: int, source: int, dest: int) -> int:
    """Get shortest path cost from source to dest. Returns 999999999 if unreachable."""
    dist: list[int] = bellman_ford(num_nodes, edges, num_edges, source)
    return dist[dest]


def test_module() -> int:
    passed: int = 0

    edges1: list[int] = [0, 1, 4, 0, 2, 5, 1, 2, -3, 2, 3, 4]
    dist1: list[int] = bellman_ford(4, edges1, 4, 0)
    if dist1[0] == 0:
        passed = passed + 1

    if dist1[2] == 1:
        passed = passed + 1

    if dist1[3] == 5:
        passed = passed + 1

    cost: int = shortest_path_cost(4, edges1, 4, 0, 3)
    if cost == 5:
        passed = passed + 1

    edges2: list[int] = [0, 1, 1, 1, 2, -1, 2, 0, -1]
    if has_negative_cycle(3, edges2, 3) == 1:
        passed = passed + 1

    edges3: list[int] = [0, 1, 2, 1, 2, 3]
    if has_negative_cycle(3, edges3, 2) == 0:
        passed = passed + 1

    dist3: list[int] = bellman_ford(3, edges3, 2, 0)
    if dist3[2] == 5:
        passed = passed + 1

    return passed
