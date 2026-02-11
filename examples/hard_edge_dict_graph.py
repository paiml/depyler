"""Graph as adjacency dict with BFS and DFS."""


def build_graph(num_nodes: int, edges: list[list[int]]) -> dict[int, list[int]]:
    """Build adjacency list from edge list."""
    graph: dict[int, list[int]] = {}
    i: int = 0
    while i < num_nodes:
        graph[i] = []
        i = i + 1
    j: int = 0
    while j < len(edges):
        edge: list[int] = edges[j]
        src: int = edge[0]
        dst: int = edge[1]
        graph[src].append(dst)
        j = j + 1
    return graph


def bfs_order(graph: dict[int, list[int]], start: int) -> list[int]:
    """BFS traversal order from start node."""
    visited: dict[int, int] = {}
    queue_list: list[int] = [start]
    visited[start] = 1
    order: list[int] = []
    while len(queue_list) > 0:
        node: int = queue_list[0]
        new_queue: list[int] = []
        qi: int = 1
        while qi < len(queue_list):
            new_queue.append(queue_list[qi])
            qi = qi + 1
        queue_list = new_queue
        order.append(node)
        neighbors: list[int] = graph[node]
        ni: int = 0
        while ni < len(neighbors):
            nb: int = neighbors[ni]
            if nb not in visited:
                visited[nb] = 1
                queue_list.append(nb)
            ni = ni + 1
    return order


def dfs_order(graph: dict[int, list[int]], start: int) -> list[int]:
    """DFS traversal order from start node using explicit stack."""
    visited: dict[int, int] = {}
    stack: list[int] = [start]
    order: list[int] = []
    while len(stack) > 0:
        last_idx: int = len(stack) - 1
        node: int = stack[last_idx]
        stack.pop()
        if node in visited:
            continue
        visited[node] = 1
        order.append(node)
        neighbors: list[int] = graph[node]
        ni: int = len(neighbors) - 1
        while ni >= 0:
            nb: int = neighbors[ni]
            if nb not in visited:
                stack.append(nb)
            ni = ni - 1
    return order


def has_path(graph: dict[int, list[int]], src: int, dst: int) -> int:
    """Return 1 if path exists from src to dst."""
    if src == dst:
        return 1
    visited: dict[int, int] = {}
    stack: list[int] = [src]
    while len(stack) > 0:
        last_idx: int = len(stack) - 1
        node: int = stack[last_idx]
        stack.pop()
        if node == dst:
            return 1
        if node in visited:
            continue
        visited[node] = 1
        neighbors: list[int] = graph[node]
        ni: int = 0
        while ni < len(neighbors):
            nb: int = neighbors[ni]
            if nb not in visited:
                stack.append(nb)
            ni = ni + 1
    return 0


def shortest_path_length(graph: dict[int, list[int]], src: int, dst: int) -> int:
    """BFS shortest path length. Returns -1 if no path."""
    if src == dst:
        return 0
    visited: dict[int, int] = {}
    visited[src] = 1
    queue_list: list[int] = [src]
    dist: dict[int, int] = {}
    dist[src] = 0
    while len(queue_list) > 0:
        node: int = queue_list[0]
        new_queue: list[int] = []
        qi: int = 1
        while qi < len(queue_list):
            new_queue.append(queue_list[qi])
            qi = qi + 1
        queue_list = new_queue
        d: int = dist[node]
        neighbors: list[int] = graph[node]
        ni: int = 0
        while ni < len(neighbors):
            nb: int = neighbors[ni]
            if nb == dst:
                return d + 1
            if nb not in visited:
                visited[nb] = 1
                dist[nb] = d + 1
                queue_list.append(nb)
            ni = ni + 1
    return -1


def count_reachable(graph: dict[int, list[int]], start: int) -> int:
    """Count number of nodes reachable from start."""
    visited: dict[int, int] = {}
    stack: list[int] = [start]
    while len(stack) > 0:
        last_idx: int = len(stack) - 1
        node: int = stack[last_idx]
        stack.pop()
        if node in visited:
            continue
        visited[node] = 1
        neighbors: list[int] = graph[node]
        ni: int = 0
        while ni < len(neighbors):
            nb: int = neighbors[ni]
            if nb not in visited:
                stack.append(nb)
            ni = ni + 1
    count: int = 0
    for vk in visited:
        count = count + 1
    return count


def test_module() -> int:
    """Test all graph functions."""
    passed: int = 0
    edges: list[list[int]] = [[0, 1], [0, 2], [1, 3], [2, 3], [3, 4]]
    g: dict[int, list[int]] = build_graph(5, edges)
    bfs: list[int] = bfs_order(g, 0)
    if bfs[0] == 0:
        passed = passed + 1
    if len(bfs) == 5:
        passed = passed + 1
    dfs: list[int] = dfs_order(g, 0)
    if dfs[0] == 0:
        passed = passed + 1
    if len(dfs) == 5:
        passed = passed + 1
    hp: int = has_path(g, 0, 4)
    if hp == 1:
        passed = passed + 1
    hp2: int = has_path(g, 4, 0)
    if hp2 == 0:
        passed = passed + 1
    sp: int = shortest_path_length(g, 0, 4)
    if sp == 3:
        passed = passed + 1
    sp2: int = shortest_path_length(g, 0, 0)
    if sp2 == 0:
        passed = passed + 1
    cr: int = count_reachable(g, 0)
    if cr == 5:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
