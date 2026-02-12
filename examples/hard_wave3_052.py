"""Graph algorithms: Bridge counting in undirected graphs.
Tests: DFS traversal, low-link computation, connectivity analysis.
"""
from typing import Dict, List, Tuple


def count_bridges_simple(adj: Dict[int, List[int]], n: int) -> int:
    """Count bridges by testing edge removal."""
    total_edges: int = 0
    edge_list: List[Tuple[int, int]] = []
    i: int = 0
    while i < n:
        if i in adj:
            for nb in adj[i]:
                if nb > i:
                    edge_list.append((i, nb))
                    total_edges += 1
        i += 1
    bridges: int = 0
    for edge in edge_list:
        u: int = edge[0]
        v: int = edge[1]
        visited: Dict[int, int] = {}
        visited[u] = 1
        queue: List[int] = [u]
        head: int = 0
        while head < len(queue):
            node: int = queue[head]
            head += 1
            if node in adj:
                for nb in adj[node]:
                    if nb not in visited:
                        if node == u and nb == v:
                            continue
                        if node == v and nb == u:
                            continue
                        visited[nb] = 1
                        queue.append(nb)
        if v not in visited:
            bridges += 1
    return bridges


def is_bridge(adj: Dict[int, List[int]], u: int, v: int, n: int) -> bool:
    """Check if edge (u,v) is a bridge."""
    visited: Dict[int, int] = {}
    visited[u] = 1
    queue: List[int] = [u]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            for nb in adj[node]:
                if nb not in visited:
                    if node == u and nb == v:
                        continue
                    if node == v and nb == u:
                        continue
                    visited[nb] = 1
                    queue.append(nb)
    return v not in visited


def count_connected_after_remove(adj: Dict[int, List[int]], n: int, remove_node: int) -> int:
    """Count connected components after removing a node."""
    visited: Dict[int, int] = {}
    visited[remove_node] = 1
    components: int = 0
    i: int = 0
    while i < n:
        if i not in visited:
            components += 1
            stack: List[int] = [i]
            while len(stack) > 0:
                node: int = stack[len(stack) - 1]
                stack.pop()
                if node not in visited:
                    visited[node] = 1
                    if node in adj:
                        for nb in adj[node]:
                            if nb not in visited:
                                stack.append(nb)
        i += 1
    return components


def test_bridges() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [0, 2]
    adj[2] = [0, 1, 3]
    adj[3] = [2]
    b: int = count_bridges_simple(adj, 4)
    if b != 1:
        ok = False
    if not is_bridge(adj, 2, 3, 4):
        ok = False
    return ok
