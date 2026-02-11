"""BFS traversal and shortest path in unweighted graph."""


def bfs_traversal(adj: list[list[int]], start: int, n: int) -> list[int]:
    """BFS traversal from start node using queue (list)."""
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    order: list[int] = []
    queue: list[int] = [start]
    visited[start] = 1
    front: int = 0
    while front < len(queue):
        node: int = queue[front]
        front = front + 1
        order.append(node)
        j: int = 0
        while j < len(adj[node]):
            nb: int = adj[node][j]
            if visited[nb] == 0:
                visited[nb] = 1
                queue.append(nb)
            j = j + 1
    return order


def shortest_path_length(adj: list[list[int]], src: int, dst: int, n: int) -> int:
    """Find shortest path length from src to dst. Returns -1 if no path."""
    if src == dst:
        return 0
    visited: list[int] = []
    dist: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        dist.append(-1)
        i = i + 1
    queue: list[int] = [src]
    visited[src] = 1
    dist[src] = 0
    front: int = 0
    while front < len(queue):
        node: int = queue[front]
        front = front + 1
        j: int = 0
        while j < len(adj[node]):
            nb: int = adj[node][j]
            if visited[nb] == 0:
                visited[nb] = 1
                dist[nb] = dist[node] + 1
                if nb == dst:
                    return dist[nb]
                queue.append(nb)
            j = j + 1
    return -1


def bfs_level_count(adj: list[list[int]], start: int, n: int) -> int:
    """Count number of BFS levels (depth of BFS tree + 1)."""
    visited: list[int] = []
    dist: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        dist.append(0)
        i = i + 1
    queue: list[int] = [start]
    visited[start] = 1
    max_dist: int = 0
    front: int = 0
    while front < len(queue):
        node: int = queue[front]
        front = front + 1
        j: int = 0
        while j < len(adj[node]):
            nb: int = adj[node][j]
            if visited[nb] == 0:
                visited[nb] = 1
                dist[nb] = dist[node] + 1
                if dist[nb] > max_dist:
                    max_dist = dist[nb]
                queue.append(nb)
            j = j + 1
    return max_dist + 1


def test_module() -> int:
    passed: int = 0

    # Graph: 0-1, 0-2, 1-3, 2-3
    adj1: list[list[int]] = [[1, 2], [0, 3], [0, 3], [1, 2]]
    bfs1: list[int] = bfs_traversal(adj1, 0, 4)
    if len(bfs1) == 4 and bfs1[0] == 0:
        passed = passed + 1

    sp1: int = shortest_path_length(adj1, 0, 3, 4)
    if sp1 == 2:
        passed = passed + 1

    sp2: int = shortest_path_length(adj1, 0, 0, 4)
    if sp2 == 0:
        passed = passed + 1

    # Disconnected
    adj2: list[list[int]] = [[1], [0], [3], [2]]
    sp3: int = shortest_path_length(adj2, 0, 2, 4)
    if sp3 == -1:
        passed = passed + 1

    lv1: int = bfs_level_count(adj1, 0, 4)
    if lv1 == 3:
        passed = passed + 1

    # Linear graph: 0-1-2-3
    adj3: list[list[int]] = [[1], [0, 2], [1, 3], [2]]
    sp4: int = shortest_path_length(adj3, 0, 3, 4)
    if sp4 == 3:
        passed = passed + 1

    lv2: int = bfs_level_count(adj3, 0, 4)
    if lv2 == 4:
        passed = passed + 1

    return passed
