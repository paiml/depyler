"""Graph algorithms: Bipartite matching.
Tests: augmenting path search, matching size computation.
"""
from typing import Dict, List, Tuple

def bipartite_max_matching(adj: Dict[int, List[int]], left_n: int, right_n: int) -> int:
    """Maximum bipartite matching using augmenting paths."""
    match_right: List[int] = []
    i: int = 0
    while i < right_n:
        match_right.append(-1)
        i += 1
    total: int = 0
    u: int = 0
    while u < left_n:
        visited: List[int] = []
        i = 0
        while i < right_n:
            visited.append(0)
            i += 1
        queue: List[int] = [u]
        head: int = 0
        found: bool = False
        while head < len(queue) and not found:
            node: int = queue[head]
            head += 1
            if node in adj:
                for v in adj[node]:
                    if visited[v] == 0:
                        visited[v] = 1
                        if match_right[v] < 0:
                            match_right[v] = node
                            found = True
                            break
                        else:
                            queue.append(match_right[v])
        if found:
            total += 1
        u += 1
    return total

def test_matching() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [0, 1]
    adj[1] = [0]
    adj[2] = [1, 2]
    m: int = bipartite_max_matching(adj, 3, 3)
    if m < 2:
        ok = False
    return ok
