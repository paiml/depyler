"""Graph algorithms: Bellman-Ford shortest path.

Tests: negative weight handling, relaxation iterations, cycle detection.
"""
from typing import Dict, List, Tuple

def bellman_ford(n: int, edges: List[Tuple[int, int, int]], start: int) -> List[int]:
    """Bellman-Ford shortest path algorithm."""
    big: int = 999999999
    dist: List[int] = []
    i: int = 0
    while i < n:
        dist.append(big)
        i += 1
    dist[start] = 0
    iteration: int = 0
    nm1: int = n - 1
    while iteration < nm1:
        changed: bool = False
        for edge in edges:
            u: int = edge[0]
            v: int = edge[1]
            w: int = edge[2]
            if dist[u] < big:
                nd: int = dist[u] + w
                if nd < dist[v]:
                    dist[v] = nd
                    changed = True
        if not changed:
            break
        iteration += 1
    return dist

def has_negative_cycle(n: int, edges: List[Tuple[int, int, int]], start: int) -> bool:
    """Check if graph has negative weight cycle reachable from start."""
    dist: List[int] = bellman_ford(n, edges, start)
    big: int = 999999999
    for edge in edges:
        u: int = edge[0]
        v: int = edge[1]
        w: int = edge[2]
        if dist[u] < big:
            nd: int = dist[u] + w
            if nd < dist[v]:
                return True
    return False

def test_bellman_ford() -> bool:
    ok: bool = True
    edges: List[Tuple[int, int, int]] = [(0, 1, 4), (0, 2, 1), (2, 1, 2), (1, 3, 1)]
    dist: List[int] = bellman_ford(4, edges, 0)
    if dist[3] != 4:
        ok = False
    if has_negative_cycle(4, edges, 0):
        ok = False
    return ok
