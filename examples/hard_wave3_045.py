"""Graph algorithms: Union-Find (Disjoint Set Union).

Tests: path compression, union by rank, component tracking.
"""
from typing import Dict, List, Tuple

def uf_init(n: int) -> Tuple[List[int], List[int]]:
    """Initialize union-find with n elements."""
    parent: List[int] = []
    rank: List[int] = []
    i: int = 0
    while i < n:
        parent.append(i)
        rank.append(0)
        i += 1
    return (parent, rank)

def uf_find(parent: List[int], x: int) -> int:
    """Find root with path compression."""
    while parent[x] != x:
        parent[x] = parent[parent[x]]
        x = parent[x]
    return x

def uf_union(parent: List[int], rnk: List[int], a: int, b: int) -> bool:
    """Union two sets by rank. Returns True if merged."""
    ra: int = uf_find(parent, a)
    rb: int = uf_find(parent, b)
    if ra == rb:
        return False
    if rnk[ra] < rnk[rb]:
        parent[ra] = rb
    elif rnk[ra] > rnk[rb]:
        parent[rb] = ra
    else:
        parent[rb] = ra
        rnk[ra] = rnk[ra] + 1
    return True

def count_uf_components(parent: List[int]) -> int:
    """Count number of disjoint components."""
    n: int = len(parent)
    roots: Dict[int, int] = {}
    i: int = 0
    while i < n:
        r: int = uf_find(parent, i)
        roots[r] = 1
        i += 1
    count: int = 0
    for k in roots:
        count += 1
    return count

def test_uf() -> bool:
    ok: bool = True
    init: Tuple[List[int], List[int]] = uf_init(5)
    parent: List[int] = init[0]
    rnk: List[int] = init[1]
    uf_union(parent, rnk, 0, 1)
    uf_union(parent, rnk, 2, 3)
    cc: int = count_uf_components(parent)
    if cc != 3:
        ok = False
    uf_union(parent, rnk, 1, 3)
    cc2: int = count_uf_components(parent)
    if cc2 != 2:
        ok = False
    return ok
