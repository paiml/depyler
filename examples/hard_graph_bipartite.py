def is_bipartite(adj: list[list[int]], n: int) -> int:
    color: list[int] = []
    i: int = 0
    while i < n:
        color.append(-1)
        i = i + 1
    node: int = 0
    while node < n:
        if color[node] == -1:
            queue: list[int] = [node]
            color[node] = 0
            front: int = 0
            while front < len(queue):
                u: int = queue[front]
                front = front + 1
                j: int = 0
                neighbors: list[int] = adj[u]
                while j < len(neighbors):
                    v: int = neighbors[j]
                    if color[v] == -1:
                        color[v] = 1 - color[u]
                        queue.append(v)
                    elif color[v] == color[u]:
                        return 0
                    j = j + 1
        node = node + 1
    return 1

def count_per_side(adj: list[list[int]], n: int) -> list[int]:
    color: list[int] = []
    i: int = 0
    while i < n:
        color.append(-1)
        i = i + 1
    queue: list[int] = [0]
    color[0] = 0
    front: int = 0
    while front < len(queue):
        u: int = queue[front]
        front = front + 1
        j: int = 0
        neighbors: list[int] = adj[u]
        while j < len(neighbors):
            v: int = neighbors[j]
            if color[v] == -1:
                color[v] = 1 - color[u]
                queue.append(v)
            j = j + 1
    side0: int = 0
    side1: int = 0
    k: int = 0
    while k < n:
        if color[k] == 0:
            side0 = side0 + 1
        else:
            side1 = side1 + 1
        k = k + 1
    return [side0, side1]

def edge_count(adj: list[list[int]], n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        total = total + len(adj[i])
        i = i + 1
    return total // 2

def test_module() -> int:
    passed: int = 0
    bip: list[list[int]] = [[1, 3], [0, 2], [1, 3], [0, 2]]
    if is_bipartite(bip, 4) == 1:
        passed = passed + 1
    tri: list[list[int]] = [[1, 2], [0, 2], [0, 1]]
    if is_bipartite(tri, 3) == 0:
        passed = passed + 1
    sides: list[int] = count_per_side(bip, 4)
    if sides[0] == 2 and sides[1] == 2:
        passed = passed + 1
    if edge_count(bip, 4) == 4:
        passed = passed + 1
    single: list[list[int]] = [[]]
    if is_bipartite(single, 1) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
