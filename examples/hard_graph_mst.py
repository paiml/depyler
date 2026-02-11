# Minimum spanning tree using Prim's algorithm with flat adjacency list


def prims_mst_cost(adj_flat: list[int], adj_off: list[int], n: int) -> int:
    # Find MST cost using Prim's algorithm.
    # adj_flat contains all [v, w, v, w, ...] concatenated
    # adj_off[u] = start index in adj_flat for node u, adj_off[n] = sentinel
    if n == 0:
        return 0
    in_mst: list[int] = []
    dist: list[int] = []
    i: int = 0
    while i < n:
        in_mst.append(0)
        dist.append(999999)
        i = i + 1
    dist[0] = 0
    total_cost: int = 0
    count: int = 0
    while count < n:
        u: int = -1
        min_dist: int = 999999
        j: int = 0
        while j < n:
            if in_mst[j] == 0 and dist[j] < min_dist:
                min_dist = dist[j]
                u = j
            j = j + 1
        if u == -1:
            return total_cost
        in_mst[u] = 1
        total_cost = total_cost + dist[u]
        count = count + 1
        k: int = adj_off[u]
        limit: int = adj_off[u + 1]
        while k < limit:
            v: int = adj_flat[k]
            w: int = adj_flat[k + 1]
            if in_mst[v] == 0 and w < dist[v]:
                dist[v] = w
            k = k + 2
    return total_cost


def count_edges(adj_off: list[int], n: int) -> int:
    # Count total undirected edges
    total: int = 0
    u: int = 0
    while u < n:
        edge_count: int = (adj_off[u + 1] - adj_off[u]) // 2
        total = total + edge_count
        u = u + 1
    return total // 2


def min_edge_weight(adj_flat: list[int], adj_off: list[int], n: int) -> int:
    # Find minimum edge weight in graph
    min_w: int = 999999
    u: int = 0
    while u < n:
        k: int = adj_off[u]
        limit: int = adj_off[u + 1]
        while k < limit:
            w: int = adj_flat[k + 1]
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
    # Node 0: [1,1, 2,3], Node 1: [0,1, 2,2], Node 2: [1,2, 0,3]
    flat1: list[int] = [1, 1, 2, 3, 0, 1, 2, 2, 1, 2, 0, 3]
    off1: list[int] = [0, 4, 8, 12]
    cost1: int = prims_mst_cost(flat1, off1, 3)
    if cost1 == 3:
        passed = passed + 1

    # Square: 0-1(1), 1-2(4), 2-3(2), 0-3(3)
    # Node 0: [1,1, 3,3], Node 1: [0,1, 2,4], Node 2: [1,4, 3,2], Node 3: [2,2, 0,3]
    flat2: list[int] = [1, 1, 3, 3, 0, 1, 2, 4, 1, 4, 3, 2, 2, 2, 0, 3]
    off2: list[int] = [0, 4, 8, 12, 16]
    cost2: int = prims_mst_cost(flat2, off2, 4)
    if cost2 == 6:
        passed = passed + 1

    # Single node (no edges)
    flat3: list[int] = []
    off3: list[int] = [0, 0]
    cost3: int = prims_mst_cost(flat3, off3, 1)
    if cost3 == 0:
        passed = passed + 1

    ec1: int = count_edges(off1, 3)
    if ec1 == 3:
        passed = passed + 1

    mw1: int = min_edge_weight(flat1, off1, 3)
    if mw1 == 1:
        passed = passed + 1

    # Empty graph (0 nodes)
    cost4: int = prims_mst_cost([], [0], 0)
    if cost4 == 0:
        passed = passed + 1

    # Single node no edges
    mw2: int = min_edge_weight([], [0, 0], 1)
    if mw2 == -1:
        passed = passed + 1

    return passed
