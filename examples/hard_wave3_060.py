"""Graph algorithms: Transitive closure and reachability matrix.
Tests: Warshall's algorithm, reachability queries, path counting.
"""
from typing import Dict, List, Tuple

def transitive_closure(adj: Dict[int, List[int]], n: int) -> List[List[int]]:
    """Compute transitive closure using Warshall's algorithm."""
    reach: List[List[int]] = []
    i: int = 0
    while i < n:
        row: List[int] = []
        j: int = 0
        while j < n:
            if i == j:
                row.append(1)
            else:
                row.append(0)
            j += 1
        reach.append(row)
        i += 1
    i = 0
    while i < n:
        if i in adj:
            for nb in adj[i]:
                reach[i][nb] = 1
        i += 1
    k: int = 0
    while k < n:
        i = 0
        while i < n:
            j: int = 0
            while j < n:
                if reach[i][k] == 1 and reach[k][j] == 1:
                    reach[i][j] = 1
                j += 1
            i += 1
        k += 1
    return reach

def count_reachable_pairs(adj: Dict[int, List[int]], n: int) -> int:
    """Count total reachable pairs."""
    tc: List[List[int]] = transitive_closure(adj, n)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if i != j and tc[i][j] == 1:
                count += 1
            j += 1
        i += 1
    return count

def test_closure() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1]
    adj[1] = [2]
    adj[2] = []
    tc: List[List[int]] = transitive_closure(adj, 3)
    if tc[0][2] != 1:
        ok = False
    if tc[2][0] != 0:
        ok = False
    return ok
