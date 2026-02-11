"""Grid path counting: count paths with obstacles and constraints."""


def count_paths_no_obstacles(rows: int, cols: int) -> int:
    """Count paths from top-left to bottom-right moving only right or down."""
    if rows <= 0 or cols <= 0:
        return 0
    # dp stored as flat array
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            if r == 0 and c == 0:
                c = c + 1
                continue
            above: int = 0
            if r > 0:
                above_idx: int = (r - 1) * cols + c
                above = dp[above_idx]
            left: int = 0
            if c > 0:
                left_idx: int = r * cols + (c - 1)
                left = dp[left_idx]
            dp[idx] = above + left
            c = c + 1
        r = r + 1
    last_idx: int = rows * cols - 1
    return dp[last_idx]


def count_paths_with_obstacles(grid: list[int], rows: int, cols: int) -> int:
    """Count paths avoiding obstacles. grid[i]=1 means obstacle at position i."""
    if rows <= 0 or cols <= 0:
        return 0
    if grid[0] == 1:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = 1
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            if grid[idx] == 1:
                dp[idx] = 0
                c = c + 1
                continue
            if r == 0 and c == 0:
                c = c + 1
                continue
            above: int = 0
            if r > 0:
                above_idx: int = (r - 1) * cols + c
                above = dp[above_idx]
            left: int = 0
            if c > 0:
                left_idx: int = r * cols + (c - 1)
                left = dp[left_idx]
            dp[idx] = above + left
            c = c + 1
        r = r + 1
    last_idx: int = rows * cols - 1
    return dp[last_idx]


def min_cost_path(cost: list[int], rows: int, cols: int) -> int:
    """Find minimum cost path from top-left to bottom-right."""
    if rows <= 0 or cols <= 0:
        return 0
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = cost[0]
    c: int = 1
    while c < cols:
        prev: int = c - 1
        dp[c] = dp[prev] + cost[c]
        c = c + 1
    r: int = 1
    while r < rows:
        idx: int = r * cols
        above_idx: int = (r - 1) * cols
        dp[idx] = dp[above_idx] + cost[idx]
        c2: int = 1
        while c2 < cols:
            cur: int = r * cols + c2
            from_above: int = dp[(r - 1) * cols + c2]
            from_left: int = dp[r * cols + (c2 - 1)]
            if from_above < from_left:
                dp[cur] = from_above + cost[cur]
            else:
                dp[cur] = from_left + cost[cur]
            c2 = c2 + 1
        r = r + 1
    last_idx: int = rows * cols - 1
    return dp[last_idx]


def test_module() -> int:
    """Test grid path functions."""
    ok: int = 0

    if count_paths_no_obstacles(3, 3) == 6:
        ok = ok + 1

    if count_paths_no_obstacles(2, 2) == 2:
        ok = ok + 1

    grid: list[int] = [0, 0, 0, 0, 1, 0, 0, 0, 0]
    if count_paths_with_obstacles(grid, 3, 3) == 2:
        ok = ok + 1

    cost: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    if min_cost_path(cost, 3, 3) == 21:
        ok = ok + 1

    if count_paths_no_obstacles(1, 1) == 1:
        ok = ok + 1

    blocked: list[int] = [1, 0, 0, 0]
    if count_paths_with_obstacles(blocked, 2, 2) == 0:
        ok = ok + 1

    return ok
