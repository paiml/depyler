"""Count unique paths in a grid with obstacles.

Tests: no obstacles, with obstacles, single cell, blocked paths, large grids.
"""


def unique_paths(m: int, n: int) -> int:
    """Return number of unique paths from top-left to bottom-right in m x n grid."""
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
    i = 0
    while i < m:
        dp[i][0] = 1
        i = i + 1
    j: int = 0
    while j < n:
        dp[0][j] = 1
        j = j + 1
    i = 1
    while i < m:
        j = 1
        while j < n:
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
            j = j + 1
        i = i + 1
    return dp[m - 1][n - 1]


def unique_paths_with_obstacles(grid: list[list[int]]) -> int:
    """Return unique paths avoiding obstacles. grid[i][j] == 1 means obstacle."""
    m: int = len(grid)
    if m == 0:
        return 0
    n: int = len(grid[0])
    if n == 0:
        return 0
    if grid[0][0] == 1:
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
    dp[0][0] = 1
    i = 1
    while i < m:
        if grid[i][0] == 0 and dp[i - 1][0] == 1:
            dp[i][0] = 1
        i = i + 1
    j: int = 1
    while j < n:
        if grid[0][j] == 0 and dp[0][j - 1] == 1:
            dp[0][j] = 1
        j = j + 1
    i = 1
    while i < m:
        j = 1
        while j < n:
            if grid[i][j] == 0:
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
            j = j + 1
        i = i + 1
    return dp[m - 1][n - 1]


def count_paths_with_diagonal(m: int, n: int) -> int:
    """Return unique paths allowing right, down, and diagonal moves."""
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
    i = 0
    while i < m:
        dp[i][0] = 1
        i = i + 1
    j: int = 0
    while j < n:
        dp[0][j] = 1
        j = j + 1
    i = 1
    while i < m:
        j = 1
        while j < n:
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1] + dp[i - 1][j - 1]
            j = j + 1
        i = i + 1
    return dp[m - 1][n - 1]


def test_module() -> int:
    """Test grid unique paths."""
    ok: int = 0

    if unique_paths(3, 3) == 6:
        ok = ok + 1
    if unique_paths(3, 7) == 28:
        ok = ok + 1
    if unique_paths(1, 1) == 1:
        ok = ok + 1
    if unique_paths(2, 2) == 2:
        ok = ok + 1

    grid1: list[list[int]] = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    if unique_paths_with_obstacles(grid1) == 2:
        ok = ok + 1

    grid2: list[list[int]] = [[1, 0], [0, 0]]
    if unique_paths_with_obstacles(grid2) == 0:
        ok = ok + 1

    grid3: list[list[int]] = [[0]]
    if unique_paths_with_obstacles(grid3) == 1:
        ok = ok + 1

    if count_paths_with_diagonal(2, 2) == 3:
        ok = ok + 1

    if unique_paths(1, 5) == 1:
        ok = ok + 1

    grid4: list[list[int]] = [[0, 0], [0, 1]]
    if unique_paths_with_obstacles(grid4) == 0:
        ok = ok + 1

    return ok
