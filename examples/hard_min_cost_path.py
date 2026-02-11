"""Minimum cost path in a 2D grid from top-left to bottom-right.

Tests: small grids, single cell, single row/column, general cases.
"""


def min_cost_path(grid: list[list[int]]) -> int:
    """Return minimum cost to reach bottom-right from top-left (move right or down)."""
    m: int = len(grid)
    if m == 0:
        return 0
    n: int = len(grid[0])
    if n == 0:
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i < m:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    dp[0][0] = grid[0][0]
    i = 1
    while i < m:
        dp[i][0] = dp[i - 1][0] + grid[i][0]
        i = i + 1
    j: int = 1
    while j < n:
        dp[0][j] = dp[0][j - 1] + grid[0][j]
        j = j + 1
    i = 1
    while i < m:
        j = 1
        while j < n:
            a: int = dp[i - 1][j]
            b: int = dp[i][j - 1]
            if a < b:
                dp[i][j] = a + grid[i][j]
            else:
                dp[i][j] = b + grid[i][j]
            j = j + 1
        i = i + 1
    return dp[m - 1][n - 1]


def min_cost_path_with_diagonal(grid: list[list[int]]) -> int:
    """Return minimum cost path allowing right, down, and diagonal moves."""
    m: int = len(grid)
    if m == 0:
        return 0
    n: int = len(grid[0])
    if n == 0:
        return 0
    dp: list[list[int]] = []
    i: int = 0
    while i < m:
        row: list[int] = []
        j: int = 0
        while j < n:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    dp[0][0] = grid[0][0]
    i = 1
    while i < m:
        dp[i][0] = dp[i - 1][0] + grid[i][0]
        i = i + 1
    j: int = 1
    while j < n:
        dp[0][j] = dp[0][j - 1] + grid[0][j]
        j = j + 1
    i = 1
    while i < m:
        j = 1
        while j < n:
            a: int = dp[i - 1][j]
            b: int = dp[i][j - 1]
            c: int = dp[i - 1][j - 1]
            best: int = a
            if b < best:
                best = b
            if c < best:
                best = c
            dp[i][j] = best + grid[i][j]
            j = j + 1
        i = i + 1
    return dp[m - 1][n - 1]


def sum_grid(grid: list[list[int]]) -> int:
    """Return sum of all elements in grid."""
    total: int = 0
    i: int = 0
    while i < len(grid):
        j: int = 0
        while j < len(grid[i]):
            total = total + grid[i][j]
            j = j + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test minimum cost path."""
    ok: int = 0

    g1: list[list[int]] = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
    if min_cost_path(g1) == 7:
        ok = ok + 1

    g2: list[list[int]] = [[5]]
    if min_cost_path(g2) == 5:
        ok = ok + 1

    g3: list[list[int]] = [[1, 2, 3]]
    if min_cost_path(g3) == 6:
        ok = ok + 1

    g4: list[list[int]] = [[1], [2], [3]]
    if min_cost_path(g4) == 6:
        ok = ok + 1

    g5: list[list[int]] = [[1, 2], [3, 4]]
    if min_cost_path(g5) == 7:
        ok = ok + 1

    if min_cost_path_with_diagonal(g5) == 5:
        ok = ok + 1

    if min_cost_path_with_diagonal(g1) == 5:
        ok = ok + 1

    if sum_grid(g1) == 19:
        ok = ok + 1

    g6: list[list[int]] = [[1, 100], [1, 1]]
    if min_cost_path(g6) == 3:
        ok = ok + 1

    g7: list[list[int]] = [[1, 1, 1], [1, 1, 1], [1, 1, 1]]
    if min_cost_path(g7) == 5:
        ok = ok + 1

    return ok
