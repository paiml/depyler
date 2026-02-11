# Pathological dict: Graph adjacency list operations
# Tests: dict[str, list[int]] as adjacency list, BFS/DFS traversal
# Note: Using node IDs as ints, edge lists as list[int], adjacency via dict[int, int]


def build_adjacency_count(edges: list[int], num_edges: int) -> dict[int, int]:
    """Build degree count from edge list (pairs: src, dst)."""
    degree: dict[int, int] = {}
    i: int = 0
    while i + 1 < num_edges * 2:
        src: int = edges[i]
        dst: int = edges[i + 1]
        if src in degree:
            degree[src] = degree[src] + 1
        else:
            degree[src] = 1
        if dst in degree:
            degree[dst] = degree[dst] + 1
        else:
            degree[dst] = 1
        i = i + 2
    return degree


def count_self_loops(edges: list[int], num_edges: int) -> int:
    """Count edges where src == dst."""
    count: int = 0
    i: int = 0
    while i + 1 < num_edges * 2:
        if edges[i] == edges[i + 1]:
            count = count + 1
        i = i + 2
    return count


def find_isolated_nodes(degree: dict[int, int], all_nodes: list[int]) -> list[int]:
    """Find nodes not in degree map (degree 0)."""
    isolated: list[int] = []
    i: int = 0
    while i < len(all_nodes):
        nd: int = all_nodes[i]
        if nd not in degree:
            isolated.append(nd)
        i = i + 1
    return isolated


def max_degree_node(degree: dict[int, int], nodes: list[int]) -> int:
    """Find node with highest degree."""
    best_node: int = 0 - 1
    best_deg: int = 0 - 1
    i: int = 0
    while i < len(nodes):
        nd: int = nodes[i]
        if nd in degree:
            d: int = degree[nd]
            if d > best_deg:
                best_deg = d
                best_node = nd
        i = i + 1
    return best_node


def reachable_count(adj_flat: list[int], adj_offsets: list[int], adj_lengths: list[int],
                    start: int, num_nodes: int) -> int:
    """BFS reachability from start node using flat adjacency representation.
    adj_flat: all neighbors concatenated
    adj_offsets[i]: start index in adj_flat for node i
    adj_lengths[i]: number of neighbors for node i
    Returns count of reachable nodes (including start)."""
    visited: list[int] = []
    i: int = 0
    while i < num_nodes:
        visited.append(0)
        i = i + 1
    queue: list[int] = [start]
    visited_list: list[int] = []
    # Mark start as visited via rebuild
    j: int = 0
    while j < num_nodes:
        if j == start:
            visited_list.append(1)
        else:
            visited_list.append(0)
        j = j + 1
    count: int = 1
    front: int = 0
    while front < len(queue):
        node: int = queue[front]
        front = front + 1
        offset: int = adj_offsets[node]
        length: int = adj_lengths[node]
        k: int = 0
        while k < length:
            neighbor: int = adj_flat[offset + k]
            if visited_list[neighbor] == 0:
                # Mark visited via rebuild
                new_visited: list[int] = []
                m: int = 0
                while m < len(visited_list):
                    if m == neighbor:
                        new_visited.append(1)
                    else:
                        new_visited.append(visited_list[m])
                    m = m + 1
                visited_list = new_visited
                queue.append(neighbor)
                count = count + 1
            k = k + 1
    return count


def test_module() -> int:
    passed: int = 0
    # Test 1: degree count
    edges: list[int] = [0, 1, 1, 2, 2, 0, 0, 3]
    degree: dict[int, int] = build_adjacency_count(edges, 4)
    if degree[0] == 3:
        passed = passed + 1
    # Test 2: self loops
    edges2: list[int] = [0, 0, 1, 2, 3, 3]
    if count_self_loops(edges2, 3) == 2:
        passed = passed + 1
    # Test 3: isolated nodes
    all_nodes: list[int] = [0, 1, 2, 3, 4, 5]
    iso: list[int] = find_isolated_nodes(degree, all_nodes)
    if len(iso) == 2:
        passed = passed + 1
    # Test 4: max degree node
    if max_degree_node(degree, [0, 1, 2, 3]) == 0:
        passed = passed + 1
    # Test 5: BFS reachability (graph: 0->1, 0->2, 1->3, node 4 isolated)
    adj_flat: list[int] = [1, 2, 3]
    adj_offsets: list[int] = [0, 2, 3, 3, 3]
    adj_lengths: list[int] = [2, 1, 0, 0, 0]
    if reachable_count(adj_flat, adj_offsets, adj_lengths, 0, 5) == 4:
        passed = passed + 1
    return passed
