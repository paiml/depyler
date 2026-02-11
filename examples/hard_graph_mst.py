"""Minimum spanning tree using Prim's algorithm with array (no priority queue)."""


def prims_mst_cost(adj_weight: list[list[int]], n: int) -> int:
    """Find MST cost using Prim's algorithm.
    adj_weight is flattened: adj_weight[u] = [v1, w1, v2, w2, ...]
    """
    if n == 0:
        return 0
    in_mst: list[int] = []
    key: list[int] = []
    i: int = 0
    while i < n:
        in_mst.append(0)
        key.append(999999)
        i = i + 1
    key[0] = 0
    total_cost: int = 0
    count: int = 0
    while count < n:
        # Find min key not in MST
        u: int = -1
        min_key: int = 999999
        j: int = 0
        while j < n:
            if in_mst[j] == 0 and key[j] < min_key:
                min_key = key[j]
                u = j
            j = j + 1
        if u == -1:
            return total_cost
        in_mst[u] = 1
        total_cost = total_cost + key[u]
        count = count + 1
        # Update neighbors
        k: int = 0
        while k < len(adj_weight[u]):
            v: int = adj_weight[u][k]
            w: int = adj_weight[u][k + 1]
            if in_mst[v] == 0 and w < key[v]:
                key[v] = w
            k = k + 2
    return total_cost


def count_edges(adj_weight: list[list[int]], n: int) -> int:
    """Count total undirected edges."""
    total: int = 0
    u: int = 0
    while u < n:
        total = total + len(adj_weight[u]) // 2
        u = u + 1
    return total // 2


def min_edge_weight(adj_weight: list[list[int]], n: int) -> int:
    """Find minimum edge weight in graph."""
    min_w: int = 999999
    u: int = 0
    while u < n:
        k: int = 0
        while k < len(adj_weight[u]):
            w: int = adj_weight[u][k + 1]
            if w < min_w:
                min_w = w
            k = k + 2
        u = u + 1
    if min_w == 999999:
        return -1
    return min_w


def test_module() -> int:
    passed: int = 0

    # Triangle: 0-1(1), 1-2(2), 0-2(3)
    adj1: list[list[int]] = [[1, 1, 2, 3], [0, 1, 2, 2], [1, 2, 0, 3]]
    cost1: int = prims_mst_cost(adj1, 3)
    if cost1 == 3:
        passed = passed + 1

    # Square: 0-1(1), 1-2(4), 2-3(2), 0-3(3)
    adj2: list[list[int]] = [[1, 1, 3, 3], [0, 1, 2, 4], [1, 4, 3, 2], [2, 2, 0, 3]]
    cost2: int = prims_mst_cost(adj2, 4)
    if cost2 == 6:
        passed = passed + 1

    # Single node
    adj3: list[list[int]] = [[]]
    cost3: int = prims_mst_cost(adj3, 1)
    if cost3 == 0:
        passed = passed + 1

    ec1: int = count_edges(adj1, 3)
    if ec1 == 3:
        passed = passed + 1

    mw1: int = min_edge_weight(adj1, 3)
    if mw1 == 1:
        passed = passed + 1

    # Empty graph
    cost4: int = prims_mst_cost([], 0)
    if cost4 == 0:
        passed = passed + 1

    mw2: int = min_edge_weight([[]], 1)
    if mw2 == -1:
        passed = passed + 1

    return passed
