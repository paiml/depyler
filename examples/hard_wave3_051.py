"""Graph algorithms: Strongly connected components.
Tests: Tarjan-style SCC, condensation graph, DAG detection.
"""
from typing import Dict, List, Tuple

def scc_kosaraju_count(adj: Dict[int, List[int]], n: int) -> int:
    """Count SCCs using Kosaraju's algorithm."""
    visited: Dict[int, int] = {}
    order: List[int] = []
    i: int = 0
    while i < n:
        if i not in visited:
            stack: List[int] = [i]
            while len(stack) > 0:
                node: int = stack[len(stack) - 1]
                if node not in visited:
                    visited[node] = 1
                    if node in adj:
                        for nb in adj[node]:
                            if nb not in visited:
                                stack.append(nb)
                else:
                    stack.pop()
                    order.append(node)
        i += 1
    rev_adj: Dict[int, List[int]] = {}
    i = 0
    while i < n:
        rev_adj[i] = []
        i += 1
    i = 0
    while i < n:
        if i in adj:
            for nb in adj[i]:
                rev_adj[nb].append(i)
        i += 1
    visited2: Dict[int, int] = {}
    scc_count: int = 0
    idx: int = len(order) - 1
    while idx >= 0:
        node: int = order[idx]
        if node not in visited2:
            scc_count += 1
            stack2: List[int] = [node]
            while len(stack2) > 0:
                nd: int = stack2[len(stack2) - 1]
                stack2.pop()
                if nd not in visited2:
                    visited2[nd] = 1
                    for nb in rev_adj[nd]:
                        if nb not in visited2:
                            stack2.append(nb)
        idx -= 1
    return scc_count

def test_scc() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1]
    adj[1] = [2]
    adj[2] = [0, 3]
    adj[3] = []
    sc: int = scc_kosaraju_count(adj, 4)
    if sc != 2:
        ok = False
    return ok
