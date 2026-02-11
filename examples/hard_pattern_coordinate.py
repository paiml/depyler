"""Coordinate compression and 2D grid operations.

Tests: compress_coords, grid_flood_fill, count_islands, manhattan_distance.
"""


def compress_coordinates(coords: list[int]) -> list[int]:
    """Compress coordinates to [0, n-1] range preserving order."""
    n: int = len(coords)
    if n == 0:
        return []
    sorted_unique: list[int] = []
    i: int = 0
    while i < n:
        val: int = coords[i]
        found: int = 0
        j: int = 0
        su_len: int = len(sorted_unique)
        while j < su_len:
            if sorted_unique[j] == val:
                found = 1
            j = j + 1
        if found == 0:
            sorted_unique.append(val)
        i = i + 1
    su_len2: int = len(sorted_unique)
    si: int = 0
    while si < su_len2 - 1:
        sj: int = si + 1
        while sj < su_len2:
            if sorted_unique[sj] < sorted_unique[si]:
                tmp: int = sorted_unique[si]
                sorted_unique[si] = sorted_unique[sj]
                sorted_unique[sj] = tmp
            sj = sj + 1
        si = si + 1
    result: list[int] = []
    k: int = 0
    while k < n:
        val2: int = coords[k]
        idx: int = 0
        m: int = 0
        while m < su_len2:
            if sorted_unique[m] == val2:
                idx = m
            m = m + 1
        result.append(idx)
        k = k + 1
    return result


def grid_create(rows: int, cols: int, fill: int) -> list[list[int]]:
    """Create a 2D grid filled with given value."""
    grid: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(fill)
            c = c + 1
        grid.append(row)
        r = r + 1
    return grid


def grid_flood_fill(grid: list[list[int]], sr: int, sc: int, new_val: int) -> int:
    """Flood fill grid from (sr, sc) with new_val. Returns count of filled cells."""
    rows: int = len(grid)
    if rows == 0:
        return 0
    cols: int = len(grid[0])
    old_val: int = grid[sr][sc]
    if old_val == new_val:
        return 0
    stack_r: list[int] = [sr]
    stack_c: list[int] = [sc]
    count: int = 0
    while len(stack_r) > 0:
        cr: int = stack_r.pop()
        cc: int = stack_c.pop()
        if cr < 0:
            continue
        if cr >= rows:
            continue
        if cc < 0:
            continue
        if cc >= cols:
            continue
        if grid[cr][cc] != old_val:
            continue
        grid[cr][cc] = new_val
        count = count + 1
        stack_r.append(cr - 1)
        stack_c.append(cc)
        stack_r.append(cr + 1)
        stack_c.append(cc)
        stack_r.append(cr)
        stack_c.append(cc - 1)
        stack_r.append(cr)
        stack_c.append(cc + 1)
    return count


def count_islands(grid: list[list[int]]) -> int:
    """Count islands (connected components of 1s) in binary grid using inline DFS."""
    rows: int = len(grid)
    if rows == 0:
        return 0
    cols: int = len(grid[0])
    visited: list[list[int]] = grid_create(rows, cols, 0)
    islands: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            if grid[r][c] == 1:
                if visited[r][c] == 0:
                    islands = islands + 1
                    stack_r: list[int] = [r]
                    stack_c: list[int] = [c]
                    while len(stack_r) > 0:
                        cr: int = stack_r.pop()
                        cc: int = stack_c.pop()
                        if cr < 0:
                            continue
                        if cr >= rows:
                            continue
                        if cc < 0:
                            continue
                        if cc >= cols:
                            continue
                        if grid[cr][cc] != 1:
                            continue
                        if visited[cr][cc] == 1:
                            continue
                        visited[cr][cc] = 1
                        stack_r.append(cr - 1)
                        stack_c.append(cc)
                        stack_r.append(cr + 1)
                        stack_c.append(cc)
                        stack_r.append(cr)
                        stack_c.append(cc - 1)
                        stack_r.append(cr)
                        stack_c.append(cc + 1)
            c = c + 1
        r = r + 1
    return islands


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Manhattan distance between two points."""
    dx: int = x1 - x2
    if dx < 0:
        dx = 0 - dx
    dy: int = y1 - y2
    if dy < 0:
        dy = 0 - dy
    return dx + dy


def test_module() -> int:
    """Test coordinate and grid operations."""
    passed: int = 0

    cc: list[int] = compress_coordinates([100, 50, 200, 50, 100])
    if cc == [1, 0, 2, 0, 1]:
        passed = passed + 1

    g: list[list[int]] = grid_create(3, 3, 0)
    g[0][0] = 1
    g[0][1] = 1
    g[1][0] = 1
    filled: int = grid_flood_fill(g, 0, 0, 2)
    if filled == 3:
        passed = passed + 1

    g2: list[list[int]] = [[1, 1, 0], [0, 1, 0], [0, 0, 1]]
    if count_islands(g2) == 2:
        passed = passed + 1

    if manhattan_distance(1, 2, 4, 6) == 7:
        passed = passed + 1

    cc2: list[int] = compress_coordinates([])
    if cc2 == []:
        passed = passed + 1

    if manhattan_distance(0, 0, 0, 0) == 0:
        passed = passed + 1

    return passed
