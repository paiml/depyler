def find_bridges(adj: list[list[int]], n: int) -> list[int]:
    disc: list[int] = []
    low: list[int] = []
    visited: list[int] = []
    parent: list[int] = []
    bridges_u: list[int] = []
    bridges_v: list[int] = []
    i: int = 0
    while i < n:
        disc.append(-1)
        low.append(-1)
        visited.append(0)
        parent.append(-1)
        i = i + 1
    timer: list[int] = [0]
    stack: list[int] = [0]
    phase: list[int] = [0]
    child_idx: list[int] = []
    i = 0
    while i < n:
        child_idx.append(0)
        i = i + 1
    visited[0] = 1
    disc[0] = 0
    low[0] = 0
    timer[0] = 1
    while len(stack) > 0:
        top: int = len(stack) - 1
        u: int = stack[top]
        neighbors: list[int] = adj[u]
        if child_idx[u] < len(neighbors):
            ci: int = child_idx[u]
            v: int = neighbors[ci]
            child_idx[u] = child_idx[u] + 1
            if visited[v] == 0:
                visited[v] = 1
                parent[v] = u
                disc[v] = timer[0]
                low[v] = timer[0]
                timer[0] = timer[0] + 1
                stack.append(v)
            elif v != parent[u]:
                if disc[v] < low[u]:
                    low[u] = disc[v]
        else:
            stack.pop()
            if len(stack) > 0:
                p: int = parent[u]
                if low[u] < low[p]:
                    low[p] = low[u]
                if low[u] > disc[p]:
                    bridges_u.append(p)
                    bridges_v.append(u)
    return bridges_u

def count_bridges(adj: list[list[int]], n: int) -> int:
    b: list[int] = find_bridges(adj, n)
    return len(b)

def degree(adj: list[list[int]], node: int) -> int:
    return len(adj[node])

def test_module() -> int:
    passed: int = 0
    adj1: list[list[int]] = [[1, 2], [0, 2], [0, 1, 3], [2]]
    if count_bridges(adj1, 4) == 1:
        passed = passed + 1
    adj2: list[list[int]] = [[1, 2], [0, 2], [0, 1]]
    if count_bridges(adj2, 3) == 0:
        passed = passed + 1
    adj3: list[list[int]] = [[1], [0, 2], [1]]
    if count_bridges(adj3, 3) == 2:
        passed = passed + 1
    if degree(adj1, 2) == 3:
        passed = passed + 1
    if degree(adj1, 0) == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
