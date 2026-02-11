"""DFS traversal and connected components count using adjacency list."""


def dfs_iterative(adj: list[list[int]], start: int, n: int) -> list[int]:
    """DFS traversal from start node using iterative stack."""
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    order: list[int] = []
    stack: list[int] = [start]
    while len(stack) > 0:
        node: int = stack[len(stack) - 1]
        stack.pop()
        if visited[node] == 0:
            visited[node] = 1
            order.append(node)
            # Push neighbors in reverse order for standard DFS order
            j: int = len(adj[node]) - 1
            while j >= 0:
                neighbor: int = adj[node][j]
                if visited[neighbor] == 0:
                    stack.append(neighbor)
                j = j - 1
    return order


def count_connected_components(adj: list[list[int]], n: int) -> int:
    """Count connected components in an undirected graph."""
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    count: int = 0
    node: int = 0
    while node < n:
        if visited[node] == 0:
            count = count + 1
            # BFS/DFS to mark all reachable nodes
            stack: list[int] = [node]
            while len(stack) > 0:
                curr: int = stack[len(stack) - 1]
                stack.pop()
                if visited[curr] == 0:
                    visited[curr] = 1
                    j: int = 0
                    while j < len(adj[curr]):
                        nb: int = adj[curr][j]
                        if visited[nb] == 0:
                            stack.append(nb)
                        j = j + 1
        node = node + 1
    return count


def has_path(adj: list[list[int]], src: int, dst: int, n: int) -> int:
    """Check if path exists from src to dst. Returns 1 or 0."""
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    stack: list[int] = [src]
    while len(stack) > 0:
        curr: int = stack[len(stack) - 1]
        stack.pop()
        if curr == dst:
            return 1
        if visited[curr] == 0:
            visited[curr] = 1
            j: int = 0
            while j < len(adj[curr]):
                nb: int = adj[curr][j]
                if visited[nb] == 0:
                    stack.append(nb)
                j = j + 1
    return 0


def test_module() -> int:
    passed: int = 0

    # Graph: 0-1, 0-2, 1-3, 2-3
    adj1: list[list[int]] = [[1, 2], [0, 3], [0, 3], [1, 2]]
    dfs1: list[int] = dfs_iterative(adj1, 0, 4)
    if len(dfs1) == 4:
        passed = passed + 1

    if dfs1[0] == 0:
        passed = passed + 1

    # Two components: {0,1} and {2,3}
    adj2: list[list[int]] = [[1], [0], [3], [2]]
    cc: int = count_connected_components(adj2, 4)
    if cc == 2:
        passed = passed + 1

    # Single node
    adj3: list[list[int]] = [[]]
    cc2: int = count_connected_components(adj3, 1)
    if cc2 == 1:
        passed = passed + 1

    if has_path(adj1, 0, 3, 4) == 1:
        passed = passed + 1

    if has_path(adj2, 0, 2, 4) == 0:
        passed = passed + 1

    # All disconnected
    adj4: list[list[int]] = [[], [], []]
    cc3: int = count_connected_components(adj4, 3)
    if cc3 == 3:
        passed = passed + 1

    return passed
