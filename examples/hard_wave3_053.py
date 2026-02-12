"""Graph algorithms: Network flow (max flow).
Tests: augmenting paths, residual graph, capacity tracking.
"""
from typing import Dict, List, Tuple

def max_flow_simple(capacity: List[List[int]], source: int, sink: int, n: int) -> int:
    """Simple Edmonds-Karp max flow algorithm."""
    residual: List[List[int]] = []
    i: int = 0
    while i < n:
        row: List[int] = []
        j: int = 0
        while j < n:
            row.append(capacity[i][j])
            j += 1
        residual.append(row)
        i += 1
    total_flow: int = 0
    iteration: int = 0
    while iteration < 1000:
        parent: List[int] = []
        i = 0
        while i < n:
            parent.append(-1)
            i += 1
        visited: List[int] = []
        i = 0
        while i < n:
            visited.append(0)
            i += 1
        queue: List[int] = [source]
        visited[source] = 1
        head: int = 0
        found: bool = False
        while head < len(queue) and not found:
            u: int = queue[head]
            head += 1
            v: int = 0
            while v < n:
                if visited[v] == 0 and residual[u][v] > 0:
                    parent[v] = u
                    visited[v] = 1
                    if v == sink:
                        found = True
                        break
                    queue.append(v)
                v += 1
        if not found:
            break
        path_flow: int = 999999
        v = sink
        while v != source:
            u = parent[v]
            if residual[u][v] < path_flow:
                path_flow = residual[u][v]
            v = u
        v = sink
        while v != source:
            u = parent[v]
            residual[u][v] = residual[u][v] - path_flow
            residual[v][u] = residual[v][u] + path_flow
            v = u
        total_flow = total_flow + path_flow
        iteration += 1
    return total_flow

def test_flow() -> bool:
    ok: bool = True
    cap: List[List[int]] = [[0, 10, 10, 0], [0, 0, 0, 10], [0, 0, 0, 10], [0, 0, 0, 0]]
    flow: int = max_flow_simple(cap, 0, 3, 4)
    if flow != 20:
        ok = False
    return ok
