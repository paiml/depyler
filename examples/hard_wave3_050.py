"""Graph algorithms: Floyd-Warshall all-pairs shortest path.

Tests: triple-nested loop, distance matrix update, path detection.
"""
from typing import Dict, List, Tuple

def floyd_warshall(n: int, edges: List[Tuple[int, int, int]]) -> List[List[int]]:
    """All-pairs shortest path using Floyd-Warshall."""
    big: int = 999999
    dist: List[List[int]] = []
    i: int = 0
    while i < n:
        row: List[int] = []
        j: int = 0
        while j < n:
            if i == j:
                row.append(0)
            else:
                row.append(big)
            j += 1
        dist.append(row)
        i += 1
    for edge in edges:
        u: int = edge[0]
        v: int = edge[1]
        w: int = edge[2]
        dist[u][v] = w
    k: int = 0
    while k < n:
        i = 0
        while i < n:
            j: int = 0
            while j < n:
                through_k: int = dist[i][k] + dist[k][j]
                if through_k < dist[i][j]:
                    dist[i][j] = through_k
                j += 1
            i += 1
        k += 1
    return dist

def diameter(n: int, edges: List[Tuple[int, int, int]]) -> int:
    """Graph diameter (longest shortest path)."""
    dist: List[List[int]] = floyd_warshall(n, edges)
    big: int = 999999
    mx: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if dist[i][j] < big and dist[i][j] > mx:
                mx = dist[i][j]
            j += 1
        i += 1
    return mx

def test_floyd() -> bool:
    ok: bool = True
    edges: List[Tuple[int, int, int]] = [(0, 1, 3), (1, 2, 1), (0, 2, 10)]
    dist: List[List[int]] = floyd_warshall(3, edges)
    if dist[0][2] != 4:
        ok = False
    d: int = diameter(3, edges)
    if d != 4:
        ok = False
    return ok
