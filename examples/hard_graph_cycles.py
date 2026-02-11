"""Cycle detection in directed graph using DFS coloring.

Tests: detect_cycle in various graph topologies, acyclic/cyclic.
"""


def has_cycle(adj: list[list[int]], num_nodes: int) -> int:
    """Detect cycle in directed graph. Returns 1 if cycle exists, 0 otherwise.
    
    Uses coloring: 0=white(unvisited), 1=gray(in progress), 2=black(done).
    """
    color: list[int] = []
    i: int = 0
    while i < num_nodes:
        color.append(0)
        i = i + 1
    node: int = 0
    while node < num_nodes:
        if color[node] == 0:
            stack: list[int] = [node]
            color[node] = 1
            while len(stack) > 0:
                u: int = stack[len(stack) - 1]
                found_white: int = 0
                j: int = 0
                while j < len(adj[u]):
                    v: int = adj[u][j]
                    if color[v] == 1:
                        return 1
                    if color[v] == 0 and found_white == 0:
                        color[v] = 1
                        stack.append(v)
                        found_white = 1
                    j = j + 1
                if found_white == 0:
                    color[u] = 2
                    stack = stack[:len(stack) - 1]
        node = node + 1
    return 0


def count_edges(adj: list[list[int]]) -> int:
    """Count total edges in adjacency list."""
    total: int = 0
    i: int = 0
    while i < len(adj):
        total = total + len(adj[i])
        i = i + 1
    return total


def has_self_loop(adj: list[list[int]]) -> int:
    """Check if any node has an edge to itself."""
    i: int = 0
    while i < len(adj):
        j: int = 0
        while j < len(adj[i]):
            if adj[i][j] == i:
                return 1
            j = j + 1
        i = i + 1
    return 0


def test_module() -> int:
    """Test cycle detection in directed graphs."""
    ok: int = 0

    # Graph with cycle: 0->1->2->0
    g1: list[list[int]] = [[1], [2], [0]]
    if has_cycle(g1, 3) == 1:
        ok = ok + 1

    # DAG: 0->1->2
    g2: list[list[int]] = [[1], [2], []]
    if has_cycle(g2, 3) == 0:
        ok = ok + 1

    # Self-loop: 0->0
    g3: list[list[int]] = [[0]]
    if has_cycle(g3, 1) == 1:
        ok = ok + 1

    # Disconnected with no cycle
    g4: list[list[int]] = [[1], [], [3], []]
    if has_cycle(g4, 4) == 0:
        ok = ok + 1

    if count_edges(g1) == 3:
        ok = ok + 1

    if has_self_loop(g3) == 1:
        ok = ok + 1

    if has_self_loop(g2) == 0:
        ok = ok + 1

    return ok
