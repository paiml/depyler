"""Graph algorithms: Cycle detection in undirected graphs.
Tests: DFS-based cycle detection, back edge finding.
"""
from typing import Dict, List, Tuple

def has_cycle_undirected(adj: Dict[int, List[int]], n: int) -> bool:
    """Detect cycle in undirected graph using DFS."""
    visited: Dict[int, int] = {}
    i: int = 0
    while i < n:
        if i not in visited:
            stack: List[Tuple[int, int]] = [(i, -1)]
            while len(stack) > 0:
                top: Tuple[int, int] = stack[len(stack) - 1]
                stack.pop()
                node: int = top[0]
                par: int = top[1]
                if node in visited:
                    continue
                visited[node] = 1
                if node in adj:
                    for nb in adj[node]:
                        if nb not in visited:
                            stack.append((nb, node))
                        elif nb != par:
                            return True
        i += 1
    return False

def count_back_edges(adj: Dict[int, List[int]], n: int) -> int:
    """Count back edges (crude cycle indicator)."""
    visited: Dict[int, int] = {}
    back_edges: int = 0
    i: int = 0
    while i < n:
        if i not in visited:
            stack: List[Tuple[int, int]] = [(i, -1)]
            while len(stack) > 0:
                top: Tuple[int, int] = stack[len(stack) - 1]
                stack.pop()
                node: int = top[0]
                par: int = top[1]
                if node in visited:
                    back_edges += 1
                    continue
                visited[node] = 1
                if node in adj:
                    for nb in adj[node]:
                        if nb != par:
                            stack.append((nb, node))
        i += 1
    return back_edges

def test_cycle() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [0, 2]
    adj[2] = [0, 1]
    if not has_cycle_undirected(adj, 3):
        ok = False
    adj2: Dict[int, List[int]] = {}
    adj2[0] = [1]
    adj2[1] = [0]
    if has_cycle_undirected(adj2, 2):
        ok = False
    return ok
