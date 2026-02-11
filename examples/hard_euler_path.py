"""Euler path detection via degree counting on undirected graphs."""


def compute_degrees(num_nodes: int, edges_u: list[int], edges_v: list[int], num_edges: int) -> list[int]:
    """Compute degree of each node. edges_u[i]->edges_v[i] is edge i (undirected)."""
    deg: list[int] = []
    i: int = 0
    while i < num_nodes:
        deg.append(0)
        i = i + 1
    e: int = 0
    while e < num_edges:
        deg[edges_u[e]] = deg[edges_u[e]] + 1
        deg[edges_v[e]] = deg[edges_v[e]] + 1
        e = e + 1
    return deg


def count_odd_degree(deg: list[int], num_nodes: int) -> int:
    """Count nodes with odd degree."""
    count: int = 0
    i: int = 0
    while i < num_nodes:
        if deg[i] % 2 == 1:
            count = count + 1
        i = i + 1
    return count


def has_euler_circuit(num_nodes: int, edges_u: list[int], edges_v: list[int], num_edges: int) -> int:
    """Check if graph has an Euler circuit (all degrees even, connected). Returns 1/0.
    Simplified: only checks degree condition.
    """
    deg: list[int] = compute_degrees(num_nodes, edges_u, edges_v, num_edges)
    odd: int = count_odd_degree(deg, num_nodes)
    if odd == 0:
        return 1
    return 0


def has_euler_path(num_nodes: int, edges_u: list[int], edges_v: list[int], num_edges: int) -> int:
    """Check if graph has an Euler path (0 or 2 odd-degree nodes). Returns 1/0."""
    deg: list[int] = compute_degrees(num_nodes, edges_u, edges_v, num_edges)
    odd: int = count_odd_degree(deg, num_nodes)
    if odd == 0 or odd == 2:
        return 1
    return 0


def max_node_degree(deg: list[int], num_nodes: int) -> int:
    """Find maximum degree among all nodes."""
    best: int = 0
    i: int = 0
    while i < num_nodes:
        if deg[i] > best:
            best = deg[i]
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    eu1: list[int] = [0, 1, 2, 3]
    ev1: list[int] = [1, 2, 3, 0]
    if has_euler_circuit(4, eu1, ev1, 4) == 1:
        passed = passed + 1

    if has_euler_path(4, eu1, ev1, 4) == 1:
        passed = passed + 1

    eu2: list[int] = [0, 0, 1]
    ev2: list[int] = [1, 2, 2]
    if has_euler_circuit(3, eu2, ev2, 3) == 1:
        passed = passed + 1

    deg: list[int] = compute_degrees(3, eu2, ev2, 3)
    if deg[0] == 2 and deg[1] == 2 and deg[2] == 2:
        passed = passed + 1

    eu3: list[int] = [0, 1, 0]
    ev3: list[int] = [1, 2, 2]
    deg3: list[int] = compute_degrees(4, eu3, ev3, 3)
    if deg3[0] == 2 and deg3[1] == 2 and deg3[2] == 2 and deg3[3] == 0:
        passed = passed + 1

    eu4: list[int] = [0, 0, 0, 1]
    ev4: list[int] = [1, 2, 3, 2]
    if has_euler_circuit(4, eu4, ev4, 4) == 0:
        passed = passed + 1

    if has_euler_path(4, eu4, ev4, 4) == 1:
        passed = passed + 1

    if max_node_degree(deg, 3) == 2:
        passed = passed + 1

    return passed
