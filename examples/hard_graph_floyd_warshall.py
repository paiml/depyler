def floyd_warshall(dist: list[list[int]], n: int) -> list[list[int]]:
    result: list[list[int]] = []
    i: int = 0
    while i < n:
        row: list[int] = []
        j: int = 0
        while j < n:
            src_row: list[int] = dist[i]
            row.append(src_row[j])
            j = j + 1
        result.append(row)
        i = i + 1
    k: int = 0
    while k < n:
        i = 0
        while i < n:
            j = 0
            while j < n:
                ri: list[int] = result[i]
                rk: list[int] = result[k]
                rik: int = ri[k]
                rkj: int = rk[j]
                if rik != 999999 and rkj != 999999:
                    candidate: int = rik + rkj
                    if candidate < ri[j]:
                        ri[j] = candidate
                j = j + 1
            i = i + 1
        k = k + 1
    return result

def shortest_dist(dist: list[list[int]], n: int, src: int, dst: int) -> int:
    fw: list[list[int]] = floyd_warshall(dist, n)
    row: list[int] = fw[src]
    return row[dst]

def diameter(dist: list[list[int]], n: int) -> int:
    fw: list[list[int]] = floyd_warshall(dist, n)
    mx: int = 0
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            row: list[int] = fw[i]
            val: int = row[j]
            if val != 999999 and val > mx:
                mx = val
            j = j + 1
        i = i + 1
    return mx

def test_module() -> int:
    passed: int = 0
    inf: int = 999999
    d: list[list[int]] = [[0, 3, inf, 7], [8, 0, 2, inf], [5, inf, 0, 1], [2, inf, inf, 0]]
    fw: list[list[int]] = floyd_warshall(d, 4)
    fw0: list[int] = fw[0]
    if fw0[1] == 3:
        passed = passed + 1
    if fw0[2] == 5:
        passed = passed + 1
    if shortest_dist(d, 4, 0, 3) == 6:
        passed = passed + 1
    if shortest_dist(d, 4, 3, 1) == 5:
        passed = passed + 1
    if diameter(d, 4) == 7:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
