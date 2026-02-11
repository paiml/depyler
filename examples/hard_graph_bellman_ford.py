def bellman_ford(n: int, edges_u: list[int], edges_v: list[int], edges_w: list[int], src: int) -> list[int]:
    dist: list[int] = []
    i: int = 0
    while i < n:
        dist.append(999999)
        i = i + 1
    dist[src] = 0
    e: int = len(edges_u)
    iteration: int = 0
    while iteration < n - 1:
        j: int = 0
        while j < e:
            u: int = edges_u[j]
            v: int = edges_v[j]
            w: int = edges_w[j]
            if dist[u] != 999999 and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
            j = j + 1
        iteration = iteration + 1
    return dist

def has_negative_cycle(n: int, edges_u: list[int], edges_v: list[int], edges_w: list[int], src: int) -> int:
    dist: list[int] = bellman_ford(n, edges_u, edges_v, edges_w, src)
    e: int = len(edges_u)
    j: int = 0
    while j < e:
        u: int = edges_u[j]
        v: int = edges_v[j]
        w: int = edges_w[j]
        if dist[u] != 999999 and dist[u] + w < dist[v]:
            return 1
        j = j + 1
    return 0

def shortest_path(n: int, edges_u: list[int], edges_v: list[int], edges_w: list[int], src: int, dst: int) -> int:
    dist: list[int] = bellman_ford(n, edges_u, edges_v, edges_w, src)
    return dist[dst]

def test_module() -> int:
    passed: int = 0
    eu: list[int] = [0, 0, 1, 2]
    ev: list[int] = [1, 2, 2, 3]
    ew: list[int] = [4, 2, 3, 1]
    d: list[int] = bellman_ford(4, eu, ev, ew, 0)
    if d[3] == 3:
        passed = passed + 1
    if d[1] == 4:
        passed = passed + 1
    if shortest_path(4, eu, ev, ew, 0, 2) == 2:
        passed = passed + 1
    if has_negative_cycle(4, eu, ev, ew, 0) == 0:
        passed = passed + 1
    eu2: list[int] = [0, 1, 2]
    ev2: list[int] = [1, 2, 0]
    ew2: list[int] = [1, 1, -3]
    if has_negative_cycle(3, eu2, ev2, ew2, 0) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
