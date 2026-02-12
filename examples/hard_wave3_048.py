"""Graph algorithms: Degree sequences and graph properties.

Tests: degree computation, density, adjacency operations.
"""
from typing import Dict, List, Tuple

def compute_degrees(adj: Dict[int, List[int]], n: int) -> List[int]:
    """Compute degree of each node."""
    deg: List[int] = []
    i: int = 0
    while i < n:
        if i in adj:
            deg.append(len(adj[i]))
        else:
            deg.append(0)
        i += 1
    return deg

def max_degree(adj: Dict[int, List[int]], n: int) -> int:
    """Find maximum degree in graph."""
    degs: List[int] = compute_degrees(adj, n)
    mx: int = 0
    for d in degs:
        if d > mx:
            mx = d
    return mx

def total_edges(adj: Dict[int, List[int]], n: int) -> int:
    """Count total edges (undirected graph, each edge counted once)."""
    total: int = 0
    degs: List[int] = compute_degrees(adj, n)
    for d in degs:
        total = total + d
    return total // 2

def is_complete(adj: Dict[int, List[int]], n: int) -> bool:
    """Check if graph is complete."""
    expected: int = n - 1
    i: int = 0
    while i < n:
        if i not in adj:
            return False
        if len(adj[i]) != expected:
            return False
        i += 1
    return True

def graph_density(adj: Dict[int, List[int]], n: int) -> float:
    """Compute graph density."""
    if n <= 1:
        return 0.0
    e: int = total_edges(adj, n)
    possible: int = n * (n - 1) // 2
    return float(e) / float(possible)

def test_degrees() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [0, 2]
    adj[2] = [0, 1]
    md: int = max_degree(adj, 3)
    if md != 2:
        ok = False
    te: int = total_edges(adj, 3)
    if te != 3:
        ok = False
    if not is_complete(adj, 3):
        ok = False
    return ok
