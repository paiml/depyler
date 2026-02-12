"""Graph algorithms: Directed acyclic graph operations.
Tests: longest path in DAG, critical path, topological processing.
"""
from typing import Dict, List, Tuple

def longest_path_dag(adj: Dict[int, List[Tuple[int, int]]], n: int, start: int) -> List[int]:
    """Longest path in DAG from start using topological order."""
    deg: List[int] = []
    i: int = 0
    while i < n:
        deg.append(0)
        i += 1
    i = 0
    while i < n:
        if i in adj:
            for edge in adj[i]:
                deg[edge[0]] = deg[edge[0]] + 1
        i += 1
    queue: List[int] = []
    i = 0
    while i < n:
        if deg[i] == 0:
            queue.append(i)
        i += 1
    order: List[int] = []
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        order.append(node)
        if node in adj:
            for edge in adj[node]:
                v: int = edge[0]
                deg[v] = deg[v] - 1
                if deg[v] == 0:
                    queue.append(v)
    big: int = -999999
    dist: List[int] = []
    i = 0
    while i < n:
        dist.append(big)
        i += 1
    dist[start] = 0
    for node in order:
        if dist[node] != big and node in adj:
            for edge in adj[node]:
                v = edge[0]
                w: int = edge[1]
                nd: int = dist[node] + w
                if nd > dist[v]:
                    dist[v] = nd
    return dist

def test_longest_path() -> bool:
    ok: bool = True
    adj: Dict[int, List[Tuple[int, int]]] = {}
    adj[0] = [(1, 3), (2, 6)]
    adj[1] = [(3, 4)]
    adj[2] = [(3, 2)]
    adj[3] = []
    dist: List[int] = longest_path_dag(adj, 4, 0)
    if dist[3] != 8:
        ok = False
    return ok
