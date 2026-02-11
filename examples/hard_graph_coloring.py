"""Graph coloring (greedy) operations.

Tests: greedy coloring, chromatic bound, is valid coloring, colors used.
"""


def greedy_color_count(n: int, edges_u: list[int], edges_v: list[int]) -> int:
    """Greedy graph coloring, returns number of colors used."""
    colors: list[int] = []
    i: int = 0
    while i < n:
        colors.append(-1)
        i = i + 1
    node: int = 0
    while node < n:
        used: list[int] = []
        j: int = 0
        while j < n:
            used.append(0)
            j = j + 1
        e: int = len(edges_u)
        j = 0
        while j < e:
            neighbor: int = -1
            if edges_u[j] == node and colors[edges_v[j]] >= 0:
                neighbor = colors[edges_v[j]]
            if edges_v[j] == node and colors[edges_u[j]] >= 0:
                neighbor = colors[edges_u[j]]
            if neighbor >= 0:
                used[neighbor] = 1
            j = j + 1
        c: int = 0
        while c < n:
            if used[c] == 0:
                colors[node] = c
                c = n
            else:
                c = c + 1
        node = node + 1
    max_color: int = 0
    for c in colors:
        if c > max_color:
            max_color = c
    return max_color + 1


def is_valid_coloring(n: int, edges_u: list[int], edges_v: list[int], colors: list[int]) -> int:
    """Returns 1 if no adjacent nodes share a color."""
    e: int = len(edges_u)
    i: int = 0
    while i < e:
        if colors[edges_u[i]] == colors[edges_v[i]]:
            return 0
        i = i + 1
    return 1


def count_edges(edges_u: list[int]) -> int:
    """Count number of edges."""
    return len(edges_u)


def test_module() -> int:
    """Test graph coloring."""
    ok: int = 0
    eu: list[int] = [0, 0, 1]
    ev: list[int] = [1, 2, 2]
    c: int = greedy_color_count(3, eu, ev)
    if c == 3:
        ok = ok + 1
    if is_valid_coloring(3, eu, ev, [0, 1, 2]) == 1:
        ok = ok + 1
    if is_valid_coloring(3, eu, ev, [0, 0, 1]) == 0:
        ok = ok + 1
    eu2: list[int] = [0, 2]
    ev2: list[int] = [1, 3]
    c2: int = greedy_color_count(4, eu2, ev2)
    if c2 == 2:
        ok = ok + 1
    if count_edges(eu) == 3:
        ok = ok + 1
    return ok
