"""Graph algorithms: Graph coloring and bipartiteness.

Tests: two-coloring check, chromatic bound estimation.
"""
from typing import Dict, List, Tuple

def is_bipartite(adj: Dict[int, List[int]], n: int) -> bool:
    """Check if graph is bipartite using BFS coloring."""
    color: Dict[int, int] = {}
    i: int = 0
    while i < n:
        if i not in color:
            color[i] = 0
            queue: List[int] = [i]
            head: int = 0
            while head < len(queue):
                node: int = queue[head]
                head += 1
                if node in adj:
                    for nb in adj[node]:
                        if nb not in color:
                            color[nb] = 1 - color[node]
                            queue.append(nb)
                        elif color[nb] == color[node]:
                            return False
        i += 1
    return True

def greedy_coloring(adj: Dict[int, List[int]], n: int) -> List[int]:
    """Greedy graph coloring, returns color assignment."""
    colors: List[int] = []
    i: int = 0
    while i < n:
        colors.append(-1)
        i += 1
    i = 0
    while i < n:
        used: Dict[int, int] = {}
        if i in adj:
            for nb in adj[i]:
                if colors[nb] >= 0:
                    used[colors[nb]] = 1
        c: int = 0
        while c in used:
            c += 1
        colors[i] = c
        i += 1
    return colors

def chromatic_upper_bound(adj: Dict[int, List[int]], n: int) -> int:
    """Estimate chromatic number upper bound."""
    colors: List[int] = greedy_coloring(adj, n)
    max_color: int = 0
    for c in colors:
        if c > max_color:
            max_color = c
    return max_color + 1

def test_coloring() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 3]
    adj[1] = [0, 2]
    adj[2] = [1, 3]
    adj[3] = [2, 0]
    if not is_bipartite(adj, 4):
        ok = False
    return ok
