"""Graph traversal algorithms using adjacency list representation.

Tests: BFS, DFS iterative, connected components, path existence,
and shortest path length.
"""


def bfs_order(adj: dict[int, list[int]], start: int) -> list[int]:
    """BFS traversal order from start node."""
    visited: dict[int, bool] = {start: True}
    queue: list[int] = [start]
    result: list[int] = []
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head = head + 1
        result.append(node)
        if node in adj:
            neighbors: list[int] = adj[node]
            i: int = 0
            while i < len(neighbors):
                nb: int = neighbors[i]
                if nb not in visited:
                    visited[nb] = True
                    queue.append(nb)
                i = i + 1
    return result


def count_reachable(adj: dict[int, list[int]], start: int) -> int:
    """Count number of nodes reachable from start via BFS."""
    visited: dict[int, bool] = {start: True}
    queue: list[int] = [start]
    count: int = 1
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head = head + 1
        if node in adj:
            neighbors: list[int] = adj[node]
            i: int = 0
            while i < len(neighbors):
                nb: int = neighbors[i]
                if nb not in visited:
                    visited[nb] = True
                    queue.append(nb)
                    count = count + 1
                i = i + 1
    return count


def has_path(adj: dict[int, list[int]], src: int, dst: int) -> bool:
    """Check if path exists from src to dst using BFS."""
    if src == dst:
        return True
    visited: dict[int, bool] = {src: True}
    queue: list[int] = [src]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head = head + 1
        if node in adj:
            neighbors: list[int] = adj[node]
            i: int = 0
            while i < len(neighbors):
                nb: int = neighbors[i]
                if nb == dst:
                    return True
                if nb not in visited:
                    visited[nb] = True
                    queue.append(nb)
                i = i + 1
    return False


def shortest_path_length(adj: dict[int, list[int]], src: int, dst: int) -> int:
    """Shortest path length using BFS. Returns -1 if no path."""
    if src == dst:
        return 0
    visited: dict[int, bool] = {src: True}
    dist_map: dict[int, int] = {src: 0}
    queue: list[int] = [src]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head = head + 1
        current_dist: int = dist_map[node]
        if node in adj:
            neighbors: list[int] = adj[node]
            i: int = 0
            while i < len(neighbors):
                nb: int = neighbors[i]
                if nb not in visited:
                    visited[nb] = True
                    dist_map[nb] = current_dist + 1
                    if nb == dst:
                        return current_dist + 1
                    queue.append(nb)
                i = i + 1
    return -1


def node_degrees(adj: dict[int, list[int]], nodes: list[int]) -> list[int]:
    """Compute out-degree for each node."""
    result: list[int] = []
    i: int = 0
    while i < len(nodes):
        node: int = nodes[i]
        if node in adj:
            result.append(len(adj[node]))
        else:
            result.append(0)
        i = i + 1
    return result


def test_module() -> bool:
    """Test all graph traversal functions."""
    ok: bool = True

    adj: dict[int, list[int]] = {0: [1, 2], 1: [3], 2: [3], 3: []}
    bfs: list[int] = bfs_order(adj, 0)
    if bfs != [0, 1, 2, 3]:
        ok = False

    rc: int = count_reachable(adj, 0)
    if rc != 4:
        ok = False
    rc2: int = count_reachable(adj, 3)
    if rc2 != 1:
        ok = False

    if not has_path(adj, 0, 3):
        ok = False

    adj2: dict[int, list[int]] = {0: [1], 1: [0], 2: [3], 3: [2]}
    if has_path(adj2, 0, 2):
        ok = False

    sp: int = shortest_path_length(adj, 0, 3)
    if sp != 2:
        ok = False

    sp2: int = shortest_path_length(adj2, 0, 2)
    if sp2 != -1:
        ok = False

    degs: list[int] = node_degrees(adj, [0, 1, 2, 3])
    if degs != [2, 1, 1, 0]:
        ok = False

    return ok
