"""Distance-vector routing simulation.

Implements Bellman-Ford based routing table updates.
Graph stored as flat edge list: [src, dst, cost, src, dst, cost, ...].
"""


def rt_init(size: int) -> list[int]:
    """Initialize with large value (infinity)."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(999999)
        i = i + 1
    return result


def rt_init_neg(size: int) -> list[int]:
    """Initialize with -1."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0 - 1)
        i = i + 1
    return result


def rt_bellman_ford(edges: list[int], num_edges: int, num_nodes: int,
                    source: int, dist: list[int], prev_hop: list[int]) -> int:
    """Run Bellman-Ford from source. Returns 0.
    dist[i] = shortest distance from source to i.
    prev_hop[i] = predecessor on shortest path."""
    dist[source] = 0
    iteration: int = 0
    while iteration < num_nodes - 1:
        e: int = 0
        while e < num_edges:
            u: int = edges[e * 3]
            v: int = edges[e * 3 + 1]
            w: int = edges[e * 3 + 2]
            d_u: int = dist[u]
            d_v: int = dist[v]
            new_dist: int = d_u + w
            if new_dist < d_v:
                dist[v] = new_dist
                prev_hop[v] = u
            e = e + 1
        iteration = iteration + 1
    return 0


def rt_get_distance(dist: list[int], node: int) -> int:
    """Get distance to node."""
    result: int = dist[node]
    return result


def rt_get_next_hop(prev_hop: list[int], source: int, dest: int) -> int:
    """Trace back from dest to find next hop from source."""
    if dest == source:
        return source
    current: int = dest
    steps: int = 0
    while steps < 100:
        p: int = prev_hop[current]
        if p == source:
            return current
        if p < 0:
            return 0 - 1
        current = p
        steps = steps + 1
    return 0 - 1


def rt_has_negative_cycle(edges: list[int], num_edges: int,
                          dist: list[int]) -> int:
    """Check for negative weight cycles. Returns 1 if found."""
    e: int = 0
    while e < num_edges:
        u: int = edges[e * 3]
        v: int = edges[e * 3 + 1]
        w: int = edges[e * 3 + 2]
        d_u: int = dist[u]
        d_v: int = dist[v]
        if d_u + w < d_v:
            return 1
        e = e + 1
    return 0


def test_module() -> int:
    """Test routing algorithm."""
    passed: int = 0
    num_nodes: int = 4
    # Edges: 0->1:1, 1->2:3, 0->2:5, 2->3:2, 1->3:7
    edges: list[int] = [0, 1, 1, 1, 2, 3, 0, 2, 5, 2, 3, 2, 1, 3, 7]
    num_edges: int = 5
    dist: list[int] = rt_init(num_nodes)
    prev_hop: list[int] = rt_init_neg(num_nodes)

    # Test 1: Bellman-Ford runs without error
    rt_bellman_ford(edges, num_edges, num_nodes, 0, dist, prev_hop)
    d0: int = rt_get_distance(dist, 0)
    if d0 == 0:
        passed = passed + 1

    # Test 2: shortest distance to node 1
    d1: int = rt_get_distance(dist, 1)
    if d1 == 1:
        passed = passed + 1

    # Test 3: shortest distance to node 2 (0->1->2 = 4, not 0->2 = 5)
    d2: int = rt_get_distance(dist, 2)
    if d2 == 4:
        passed = passed + 1

    # Test 4: shortest distance to node 3 (0->1->2->3 = 6)
    d3: int = rt_get_distance(dist, 3)
    if d3 == 6:
        passed = passed + 1

    # Test 5: no negative cycle
    neg: int = rt_has_negative_cycle(edges, num_edges, dist)
    if neg == 0:
        passed = passed + 1

    return passed
