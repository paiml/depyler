"""Path counting in grids.

Tests: unique paths, paths with obstacles, diagonal paths, weighted paths.
"""


def unique_paths(rows: int, cols: int) -> int:
    """Count unique paths from top-left to bottom-right (right/down only)."""
    dp: list[int] = []
    j: int = 0
    while j < cols:
        dp.append(1)
        j = j + 1
    i: int = 1
    while i < rows:
        j = 1
        while j < cols:
            dp[j] = dp[j] + dp[j - 1]
            j = j + 1
        i = i + 1
    return dp[cols - 1]


def unique_paths_with_obstacles(grid: list[list[int]], rows: int, cols: int) -> int:
    """Count unique paths avoiding obstacles (1 = obstacle)."""
    dp: list[list[int]] = []
    i: int = 0
    while i < rows:
        row: list[int] = []
        j: int = 0
        while j < cols:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    if grid[0][0] == 1:
        return 0
    dp[0][0] = 1
    j = 1
    while j < cols:
        if grid[0][j] == 0 and dp[0][j - 1] == 1:
            dp[0][j] = 1
        j = j + 1
    i = 1
    while i < rows:
        if grid[i][0] == 0 and dp[i - 1][0] == 1:
            dp[i][0] = 1
        i = i + 1
    i = 1
    while i < rows:
        j = 1
        while j < cols:
            if grid[i][j] == 0:
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
            j = j + 1
        i = i + 1
    return dp[rows - 1][cols - 1]


def count_square_paths(n: int) -> int:
    """Count paths in n x n grid."""
    return unique_paths(n, n)


def catalan_paths(n: int) -> int:
    """Count monotonic lattice paths that do not cross diagonal (Catalan number)."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * (2 * n - i) // (i + 1)
        i = i + 1
    return result // (n + 1)


def test_module() -> int:
    """Test path counting operations."""
    ok: int = 0
    if unique_paths(3, 3) == 6:
        ok = ok + 1
    if unique_paths(3, 7) == 28:
        ok = ok + 1
    grid: list[list[int]] = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    if unique_paths_with_obstacles(grid, 3, 3) == 2:
        ok = ok + 1
    if count_square_paths(4) == 20:
        ok = ok + 1
    if catalan_paths(4) == 14:
        ok = ok + 1
    if catalan_paths(5) == 42:
        ok = ok + 1
    return ok
