"""Bipartite graph checking using BFS coloring on adjacency list (flat arrays)."""


def is_bipartite(num_nodes: int, adj: list[int], adj_offsets: list[int]) -> int:
    """Check if undirected graph is bipartite using BFS.
    adj = flat adjacency list, adj_offsets[i] = start index for node i's neighbors,
    adj_offsets[num_nodes] = total length of adj.
    Returns 1 if bipartite, 0 otherwise.
    """
    color: list[int] = []
    i: int = 0
    while i < num_nodes:
        color.append(-1)
        i = i + 1
    queue: list[int] = []
    q_idx: int = 0
    while q_idx < num_nodes:
        queue.append(-1)
        q_idx = q_idx + 1
    node: int = 0
    while node < num_nodes:
        if color[node] != -1:
            node = node + 1
            continue
        color[node] = 0
        q_front: int = 0
        q_back: int = 0
        queue[q_back] = node
        q_back = q_back + 1
        while q_front < q_back:
            u: int = queue[q_front]
            q_front = q_front + 1
            start: int = adj_offsets[u]
            end: int = adj_offsets[u + 1]
            idx: int = start
            while idx < end:
                v: int = adj[idx]
                if color[v] == -1:
                    color[v] = 1 - color[u]
                    queue[q_back] = v
                    q_back = q_back + 1
                elif color[v] == color[u]:
                    return 0
                idx = idx + 1
        node = node + 1
    return 1


def count_edges_bipartite(num_nodes: int, adj_offsets: list[int]) -> int:
    """Count total edges in undirected graph (each edge counted once)."""
    total: int = 0
    i: int = 0
    while i < num_nodes:
        degree: int = adj_offsets[i + 1] - adj_offsets[i]
        total = total + degree
        i = i + 1
    return total // 2


def max_degree(num_nodes: int, adj_offsets: list[int]) -> int:
    """Find maximum degree in the graph."""
    best: int = 0
    i: int = 0
    while i < num_nodes:
        degree: int = adj_offsets[i + 1] - adj_offsets[i]
        if degree > best:
            best = degree
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    adj1: list[int] = [1, 3, 0, 2, 1, 3, 0, 2]
    off1: list[int] = [0, 2, 4, 6, 8]
    if is_bipartite(4, adj1, off1) == 1:
        passed = passed + 1

    adj2: list[int] = [1, 2, 0, 2, 0, 1]
    off2: list[int] = [0, 2, 4, 6]
    if is_bipartite(3, adj2, off2) == 0:
        passed = passed + 1

    if count_edges_bipartite(4, off1) == 4:
        passed = passed + 1

    if max_degree(4, off1) == 2:
        passed = passed + 1

    adj3: list[int] = [1, 0]
    off3: list[int] = [0, 1, 2]
    if is_bipartite(2, adj3, off3) == 1:
        passed = passed + 1

    adj4: list[int] = []
    off4: list[int] = [0, 0, 0]
    if is_bipartite(2, adj4, off4) == 1:
        passed = passed + 1

    if count_edges_bipartite(3, off2) == 3:
        passed = passed + 1

    return passed
