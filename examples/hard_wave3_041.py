"""Graph algorithms: BFS traversal and shortest path.

Tests: queue simulation, visited tracking, level-order traversal,
adjacency list processing, distance computation.
"""

from typing import Dict, List, Tuple


def bfs_distances(adj: Dict[int, List[int]], start: int) -> Dict[int, int]:
    """BFS from start, return distances to all reachable nodes."""
    dist: Dict[int, int] = {}
    dist[start] = 0
    queue: List[int] = [start]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            neighbors: List[int] = adj[node]
            for nb in neighbors:
                if nb not in dist:
                    dist[nb] = dist[node] + 1
                    queue.append(nb)
    return dist


def bfs_count_reachable(adj: Dict[int, List[int]], start: int) -> int:
    """Count nodes reachable from start via BFS."""
    visited: Dict[int, int] = {}
    visited[start] = 1
    queue: List[int] = [start]
    head: int = 0
    count: int = 1
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            for nb in adj[node]:
                if nb not in visited:
                    visited[nb] = 1
                    queue.append(nb)
                    count += 1
    return count


def bfs_path_exists(adj: Dict[int, List[int]], start: int, end: int) -> bool:
    """Check if path exists from start to end."""
    if start == end:
        return True
    visited: Dict[int, int] = {}
    visited[start] = 1
    queue: List[int] = [start]
    head: int = 0
    while head < len(queue):
        node: int = queue[head]
        head += 1
        if node in adj:
            for nb in adj[node]:
                if nb == end:
                    return True
                if nb not in visited:
                    visited[nb] = 1
                    queue.append(nb)
    return False


def bfs_level_sizes(adj: Dict[int, List[int]], start: int) -> List[int]:
    """Return size of each BFS level."""
    sizes: List[int] = []
    visited: Dict[int, int] = {}
    visited[start] = 1
    current_level: List[int] = [start]
    while len(current_level) > 0:
        sizes.append(len(current_level))
        next_level: List[int] = []
        for node in current_level:
            if node in adj:
                for nb in adj[node]:
                    if nb not in visited:
                        visited[nb] = 1
                        next_level.append(nb)
        current_level = next_level
    return sizes


def test_bfs() -> bool:
    """Test BFS functions."""
    ok: bool = True
    adj: Dict[int, List[int]] = {}
    adj[0] = [1, 2]
    adj[1] = [3]
    adj[2] = [3]
    adj[3] = []
    dist: Dict[int, int] = bfs_distances(adj, 0)
    if dist[3] != 2:
        ok = False
    if not bfs_path_exists(adj, 0, 3):
        ok = False
    rc: int = bfs_count_reachable(adj, 0)
    if rc != 4:
        ok = False
    return ok
