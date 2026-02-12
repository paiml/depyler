"""Graph algorithms: DFS traversal and connected components.

Tests: stack-based DFS, visited tracking, component counting,
path detection, reachability analysis.
"""

from typing import Dict, List, Tuple


def dfs_reachable(adj: Dict[int, List[int]], start: int) -> List[int]:
    """DFS from start, return all reachable nodes."""
    visited: Dict[int, int] = {}
    stack: List[int] = [start]
    result: List[int] = []
    while len(stack) > 0:
        node: int = stack[len(stack) - 1]
        stack.pop()
        if node not in visited:
            visited[node] = 1
            result.append(node)
            if node in adj:
                for nb in adj[node]:
                    if nb not in visited:
                        stack.append(nb)
    return result


def count_components(n: int, edges: List[Tuple[int, int]]) -> int:
    """Count connected components in undirected graph."""
    adj: Dict[int, List[int]] = {}
    i: int = 0
    while i < n:
        adj[i] = []
        i += 1
    for edge in edges:
        u: int = edge[0]
        v: int = edge[1]
        adj[u].append(v)
        adj[v].append(u)
    visited: Dict[int, int] = {}
    components: int = 0
    i = 0
    while i < n:
        if i not in visited:
            components += 1
            stack: List[int] = [i]
            while len(stack) > 0:
                node: int = stack[len(stack) - 1]
                stack.pop()
                if node not in visited:
                    visited[node] = 1
                    for nb in adj[node]:
                        if nb not in visited:
                            stack.append(nb)
        i += 1
    return components


def dfs_path_exists(adj: Dict[int, List[int]], start: int, end: int) -> bool:
    """Check if path exists using DFS."""
    if start == end:
        return True
    visited: Dict[int, int] = {}
    stack: List[int] = [start]
    while len(stack) > 0:
        node: int = stack[len(stack) - 1]
        stack.pop()
        if node not in visited:
            visited[node] = 1
            if node in adj:
                for nb in adj[node]:
                    if nb == end:
                        return True
                    if nb not in visited:
                        stack.append(nb)
    return False


def test_dfs() -> bool:
    """Test DFS functions."""
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [3]
    adj[2] = []
    adj[3] = []
    reached: List[int] = dfs_reachable(adj, 0)
    if len(reached) != 4:
        ok = False
    edges: List[Tuple[int, int]] = [(0, 1), (2, 3)]
    cc: int = count_components(4, edges)
    if cc != 2:
        ok = False
    return ok
