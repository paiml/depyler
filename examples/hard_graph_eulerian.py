def degree_list(adj: list[list[int]], n: int) -> list[int]:
    degs: list[int] = []
    i: int = 0
    while i < n:
        degs.append(len(adj[i]))
        i = i + 1
    return degs

def count_odd_degree(adj: list[list[int]], n: int) -> int:
    degs: list[int] = degree_list(adj, n)
    count: int = 0
    i: int = 0
    while i < n:
        if degs[i] % 2 == 1:
            count = count + 1
        i = i + 1
    return count

def has_euler_circuit(adj: list[list[int]], n: int) -> int:
    odd: int = count_odd_degree(adj, n)
    if odd == 0:
        return 1
    return 0

def has_euler_path(adj: list[list[int]], n: int) -> int:
    odd: int = count_odd_degree(adj, n)
    if odd == 0 or odd == 2:
        return 1
    return 0

def total_edges(adj: list[list[int]], n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        total = total + len(adj[i])
        i = i + 1
    return total // 2

def is_connected(adj: list[list[int]], n: int) -> int:
    if n == 0:
        return 1
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    queue: list[int] = [0]
    visited[0] = 1
    front: int = 0
    while front < len(queue):
        u: int = queue[front]
        front = front + 1
        j: int = 0
        neighbors: list[int] = adj[u]
        while j < len(neighbors):
            v: int = neighbors[j]
            if visited[v] == 0:
                visited[v] = 1
                queue.append(v)
            j = j + 1
    count: int = 0
    k: int = 0
    while k < n:
        if visited[k] == 1:
            count = count + 1
        k = k + 1
    if count == n:
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    tri: list[list[int]] = [[1, 2], [0, 2], [0, 1]]
    if has_euler_circuit(tri, 3) == 1:
        passed = passed + 1
    path_g: list[list[int]] = [[1], [0, 2], [1]]
    if has_euler_path(path_g, 3) == 1:
        passed = passed + 1
    if has_euler_circuit(path_g, 3) == 0:
        passed = passed + 1
    if total_edges(tri, 3) == 3:
        passed = passed + 1
    if is_connected(tri, 3) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
