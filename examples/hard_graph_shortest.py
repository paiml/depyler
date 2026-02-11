"""Dijkstra's shortest path using array (no priority queue)."""


def dijkstra(adj_weight: list[list[int]], src: int, n: int) -> list[int]:
    """Dijkstra from src. adj_weight[u] = [v1, w1, v2, w2, ...].
    Returns dist array. dist[i] = shortest dist from src to i, or 999999.
    """
    dist: list[int] = []
    visited: list[int] = []
    i: int = 0
    while i < n:
        dist.append(999999)
        visited.append(0)
        i = i + 1
    dist[src] = 0
    count: int = 0
    while count < n:
        # Find unvisited with min dist
        u: int = -1
        min_d: int = 999999
        j: int = 0
        while j < n:
            if visited[j] == 0 and dist[j] < min_d:
                min_d = dist[j]
                u = j
            j = j + 1
        if u == -1:
            count = n
        else:
            visited[u] = 1
            k: int = 0
            while k < len(adj_weight[u]):
                v: int = adj_weight[u][k]
                w: int = adj_weight[u][k + 1]
                new_dist: int = dist[u] + w
                if new_dist < dist[v]:
                    dist[v] = new_dist
                k = k + 2
            count = count + 1
    return dist


def shortest_dist(adj_weight: list[list[int]], src: int, dst: int, n: int) -> int:
    """Get shortest distance from src to dst. Returns -1 if unreachable."""
    d: list[int] = dijkstra(adj_weight, src, n)
    if d[dst] == 999999:
        return -1
    return d[dst]


def is_reachable(adj_weight: list[list[int]], src: int, dst: int, n: int) -> int:
    """Check if dst is reachable from src. Returns 1 or 0."""
    d: list[int] = dijkstra(adj_weight, src, n)
    if d[dst] == 999999:
        return 0
    return 1


def test_module() -> int:
    passed: int = 0

    # 0->1(4), 0->2(1), 2->1(2), 1->3(1), 2->3(5)
    adj1: list[list[int]] = [[1, 4, 2, 1], [3, 1], [1, 2, 3, 5], []]
    d1: list[int] = dijkstra(adj1, 0, 4)
    if d1[0] == 0:
        passed = passed + 1

    if d1[1] == 3:
        passed = passed + 1

    if d1[3] == 4:
        passed = passed + 1

    sd1: int = shortest_dist(adj1, 0, 3, 4)
    if sd1 == 4:
        passed = passed + 1

    # Disconnected: 0->1(1), 2 isolated
    adj2: list[list[int]] = [[1, 1], [], []]
    sd2: int = shortest_dist(adj2, 0, 2, 3)
    if sd2 == -1:
        passed = passed + 1

    if is_reachable(adj2, 0, 1, 3) == 1:
        passed = passed + 1

    if is_reachable(adj2, 0, 2, 3) == 0:
        passed = passed + 1

    return passed
