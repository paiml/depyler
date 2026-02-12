"""Graph algorithms: Euler path and circuit detection.
Tests: degree parity check, connected component verification.
"""
from typing import Dict, List, Tuple

def has_euler_circuit(adj: Dict[int, List[int]], n: int) -> bool:
    """Check if undirected graph has Euler circuit (all even degrees)."""
    i: int = 0
    while i < n:
        if i in adj:
            if len(adj[i]) % 2 != 0:
                return False
        i += 1
    visited: Dict[int, int] = {}
    start: int = -1
    i = 0
    while i < n:
        if i in adj and len(adj[i]) > 0:
            start = i
            break
        i += 1
    if start < 0:
        return True
    stack: List[int] = [start]
    while len(stack) > 0:
        node: int = stack[len(stack) - 1]
        stack.pop()
        if node not in visited:
            visited[node] = 1
            if node in adj:
                for nb in adj[node]:
                    if nb not in visited:
                        stack.append(nb)
    i = 0
    while i < n:
        if i in adj and len(adj[i]) > 0 and i not in visited:
            return False
        i += 1
    return True

def has_euler_path(adj: Dict[int, List[int]], n: int) -> bool:
    """Check if undirected graph has Euler path (0 or 2 odd-degree nodes)."""
    odd_count: int = 0
    i: int = 0
    while i < n:
        if i in adj:
            if len(adj[i]) % 2 != 0:
                odd_count += 1
        i += 1
    return odd_count == 0 or odd_count == 2

def test_euler() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [0, 2]
    adj[2] = [0, 1]
    if not has_euler_circuit(adj, 3):
        ok = False
    return ok
