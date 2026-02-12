"""Graph algorithms: Shortest path (Dijkstra-like with array).

Tests: priority simulation with arrays, relaxation, path tracking.
"""
from typing import Dict, List, Tuple

def dijkstra_array(adj: Dict[int, List[Tuple[int, int]]], start: int, n: int) -> List[int]:
    """Dijkstra shortest path using array-based min search."""
    big: int = 999999999
    dist: List[int] = []
    visited: List[int] = []
    i: int = 0
    while i < n:
        dist.append(big)
        visited.append(0)
        i += 1
    dist[start] = 0
    step: int = 0
    while step < n:
        u: int = -1
        u_dist: int = big
        j: int = 0
        while j < n:
            if visited[j] == 0 and dist[j] < u_dist:
                u = j
                u_dist = dist[j]
            j += 1
        if u < 0:
            break
        visited[u] = 1
        if u in adj:
            for edge in adj[u]:
                v: int = edge[0]
                w: int = edge[1]
                nd: int = dist[u] + w
                if nd < dist[v]:
                    dist[v] = nd
        step += 1
    return dist

def shortest_path_length(adj: Dict[int, List[Tuple[int, int]]], start: int, end: int, n: int) -> int:
    """Get shortest path length between two nodes."""
    dists: List[int] = dijkstra_array(adj, start, n)
    return dists[end]

def test_dijkstra() -> bool:
    ok: bool = True
    adj: Dict[int, List[Tuple[int, int]]] = {}
    adj[0] = [(1, 4), (2, 1)]
    adj[1] = [(3, 1)]
    adj[2] = [(1, 2), (3, 5)]
    adj[3] = []
    dists: List[int] = dijkstra_array(adj, 0, 4)
    if dists[3] != 4:
        ok = False
    return ok
