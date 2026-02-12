"""Graph algorithms: Topological sort.

Tests: in-degree computation, Kahn's algorithm, DAG validation.
"""
from typing import Dict, List, Tuple

def compute_in_degree(adj: Dict[int, List[int]], n: int) -> List[int]:
    """Compute in-degree for each node."""
    deg: List[int] = []
    i: int = 0
    while i < n:
        deg.append(0)
        i += 1
    i = 0
    while i < n:
        if i in adj:
            for nb in adj[i]:
                deg[nb] = deg[nb] + 1
        i += 1
    return deg

def topological_sort(adj: Dict[int, List[int]], n: int) -> List[int]:
    """Kahn's algorithm for topological sort."""
    deg: List[int] = compute_in_degree(adj, n)
    queue: List[int] = []
    i: int = 0
    while i < n:
        if deg[i] == 0:
            queue.append(i)
        i += 1
    result: List[int] = []
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        result.append(node)
        if node in adj:
            for nb in adj[node]:
                deg[nb] = deg[nb] - 1
                if deg[nb] == 0:
                    queue.append(nb)
    return result

def is_dag(adj: Dict[int, List[int]], n: int) -> bool:
    """Check if directed graph is acyclic."""
    order: List[int] = topological_sort(adj, n)
    return len(order) == n

def test_topo() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [3]
    adj[2] = [3]
    adj[3] = []
    order: List[int] = topological_sort(adj, 4)
    if len(order) != 4:
        ok = False
    if not is_dag(adj, 4):
        ok = False
    return ok
