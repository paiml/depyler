"""Bipartite graph check using BFS-style coloring.

Tests: is bipartite, partition sizes, is tree, edge count.
"""


def is_bipartite(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Returns 1 if graph is bipartite (2-colorable)."""
    color: list[int] = []
    i: int = 0
    while i < n:
        color.append(-1)
        i = i + 1
    node: int = 0
    while node < n:
        if color[node] == -1:
            color[node] = 0
            queue: list[int] = [node]
            qi: int = 0
            while qi < len(queue):
                cur: int = queue[qi]
                e: int = len(edges_u)
                j: int = 0
                while j < e:
                    neighbor: int = -1
                    if edges_u[j] == cur:
                        neighbor = edges_v[j]
                    if edges_v[j] == cur:
                        neighbor = edges_u[j]
                    if neighbor >= 0 and neighbor != cur:
                        if color[neighbor] == -1:
                            color[neighbor] = 1 - color[cur]
                            queue.append(neighbor)
                        elif color[neighbor] == color[cur]:
                            return 0
                    j = j + 1
                qi = qi + 1
        node = node + 1
    return 1


def partition_sizes(n: int, edges_u: list[int], edges_v: list[int]) -> list[int]:
    """Returns [size_A, size_B] of bipartite partition. [0,0] if not bipartite."""
    color: list[int] = []
    i: int = 0
    while i < n:
        color.append(-1)
        i = i + 1
    node: int = 0
    while node < n:
        if color[node] == -1:
            color[node] = 0
            queue: list[int] = [node]
            qi: int = 0
            while qi < len(queue):
                cur: int = queue[qi]
                e: int = len(edges_u)
                j: int = 0
                while j < e:
                    neighbor: int = -1
                    if edges_u[j] == cur:
                        neighbor = edges_v[j]
                    if edges_v[j] == cur:
                        neighbor = edges_u[j]
                    if neighbor >= 0 and neighbor != cur:
                        if color[neighbor] == -1:
                            color[neighbor] = 1 - color[cur]
                            queue.append(neighbor)
                        elif color[neighbor] == color[cur]:
                            return [0, 0]
                    j = j + 1
                qi = qi + 1
        node = node + 1
    count_a: int = 0
    count_b: int = 0
    for c in color:
        if c == 0:
            count_a = count_a + 1
        else:
            count_b = count_b + 1
    return [count_a, count_b]


def edge_count(edges_u: list[int]) -> int:
    """Number of edges."""
    return len(edges_u)


def test_module() -> int:
    """Test bipartite check."""
    ok: int = 0
    eu: list[int] = [0, 0, 1, 1]
    ev: list[int] = [2, 3, 2, 3]
    if is_bipartite(4, eu, ev) == 1:
        ok = ok + 1
    eu2: list[int] = [0, 0, 1]
    ev2: list[int] = [1, 2, 2]
    if is_bipartite(3, eu2, ev2) == 0:
        ok = ok + 1
    ps: list[int] = partition_sizes(4, eu, ev)
    if ps[0] == 2 and ps[1] == 2:
        ok = ok + 1
    if edge_count(eu) == 4:
        ok = ok + 1
    if is_bipartite(1, [], []) == 1:
        ok = ok + 1
    return ok
