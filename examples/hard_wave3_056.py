"""Graph algorithms: Shortest path tree and predecessor tracking.
Tests: path reconstruction, predecessor array, path length.
"""
from typing import Dict, List, Tuple

def bfs_with_pred(adj: Dict[int, List[int]], start: int, n: int) -> Tuple[List[int], List[int]]:
    """BFS returning distance and predecessor arrays."""
    big: int = -1
    dist: List[int] = []
    pred: List[int] = []
    i: int = 0
    while i < n:
        dist.append(big)
        pred.append(big)
        i += 1
    dist[start] = 0
    queue: List[int] = [start]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            for nb in adj[node]:
                if dist[nb] == big:
                    dist[nb] = dist[node] + 1
                    pred[nb] = node
                    queue.append(nb)
    return (dist, pred)

def reconstruct_path(pred: List[int], start: int, end: int) -> List[int]:
    """Reconstruct path from predecessor array."""
    if pred[end] == -1 and end != start:
        return []
    path: List[int] = []
    node: int = end
    while node != -1:
        path.append(node)
        if node == start:
            break
        node = pred[node]
    path.reverse()
    return path

def shortest_path_nodes(adj: Dict[int, List[int]], start: int, end: int, n: int) -> List[int]:
    """Get nodes on shortest path from start to end."""
    result: Tuple[List[int], List[int]] = bfs_with_pred(adj, start, n)
    return reconstruct_path(result[1], start, end)

def test_pred() -> bool:
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [3]
    adj[2] = [3]
    adj[3] = []
    path: List[int] = shortest_path_nodes(adj, 0, 3, 4)
    if len(path) != 3:
        ok = False
    return ok
