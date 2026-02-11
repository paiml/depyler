"""Minimum path sum in a grid (flat array representation).

Tests: min path sum, grid value, path count, max path sum.
"""


def min_path_sum(grid: list[int], rows: int, cols: int) -> int:
    """Min path sum top-left to bottom-right, moving right or down."""
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = grid[0]
    c: int = 1
    while c < cols:
        dp[c] = dp[c - 1] + grid[c]
        c = c + 1
    r: int = 1
    while r < rows:
        dp[r * cols] = dp[(r - 1) * cols] + grid[r * cols]
        r = r + 1
    r = 1
    while r < rows:
        c = 1
        while c < cols:
            up: int = dp[(r - 1) * cols + c]
            left: int = dp[r * cols + c - 1]
            smaller: int = up
            if left < smaller:
                smaller = left
            dp[r * cols + c] = grid[r * cols + c] + smaller
            c = c + 1
        r = r + 1
    return dp[rows * cols - 1]


def grid_total(grid: list[int]) -> int:
    """Sum of all grid values."""
    total: int = 0
    for v in grid:
        total = total + v
    return total


def count_paths(rows: int, cols: int) -> int:
    """Count unique paths top-left to bottom-right."""
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    c: int = 0
    while c < cols:
        dp[c] = 1
        c = c + 1
    r: int = 0
    while r < rows:
        dp[r * cols] = 1
        r = r + 1
    r = 1
    while r < rows:
        c = 1
        while c < cols:
            dp[r * cols + c] = dp[(r - 1) * cols + c] + dp[r * cols + c - 1]
            c = c + 1
        r = r + 1
    return dp[rows * cols - 1]


def test_module() -> int:
    """Test minimum path sum."""
    ok: int = 0
    g: list[int] = [1, 3, 1, 1, 5, 1, 4, 2, 1]
    if min_path_sum(g, 3, 3) == 7:
        ok = ok + 1
    if grid_total(g) == 19:
        ok = ok + 1
    if count_paths(3, 3) == 6:
        ok = ok + 1
    if count_paths(2, 2) == 2:
        ok = ok + 1
    if count_paths(1, 5) == 1:
        ok = ok + 1
    return ok
