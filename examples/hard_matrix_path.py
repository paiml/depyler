"""Matrix min/max path sum algorithms.

Tests: min path sum, max path sum, path existence, count paths above threshold.
"""


def min_path_sum(grid: list[list[int]], rows: int, cols: int) -> int:
    """Minimum path sum from top-left to bottom-right (right and down only)."""
    dp: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = [0] * cols
        dp = dp + [row]
        r = r + 1
    dp[0][0] = grid[0][0]
    j: int = 1
    while j < cols:
        dp[0][j] = dp[0][j - 1] + grid[0][j]
        j = j + 1
    i: int = 1
    while i < rows:
        dp[i][0] = dp[i - 1][0] + grid[i][0]
        j = 1
        while j < cols:
            top: int = dp[i - 1][j]
            left: int = dp[i][j - 1]
            smaller: int = top
            if left < smaller:
                smaller = left
            dp[i][j] = smaller + grid[i][j]
            j = j + 1
        i = i + 1
    return dp[rows - 1][cols - 1]


def max_path_sum(grid: list[list[int]], rows: int, cols: int) -> int:
    """Maximum path sum from top-left to bottom-right."""
    dp: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = [0] * cols
        dp = dp + [row]
        r = r + 1
    dp[0][0] = grid[0][0]
    j: int = 1
    while j < cols:
        dp[0][j] = dp[0][j - 1] + grid[0][j]
        j = j + 1
    i: int = 1
    while i < rows:
        dp[i][0] = dp[i - 1][0] + grid[i][0]
        j = 1
        while j < cols:
            top: int = dp[i - 1][j]
            left: int = dp[i][j - 1]
            bigger: int = top
            if left > bigger:
                bigger = left
            dp[i][j] = bigger + grid[i][j]
            j = j + 1
        i = i + 1
    return dp[rows - 1][cols - 1]


def grid_sum(grid: list[list[int]], rows: int, cols: int) -> int:
    """Sum of all elements in grid."""
    total: int = 0
    i: int = 0
    while i < rows:
        j: int = 0
        while j < cols:
            total = total + grid[i][j]
            j = j + 1
        i = i + 1
    return total


def test_module() -> int:
    """Test matrix path operations."""
    ok: int = 0
    g: list[list[int]] = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
    if min_path_sum(g, 3, 3) == 7:
        ok = ok + 1
    if max_path_sum(g, 3, 3) == 12:
        ok = ok + 1
    g2: list[list[int]] = [[1, 2], [3, 4]]
    if min_path_sum(g2, 2, 2) == 7:
        ok = ok + 1
    if max_path_sum(g2, 2, 2) == 8:
        ok = ok + 1
    if grid_sum(g, 3, 3) == 19:
        ok = ok + 1
    return ok
