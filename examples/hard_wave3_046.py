"""Graph algorithms: Minimum spanning tree (Kruskal's).

Tests: edge sorting, union-find integration, MST weight.
"""
from typing import Dict, List, Tuple

def kruskal_mst_weight(n: int, edges: List[Tuple[int, int, int]]) -> int:
    """Compute MST weight using Kruskal's algorithm."""
    sorted_edges: List[Tuple[int, int, int]] = []
    for e in edges:
        sorted_edges.append(e)
    ne: int = len(sorted_edges)
    i: int = 0
    while i < ne:
        j: int = 0
        lim: int = ne - i - 1
        while j < lim:
            if sorted_edges[j][2] > sorted_edges[j + 1][2]:
                temp: Tuple[int, int, int] = sorted_edges[j]
                sorted_edges[j] = sorted_edges[j + 1]
                sorted_edges[j + 1] = temp
            j += 1
        i += 1
    parent: List[int] = []
    rnk: List[int] = []
    i = 0
    while i < n:
        parent.append(i)
        rnk.append(0)
        i += 1
    total: int = 0
    count: int = 0
    for e in sorted_edges:
        u: int = e[0]
        v: int = e[1]
        w: int = e[2]
        ru: int = u
        while parent[ru] != ru:
            parent[ru] = parent[parent[ru]]
            ru = parent[ru]
        rv: int = v
        while parent[rv] != rv:
            parent[rv] = parent[parent[rv]]
            rv = parent[rv]
        if ru != rv:
            if rnk[ru] < rnk[rv]:
                parent[ru] = rv
            elif rnk[ru] > rnk[rv]:
                parent[rv] = ru
            else:
                parent[rv] = ru
                rnk[ru] = rnk[ru] + 1
            total = total + w
            count += 1
        expected: int = n - 1
        if count == expected:
            break
    return total

def test_kruskal() -> bool:
    ok: bool = True
    edges: List[Tuple[int, int, int]] = [(0, 1, 4), (0, 2, 1), (1, 2, 2), (1, 3, 5), (2, 3, 8)]
    w: int = kruskal_mst_weight(4, edges)
    if w != 8:
        ok = False
    return ok
