"""Floyd-Warshall all-pairs shortest paths using flat matrix."""


def fw_init(n: int) -> list[int]:
    """Initialize n x n distance matrix with infinity (999999999)."""
    inf: int = 999999999
    size: int = n * n
    mat: list[int] = []
    i: int = 0
    while i < size:
        mat.append(inf)
        i = i + 1
    j: int = 0
    while j < n:
        mat[j * n + j] = 0
        j = j + 1
    return mat


def fw_set_edge(mat: list[int], n: int, u: int, v: int, w: int) -> int:
    """Set edge weight in the matrix. Returns 0."""
    mat[u * n + v] = w
    return 0


def fw_solve(mat: list[int], n: int) -> int:
    """Run Floyd-Warshall in-place. Returns 0."""
    inf: int = 999999999
    via: int = 0
    while via < n:
        i: int = 0
        while i < n:
            j: int = 0
            while j < n:
                d_ik: int = mat[i * n + via]
                d_kj: int = mat[via * n + j]
                if d_ik != inf and d_kj != inf:
                    new_dist: int = d_ik + d_kj
                    if new_dist < mat[i * n + j]:
                        mat[i * n + j] = new_dist
                j = j + 1
            i = i + 1
        via = via + 1
    return 0


def fw_query(mat: list[int], n: int, u: int, v: int) -> int:
    """Query shortest distance from u to v."""
    return mat[u * n + v]


def has_negative_cycle_fw(mat: list[int], n: int) -> int:
    """Check for negative cycle: any diagonal < 0. Returns 1/0."""
    i: int = 0
    while i < n:
        if mat[i * n + i] < 0:
            return 1
        i = i + 1
    return 0


def test_module() -> int:
    passed: int = 0

    n: int = 4
    mat: list[int] = fw_init(n)
    fw_set_edge(mat, n, 0, 1, 3)
    fw_set_edge(mat, n, 0, 3, 7)
    fw_set_edge(mat, n, 1, 0, 8)
    fw_set_edge(mat, n, 1, 2, 2)
    fw_set_edge(mat, n, 2, 0, 5)
    fw_set_edge(mat, n, 2, 3, 1)
    fw_set_edge(mat, n, 3, 0, 2)
    fw_solve(mat, n)

    if fw_query(mat, n, 0, 0) == 0:
        passed = passed + 1

    if fw_query(mat, n, 0, 2) == 5:
        passed = passed + 1

    if fw_query(mat, n, 0, 3) == 6:
        passed = passed + 1

    if fw_query(mat, n, 1, 3) == 3:
        passed = passed + 1

    if has_negative_cycle_fw(mat, n) == 0:
        passed = passed + 1

    if fw_query(mat, n, 3, 1) == 5:
        passed = passed + 1

    if fw_query(mat, n, 2, 1) == 6:
        passed = passed + 1

    return passed
