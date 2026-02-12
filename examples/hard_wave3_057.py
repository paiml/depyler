"""Graph algorithms: Graph isomorphism primitives.
Tests: degree sequence comparison, adjacency verification.
"""
from typing import Dict, List, Tuple

def degree_sequence(adj: Dict[int, List[int]], n: int) -> List[int]:
    """Get sorted degree sequence."""
    degs: List[int] = []
    i: int = 0
    while i < n:
        if i in adj:
            degs.append(len(adj[i]))
        else:
            degs.append(0)
        i += 1
    nd: int = len(degs)
    i = 0
    while i < nd:
        j: int = 0
        lim: int = nd - i - 1
        while j < lim:
            if degs[j] > degs[j + 1]:
                temp: int = degs[j]
                degs[j] = degs[j + 1]
                degs[j + 1] = temp
            j += 1
        i += 1
    return degs

def same_degree_sequence(adj1: Dict[int, List[int]], n1: int,
                         adj2: Dict[int, List[int]], n2: int) -> bool:
    """Check if two graphs have same degree sequence."""
    if n1 != n2:
        return False
    ds1: List[int] = degree_sequence(adj1, n1)
    ds2: List[int] = degree_sequence(adj2, n2)
    i: int = 0
    while i < n1:
        if ds1[i] != ds2[i]:
            return False
        i += 1
    return True

def edge_count(adj: Dict[int, List[int]], n: int) -> int:
    """Count edges in undirected graph."""
    total: int = 0
    i: int = 0
    while i < n:
        if i in adj:
            total = total + len(adj[i])
        i += 1
    return total // 2

def test_isomorphism() -> bool:
    ok: bool = True
    adj1: Dict[int, List[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    adj2: Dict[int, List[int]] = {0: [1, 2], 1: [0, 2], 2: [0, 1]}
    if not same_degree_sequence(adj1, 3, adj2, 3):
        ok = False
    return ok
