"""Grid path counting with dynamic programming.

Tests: unique paths, paths with obstacles, min cost path, max value path.
"""


def unique_paths(m: int, n: int) -> int:
    """Count unique paths from top-left to bottom-right in m x n grid."""
    dp: list[int] = [1] * n
    i: int = 1
    while i < m:
        j: int = 1
        while j < n:
            dp[j] = dp[j] + dp[j - 1]
            j = j + 1
        i = i + 1
    return dp[n - 1]


def paths_with_obstacles(grid: list[list[int]], rows: int, cols: int) -> int:
    """Count paths avoiding obstacles (1 = obstacle, 0 = free)."""
    if grid[0][0] == 1:
        return 0
    dp: list[int] = [0] * cols
    dp[0] = 1
    i: int = 0
    while i < rows:
        j: int = 0
        while j < cols:
            if grid[i][j] == 1:
                dp[j] = 0
            elif j > 0:
                dp[j] = dp[j] + dp[j - 1]
            j = j + 1
        i = i + 1
    return dp[cols - 1]


def min_path_sum_1d(grid: list[int], rows: int, cols: int) -> int:
    """Minimum path sum in a grid stored as 1D array (row-major)."""
    dp: list[int] = [0] * (rows * cols)
    dp[0] = grid[0]
    j: int = 1
    while j < cols:
        dp[j] = dp[j - 1] + grid[j]
        j = j + 1
    i: int = 1
    while i < rows:
        dp[i * cols] = dp[(i - 1) * cols] + grid[i * cols]
        j = 1
        while j < cols:
            from_top: int = dp[(i - 1) * cols + j]
            from_left: int = dp[i * cols + j - 1]
            smaller: int = from_top
            if from_left < smaller:
                smaller = from_left
            dp[i * cols + j] = smaller + grid[i * cols + j]
            j = j + 1
        i = i + 1
    return dp[rows * cols - 1]


def count_paths_with_sum(m: int, n: int) -> int:
    """Sum of unique_paths for all sub-grids from (1,1) to (m,n)."""
    total: int = 0
    r: int = 1
    while r <= m:
        c: int = 1
        while c <= n:
            total = total + unique_paths(r, c)
            c = c + 1
        r = r + 1
    return total


def test_module() -> int:
    """Test grid path operations."""
    ok: int = 0
    if unique_paths(3, 3) == 6:
        ok = ok + 1
    if unique_paths(3, 7) == 28:
        ok = ok + 1
    if unique_paths(1, 1) == 1:
        ok = ok + 1
    g1: list[list[int]] = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    if paths_with_obstacles(g1, 3, 3) == 2:
        ok = ok + 1
    flat: list[int] = [1, 3, 1, 1, 5, 1, 4, 2, 1]
    if min_path_sum_1d(flat, 3, 3) == 7:
        ok = ok + 1
    return ok
