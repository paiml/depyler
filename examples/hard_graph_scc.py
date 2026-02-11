def dfs_order(adj: list[list[int]], n: int) -> list[int]:
    visited: list[int] = []
    order: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    node: int = 0
    while node < n:
        if visited[node] == 0:
            stk: list[int] = [node]
            stk_phase: list[int] = [0]
            visited[node] = 1
            while len(stk) > 0:
                top: int = len(stk) - 1
                u: int = stk[top]
                ph: int = stk_phase[top]
                neighbors: list[int] = adj[u]
                if ph < len(neighbors):
                    stk_phase[top] = ph + 1
                    v: int = neighbors[ph]
                    if visited[v] == 0:
                        visited[v] = 1
                        stk.append(v)
                        stk_phase.append(0)
                else:
                    order.append(u)
                    stk.pop()
                    stk_phase.pop()
        node = node + 1
    return order

def transpose_graph(adj: list[list[int]], n: int) -> list[list[int]]:
    rev: list[list[int]] = []
    i: int = 0
    while i < n:
        rev.append([])
        i = i + 1
    u: int = 0
    while u < n:
        j: int = 0
        neighbors: list[int] = adj[u]
        while j < len(neighbors):
            v: int = neighbors[j]
            rev[v].append(u)
            j = j + 1
        u = u + 1
    return rev

def count_scc(adj: list[list[int]], n: int) -> int:
    order: list[int] = dfs_order(adj, n)
    rev: list[list[int]] = transpose_graph(adj, n)
    visited: list[int] = []
    i: int = 0
    while i < n:
        visited.append(0)
        i = i + 1
    count: int = 0
    idx: int = len(order) - 1
    while idx >= 0:
        node: int = order[idx]
        if visited[node] == 0:
            count = count + 1
            stk: list[int] = [node]
            visited[node] = 1
            while len(stk) > 0:
                top: int = len(stk) - 1
                u: int = stk[top]
                stk.pop()
                j: int = 0
                rn: list[int] = rev[u]
                while j < len(rn):
                    v: int = rn[j]
                    if visited[v] == 0:
                        visited[v] = 1
                        stk.append(v)
                    j = j + 1
        idx = idx - 1
    return count

def test_module() -> int:
    passed: int = 0
    adj1: list[list[int]] = [[1], [2], [0, 3], [4], []]
    if count_scc(adj1, 5) == 3:
        passed = passed + 1
    adj2: list[list[int]] = [[1], [0]]
    if count_scc(adj2, 2) == 1:
        passed = passed + 1
    adj3: list[list[int]] = [[], []]
    if count_scc(adj3, 2) == 2:
        passed = passed + 1
    adj4: list[list[int]] = [[1], [2], [0]]
    if count_scc(adj4, 3) == 1:
        passed = passed + 1
    rev: list[list[int]] = transpose_graph(adj4, 3)
    r0: list[int] = rev[0]
    if r0[0] == 2:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
